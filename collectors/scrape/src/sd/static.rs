// --------------------------------------------------------------------
// Gufo Agent: static service discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ServiceDiscovery, ServiceItem};
use common::{AgentError, AgentResult, Labels, LabelsConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct StaticSdConfig {
    targets: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: LabelsConfig,
}

pub(crate) struct StaticSd {
    items: Vec<ServiceItem>,
}

impl TryFrom<StaticSdConfig> for StaticSd {
    type Error = AgentError;

    fn try_from(value: StaticSdConfig) -> Result<Self, Self::Error> {
        let labels: Labels = value.labels.into();
        Ok(Self {
            items: value
                .targets
                .iter()
                .map(|x| ServiceItem {
                    target: x.to_owned(),
                    labels: labels.clone(),
                })
                .collect(),
        })
    }
}

impl ServiceDiscovery for StaticSd {
    fn get_services(&self) -> AgentResult<Vec<ServiceItem>> {
        Ok(self.items.clone())
    }
}
