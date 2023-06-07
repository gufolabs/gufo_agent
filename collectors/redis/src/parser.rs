// --------------------------------------------------------------------
// Gufo Agent: redis INFO parser
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use common::{AgentError, AgentResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, line_ending, not_line_ending},
    combinator::{opt, recognize},
    multi::{many0, many0_count},
    sequence::{pair, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
pub(crate) enum InfoItem<'a> {
    Value(&'a str, InfoValue<'a>),
    Ignored,
}

#[derive(Debug)]
pub(crate) enum InfoValue<'a> {
    Int(i64),
    Float(f32),
    Str(&'a str),
}

pub(crate) fn parse(input: &str) -> AgentResult<Vec<InfoItem<'_>>> {
    let (_, r) = many0(parse_line)(input).map_err(|e| AgentError::ParseError(e.to_string()))?;
    Ok(r)
}

fn parse_line(input: &str) -> IResult<&str, InfoItem<'_>> {
    let (input, r) = alt((parse_key_value, skip_line))(input)?;
    Ok((input, r))
}

fn parse_key_value(input: &str) -> IResult<&str, InfoItem<'_>> {
    let (input, (k, v)) = separated_pair(
        // Name
        recognize(pair(alpha1, many0_count(alt((alphanumeric1, tag("_")))))),
        tag(":"),
        parse_value,
    )(input)?;
    Ok((input, InfoItem::Value(k, v)))
}

fn parse_value(input: &str) -> IResult<&str, InfoValue<'_>> {
    let (input, value) = alt((parse_int, parse_float, parse_str))(input)?;
    Ok((input, value))
}

fn parse_int(input: &str) -> IResult<&str, InfoValue<'_>> {
    let (input, value) = recognize(tuple((opt(tag("-")), digit1)))(input)?;
    let (input, _) = end(input)?;
    Ok((input, InfoValue::Int(value.parse().unwrap_or(0))))
}

fn parse_float(input: &str) -> IResult<&str, InfoValue<'_>> {
    let (input, value) = recognize(tuple((opt(tag("-")), digit1, tag("."), digit1)))(input)?;
    let (input, _) = end(input)?;
    Ok((input, InfoValue::Float(value.parse().unwrap_or(0.0))))
}

fn parse_str(input: &str) -> IResult<&str, InfoValue<'_>> {
    let (input, value) = not_line_ending(input)?;
    let (input, _) = end(input)?;
    Ok((input, InfoValue::Str(value)))
}

fn skip_line(input: &str) -> IResult<&str, InfoItem<'_>> {
    let (input, _) = not_line_ending(input)?;
    let (input, _) = end(input)?;
    Ok((input, InfoItem::Ignored))
}

fn end(input: &str) -> IResult<&str, ()> {
    let (input, _) = line_ending(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::parse;

    const INFO: &str = r#"# Server
redis_version:7.0.11
redis_git_sha1:00000000
redis_git_dirty:0
redis_build_id:c87ff843cceeb98e
redis_mode:standalone
os:Linux 5.15.49-linuxkit x86_64
arch_bits:64
monotonic_clock:POSIX clock_gettime
multiplexing_api:epoll
atomicvar_api:c11-builtin
gcc_version:10.2.1
process_id:1
process_supervised:no
run_id:e5ece7b7923ace305414ca737613438fc6f5bcc2
tcp_port:6379
server_time_usec:1686056966422751
uptime_in_seconds:119
uptime_in_days:0
hz:10
configured_hz:10
lru_clock:8335366
executable:/data/redis-server
config_file:
io_threads_active:0

# Clients
connected_clients:1
cluster_connections:0
maxclients:10000
client_recent_max_input_buffer:0
client_recent_max_output_buffer:0
blocked_clients:0
tracking_clients:0
clients_in_timeout_table:0

# Memory
used_memory:902672
used_memory_human:881.52K
used_memory_rss:8458240
used_memory_rss_human:8.07M
used_memory_peak:902672
used_memory_peak_human:881.52K
used_memory_peak_perc:102.60%
used_memory_overhead:862232
used_memory_startup:862048
used_memory_dataset:40440
used_memory_dataset_perc:99.55%
allocator_allocated:1058424
allocator_active:1249280
allocator_resident:4104192
total_system_memory:6232166400
total_system_memory_human:5.80G
used_memory_lua:31744
used_memory_vm_eval:31744
used_memory_lua_human:31.00K
used_memory_scripts_eval:0
number_of_cached_scripts:0
number_of_functions:0
number_of_libraries:0
used_memory_vm_functions:32768
used_memory_vm_total:64512
used_memory_vm_total_human:63.00K
used_memory_functions:184
used_memory_scripts:184
used_memory_scripts_human:184B
maxmemory:0
maxmemory_human:0B
maxmemory_policy:noeviction
allocator_frag_ratio:1.18
allocator_frag_bytes:190856
allocator_rss_ratio:3.29
allocator_rss_bytes:2854912
rss_overhead_ratio:2.06
rss_overhead_bytes:4354048
mem_fragmentation_ratio:9.81
mem_fragmentation_bytes:7596048
mem_not_counted_for_evict:0
mem_replication_backlog:0
mem_total_replication_buffers:0
mem_clients_slaves:0
mem_clients_normal:0
mem_cluster_links:0
mem_aof_buffer:0
mem_allocator:jemalloc-5.2.1
active_defrag_running:0
lazyfree_pending_objects:0
lazyfreed_objects:0

# Persistence
loading:0
async_loading:0
current_cow_peak:0
current_cow_size:0
current_cow_size_age:0
current_fork_perc:0.00
current_save_keys_processed:0
current_save_keys_total:0
rdb_changes_since_last_save:0
rdb_bgsave_in_progress:0
rdb_last_save_time:1686056847
rdb_last_bgsave_status:ok
rdb_last_bgsave_time_sec:-1
rdb_current_bgsave_time_sec:-1
rdb_saves:0
rdb_last_cow_size:0
rdb_last_load_keys_expired:0
rdb_last_load_keys_loaded:0
aof_enabled:0
aof_rewrite_in_progress:0
aof_rewrite_scheduled:0
aof_last_rewrite_time_sec:-1
aof_current_rewrite_time_sec:-1
aof_last_bgrewrite_status:ok
aof_rewrites:0
aof_rewrites_consecutive_failures:0
aof_last_write_status:ok
aof_last_cow_size:0
module_fork_in_progress:0
module_fork_last_cow_size:0

# Stats
total_connections_received:1
total_commands_processed:0
instantaneous_ops_per_sec:0
total_net_input_bytes:14
total_net_output_bytes:0
total_net_repl_input_bytes:0
total_net_repl_output_bytes:0
instantaneous_input_kbps:0.00
instantaneous_output_kbps:0.00
instantaneous_input_repl_kbps:0.00
instantaneous_output_repl_kbps:0.00
rejected_connections:0
sync_full:0
sync_partial_ok:0
sync_partial_err:0
expired_keys:0
expired_stale_perc:0.00
expired_time_cap_reached_count:0
expire_cycle_cpu_milliseconds:2
evicted_keys:0
evicted_clients:0
total_eviction_exceeded_time:0
current_eviction_exceeded_time:0
keyspace_hits:0
keyspace_misses:0
pubsub_channels:0
pubsub_patterns:0
pubsubshard_channels:0
latest_fork_usec:0
total_forks:0
migrate_cached_sockets:0
slave_expires_tracked_keys:0
active_defrag_hits:0
active_defrag_misses:0
active_defrag_key_hits:0
active_defrag_key_misses:0
total_active_defrag_time:0
current_active_defrag_time:0
tracking_total_keys:0
tracking_total_items:0
tracking_total_prefixes:0
unexpected_error_replies:0
total_error_replies:0
dump_payload_sanitizations:0
total_reads_processed:1
total_writes_processed:0
io_threaded_reads_processed:0
io_threaded_writes_processed:0
reply_buffer_shrinks:0
reply_buffer_expands:0

# Replication
role:master
connected_slaves:0
master_failover_state:no-failover
master_replid:4a4c6ec8b7270086a0b2817c6365adf068f08d71
master_replid2:0000000000000000000000000000000000000000
master_repl_offset:0
second_repl_offset:-1
repl_backlog_active:0
repl_backlog_size:1048576
repl_backlog_first_byte_offset:0
repl_backlog_histlen:0

# CPU
used_cpu_sys:0.207564
used_cpu_user:0.134214
used_cpu_sys_children:0.004313
used_cpu_user_children:0.003942
used_cpu_sys_main_thread:0.197237
used_cpu_user_main_thread:0.133562

# Modules

# Errorstats

# Cluster
cluster_enabled:0

# Keyspace

# EOF    
"#;
    #[test]
    fn test_parser() {
        parse(INFO).unwrap();
    }
}
