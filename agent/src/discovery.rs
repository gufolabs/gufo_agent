// --------------------------------------------------------------------
// Gufo Agent: Config discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{CollectorConfig, Collectors, Config, SenderConfig};
use common::{AgentError, ConfigDiscoveryOpts};

pub fn config_from_discovery(opts: &ConfigDiscoveryOpts) -> Result<String, AgentError> {
    let mut r = Config {
        version: "1.0".into(),
        r#type: "zeroconf".into(),
        labels: None,
        sender: SenderConfig {
            r#type: "openmetrics".into(),
            mode: "pull".into(),
            listen: "0.0.0.0:3000".into(),
            path: "/metrics".into(),
        },
        collectors: Vec::new(),
    };

    for (name, configs) in Collectors::discover_config(opts)?.iter() {
        for cfg in configs.iter() {
            // @todo: Build name
            r.collectors.push(CollectorConfig {
                id: name.to_string(),
                r#type: name.to_string(),
                disabled: false,
                interval: 10, // @todo: Configurable
                labels: None,
                config: cfg.config.clone(),
            });
        }
    }
    serde_yaml::to_string(&r).map_err(|e| AgentError::ConfigurationError(e.to_string()))
}
