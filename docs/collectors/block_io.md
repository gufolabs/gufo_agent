# block_io collector

`block_io` collects the host's block devices' input/output statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
| ---------- | ------- | ------- | -------------------------------------------------- |
| `id`       | String  |         | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |         | Must be `block_io`                                 |
| `interval` | Integer |         | Repetition interval in seconds                     |
| `labels`   | Object  |         | Additional collector-level labels                  |

Config example:

``` yaml
- id: Block I/O
  type: block_io
  interval: 10
```

## Collected Metrics

=== "OpenMetrics"

  | Metric                   | Metric Type | Labels | Description |
  | ------------------------ | ----------- | ------ | ----------- |
  | `block_io_read_ios`      | Gauge       | dev    | ???         |
  | `block_io_read_merges`   | Gauge       | dev    | ???         |
  | `block_io_read_sectors`  | Gauge       | dev    | ???         |
  | `block_io_read_ticks`    | Gauge       | dev    | ???         |
  | `block_io_write_ios`     | Gauge       | dev    | ???         |
  | `block_io_write_merges`  | Gauge       | dev    | ???         |
  | `block_io_write_sectors` | Gauge       | dev    | ???         |
  | `block_io_write_ticks`   | Gauge       | dev    | ???         |
  | `block_io_in_flight`     | Gauge       | dev    | ???         |
  | `block_io_io_ticks`      | Gauge       | dev    | ???         |
  | `block_io_time_in_queue` | Gauge       | dev    | ???         |

## Labels

`block_io` collector appends the following labels:

| Label | Description       |
| ----- | ----------------- |
| `dev` | Block device name |

## Sample Output