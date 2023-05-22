# exec collector

`exec` runs command and parses its stdout as [OpenMetrics](../openmetrics.md) format.
See [OpenMetrics Format Specification](../openmetrics.md) for the recognized
format.

## Configuration

| Parameter  | Type    | Default                   | Description                                           |
| ---------- | ------- | ------------------------- | ----------------------------------------------------- |
| `id`       | String  |                           | Collector's ID. Must be unique per agent instance.    |
| `type`     | String  |                           | Must be `sockets`                                     |
| `interval` | Integer | `agent.defaults.interval` | Repetition interval in seconds                        |
| `labels`   | Object  |                           | Additional collector-level labels                     |
| `cmd`      | List    |                           | Command and its arguments. Each as separate list item |
| `cd`       | String  |                           | Change working directory, if set                      |
| `env`      | Object  |                           | Set environment variables, if set                     |

Config example:

``` yaml
- id: Script
  type: exec
  cmd:
    - ./examples/scripts/collector/sample.sh
  env:
    VAR1: value1
    VAR2: value2
```

## Collected Metrics

=== "OpenMetrics"
  | Metric        | Metric Type | Description         |
  | ------------- | ----------- | ------------------- |
  | `exec_parsed` | Counter     | Parsed metric items |

  In addition to the own metrics `exec` exposes metrics read from the files.

## Labels

`exec` collector appends the following labels:

| Label    | Description     |
| -------- | --------------- |
| `script` | Script executed |

## Sample Output

=== "OpenMetrics"

    ```
    # HELP exec_parsed Parsed metric items
    # TYPE exec_parsed counter
    exec_parsed{agent="gufo",collector="exec",host="d20e7299d8e1",script="./examples/scripts/collector/sample.sh",zone="DC1"} 8 1683613059
    # HELP job_failed Failed job
    # TYPE job_failed counter
    job_failed{agent="gufo",collector="exec",dc="east",dept="business",host="d20e7299d8e1",zone="DC1"} 1 1683613059
    job_failed{agent="gufo",collector="exec",dc="west",dept="tech",host="d20e7299d8e1",zone="DC1"} 4 1683613059
    # HELP job_success Successful job
    # TYPE job_success counter
    job_success{agent="gufo",collector="exec",dc="east",dept="business",host="d20e7299d8e1",zone="DC1"} 12 1683613059
    job_success{agent="gufo",collector="exec",dc="west",dept="tech",host="d20e7299d8e1",zone="DC1"} 12 1683613059
    # EOF
    ```