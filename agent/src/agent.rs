// --------------------------------------------------------------------
// Gufo Agent: Agent implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{CollectorConfig, Config, ConfigResolver, Schedule, Sender, SenderCommand};
use common::{AgentError, AgentResult, Label, Labels};
use gethostname::gethostname;
use std::collections::{HashMap, HashSet};
use tokio::{
    runtime::Runtime,
    signal::unix::{signal, SignalKind},
    sync::mpsc,
    task::JoinHandle,
};

pub(crate) const AGENT_DEFAULT_INTERVAL: u64 = 60;

pub struct Agent {
    resolver: ConfigResolver,
    running: HashMap<String, RunningCollector>,
    sender_tx: Option<mpsc::Sender<SenderCommand>>,
    hostname: String,
    dump_metrics: bool,
    default_interval: u64,
    mode: AgentMode,
}

#[derive(Debug, Clone, Default)]
pub enum AgentMode {
    Check,
    Test,
    #[default]
    Run,
}

pub struct RunningCollector {
    handle: JoinHandle<()>,
    config_hash: u64,
}

impl Agent {
    pub fn builder() -> AgentBuilder {
        AgentBuilder::default()
    }
}

#[derive(Default)]
pub struct AgentBuilder {
    cert_validation: bool,
    dump_metrics: bool,
    config: Option<String>,
    hostname: Option<String>,
    mode: AgentMode,
}

impl AgentBuilder {
    pub fn set_cert_validation(&mut self, status: bool) -> &mut Self {
        self.cert_validation = status;
        self
    }
    pub fn set_config(&mut self, config: Option<String>) -> &mut Self {
        self.config = config;
        self
    }
    pub fn set_dump_metrics(&mut self, status: bool) -> &mut Self {
        self.dump_metrics = status;
        self
    }
    pub fn set_hostname(&mut self, hostname: Option<String>) -> &mut Self {
        self.hostname = hostname;
        self
    }
    pub fn set_mode(&mut self, mode: AgentMode) -> &mut Self {
        self.mode = mode;
        self
    }
    pub fn build(&self) -> Agent {
        Agent {
            resolver: ConfigResolver::builder()
                .set_cert_validation(self.cert_validation)
                .set_url(self.config.clone())
                .build(),
            running: HashMap::new(),
            sender_tx: None,
            hostname: self
                .hostname
                .clone()
                .unwrap_or_else(|| gethostname().into_string().unwrap_or("localhost".into())),
            dump_metrics: self.dump_metrics,
            default_interval: AGENT_DEFAULT_INTERVAL,
            mode: self.mode.clone(),
        }
    }
}

impl Agent {
    pub fn run(&mut self) -> AgentResult<()> {
        let runtime = Runtime::new()?;
        runtime.block_on(async move {
            match self.mode {
                AgentMode::Check | AgentMode::Test => self.run_test().await,
                AgentMode::Run => {
                    log::error!("Running agent on {}", self.hostname);
                    let r = self.bootstrap().await;
                    log::error!("Stopping agent");
                    r
                }
            }
        })
    }
    async fn bootstrap(&mut self) -> AgentResult<()> {
        // Subscribe to SIGHUP
        let mut hup_stream = signal(SignalKind::hangup())?;
        // Initialize resolver
        self.resolver.bootstrap().await?;
        //
        loop {
            match self.configure().await {
                Ok(_) => {
                    // Wait for sighup
                    hup_stream.recv().await;
                    log::info!("SIGHUP received, reloading configuration");
                }
                Err(e) => {
                    log::error!("Failed to process configuration: {}", e);
                    break;
                }
            }
        }
        self.wait_all().await?;
        Ok(())
    }
    async fn configure(&mut self) -> Result<(), AgentError> {
        let config = self.resolver.get_config().await?;
        // @todo: Apply resolver config
        self.apply(config).await?;
        Ok(())
    }
    async fn apply(&mut self, cfg: Config) -> Result<(), AgentError> {
        self.configure_agent(&cfg).await?;
        self.configure_sender(&cfg).await?;
        // Configure collectors
        let mut id_set = HashSet::new();
        for collector_cfg in cfg.collectors.iter() {
            let collector_id = collector_cfg.id.clone();
            if collector_cfg.disabled {
                log::debug!("[{}] Collector is disabled", collector_id);
                // Stop if runnning
                if self.running.get(&collector_id).is_some() {
                    if let Err(e) = self.stop_collector(&collector_id).await {
                        log::error!("[{}] Failed to stop collector: {}", collector_id, e);
                    }
                }
                continue;
            }
            let r = match self.running.get(&collector_id) {
                Some(_) => self.update_collector(collector_cfg).await,
                None => self.spawn_collector(collector_cfg).await,
            };
            if let Err(e) = r {
                log::error!(
                    "[{}] Failed to initialize collector: {:?}",
                    &collector_id,
                    e
                )
            }
            id_set.insert(collector_id);
        }
        // Stop unused collectors
        let mut stop_set = HashSet::new();
        for x in self.running.keys() {
            if !id_set.contains(x) {
                stop_set.insert(x.clone());
            }
        }
        for x in stop_set.iter() {
            self.stop_collector(x).await?;
        }
        Ok(())
    }
    // Configure agent
    async fn configure_agent(&mut self, cfg: &Config) -> AgentResult<()> {
        if let Some(host) = &cfg.agent.host {
            self.hostname = host.clone();
        }
        self.default_interval = cfg.agent.defaults.interval;
        Ok(())
    }
    // Configure sender
    async fn configure_sender(&mut self, cfg: &Config) -> Result<(), AgentError> {
        if self.sender_tx.is_none() {
            // Not confugured yet, run sender
            let mut sender = Sender::try_from(&cfg.sender)?;
            sender.set_dump_metrics(self.dump_metrics);
            self.sender_tx = Some(sender.get_tx());
            tokio::spawn(async move {
                sender.run().await;
            });
        }
        // Configure labels
        if let Some(tx) = &self.sender_tx {
            let mut labels: Labels = cfg.agent.labels.clone().into();
            labels.push(Label::new("host", self.hostname.clone()));
            if let Err(e) = tx.send(SenderCommand::SetAgentLabels(labels)).await {
                log::error!("Failed to set labels: {}", e);
            }
        }
        Ok(())
    }
    //
    fn apply_defaults(&self, config: &CollectorConfig) -> AgentResult<CollectorConfig> {
        let mut config = config.clone();
        if config.interval.is_none() {
            config.interval = Some(self.default_interval);
        }
        Ok(config)
    }
    // Start new collector instance
    async fn spawn_collector(&mut self, config: &CollectorConfig) -> Result<(), AgentError> {
        let config = self.apply_defaults(config)?;
        let config_id = config.id.clone();
        let config_hash = config.get_hash();
        log::debug!("[{}] Starting collector", config_id);
        let mut schedule = Schedule::try_from(config)?;
        schedule.set_sender(self.sender_tx.clone());
        let handle = tokio::spawn(async move { schedule.run().await });
        self.running.insert(
            config_id,
            RunningCollector {
                handle,
                config_hash,
            },
        );
        Ok(())
    }
    // // Stop running collector
    async fn stop_collector(&mut self, collector_id: &str) -> Result<(), AgentError> {
        log::debug!("[{}] Stopping", collector_id);
        if let Some(item) = self.running.remove(collector_id) {
            item.abort();
        }
        Ok(())
    }
    // // Update running collector configuration
    async fn update_collector(&mut self, config: &CollectorConfig) -> Result<(), AgentError> {
        let config = self.apply_defaults(config)?;
        if let Some(item) = self.running.get(&config.id) {
            if item.is_changed(config.get_hash()) {
                log::debug!("[{}] Configuration changed, restarting", &config.id);
                self.stop_collector(&config.id).await?;
                self.spawn_collector(&config).await?;
            }
        }
        Ok(())
    }
    // Wait for all running collectors to complete
    async fn wait_all(&mut self) -> Result<(), AgentError> {
        for (_, item) in self.running.drain() {
            item.handle
                .await
                .map_err(|e| AgentError::InternalError(e.to_string()))?;
        }
        Ok(())
    }
    //
    async fn run_test(&mut self) -> AgentResult<()> {
        // Get config
        let config = self.resolver.get_config().await?;
        // Configure agent
        self.configure_agent(&config).await?;
        self.configure_sender(&config).await?;
        //
        for collector_cfg in config.collectors.iter() {
            if collector_cfg.disabled {
                continue;
            }
            let config = self.apply_defaults(collector_cfg)?;
            let config_id = config.id.clone();
            log::debug!("[{}] Starting collector", config_id);
            let mut schedule = Schedule::try_from(config)?;
            if let AgentMode::Test = self.mode {
                schedule.set_sender(self.sender_tx.clone());
                schedule.run_once().await;
            }
        }
        // Dump database
        if let AgentMode::Test = self.mode {
            if let Some(tx) = &self.sender_tx {
                tx.send(SenderCommand::Dump)
                    .await
                    .map_err(|e| AgentError::InternalError(e.to_string()))?;
                tx.send(SenderCommand::Shutdown)
                    .await
                    .map_err(|e| AgentError::InternalError(e.to_string()))?;
                tx.closed().await;
            }
        }
        //
        Ok(())
    }
}

impl RunningCollector {
    pub fn abort(&self) {
        self.handle.abort()
    }
    pub fn is_changed(&self, hash: u64) -> bool {
        self.config_hash != hash
    }
}
