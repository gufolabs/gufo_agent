# pgbouncer collector

`pgbouncer` collects PgBouncer statistics.

## Configuration

{{ collector_config("pgbouncer") }}

The collector-specific configuration is:

| Parameter  | Type    | Default     | Description                             |
| ---------- | ------- | ----------- | --------------------------------------- |
| `host`     | String  |             | Server instance host for TCP connection |
| `port`     | Integer |             | Server instance port for TCP connection |
| `socket`   | String  |             | Unix socket path                        |
| `username` | String  |             | Username to connect database            |
| `password` | String  |             | Password to connect database            |
| `database` | String  | `pgbouncer` | The name of pgbouncer internal database |


Config example:

``` yaml
- id: Pgbouncer
  type: pgbouncer
  host: 127.0.0.1
  username: postgres
  password: secret
```

## Collected Metrics

=== "OpenMetrics"

    | Metric                  | Metric Type | Description                                                                                                                               |
    | ----------------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
    | `pgb_total_xact_count`  |             | Total number of SQL transactions pooled by pgbouncer                                                                                      |
    | `pgb_total_query_count` |             | Total number of SQL queries pooled by pgbouncer                                                                                           |
    | `pgb_total_received`    |             | Total volume in bytes of network traffic received by pgbouncer                                                                            |
    | `pgb_total_sent`        |             | Total volume in bytes of network traffic sent by pgbouncer                                                                                |
    | `pgb_total_xact_time`   |             | Total number of seconds spent by pgbouncer when connected to PostgreSQL in a transaction, either idle in transaction or executing queries |
    | `pgb_total_query_time`  |             | Total number of seconds spent by pgbouncer when actively connected to PostgreSQL, executing queries                                       |
    | `pgb_total_wait_time`   |             | Time spent by clients waiting for a server, in seconds                                                                                    |
);
## Labels

`pgbouncer` collector appends the following labels

| Label | Description   |
| ----- | ------------- |
| `db`  | Database name |

## PgBouncer Setup

Ensure your `pgbouncer.ini` has the following sections:

```
stats_users=<username>
ignore_startup_parameters=extra_float_digits
```

Where the `<username>` must match to the collector's configured `username`.

## Sample Output

=== "OpenMetrics"

    ```
    ```