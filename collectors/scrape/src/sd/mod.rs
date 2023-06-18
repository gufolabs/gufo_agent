// --------------------------------------------------------------------
// Gufo Agent: scrape service discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

pub(crate) mod dns;
pub(crate) mod r#static;

use async_trait::async_trait;
use common::{AgentError, AgentResult, Label};
use dns::{DnsSd, DnsSdConfig};
use r#static::{StaticSd, StaticSdConfig};
use relabel::{ActionResult, ActiveLabels, RelabelRuleConfig, RelabelRuleset, Relabeler};
use serde::{Deserialize, Serialize};

const DEFAULT_SCHEMA: &str = "http";
const DEFAULT_PATH: &str = "/metrics";

pub(crate) const LABEL_ADDRESS: &str = "__address__";
pub(crate) const LABEL_SCHEMA: &str = "__meta_sd_schema";
pub(crate) const LABEL_PATH: &str = "__meta_sd_path";

#[derive(Serialize, Deserialize)]
pub(crate) struct SdConfig {
    #[serde(flatten)]
    pub method: SdMethodConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relabel: Option<Vec<RelabelRuleConfig>>,
    #[serde(default = "default_http", skip_serializing_if = "is_default_schema")]
    schema: String,
    #[serde(default = "default_metrics", skip_serializing_if = "is_default_path")]
    path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum SdMethodConfig {
    #[serde(rename = "dns")]
    Dns(DnsSdConfig),
    #[serde(rename = "static")]
    Static(StaticSdConfig),
}

pub(crate) struct Sd {
    method: SdMethod,
    relabel: Option<RelabelRuleset>,
    default_schema: String,
    default_path: String,
}

pub(crate) enum SdMethod {
    Dns(DnsSd),
    Static(StaticSd),
}

#[async_trait]
pub(crate) trait ServiceDiscovery {
    async fn get_services(&self) -> AgentResult<Vec<ActiveLabels>>;
}

impl TryFrom<SdConfig> for Sd {
    type Error = AgentError;

    fn try_from(value: SdConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            method: match value.method {
                SdMethodConfig::Static(cfg) => SdMethod::Static(StaticSd::try_from(cfg)?),
                SdMethodConfig::Dns(cfg) => SdMethod::Dns(DnsSd::try_from(cfg)?),
            },
            relabel: match &value.relabel {
                Some(v) => Some(RelabelRuleset::try_from(v)?),
                None => None,
            },
            default_schema: value.schema,
            default_path: value.path,
        })
    }
}

#[async_trait]
impl ServiceDiscovery for Sd {
    async fn get_services(&self) -> AgentResult<Vec<ActiveLabels>> {
        let mut services = match &self.method {
            SdMethod::Dns(sd) => sd.get_services().await?,
            SdMethod::Static(sd) => sd.get_services().await?,
        };
        Ok(match &self.relabel {
            Some(ruleset) => {
                if services.is_empty() {
                    services
                } else {
                    let mut r = Vec::with_capacity(services.len());
                    // Apply relabeling
                    for mut labels in services.drain(..) {
                        labels.insert(Label::new(LABEL_SCHEMA, self.default_schema.to_owned()));
                        labels.insert(Label::new(LABEL_PATH, self.default_path.to_owned()));
                        match ruleset.apply(&mut labels)? {
                            ActionResult::Pass => r.push(labels),
                            ActionResult::Drop => log::debug!(
                                "Target {} is dropped by rule",
                                self.get_address(&labels)
                            ),
                        }
                    }
                    r
                }
            }
            None => services,
        })
    }
}

impl Sd {
    pub(crate) fn get_url(&self, labels: &ActiveLabels) -> String {
        let schema = match labels.get(LABEL_SCHEMA) {
            Some(x) => x.clone(),
            None => DEFAULT_SCHEMA.to_string(),
        };
        let address = self.get_address(labels);
        let path = match labels.get(LABEL_PATH) {
            Some(x) => x.clone(),
            None => DEFAULT_PATH.to_string(),
        };
        format!("{}://{}{}", schema, address, path)
    }
    pub(crate) fn get_address(&self, labels: &ActiveLabels) -> String {
        match labels.get(LABEL_ADDRESS) {
            Some(x) => x.clone(),
            None => "127.0.0.1:3000".to_string(),
        }
    }
}

fn default_http() -> String {
    DEFAULT_SCHEMA.into()
}

fn default_metrics() -> String {
    DEFAULT_PATH.into()
}

fn is_default_schema(v: &String) -> bool {
    v == DEFAULT_SCHEMA
}
fn is_default_path(v: &String) -> bool {
    v == DEFAULT_PATH
}
