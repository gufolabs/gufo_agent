// --------------------------------------------------------------------
// Gufo Agent: Relabeling configuration
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use serde::{Deserialize, Serialize};

const DEFAULT_SEPARATOR: &str = ";";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelabelRuleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_labels: Option<Vec<String>>,
    #[serde(
        default = "default_separator",
        skip_serializing_if = "is_default_separator"
    )]
    pub separator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

fn default_separator() -> String {
    DEFAULT_SEPARATOR.into()
}

fn is_default_separator(s: &String) -> bool {
    s == DEFAULT_SEPARATOR
}
