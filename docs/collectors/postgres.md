# postgres collector

`postgres` collects PostgreSQL instance statistics.

## Configuration

| Parameter  | Type    | Default                   | Description                                        |
| ---------- | ------- | ------------------------- | -------------------------------------------------- |
| `id`       | String  |                           | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |                           | Must be `http`                                     |
| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                     |
| `labels`   | Object  |                           | Additional collector-level labels                  |
| `host`     | String  |                           | Server instance host for TCP connection            |
| `port`     | Integer |                           | Server instance port for TCP connection            |
| `socket`   | String  |                           | Unix socket path                                   |
| `username` | String  |                           | Username to connect database                       |
| `password` | String  |                           | Password to connect database                       |


Config example:

``` yaml
- id: Postgres
disabled: false
type: postgres
host: 127.0.0.1
username: postgres
```

## Collected Metrics

=== "OpenMetrics"

    | Metric                        | Metric Type | Description                                                                                                  |
    | ----------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------ |
    | `pg_numbackends`              | Gauge       | Number of backends currently connected to this database.                                                     |
    | `pg_xact_commit`              | Counter     | Number of transactions in this database that have been committed.                                            |
    | `pg_xact_rollback`            | Counter     | Number of transactions in this database that have been rolled back.                                          |
    | `pg_blks_read`                | Counter     | Number of disk blocks read in this database.                                                                 |
    | `pg_blks_hit`                 | Counter     | Number of times disk blocks were found already in the buffer cache, so that a read was not necessary         | db); |
    | `pg_tup_returned`             | Counter     | Number of live rows fetched by sequential scans and index entries returned by index scans in this database.  | db); |
    | `pg_tup_fetched`              | Counter     | Number of live rows fetched by index scans in this database.                                                 |
    | `pg_tup_inserted`             | Counter     | Number of rows inserted by queries in this database.                                                         |
    | `pg_tup_updated`              | Counter     | Number of rows updated by queries in this database.                                                          |
    | `pg_tup_deleted`              | Counter     | Number of rows deleted by queries in this database.                                                          |
    | `pg_conflicts`                | Counter     | Number of queries canceled due to conflicts with recovery in this database                                   |
    | `pg_temp_files`               | Counter     | Number of temporary files created by queries in this database                                                |
    | `pg_temp_bytes`               | Counter     | Total amount of data written to temporary files by queries in this database                                  |
    | `pg_deadlocks`                | Counter     | Number of deadlocks detected in this database.                                                               |
    | `pg_checksum_failures`        | Counter     | Number of data page checksum failures detected in this database                                              |
    | `pg_blk_read_time`            | Counter     | Time spent reading data file blocks by backends in this database, in seconds                                 |
    | `pg_blk_write_time`           | Counter     | Time spent writing data file blocks by backends in this database, in seconds                                 |
    | `pg_session_time`             | Counter     | Time spent by database sessions in this database, in seconds                                                 |
    | `pg_active_time`              | Counter     | Time spent executing SQL statements in this database, in seconds                                             |
    | `pg_idle_in_transaction_time` | Counter     | Time spent idling while in a transaction in this database, in seconds                                        |
    | `pg_sessions`                 | Counter     | Total number of sessions established to this database.                                                       |
    | `pg_sessions_abandoned`       | Counter     | Number of database sessions to this database that were terminated because connection to the client was lost. | db); |
    | `pg_sessions_fatal`           | Counter     | Number of database sessions to this database that were terminated by fatal errors.                           |
    | `pg_sessions_killed`          | Counter     | Number of database sessions to this database that were terminated by operator intervention.                  |

## Labels

`postgres` collector appends the following labels

| Label | Description   |
| ----- | ------------- |
| `db`  | Database name |

## Sample Output

=== "OpenMetrics"

    ```
    ```