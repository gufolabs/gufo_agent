# Agent Man Page

## Name

`gufo-agent` - an universal agent for infrastructure monitoring.


## Synopsys

```
Usage: gufo-agent [OPTIONS]

Options:
  -q, --quiet            
  -v, --verbose...       
  -k, --insecure         [env: GA_INSECURE=]
  -c, --config <CONFIG>  [env: GA_CONFIG=]
      --list-collectors  
      --dump-metrics     [env: GA_DUMP_METRICS=]
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
* <a name="opt_insecure"></a>`-c`, `--config` `<CONFIG>` - Load configuration from `<CONFIG>` path.
* <a name="opt_config"></a>`--list-collectors` - Print list of compiled collectors and exit.
* <a name="opt_dump_metrics"></a>`--dump-metrics` - Dump metrics database state to stdout after each collector run.
* `-h`, `--help` - Print help and exit.
* `-V`, `--version` - Print agent version and exit.

## Environment

The following environment variables affect the execution of `gufo-agent`:

* `GA_CONFIG` - same as [`--config`](#opt_config) option.
* `GA_DUMP_METRICS` - same as [`--dump-metrics`](#opt_dump_metrics) option.
* `GA_INSECURE` - same as [`--insecure`](#opt_insecure) option.

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