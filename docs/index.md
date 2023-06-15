# Gufo Agent

`Gufo Agent` is a lightweight software for collecting and exposing system metrics 
and performing QoS and SLA checks. We implement it in the Rust language with correctness,
performance, and low system resource usage in mind.
The `Gufo Agent` is built around four cornerstones:

* Zeroconf configuration system allows plain [YAML configuration files](configuration.md)
  in simple cases while allowing the flexible config resolution process 
  for the centralized management in more complex ones. 
  The sophisticated [Config Discovery](config_discovery.md)
  allows the automatic generation of config in most cases.
* The collector plugins, perform the measurements and collect the metrics.
  The API is developer-friendly, enforces correctness, and allows a rapid development process.
* Internal scheduler which runs the collectors as defined in the config.
* The sender exposes the collected metrics. The openmetrics endpoint is available out-of-the-box,
  allowing seamless [Prometheus][Prometheus] integration.

The Rust language's unique properties allowed us to build an agent which can be used 
not only for the trivial system metrics collection but for performing high-precision measurements
as well.

## Compatibility

Supported platforms:

| OS    | libc        | Arch    |
| ----- | ----------- | ------- |
| Linux | glibc 2.17+ | amd64   |
| Linux | glibc 2.17+ | aarch64 |

## Obtaining the Gufo Agent

View the [CHANGELOG](CHANGELOG.md) for the latest updates
and changes by version.

### Automated Install

```
curl https://sh.gufolabs.com/ga | sh
```

### Binary Downloads

The binary downloads are available from each of the
[Github Releases](https://github.com/gufolabs/gufo_agent/releases)
page in the "Assets" section.

### Building from Source

1. Clone Gufo Agent repository
```
git clone https://github.com/gufolabs/gufo_agent.git
```
2. Go to Gufo Agent directory
```
cd gufo_agent
```
3. Install proper Rust toolchain
```
./tools/build/setup-rust.sh
```
4. Build
```
cargo build --release
```
5. Resulting binary location is `./target/release/gufo-agent`

## Running

To generate the default configuration use:

```
gufo-agent --config-discovery > config.yml
```

See [Config Discovery](config_discovery.md) for details.

Then run the Gufo Agent:

```
gufo-agent --config=config.yml
```

See [Gufo Agent Man Pages](man.md) for details.

## Available Collectors

| Type                                             | Description                                       |
| ------------------------------------------------ | ------------------------------------------------- |
| [block_io](collectors/block_io.md)               | Block I/O devices statistics                      |
| [cpu](collectors/cpu.md)                         | CPU statistics                                    |
| [dns](collectors/dns.md)                         | Perform DNS request using system resolver         |
| [exec](collectors/exec.md)                       | Execute command and read output                   |
| [fs](collectors/fs.md)                           | File systems statistic                            |
| [http](collectors/http.md)                       | Perform HTTP request                              |
| [memory](collectors/memory.md)                   | Host's memory statistics                          |
| [modbus_rtu](collectors/modbus_rtu.md)           | Perform Modbus RTU requests                       |
| [modbus_tcp](collectors/modbus_tcp.md)           | Perform Modbus TCP requests                       |
| [mysql](collectors/mysql.md)                     | MySQL instance statistics                         |
| [mysql_query](collectors/mysql_query.md)         | Perform MySQL queries                             |
| [network](collectors/network.md)                 | Host's network interface statistics               |
| [pgbouncer](collectors/pgbouncer.md)             | PgBouncer statistics                              |
| [postgres](collectors/postgres.md)               | PostgreSQL instance statistics                    |
| [postgres_query](collectors/postgres_query.md)   | Perform PostgreSQL queries                        |
| [procstat](collectors/procstat.md)               | Process' statistics                               |
| [redis](collectors/redis.md)                     | Redis instance statistics                         |
| [scrape](collectors/scrape.md)                   | Fetch data from OpenMetrics/Prometheus endpoints  |
| [sockets](collectors/sockets.md)                 | Host's sockets statistics                         |
| [spool](collectors/spool.md)                     | Read files in openmetrics format from a directory |
| [twamp_reflector](collectors/twamp_reflector.md) | TWAMP reflector for SLA probing                   |
| [twamp_sender](collectors/twamp_sender.md)       | TWAMP sender for SLA probing                      |
| [uptime](collectors/uptime.md)                   | System uptime                                     |

## On Gufo Stack

This product is a part of [Gufo Stack][Gufo Stack] - the collaborative effort 
led by [Gufo Labs][Gufo Labs]. Our goal is to create a robust and flexible 
set of tools to create network management software and automate 
routine administration tasks.

To do this, we extract the key technologies that have proven themselves 
in the [NOC][NOC] and bring them as separate packages. Then we work on API,
performance tuning, documentation, and testing. The [NOC][NOC] uses the final result
as the external dependencies.

[Gufo Stack][Gufo Stack] makes the [NOC][NOC] better, and this is our primary task. But other products
can benefit from [Gufo Stack][Gufo Stack] too. So we believe that our effort will make 
the other network management products better.

[Gufo Labs]: https://gufolabs.com/
[Gufo Stack]: https://gufolabs.com/products/gufo-stack/
[NOC]: https://getnoc.com/
[Rust]: https://rust-lang.org/
[Prometheus]: https://prometheus.io