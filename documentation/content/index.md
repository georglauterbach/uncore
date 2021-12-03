# The _unCORE_ Operating System Kernel

## Introduction

Welcome to the official **_unCORE_** operating system kernel documentation. **_unCORE_** is an [operating system] [micro-kernel] completely written in pure, idiomatic [Rust]. **_unCORE_** makes use of the [Rust] ecosystem, avoiding unnecessary complexity while being stable and performant. If you're new to this project, we highly recommend reading the [Getting Started][docs-getting-started] section. Everything you need to know about development guidelines can be found under [Development][docs-development]. The [Modules][docs-modules] section contains all the information about the kernel modules in **_unCORE_**. This documentation is only one half of the whole documentation that is available. The other part is the code documentation which can be built with `#!bash cargo doc --open`.

!!! check "Code of Conduct and Contributing Guidelines"
    By working on this projects and with other participants, you agree to the **code of conduct** and the **contributing guidelines** set by this project.

!!! danger "Developer Instructions"
    Make sure you read the [**Development Guidelines**][docs-development] _carefully_. Adhering to a consistent style and conventions allows everyone to work efficiently.

## Getting Started

### Overview

Please read the full [`README`](https://github.com/georglauterbach/uncore/blob/master/README.md) of this project carefully. We expect you to have _some_ experience with [Rust]. You need not be an expert, but we require you to understand the basics. We highly recommend you to read the [official Rust book].

### Tooling

You may execute the `tools.sh` scripts if you're on Linux. This script can be invoked by running `./scripts/tools.sh` from the repository root. It will install

1. A complete [Rust] installation including
    - [`rustup`](https://rustup.rs/) - the [Rust] toolchain installer
    - [`rustc`](https://doc.rust-lang.org/rustc/what-is-rustc.html) - the [Rust] compiler
    - [`cargo`](https://doc.rust-lang.org/cargo/) - the [Rust] package manager
2. [`bootimage`](https://github.com/rust-osdev/bootimage), a binary installed via [`cargo`](https://doc.rust-lang.org/cargo/) to work easily with [QEMU]
3. [`just`](https://github.com/casey/just), a command runner installed via [`cargo`](https://doc.rust-lang.org/cargo/) as well

The script will also check whether you have [Docker] or [Podman] installed. These tools are needed to lint the code and to work the rendered version of the documentation. The script will _not_ install the container runtimes for you. Also, make sure you have [QEMU] for your OS installed.

If you're on Windows or macOS, you will need to install these tools yourself.

## Goals

**_unCORE_** makes use of the [Rust] ecosystem, avoiding unnecessary complexity while being stable and fast. The five main goals are

1. Robustness - TODO.
2. Safety - TODO.
3. Performance - TODO.
4. Correctness - TODO.
5. Simplicity - TODO.

## Vision

TODO.

## Architecture

**_unCORE_** aspires to be cleanly separated into **loosely coupled modules with high cohesion**. Loose coupling ensures that it is possible to change modules themselves or for one another without major code changes in other modules. High cohesion ensures that a single module does not integrate functionality that does fall under the area of responsibility of another modules.

The architectural overview in a "lateral view" is depicted in the following illustration where `─` depicts the component separation and `┃` information and control flow:

``` TXT
  USER SPACE                                    ┃
  ──────────────────────────────────────────────╂───────────────────────────────────────────────
  KERNEL SPACE                                  ┃
                                                ┃
  ┌─────────────────────────────────────────────┸──────────────────────────────────────────────┐
  │                                                                                            │
  │                                SYSTEM CALL INTERFACE [SCI]                                 │
  │                                                                                            │
  └───────────┰─────────────────────┰───────────────────────┰──────────────────────┰───────────┘
              ┃                     ┃                       ┃                      ┃
  ┌───────────┸─────────────────────┸───────────────────────┸──────────────────────┸───────────┐
  │                                                                                            │
  │ KERNEL                                                                                     │
  │                                                                                            │
  │ ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐ │
  │ │                   │  │                   │  │                   │  │                   │ │
  │ │ MEMORY            │  │ SCHEDULING        │  │ PROCESSES         │  │ INTER-PROCESS     │ │
  │ │ MANAGEMENT        │  │                   │  │                   │  │ COMMUNICATION     │ │
  │ │                   │  │                   │  │                   │  │                   │ │
  │ └───────────────────┘  └───────────────────┘  └───────────────────┘  └───────────────────┘ │
  │                                                                                            │
  └───────────┰──────────────────────┰──────────────────────┰──────────────────────┰───────────┘
              ┃                      ┃                      ┃                      ┃
  ┌───────────┸──────────────────────┸──────────────────────┸──────────────────────┸───────────┐
  │                                                                                            │
  │                             HARDWARE ABSTRACTION LAYER [HAL]                               │
  │                                                                                            │
  └─────────────────────────────────────────────┰──────────────────────────────────────────────┘
                                                ┃
  KERNEL SPACE                                  ┃
  ──────────────────────────────────────────────╂────────────────────────────────────────────────
  HARDWARE                                      ┃
```

## Disclaimers

This work was and is heavily inspired by [_Phillip Oppermann_'s _BlogOS_][blog-os] project. The purpose of **_unCORE_** is to explore Rust's capabilities, get a better understanding of how Rust and operating system kernels work, and to provide a kernel implementation.

!!! info "Licensing"
    This project is licensed under the [GNU General Public License v3], except for those parts (lines of code from used libraries) already licensed under other licenses. Moreover, code taken from [_Phillip Oppermann_'s _BlogOS_ project][blog-os] is not covered by the license of this project as well.

[//]: # (Links)

[docs-getting-started]: #getting-started
[docs-development]: ./development.md
[docs-modules]: ./modules/modules.md

[operating system]: https://en.wikipedia.org/wiki/Operating_system
[micro-kernel]: https://en.wikipedia.org/wiki/Microkernel
[Rust]: https://www.rust-lang.org/
[official Rust book]: https://doc.rust-lang.org/book/

[QEMU]: https://www.qemu.org/
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/

[blog-os]: https://os.phil-opp.com/
[GNU General Public License v3]: https://www.gnu.org/licenses/gpl-3.0.txt
