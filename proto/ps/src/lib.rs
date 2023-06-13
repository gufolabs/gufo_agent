// --------------------------------------------------------------------
// Gufo Agent: process utilities
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use common::AgentError;
use regex::Regex;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct ProcStat {
    pub pid: Pid,
    pub uid: Option<Uid>,
    pub gid: Option<Gid>,
    pub process_name: Option<String>,
    pub cmd: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
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
    pub cpu_online: usize,
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

#[derive(Debug, Default)]
pub struct QueryConf {
    // Fill cmd field
    pub cmd: bool,
    // Fill env field
    pub env: bool,
}

impl ProcStat {
    pub fn cpu_total(&self) -> Option<f32> {
        if self.cpu_time_user.is_none()
            && self.cpu_time_system.is_none()
            && self.cpu_time_iowait.is_none()
        {
            None
        } else {
            Some(
                self.cpu_time_user.unwrap_or_default()
                    + self.cpu_time_system.unwrap_or_default()
                    + self.cpu_time_iowait.unwrap_or_default(),
            )
        }
    }
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
    fn get_stats(pids: &HashSet<Pid>, conf: &QueryConf) -> Vec<ProcStat>;
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{Gid, Pid, Ps, Uid};
