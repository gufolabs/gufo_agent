# redis collector

`redis` collects Redis instance statistics.

## Configuration

| Parameter  | Type    | Default                   | Description                                        |
| ---------- | ------- | ------------------------- | -------------------------------------------------- |
| `id`       | String  |                           | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |                           | Must be `http`                                     |
| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                     |
| `labels`   | Object  |                           | Additional collector-level labels                  |
| `host`     | String  | `127.0.0.1`               | Redis server address                               |
| `port`     | Integer | `6379`                    | Redis server port                                  |
| `db`       | Integer | `0`                       | Redis database                                     |
| `user`     | String  |                           | Optional database user                             |
| `password` | String  |                           | Optional database password                         |


Config example:

``` yaml
- id: ???
  disabled: false
  type: redis
  host: 127.0.0.1
  port: 6379
```

## Collected Metrics

=== "OpenMetrics"

    | Metric                                    | Metric Type | Description |
    | ----------------------------------------- | ----------- | ----------- |
    | `redis_uptime_in_seconds`                 | Gauge       | ???         |
    | `redis_io_threads_active`                 | Gauge       | ???         |
    | `redis_connected_clients`                 | Gauge       | ???         |
    | `redis_cluster_connections`               | Gauge       | ???         |
    | `redis_maxclients`                        | Gauge       | ???         |
    | `redis_client_recent_max_input_buffer`    | Gauge       | ???         |
    | `redis_client_recent_max_output_buffer`   | Gauge       | ???         |
    | `redis_blocked_clients`                   | Gauge       | ???         |
    | `redis_tracking_clients`                  | Gauge       | ???         |
    | `redis_clients_in_timeout_table`          | Gauge       | ???         |
    | `redis_used_memory`                       | Gauge       | ???         |
    | `redis_used_memory_rss`                   | Gauge       | ???         |
    | `redis_used_memory_peak`                  | Gauge       | ???         |
    | `redis_used_memory_overhead`              | Gauge       | ???         |
    | `redis_used_memory_startup`               | Gauge       | ???         |
    | `redis_used_memory_dataset`               | Gauge       | ???         |
    | `redis_allocator_allocated`               | Gauge       | ???         |
    | `redis_allocator_active`                  | Gauge       | ???         |
    | `redis_allocator_resident`                | Gauge       | ???         |
    | `redis_total_system_memory`               | Gauge       | ???         |
    | `redis_used_memory_lua`                   | Gauge       | ???         |
    | `redis_used_memory_vm_eval`               | Gauge       | ???         |
    | `redis_used_memory_scripts_eval`          | Gauge       | ???         |
    | `redis_number_of_cached_scripts`          | Gauge       | ???         |
    | `redis_number_of_functions`               | Gauge       | ???         |
    | `redis_number_of_libraries`               | Gauge       | ???         |
    | `redis_used_memory_vm_functions`          | Gauge       | ???         |
    | `redis_used_memory_vm_total`              | Gauge       | ???         |
    | `redis_used_memory_functions`             | Gauge       | ???         |
    | `redis_used_memory_scripts`               | Gauge       | ???         |
    | `redis_maxmemory`                         | Gauge       | ???         |
    | `redis_allocator_frag_ratio`              | Gauge       | ???         |
    | `redis_allocator_frag_bytes`              | Gauge       | ???         |
    | `redis_allocator_rss_ratio`               | Gauge       | ???         |
    | `redis_allocator_rss_bytes`               | Gauge       | ???         |
    | `redis_rss_overhead_ratio`                | Gauge       | ???         |
    | `redis_rss_overhead_bytes`                | Gauge       | ???         |
    | `redis_mem_fragmentation_ratio`           | Gauge       | ???         |
    | `redis_mem_fragmentation_bytes`           | Gauge       | ???         |
    | `redis_mem_not_counted_for_evict`         | Gauge       | ???         |
    | `redis_mem_replication_backlog`           | Gauge       | ???         |
    | `redis_mem_total_replication_buffers`     | Gauge       | ???         |
    | `redis_mem_clients_slaves`                | Gauge       | ???         |
    | `redis_mem_clients_normal`                | Gauge       | ???         |
    | `redis_mem_cluster_links`                 | Gauge       | ???         |
    | `redis_mem_aof_buffer`                    | Gauge       | ???         |
    | `redis_active_defrag_running`             | Gauge       | ???         |
    | `redis_lazyfree_pending_objects`          | Gauge       | ???         |
    | `redis_lazyfreed_objects`                 | Gauge       | ???         |
    | `redis_loading`                           | Gauge       | ???         |
    | `redis_async_loading`                     | Gauge       | ???         |
    | `redis_current_cow_peak`                  | Gauge       | ???         |
    | `redis_current_cow_size`                  | Gauge       | ???         |
    | `redis_current_cow_size_age`              | Gauge       | ???         |
    | `redis_current_save_keys_processed`       | Gauge       | ???         |
    | `redis_current_save_keys_total`           | Gauge       | ???         |
    | `redis_rdb_changes_since_last_save`       | Gauge       | ???         |
    | `redis_rdb_bgsave_in_progress`            | Gauge       | ???         |
    | `redis_rdb_last_save_time`                | Gauge       | ???         |
    | `redis_rdb_last_bgsave_time_sec`          | Gauge       | ???         |
    | `redis_rdb_current_bgsave_time_sec`       | Gauge       | ???         |
    | `redis_rdb_saves`                         | Gauge       | ???         |
    | `redis_rdb_last_cow_size`                 | Gauge       | ???         |
    | `redis_rdb_last_load_keys_expired`        | Gauge       | ???         |
    | `redis_rdb_last_load_keys_loaded`         | Gauge       | ???         |
    | `redis_aof_enabled`                       | Gauge       | ???         |
    | `redis_aof_rewrite_in_progress`           | Gauge       | ???         |
    | `redis_aof_rewrite_scheduled`             | Gauge       | ???         |
    | `redis_aof_last_rewrite_time_sec`         | Gauge       | ???         |
    | `redis_aof_current_rewrite_time_sec`      | Gauge       | ???         |
    | `redis_aof_rewrites`                      | Gauge       | ???         |
    | `redis_aof_rewrites_consecutive_failures` | Gauge       | ???         |
    | `redis_aof_last_cow_size`                 | Gauge       | ???         |
    | `redis_module_fork_in_progress`           | Gauge       | ???         |
    | `redis_module_fork_last_cow_size`         | Gauge       | ???         |
    | `redis_total_connections_received`        | Gauge       | ???         |
    | `redis_total_commands_processed`          | Gauge       | ???         |
    | `redis_instantaneous_ops_per_sec`         | Gauge       | ???         |
    | `redis_total_net_input_bytes`             | Gauge       | ???         |
    | `redis_total_net_output_bytes`            | Gauge       | ???         |
    | `redis_total_net_repl_input_bytes`        | Gauge       | ???         |
    | `redis_total_net_repl_output_bytes`       | Gauge       | ???         |
    | `redis_instantaneous_input_kbps`          | Gauge       | ???         |
    | `redis_instantaneous_output_kbps`         | Gauge       | ???         |
    | `redis_instantaneous_input_repl_kbps`     | Gauge       | ???         |
    | `redis_instantaneous_output_repl_kbps`    | Gauge       | ???         |
    | `redis_rejected_connections`              | Gauge       | ???         |
    | `redis_sync_full`                         | Gauge       | ???         |
    | `redis_sync_partial_ok`                   | Gauge       | ???         |
    | `redis_sync_partial_err`                  | Gauge       | ???         |
    | `redis_expired_keys`                      | Gauge       | ???         |
    | `redis_expired_stale_perc`                | Gauge       | ???         |
    | `redis_expired_time_cap_reached_count`    | Gauge       | ???         |
    | `redis_expire_cycle_cpu_milliseconds`     | Gauge       | ???         |
    | `redis_evicted_keys`                      | Gauge       | ???         |
    | `redis_evicted_clients`                   | Gauge       | ???         |
    | `redis_total_eviction_exceeded_time`      | Gauge       | ???         |
    | `redis_current_eviction_exceeded_time`    | Gauge       | ???         |
    | `redis_keyspace_hits`                     | Gauge       | ???         |
    | `redis_keyspace_misses`                   | Gauge       | ???         |
    | `redis_pubsub_channels`                   | Gauge       | ???         |
    | `redis_pubsub_patterns`                   | Gauge       | ???         |
    | `redis_pubsubshard_channels`              | Gauge       | ???         |
    | `redis_latest_fork_usec`                  | Gauge       | ???         |
    | `redis_total_forks`                       | Gauge       | ???         |
    | `redis_migrate_cached_sockets`            | Gauge       | ???         |
    | `redis_slave_expires_tracked_keys`        | Gauge       | ???         |
    | `redis_active_defrag_hits`                | Gauge       | ???         |
    | `redis_active_defrag_misses`              | Gauge       | ???         |
    | `redis_active_defrag_key_hits`            | Gauge       | ???         |
    | `redis_active_defrag_key_misses`          | Gauge       | ???         |
    | `redis_total_active_defrag_time`          | Gauge       | ???         |
    | `redis_current_active_defrag_time`        | Gauge       | ???         |
    | `redis_tracking_total_keys`               | Gauge       | ???         |
    | `redis_tracking_total_items`              | Gauge       | ???         |
    | `redis_tracking_total_prefixes`           | Gauge       | ???         |
    | `redis_unexpected_error_replies`          | Gauge       | ???         |
    | `redis_total_error_replies`               | Gauge       | ???         |
    | `redis_dump_payload_sanitizations`        | Gauge       | ???         |
    | `redis_total_reads_processed`             | Gauge       | ???         |
    | `redis_total_writes_processed`            | Gauge       | ???         |
    | `redis_io_threaded_reads_processed`       | Gauge       | ???         |
    | `redis_io_threaded_writes_processed`      | Gauge       | ???         |
    | `redis_reply_buffer_shrinks`              | Gauge       | ???         |
    | `redis_reply_buffer_expands`              | Gauge       | ???         |
    | `redis_connected_slaves`                  | Gauge       | ???         |
    | `redis_master_replid2`                    | Gauge       | ???         |
    | `redis_master_repl_offset`                | Gauge       | ???         |
    | `redis_second_repl_offset`                | Gauge       | ???         |
    | `redis_repl_backlog_active`               | Gauge       | ???         |
    | `redis_repl_backlog_size`                 | Gauge       | ???         |
    | `redis_repl_backlog_first_byte_offset`    | Gauge       | ???         |
    | `redis_repl_backlog_histlen`              | Gauge       | ???         |
    | `redis_used_cpu_sys`                      | Gauge       | ???         |
    | `redis_used_cpu_user`                     | Gauge       | ???         |
    | `redis_used_cpu_sys_children`             | Gauge       | ???         |
    | `redis_used_cpu_user_children`            | Gauge       | ???         |
    | `redis_used_cpu_sys_main_thread`          | Gauge       | ???         |
    | `redis_used_cpu_user_main_thread`         | Gauge       | ???         |

## Labels

`redis` collector appends the following labels

| Label  | Description                                                |
| ------ | ---------------------------------------------------------- |
| `host` | Server address, same as the `host` configuration parameter |
| `port` | Server  port, same  as the `port` configuration  parameter |

## Sample Output

=== "OpenMetrics"

    ```
    ```
