# Building and Testing

## Building

=== "Production"

    ```
    cargo build --release
    ```

    The final result will be placed to `./target/release/gufo-agent`

=== "Development"

    ```
    cargo build
    ```

    The final result will be place to `./target/debug/gufo-agent`

## Linting

Fast check:

```
cargo check
```

More detailed check with clippy

```
cargo clippy
```

## Running tests

```
cargo test
```

## Profiling

### Prerequisites

Before running profiler install Valgrind:

```
apt-get install build-essential valgrind
```

Then install `iai-callgrind-runner`:

```
cargo install --version 0.3.1 iai-callgrind-runner
```

Docker devcontainer must be run in `--priveleged` mode, i.e.
ensure, that `.devcontainer/devcontainer.json` has proper `runArgs` setting:

```
    "runArgs": [
        "--init",
        "--privileged"
    ],
```

### Run Benchmarks

Run all benchmarks:

```
cargo bench
```

Run benchmarks for specific crate  (i.e. `twamp`):

```
cargo bench -p twamp
```

## Building Documentation

To rebuild and check documentation run

```
$ mkdocs serve
```

We recommend using [Grammarly][Grammarly] service to check
documentation for common errors.

[Grammarly]: https://grammarly.com/