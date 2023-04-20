# dns collector

`dns` collector performs the serie of DNS queries using host's system resolver
and collects query statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
| ---------- | ------- | ------- | -------------------------------------------------- |
| `id`       | String  |         | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |         | Must be `dns`                                      |
| `interval` | Integer |         | Repetition interval in seconds                     |
| `labels`   | Object  |         | Additional collector-level labels                  |
| `query`    | String  |         | DNS Query to perform                               |
| `type`     | String  | `A`     | DNS Query type                                     |
| `n`        | Integer | `1`     | Number of queries in the serie.                    |

Config example:

``` yaml
- id: DNS GufoLabs
disabled: true
type: dns
interval: 15
labels:
    project: Gufo
query: gufolabs.com
n: 10
```

## Collected Metrics

| Metric             | Metric Type | Labels          | Description                                               |
| ------------------ | ----------- | --------------- | --------------------------------------------------------- |
| `requests_total`   | Counter     | `query`, `type` | Total number of DNS requests performed.                   |
| `requests_success` | Counter     | `query`, `type` | Successful DNS requests.                                  |
| `requests_failed`  | Counter     | `query`, `type` | Failed DNS requests.                                      |
| `min_ns`           | Gauge       | `query`, `type` | Minimal response delay ofthe serie in nanoseconds.        |
| `max_ns`           | Gauge       | `query`, `type` | Maximal response delay ofthe serie in nanoseconds.        |
| `avg_ns`           | Gauge       | `query`, `type` | response delay ofthe serie in nanoseconds.                |
| `jitter_ns`        | Gauge       | `query`, `type` | Jitter of the response delay of the serie in nanoseconds. |

## Labels

`dns` collector appends following labels

| Label   | Description                               |
| ------- | ----------------------------------------- |
| `query` | DNS query. Matches `query` in config.     |
| `type`  | DNS query type. Matches `type` in config. |

## Sample Output