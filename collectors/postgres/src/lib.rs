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
        let mut connect_opts = PgConnectOptions::new();
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
            //
            if let Ok(v) = row.try_get::<i32, &str>("numbackends") {
                r.push(pg_numbackends(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("xact_commit") {
                r.push(pg_xact_commit(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("xact_rollback") {
                r.push(pg_xact_rollback(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("blks_read") {
                r.push(pg_blks_read(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("blks_hit") {
                r.push(pg_blks_hit(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("tup_returned") {
                r.push(pg_tup_returned(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("tup_fetched") {
                r.push(pg_tup_fetched(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("tup_inserted") {
                r.push(pg_tup_inserted(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("tup_updated") {
                r.push(pg_tup_updated(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("tup_deleted") {
                r.push(pg_tup_deleted(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("conflicts") {
                r.push(pg_conflicts(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("temp_files") {
                r.push(pg_temp_files(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("temp_bytes") {
                r.push(pg_temp_bytes(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("deadlocks") {
                r.push(pg_deadlocks(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("checksum_failures") {
                r.push(pg_checksum_failures(v as u64, db.clone()));
            }
            // pg_checksum_last_failure
            if let Ok(v) = row.try_get::<f64, &str>("blk_read_time") {
                r.push(pg_blk_read_time((v / MS) as f32, db.clone()));
            }
            if let Ok(v) = row.try_get::<f64, &str>("blk_write_time") {
                r.push(pg_blk_write_time((v / MS) as f32, db.clone()));
            }
            if let Ok(v) = row.try_get::<f64, &str>("session_time") {
                r.push(pg_session_time((v / MS) as f32, db.clone()));
            }
            if let Ok(v) = row.try_get::<f64, &str>("active_time") {
                r.push(pg_active_time((v / MS) as f32, db.clone()));
            }
            if let Ok(v) = row.try_get::<f64, &str>("idle_in_transaction_time") {
                r.push(pg_idle_in_transaction_time((v / MS) as f32, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("sessions") {
                r.push(pg_sessions(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("sessions_abandoned") {
                r.push(pg_sessions_abandoned(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("sessions_fatal") {
                r.push(pg_sessions_fatal(v as u64, db.clone()));
            }
            if let Ok(v) = row.try_get::<i64, &str>("sessions_killed") {
                r.push(pg_sessions_killed(v as u64, db.clone()));
            }
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
