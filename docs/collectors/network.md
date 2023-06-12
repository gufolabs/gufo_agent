# network collector

`network` collects host's network interface statistics.

## Configuration

{{ collector_config("network") }}

Config example:

``` yaml
- id: CPU
  type: network
```

## Collected Metrics

=== "OpenMetrics"
  | Metric           | Metric Type | Labels | Description                       |
  | ---------------- | ----------- | ------ | --------------------------------- |
  | `net_rx_octets`  | Counter     | iface  | Total number of octets received   |
  | `net_tx_octets`  | Counter     | iface  | Total number of octets sent       |
  | `net_rx_packets` | Counter     | iface  | Total number of packets received" |
  | `net_tx_packets` | Counter     | iface  | Total number of packets sent      |
  | `net_rx_errors`  | Counter     | iface  | Total number of receive errors    |
  | `net_tx_errors`  | Counter     | iface  | Total number of transmit errors   |

## Labels

`network` collector appends the following labels

| Label   | Description    |
| ------- | -------------- |
| `iface` | Interface name |

## Config Discovery

`network` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-network
```

## Sample Output

=== "OpenMetrics"

    ```
    network_rx_packets{iface="virbr0"} 0 1682413634
    network_rx_packets{iface="wlo1"} 4817460 1682413634
    # HELP network_tx_errors Total number of transmit errors
    # TYPE network_tx_errors counter
    network_tx_errors{iface="virbr0"} 0 1682413634
    network_tx_errors{iface="wlo1"} 0 1682413634
    # HELP network_tx_octets Total number of octets sent
    # TYPE network_tx_octets counter
    network_tx_octets{iface="virbr0"} 0 1682413634
    network_tx_octets{iface="wlo1"} 510399868 1682413634
    # HELP network_tx_packets Total number of packets sent
    # TYPE network_tx_packets counter
    network_tx_packets{iface="virbr0"} 0 1682413634
    network_tx_packets{iface="wlo1"} 2376071 1682413634
    ```