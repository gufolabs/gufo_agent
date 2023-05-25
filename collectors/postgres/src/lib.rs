// --------------------------------------------------------------------
// Gufo Agent: postgres collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{counter, counter_f, gauge, AgentError, AgentResult, Collectable, Measure};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgConnection},
    Connection, Row,
};

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    host: Option<String>,
    port: Option<u16>,
    socket: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

// Collector structure
pub struct Collector {
    connect_opts: PgConnectOptions,
}

const MS: f64 = 1_000.0;

// Generated metrics
gauge!(
    pg_numbackends,
    "Number of backends currently connected to this database.",
    db
);
counter!(
    pg_xact_commit,
    "Number of transactions in this database that have been committed.",
    db
);
counter!(
    pg_xact_rollback,
    "Number of transactions in this database that have been rolled back.",
    db
);
counter!(
    pg_blks_read,
    "Number of disk blocks read in this database.",
    db
);
counter!(pg_blks_hit, "Number of times disk blocks were found already in the buffer cache, so that a read was not necessary", db);
counter!(pg_tup_returned, "Number of live rows fetched by sequential scans and index entries returned by index scans in this database.", db);
counter!(
    pg_tup_fetched,
    "Number of live rows fetched by index scans in this database.",
    db
);
counter!(
    pg_tup_inserted,
    "Number of rows inserted by queries in this database.",
    db
);
counter!(
    pg_tup_updated,
    "Number of rows updated by queries in this database.",
    db
);
counter!(
    pg_tup_deleted,
    "Number of rows deleted by queries in this database.",
    db
);
counter!(
    pg_conflicts,
    "Number of queries canceled due to conflicts with recovery in this database",
    db
);
counter!(
    pg_temp_files,
    "Number of temporary files created by queries in this database",
    db
);
counter!(
    pg_temp_bytes,
    "Total amount of data written to temporary files by queries in this database",
    db
);
counter!(
    pg_deadlocks,
    "Number of deadlocks detected in this database.",
    db
);
counter!(
    pg_checksum_failures,
    "Number of data page checksum failures detected in this database",
    db
);
// counter!(
//     pg_checksum_last_failure,
//     "Time at which the last data page checksum failure was detected in this database",
//     db
// );
counter_f!(
    pg_blk_read_time,
    "Time spent reading data file blocks by backends in this database, in seconds",
    db
);
counter_f!(
    pg_blk_write_time,
    "Time spent writing data file blocks by backends in this database, in seconds",
    db
);
counter_f!(
    pg_session_time,
    "Time spent by database sessions in this database, in seconds",
    db
);
counter_f!(
    pg_active_time,
    "Time spent executing SQL statements in this database, in seconds",
    db
);
counter_f!(
    pg_idle_in_transaction_time,
    "Time spent idling while in a transaction in this database, in seconds",
    db
);
counter!(
    pg_sessions,
    "Total number of sessions established to this database.",
    db
);
counter!(pg_sessions_abandoned, "Number of database sessions to this database that were terminated because connection to the client was lost.", db);
counter!(
    pg_sessions_fatal,
    "Number of database sessions to this database that were terminated by fatal errors.",
    db
);
counter!(
    pg_sessions_killed,
    "Number of database sessions to this database that were terminated by operator intervention.",
    db
);
// counter!(
//     pg_stats_reset,
//     "Time at which these statistics were last reset.",
//     db
// );

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let mut connect_opts = PgConnectOptions::new().application_name("gufo-agent");
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
    ($r:expr, $fn:ident, $row:expr, $name:expr, $db:expr) => {
        if let Ok(v) = $row.try_get::<i64, &str>($name) {
            $r.push($fn(v as u64, $db.clone()));
        }
    };
}

macro_rules! apply32 {
    ($r:expr, $fn:ident, $row:expr, $name:expr, $db:expr) => {
        if let Ok(v) = $row.try_get::<i32, &str>($name) {
            $r.push($fn(v as u64, $db.clone()));
        }
    };
}
macro_rules! apply_ms {
    ($r:expr, $fn:ident, $row:expr, $name:expr, $db:expr) => {
        if let Ok(v) = $row.try_get::<f64, &str>($name) {
            $r.push($fn((v / MS) as f32, $db.clone()));
        }
    };
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "postgres";
    type Config = Config;

    async fn collect(&mut self) -> AgentResult<Vec<Measure>> {
        // Connect to database
        let mut conn = PgConnection::connect_with(&self.connect_opts)
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Collect data
        let mut rows = sqlx::query("SELECT * FROM pg_stat_database")
            .persistent(false)
            .fetch(&mut conn);
        let mut r = Vec::new();
        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?
        {
            // Get database
            let db: String = match row.try_get("datname") {
                Ok(x) => x,
                Err(_) => continue,
            };
            // r, fn, row, name, db
            apply32!(r, pg_numbackends, row, "numbackends", db);
            apply!(r, pg_xact_commit, row, "xact_commit", db);
            apply!(r, pg_xact_rollback, row, "xact_rollback", db);
            apply!(r, pg_blks_read, row, "blks_read", db);
            apply!(r, pg_blks_hit, row, "blks_hit", db);
            apply!(r, pg_tup_returned, row, "tup_returned", db);
            apply!(r, pg_tup_fetched, row, "tup_fetched", db);
            apply!(r, pg_tup_inserted, row, "tup_inserted", db);
            apply!(r, pg_tup_updated, row, "tup_updated", db);
            apply!(r, pg_tup_deleted, row, "tup_deleted", db);
            apply!(r, pg_conflicts, row, "conflicts", db);
            apply!(r, pg_temp_files, row, "temp_files", db);
            apply!(r, pg_temp_bytes, row, "temp_bytes", db);
            apply!(r, pg_deadlocks, row, "deadlocks", db);
            apply!(r, pg_checksum_failures, row, "checksum_failures", db);
            // pg_checksum_last_failure
            apply_ms!(r, pg_blk_read_time, row, "blk_read_time", db);
            apply_ms!(r, pg_blk_write_time, row, "blk_write_time", db);
            apply_ms!(r, pg_session_time, row, "session_time", db);
            apply_ms!(r, pg_active_time, row, "active_time", db);
            apply_ms!(
                r,
                pg_idle_in_transaction_time,
                row,
                "idle_in_transaction_time",
                db
            );
            apply!(r, pg_sessions, row, "sessions", db);
            apply!(r, pg_sessions_abandoned, row, "sessions_abandoned", db);
            apply!(r, pg_sessions_fatal, row, "sessions_fatal", db);
            apply!(r, pg_sessions_killed, row, "sessions_killed", db);
            //pg_stats_reset,
        }
        Ok(r)
    }
    // !!! Uncomment for config discovery
    // fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
    //     let cfg = Config;
    //     Ok(vec![ConfigItem::from_config(cfg)?])
    // }
}
