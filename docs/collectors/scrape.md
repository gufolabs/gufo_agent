# scrape collector

`scrape` collects data from OpenMetrics/Prometheus endpoints.

## Configuration

{{ collector_config("scrape") }}


The collector-specific configuration is:

| Parameter           | Type   | Default | Description                                           |
| ------------------- | ------ | ------- | ----------------------------------------------------- |
| `service_discovery` | Object |         | [Service Discovery](#service-discovery) configuration |
| `trust_timestamps`  | Bool   | `false` | Ignore timestamps in output, if `false`               |

Config example:

``` yaml
- id: scrape
  disabled: false
  type: scrape
  service_discovery:
    type: static
    targets:
      - "127.0.0.1:3000"
      - "127.0.0.1:3001"
```

## Service Discovery

Target enpoints are obtained via the *Service Discovery*  process.

### Static

`static` discovery returns addresses set in `targets` configuration parameter.

Configuration:

| Parameter | Type   | Default | Description                               |
| --------- | ------ | ------- | ----------------------------------------- |
| `type`    | String |         | Must be `static`                          |
| `targets` | Array  |         | List of targets in `<host>:<port>` format |

Example:

``` yaml
service_discovery:
  type: static
  targets:
    - "agent1:3000"
    - "agent2:3000"
```

## Collected Metrics

`scrape` collector re-exposes collected metrics.

## Labels

`scrape` collector appends the following labels

| Label         | Description                      |
| ------------- | -------------------------------- |
| `__address__` | `<address>:<port>` of the source |

## Sample Output

=== "OpenMetrics"

    ```
    ```
