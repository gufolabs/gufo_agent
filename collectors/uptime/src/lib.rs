// --------------------------------------------------------------------
// Gufo Agent: uptime collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, AgentError, Collectable, Measure};
use serde::Deserialize;
use systemstat::{Platform, System};

// Collector config
#[derive(Deserialize)]
pub struct Config;

// Collector structure
pub struct Collector;

// Generated metrics
counter!(uptime, "System uptime");

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
    const NAME: &'static str = "uptime";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Collect data
        let sys = System::new();
        let v = sys
            .uptime()
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Push result
        Ok(vec![uptime(v.as_secs())])
    }
}
