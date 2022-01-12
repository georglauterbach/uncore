<p align="center">
  <img
    height="25%" width="25%"
    alt="unCORE Operating System Kernel Logo"
    src="./documentation/content/images/logo.png">
</p>

# unCORE

[![License][badge::license]][badge::licence::link] [![Documentation][badge::documentation]][badge::documentation::link]

[![CI][badge::ci::kernel-code-tests]][badge::ci::kernel-code-tests::link] [![CI Linting][badge::ci::security]][badge::ci::security::link] [![CI Linting][badge::ci::linting]][badge::ci::linting::link]

## About

_unCORE_ is an [operating system] [kernel] completely written in pure, idiomatic [Rust]. _unCORE_ makes use of the [Rust] ecosystem, avoiding unnecessary complexity while being stable and performant.

## Getting Started

The documentation has a dedicated [Getting Started][docs-getting-started] section. You will need to have [Rust]'s basic tools (in the form of `rustc`, `rustup` and `cargo`) installed. To run a pre-defined set of commands (building, linting, formatting, testing, etc.), you may use [Just], a command runner. It is **highly recommended** to install [Just] in order to make working with _unCORE_ easier. You can then run `just help` to get an overview of available commands to run.

To check whether you have all needed tools, and install them if you do not already have them installed, run

``` CONSOLE
$ pwd
/uncore
$ ./scripts/install_tools.sh
tools        INFO    'rustup' is installed
tools        SUCCESS Your Rust installation is complete
...
```

## Documentation and Licensing

The documentation is written in Markdown, built with [MkDocs] and can be found under `documentation/`. You may build and serve the documentation locally with a container runtime (like [Docker] or [Podman]) by running `./scripts/documentation.sh`, serving it under <http://127.0.0.1:8080>.

This project is licensed under the [GNU General Public License v3], **except** for those parts (lines of code from libraries used in this project) already licensed under other licenses.

[//]: # (Badges)

[badge::license]: https://img.shields.io/github/license/georglauterbach/uncore.svg?label=LICENSE&color=informational&style=for-the-badge
[badge::licence::link]: ./LICENSE
[badge::documentation]: https://img.shields.io/badge/DOCUMENTATION-MKDOCS-informational?style=for-the-badge
[badge::documentation::link]: https://georglauterbach.github.io/uncore/edge/

[badge::ci::linting]: https://img.shields.io/github/workflow/status/georglauterbach/uncore/Linting?label=CI%20-%20Linting&logo=github&logoColor=white&style=for-the-badge
[badge::ci::linting::link]: https://github.com/georglauterbach/uncore/actions/workflows/linting.yml

[badge::ci::security]: https://img.shields.io/github/workflow/status/georglauterbach/uncore/Security%20Audit?label=CI%20-%20Security%20Audit&logo=github&logoColor=white&style=for-the-badge
[badge::ci::security::link]: https://github.com/georglauterbach/uncore/actions/workflows/security.yml

[badge::ci::kernel-code-tests]: https://img.shields.io/github/workflow/status/georglauterbach/uncore/Kernel%20Code%20Tests?label=CI%20-%20Kernel%20Code%20Tests&logo=github&logoColor=white&style=for-the-badge
[badge::ci::kernel-code-tests::link]: https://github.com/georglauterbach/uncore/actions/workflows/kernel-tests.yml

[//]: # (Links)

[docs-getting-started]: ./documentation/content/index.md#getting-started

[operating system]: https://en.wikipedia.org/wiki/Operating_system
[kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[Rust]: https://www.rust-lang.org/

[Just]: https://github.com/casey/just

[MkDocs]: https://www.mkdocs.org/
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/
[GNU General Public License v3]: https://www.gnu.org/licenses/gpl-3.0.txt
