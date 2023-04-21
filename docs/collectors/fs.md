# block_io collector

`fs` collects host's block devices input/output statistics.

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
  | `fs_files`           |             | mount, type |             |
  | `fs_files_total`     |             | mount, type |             |
  | `fs_files_available` |             | mount, type |             |
  | `free`               |             | mount, type |             |
  | `total`              |             | mount, type |             |
  | `available`          |             | mount, type |             |

## Labels

`fs` collector appends following labels:

| Label   | Description                    |
| ------- | ------------------------------ |
| `mount` | File system mount point        |
| `type`  | File system type (i.e. `ext4`) |

## Notes

fs collector ignores following file system types:

=== "Linux"

    * `cgroup`
    * `devpts`
    * `overlay`
    * `proc`
    * `sysfs`

It also ignores all file systems mounted besides following paths:

=== "Linux"

    * `/dev/`
    * `/proc/`
    * `/sys/`

## Sample Output