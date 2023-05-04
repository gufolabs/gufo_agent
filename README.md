# Gufo Agent

*Gufo Agent is the lightweight infrastructure monitoring agent, implemented in [Rust][Rust].

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
![Build](https://img.shields.io/github/actions/workflow/status/gufolabs/gufo_agent/tests.yml?branch=master)
![Sponsors](https://img.shields.io/github/sponsors/gufolabs)

---

**Documentation**: [https://docs.gufolabs.com/gufo_agent/](https://docs.gufolabs.com/gufo_agent/)

**Source Code**: [https://github.com/gufolabs/gufo_agent](https://github.com/gufolabs/gufo_agent/)

---

`Gufo Agent` is a lightweight software for collecting and exposing system metrics 
and performing QoS and SLA checks. We implement it in the Rust language with correctness,
performance, and low system resource usage in mind.
The `Gufo Agent` is built around four cornerstones:

* Zeroconf configuration system allows plain YAML configuration files
  in simple cases while allowing the flexible config resolution process 
  for the centralized management in more complex ones.
* The collector plugins, perform the measurements and collect the metrics.
  The API is developer-friendly, enforces correctness, and allows a rapid development process.
* Internal scheduler which runs the collectors as defined in the config.
* The sender exposes the collected metrics. The openmetrics endpoint is available out-of-the-box,
  allowing seamless Prometheus integration.

The Rust language's unique properties allowed us to build an agent which can be used 
not only for the trivial system metrics collection but for performing high-precision measurements
as well.

## Compatibility

Work in progress

## Obtaining the Gufo Agent

Work in progress

## Running

To generate the default configuration use:

```
gufo-agent --config-discovery > config.yml
```

See "Config Discovery" for details.

Then run the Gufo Agent:

```
gufo-agent --config=config.yml
```

See "Gufo Agent Man Pages" for details.

## Available Collectors

| Type              | Description                               |
| ----------------- | ----------------------------------------- |
| `block_io`        | Block I/O devices statistics              |
| `cpu`             | CPU statistics                            |
| `dns`             | Perform DNS request using system resolver |
| `fs`              | File systems statistic                    |
| `http`            | Perform HTTP request                      |
| `memory`          | Host's memory statistics                  |
| `network`         | Host's network interface statistics       |
| `sockets`         | Host's sockets statistics                 |
| `twamp_reflector` | TWAMP reflector for SLA probing           |
| `twamp_sender`    | TWAMP sender for SLA probing              |
| `uptime`          | System uptime                             |

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
