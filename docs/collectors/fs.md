# fs collector

`fs` collects the host's filesystems' statistics.

## Configuration

{{ collector_config("fs") }}

Config example:

``` yaml
- id: File System
  type: fs
```
## Config Discovery

`fs` collector supports the [Config Discovery](../config_discovery.md) by default.
To disable a particular block use the `--config-discovery-opts` option:

``` shell
gufo-agent --config-discovery --config-discovery-opts=-fs
```

## Collected Metrics

=== "OpenMetrics"

  | Metric                | Metric Type | Labels      | Description                 |
  | --------------------- | ----------- | ----------- | --------------------------- |
  | `fs_inodes`           | Gauge       | mount, type | Inodes used                 |
  | `fs_inodes_total`     | Gauge       | mount, type | Total inodes count          |
  | `fs_inodes_available` | Gauge       | mount, type | Inodes available            |
  | `fs_free`             | Gauge       | mount, type | Free disk space, bytes      |
  | `fs_total`            | Gauge       | mount, type | Total disk space, bytes     |
  | `fs_available`        | Gauge       | mount, type | Available disk space, bytes |

## Labels

`fs` collector appends the following labels:

| Label   | Description                    |
| ------- | ------------------------------ |
| `dev`   | Device path                    |
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