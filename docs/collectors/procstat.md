# procstat collector

`procstat` collects the host's processes' statistics.

## Configuration

| Parameter  | Type    | Default | Description                                                        |
| ---------- | ------- | ------- | ------------------------------------------------------------------ |
| `id`       | String  |         | Collector's ID. Must be unique per agent instance.                 |
| `type`     | String  |         | Must be `memory`                                                   |
| `interval` | Integer |         | Repetition interval in seconds                                     |
| `labels`   | Object  |         | Additional collector-level labels                                  |
| `self_pid` | Boolean |         | Include agent's own pid                                            |
| `pid_file` | String  |         | Optional path to pid file                                          |
| `pattern`  | String  |         | Optional regular expression that matches the process' command line |

Config example:

``` yaml
- id: procstat
  type: procstat
  interval: 10
  self_pid: true
```

## Collected Metrics

=== "OpenMetrics"

  | Metric                            | Metric Type | Platform | Description                                                                 |
  | --------------------------------- | ----------- | -------- | --------------------------------------------------------------------------- |
  | `ps_num_fds`                      | Gauge       | Linux    | Number of open files                                                        |
  | `ps_num_threads`                  | Gauge       | Linux    | Number of threads                                                           |
  | `ps_voluntary_context_switches`   | Counter     | Linux    | Total voluntary context switches                                            |
  | `ps_involuntary_context_switches` | Counter     | Linux    | Total involuntary context switches                                          |
  | `ps_minor_faults`                 | Counter     | Linux    | Total number of minor faults which do not requirie loading memory from disk |
  | `ps_major_faults`                 | Counter     | Linux    | Total number of major faults which require loading memory from disk         |
  | `ps_child_minor_faults`           | Counter     | Linux    | Total number of minor faults that process waited-for children made          |
  | `ps_child_major_faults`           | Counter     | Linux    | Total number of major faults that process waited-for children made          |
  | `ps_cpu_time_user`                | Counter     | Linux    | CPU time in user mode in seconds                                            |
  | `ps_cpu_time_system`              | Counter     | Linux    | CPU time in system mode in seconds                                          |
  | `ps_cpu_time_iowait`              | Counter     | Linux    | CPU time iowait in seconds                                                  |
  | `ps_mem_total`                    | Counter     | Linux    | Total memory                                                                |
  | `ps_mem_rss`                      | Counter     | Linux    | Resident set size                                                           |
  | `ps_mem_swap`                     | Gauge       | Linux    | Swapped-out virtual memory size                                             |
  | `ps_mem_data`                     | Gauge       | Linux    | Data segment size                                                           |
  | `ps_mem_stack`                    | Gauge       | Linux    | Stack segment size                                                          |
  | `ps_mem_text`                     | Gauge       | Linux    | Text segment size                                                           |
  | `ps_mem_lib`                      | Gauge       | Linux    | Shared library code size                                                    |
  | `ps_mem_locked`                   | Gauge       | Linux    | Locked memory size                                                          |
  | `ps_read_count`                   | Counter     | Linux    | Total read I/O operations                                                   |
  | `ps_write_count`                  | Counter     | Linux    | Total write I/O operations                                                  |
  | `ps_read_bytes`                   | Counter     | Linux    | Total bytes read                                                            |
  | `ps_write_bytes`                  | Counter     | Linux    | Total bytes written                                                         |

## Labels

`procstat` collector appends the following labels:

| Label          | Description         |
| -------------- | ------------------- |
| `process_name` | Name of the process |

## Config Discovery

`procstat` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-procstat
```

## Sample Output

=== "OpenMetrics"

    ```
    ```