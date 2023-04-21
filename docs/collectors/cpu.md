# cpu collector

`cpu` collects host's CPU usage statistics.

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
  | `cpu_user`      |             | cpu    | All      |             |
  | `cpu_nice`      |             | cpu    | All      |             |
  | `cpu_system`    |             | cpu    | All      |             |
  | `cpu_interrupt` |             | cpu    | All      |             |
  | `cpu_idle`      |             | cpu    | All      |             |
  | `cpu_iowait`    |             | cpu    | Linux    |             |

## Labels

`cpu` collector appends following labels

| Label | Description |
| ----- | ----------- |
| `cpu` | CPU number  |

## Sample Output