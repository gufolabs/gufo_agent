# cpu collector

`cpu` collects the host's CPU usage statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
| ---------- | ------- | ------- | -------------------------------------------------- |
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
| ----- | ----------- |
| `cpu` | CPU number  |

## Sample Output