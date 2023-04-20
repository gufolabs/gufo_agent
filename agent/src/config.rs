// --------------------------------------------------------------------
// Gufo Agent: Config format
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------
use common::LabelsConfig;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "$version")]
    pub version: String,
    #[serde(rename = "$type")]
    pub r#type: String,
    pub labels: LabelsConfig,
    pub collectors: Vec<CollectorConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CollectorConfig {
    pub id: String,
    pub r#type: String,
    pub interval: u64,
    #[serde(default)]
    pub disabled: bool,
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
