// --------------------------------------------------------------------
// Gufo Agent: Config format
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------
use crate::AGENT_DEFAULT_INTERVAL;
use common::LabelsConfig;
use relabel::RelabelRuleConfig;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    #[serde(rename = "$version")]
    pub version: String,
    #[serde(rename = "$type")]
    pub r#type: String,
    pub agent: AgentConfig,
    pub sender: SenderConfig,
    pub collectors: Vec<CollectorConfig>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct AgentConfig {
    #[serde(default = "default_none", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: LabelsConfig,
    #[serde(default = "AgentDefaults::default")]
    pub defaults: AgentDefaults,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AgentDefaults {
    #[serde(default = "default_interval")]
    pub interval: u64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SenderConfig {
    #[serde(default = "default_openmetrics")]
    pub r#type: String,
    #[serde(default = "default_pull")]
    pub mode: String,
    pub listen: Option<String>,
    pub listen_tls: Option<String>,
    #[serde(default = "default_false", skip_serializing_if = "is_false")]
    pub tls_redirect: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub client_auth_requred_path: Option<String>,
    #[serde(default = "default_metrics")]
    pub path: String,
    // tls
    // cert_path, key_path
    // auth
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectorConfig {
    pub id: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u64>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: LabelsConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relabel: Option<Vec<RelabelRuleConfig>>,
    #[serde(flatten)]
    pub config: serde_yaml::Value,
}

impl Hash for CollectorConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.r#type.hash(state);
        self.interval.hash(state);
        if let Some(labels) = &self.labels {
            labels.hash(state);
        }
        self.disabled.hash(state);
        self.config.hash(state);
    }
}

impl CollectorConfig {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for AgentDefaults {
    fn default() -> Self {
        AgentDefaults {
            interval: AGENT_DEFAULT_INTERVAL,
        }
    }
}

impl Default for SenderConfig {
    fn default() -> Self {
        SenderConfig {
            r#type: "openmetrics".into(),
            mode: "pull".into(),
            listen: Some("0.0.0.0:3000".into()),
            listen_tls: None,
            path: "/metrics".into(),
            tls_redirect: false,
            cert_path: None,
            key_path: None,
            client_auth_requred_path: None,
        }
    }
}

fn default_openmetrics() -> String {
    "openmetrics".into()
}

fn default_pull() -> String {
    "pull".into()
}

fn default_metrics() -> String {
    "/metrics".into()
}

fn default_none() -> Option<String> {
    None
}

fn default_interval() -> u64 {
    AGENT_DEFAULT_INTERVAL
}

fn default_false() -> bool {
    false
}

fn is_false(v: &bool) -> bool {
    !v
}
