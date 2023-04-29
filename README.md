# Gufo Agent

*Gufo Agent is the lightweight infrastructure monitoring agent, implemented in [Rust][Rust].

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
![Build](https://img.shields.io/github/actions/workflow/status/gufolabs/gufo_agent/tests.yml?branch=master)
![Sponsors](https://img.shields.io/github/sponsors/gufolabs)

---

**Documentation**: [https://docs.gufolabs.com/gufo_agent/](https://docs.gufolabs.com/gufo_agent/)

**Source Code**: [https://github.com/gufolabs/gufo_agent](https://github.com/gufolabs/gufo_agent/)

---

## Work in progress

!!! WARNING: Work in a progress

We plan to cover following issues until first public release:

### Packaging

* [ ] rpm
* [ ] deb
* [ ] windows

### Port collectors
* [x] block_io
* [x] cpu
* [x] dns
* [x] fs
* [x] http
* [x] memory
* [ ] modbus_rtu
* [ ] modbus_tcp
* [x] network
* [x] twamp_reflector
* [x] twamp_sender
* [x] uptime

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
