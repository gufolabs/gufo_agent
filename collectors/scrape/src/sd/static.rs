// --------------------------------------------------------------------
// Gufo Agent: static service discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use super::{ServiceDiscovery, LABEL_ADDRESS};
use common::{AgentError, AgentResult, Label};
use relabel::ActiveLabels;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct StaticSdConfig {
    targets: Vec<String>,
}

pub(crate) struct StaticSd {
    items: Vec<ActiveLabels>,
}

impl TryFrom<StaticSdConfig> for StaticSd {
    type Error = AgentError;

    fn try_from(value: StaticSdConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            items: value
                .targets
                .iter()
                .map(|x| ActiveLabels::new(vec![Label::new(LABEL_ADDRESS, x.to_owned())]))
                .collect(),
        })
    }
}

impl ServiceDiscovery for StaticSd {
    fn get_services(&self) -> AgentResult<Vec<ActiveLabels>> {
        Ok(self.items.clone())
    }
}
