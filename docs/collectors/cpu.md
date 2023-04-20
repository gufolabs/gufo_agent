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

| Metric      | Metric Type | Labels | Platform | Description |
| ----------- | ----------- | ------ | -------- | ----------- |
| `user`      |             | cpu    | All      |             |
| `nice`      |             | cpu    | All      |             |
| `system`    |             | cpu    | All      |             |
| `interrupt` |             | cpu    | All      |             |
| `idle`      |             | cpu    | All      |             |
| `iowait`    |             | cpu    | Linux    |             |

## Labels

`cpu` collector appends following labels

| Label | Description |
| ----- | ----------- |
| `cpu` | CPU number  |

## Sample Output