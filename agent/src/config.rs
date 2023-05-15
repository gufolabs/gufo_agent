// --------------------------------------------------------------------
// Gufo Agent: Config format
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------
use common::LabelsConfig;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    #[serde(rename = "$version")]
    pub version: String,
    #[serde(rename = "$type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: LabelsConfig,
    pub sender: SenderConfig,
    pub collectors: Vec<CollectorConfig>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AgentConfig {
    #[serde(default = "default_none", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SenderConfig {
    #[serde(default = "default_openmetrics")]
    pub r#type: String,
    #[serde(default = "default_pull")]
    pub mode: String,
    #[serde(default = "default_3000")]
    pub listen: String,
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
    pub interval: u64,
    #[serde(default)]
    pub disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: LabelsConfig,
    #[serde(flatten)]
    pub config: serde_yaml::Value,
}

impl Hash for CollectorConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.r#type.hash(state);
        self.interval.hash(state);
        // @todo: Hash the rest
        //self.labels.hash(state);
        // pub disabled: bool,
        // #[serde(default)]
        // pub labels: Vec<String>,
        // #[serde(flatten)]
        // pub config: serde_json::Value,
    }
}

impl CollectorConfig {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

fn default_openmetrics() -> String {
    "openmetrics".into()
}

fn default_pull() -> String {
    "pull".into()
}

fn default_3000() -> String {
    "0.0.0.0:3000".into()
}

fn default_metrics() -> String {
    "/metrics".into()
}

fn default_none() -> Option<String> {
    None
}
