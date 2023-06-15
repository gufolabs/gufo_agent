// --------------------------------------------------------------------
// Gufo Agent: scrape service discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

pub(crate) mod r#static;

use common::{AgentError, AgentResult, Labels};
use r#static::{StaticSd, StaticSdConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum SdConfig {
    #[serde(rename = "static")]
    Static(StaticSdConfig),
}

pub(crate) enum Sd {
    Static(StaticSd),
}

#[derive(Debug, Clone)]
pub(crate) struct ServiceItem {
    pub target: String,
    pub labels: Labels,
}

pub(crate) trait ServiceDiscovery {
    fn get_services(&self) -> AgentResult<Vec<ServiceItem>>;
}

impl TryFrom<SdConfig> for Sd {
    type Error = AgentError;

    fn try_from(value: SdConfig) -> Result<Self, Self::Error> {
        match value {
            SdConfig::Static(cfg) => Ok(Sd::Static(StaticSd::try_from(cfg)?)),
        }
    }
}

impl ServiceDiscovery for Sd {
    fn get_services(&self) -> AgentResult<Vec<ServiceItem>> {
        match self {
            Sd::Static(sd) => sd.get_services(),
        }
    }
}
