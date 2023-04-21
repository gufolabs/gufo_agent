// --------------------------------------------------------------------
// Gufo Agent: cpu collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{gauge, AgentError, Collectable, Measure};
use serde::Deserialize;
use systemstat::{Platform, System};
use tokio::time::{sleep, Duration};

// Collector config
#[derive(Deserialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
gauge!(user, "???", cpu);
gauge!(nice, "???", cpu);
gauge!(system, "???", cpu);
gauge!(interrupt, "???", cpu);
gauge!(idle, "???", cpu);
#[cfg(target_os = "linux")]
gauge!(iowait, "???", cpu);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(_: Config) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "cpu";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let delayed_stats = System::new()
            .cpu_load()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Wait for CPU statistics been collected
        sleep(Duration::from_secs(1)).await;
        let stats = delayed_stats
            .done()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut r = Vec::with_capacity(stats.len() * 5);
        for (i, s) in stats.iter().enumerate() {
            let cpu = format!("{}", i);
            r.push(user((s.user * 100.0) as u64, &cpu));
            r.push(nice((s.nice * 100.0) as u64, &cpu));
            r.push(system((s.system * 100.0) as u64, &cpu));
            r.push(interrupt((s.interrupt * 100.0) as u64, &cpu));
            r.push(idle((s.idle * 100.0) as u64, &cpu));
            // Platform-dependent metrics
            #[cfg(target_os = "linux")]
            r.push(iowait((s.platform.iowait * 100.0) as u64, &cpu));
        }
        // Push result
        Ok(r)
    }
}
