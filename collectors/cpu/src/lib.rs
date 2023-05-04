// --------------------------------------------------------------------
// Gufo Agent: cpu collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{gauge_f, AgentError, Collectable, ConfigDiscoveryOpts, ConfigItem, Measure};
use serde::{Deserialize, Serialize};
use systemstat::{Platform, System};
use tokio::time::{sleep, Duration};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
gauge_f!(cpu_user, "CPU User time, %", cpu);
gauge_f!(cpu_nice, "CPU Nice time, %", cpu);
gauge_f!(cpu_system, "CPU System time, %", cpu);
gauge_f!(cpu_interrupt, "CPU Interrupt time, %", cpu);
gauge_f!(cpu_idle, "CPU Idle time, %", cpu);
#[cfg(target_os = "linux")]
gauge_f!(cpu_iowait, "CPU IOwait time, %", cpu);

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
            let cpu = i.to_string();
            r.push(cpu_user(s.user * 100.0, &cpu));
            r.push(cpu_nice(s.nice * 100.0, &cpu));
            r.push(cpu_system(s.system * 100.0, &cpu));
            r.push(cpu_interrupt(s.interrupt * 100.0, &cpu));
            r.push(cpu_idle(s.idle * 100.0, &cpu));
            // Platform-dependent metrics
            #[cfg(target_os = "linux")]
            r.push(cpu_iowait(s.platform.iowait * 100.0, &cpu));
        }
        // Push result
        Ok(r)
    }
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config;
        Ok(vec![ConfigItem::from_config(cfg)?])
    }
}
