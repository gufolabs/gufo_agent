// --------------------------------------------------------------------
// Gufo Agent: mysql collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{AgentError, Collectable, Labels, Measure};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlConnection},
    Connection, Row,
};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
}

// Collector structure
pub struct Collector {
    connect_opts: MySqlConnectOptions,
}

// Generated metrics

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let mut connect_opts = MySqlConnectOptions::new();
        if let Some(host) = &value.host {
            connect_opts = connect_opts.host(host.as_str());
        }
        if let Some(port) = &value.port {
            connect_opts = connect_opts.port(*port);
        }
        if let Some(username) = &value.username {
            connect_opts = connect_opts.username(username.as_str());
        }
        if let Some(password) = &value.password {
            connect_opts = connect_opts.password(password.as_str());
        }
        Ok(Self { connect_opts })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "mysql";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Connect to database
        let mut conn = MySqlConnection::connect_with(&self.connect_opts)
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Collect data
        let mut r = Vec::new();
        // SHOW GLOBAL STATUS
        let mut rows = sqlx::query("SHOW GLOBAL STATUS")
            .persistent(false)
            .fetch(&mut conn);
        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?
        {
            let name: &str = row.get(0);
            let value: &str = row.get(1);
            if let Ok(v) = value.parse::<u64>() {
                let full_name = format!("mysql_{}", name.to_lowercase());
                r.push(Measure {
                    name: full_name,
                    help: String::default(),
                    value: common::Value::Gauge(v),
                    labels: Labels::default(),
                    timestamp: None,
                })
            }
        }
        // Push result
        Ok(r)
    }
    // !!! Uncomment for config discovery
    // fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
    //     let cfg = Config;
    //     Ok(vec![ConfigItem::from_config(cfg)?])
    // }
}
