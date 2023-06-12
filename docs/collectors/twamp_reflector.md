# twamp_reflector collector

`twamp_reflector` is the [Two-way Active Measurement Protocol(TWAMP)][TWAMP]
reflector for the channel SLA measurements. Usually it accompanies the
[twamp_sender](twamp_sender.md), although can serve as reflector for other
TWAMP-compatible implementations.

## Configuration

{{ collector_config("twamp_reflector") }}

The collector-specifig configuration is:

| Parameter | Type    | Default   | Description                         |
| --------- | ------- | --------- | ----------------------------------- |
| `listen`  | String  | `0.0.0.0` | Address to bind the control channel |
| `port`    | Integer | `862`     | Port for the control channel        |

Config example:

``` yaml
- id: TWAMP Reflector
  type: twamp_reflector
  listen: "0.0.0.0"
  port: 862
```

## Collected Metrics

=== "OpenMetrics"
  | Metric             | Metric Type | Description                            |
  | ------------------ | ----------- | -------------------------------------- |
  | `session_attempts` | Counter     | Total amount of the attempted sessions |
  | `session_started`  | Counter     | Total amount of the started sessions   |
  | `reflected_pkt`    | Counter     | Total amount of the reflected packets  |
  | `reflected_octets` | Counter     | Total amount of the reflected octets   |

## Labels

`twamp_reflector` collector doesn't append its labels, though they can be configured
via `labels` option.

## Configuring Senders

### Gufo Agent

``` yaml
collectors:
    - id: TWAMP Session
      type: twamp_sender
      interval: 10
      reflector: 127.0.0.1
      port: 862
      n_packets: 200
      model: g711
      dscp: ef
      labels:
        service: voip
```

See [twamp_sender](twamp_sender.md) for configuration options details.

## Sample Output

=== "OpenMetrics"

    ```
    # HELP twamp_reflector_reflected_octets Total amount of the reflected octets
    # TYPE twamp_reflector_reflected_octets counter
    twamp_reflector_reflected_octets 1978000 1682593154
    # HELP twamp_reflector_reflected_pkt Total amount of the reflected packets
    # TYPE twamp_reflector_reflected_pkt counter
    twamp_reflector_reflected_pkt 11500 1682593154
    # HELP twamp_reflector_session_attempts Total amount of the attempted sessions
    # TYPE twamp_reflector_session_attempts counter
    twamp_reflector_session_attempts 115 1682593154
    # HELP twamp_reflector_session_started Total amount of the started sessions
    # TYPE twamp_reflector_session_started counter
    twamp_reflector_session_started 115 1682593154
    ```

[TWAMP]: https://www.juniper.net/documentation/us/en/software/junos/flow-monitoring/topics/concept/twamp-overview.html