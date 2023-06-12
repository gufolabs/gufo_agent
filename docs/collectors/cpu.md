# cpu collector

`cpu` collects the host's CPU usage statistics.

## Configuration

{{ collector_config("cpu") }}

Config example:

``` yaml
- id: CPU
  type: cpu
```

## Collected Metrics

=== "OpenMetrics"
  | Metric          | Metric Type | Labels | Platform | Description |
  | --------------- | ----------- | ------ | -------- | ----------- |
  | `cpu_user`      | Gauge       | cpu    | All      | ???         |
  | `cpu_nice`      | Gauge       | cpu    | All      | ???         |
  | `cpu_system`    | Gauge       | cpu    | All      | ???         |
  | `cpu_interrupt` | Gauge       | cpu    | All      | ???         |
  | `cpu_idle`      | Gauge       | cpu    | All      | ???         |
  | `cpu_iowait`    | Gauge       | cpu    | Linux    | ???         |

## Labels

`cpu` collector appends the following labels

| Label | Description |
| ----- | ----------- |
| `cpu` | CPU number  |

## Config Discovery

`cpu` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-cpu
```

## Sample Output

=== "OpenMetrics"

    ```
    # HELP cpu_idle CPU Idle time, %
    # TYPE cpu_idle gauge
    cpu_idle{cpu="0"} 97 1682413626
    cpu_idle{cpu="1"} 93 1682413626
    cpu_idle{cpu="2"} 98 1682413626
    cpu_idle{cpu="3"} 94 1682413626
    # HELP cpu_interrupt CPU Interrupt time, %
    # TYPE cpu_interrupt gauge
    cpu_interrupt{cpu="0"} 0 1682413626
    cpu_interrupt{cpu="1"} 0 1682413626
    cpu_interrupt{cpu="2"} 0 1682413626
    cpu_interrupt{cpu="3"} 0 1682413626
    # HELP cpu_iowait CPU IOwait time, %
    # TYPE cpu_iowait gauge
    cpu_iowait{cpu="0"} 0 1682413626
    cpu_iowait{cpu="1"} 0 1682413626
    cpu_iowait{cpu="2"} 0 1682413626
    cpu_iowait{cpu="3"} 0 1682413626
    # HELP cpu_nice CPU Nice time, %
    # TYPE cpu_nice gauge
    cpu_nice{cpu="0"} 0 1682413626
    cpu_nice{cpu="1"} 0 1682413626
    cpu_nice{cpu="2"} 0 1682413626
    cpu_nice{cpu="3"} 0 1682413626
    # HELP cpu_system CPU System time, %
    # TYPE cpu_system gauge
    cpu_system{cpu="0"} 0 1682413626
    cpu_system{cpu="1"} 2 1682413626
    cpu_system{cpu="2"} 0 1682413626
    cpu_system{cpu="3"} 3 1682413626
    # HELP cpu_user CPU User time, %
    # TYPE cpu_user gauge
    cpu_user{cpu="0"} 2 1682413626
    cpu_user{cpu="1"} 5 1682413626
    cpu_user{cpu="2"} 1 1682413626
    cpu_user{cpu="3"} 2 1682413626
    ```