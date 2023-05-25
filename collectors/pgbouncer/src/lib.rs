// --------------------------------------------------------------------
// Gufo Agent: pgbouncer collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use bigdecimal::{BigDecimal, ToPrimitive};
use common::{counter, counter_f, AgentError, Collectable, Measure};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgConnection},
    Connection, Executor, Row,
};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    host: Option<String>,
    port: Option<u16>,
    socket: Option<String>,
    username: Option<String>,
    password: Option<String>,
    #[serde(default = "default_pgbouncer")]
    database: String,
}

// Collector structure
pub struct Collector {
    connect_opts: PgConnectOptions,
}

const US: f64 = 1_000_000.0;

// Generated metrics
counter!(
    pgb_total_xact_count,
    "Total number of SQL transactions pooled by pgbouncer",
    db
);
counter!(
    pgb_total_query_count,
    "Total number of SQL queries pooled by pgbouncer",
    db
);
counter!(
    pgb_total_received,
    "Total volume in bytes of network traffic received by pgbouncer",
    db
);
counter!(
    pgb_total_sent,
    "Total volume in bytes of network traffic sent by pgbouncer",
    db
);
counter_f!(pgb_total_xact_time,"Total number of seconds spent by pgbouncer when connected to PostgreSQL in a transaction, either idle in transaction or executing queries", db);
counter_f!(pgb_total_query_time, "Total number of seconds spent by pgbouncer when actively connected to PostgreSQL, executing queries", db);
counter_f!(
    pgb_total_wait_time,
    "Time spent by clients waiting for a server, in seconds",
    db
);
/*
avg_xact_count
Average transactions per second in last stat period.
avg_query_count
Average queries per second in last stat period.
avg_recv
Average received (from clients) bytes per second.
avg_sent
Average sent (to clients) bytes per second.
avg_xact_time
Average transaction duration, in microseconds.
avg_query_time
Average query duration, in microseconds.
avg_wait_time
Average time spent by clients waiting for a server that were assigned a backend connection within the current stats_period, in microseconds (averaged per second within that period).
*/

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let mut connect_opts = PgConnectOptions::new().database(&value.database);
        if let Some(host) = &value.host {
            connect_opts = connect_opts.host(host.as_str());
        }
        if let Some(port) = &value.port {
            connect_opts = connect_opts.port(*port);
        }
        if let Some(socket) = &value.socket {
            connect_opts = connect_opts.socket(socket.as_str());
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

macro_rules! apply {
    ($r:expr, $fn:ident, $row:expr, $col:expr, $db:expr) => {
        if let Ok(v) = $row.try_get::<BigDecimal, &str>($col) {
            if let Some(x) = v.to_u64() {
                $r.push($fn(x, $db.clone()));
            }
        }
    };
}
macro_rules! apply_us {
    ($r:expr, $fn:ident, $row:expr, $col:expr, $db:expr) => {
        if let Ok(v) = $row.try_get::<BigDecimal, &str>($col) {
            if let Some(x) = v.to_f64() {
                $r.push($fn((x / US) as f32, $db.clone()));
            }
        }
    };
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "pgbouncer";
    type Config = Config;

    async fn collect(&mut self) -> Result<Vec<Measure>, AgentError> {
        // Connect to database
        let mut conn = PgConnection::connect_with(&self.connect_opts)
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Collect data
        let mut r = Vec::new();
        for row in conn
            .fetch_all("SHOW STATS")
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?
            .iter()
        {
            let db: String = match row.try_get("database") {
                Ok(x) => x,
                Err(_) => continue,
            };
            // Remove internal database
            match self.connect_opts.get_database() {
                Some(i_db) => {
                    if db == i_db {
                        continue;
                    }
                }
                None => continue,
            }
            // Push metrics
            // r, fn, row, col, db
            apply!(r, pgb_total_xact_count, row, "total_xact_count", db);
            apply!(r, pgb_total_query_count, row, "total_query_count", db);
            apply!(r, pgb_total_received, row, "total_received", db);
            apply!(r, pgb_total_sent, row, "total_sent", db);
            apply_us!(r, pgb_total_xact_time, row, "total_xact_time", db);
            apply_us!(r, pgb_total_query_time, row, "total_query_time", db);
            apply_us!(r, pgb_total_wait_time, row, "total_wait_time", db);
        }
        Ok(r)
    }
    // !!! Uncomment for config discovery
    // fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
    //     let cfg = Config;
    //     Ok(vec![ConfigItem::from_config(cfg)?])
    // }
}

fn default_pgbouncer() -> String {
    "pgbouncer".to_string()
}
