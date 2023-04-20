# uptime collector

`uptime` collects host's system uptime.

## Configuration

| Parameter  | Type    | Default | Description                                        |
| ---------- | ------- | ------- | -------------------------------------------------- |
| `id`       | String  |         | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |         | Must be `cpu`                                      |
| `interval` | Integer |         | Repetition interval in seconds                     |
| `labels`   | Object  |         | Additional collector-level labels                  |

Config example:

``` yaml
- id: Uptime
  type: uptime
  interval: 10
```

## Collected Metrics

| Metric   | Metric Type | Description               |
| -------- | ----------- | ------------------------- |
| `uptime` | Counter     | System uptime, in seconds |

## Labels

`uptime` collector doesn't append own labels.

## Sample Output