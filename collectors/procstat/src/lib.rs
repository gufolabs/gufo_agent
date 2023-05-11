// --------------------------------------------------------------------
// Gufo Agent: procstat collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{
    counter, gauge, gauge_f, AgentError, AgentResult, Collectable, ConfigDiscoveryOpts, ConfigItem,
    Measure,
};
use ps::{Pid, Ps, PsFinder};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::read_to_string;

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pid_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_pid: Option<bool>,
}

// Collector structure
pub struct Collector {
    pid_file: Option<String>,
    pattern: Option<Regex>,
    self_pid: bool,
}

// Generated metrics
gauge!(ps_num_fds, "Number of open files");
gauge!(ps_num_threads, "Number of threads");
// ctx switches
counter!(
    ps_voluntary_context_switches,
    "Total voluntary context switches"
);
counter!(
    ps_involuntary_context_switches,
    "Total involuntary context switches"
);
// page faults
counter!(
    ps_minor_faults,
    "Total number of minor faults which do not requirie loading memory from disk"
);
counter!(
    ps_major_faults,
    "Total number of major faults which require loading memory from disk"
);
counter!(
    ps_child_minor_faults,
    "Total number of minor faults that process waited-for children made"
);
counter!(
    ps_child_major_faults,
    "Total number of major faults that process waited-for children made"
);
// CPU
gauge_f!(ps_cpu_time_user, "CPU time in user mode in seconds");
gauge_f!(ps_cpu_time_system, "CPU time in system mode in seconds");
gauge_f!(ps_cpu_time_iowait, "CPU time iowait in seconds");
// Mem
gauge!(ps_mem_total, "Total memory");
gauge!(ps_mem_rss, "Resident set size");
gauge!(ps_mem_swap, "Swapped-out virtual memory size");
gauge!(ps_mem_data, "Data segment size");
gauge!(ps_mem_stack, "Stack segment size");
gauge!(ps_mem_text, "Text segment size");
gauge!(ps_mem_lib, "Shared library code size");
gauge!(ps_mem_locked, "Locked memory size");
// I/O
counter!(ps_read_count, "Total read I/O operations");
counter!(ps_write_count, "Total write I/O operations");
counter!(ps_read_bytes, "Total bytes read");
counter!(ps_write_bytes, "Total bytes written");

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
        //
        Ok(Self {
            pid_file: value.pid_file,
            pattern: pattern,
            self_pid,
        })
    }
}

macro_rules! apply_some {
    ($r:ident, $v:expr, $fn:ident) => {
        if let Some(x) = $v {
            $r.push($fn(x));
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
        for stat in Ps::get_stats(&all_pids).into_iter() {
            apply_some!(r, stat.num_fds, ps_num_fds);
            apply_some!(r, stat.num_threads, ps_num_threads);
            // ctx
            apply_some!(
                r,
                stat.voluntary_context_switches,
                ps_voluntary_context_switches
            );
            apply_some!(
                r,
                stat.involuntary_context_switches,
                ps_involuntary_context_switches
            );
            // Page faults
            apply_some!(r, stat.minor_faults, ps_minor_faults);
            apply_some!(r, stat.major_faults, ps_major_faults);
            apply_some!(r, stat.child_minor_faults, ps_child_minor_faults);
            apply_some!(r, stat.child_major_faults, ps_child_major_faults);
            // CPU
            apply_some!(r, stat.cpu_time_user, ps_cpu_time_user);
            apply_some!(r, stat.cpu_time_system, ps_cpu_time_system);
            apply_some!(r, stat.cpu_time_iowait, ps_cpu_time_iowait);
            // Memory
            apply_some!(r, stat.mem_total, ps_mem_total);
            apply_some!(r, stat.mem_rss, ps_mem_rss);
            apply_some!(r, stat.mem_swap, ps_mem_swap);
            apply_some!(r, stat.mem_data, ps_mem_data);
            apply_some!(r, stat.mem_stack, ps_mem_stack);
            apply_some!(r, stat.mem_text, ps_mem_text);
            apply_some!(r, stat.mem_lib, ps_mem_lib);
            apply_some!(r, stat.mem_locked, ps_mem_locked);
            // I/O
            apply_some!(r, stat.io_read_count, ps_read_count);
            apply_some!(r, stat.io_write_count, ps_write_count);
            apply_some!(r, stat.io_read_bytes, ps_read_bytes);
            apply_some!(r, stat.io_write_bytes, ps_write_bytes);
        }
        // Push result
        Ok(r)
    }
    // Setup self-monitoring
    fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
        let cfg = Config {
            pid_file: None,
            pattern: None,
            self_pid: Some(true),
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
