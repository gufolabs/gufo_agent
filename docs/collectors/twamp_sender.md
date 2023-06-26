# twamp_sender collector

`twamp_sender` is the [Two-way Active Measurement Protocol(TWAMP)][TWAMP]
client which is used along with a TWAMP reflector to perform the channel quality measurements, including delay, jitter, and packet loss in both directions.

## Configuration

{{ collector_config("twamp_sender") }}

The collector-specific configuration is:

| Parameter        | Type    | Default | Description                                                                                 |
| ---------------- | ------- | ------- | ------------------------------------------------------------------------------------------- |
| `reflector`      | String  |         | IP address of the reflector                                                                 |
| `port`           | Integer | `862`   | Port of the reflector's control channel                                                     |
| `reflector_port` | Integer | `0`     | Demand explicit reflector port, if not `0`. Used to fix weird TWAMP reflector implentations |
| `dscp`           | String  | `be`    | Mark outgoing test packets with appropriate DSCP label                                      |
| `n_packets`      | Integer |         | Number of packets to send                                                                   |
| `model`          | String  |         | Traffic model. See for [Traffic Models](#traffic_models) for details.                       |

Config example:

``` yaml
  - id: Twamp Sender
    type: twamp_sender
    reflector: 127.0.0.1
    n_packets: 100
    model: g711
```

## Traffic Models

Traffic models define the profile of outgoing packets and mimics the usage
of the real-world traffic. The following traffic models are supported:

* `g711` - G.711 codec.
* `g729` - G.729 codec.
* `cbr` - Constant bitrate.
* `imix` - IMIX profile.

### g711

`g711` traffic model mimics the VoIP RTP stream of the G.711 codec. It doesn't requre an additional
configuration.

Config example:

``` yaml
  - id: Twamp Sender
    type: twamp_sender
    interval: 10
    reflector: 127.0.0.1
    n_packets: 100
    model: g711
```

Selecting `g711` model enables [MOS calculation](#mos-calculation).

### g729

`g729` traffic model mimics the VoIP RTP stream of the G.729 codec. It doesn't requre an additional
configuration.

Config example:

``` yaml
  - id: Twamp Sender
    type: twamp_sender
    interval: 10
    reflector: 127.0.0.1
    n_packets: 100
    model: g729
```

Selecting `g729` model enables [MOS calculation](#mos-calculation).

### cbr

`cbr` traffic model generates constant load with given bitrate and packet size.

Configuration:

| Name        | Type    | Description                      |
| ----------- | ------- | -------------------------------- |
| `bandwidth` | Integer | Used bandwidth, in bit/s         |
| `size`      | Integer | Packet size, including IP header |

Config example:

``` yaml
  - id: Twamp Sender
    type: twamp_sender
    interval: 10
    reflector: 127.0.0.1
    n_packets: 100
    model: cbr
    bandwidth: 10000000
    size: 256
```

### imix

`imix` mimics Internet MIX (IMIX) packet model and fills the required 
bandwidth by most-commonly observed packets sizes.

| Name        | Type    | Description              |
| ----------- | ------- | ------------------------ |
| `bandwidth` | Integer | Used bandwidth, in bit/s |

Packet distribution:

| Packet size | Packets | Distribution (packets) | Bytes | Distribution (bytes) |
| ----------: | ------: | ---------------------: | ----: | -------------------: |
|          70 |       7 |                 58.33% |   490 |               11.41% |
|         576 |       4 |                 33.33% |  2304 |               53.65% |
|        1500 |       1 |                  8.33% |  1500 |               34.94% |

!!! note

    TWAMP protocol doesn't allow test packets under the 70 octets.
    So provided model uses small packets of bigger size and
    resulting distribution slightly differs from commonly used
    IMIX models.

Config example:

``` yaml
  - id: Twamp Sender
    type: twamp_sender
    interval: 10
    reflector: 127.0.0.1
    n_packets: 100
    model: imix
    bandwidth: 10000000
```

## Collected Metrics

=== "OpenMetrics"
| Metric                   | Type  | Description                                   |
| ------------------------ | ----- | --------------------------------------------- |
| `twamp_tx_packets`       | Gauge | Transmitted packets                           |
| `twamp_rx_packets`       | Gauge | Received packets                              |
| `twamp_tx_bytes`         | Gauge | Transmitted octets                            |
| `twamp_rx_bytes`         | Gauge | Received octets                               |
| `twamp_duration_ns`      | Gauge | Session duration in nanoseconds               |
| `twamp_tx_pps`           | Gauge | Transmitted packets-per-second rate           |
| `twamp_rx_pps`           | Gauge | Received packet-per-second rate               |
| `twamp_tx_bitrate`       | Gauge | Transmitted bitrate                           |
| `twamp_rx_bitrate`       | Gauge | Received bitrate                              |
| **Inbound**              |       |                                               |
| `twamp_in_min_delay_ns`  | Gauge | Minimum inbound delay in nanoseconds          |
| `twamp_in_max_delay_ns`  | Gauge | Maximum inbound delay in nanoseconds          |
| `twamp_in_avg_delay_ns`  | Gauge | Average inbound delay in nanoseconds          |
| `twamp_in_jitter_ns`     | Gauge | Jitter of the inbound delay in nanoseconds    |
| `twamp_in_loss`          | Gauge | Packet loss in inbound direction              |
| `twamp_in_mos`           | Gauge | eMOS for local end                            |
| **Outbound**             |       |                                               |
| `twamp_out_min_delay_ns` | Gauge | Minimum outbound delay in nanoseconds         |
| `twamp_out_max_delay_ns` | Gauge | Maximum outbound delay in nanoseconds         |
| `twamp_out_avg_delay_ns` | Gauge | Average outbound delay in nanoseconds         |
| `twamp_out_jitter_ns`    | Gauge | Jitter of the outbound delay in nanoseconds   |
| `twamp_out_loss`         | Gauge | Packet loss in outbound direction             |
| `twamp_out_mos`          | Gauge | eMOS for remote end                           |
| **Round-trip**           |       |                                               |
| `twamp_rt_min_delay_ns`  | Gauge | Minimum round-trip delay in nanoseconds       |
| `twamp_rt_max_delay_ns`  | Gauge | Maximum round-trip delay in nanoseconds       |
| `twamp_rt_avg_delay_ns`  | Gauge | Average round-trip delay in nanoseconds       |
| `twamp_rt_jitter_ns`     | Gauge | Jitter of the round-trip delay in nanoseconds |
| `twamp_rt_loss`          | Gauge | Packet loss in both directions                |

## Labels

`twamp_sender` collector doesn't append its labels, though they can be configured
via `labels` option.

## MOS Calculation

Gufo Agent automatically calculates the Mean-Opinion Score (MOS) for VoIP packet models.
MOS calculated according to ITU-T G.107 recommendations using default conditions.

MOS calculated for both directions:

* `in_mos` is for inbound traffic and means the voice quality expectation for the user
  at the TWAMP sender's side.
* `out_mos` is for outbound traffic and shows the voice quality expectation from the user
  at the TWAMP reflector's side.


## Configuring Reflectors

### Gufo Agent

``` yaml
collectors:
    - id: TWAMP Reflector
      type: twamp_reflector
      interval: 10
      listen: "0.0.0.0"
      port: 862
```

See [twamp_reflector](twamp_reflector.md) for configuration options details.

### Juniper JUNOS

```text
chassis {
    fpc 0 {
        pic 0 {
            inline-services {
                bandwidth 1g;
            }
        }
    }
}
services {
    rpm {
            server {
                authentication-mode none;
                port 862;
                client-list Client1 {
                    address {
                        10.1.0.1/24;
                        10.1.0.2/24;
                    }
                }
            }
        }
    }
}
interfaces {
    si-0/0/0 {
        unit 0 {
            family inet;
        }
        unit 10 {
            rpm twamp-server;
            family inet {
                address 10.0.0.1/32;
            }
        }
    }
}
```

Where `10.0.0.1` is the reflector address, and the allowed clients
are in `Client1` list.

### Cisco IOS

IOS implementation demands explicit reflector port number in session request.
Sender must be configured using `reflector_port` parameters:

```yaml
reflector_port: 9447
```

IOS configuration for reflector:

``` text
ip sla server twamp
  exit
ip sla responder twamp
  exit
```

## Sample Output

=== "OpenMetrics"

    ```    
    twamp_sender_duration_ns 1981100036 1682595808
    # HELP twamp_sender_in_avg_delay Average inbound delay in nanoseconds
    # TYPE twamp_sender_in_avg_delay gauge
    twamp_sender_in_avg_delay 37879 1682595808
    # HELP twamp_sender_in_jitter itter of the inbound delay in nanoseconds
    # TYPE twamp_sender_in_jitter gauge
    twamp_sender_in_jitter 26857 1682595808
    # HELP twamp_sender_in_loss Packet loss in inbound direction
    # TYPE twamp_sender_in_loss gauge
    twamp_sender_in_loss 0 1682595808
    # HELP twamp_sender_in_max_delay Maximum inbound delay in nanoseconds
    # TYPE twamp_sender_in_max_delay gauge
    twamp_sender_in_max_delay 172580 1682595808
    # HELP twamp_sender_in_min_delay Minimum inbound delay in nanoseconds
    # TYPE twamp_sender_in_min_delay gauge
    twamp_sender_in_min_delay 18112 1682595808
    # HELP in_mos eMOS for local end
    # TYPE in_mos gauge
    in_mos 4.4043684 1682595808    
    # HELP twamp_sender_out_avg_delay Average outbound delay in nanoseconds
    # TYPE twamp_sender_out_avg_delay gauge
    twamp_sender_out_avg_delay 90004 1682595808
    # HELP twamp_sender_out_jitter Jitter of the outbound delay in nanoseconds
    # TYPE twamp_sender_out_jitter gauge
    twamp_sender_out_jitter 22222 1682595808
    # HELP twamp_sender_out_loss Packet loss in outbound direction
    # TYPE twamp_sender_out_loss gauge
    twamp_sender_out_loss 0 1682595808
    # HELP twamp_sender_out_max_delay Maximum outbound delay in nanoseconds
    # TYPE twamp_sender_out_max_delay gauge
    twamp_sender_out_max_delay 211155 1682595808
    # HELP twamp_sender_out_min_delay Minimum outbound delay in nanoseconds
    # TYPE twamp_sender_out_min_delay gauge
    twamp_sender_out_min_delay 29212 1682595808
    # HELP out_mos eMOS for remote end
    # TYPE out_mos gauge
    out_mos 4.4043336 1682595808    
    # HELP twamp_sender_rt_avg_delay Average round-trip delay in nanoseconds
    # TYPE twamp_sender_rt_avg_delay gauge    
    twamp_sender_rt_avg_delay 127884 1682595808
    # HELP twamp_sender_rt_jitter Jitter of the round-trip delay in nanoseconds
    # TYPE twamp_sender_rt_jitter gauge
    twamp_sender_rt_jitter 43653 1682595808
    # HELP twamp_sender_rt_loss Packet loss in both directions
    # TYPE twamp_sender_rt_loss gauge
    twamp_sender_rt_loss 0 1682595808
    # HELP twamp_sender_rt_max_delay Maximum round-trip delay in nanoseconds
    # TYPE twamp_sender_rt_max_delay gauge
    twamp_sender_rt_max_delay 262171 1682595808
    # HELP twamp_sender_rt_min_delay Minimum round-trip delay in nanoseconds
    # TYPE twamp_sender_rt_min_delay gauge
    twamp_sender_rt_min_delay 58006 1682595808
    # HELP twamp_sender_rx_bitrate Received bitrate
    # TYPE twamp_sender_rx_bitrate gauge
    twamp_sender_rx_bitrate 80242 1682595808
    # HELP twamp_sender_rx_bytes Received octets
    # TYPE twamp_sender_rx_bytes gauge
    twamp_sender_rx_bytes 19872 1682595808
    # HELP twamp_sender_rx_packets Received packets
    # TYPE twamp_sender_rx_packets counter
    twamp_sender_rx_packets 100 1682595808
    # HELP twamp_sender_rx_pps Received packet-per-second rate
    # TYPE twamp_sender_rx_pps gauge
    twamp_sender_rx_pps 50 1682595808
    # HELP twamp_sender_tx_bitrate Transmitted bitrate
    # TYPE twamp_sender_tx_bitrate gauge
    twamp_sender_tx_bitrate 80763 1682595808
    # HELP twamp_sender_tx_bytes Transmitted octets
    # TYPE twamp_sender_tx_bytes counter
    twamp_sender_tx_bytes 20000 1682595808
    # HELP twamp_sender_tx_packets Transmitted packets
    # TYPE twamp_sender_tx_packets counter
    twamp_sender_tx_packets 100 1682595808
    # HELP twamp_sender_tx_pps Transmitted packets-per-second rate
    # TYPE twamp_sender_tx_pps gauge
    twamp_sender_tx_pps 50 1682595808
    ```

[TWAMP]: https://www.juniper.net/documentation/us/en/software/junos/flow-monitoring/topics/concept/twamp-overview.html