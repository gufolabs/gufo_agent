# Collectors Reference

This section contains formal reference of the supported
collectors, their configuration, generated metrics,
configuration and output samples.

## Configuration

All collectors share common part of configuration.

| Parameter  | Type    | Description                                                              |
| ---------- | ------- | ------------------------------------------------------------------------ |
| `id`       | String  | Collector's ID. Must be unique per agent instance.                       |
| `type`     | String  | The type of collector. See [Collectors](#collectors) for available types |
| `interval` | Integer | Repetition interval in seconds                                           |
| `labels`   | Object  | Additional collector-level labels                                        |

Example:

``` yaml
- id: File System
  type: fs
  interval: 10
```

`gufo-agent` allows to append user-defined labels to the collector's output. User-defined
labels are set as key-value pairs:

``` yaml
- id: dns1
  type: dns
  interval: 10
  labels:
    dc: DC1
    project: P1
- id: dns2
  type: dns
  interval: 10
  labels:
    dc: DC2
    project: P2
```


## Collectors

| Type                    | Description                               |
| ----------------------- | ----------------------------------------- |
| [block_io](block_io.md) | Block I/O devices statistics              |
| [cpu](cpu.md)           | CPU statistics                            |
| [dns](dns.md)           | Perform DNS request using system resolver |
| [fs](fs.md)             | File systems statistic                    |
| [memory](memory.md)     | Host's memory statistics                  |
| [uptime](uptime.md)     | System uptime                             |