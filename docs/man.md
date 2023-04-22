# Agent Man Page

## Name

`gufo-agent` - an universal agent for infrastructure monitoring.


## Synopsys

```
Usage: gufo-agent [OPTIONS]

Options:
  -q, --quiet
  -v, --verbose...
  -k, --insecure
  -c, --config <CONFIG>
      --list-collectors
      --dump-metrics
  -h, --help             Print help
  -V, --version          Print version
```

## Description

The `gufo-agent` is an universal agent for infrastructure monitoring, which
collects the configured metrics and exposes them to the OpenMetrics compatible
endpoint.

The following options are available:

* `-q`, `--quiet` - Be quiet and disable logging.
* `-v`, `--verbose` - Increase logging verbosity. Repeat option to increase verbosity further.
  The appropriate logging levels:

    * `-v` - info.
    * `-vv` - debug.

* `-k`, `--insecure` - Do not check TLS certificate when fetching config over HTTPS.
* `-c`, `--config` `<CONFIG>` - Load configuration from `<CONFIG>` path.
* `--list-collectors` - Print list of compiled collectors and exit.
* `--dump-metrics` - Dump metrics database state to stdout after each collector run.
* `-h`, `--help` - Print help and exit.
* `-V`, `--version` - Print agent version and exit.

## Environment

## Exit Status

The `gufo-agent` returns:

* `0` - on successful exit.
* `1` - on error.
* `2` - on invalid command-line option.

## Examples

Show version:

```
$ gufo-agent --version
gufo-agent 0.9.0
```

Show available collectors:

```
$ gufo-agent --list-collectors
block_io
cpu
dns
uptime
```

Run:

```
$ gufo-agent -vv --config=/etc/gufo-agent/config.yml
```