# Developer's Tools

Gufo Agent contains several developer tools which can be used to speedup development.

## configure.py

```
./tools/dev/configure.py
```

Check dependencies version and expand code templates. Raises an error
if the several crates depends on the different versions of the same dependency.

Should be used every time when rust code templates is changed. Launched
automatically by `new-collector.py`

## list-deps.py

```
./tools/dev/list-deps.py
```
Parses and prints all dependencies versions and appropriate crates:

```
...
* bigdecimal v0.3:
    collectors/pgbouncer
    collectors/postgres_query
* bytes v1.4:
    agent
    collectors/twamp_reflector
    collectors/twamp_sender
    proto/connection
    proto/frame
    proto/twamp
    proto/udp
* cfg-if v1.0:
    collectors/memory
...
```

To show single dependency use it as parameter

```
$ ./tools/dev/list-deps.py nom
* nom v7.1:
    proto/openmetrics
```

## new-collector.py

```
./tools/dev/new-collector.py <name>
```

Create boilerplate for a new collector `<name>`.
Calls `configure.py`

## setup-rust.sh

```
./tools/build/setup-rust.sh
```

Install and set-up proper version of the Rust toolchain.
Called by GitHub CI and from Dockerfile.

## build-all.sh

Install Cross and build all supported distributions.
Called by GitHub CI.
