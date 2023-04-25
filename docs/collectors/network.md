# network collector

`network` collects host's network interface statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
|------------|---------|---------|----------------------------------------------------|
| `id`       | String  |         | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |         | Must be `network`                                  |
| `interval` | Integer |         | Repetition interval in seconds                     |
| `labels`   | Object  |         | Additional collector-level labels                  |

Config example:

``` yaml
- id: CPU
  type: network
  interval: 10
```

## Collected Metrics

=== "OpenMetrics"
  | Metric               | Metric Type | Labels | Description                       |
  | -------------------- | ----------- | ------ | --------------------------------- |
  | `network_rx_octets`  | Counter     | iface  | Total number of octets received   |
  | `network_tx_octets`  | Counter     | iface  | Total number of octets sent       |
  | `network_rx_packets` | Counter     | iface  | Total number of packets received" |
  | `network_tx_packets` | Counter     | iface  | Total number of packets sent      |
  | `network_rx_errors`  | Counter     | iface  | Total number of receive errors    |
  | `network_tx_errors`  | Counter     | iface  | Total number of transmit errors   |

## Labels

`network` collector appends the following labels

| Label   | Description    |
|---------|----------------|
| `iface` | Interface name |

## Sample Output

=== "OpenMetrics"

    ```
    network_rx_packets{agent="gufo",host="ek-light",iface="virbr0",zone="DC1"} 0 1682413634
    network_rx_packets{agent="gufo",host="ek-light",iface="wlo1",zone="DC1"} 4817460 1682413634
    # HELP network_tx_errors Total number of transmit errors
    # TYPE network_tx_errors counter
    network_tx_errors{agent="gufo",host="ek-light",iface="virbr0",zone="DC1"} 0 1682413634
    network_tx_errors{agent="gufo",host="ek-light",iface="wlo1",zone="DC1"} 0 1682413634
    # HELP network_tx_octets Total number of octets sent
    # TYPE network_tx_octets counter
    network_tx_octets{agent="gufo",host="ek-light",iface="virbr0",zone="DC1"} 0 1682413634
    network_tx_octets{agent="gufo",host="ek-light",iface="wlo1",zone="DC1"} 510399868 1682413634
    # HELP network_tx_packets Total number of packets sent
    # TYPE network_tx_packets counter
    network_tx_packets{agent="gufo",host="ek-light",iface="virbr0",zone="DC1"} 0 1682413634
    network_tx_packets{agent="gufo",host="ek-light",iface="wlo1",zone="DC1"} 2376071 1682413634
    ```