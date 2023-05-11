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

  | Metric           | Metric Type | Platform | Description                |
  | ---------------- | ----------- | -------- | -------------------------- |
  | `ps_num_fds`     | Gauge       | Linux    | Number of open files       |
  | `ps_num_threads` | Gauge       | Linux    | Number of threads          |
  | `ps_mem_total`   | Gauge       | Linux    | Total memory               |
  | `ps_mem_rss`     | Gauge       | Linux    | Resident set size          |
  | `ps_read_count`  | Gauge       | Linux    | Total read I/O operations  |
  | `ps_write_count` | Gauge       | Linux    | Total write I/O operations |
  | `ps_read_bytes`  | Gauge       | Linux    | Total bytes read           |
  | `ps_write_bytes` | Gauge       | Linux    | Total bytes written        |
  |                  |

## Labels

`procstat` collector doesn't append its own labels.

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