// --------------------------------------------------------------------
// Gufo Agent: exec collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, AgentError, Collectable, Measure};
use openmetrics::{parse, ParseConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    cmd: Vec<String>,
    cd: Option<String>,
    env: Option<HashMap<String, String>>,
    #[serde(default = "default_false")]
    trust_timestamps: bool,
}

// Collector structure
pub struct Collector {
    cmd: Vec<String>,
    cd: Option<String>,
    env: Option<HashMap<String, String>>,
    parse_cfg: ParseConfig,
    // Stats
    exec_parsed: u64,
}

// Generated metrics
counter!(exec_parsed, "Parsed metric items", script);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        if value.cmd.is_empty() {
            return Err(AgentError::ConfigurationError(
                "cmd must not be empty".to_string(),
            ));
        }
        Ok(Self {
            cmd: value.cmd,
            cd: value.cd,
            env: value.env,
            parse_cfg: ParseConfig {
                trust_timestamps: value.trust_timestamps,
            },
            // Stats
            exec_parsed: 0,
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "exec";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Construct command
        let mut cmd = &mut Command::new(self.cmd[0].clone());
        // Args
        if self.cmd.len() > 1 {
            cmd = cmd.args(&self.cmd[1..]);
        }
        // Current directory
        if let Some(cwd) = &self.cd {
            cmd = cmd.current_dir(cwd);
        }
        // Environment
        if let Some(env) = &self.env {
            cmd = cmd.envs(env.clone().into_iter());
        }
        // Detach stdio
        cmd = cmd.stdin(Stdio::null());
        // Run command
        let output = cmd
            .output()
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Parse stdout
        let out = String::from_utf8(output.stdout)
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut parsed = parse(out.as_str(), &self.parse_cfg)?;
        self.exec_parsed += parsed.len() as u64;
        // Push result
        let mut r = Vec::new();
        r.append(&mut parsed);
        // Apply internal metrics
        r.push(exec_parsed(self.exec_parsed, self.cmd[0].clone()));
        //
        Ok(r)
    }
}

fn default_false() -> bool {
    false
}
