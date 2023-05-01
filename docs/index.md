# Gufo Agent

`Gufo Agent` is a lightweight software for collecting and exposing system metrics 
and performing QoS and SLA checks. We implement it in the Rust language with correctness,
performance, and low system resource usage in mind.
The `Gufo Agent` is built around four cornerstones:

* Zeroconf configuration system allows plain [YAML configuration files](configuration.md)
  in simple cases while allowing the flexible config resolution process 
  for the centralized management in more complex ones.
* The collector plugins, perform the measurements and collect the metrics.
  The API is developer-friendly, enforces correctness, and allows a rapid development process.
* Internal scheduler which runs the collectors as defined in the config.
* The sender exposes the collected metrics. The openmetrics endpoint is available out-of-the-box,
  allowing seamless [Prometheus][Prometheus] integration.

The Rust language's unique properties allowed us to build an agent which can be used 
not only for the trivial system metrics collection but for performing high-precision measurements
as well.

## Compatibility

Work in progress

## Obtaining the Gufo Agent

Work in progress

## Available Collectors

| Type                                             | Description                               |
| ------------------------------------------------ | ----------------------------------------- |
| [block_io](collectors/block_io.md)               | Block I/O devices statistics              |
| [cpu](collectors/cpu.md)                         | CPU statistics                            |
| [dns](collectors/dns.md)                         | Perform DNS request using system resolver |
| [fs](collectors/fs.md)                           | File systems statistic                    |
| [http](collectors/http.md)                       | Perform HTTP request                      |
| [memory](collectors/memory.md)                   | Host's memory statistics                  |
| [network](collectors/network.md)                 | Host's network interface statistics       |
| [twamp_reflector](collectors/twamp_reflector.md) | TWAMP reflector for SLA probing           |
| [twamp_sender](collectors/twamp_sender.md)       | TWAMP sender for SLA probing              |
| [uptime](collectors/uptime.md)                   | System uptime                             |

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