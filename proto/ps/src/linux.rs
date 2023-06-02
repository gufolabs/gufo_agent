// --------------------------------------------------------------------
// Gufo Agent: process utilities, linux-specific implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use crate::{ProcStat, PsFinder};
use common::{AgentError, AgentResult};
use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, line_ending, space1},
    combinator::{eof, map_res},
    multi::{many0, separated_list1},
    sequence::pair,
    IResult,
};
use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use sysconf::{pagesize, sysconf, SysconfVariable};

pub type Pid = u32;
pub type Uid = u32;
pub type Gid = u32;
pub struct Ps;

struct SysConf {
    page_size: u64,
    tick: f32,
    n_cpu: usize,
}

impl Default for SysConf {
    fn default() -> Self {
        // Get Page Size
        let page_size = pagesize() as u64;
        // Get Tick
        let tick = sysconf(SysconfVariable::ScClkTck).unwrap_or(100) as f32;
        // Number of CPU online
        let n_cpu = sysconf(SysconfVariable::ScNprocessorsOnln).unwrap_or(1) as usize;
        Self {
            page_size,
            tick,
            n_cpu,
        }
    }
}

lazy_static! {
    static ref SYSCONF: SysConf = SysConf::default();
}

impl PsFinder for Ps {
    fn pids() -> Result<Vec<Pid>, AgentError> {
        println!(
            "SYSCONF page_size={} tick={}",
            SYSCONF.page_size, SYSCONF.tick
        );
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
    // Returns own pid
    fn filter_by_self() -> Result<Vec<Pid>, AgentError> {
        Ok(vec![std::process::id()])
    }
    // Get stats for given pids
    fn get_stats(pids: &HashSet<Pid>) -> Vec<ProcStat> {
        let mut r = Vec::with_capacity(pids.len());
        const KB: u64 = 1024;
        for &pid in pids.iter() {
            let mut stats = ProcStat {
                pid,
                num_fds: get_num_fds(pid),
                cpu_online: SYSCONF.n_cpu,
                ..Default::default()
            };
            // Process /proc/<pid>/stat
            // See man 5 proc for details
            if let Some(data) = read_procfs(pid, "stat") {
                let parts: Vec<&str> = data.split(' ').collect();
                // process name
                let pn = parts[STAT_COMM];
                let process_name = if pn.starts_with('(') && pn.ends_with(')') {
                    String::from(&pn[1..pn.len() - 1])
                } else {
                    String::from(pn)
                };
                if !process_name.is_empty() {
                    stats.process_name = Some(process_name)
                }
                stats.num_threads = parse_field(parts[STAT_NUM_THREADS]);
                // faults
                stats.minor_faults = parse_field(parts[STAT_MINFLT]);
                stats.major_faults = parse_field(parts[STAT_MAJFLT]);
                stats.child_minor_faults = parse_field(parts[STAT_CMINFLT]);
                stats.child_major_faults = parse_field(parts[STAT_CMAJFLT]);
                // cpu
                stats.cpu_time_user = parse_field(parts[STAT_UTIME]).map(|x: f32| x / SYSCONF.tick);
                stats.cpu_time_system =
                    parse_field(parts[STAT_STIME]).map(|x: f32| x / SYSCONF.tick);
                if parts.len() >= STAT_DELAYACCT_BLKIO_TICKS {
                    // Linux 2.6.18
                    stats.cpu_time_iowait = parse_field(parts[STAT_DELAYACCT_BLKIO_TICKS])
                        .map(|x: f32| x / SYSCONF.tick);
                }
            }
            // Process /proc/<pid>/statm
            // See man 5 proc for detaults
            if let Some(data) = read_procfs(pid, "statm") {
                let parts: Vec<&str> = data.split(' ').collect();
                stats.mem_total =
                    parse_field(parts[STATM_SIZE]).map(|x: u64| x * SYSCONF.page_size);
                stats.mem_rss =
                    parse_field(parts[STATM_RESIDENT]).map(|x: u64| x * SYSCONF.page_size);
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
            // Process /proc/<pid>/status
            // See man 5 proc for detaults
            if let Some(data) = read_procfs(pid, "status") {
                if let Ok(items) = parse_status(data.as_str()) {
                    for (k, v) in items.into_iter() {
                        match k {
                            "Uid" => stats.uid = v.map(|x| x as Uid),
                            "Gid" => stats.gid = v.map(|x| x as Gid),
                            "VmSwap" => stats.mem_swap = v.map(|x| x * KB),
                            "VmData" => stats.mem_data = v.map(|x| x * KB),
                            "VmStk" => stats.mem_stack = v.map(|x| x * KB),
                            "VmLck" => stats.mem_locked = v.map(|x| x * KB),
                            "VmExe" => stats.mem_text = v.map(|x| x * KB),
                            "VmLib" => stats.mem_lib = v.map(|x| x * KB),
                            "voluntary_ctxt_switches" => stats.voluntary_context_switches = v,
                            "nonvoluntary_ctxt_switches" => stats.involuntary_context_switches = v,
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
    let (input, value) = map_res(digit1, |x: &str| x.parse::<u64>())(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, (t, value)))
}

fn parse_io(input: &str) -> AgentResult<Vec<(&str, u64)>> {
    let (_, r) =
        many0(parse_io_line)(input).map_err(|e| AgentError::InternalError(e.to_string()))?;
    Ok(r)
}

// /proc/<pid>/status parser
fn parse_status_kb(input: &str) -> IResult<&str, Option<u64>> {
    let (input, value) = map_res(digit1, |x: &str| x.parse::<u64>())(input)?;
    let (input, _) = pair(space1, tag("kB"))(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, Some(value)))
}

fn parse_status_num(input: &str) -> IResult<&str, Option<u64>> {
    let (input, value) = map_res(digit1, |x: &str| x.parse::<u64>())(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, Some(value)))
}

// Parse 4-digits of uid/gid
fn parse_status_uids(input: &str) -> IResult<&str, Option<u64>> {
    let (input, uids) =
        separated_list1(space1, map_res(digit1, |x: &str| x.parse::<u64>()))(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, Some(uids[0])))
}

fn parse_status_str(input: &str) -> IResult<&str, Option<u64>> {
    let (input, _) = is_not("\n")(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;
    Ok((input, None))
}

fn parse_status_line(input: &str) -> IResult<&str, (&str, Option<u64>)> {
    // tag
    let (input, t) = is_not(":")(input)?;
    // :
    let (input, _) = tag(":")(input)?;
    // <space>+
    let (input, _) = space1(input)?;
    // value
    let (input, value) = alt((
        parse_status_kb,
        parse_status_num,
        parse_status_uids,
        parse_status_str,
    ))(input)?;
    Ok((input, (t, value)))
}

fn parse_status(input: &str) -> AgentResult<Vec<(&str, Option<u64>)>> {
    let (_, r) =
        many0(parse_status_line)(input).map_err(|e| AgentError::InternalError(e.to_string()))?;
    Ok(r)
}

// Constants
// /proc/<pid>/stat fields
// const STAT_PID: usize = 0;
const STAT_COMM: usize = 1;
// const STAT_STATE:  usize = 2;
// const STAT_PPID: usize = 3;
// const STAT_PGRP: usize = 4;
// const STAT_SESSION: usize = 5;
// const STAT_TTY_NR: usize = 6;
// const STAT_TPGID: usize = 7;
// const STAT_FLAGS: usize = 8;
const STAT_MINFLT: usize = 9;
const STAT_CMINFLT: usize = 10;
const STAT_MAJFLT: usize = 11;
const STAT_CMAJFLT: usize = 12;
const STAT_UTIME: usize = 13;
const STAT_STIME: usize = 14;
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
const STAT_DELAYACCT_BLKIO_TICKS: usize = 41;
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

// /proc/pid/statm fields
const STATM_SIZE: usize = 0;
const STATM_RESIDENT: usize = 1;
// const STATM_SHARED: usize = 2;
// const STATM_TEXT: usize = 3;
// const STATM_LIB: usize = 4;
// const STATM_DATA: usize = 5;
// const STATM_DT: usize = 6;

#[cfg(test)]
mod tests {
    use super::{parse_io, parse_status};
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
    #[test]
    fn test_status_parser() {
        let s = r#"Name:   cat
Umask:  0022
State:  R (running)
Tgid:   25928
Ngid:   0
Pid:    25928
PPid:   528
TracerPid:      0
Uid:    0       0       0       0
Gid:    0       0       0       0
FDSize: 256
Groups: 0 
NStgid: 25928
NSpid:  25928
NSpgid: 25928
NSsid:  528
VmPeak:     4424 kB
VmSize:     4424 kB
VmLck:         0 kB
VmPin:         0 kB
VmHWM:       568 kB
VmRSS:       568 kB
RssAnon:              64 kB
RssFile:             504 kB
RssShmem:              0 kB
VmData:      312 kB
VmStk:       132 kB
VmExe:        20 kB
VmLib:      1520 kB
VmPTE:        44 kB
VmSwap:        0 kB
HugetlbPages:          0 kB
CoreDumping:    0
THP_enabled:    1
Threads:        1
SigQ:   1/23293
SigPnd: 0000000000000000
ShdPnd: 0000000000000000
SigBlk: 0000000000000000
SigIgn: 0000000000000000
SigCgt: 0000000000000000
CapInh: 0000000000000000
CapPrm: 00000000a80425fb
CapEff: 00000000a80425fb
CapBnd: 00000000a80425fb
CapAmb: 0000000000000000
NoNewPrivs:     0
Seccomp:        2
Seccomp_filters:        2
Speculation_Store_Bypass:       vulnerable
SpeculationIndirectBranch:      always enabled
Cpus_allowed:   f
Cpus_allowed_list:      0-3
Mems_allowed:   1
Mems_allowed_list:      0
voluntary_ctxt_switches:        1
nonvoluntary_ctxt_switches:     2"#;
        let r = parse_status(s).unwrap();
        assert_eq!(
            r,
            vec![
                ("Name", None),
                ("Umask", Some(22)),
                ("State", None),
                ("Tgid", Some(25928)),
                ("Ngid", Some(0)),
                ("Pid", Some(25928)),
                ("PPid", Some(528)),
                ("TracerPid", Some(0)),
                ("Uid", Some(0)),
                ("Gid", Some(0)),
                ("FDSize", Some(256)),
                ("Groups", None),
                ("NStgid", Some(25928)),
                ("NSpid", Some(25928)),
                ("NSpgid", Some(25928)),
                ("NSsid", Some(528)),
                ("VmPeak", Some(4424)),
                ("VmSize", Some(4424)),
                ("VmLck", Some(0)),
                ("VmPin", Some(0)),
                ("VmHWM", Some(568)),
                ("VmRSS", Some(568)),
                ("RssAnon", Some(64)),
                ("RssFile", Some(504)),
                ("RssShmem", Some(0)),
                ("VmData", Some(312)),
                ("VmStk", Some(132)),
                ("VmExe", Some(20)),
                ("VmLib", Some(1520)),
                ("VmPTE", Some(44)),
                ("VmSwap", Some(0)),
                ("HugetlbPages", Some(0)),
                ("CoreDumping", Some(0)),
                ("THP_enabled", Some(1)),
                ("Threads", Some(1)),
                ("SigQ", None),
                ("SigPnd", Some(0)),
                ("ShdPnd", Some(0)),
                ("SigBlk", Some(0)),
                ("SigIgn", Some(0)),
                ("SigCgt", Some(0)),
                ("CapInh", Some(0)),
                ("CapPrm", None),
                ("CapEff", None),
                ("CapBnd", None),
                ("CapAmb", Some(0)),
                ("NoNewPrivs", Some(0)),
                ("Seccomp", Some(2)),
                ("Seccomp_filters", Some(2)),
                ("Speculation_Store_Bypass", None),
                ("SpeculationIndirectBranch", None),
                ("Cpus_allowed", None),
                ("Cpus_allowed_list", None),
                ("Mems_allowed", Some(1)),
                ("Mems_allowed_list", Some(0)),
                ("voluntary_ctxt_switches", Some(1)),
                ("nonvoluntary_ctxt_switches", Some(2))
            ]
        );
    }
}
