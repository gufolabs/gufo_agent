# Gufo Agent's Code Base

The code base of the project has following structure:

* `.devcontainer/` - Developer's container configuration for 
  [VSCode Remote Containers][Remote Containers]. Just reopen
  project in remote container to get ready-to-development
  environment.
* `.github/` - GitHub settings

    * `workflows/` - [GitHub Actions Workflows][GitHub Workflows] settings.
      Used to run tests and build the documentation.

* `.requirements/` - Python dependencies for development environment.
  
    * `docs.txt` - [Mkdocs Material][Mkdocs Material] dependencies.

* `agent/` - Agent implementation crate.
* `collectors/` - Built-in collectors' implementation crates.
* `common/` - `common` crate shared between all collectors and agent.
* `docs/` - [Mkdocs][Mkdocs] documentation.
* `examples/` - Various examples.
* `main/` - Main crate. Parses command-line and runs agent. Compiles to binary.
* `proto/` - Various protocols implementation which can be shared between collectors.

    * `connection` - `Connection` struct. A wrapper of TCP Connection, input, and output buffers.
    * `emodel` - G.107 E-Model calculations and constants.
    * `frame` - `FrameReader` and `FrameWriter` traits.
    * `openmetrics` - OpenMetrics parser.
    * `ps` - Process statistics collection.
    * `tos` - DSCP/ToS constants.
    * `twamp` - TWAMP protocol frames.
    * `udp` - `UdpConnection` wrapper.

* `.gitignore` - [Gitignore][Gitignore] file.
* `Cargo.toml` - Project's workspace configuration.
* `Dockerfile` - [Dockerfile][Dockerfile] for development container.
* `macros.py` - Project's macroses for [Mkdocs][Mkdocs].
* `mkdocs.yml` - [Mkdocs][Mkdocs] configuration file.
 
[Remote Containers]: https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers
[GitHub Workflows]: https://docs.github.com/en/actions/using-workflows
[Mkdocs]: https://www.mkdocs.org
[Mkdocs Material]: https://squidfunk.github.io/mkdocs-material/
[Dockerfile]: https://docs.docker.com/engine/reference/builder/
[Gitignore]: https://git-scm.com/docs/gitignore