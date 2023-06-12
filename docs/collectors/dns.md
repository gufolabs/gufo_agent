# dns collector

`dns` collector performs the series of DNS queries using the host's system resolver
and collects query statistics.

## Configuration

{{ collector_config("dns") }}

The collector-specific configuration is:

| Parameter | Type    | Default | Description                     |
| --------- | ------- | ------- | ------------------------------- |
| `query`   | String  |         | DNS Query to perform            |
| `type`    | String  | `A`     | DNS Query type                  |
| `n`       | Integer | `1`     | Number of queries in the serie. |

Config example:

``` yaml
- id: DNS GufoLabs
disabled: true
type: dns
labels:
    project: Gufo
query: gufolabs.com
n: 10
```

## Collected Metrics

=== "OpenMetrics"

    | Metric                 | Metric Type | Labels          | Description                                               |
    | ---------------------- | ----------- | --------------- | --------------------------------------------------------- |
    | `dns_requests_total`   | Counter     | `query`, `type` | Total number of DNS requests performed.                   |
    | `dns_requests_success` | Counter     | `query`, `type` | Successful DNS requests.                                  |
    | `dns_requests_failed`  | Counter     | `query`, `type` | Failed DNS requests.                                      |
    | `dns_min_ns`           | Gauge       | `query`, `type` | Minimal response delay ofthe serie in nanoseconds.        |
    | `dns_max_ns`           | Gauge       | `query`, `type` | Maximal response delay ofthe serie in nanoseconds.        |
    | `dns_avg_ns`           | Gauge       | `query`, `type` | response delay ofthe serie in nanoseconds.                |
    | `dns_jitter_ns`        | Gauge       | `query`, `type` | Jitter of the response delay of the serie in nanoseconds. |

## Labels

`dns` collector appends the following labels

| Label   | Description                               |
| ------- | ----------------------------------------- |
| `query` | DNS query. Matches `query` in config.     |
| `type`  | DNS query type. Matches `type` in config. |

## Sample Output

=== "OpenMetrics"

    ```
    # HELP dns_avg_ns Average response delay of the serie in nanoseconds
    # TYPE dns_avg_ns gauge
    dns_avg_ns{query="gufolabs.com",type="A"} 3849061 1682060489
    # HELP dns_jitter_ns Jitter of the response delay of the serie in nanoseconds
    # TYPE dns_jitter_ns gauge
    dns_jitter_ns{query="gufolabs.com",type="A"} 2774319 1682060489
    # HELP dns_max_ns Maximal response delay of the serie in nanoseconds
    # TYPE dns_max_ns gauge
    dns_max_ns{query="gufolabs.com",type="A"} 38388860 1682060489
    # HELP dns_min_ns Minimal response delay of the serie in nanoseconds
    # TYPE dns_min_ns gauge
    dns_min_ns{query="gufolabs.com",type="A"} 6596 1682060489
    # HELP dns_requests_failed Failed DNS requests
    # TYPE dns_requests_failed counter
    dns_requests_failed{query="gufolabs.com",type="A"} 0 1682060489
    # HELP dns_requests_success Successful DNS requests
    # TYPE dns_requests_success counter
    dns_requests_success{query="gufolabs.com",type="A"} 10 1682060489
    # HELP dns_requests_total Total DNS requests performed
    # TYPE dns_requests_total counter
    dns_requests_total{query="gufolabs.com",type="A"} 10 1682060489
    ```