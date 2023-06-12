# uptime collector

`uptime` collects the host's system uptime.

## Configuration

{{ collector_config("uptime") }}

Config example:

``` yaml
- id: Uptime
  type: uptime
```

## Collected Metrics

=== "OpenMetrics"
  | Metric          | Metric Type | Description               |
  | --------------- | ----------- | ------------------------- |
  | `uptime_uptime` | Counter     | System uptime, in seconds |

## Labels

`uptime` collector doesn't append its own labels.

## Config Discovery

`uptime` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-uptime
```

## Sample Output

=== "OpenMetrics"

    ```
    # HELP uptime_uptime System uptime
    # TYPE uptime_uptime counter
    uptime_uptime 149461 1682413628
    ```