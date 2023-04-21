# block_io collector

`fs` collects the host's block devices' input/output statistics.

## Configuration

| Parameter  | Type    | Default | Description                                        |
| ---------- | ------- | ------- | -------------------------------------------------- |
| `id`       | String  |         | Collector's ID. Must be unique per agent instance. |
| `type`     | String  |         | Must be `block_io`                                 |
| `interval` | Integer |         | Repetition interval in seconds                     |
| `labels`   | Object  |         | Additional collector-level labels                  |

Config example:

``` yaml
- id: File System
  type: fs
  interval: 10
```

## Collected Metrics

=== "OpenMetrics"

  | Metric               | Metric Type | Labels      | Description |
  | -------------------- | ----------- | ----------- | ----------- |
  | `fs_files`           | Gauge       | mount, type | ???         |
  | `fs_files_total`     | Gauge       | mount, type | ???         |
  | `fs_files_available` | Gauge       | mount, type | ???         |
  | `fs_free`            | Gauge       | mount, type | ???         |
  | `fs_total`           | Gauge       | mount, type | ???         |
  | `fs_available`       | Gauge       | mount, type | ???         |

## Labels

`fs` collector appends the following labels:

| Label   | Description                    |
| ------- | ------------------------------ |
| `mount` | File system mount point        |
| `type`  | File system type (i.e. `ext4`) |

## Notes

fs collector ignores the following file system types:

=== "Linux"

    * `cgroup`
    * `devpts`
    * `overlay`
    * `proc`
    * `sysfs`

It also ignores all file systems mounted besides the following paths:

=== "Linux"

    * `/dev/`
    * `/proc/`
    * `/sys/`

## Sample Output