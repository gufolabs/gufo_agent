# block_io collector

`block_io` collects host's block devices input/output statistics.

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

| Metric          | Metric Type | Labels | Description |
| --------------- | ----------- | ------ | ----------- |
| `read_ios`      |             | dev    |             |
| `read_merges`   |             | dev    |             |
| `read_sectors`  |             | dev    |             |
| `read_ticks`    |             | dev    |             |
| `write_ios`     |             | dev    |             |
| `write_merges`  |             | dev    |             |
| `write_sectors` |             | dev    |             |
| `write_ticks`   |             | dev    |             |
| `in_flight`     |             | dev    |             |
| `io_ticks`      |             | dev    |             |
| `time_in_queue` |             | dev    |             |

## Labels

`block_io` collector appends following labels:

| Label | Description       |
| ----- | ----------------- |
| `dev` | Block device name |

## Sample Output