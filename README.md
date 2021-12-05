<p align="center">
  <img
    height="25%" width="25%"
    alt="unCORE Operating System Kernel Logo"
    src="./documentation/content/images/logo.png">
</p>

# unCORE

[![License][badge::license]][badge::licence::link] [![Documentation][badge::documentation]][badge::documentation::link]

[![CI Linting][badge::ci::linting]][badge::ci::link]

## About

_unCORE_ is an [operating system] [kernel] completely written in pure, idiomatic [Rust]. _unCORE_ makes use of the [Rust] ecosystem, avoiding unnecessary complexity while being stable and performant.

## Getting Started

The documentation has a dedicated [Getting Started][docs-getting-started] section. You will need to have [Rust]'s basic tools (in the form of `rustc`, `rustup` and `cargo`) installed. To run a pre-defined set of commands (building, linting, formatting, testing, etc.), you may use [Just], a command runner. You can then run `just help` to get an overview of available commands to run.

To check whether you have all needed tools, and install them if you do not already have them installed, run

``` CONSOLE
$ ./scripts/tools.sh
tools INFO    'rustup' is installed
tools INFO    Setting Rust toolchain and ...
...
```

from this repository's root directory.

## Repository Structure

This repository is structured into different modules:

``` TXT
/
├── .github/        # GitHub's issue and pull request templates
├── documentation/  # documentation resides here
├── modules/        # kernel modules and Rust code
└── scripts/        # holds all Bash scripts for administration
```

A dedicated [Modules][docs-modules] section in the documentation covers the contents of the modules in which [Rust] code resides.

## Documentation and Licensing

The documentation is written in Markdown, built with [MkDocs] and can be found under `documentation/`. You may build and serve the documentation locally with a container runtime (like [Docker] or [Podman]) by running `./scripts/documentation.sh`, serving it under <http://127.0.0.1:8080>.

This project is licensed under the [GNU General Public License v3], except for those parts (lines of code from libraries used in this project) already licensed under other licenses. Moreover, code taken from [_Phillip Oppermann_'s _BlogOS_ project][blog-os] is not covered by the license of this project as well.

[//]: # (Badges)

[badge::license]: https://img.shields.io/github/license/georglauterbach/uncore.svg?label=LICENSE&color=informational&style=for-the-badge
[badge::licence::link]: https://github.com/georglauterbach/uncore/blob/master/LICENSE
[badge::documentation]: https://img.shields.io/badge/DOCUMENTATION-MKDOCS-informational?style=for-the-badge
[badge::documentation::link]: https://github.com/georglauterbach/uncore/tree/master/documentation

[badge::ci::link]: https://github.com/docker-mailserver/docker-mailserver/actions
[badge::ci::linting]: https://img.shields.io/github/workflow/status/georglauterbach/uncore/Linting?label=CI%20-%20Linting&logo=github&logoColor=white&style=for-the-badge

[//]: # (Links)

[docs-getting-started]: ./documentation/content/index.md#getting-started
[docs-modules]: ./documentation/content/modules/modules.md

[operating system]: https://en.wikipedia.org/wiki/Operating_system
[kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[Rust]: https://www.rust-lang.org/

[Just]: https://github.com/casey/just

[MkDocs]: https://www.mkdocs.org/
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/
[GNU General Public License v3]: https://www.gnu.org/licenses/gpl-3.0.txt
[blog-os]: https://github.com/phil-opp/blog_os#license
