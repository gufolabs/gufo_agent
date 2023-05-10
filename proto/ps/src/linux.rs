// --------------------------------------------------------------------
// Gufo Agent: process utilities, linux-specific implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{Pid, ProcStat, PsFinder};
use common::{AgentError, AgentResult};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, line_ending},
    combinator::eof,
    multi::many0,
    IResult,
};
use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub struct Ps;

impl PsFinder for Ps {
    fn pids() -> Result<Vec<Pid>, AgentError> {
        let mut r = Vec::new();
        for entry in read_dir("/proc")
            .map_err(|e| AgentError::InternalError(e.to_string()))?
            .flatten()
        {
            let filename = entry.file_name();
            if let Ok(pid) = filename.to_string_lossy().parse::<Pid>() {
                r.push(pid);
            }
        }
        Ok(r)
    }
    fn has_pid(pid: Pid) -> bool {
        Path::new("/proc").join(pid.to_string()).is_dir()
    }
    fn cmdline(pid: Pid) -> Option<Vec<String>> {
        read_procfs(pid, "cmdline")
            .map(|data| data.split_terminator('\0').map(|x| x.to_string()).collect())
    }
    // Get stats for given pids
    fn get_stats(pids: &HashSet<Pid>) -> Vec<ProcStat> {
        let p_size = page_size::get() as u64;
        let mut r = Vec::with_capacity(pids.len());
        for &pid in pids.iter() {
            let mut stats = ProcStat::default();
            stats.pid = pid;
            // num_fds
            stats.num_fds = get_num_fds(pid);
            // Process /proc/<pid>/stat
            // See man 5 proc for details
            if let Some(data) = read_procfs(pid, "stat") {
                let parts: Vec<&str> = data.split(' ').collect();
                stats.num_threads = parse_field(parts[STAT_NUM_THREADS]);
            }
            // Process /proc/<pid>/statm
            // See man 5 proc for detaults
            if let Some(data) = read_procfs(pid, "statm") {
                let parts: Vec<&str> = data.split(' ').collect();
                stats.mem_total = parse_field(parts[STATM_SIZE]).map(|x: u64| x * p_size);
                stats.mem_rss = parse_field(parts[STATM_RESIDENT]).map(|x: u64| x * p_size);
            }
            // Process /proc/<pid>/io
            // See man 5 proc for detaults
            if let Some(data) = read_procfs(pid, "io") {
                if let Ok(items) = parse_io(data.as_str()) {
                    for (k, v) in items.into_iter() {
                        match k {
                            "syscr" => stats.io_read_count = Some(v),
                            "syscw" => stats.io_write_count = Some(v),
                            "read_bytes" => stats.io_read_bytes = Some(v),
                            "write_bytes" => stats.io_write_bytes = Some(v),
                            _ => {}
                        }
                    }
                }
            }
            //
            r.push(stats);
        }
        r
    }
}

fn procfs_path(pid: Pid, name: &str) -> PathBuf {
    Path::new("/proc").join(pid.to_string()).join(name)
}
// Read procfs file
fn read_procfs(pid: Pid, name: &str) -> Option<String> {
    match read_to_string(procfs_path(pid, name)) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}
// Calculate number of fd from /proc/<pid>/fd
fn get_num_fds(pid: Pid) -> Option<u64> {
    match read_dir(procfs_path(pid, "fd")) {
        Ok(dirlist) => Some(dirlist.flatten().count() as u64),
        Err(_) => None,
    }
}
//
fn parse_field<T: FromStr>(s: &str) -> Option<T> {
    match s.parse() {
        Ok(x) => Some(x),
        Err(_) => None,
    }
}

// proc/<pid>/io parser
fn parse_io_line(input: &str) -> IResult<&str, (&str, u64)> {
    let (input, t) = is_not(":")(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, value) = digit1(input)?;
    let pv = value.parse().unwrap();
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, (t, pv)))
}

fn parse_io(input: &str) -> AgentResult<Vec<(&str, u64)>> {
    let (_, r) =
        many0(parse_io_line)(input).map_err(|e| AgentError::InternalError(e.to_string()))?;
    Ok(r)
}

// Constants
// stat fields
// const STAT_PID: usize = 0;
// const STAT_COMM: usize = 1;
// const STAT_STATE:  usize = 2;
// const STAT_PPID: usize = 3;
// const STAT_PGRP: usize = 4;
// const STAT_SESSION: usize = 5;
// const STAT_TTY_NR: usize = 6;
// const STAT_TPGID: usize = 7;
// const STAT_FLAGS: usize = 8;
// const STAT_MINFLT: usize = 9;
// const STAT_CMINFLT: usize = 10;
// const STAT_MAJFLT: usize = 11;
// const STAT_CMAJFLT: usize = 12;
// const STAT_UTIME: usize = 13;
// const STAT_STIME: usize = 14;
// const STAT_CUTIME: usize = 15;
// const STAT_CSTIME: usize = 16;
// const STAT_PRIORITY: usize = 17;
// const STAT_NICE: usize = 18;
const STAT_NUM_THREADS: usize = 19;
// const STAT_ITREALVALUE: usize = 20;
// const STAT_STARTTIME: usize = 21;
// const STAT_VSIZE: usize = 22;
// const STAT_RSS: usize = 23;
// const STAT_RSSLIM: usize = 24;
// const STAT_STARTCODE: usize = 25;
// const STAT_ENDCODE: usize = 26;
// const STAT_STARTSTACK: usize = 27;
// const STAT_KSTKESP: usize = 28;
// const STAT_KSTKEIP: usize = 29;
// const STAT_SIGNAL: usize = 30;
// const STAT_BLOCKED: usize = 31;
// const STAT_SIGIGNORE: usize = 32;
// const STAT_SIGCATCH: usize = 33;
// const STAT_WCHAN: usize = 34;
// const STAT_NSWAP: usize = 35;
// const STAT_CNSWAP: usize = 36;
// const STAT_EXIT_SIGNAL: usize = 37;
// const STAT_PROCESSOR: usize = 38;
// const STAT_RT_PRIORITY: usize = 39;
// const STAT_POLICY: usize = 40;
// const STAT_DELAYACCT_BLKIO_TICKS: usize = 41;
// const STAT_GUEST_TIME: usize = 42;
// const STAT_CGUEST_TIME: usize = 43;
// const STAT_START_DATA: usize = 44;
// const STAT_END_DATA: usize = 45;
// const STAT_START_BRK: usize = 46;
// const STAT_ARG_START: usize = 47;
// const STAT_ARG_END: usize = 48;
// const STAT_ENV_START: usize = 49;
// const STAT_ENV_END: usize = 50;
// const STAT_EXIT_CODE: usize = 51;
// statm fields
const STATM_SIZE: usize = 0;
const STATM_RESIDENT: usize = 1;
// const STATM_SHARED: usize = 2;
// const STATM_TEXT: usize = 3;
// const STATM_LIB: usize = 4;
// const STATM_DATA: usize = 5;
// const STATM_DT: usize = 6;

#[cfg(test)]
mod tests {
    use super::parse_io;
    #[test]
    fn test_io_parser() {
        let s = r#"rchar: 2300
wchar: 0
syscr: 7
syscw: 0
read_bytes: 45056
write_bytes: 0
cancelled_write_bytes: 0
"#;
        let r = parse_io(s).unwrap();
        assert_eq!(
            r,
            vec![
                ("rchar", 2300),
                ("wchar", 0),
                ("syscr", 7),
                ("syscw", 0),
                ("read_bytes", 45056),
                ("write_bytes", 0),
                ("cancelled_write_bytes", 0)
            ]
        );
    }
}
