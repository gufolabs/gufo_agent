# scrape collector

`scrape` collects data from OpenMetrics/Prometheus endpoints.

## Configuration

{{ collector_config("scrape") }}


The collector-specific configuration is:

| Parameter           | Type    | Default | Description                                           |
| ------------------- | ------- | ------- | ----------------------------------------------------- |
| `service_discovery` | Object  |         | [Service Discovery](#service-discovery) configuration |
| `trust_timestamps`  | Bool    | `false` | Ignore timestamps in output, if `false`               |
| `concurrency`       | Integer | `10`    | Limit amount of parralel tasks                        |

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

Target endpoints are obtained via the *Service Discovery*  process.
Service Discovey returns the set of labels for

The common labels available for all types of the service discovery are:

| Label              | Desciption                              |
| ------------------ | --------------------------------------- |
| `__address__`      | `<host>:<port>` for a target service    |
| `__meta_sd_schema` | Request schema: `http` or `https`       |
| `__meta_sd_path`   | Metrics endpoint path (i.e. `/metrics`) |

### Static

`static` discovery returns addresses set in `targets` configuration parameter.

Configuration:

| Parameter | Type   | Default    | Description                                             |
| --------- | ------ | ---------- | ------------------------------------------------------- |
| `type`    | String |            | Must be `static`                                        |
| `targets` | Array  |            | List of targets in `<host>:<port>` format               |
| `relabel` | Array  |            | [Relabeling Rules](../relabel.md) for service discovery |
| `schema`  | String | `http`     | Default service endpoint schema                         |
| `path`    | String | `/metrics` | Default service endpoint path                           |

Example:

``` yaml
service_discovery:
  type: static
  targets:
    - "agent1:3000"
    - "agent2:3000"
```

### DNS

`dns` discovery performs DNS queries to resolve the targets.

Configuration:

| Parameter    | Type    | Default | Description                            |
| ------------ | ------- | ------- | -------------------------------------- |
| `type`       | String  |         | Must be `dns`                          |
| `query`      | String  |         | DNS query                              |
| `query_type` | String  | `A`     | DNS query type. Either `A` or `SRV`    |
| `port`       | Integer |         | Target port. Mandatory for `A` queries |

Example (`A` type):

``` yaml
service_discovery:
  type: dns
  query: agent1.local
  port: 3000
```

Example (`SRV` type):

``` yaml
service_discovery:
  type: dns
  query: _dnssd._tcp.ga.test.gufolabs.com
  query_type: SRV
```

### Consul

`consul` discovery allows retrieving targets from 
[Consul Agent API](https://developer.hashicorp.com/consul/api-docs/agent/service#list-services)

Configuration:

| Parameter       | Type   | Default          | Description                                  |
| --------------- | ------ | ---------------- | -------------------------------------------- |
| `type`          | String |                  | Must be `consul`                             |
| `server`        | String | `127.0.0.1:8500` | Consul's server addreess                     |
| `filter`        | String |                  | Optional Consul query filter                 |
| `tag_separator` | String | `,`              | Separator to join `__meta_consul_tags` label |

`consul` discovery defines additional labels for relabeling process:

| Label                           | Desciption                                 |
| ------------------------------- | ------------------------------------------ |
| `__meta_consul_dc`              | Consul datacenter                          |
| `__meta_consul_service`         | Consul service name                        |
| `__meta_consul_service_id`      | Consul service id                          |
| `__meta_consul_service_address` | Target address                             |
| `__meta_consul_service_port`    | Target port                                |
| `__meta_consul_tags`            | The list of tags joined by `tag_separator` |
| `__meta_consul_meta_<key>`      | Each service metadata key/value for target |

Example:

``` yaml
service_discovery:
  type: consul
  server: "127.0.0.1:8500"
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
