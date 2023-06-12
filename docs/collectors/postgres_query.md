# postgres collector

`postgres_query` performs queries over PostgreSQL database and exposes results as a metrics.

## Configuration

{{ collector_config("postgres_query") }}

The collector-specific configuration is:

| Parameter                  | Type                | Default | Description                                                                              |
| -------------------------- | ------------------- | ------- | ---------------------------------------------------------------------------------------- |
| `host`                     | String              |         | Server instance host for TCP connection                                                  |
| `port`                     | Integer             |         | Server instance port for TCP connection                                                  |
| `socket`                   | String              |         | Unix socket path                                                                         |
| `database`                 | String              |         | Database name                                                                            |
| `username`                 | String              |         | Username to connect database                                                             |
| `password`                 | String              |         | Password to connect database                                                             |
| `items`                    | Array {{ complex }} |         | List of query configurations                                                             |
| {{ tab }} `query`          | String              |         | SQL query                                                                                |
| {{ tab }} `name`           | String              |         | Metric name. Overriden by `name_column`.                                                 |
| {{ tab }} `name_column`    | String              |         | Column with metric name. Overrides `name` configuration.                                 |
| {{ tab }} `help`           | String              |         | Metric help. Overriden by `help_column`.                                                 |
| {{ tab }} `help_column`    | String              |         | Optional column with metric help. Overrides `help` connfiguration.                       |
| {{ tab }} `value_column`   | String              | `value` | Column with metric value                                                                 |
| {{ tab }} `labels`         | Object              |         | Additional item-level labels.                                                            |
| {{ tab }} `labels_columns` | Array               |         | List of column names which to be exposed as labels. Label name matches with column name. |

Config example:

``` yaml
  - id: Query
    type: postgres_query
    host: postgres
    username: postgres
    password: secret
    database: metrics
    items:
      # name, value, and help from table
      - query: SELECT name, value, help FROM metrics
        help_column: help
      # Static name, help, and labels
      - query: SELECT 42 as value
        name: meaning_of_life
        help: The most important question
        labels:
          region: galaxy
          transport: autostop
      # Static name. Labels and value are from query
      - query: SELECT dept, region, SUM(value) AS value FROM expenses GROUP BY 1, 2
        name: expenses
        help: Expenses by department and the region
        label_columns: [dept, region]
```

## Collected Metrics

`postgres_queery` doesn't impose the specific format of the metrics. The generated
metrics are fully configurable.

## Labels

Metric labels depend on configures queries.

## Database Table Requirements

* Metric name field must be of text type: CHAR, VARCHAR, TEXT.
* Value field must must be of type: SMALLINT, SMALLSERIAL, INT2, INT, INT4, SERIAL,
  BIGINT, BIGSERIAL, INT8, REAL, FLOAT4, DOUBLE PRECISION, FLOAT8, NUMERIC.
* Help field is optional and must be any text type: CHAR, VARCHAR, TEXT.

## Example

Create metrics table or view with the following structure:

``` sql
CREATE TABLE metrics(
    name VARCHAR(256), 
    help VARCHAR(256), 
    value INTEGER
);
```

Populate with data:

``` sql
INSERT INTO metrics(name, help, value)
VALUES
    ('myapp_read', 'Total reads', 12),
    ('myapp_write', 'Total writes', 28),
    ('myapp_delete', 'Total deletes', 1);
```

## Sample Output

=== "OpenMetrics"

    ```
    ```