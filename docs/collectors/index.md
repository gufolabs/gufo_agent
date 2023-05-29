# Collectors Reference

This section contains the formal reference of the supported
collectors, their configuration, generated metrics,
configuration and output samples.

## Configuration

All collectors share a common part of the configuration.

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

`gufo-agent` allows appending user-defined labels to the collector's output. User-defined
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

| Type                                  | Description                                       |
| ------------------------------------- | ------------------------------------------------- |
| [block_io](block_io.md)               | Block I/O devices statistics                      |
| [cpu](cpu.md)                         | CPU statistics                                    |
| [dns](dns.md)                         | Perform DNS request using system resolver         |
| [exec](exec.md)                       | Execute command and read output                   |
| [fs](fs.md)                           | File systems statistic                            |
| [http](http.md)                       | Perform HTTP request                              |
| [memory](memory.md)                   | Host's memory statistics                          |
| [modbus_rtu](modbus_rtu.md)           | Perform Modbus RTU requests                       |
| [modbus_tcp](modbus_tcp.md)           | Perform Modbus TCP requests                       |
| [network](network.md)                 | Host's network interface statistics               |
| [pgbouncer](pgbouncer.md)             | PgBouncer statistics                              |
| [postgres](postgres.md)               | PostgreSQL instance statistics                    |
| [postgres_query](postgres_query.md)   | Perform PostgreSQL queries                        |
| [procstat](procstat.md)               | Process' statistics                               |
| [sockets](sockets.md)                 | Host's sockets statistics                         |
| [spool](spool.md)                     | Read files in openmetrics format from a directory |
| [twamp_reflector](twamp_reflector.md) | TWAMP reflector for SLA probing                   |
| [twamp_sender](twamp_sender.md)       | TWAMP sender for SLA probing                      |
| [uptime](uptime.md)                   | System uptime                                     |
