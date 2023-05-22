# sockets collector

`sockets` collects the host's sockets statistics.

## Configuration

| Parameter  | Type    | Default                   | Description                                        |
| ---------- | ------- | ------------------------- | -------------------------------------------------- |
| `id`       | String  |                           | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |                           | Must be `sockets`                                  |
| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                     |
| `labels`   | Object  |                           | Additional collector-level labels                  |

Config example:

``` yaml
- id: Sockets
  type: sockets
```

## Collected Metrics

=== "OpenMetrics"
  | Metric            | Metric Type | Description                           |
  | ----------------- | ----------- | ------------------------------------- |
  | tcp4_sockets_used | Gauge       | Total amount of IPv4 TCP sockets used |
  | tcp6_sockets_used | Gauge       | Total amount of IPv6 TCP sockets used |
  | udp4_sockets_used | Gauge       | Total amount of IPv4 UDP sockets used |
  | udp6_sockets_used | Gauge       | Total amount of IPv6 UDP sockets used |

## Labels

`sockets` collector doesn't append its own labels.

## Config Discovery

`sockets` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-sockets
```

## Sample Output

=== "OpenMetrics"

    ```
    # HELP sockets_tcp4_sockets_used Total amount of IPv4 TCP sockets used
    # TYPE sockets_tcp4_sockets_used gauge
    sockets_tcp4_sockets_used 19 1683011185
    # HELP sockets_tcp6_sockets_used Total amount of IPv6 TCP sockets used
    # TYPE sockets_tcp6_sockets_used gauge
    sockets_tcp6_sockets_used 0 1683011185
    # HELP sockets_udp4_sockets_used Total amount of IPv4 UDP sockets used
    # TYPE sockets_udp4_sockets_used gauge
    sockets_udp4_sockets_used 0 1683011185
    # HELP sockets_udp6_sockets_used Total amount of IPv6 UDP sockets used
    # TYPE sockets_udp6_sockets_used gauge
    sockets_udp6_sockets_used 0 1683011185    
    ```