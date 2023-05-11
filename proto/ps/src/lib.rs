// --------------------------------------------------------------------
// Gufo Agent: process utilities
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use common::AgentError;
use regex::Regex;
use std::collections::HashSet;

pub type Pid = u32;

#[derive(Default, Debug)]
pub struct ProcStat {
    pub pid: Pid,
    pub num_threads: Option<u64>,
    pub num_fds: Option<u64>,
    // ctx switch
    pub voluntary_context_switches: Option<u64>,
    pub involuntary_context_switches: Option<u64>,
    // page faults
    pub minor_faults: Option<u64>,
    pub child_minor_faults: Option<u64>,
    pub major_faults: Option<u64>,
    pub child_major_faults: Option<u64>,
    // cpu
    pub cpu_time_user: Option<f32>,
    pub cpu_time_system: Option<f32>,
    pub cpu_time_iowait: Option<f32>,
    // memory
    pub mem_rss: Option<u64>,
    pub mem_total: Option<u64>,
    pub mem_swap: Option<u64>,
    pub mem_data: Option<u64>,
    pub mem_stack: Option<u64>,
    pub mem_locked: Option<u64>,
    pub mem_text: Option<u64>,
    pub mem_lib: Option<u64>,
    // I/O
    pub io_read_count: Option<u64>,
    pub io_write_count: Option<u64>,
    pub io_read_bytes: Option<u64>,
    pub io_write_bytes: Option<u64>,
}

pub trait PsFinder {
    // Enumerate all running processes
    fn pids() -> Result<Vec<Pid>, AgentError>;
    // Check if process is exists
    fn has_pid(pid: Pid) -> bool;
    // Get process command line, if exists
    fn cmdline(pid: Pid) -> Option<Vec<String>>;
    // Returns own pid
    fn filter_by_self() -> Result<Vec<Pid>, AgentError>;
    // Returns process with given Pid, if exists
    fn filter_by_pid(pid: Pid) -> Result<Vec<Pid>, AgentError> {
        Ok(if Self::has_pid(pid) {
            vec![pid]
        } else {
            vec![]
        })
    }
    // Returns processes matching given command line pattern
    fn filter_by_pattern(pattern: &Regex) -> Result<Vec<Pid>, AgentError> {
        Ok(Self::pids()?
            .into_iter()
            .filter(|p| match Self::cmdline(*p) {
                Some(args) => args.into_iter().any(|a| pattern.is_match(a.as_str())),
                None => false,
            })
            .collect())
    }
    // Get stats for given pids
    fn get_stats(pids: &HashSet<Pid>) -> Vec<ProcStat>;
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::Ps;
