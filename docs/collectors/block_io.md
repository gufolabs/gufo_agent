# block_io collector

`block_io` collects the host's block devices' input/output statistics.

## Configuration

| Parameter  | Type    | Default                   | Description                                        |
| ---------- | ------- | ------------------------- | -------------------------------------------------- |
| `id`       | String  |                           | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |                           | Must be `block_io`                                 |
| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                     |
| `labels`   | Object  |                           | Additional collector-level labels                  |

Config example:

``` yaml
- id: Block I/O
  type: block_io
```

## Collected Metrics

=== "OpenMetrics"

  | Metric             | Metric Type | Labels | Description                                      |
  | ------------------ | ----------- | ------ | ------------------------------------------------ |
  | `io_read_ios`      | Gauge       | dev    | Number of read I/Os processed                    |
  | `io_read_merges`   | Gauge       | dev    | Number of read I/Os merged with in-queue I/O     |
  | `io_read_sectors`  | Gauge       | dev    | Number of sectors read                           |
  | `io_read_ticks`    | Gauge       | dev    | Total wait time for read requests, ms            |
  | `io_write_ios`     | Gauge       | dev    | Number of write I/Os processed                   |
  | `io_write_merges`  | Gauge       | dev    | Number of write I/Os merged with in-queue I/O    |
  | `io_write_sectors` | Gauge       | dev    | Number of sectors written                        |
  | `io_write_ticks`   | Gauge       | dev    | Total wait time for write requests, ms           |
  | `io_in_flight`     | Gauge       | dev    | Number of I/Os currently in flight, requests     |
  | `io_ticks`         | Gauge       | dev    | Total time this block device has been active, ms |
  | `io_time_in_queue` | Gauge       | dev    | Total wait time for all requests, ms             |

## Labels

`block_io` collector appends the following labels:

| Label | Description       |
| ----- | ----------------- |
| `dev` | Block device name |

## Config Discovery

`block_io` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-block_io
```

## Sample Output

=== "OpenMetrics"

    ```
    # HELP block_io_in_flight Number of I/Os currently in flight, requests
    # TYPE block_io_in_flight gauge
    block_io_in_flight{dev="dm-0} 0 1682413629
    block_io_in_flight{dev="dm-1} 0 1682413629
    block_io_in_flight{dev="dm-2} 0 1682413629
    # HELP block_io_io_ticks Total time this block device has been active, ms
    # TYPE block_io_io_ticks gauge
    block_io_io_ticks{dev="dm-0"} 1091288 1682413629
    block_io_io_ticks{dev="dm-1"} 681716 1682413629
    block_io_io_ticks{dev="dm-2"} 577728 1682413629
    # HELP block_io_read_ios Number of read I/Os processed
    # TYPE block_io_read_ios counter
    block_io_read_ios{dev="dm-0"} 180155 1682413629
    block_io_read_ios{dev="dm-1"} 17722 1682413629
    block_io_read_ios{dev="dm-2"} 20178 1682413629
    ```