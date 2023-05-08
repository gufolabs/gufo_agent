# OpenMetrics Format Specification

OpenMetrics is today's de-facto standard for transmitting cloud-native metrics at scale.
For now, Gufo Agent supports only text representation for reading metrics from third-party
sources and to expose all collected metrics.

This chapter gets a quick introduction into the format and inteded for external plugins
developers. See [OpenMetrics Specification][Specification] for a formal explanation.

Lets see a simple example:

``` txt title="sample1.txt" linenums="1"
--8<-- "examples/samples/sample1.txt"
```

The file consist of the `metric families`. Each family starts with descriptors:

``` txt title="sample1.txt" linenums="1" hl_lines="1 2"
--8<-- "examples/samples/sample1.txt"
```

followed by the samples:

``` txt title="sample1.txt" linenums="1" hl_lines="3 4"
--8<-- "examples/samples/sample1.txt"
```

## Descriptors

``` txt title="sample1.txt" linenums="1" hl_lines="1 2"
--8<-- "examples/samples/sample1.txt"
```

The descriptor line has format:

```
# <type> <metric_name> <value>
```

Where:

* `<type>` is a type of descriptor.
* `<metric_name>` is a name of the following metric.
* `<value>` - descriptor-specific value.

Gufo Agent recognizes following descriptor type:

* `HELP` - Brief textual description of the metrics family.
``` txt title="sample1.txt" linenums="1" hl_lines="1"
--8<-- "examples/samples/sample1.txt"
```

* `TYPE` - Metric type. One of:
``` txt title="sample1.txt" linenums="1" hl_lines="2"
--8<-- "examples/samples/sample1.txt"
```
  
    * `gauge` - Measurement result.
    * `counter` - Incrementaly increased counter.

* `UNIT` - Measurement units, now ignored.

## Samples

``` txt title="sample1.txt" linenums="1" hl_lines="3 4"
--8<-- "examples/samples/sample1.txt"
```

Each metric family consists of one or more samples. Sample line has following format:

```
<metric_name>[<labels>] <value>[ <timestamp>]
```

Where:

* `<metric_name>` - Name of the metric.
* `<labels>` - Optional labels are enclosed between `{` and `}`.
* `<value>` - Measured value.
* `<timestamp>` - Optional timestamp in seconds from the UNIX epoch.

## EOF mark

``` txt title="sample1.txt" linenums="1" hl_lines="9"
--8<-- "examples/samples/sample1.txt"
```

End-of-file mark placed to the end of the output. Though it is advisored for the
Gufo Agent, other OpenMetrics implementations may require it.


[Specification]: https://github.com/OpenObservability/OpenMetrics/blob/main/specification/OpenMetrics.md