# postgres collector

`postgres_query` performs queries over PostgreSQL database and exposes results as a metrics.

## Configuration

| Parameter                | Type                | Default                   | Description                                        |
| ------------------------ | ------------------- | ------------------------- | -------------------------------------------------- |
| `id`                     | String              |                           | Collector's ID. Must be unique per agent instance. |
| `type`                   | String              |                           | Must be `http`                                     |
| `interval`               | Integer             | `agent.defaults.interval` | Repetition interval in seconds                     |
| `labels`                 | Object              |                           | Additional collector-level labels                  |
| `host`                   | String              |                           | Server instance host for TCP connection            |
| `port`                   | Integer             |                           | Server instance port for TCP connection            |
| `socket`                 | String              |                           | Unix socket path                                   |
| `database`               | String              |                           | Database name                                      |
| `username`               | String              |                           | Username to connect database                       |
| `password`               | String              |                           | Password to connect database                       |
| `items`                  | Array {{ complex }} |                           | List of query configurations                       |
| {{ tab }} `query`        | String              |                           | SQL query                                          |
| {{ tab }} `name_column`  | String              | `name`                    | Column with metric name                            |
| {{ tab }} `value_column` | String              | `value`                   | Column with metric value                           |
| {{ tab }} `help_column`  | String              |                           | Optional column with metric help                   |

Config example:

``` yaml
  - id: Query
    type: postgres_query
    host: postgres
    port: 6432
    username: postgres
    password: secret
    database: metrics
    items:
      - query: SELECT name, value, help FROM metrics
        help_column: help
```

## Collected Metrics

`postgres_queery` doesn't impose the specific format of the metrics. The generated
metrics are fully configurable.

## Labels

Metric labels depend on configures queries.

## Sample Output

=== "OpenMetrics"

    ```
    ```