// --------------------------------------------------------------------
// Gufo Agent: Config discovery
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{CollectorConfig, Collectors, Config, SenderConfig};
use common::{AgentError, ConfigDiscoveryOpts};
use std::fs::{metadata, read_dir};
use std::path::Path;
use std::process::Command;

// Run all the config discoveries and build resulting connfig
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
    // Built-in
    let mut collectors = config_from_collectors(opts)?;
    r.collectors.append(&mut collectors);
    // Scripts
    let mut collectors = config_from_scripts(opts)?;
    r.collectors.append(&mut collectors);
    //
    serde_yaml::to_string(&r).map_err(|e| AgentError::ConfigurationError(e.to_string()))
}

// Run collector-level config discovery
fn config_from_collectors(opts: &ConfigDiscoveryOpts) -> Result<Vec<CollectorConfig>, AgentError> {
    let mut r = Vec::new();
    for (name, configs) in Collectors::discover_config(opts)?.iter() {
        for cfg in configs.iter() {
            // @todo: Build name
            r.push(CollectorConfig {
                id: name.to_string(),
                r#type: name.to_string(),
                disabled: false,
                interval: opts.get_interval(name),
                labels: None,
                config: cfg.config.clone(),
            });
        }
    }
    Ok(r)
}

// Run collector-level config discovery
fn config_from_scripts(opts: &ConfigDiscoveryOpts) -> Result<Vec<CollectorConfig>, AgentError> {
    let mut r = Vec::new();
    for path in opts.script_paths() {
        let mut sr = config_from_scripts_dir(opts, path)?;
        r.append(&mut sr);
    }
    Ok(r)
}

// Scan directory and run scripts for config
fn config_from_scripts_dir(
    _: &ConfigDiscoveryOpts,
    path: String,
) -> Result<Vec<CollectorConfig>, AgentError> {
    let mut r = Vec::new();
    if let Ok(dirlist) = read_dir(path) {
        for entry in dirlist.flatten() {
            let meta = metadata(entry.path())?;
            if meta.is_file() {
                let mut sr = config_from_script(entry.path().as_path())?;
                r.append(&mut sr);
            }
        }
    }
    Ok(r)
}

fn config_from_script(path: &Path) -> Result<Vec<CollectorConfig>, AgentError> {
    let mut r = Vec::new();
    // Run script
    if let Ok(out) = Command::new(path).output() {
        let v = serde_yaml::from_slice::<serde_yaml::Value>(&out.stdout)
            .map_err(|e| AgentError::ConfigurationError(e.to_string()))?;
        match v {
            // Multiple configs
            serde_yaml::Value::Sequence(seq) => {
                for item in seq {
                    r.push(
                        serde_yaml::from_value::<CollectorConfig>(item)
                            .map_err(|e| AgentError::ConfigurationError(e.to_string()))?,
                    );
                }
            }
            // Single config
            serde_yaml::Value::Mapping(_) => {
                r.push(
                    serde_yaml::from_value::<CollectorConfig>(v)
                        .map_err(|e| AgentError::ConfigurationError(e.to_string()))?,
                );
            }
            _ => {}
        }
    }
    Ok(r)
}
