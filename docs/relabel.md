# Relabeling Rules

Relabeling is the process of the manipulation of the metrics, based on labels.
The manipulation rules are applied on the collector level via a `relabel` configuration.
The rule set contains one or more relabeling rules, each one performing one or more tasks:

* [replace](#replace) - Create or update labels basing on context of the other labels.
* [drop](#drop) - Drop matched metrics.
* [keep](#keep) - Keep matched metrics.
* [labeldrop](#label-drop) - Drop matched labels.
* [labelkeep](#label-keep) - Keep matched labels.
* [labelmap](#label-map) - Map one or more label name to different label names.

## Virtual Labels

Unlike the measure labels, virtual labels are available only during the relabeling process and are not exposed to the database. The following virtual labels are available:

| Label      | Description       |
| ---------- | ----------------- |
| `__name__` | The metric's name |

## Actions

### Replace

`replace` action creates or updates labels basing on context of the other labels. The configuration is:

| Parameter       | Default   | Description                                                                                                                            |
| --------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `action`        | `replace` | Must be `replace`                                                                                                                      |
| `source_labels` |           | The list of label names to be extracted                                                                                                |
| `separator`     | `;`       | The separator                                                                                                                          |
| `regex`         | `(.+)`    | The regular expression to match                                                                                                        |
| `replacement`   |           | The resulting expression. `regex` matching groups should be referred by number (i.e. `$1`, `$2`) or by name (i.e. `$first`, `$second`) |
| `target_label`  |           | The name of the label to be created or replaced                                                                                        |

The `replace` rule performs the following steps:

1. Extracts the values of the all labels specified in `source_labels`. 
   If any of the `source_labels` is missed, the rule is considered failed and the processing
   passed to the next rule.
2. The extracted values are concatenated together using `separator` building the value string
3. The value string is matched against the `regex`. If the `regex` is failed to match the
   rule is considered failed and processing is passed to the next rule.
4. `replacement` expression is used to build the result. It may contain the references to the
   capture groups of the `regex`:

      * `$0` - expands to the whole match.
      * `$<n>` - where `<n>` is a number: match n-th capture group. I.e. `$1`, `$2`, ...
      * `$<name>` - match a named capture group, i.e `$first`, `$last`.

5. The result of the expansion of the `replacement` is placed into label defined by `target_label`.
6. The processing is passed to the next rule.

!!! note

    The rewriting of the `__name__` virtual label changes the metric's name.

Examples:

Rewrite the `user` label as the `<user>@<zone>`:

``` yaml
- source_labels: [__name__, user, zone]
  separator: "@"
  regex: "ps_write_count@(.+)"
  replacement: "$1"
  target_label: user
  action: replace
```

Rewrite `ps_write_count` metric name to `total_writes`:

``` yaml
- source_labels: [__name__]
  regex: ps_write_count
  replacement: total_writes
  target_label: __name__
```

### Drop

`drop` actions drops matched metrics. The configuration is:

| Parameter       | Default | Description                             |
| --------------- | ------- | --------------------------------------- |
| `action`        |         | Must be `drop`                          |
| `source_labels` |         | The list of label names to be extracted |
| `separator`     | `;`     | The separator                           |
| `regex`         | `(.+)`  | The regular expression to match         |

The `replace` rule performs the following steps:

1. Extracts the values of the all labels specified in `source_labels`. 
   If any of the `source_labels` is missed, the rule is considered failed and the processing
   passed to the next rule.
2. The extracted values are concatenated together using `separator` building the value string
3. The value string is matched against the `regex`. If the `regex` is failed to match the
   rule is considered failed and processing is passed to the next rule.
4. The metric is considered as matched and discarded.
5. Processing is stopped.

Examples:

Drop `ps_write_count` metric:

``` yaml
- source_labels: [__name__]
  regex: ps_write_count
  action: drop
```

Drop `ps_write_count` metric for user `scott` in zone `tiger`:

``` yaml
- source_labels: [__name__, user, zone]
  regex: "ps_write_count;scott;tiger"
  action: drop
```

### Keep

`keep` actions keeps matched metrics. The configuration is:

| Parameter       | Default | Description                             |
| --------------- | ------- | --------------------------------------- |
| `action`        |         | Must be `keep`                          |
| `source_labels` |         | The list of label names to be extracted |
| `separator`     | `;`     | The separator                           |
| `regex`         | `(.+)`  | The regular expression to match         |

The `replace` rule performs the following steps:

1. Extracts the values of the all labels specified in `source_labels`. 
   If any of the `source_labels` is missed, the rule is considered failed and the processing
   passed to the next rule.
2. The extracted values are concatenated together using `separator` building the value string
3. The value string is matched against the `regex`. If the `regex` is failed 
   the metric is discarded and processing is stopped.
4. Otherwise, processing is passed to the next rule.

Examples:

Drop all metrics except `ps_read_count` and `ps_write_count`:

``` yaml
- source_labels: [__name__]
  regex: ps_read_count|ps_write_count
  action: keep
```

### Label Drop

`labeldrop` action drops all matching labels. The configuration is:

| Parameter | Default | Description                     |
| --------- | ------- | ------------------------------- |
| `action`  |         | Must be `labelkeep`             |
| `regex`   |         | The regular expression to match |

The `labeldrop` rule perform following steps:

1. All labels are matched against the `regex`. Matching labels are discarded.
2. Processing is passed to the next rule.

!!! note

    `labeldrop` doesn't affect [virtual labels](#virtual-labels).

Example:

Drop `user` and `zone` labels.

``` yaml
- action: labeldrop
  regex: user|zone
```

### Label Keep

`labelkeep` action drops all matching labels. The configuration is:

| Parameter | Default | Description                     |
| --------- | ------- | ------------------------------- |
| `action`  |         | Must be `labeldrop`             |
| `regex`   |         | The regular expression to match |

The `labelkeep` rule perform following steps:

1. All labels are matched against the `regex`. Matching labels are kept, not matched ones are discarded.
2. Processing is passed to the next rule.

!!! note

    `labelkeep` doesn't affect [virtual labels](#virtual-labels).

Example:

Keep only `mount` and `dev` labels.

``` yaml
- action: labelkeep
  regex: mount|dev
```

### Label Map

`labelmap` action maps one or more label names to the different label names. The configuration is:

| Parameter     | Default | Description                                                                                                                            |
| ------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `action`      |         | Must be `labelmap`                                                                                                                     |
| `regex`       |         | The regular expression to match                                                                                                        |
| `replacement` |         | The resulting expression. `regex` matching groups should be referred by number (i.e. `$1`, `$2`) or by name (i.e. `$first`, `$second`) |

The `labelmap` rule perform following steps:

1. All labels are matched against the `regex`. Matching labels are applied to the `replacement` and are renamed.
2. Processing is passed to the next rule.

Examples:

Rename `dc` label to `datacenter`:

``` yaml
- action: labelmap
  regex: dc
  replacement: datacenter
```

Rename virtual labels started with `__meta_kubernetes_`:

``` yaml
- action: labelmap
  regex: __meta_kubernetes_(.+)
  replacement: k8s_$1
```