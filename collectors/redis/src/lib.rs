// --------------------------------------------------------------------
// Gufo Agent: redis collector implementation
// --------------------------------------------------------------------
// Copyright (C) 2021-2023, Gufo Labs
// --------------------------------------------------------------------

use async_trait::async_trait;
use common::{gauge, gauge_f, gauge_i, AgentError, AgentResult, Collectable, Measure};
use redis_client::{Client, ConnectionAddr, ConnectionInfo, RedisConnectionInfo};
use serde::{Deserialize, Serialize};
mod parser;
use parser::{parse, InfoItem, InfoValue};

const REDIS_DEFAULT_HOST: &str = "127.0.0.1";
const REDIS_DEFAULT_PORT: u16 = 6379;
const REDIS_DEFAULT_DB: i64 = 0;

// Collector config
#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_host", skip_serializing_if = "is_default_host")]
    host: String,
    #[serde(default = "default_port", skip_serializing_if = "is_default_port")]
    port: u16,
    #[serde(default = "default_db", skip_serializing_if = "is_default_db")]
    db: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
}

// Collector structure
pub struct Collector {
    conn_info: ConnectionInfo,
}

// Generated metrics
gauge!(redis_uptime_in_seconds, "???", host, port);
gauge!(redis_io_threads_active, "???", host, port);
gauge!(redis_connected_clients, "???", host, port);
gauge!(redis_cluster_connections, "???", host, port);
gauge!(redis_maxclients, "???", host, port);
gauge!(redis_client_recent_max_input_buffer, "???", host, port);
gauge!(redis_client_recent_max_output_buffer, "???", host, port);
gauge!(redis_blocked_clients, "???", host, port);
gauge!(redis_tracking_clients, "???", host, port);
gauge!(redis_clients_in_timeout_table, "???", host, port);
gauge!(redis_used_memory, "???", host, port);
gauge!(redis_used_memory_rss, "???", host, port);
gauge!(redis_used_memory_peak, "???", host, port);
gauge!(redis_used_memory_overhead, "???", host, port);
gauge!(redis_used_memory_startup, "???", host, port);
gauge!(redis_used_memory_dataset, "???", host, port);
gauge!(redis_allocator_allocated, "???", host, port);
gauge!(redis_allocator_active, "???", host, port);
gauge!(redis_allocator_resident, "???", host, port);
gauge!(redis_total_system_memory, "???", host, port);
gauge!(redis_used_memory_lua, "???", host, port);
gauge!(redis_used_memory_vm_eval, "???", host, port);
gauge!(redis_used_memory_scripts_eval, "???", host, port);
gauge!(redis_number_of_cached_scripts, "???", host, port);
gauge!(redis_number_of_functions, "???", host, port);
gauge!(redis_number_of_libraries, "???", host, port);
gauge!(redis_used_memory_vm_functions, "???", host, port);
gauge!(redis_used_memory_vm_total, "???", host, port);
gauge!(redis_used_memory_functions, "???", host, port);
gauge!(redis_used_memory_scripts, "???", host, port);
gauge!(redis_maxmemory, "???", host, port);
gauge_f!(redis_allocator_frag_ratio, "???", host, port);
gauge!(redis_allocator_frag_bytes, "???", host, port);
gauge_f!(redis_allocator_rss_ratio, "???", host, port);
gauge!(redis_allocator_rss_bytes, "???", host, port);
gauge_f!(redis_rss_overhead_ratio, "???", host, port);
gauge!(redis_rss_overhead_bytes, "???", host, port);
gauge_f!(redis_mem_fragmentation_ratio, "???", host, port);
gauge!(redis_mem_fragmentation_bytes, "???", host, port);
gauge!(redis_mem_not_counted_for_evict, "???", host, port);
gauge!(redis_mem_replication_backlog, "???", host, port);
gauge!(redis_mem_total_replication_buffers, "???", host, port);
gauge!(redis_mem_clients_slaves, "???", host, port);
gauge!(redis_mem_clients_normal, "???", host, port);
gauge!(redis_mem_cluster_links, "???", host, port);
gauge!(redis_mem_aof_buffer, "???", host, port);
gauge!(redis_active_defrag_running, "???", host, port);
gauge!(redis_lazyfree_pending_objects, "???", host, port);
gauge!(redis_lazyfreed_objects, "???", host, port);
gauge!(redis_loading, "???", host, port);
gauge!(redis_async_loading, "???", host, port);
gauge!(redis_current_cow_peak, "???", host, port);
gauge!(redis_current_cow_size, "???", host, port);
gauge!(redis_current_cow_size_age, "???", host, port);
gauge!(redis_current_save_keys_processed, "???", host, port);
gauge!(redis_current_save_keys_total, "???", host, port);
gauge!(redis_rdb_changes_since_last_save, "???", host, port);
gauge!(redis_rdb_bgsave_in_progress, "???", host, port);
gauge!(redis_rdb_last_save_time, "???", host, port);
gauge_i!(redis_rdb_last_bgsave_time_sec, "???", host, port);
gauge_i!(redis_rdb_current_bgsave_time_sec, "???", host, port);
gauge!(redis_rdb_saves, "???", host, port);
gauge!(redis_rdb_last_cow_size, "???", host, port);
gauge!(redis_rdb_last_load_keys_expired, "???", host, port);
gauge!(redis_rdb_last_load_keys_loaded, "???", host, port);
gauge!(redis_aof_enabled, "???", host, port);
gauge!(redis_aof_rewrite_in_progress, "???", host, port);
gauge!(redis_aof_rewrite_scheduled, "???", host, port);
gauge_i!(redis_aof_last_rewrite_time_sec, "???", host, port);
gauge_i!(redis_aof_current_rewrite_time_sec, "???", host, port);
gauge!(redis_aof_rewrites, "???", host, port);
gauge!(redis_aof_rewrites_consecutive_failures, "???", host, port);
gauge!(redis_aof_last_cow_size, "???", host, port);
gauge!(redis_module_fork_in_progress, "???", host, port);
gauge!(redis_module_fork_last_cow_size, "???", host, port);
gauge!(redis_total_connections_received, "???", host, port);
gauge!(redis_total_commands_processed, "???", host, port);
gauge!(redis_instantaneous_ops_per_sec, "???", host, port);
gauge!(redis_total_net_input_bytes, "???", host, port);
gauge!(redis_total_net_output_bytes, "???", host, port);
gauge!(redis_total_net_repl_input_bytes, "???", host, port);
gauge!(redis_total_net_repl_output_bytes, "???", host, port);
gauge_f!(redis_instantaneous_input_kbps, "???", host, port);
gauge_f!(redis_instantaneous_output_kbps, "???", host, port);
gauge_f!(redis_instantaneous_input_repl_kbps, "???", host, port);
gauge_f!(redis_instantaneous_output_repl_kbps, "???", host, port);
gauge!(redis_rejected_connections, "???", host, port);
gauge!(redis_sync_full, "???", host, port);
gauge!(redis_sync_partial_ok, "???", host, port);
gauge!(redis_sync_partial_err, "???", host, port);
gauge!(redis_expired_keys, "???", host, port);
gauge_f!(redis_expired_stale_perc, "???", host, port);
gauge!(redis_expired_time_cap_reached_count, "???", host, port);
gauge!(redis_expire_cycle_cpu_milliseconds, "???", host, port);
gauge!(redis_evicted_keys, "???", host, port);
gauge!(redis_evicted_clients, "???", host, port);
gauge!(redis_total_eviction_exceeded_time, "???", host, port);
gauge!(redis_current_eviction_exceeded_time, "???", host, port);
gauge!(redis_keyspace_hits, "???", host, port);
gauge!(redis_keyspace_misses, "???", host, port);
gauge!(redis_pubsub_channels, "???", host, port);
gauge!(redis_pubsub_patterns, "???", host, port);
gauge!(redis_pubsubshard_channels, "???", host, port);
gauge!(redis_latest_fork_usec, "???", host, port);
gauge!(redis_total_forks, "???", host, port);
gauge!(redis_migrate_cached_sockets, "???", host, port);
gauge!(redis_slave_expires_tracked_keys, "???", host, port);
gauge!(redis_active_defrag_hits, "???", host, port);
gauge!(redis_active_defrag_misses, "???", host, port);
gauge!(redis_active_defrag_key_hits, "???", host, port);
gauge!(redis_active_defrag_key_misses, "???", host, port);
gauge!(redis_total_active_defrag_time, "???", host, port);
gauge!(redis_current_active_defrag_time, "???", host, port);
gauge!(redis_tracking_total_keys, "???", host, port);
gauge!(redis_tracking_total_items, "???", host, port);
gauge!(redis_tracking_total_prefixes, "???", host, port);
gauge!(redis_unexpected_error_replies, "???", host, port);
gauge!(redis_total_error_replies, "???", host, port);
gauge!(redis_dump_payload_sanitizations, "???", host, port);
gauge!(redis_total_reads_processed, "???", host, port);
gauge!(redis_total_writes_processed, "???", host, port);
gauge!(redis_io_threaded_reads_processed, "???", host, port);
gauge!(redis_io_threaded_writes_processed, "???", host, port);
gauge!(redis_reply_buffer_shrinks, "???", host, port);
gauge!(redis_reply_buffer_expands, "???", host, port);
gauge!(redis_connected_slaves, "???", host, port);
gauge!(redis_master_replid2, "???", host, port);
gauge!(redis_master_repl_offset, "???", host, port);
gauge_f!(redis_second_repl_offset, "???", host, port);
gauge!(redis_repl_backlog_active, "???", host, port);
gauge!(redis_repl_backlog_size, "???", host, port);
gauge!(redis_repl_backlog_first_byte_offset, "???", host, port);
gauge!(redis_repl_backlog_histlen, "???", host, port);
gauge_f!(redis_used_cpu_sys, "???", host, port);
gauge_f!(redis_used_cpu_user, "???", host, port);
gauge_f!(redis_used_cpu_sys_children, "???", host, port);
gauge_f!(redis_used_cpu_user_children, "???", host, port);
gauge_f!(redis_used_cpu_sys_main_thread, "???", host, port);
gauge_f!(redis_used_cpu_user_main_thread, "???", host, port);

// Instantiate collector from given config
impl TryFrom<Config> for Collector {
    type Error = AgentError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let addr = ConnectionAddr::Tcp(value.host, value.port);
        let redis = RedisConnectionInfo {
            db: value.db,
            username: value.user,
            password: value.password,
        };
        Ok(Self {
            conn_info: ConnectionInfo { addr, redis },
        })
    }
}

// Collector implementation
#[async_trait]
impl Collectable for Collector {
    const NAME: &'static str = "redis";
    type Config = Config;

    async fn collect(&mut self) -> AgentResult<Vec<Measure>> {
        // Connect to redis
        let client = Client::open(self.conn_info.clone())
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        //
        let (host, port) = match &self.conn_info.addr {
            ConnectionAddr::Tcp(h, p) => (h.clone(), p.to_string()),
            ConnectionAddr::TcpTls {
                host: h,
                port: p,
                insecure: _,
            } => (h.clone(), p.to_string()),
            ConnectionAddr::Unix(_) => {
                return Err(AgentError::InternalError(
                    "unix sockets are not supported".to_string(),
                ))
            }
        };
        // Collect data
        let info: String = redis_client::cmd("INFO")
            .query_async(&mut conn)
            .await
            .map_err(|e| AgentError::InternalError(e.to_string()))?;
        // Parse data
        let items = parse(&info)?;
        let mut r = Vec::with_capacity(items.len());
        for item in items.into_iter() {
            if let InfoItem::Value(k, v) = item {
                r.push(match v {
                    InfoValue::Int(x) => match k {
                        "active_defrag_hits" => {
                            redis_active_defrag_hits(x as u64, host.clone(), port.clone())
                        }
                        "active_defrag_key_hits" => {
                            redis_active_defrag_key_hits(x as u64, host.clone(), port.clone())
                        }
                        "active_defrag_key_misses" => {
                            redis_active_defrag_key_misses(x as u64, host.clone(), port.clone())
                        }
                        "active_defrag_misses" => {
                            redis_active_defrag_misses(x as u64, host.clone(), port.clone())
                        }
                        "active_defrag_running" => {
                            redis_active_defrag_running(x as u64, host.clone(), port.clone())
                        }
                        "allocator_active" => {
                            redis_allocator_active(x as u64, host.clone(), port.clone())
                        }
                        "allocator_allocated" => {
                            redis_allocator_allocated(x as u64, host.clone(), port.clone())
                        }
                        "allocator_frag_bytes" => {
                            redis_allocator_frag_bytes(x as u64, host.clone(), port.clone())
                        }
                        "allocator_resident" => {
                            redis_allocator_resident(x as u64, host.clone(), port.clone())
                        }
                        "allocator_rss_bytes" => {
                            redis_allocator_rss_bytes(x as u64, host.clone(), port.clone())
                        }
                        "aof_current_rewrite_time_sec" => {
                            redis_aof_current_rewrite_time_sec(x, host.clone(), port.clone())
                        }
                        "aof_enabled" => redis_aof_enabled(x as u64, host.clone(), port.clone()),
                        "aof_last_cow_size" => {
                            redis_aof_last_cow_size(x as u64, host.clone(), port.clone())
                        }
                        "aof_last_rewrite_time_sec" => {
                            redis_aof_last_rewrite_time_sec(x, host.clone(), port.clone())
                        }
                        "aof_rewrite_in_progress" => {
                            redis_aof_rewrite_in_progress(x as u64, host.clone(), port.clone())
                        }
                        "aof_rewrite_scheduled" => {
                            redis_aof_rewrite_scheduled(x as u64, host.clone(), port.clone())
                        }
                        "aof_rewrites" => redis_aof_rewrites(x as u64, host.clone(), port.clone()),
                        "aof_rewrites_consecutive_failures" => {
                            redis_aof_rewrites_consecutive_failures(
                                x as u64,
                                host.clone(),
                                port.clone(),
                            )
                        }
                        "async_loading" => {
                            redis_async_loading(x as u64, host.clone(), port.clone())
                        }
                        "blocked_clients" => {
                            redis_blocked_clients(x as u64, host.clone(), port.clone())
                        }
                        "client_recent_max_input_buffer" => redis_client_recent_max_input_buffer(
                            x as u64,
                            host.clone(),
                            port.clone(),
                        ),
                        "client_recent_max_output_buffer" => redis_client_recent_max_output_buffer(
                            x as u64,
                            host.clone(),
                            port.clone(),
                        ),
                        "clients_in_timeout_table" => {
                            redis_clients_in_timeout_table(x as u64, host.clone(), port.clone())
                        }
                        "cluster_connections" => {
                            redis_cluster_connections(x as u64, host.clone(), port.clone())
                        }
                        "connected_clients" => {
                            redis_connected_clients(x as u64, host.clone(), port.clone())
                        }
                        "connected_slaves" => {
                            redis_connected_slaves(x as u64, host.clone(), port.clone())
                        }
                        "current_active_defrag_time" => {
                            redis_current_active_defrag_time(x as u64, host.clone(), port.clone())
                        }
                        "current_cow_peak" => {
                            redis_current_cow_peak(x as u64, host.clone(), port.clone())
                        }
                        "current_cow_size" => {
                            redis_current_cow_size(x as u64, host.clone(), port.clone())
                        }
                        "current_cow_size_age" => {
                            redis_current_cow_size_age(x as u64, host.clone(), port.clone())
                        }
                        "current_eviction_exceeded_time" => redis_current_eviction_exceeded_time(
                            x as u64,
                            host.clone(),
                            port.clone(),
                        ),
                        "current_save_keys_processed" => {
                            redis_current_save_keys_processed(x as u64, host.clone(), port.clone())
                        }
                        "current_save_keys_total" => {
                            redis_current_save_keys_total(x as u64, host.clone(), port.clone())
                        }
                        "dump_payload_sanitizations" => {
                            redis_dump_payload_sanitizations(x as u64, host.clone(), port.clone())
                        }
                        "evicted_clients" => {
                            redis_evicted_clients(x as u64, host.clone(), port.clone())
                        }
                        "evicted_keys" => redis_evicted_keys(x as u64, host.clone(), port.clone()),
                        "expire_cycle_cpu_milliseconds" => redis_expire_cycle_cpu_milliseconds(
                            x as u64,
                            host.clone(),
                            port.clone(),
                        ),
                        "expired_keys" => redis_expired_keys(x as u64, host.clone(), port.clone()),
                        "expired_time_cap_reached_count" => redis_expired_time_cap_reached_count(
                            x as u64,
                            host.clone(),
                            port.clone(),
                        ),
                        "instantaneous_ops_per_sec" => {
                            redis_instantaneous_ops_per_sec(x as u64, host.clone(), port.clone())
                        }
                        "io_threaded_reads_processed" => {
                            redis_io_threaded_reads_processed(x as u64, host.clone(), port.clone())
                        }
                        "io_threaded_writes_processed" => {
                            redis_io_threaded_writes_processed(x as u64, host.clone(), port.clone())
                        }
                        "io_threads_active" => {
                            redis_io_threads_active(x as u64, host.clone(), port.clone())
                        }
                        "keyspace_hits" => {
                            redis_keyspace_hits(x as u64, host.clone(), port.clone())
                        }
                        "keyspace_misses" => {
                            redis_keyspace_misses(x as u64, host.clone(), port.clone())
                        }
                        "latest_fork_usec" => {
                            redis_latest_fork_usec(x as u64, host.clone(), port.clone())
                        }
                        "lazyfree_pending_objects" => {
                            redis_lazyfree_pending_objects(x as u64, host.clone(), port.clone())
                        }
                        "lazyfreed_objects" => {
                            redis_lazyfreed_objects(x as u64, host.clone(), port.clone())
                        }
                        "loading" => redis_loading(x as u64, host.clone(), port.clone()),
                        "master_repl_offset" => {
                            redis_master_repl_offset(x as u64, host.clone(), port.clone())
                        }
                        "master_replid2" => {
                            redis_master_replid2(x as u64, host.clone(), port.clone())
                        }
                        "maxclients" => redis_maxclients(x as u64, host.clone(), port.clone()),
                        "maxmemory" => redis_maxmemory(x as u64, host.clone(), port.clone()),
                        "mem_aof_buffer" => {
                            redis_mem_aof_buffer(x as u64, host.clone(), port.clone())
                        }
                        "mem_clients_normal" => {
                            redis_mem_clients_normal(x as u64, host.clone(), port.clone())
                        }
                        "mem_clients_slaves" => {
                            redis_mem_clients_slaves(x as u64, host.clone(), port.clone())
                        }
                        "mem_cluster_links" => {
                            redis_mem_cluster_links(x as u64, host.clone(), port.clone())
                        }
                        "mem_fragmentation_bytes" => {
                            redis_mem_fragmentation_bytes(x as u64, host.clone(), port.clone())
                        }
                        "mem_not_counted_for_evict" => {
                            redis_mem_not_counted_for_evict(x as u64, host.clone(), port.clone())
                        }
                        "mem_replication_backlog" => {
                            redis_mem_replication_backlog(x as u64, host.clone(), port.clone())
                        }
                        "mem_total_replication_buffers" => redis_mem_total_replication_buffers(
                            x as u64,
                            host.clone(),
                            port.clone(),
                        ),
                        "migrate_cached_sockets" => {
                            redis_migrate_cached_sockets(x as u64, host.clone(), port.clone())
                        }
                        "module_fork_in_progress" => {
                            redis_module_fork_in_progress(x as u64, host.clone(), port.clone())
                        }
                        "module_fork_last_cow_size" => {
                            redis_module_fork_last_cow_size(x as u64, host.clone(), port.clone())
                        }
                        "number_of_cached_scripts" => {
                            redis_number_of_cached_scripts(x as u64, host.clone(), port.clone())
                        }
                        "number_of_functions" => {
                            redis_number_of_functions(x as u64, host.clone(), port.clone())
                        }
                        "number_of_libraries" => {
                            redis_number_of_libraries(x as u64, host.clone(), port.clone())
                        }
                        "pubsub_channels" => {
                            redis_pubsub_channels(x as u64, host.clone(), port.clone())
                        }
                        "pubsub_patterns" => {
                            redis_pubsub_patterns(x as u64, host.clone(), port.clone())
                        }
                        "pubsubshard_channels" => {
                            redis_pubsubshard_channels(x as u64, host.clone(), port.clone())
                        }
                        "rdb_bgsave_in_progress" => {
                            redis_rdb_bgsave_in_progress(x as u64, host.clone(), port.clone())
                        }
                        "rdb_changes_since_last_save" => {
                            redis_rdb_changes_since_last_save(x as u64, host.clone(), port.clone())
                        }
                        "rdb_current_bgsave_time_sec" => {
                            redis_rdb_current_bgsave_time_sec(x, host.clone(), port.clone())
                        }
                        "rdb_last_bgsave_time_sec" => {
                            redis_rdb_last_bgsave_time_sec(x, host.clone(), port.clone())
                        }
                        "rdb_last_cow_size" => {
                            redis_rdb_last_cow_size(x as u64, host.clone(), port.clone())
                        }
                        "rdb_last_load_keys_expired" => {
                            redis_rdb_last_load_keys_expired(x as u64, host.clone(), port.clone())
                        }
                        "rdb_last_load_keys_loaded" => {
                            redis_rdb_last_load_keys_loaded(x as u64, host.clone(), port.clone())
                        }
                        "rdb_last_save_time" => {
                            redis_rdb_last_save_time(x as u64, host.clone(), port.clone())
                        }
                        "rdb_saves" => redis_rdb_saves(x as u64, host.clone(), port.clone()),
                        "rejected_connections" => {
                            redis_rejected_connections(x as u64, host.clone(), port.clone())
                        }
                        "repl_backlog_active" => {
                            redis_repl_backlog_active(x as u64, host.clone(), port.clone())
                        }
                        "repl_backlog_first_byte_offset" => redis_repl_backlog_first_byte_offset(
                            x as u64,
                            host.clone(),
                            port.clone(),
                        ),
                        "repl_backlog_histlen" => {
                            redis_repl_backlog_histlen(x as u64, host.clone(), port.clone())
                        }
                        "repl_backlog_size" => {
                            redis_repl_backlog_size(x as u64, host.clone(), port.clone())
                        }
                        "reply_buffer_expands" => {
                            redis_reply_buffer_expands(x as u64, host.clone(), port.clone())
                        }
                        "reply_buffer_shrinks" => {
                            redis_reply_buffer_shrinks(x as u64, host.clone(), port.clone())
                        }
                        "rss_overhead_bytes" => {
                            redis_rss_overhead_bytes(x as u64, host.clone(), port.clone())
                        }
                        "slave_expires_tracked_keys" => {
                            redis_slave_expires_tracked_keys(x as u64, host.clone(), port.clone())
                        }
                        "sync_full" => redis_sync_full(x as u64, host.clone(), port.clone()),
                        "sync_partial_err" => {
                            redis_sync_partial_err(x as u64, host.clone(), port.clone())
                        }
                        "sync_partial_ok" => {
                            redis_sync_partial_ok(x as u64, host.clone(), port.clone())
                        }
                        "total_active_defrag_time" => {
                            redis_total_active_defrag_time(x as u64, host.clone(), port.clone())
                        }
                        "total_commands_processed" => {
                            redis_total_commands_processed(x as u64, host.clone(), port.clone())
                        }
                        "total_connections_received" => {
                            redis_total_connections_received(x as u64, host.clone(), port.clone())
                        }
                        "total_error_replies" => {
                            redis_total_error_replies(x as u64, host.clone(), port.clone())
                        }
                        "total_eviction_exceeded_time" => {
                            redis_total_eviction_exceeded_time(x as u64, host.clone(), port.clone())
                        }
                        "total_forks" => redis_total_forks(x as u64, host.clone(), port.clone()),
                        "total_net_input_bytes" => {
                            redis_total_net_input_bytes(x as u64, host.clone(), port.clone())
                        }
                        "total_net_output_bytes" => {
                            redis_total_net_output_bytes(x as u64, host.clone(), port.clone())
                        }
                        "total_net_repl_input_bytes" => {
                            redis_total_net_repl_input_bytes(x as u64, host.clone(), port.clone())
                        }
                        "total_net_repl_output_bytes" => {
                            redis_total_net_repl_output_bytes(x as u64, host.clone(), port.clone())
                        }
                        "total_reads_processed" => {
                            redis_total_reads_processed(x as u64, host.clone(), port.clone())
                        }
                        "total_system_memory" => {
                            redis_total_system_memory(x as u64, host.clone(), port.clone())
                        }
                        "total_writes_processed" => {
                            redis_total_writes_processed(x as u64, host.clone(), port.clone())
                        }
                        "tracking_clients" => {
                            redis_tracking_clients(x as u64, host.clone(), port.clone())
                        }
                        "tracking_total_items" => {
                            redis_tracking_total_items(x as u64, host.clone(), port.clone())
                        }
                        "tracking_total_keys" => {
                            redis_tracking_total_keys(x as u64, host.clone(), port.clone())
                        }
                        "tracking_total_prefixes" => {
                            redis_tracking_total_prefixes(x as u64, host.clone(), port.clone())
                        }
                        "unexpected_error_replies" => {
                            redis_unexpected_error_replies(x as u64, host.clone(), port.clone())
                        }
                        "uptime_in_seconds" => {
                            redis_uptime_in_seconds(x as u64, host.clone(), port.clone())
                        }
                        "used_memory" => redis_used_memory(x as u64, host.clone(), port.clone()),
                        "used_memory_dataset" => {
                            redis_used_memory_dataset(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_functions" => {
                            redis_used_memory_functions(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_lua" => {
                            redis_used_memory_lua(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_overhead" => {
                            redis_used_memory_overhead(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_peak" => {
                            redis_used_memory_peak(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_rss" => {
                            redis_used_memory_rss(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_scripts" => {
                            redis_used_memory_scripts(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_scripts_eval" => {
                            redis_used_memory_scripts_eval(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_startup" => {
                            redis_used_memory_startup(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_vm_eval" => {
                            redis_used_memory_vm_eval(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_vm_functions" => {
                            redis_used_memory_vm_functions(x as u64, host.clone(), port.clone())
                        }
                        "used_memory_vm_total" => {
                            redis_used_memory_vm_total(x as u64, host.clone(), port.clone())
                        }
                        _ => continue,
                    },
                    InfoValue::Float(x) => match k {
                        "allocator_frag_ratio" => {
                            redis_allocator_frag_ratio(x, host.clone(), port.clone())
                        }
                        "allocator_rss_ratio" => {
                            redis_allocator_rss_ratio(x, host.clone(), port.clone())
                        }
                        "expired_stale_perc" => {
                            redis_expired_stale_perc(x, host.clone(), port.clone())
                        }
                        "instantaneous_input_kbps" => {
                            redis_instantaneous_input_kbps(x, host.clone(), port.clone())
                        }
                        "instantaneous_input_repl_kbps" => {
                            redis_instantaneous_input_repl_kbps(x, host.clone(), port.clone())
                        }
                        "instantaneous_output_kbps" => {
                            redis_instantaneous_output_kbps(x, host.clone(), port.clone())
                        }
                        "instantaneous_output_repl_kbps" => {
                            redis_instantaneous_output_repl_kbps(x, host.clone(), port.clone())
                        }
                        "mem_fragmentation_ratio" => {
                            redis_mem_fragmentation_ratio(x, host.clone(), port.clone())
                        }
                        "rss_overhead_ratio" => {
                            redis_rss_overhead_ratio(x, host.clone(), port.clone())
                        }
                        "second_repl_offset" => {
                            redis_second_repl_offset(x, host.clone(), port.clone())
                        }
                        "used_cpu_sys" => redis_used_cpu_sys(x, host.clone(), port.clone()),
                        "used_cpu_sys_children" => {
                            redis_used_cpu_sys_children(x, host.clone(), port.clone())
                        }
                        "used_cpu_sys_main_thread" => {
                            redis_used_cpu_sys_main_thread(x, host.clone(), port.clone())
                        }
                        "used_cpu_user" => redis_used_cpu_user(x, host.clone(), port.clone()),
                        "used_cpu_user_children" => {
                            redis_used_cpu_user_children(x, host.clone(), port.clone())
                        }
                        "used_cpu_user_main_thread" => {
                            redis_used_cpu_user_main_thread(x, host.clone(), port.clone())
                        }
                        _ => continue,
                    },
                    InfoValue::Str(_) => continue,
                });
            }
        }
        // Push result
        Ok(r)
    }
    // !!! Uncomment for config discovery
    // fn discover_config(_: &ConfigDiscoveryOpts) -> Result<Vec<ConfigItem>, AgentError> {
    //     let cfg = Config;
    //     Ok(vec![ConfigItem::from_config(cfg)?])
    // }
}

fn default_host() -> String {
    REDIS_DEFAULT_HOST.into()
}

fn default_db() -> i64 {
    REDIS_DEFAULT_DB
}

fn default_port() -> u16 {
    REDIS_DEFAULT_PORT
}

fn is_default_host(host: &String) -> bool {
    *host == REDIS_DEFAULT_HOST
}

fn is_default_db(db: &i64) -> bool {
    *db == REDIS_DEFAULT_DB
}

fn is_default_port(port: &u16) -> bool {
    *port == REDIS_DEFAULT_PORT
}
