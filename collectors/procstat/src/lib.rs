// --------------------------------------------------------------------
// Gufo Agent: procstat collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{
    counter, counter_f, gauge, gauge_f, AgentError, AgentResult, Collectable, ConfigDiscoveryOpts,
    ConfigItem, Label, Labels, Measure,
};
use lazy_static::lazy_static;
use ps::{Pid, Ps, PsFinder, QueryConf};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use users::{Users, UsersCache};

const DEFAULT_CMD_SEPARATOR: &str = " ";
const DEFAULT_ENV_SEPARATOR: &str = ";";

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_pid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_labels: Option<Vec<String>>,
    #[serde(
        default = "default_cmd_separator",
        skip_serializing_if = "is_default_cmd_separator"
    )]
    cmd_separator: String,
    #[serde(
        default = "default_env_separator",
        skip_serializing_if = "is_default_env_separator"
    )]
    env_separator: String,
}

// Collector structure
pub struct Collector {
    pid_file: Option<String>,
    pattern: Option<Regex>,
    self_pid: bool,
    expose_user: bool,
    expose_cmd: bool,
    expose_meta_cmd: bool,
    expose_meta_env: bool,
    last_run: Option<Instant>,
    cpu_totals: HashMap<Pid, f32>,
    cmd_separator: String,
    env_separator: String,
}

// Users cache
lazy_static! {
    static ref USERS_CACHE: Arc<Mutex<UsersCache>> = Arc::new(Mutex::new(UsersCache::new()));
}

// Generated metrics
gauge!(ps_num_fds, "Number of open files", Labels);
gauge!(ps_num_threads, "Number of threads", Labels);
// ctx switches
counter!(
    ps_voluntary_context_switches,
    "Total voluntary context switches",
    Labels
);
counter!(
    ps_involuntary_context_switches,
    "Total involuntary context switches",
    Labels
);
// page faults
counter!(
    ps_minor_faults,
    "Total number of minor faults which do not requirie loading memory from disk",
    Labels
);
counter!(
    ps_major_faults,
    "Total number of major faults which require loading memory from disk",
    Labels
);
counter!(
    ps_child_minor_faults,
    "Total number of minor faults that process waited-for children made",
    Labels
);
counter!(
    ps_child_major_faults,
    "Total number of major faults that process waited-for children made",
    Labels
);
// CPU
counter_f!(ps_cpu_time_user, "CPU time in user mode in seconds", Labels);
counter_f!(
    ps_cpu_time_system,
    "CPU time in system mode in seconds",
    Labels
);
counter_f!(ps_cpu_time_iowait, "CPU time iowait in seconds", Labels);
gauge_f!(ps_cpu_usage, "Total CPU usage in percents", Labels);
// Mem
gauge!(ps_mem_total, "Total memory", Labels);
gauge!(ps_mem_rss, "Resident set size", Labels);
gauge!(ps_mem_swap, "Swapped-out virtual memory size", Labels);
gauge!(ps_mem_data, "Data segment size", Labels);
gauge!(ps_mem_stack, "Stack segment size", Labels);
gauge!(ps_mem_text, "Text segment size", Labels);
gauge!(ps_mem_lib, "Shared library code size", Labels);
gauge!(ps_mem_locked, "Locked memory size", Labels);
// I/O
counter!(ps_read_count, "Total read I/O operations", Labels);
counter!(ps_write_count, "Total write I/O operations", Labels);
counter!(ps_read_bytes, "Total bytes read", Labels);
counter!(ps_write_bytes, "Total bytes written", Labels);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let self_pid = value.self_pid.unwrap_or(false);
        // Check if configured
        if value.pid_file.is_none() && value.pattern.is_none() && !self_pid {
            return Err(AgentError::ConfigurationError(
                "pid_file, pattern, or self_pid must be set".to_string(),
            ));
        }
        // Compile pattern if any
        let pattern = match value.pattern {
            Some(re) => {
                Some(Regex::new(&re).map_err(|e| AgentError::ConfigurationError(e.to_string()))?)
            }
            None => None,
        };
        // Process expose_labels
        let (expose_user, expose_cmd, expose_meta_cmd, expose_meta_env) = match value.expose_labels
        {
            Some(expose) => (
                expose.contains(&"user".to_string()),
                expose.contains(&"cmd".to_string()),
                expose.contains(&"__meta_cmd".to_string()),
                expose.contains(&"__meta_env".to_string()),
            ),
            None => (false, false, false, false),
        };
        //
        Ok(Self {
            pid_file: value.pid_file,
            pattern,
            self_pid,
            expose_user,
            expose_cmd,
            expose_meta_cmd,
            expose_meta_env,
            last_run: None,
            cpu_totals: HashMap::default(),
            cmd_separator: value.cmd_separator,
            env_separator: value.env_separator,
        })
    }
}

macro_rules! apply_some {
    ($r:ident, $v:expr, $fn:ident, $labels:expr) => {
        if let Some(x) = $v {
            $r.push($fn(x, $labels));
        }
    };
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "procstat";
    type Config = Config;

    async fn collect(&mut self) -> AgentResult<Vec<Measure>> {
        // Filter pids
        let mut all_pids = HashSet::new();
        // self_pid
        if self.self_pid {
            match Ps::filter_by_self() {
                Ok(pids) => Self::apply_pids(&mut all_pids, pids),
                Err(e) => log::error!("Failed to self pid: {}", e),
            }
        }
        // pid_file
        if let Some(pid_file) = &self.pid_file {
            match Self::read_pid_file(pid_file) {
                Ok(pid) => match Ps::filter_by_pid(pid) {
                    Ok(pids) => Self::apply_pids(&mut all_pids, pids),
                    Err(e) => log::error!("Failed to get pids: {}", e),
                },
                Err(e) => log::error!("Failed to read pid file {}: {}", pid_file, e),
            }
        }
        // pattern
        if let Some(pattern) = &self.pattern {
            match Ps::filter_by_pattern(pattern) {
                Ok(pids) => Self::apply_pids(&mut all_pids, pids),
                Err(e) => log::error!("Failed to query patterns: {}", e),
            }
        }
        // Check if we have pids to query
        if all_pids.is_empty() {
            log::error!("No pids to query. Skipping");
            return Ok(vec![]);
        }
        // Collect data
        let mut r = Vec::with_capacity(all_pids.len() * 20);
        // Current timestamp and delta
        let now = Instant::now();
        let delta = self.last_run.map(|x| now.duration_since(x).as_secs_f32());
        // New CPU totals
        let mut new_cpu_totals = HashMap::default();
        // Configure query
        let query_conf = QueryConf {
            cmd: self.expose_cmd || self.expose_meta_cmd,
            env: self.expose_meta_env,
        };
        // Collect before users lock
        let stats = Ps::get_stats(&all_pids, &query_conf);
        // Grab users lock
        // @todo: Grab only when user label exposed
        let users_cache = USERS_CACHE.lock().await;
        for stat in stats.into_iter() {
            // Build labels
            let mut labels = Labels::default();
            if let Some(process_name) = &stat.process_name {
                labels.push(Label::new("process_name", process_name.to_owned()))
            }
            if self.expose_user {
                if let Some(uid) = stat.uid {
                    // Try to use cached or resolve
                    if let Some(user) = users_cache.get_user_by_uid(uid) {
                        labels.push(Label::new(
                            "user",
                            user.name().to_string_lossy().to_string(),
                        ));
                    }
                }
            }
            if self.expose_cmd || self.expose_meta_cmd {
                if let Some(cmd) = &stat.cmd {
                    let c = cmd.join(&self.cmd_separator);
                    if self.expose_cmd {
                        labels.push(Label::new("cmd", c.clone()));
                    }
                    if self.expose_meta_cmd {
                        labels.push(Label::new("__meta_cmd", c));
                    }
                }
            }
            if self.expose_meta_env {
                if let Some(env) = &stat.env {
                    labels.push(Label::new("__meta_env", env.join(&self.env_separator)));
                }
            }
            //
            apply_some!(r, stat.num_fds, ps_num_fds, labels.clone());
            apply_some!(r, stat.num_threads, ps_num_threads, labels.clone());
            // ctx
            apply_some!(
                r,
                stat.voluntary_context_switches,
                ps_voluntary_context_switches,
                labels.clone()
            );
            apply_some!(
                r,
                stat.involuntary_context_switches,
                ps_involuntary_context_switches,
                labels.clone()
            );
            // Page faults
            apply_some!(r, stat.minor_faults, ps_minor_faults, labels.clone());
            apply_some!(r, stat.major_faults, ps_major_faults, labels.clone());
            apply_some!(
                r,
                stat.child_minor_faults,
                ps_child_minor_faults,
                labels.clone()
            );
            apply_some!(
                r,
                stat.child_major_faults,
                ps_child_major_faults,
                labels.clone()
            );
            // CPU
            apply_some!(r, stat.cpu_time_user, ps_cpu_time_user, labels.clone());
            apply_some!(r, stat.cpu_time_system, ps_cpu_time_system, labels.clone());
            apply_some!(r, stat.cpu_time_iowait, ps_cpu_time_iowait, labels.clone());
            // CPU Totals
            if let Some(total) = stat.cpu_total() {
                if let Some(dt) = delta {
                    // At least one run
                    if let Some(last_total) = self.cpu_totals.get(&stat.pid) {
                        // And already registered
                        r.push(ps_cpu_usage(
                            (total - last_total) * 100.0 / dt,
                            labels.clone(),
                        ))
                    }
                }
                new_cpu_totals.insert(stat.pid, total); // Remember new values
            }
            // Memory
            apply_some!(r, stat.mem_total, ps_mem_total, labels.clone());
            apply_some!(r, stat.mem_rss, ps_mem_rss, labels.clone());
            apply_some!(r, stat.mem_swap, ps_mem_swap, labels.clone());
            apply_some!(r, stat.mem_data, ps_mem_data, labels.clone());
            apply_some!(r, stat.mem_stack, ps_mem_stack, labels.clone());
            apply_some!(r, stat.mem_text, ps_mem_text, labels.clone());
            apply_some!(r, stat.mem_lib, ps_mem_lib, labels.clone());
            apply_some!(r, stat.mem_locked, ps_mem_locked, labels.clone());
            // I/O
            apply_some!(r, stat.io_read_count, ps_read_count, labels.clone());
            apply_some!(r, stat.io_write_count, ps_write_count, labels.clone());
            apply_some!(r, stat.io_read_bytes, ps_read_bytes, labels.clone());
            apply_some!(r, stat.io_write_bytes, ps_write_bytes, labels.clone());
        }
        self.last_run = Some(now);
        self.cpu_totals = new_cpu_totals;
        // Push result
        Ok(r)
    }
    // Setup self-monitoring
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config {
            pid_file: None,
            pattern: None,
            self_pid: Some(true),
            expose_labels: None,
            cmd_separator: DEFAULT_CMD_SEPARATOR.to_string(),
            env_separator: DEFAULT_ENV_SEPARATOR.to_string(),
        };
        Ok(vec![ConfigItem::from_config(cfg)?])
    }
}

impl Collector {
    // Read Pid from file
    fn read_pid_file(path: &String) -> AgentResult<Pid> {
        // Read file
        let data = read_to_string(path).map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Parse and return
        data.parse()
            .map_err(|_| AgentError::InternalError("failed to parse pid file".to_string()))
    }
    // Apply pids to hash map
    fn apply_pids(map: &mut HashSet<Pid>, pids: Vec<Pid>) {
        for pid in pids.into_iter() {
            map.insert(pid);
        }
    }
}

fn default_cmd_separator() -> String {
    DEFAULT_CMD_SEPARATOR.into()
}

fn default_env_separator() -> String {
    DEFAULT_ENV_SEPARATOR.into()
}

fn is_default_cmd_separator(s: &String) -> bool {
    s == DEFAULT_CMD_SEPARATOR
}

fn is_default_env_separator(s: &String) -> bool {
    s == DEFAULT_ENV_SEPARATOR
}
