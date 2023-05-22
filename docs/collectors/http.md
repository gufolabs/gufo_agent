# http collector

`http` collector performs the HTTP request and collects query statistics.

## Configuration

| Parameter  | Type    | Default                   | Description                                        |
| ---------- | ------- | ------------------------- | -------------------------------------------------- |
| `id`       | String  |                           | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |                           | Must be `http`                                     |
| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                     |
| `labels`   | Object  |                           | Additional collector-level labels                  |
| `url`      | String  |                           | Request URL                                        |

Config example:

``` yaml
- id: GufoLabs Site
disabled: true
type: http
labels:
    project: Gufo
url: https://gufolabs.com/
```

## Collected Metrics

=== "OpenMetrics"

    | Metric             | Metric Type | Description                       |
    | ------------------ | ----------- | --------------------------------- |
    | `http_time_ns`     | Gauge       | Response time in nanoseconds"     |
    | `bytes`            | Gauge       | Response size in bytes            |
    | `compressed_bytes` | Gauge       | Compressed response size in bytes |

## Labels

`http` collector doesn't append its labels, though they can be configured
via `labels` option.

## Sample Output

=== "OpenMetrics"

    ```
    # HELP http_bytes Response size in bytes
    # TYPE http_bytes gauge
    http_bytes{agent="gufo",host="d20e7299d8e1",zone="DC1"} 1256 1682757242
    # HELP http_compressed_bytes Compressed response size in bytes
    # TYPE http_compressed_bytes gauge
    http_compressed_bytes{agent="gufo",host="d20e7299d8e1",zone="DC1"} 1256 1682757242
    # HELP http_time_ns Response time in nanoseconds
    # TYPE http_time_ns gauge
    http_time_ns{agent="gufo",host="d20e7299d8e1",zone="DC1"} 180379199 1682757242
    ```