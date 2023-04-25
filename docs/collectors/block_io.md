# block_io collector

`block_io` collects the host's block devices' input/output statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
|------------|---------|---------|----------------------------------------------------|
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

  | Metric                   | Metric Type | Labels | Description                                      |
  |--------------------------|-------------|--------|--------------------------------------------------|
  | `block_io_read_ios`      | Gauge       | dev    | Number of read I/Os processed                    |
  | `block_io_read_merges`   | Gauge       | dev    | Number of read I/Os merged with in-queue I/O     |
  | `block_io_read_sectors`  | Gauge       | dev    | Number of sectors read                           |
  | `block_io_read_ticks`    | Gauge       | dev    | Total wait time for read requests, ms            |
  | `block_io_write_ios`     | Gauge       | dev    | Number of write I/Os processed                   |
  | `block_io_write_merges`  | Gauge       | dev    | Number of write I/Os merged with in-queue I/O    |
  | `block_io_write_sectors` | Gauge       | dev    | Number of sectors written                        |
  | `block_io_write_ticks`   | Gauge       | dev    | Total wait time for write requests, ms           |
  | `block_io_in_flight`     | Gauge       | dev    | Number of I/Os currently in flight, requests     |
  | `block_io_io_ticks`      | Gauge       | dev    | Total time this block device has been active, ms |
  | `block_io_time_in_queue` | Gauge       | dev    | Total wait time for all requests, ms             |

## Labels

`block_io` collector appends the following labels:

| Label | Description       |
|-------|-------------------|
| `dev` | Block device name |

## Sample Output

=== "OpenMetrics"

    ```
    # HELP block_io_in_flight Number of I/Os currently in flight, requests
    # TYPE block_io_in_flight gauge
    block_io_in_flight{agent="gufo",dev="dm-0",host="ek-light",zone="DC1"} 0 1682413629
    block_io_in_flight{agent="gufo",dev="dm-1",host="ek-light",zone="DC1"} 0 1682413629
    block_io_in_flight{agent="gufo",dev="dm-2",host="ek-light",zone="DC1"} 0 1682413629
    # HELP block_io_io_ticks Total time this block device has been active, ms
    # TYPE block_io_io_ticks gauge
    block_io_io_ticks{agent="gufo",dev="dm-0",host="ek-light",zone="DC1"} 1091288 1682413629
    block_io_io_ticks{agent="gufo",dev="dm-1",host="ek-light",zone="DC1"} 681716 1682413629
    block_io_io_ticks{agent="gufo",dev="dm-2",host="ek-light",zone="DC1"} 577728 1682413629
    # HELP block_io_read_ios Number of read I/Os processed
    # TYPE block_io_read_ios counter
    block_io_read_ios{agent="gufo",dev="dm-0",host="ek-light",zone="DC1"} 180155 1682413629
    block_io_read_ios{agent="gufo",dev="dm-1",host="ek-light",zone="DC1"} 17722 1682413629
    block_io_read_ios{agent="gufo",dev="dm-2",host="ek-light",zone="DC1"} 20178 1682413629
    ```