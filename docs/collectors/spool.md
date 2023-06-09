# spool collector

`spool` reads files in [OpenMetrics](../openmetrics.md) format from a directory and
exposes collected data. The files are scanned in alphabetical order, parsed, 
and removed (unless the `dry_run` option is set). See 
[OpenMetrics Format Specification](../openmetrics.md) for the recognized
file format.

## Configuration

{{ collector_config("spool") }}

The collector-specific configuration is:

| Parameter          | Type    | Default | Description                             |
| ------------------ | ------- | ------- | --------------------------------------- |
| `path`             | String  |         | Path to the spool directory             |
| `trust_timestamps` | Bool    | `false` | Ignore timestamps in output, if `false` |
| `dry_run`          | Boolean | `false` | If set to `true` - do not remove files  |

Config example:

``` yaml
- id: Spool
  type: spool
  path: /var/gufo_agent/spool
```

## Collected Metrics

=== "OpenMetrics"
  | Metric               | Metric Type | Description                       |
  | -------------------- | ----------- | --------------------------------- |
  | `spool_jobs`         | Counter     | Total spool jobs processed        |
  | `spool_jobs_success` | Counter     | Spool jobs processed successfully |
  | `spool_jobs_failed`  | Counter     | Spool jobs failed to process      |
  | `spool_parsed`       | Counter     | Parsed metric items               |

  In addition to the own metrics `spool` exposes metrics read from the files.

## Labels

`spool` collector appends the following labels:

| Label  | Description          |
| ------ | -------------------- |
| `path` | Spool directory path |

## Sample Output

=== "OpenMetrics"

    ```
    # HELP job1 Result of the running of job1
    # TYPE job1 gauge
    job1{collector="spool"} 15 1683550695
    # HELP job2 Result of the running of job2
    # TYPE job2 gauge
    job2{collector="spool"} 15.9 1683550695
    # HELP spool_jobs Total spool jobs processed
    # TYPE spool_jobs counter
    spool_jobs{collector="spool",path="var/spool"} 1 1683550695
    # HELP spool_jobs_failed Spool jobs failed to process
    # TYPE spool_jobs_failed counter
    spool_jobs_failed{collector="spool"path="var/spool"} 0 1683550695
    # HELP spool_jobs_success Spool jobs processed successfully
    # TYPE spool_jobs_success counter
    spool_jobs_success{collector="spool",path="var/spool"} 1 1683550695
    # HELP spool_parsed Parsed metric items
    # TYPE spool_parsed counter
    spool_parsed{collector="spool",path="var/spool"} 2 1683550695
    # EOF
    ```