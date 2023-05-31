# Gufo Agent Man Page

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
      --hostname <HOSTNAME>  [env: GA_HOSTNAME=]
      --list-collectors
      --dump-metrics     [env: GA_DUMP_METRICS=]
      --config-discovery
      --config-discovery-opts <CONFIG_DISCOVERY_OPTS>  [env: GA_CONFIG_DISCOVERY_OPTS=]
      --config-scripts <CONFIG_SCRIPTS>
      --test
      --check
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
* <a name="opt_hostname"></a>`--hostname` - Override agent's hostname.
* <a name="opt_config"></a>`--list-collectors` - Print list of compiled collectors and exit.
* <a name="opt_dump_metrics"></a>`--dump-metrics` - Dump metrics database state to stdout after each collector run.
* <a name="opt_config_discovery"></a>`--config-discovery` - Run config discovery, dump resulting config to stdout and exit. See [Config Discovery](config_discovery.md) for details.
* <a name="opt_config_discovery_opts"></a>`--config-discovery-opts` - Optional config discovery configuration. See [Config Discovery](config_discovery.md) for details.
* <a name="opt_config_scripts"></a>`--config-scripts` - A path to the directory containing
  config discovery scripts. See [Config Discovery](config_discovery.md) for details.
* `--test` - Test run. Lauch all configured collectors once, dump resulting database and exit.
* `--check` - Test configuration and return non-zero code on error.
* `-h`, `--help` - Print help and exit.
* `-V`, `--version` - Print agent version and exit.

## Environment

The following environment variables affect the execution of `gufo-agent`:

* `GA_CONFIG` - same as [`--config`](#opt_config) option.
* `GA_DUMP_METRICS` - same as [`--dump-metrics`](#opt_dump_metrics) option.
* `GA_HOSTNAME` - same as [`--hostname`](#opt_hostname) option.
* `GA_INSECURE` - same as [`--insecure`](#opt_insecure) option.
* `GA_CONFIG_DISCOVERY_OPTS` - same as [`--config-discovery-opts`](#opt_config_discovery_opts) option.
* `GA_CONFIG_SCRIPTS` - a colon-sparated list of directories, containing discovery scripts.

## Signals

* `SIGHUP` - reload configuration.

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

Generate config:

```
$ gufo-agent --config-discovery > config.yml
```

Run:

```
$ gufo-agent -vv --config=/etc/gufo-agent/config.yml
```