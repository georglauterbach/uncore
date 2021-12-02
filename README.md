<p align="center">
  <img
    height="25%" width="25%"
    alt="unCORE Operating System Kernel Logo"
    src="./documentation/content/images/logo.png">
</p>

# unCORE

## About

**_unCORE_** is an [operating system] [micro-kernel] completely written in pure, idiomatic [Rust]. **_unCORE_** makes use of the [Rust] ecosystem, avoiding unnecessary complexity while being stable and performant.

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

[//]: # (Links)

[docs-getting-started]: ./documentation/content/index.md#getting-started
[docs-modules]: ./documentation/content/modules/modules.md

[operating system]: https://en.wikipedia.org/wiki/Operating_system
[micro-kernel]: https://en.wikipedia.org/wiki/Microkernel
[Rust]: https://www.rust-lang.org/

[Just]: https://github.com/casey/just

[MkDocs]: https://www.mkdocs.org/
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/
[GNU General Public License v3]: https://www.gnu.org/licenses/gpl-3.0.txt
[blog-os]: https://github.com/phil-opp/blog_os#license
