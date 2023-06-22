// --------------------------------------------------------------------
// Gufo Agent: scrape collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

mod sd;

use async_trait::async_trait;
use common::{AgentError, AgentResult, Collectable, Label, Measure};
use openmetrics::{parse, ParseConfig};
use sd::{Sd, SdConfig, ServiceDiscovery, LABEL_ADDRESS};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::{sync::Semaphore, task::JoinSet};

const DEFAULT_CONCURRENCY: usize = 10;

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    service_discovery: SdConfig,
    #[serde(default = "default_false")]
    trust_timestamps: bool,
    #[serde(
        default = "default_concurrency",
        skip_serializing_if = "is_default_concurrency"
    )]
    concurrency: usize,
}

// Collector structure
pub struct Collector {
    sd: Sd,
    parse_cfg: ParseConfig,
    concurrency: usize,
}

// Generated metrics

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        Ok(Self {
            sd: Sd::try_from(value.service_discovery)?,
            parse_cfg: ParseConfig {
                trust_timestamps: value.trust_timestamps,
            },
            concurrency: value.concurrency,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "scrape";
    type Config = Config;

    async fn collect(&mut self) -> AgentResult<Vec<Measure>> {
        // Perform service discovery
        let services = self.sd.get_services().await?;
        if services.is_empty() {
            log::info!("No services discovered, exiting");
            return Ok(Vec::default());
        }
        // Run scrapes
        let mut tasks = JoinSet::new();
        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        for labels in services.iter() {
            let url = self.sd.get_url(labels);
            let address_label = Label::new(LABEL_ADDRESS, self.sd.get_address(labels).to_owned());
            let parse_cfg = self.parse_cfg.clone();
            let sem = semaphore.clone();
            tasks.spawn(async move {
                // Limit run, drops at the return
                let _permit = sem
                    .acquire()
                    .await
                    .map_err(|e| AgentError::InternalError(e.to_string()))?;
                // Fetch
                let client = reqwest::Client::builder() // @todo: move into
                    .gzip(true)
                    .build()
                    .map_err(|e| AgentError::InternalError(e.to_string()))?;
                match client.get(&url).send().await {
                    Ok(resp) => match resp.text().await {
                        Ok(data) => match parse(data.as_str(), &parse_cfg) {
                            Ok(mut parsed) => {
                                // Install virtual labels
                                for item in parsed.iter_mut() {
                                    item.labels.push(address_label.to_owned());
                                }
                                Ok(parsed)
                            }
                            Err(e) => Err(AgentError::InternalError(format!(
                                "Failed to parse from {}: {}",
                                url, e
                            ))),
                        },
                        Err(e) => Err(AgentError::InternalError(format!(
                            "Failed to fetch {}: {}",
                            url, e
                        ))),
                    },
                    Err(e) => Err(AgentError::InternalError(format!(
                        "Failed to fetch {}: {}",
                        url, e
                    ))),
                }
            });
        }
        // Join and fetch results
        let mut r = Vec::default();
        while let Some(rs) = tasks.join_next().await {
            if let Ok(irs) = rs {
                match irs {
                    Ok(mut ms) => r.append(&mut ms),
                    Err(e) => log::error!("{}", e),
                }
            }
        }
        // Push result
        Ok(r)
    }
    // !!! Uncomment for config discovery
    // fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
    //     let cfg = Config;
    //     Ok(vec![ConfigItem::from_config(cfg)?])
    // }
}

fn default_false() -> bool {
    false
}

fn default_concurrency() -> usize {
    DEFAULT_CONCURRENCY
}

fn is_default_concurrency(v: &usize) -> bool {
    *v == DEFAULT_CONCURRENCY
}
