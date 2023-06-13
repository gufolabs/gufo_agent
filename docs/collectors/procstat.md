# procstat collector

`procstat` collects the host's processes' statistics.

## Configuration

{{ collector_config("procstat") }}

The collector-specific configuration is:

| Parameter       | Type    | Default | Description                                                                |
| --------------- | ------- | ------- | -------------------------------------------------------------------------- |
| `expose_labels` | Array   |         | List of value to enable optional labels. See [Labels](#labels) for details |
| `self_pid`      | Boolean |         | Include agent's own pid                                                    |
| `pid_file`      | String  |         | Optional path to pid file                                                  |
| `pattern`       | String  |         | Optional regular expression that matches the process' command line         |

Config example:

``` yaml
- id: procstat
  type: procstat
  expose_labels: [user]
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
  | `ps_cpu_usage`                    | Gauge       | Linux    | Total CPU usage in percents                                                 |
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

| Label          | `expose_labels` | Description                                                                                                    |
| -------------- | --------------- | -------------------------------------------------------------------------------------------------------------- |
| `process_name` |                 | Name of the process                                                                                            |
| `user`         | `user`          | Process' effective user name                                                                                   |
| `cmd`          | `cmd`           | Full command line separated by `cmd_separator`                                                                 |
| `__meta_cmd`   | `__meta_cmd`    | [Virtual label](../relabel.md#virtual-labels) version of `cmd`                                                 |
| `__meta_env`   | `__meta_env`    | [Virtual label](../relabel.md#virtual-labels) containing the process' environmenr separated by `env_separator` |

`expose_labels` may take the following values:

* `user` - Expose `user` label.
* `cmd` - Expose `cmd` label. 
* `__meta_cmd` - Expose `__meta_cmd` label.
* `__meta_env` - Expose `__meta_env` label.

## Config Discovery

`procstat` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-procstat
```

## Process Name Rewritting

Sometimes, the process name is non-unique and misleading. Then the [Relabeling Rules](../relabel.md)
come to the resque. Consider we have a set of the services launched by `run` command using syntax

```
run <service>
```

Then we can configure `procstat` collector to fetch proper name to the labels:

``` yaml
- id: Procstat
  type: procstat
  ...
  expose_labels: [__meta_cmd]
  relabel:
  - source_labels: ["__meta_cmd"]
    regex: "run (.+)"
    replacement: "$1"
    target_label: process_name
    action: replace
```

Then the `process_name` label will contain service name, instead of `run`.

## Sample Output

=== "OpenMetrics"

    ```
    ```