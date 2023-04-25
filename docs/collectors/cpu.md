# cpu collector

`cpu` collects the host's CPU usage statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
|------------|---------|---------|----------------------------------------------------|
| `id`       | String  |         | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |         | Must be `cpu`                                      |
| `interval` | Integer |         | Repetition interval in seconds                     |
| `labels`   | Object  |         | Additional collector-level labels                  |

Config example:

``` yaml
- id: CPU
  type: cpu
  interval: 10
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
|-------|-------------|
| `cpu` | CPU number  |

## Sample Output

=== "OpenMetrics"

    ```
    # HELP cpu_idle CPU Idle time, %
    # TYPE cpu_idle gauge
    cpu_idle{agent="gufo",cpu="0",host="ek-light",zone="DC1"} 97 1682413626
    cpu_idle{agent="gufo",cpu="1",host="ek-light",zone="DC1"} 93 1682413626
    cpu_idle{agent="gufo",cpu="2",host="ek-light",zone="DC1"} 98 1682413626
    cpu_idle{agent="gufo",cpu="3",host="ek-light",zone="DC1"} 94 1682413626
    # HELP cpu_interrupt CPU Interrupt time, %
    # TYPE cpu_interrupt gauge
    cpu_interrupt{agent="gufo",cpu="0",host="ek-light",zone="DC1"} 0 1682413626
    cpu_interrupt{agent="gufo",cpu="1",host="ek-light",zone="DC1"} 0 1682413626
    cpu_interrupt{agent="gufo",cpu="2",host="ek-light",zone="DC1"} 0 1682413626
    cpu_interrupt{agent="gufo",cpu="3",host="ek-light",zone="DC1"} 0 1682413626
    # HELP cpu_iowait CPU IOwait time, %
    # TYPE cpu_iowait gauge
    cpu_iowait{agent="gufo",cpu="0",host="ek-light",zone="DC1"} 0 1682413626
    cpu_iowait{agent="gufo",cpu="1",host="ek-light",zone="DC1"} 0 1682413626
    cpu_iowait{agent="gufo",cpu="2",host="ek-light",zone="DC1"} 0 1682413626
    cpu_iowait{agent="gufo",cpu="3",host="ek-light",zone="DC1"} 0 1682413626
    # HELP cpu_nice CPU Nice time, %
    # TYPE cpu_nice gauge
    cpu_nice{agent="gufo",cpu="0",host="ek-light",zone="DC1"} 0 1682413626
    cpu_nice{agent="gufo",cpu="1",host="ek-light",zone="DC1"} 0 1682413626
    cpu_nice{agent="gufo",cpu="2",host="ek-light",zone="DC1"} 0 1682413626
    cpu_nice{agent="gufo",cpu="3",host="ek-light",zone="DC1"} 0 1682413626
    # HELP cpu_system CPU System time, %
    # TYPE cpu_system gauge
    cpu_system{agent="gufo",cpu="0",host="ek-light",zone="DC1"} 0 1682413626
    cpu_system{agent="gufo",cpu="1",host="ek-light",zone="DC1"} 2 1682413626
    cpu_system{agent="gufo",cpu="2",host="ek-light",zone="DC1"} 0 1682413626
    cpu_system{agent="gufo",cpu="3",host="ek-light",zone="DC1"} 3 1682413626
    # HELP cpu_user CPU User time, %
    # TYPE cpu_user gauge
    cpu_user{agent="gufo",cpu="0",host="ek-light",zone="DC1"} 2 1682413626
    cpu_user{agent="gufo",cpu="1",host="ek-light",zone="DC1"} 5 1682413626
    cpu_user{agent="gufo",cpu="2",host="ek-light",zone="DC1"} 1 1682413626
    cpu_user{agent="gufo",cpu="3",host="ek-light",zone="DC1"} 2 1682413626
    ```