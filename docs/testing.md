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

## Building Documentation

To rebuild and check documentation run

```
$ mkdocs serve
```

We recommend using [Grammarly][Grammarly] service to check
documentation for common errors.

[Grammarly]: https://grammarly.com/