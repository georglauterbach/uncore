# The _unCORE_ Operating System Kernel

## Introduction

Welcome to the official _unCORE_ operating system kernel documentation. _unCORE_ is an [operating system] [kernel] completely written in pure, idiomatic _Rust_. _unCORE_ makes use of the _Rust_ ecosystem, avoiding unnecessary complexity while being stable and performant. If you're new to this project, we highly recommend reading the [Getting Started][docs::getting-started] section. Everything you need to know about development guidelines can be found under [Development][docs::development]. The [Building and Running][docs::building-and-running] site contains information on how to build and run the kernel (with QEMU). The [Kernel Architecture][docs::kernel-architecture] section contains all the information about the kernel's internal structure and composition. The [Testing][docs::testing] page contains information on unit- and integration tests. This documentation is only one half of the whole documentation that is available. The other part is the code documentation which can be built with `#!bash cargo doc --open`.

!!! check "Code of Conduct and Contributing Guidelines"
    By working on this projects and with other participants, you agree to the **code of conduct** and the **contributing guidelines** set by this project.

!!! danger "Developer Instructions"
    Make sure you read the [**Development Guidelines**][docs::development] _carefully_. Adhering to a consistent style and conventions allows everyone to work efficiently.

## Getting Started

### Overview

Please read the full [`README`](https://github.com/georglauterbach/uncore/blob/master/README.md) of this project carefully. We expect you to have _some_ experience with _Rust_. You need not be an expert, but we require you to understand the basics. We highly recommend you to read the [official Rust book].

### Repository Structure

This repository is structured into different subdirectories:

``` BASH
uncore/
├── .github/        # GitHub's templates and CI workflows
├── documentation/  # full kernel documentation
├── kernel/         # kernel files including all Rust code
└── scripts/        # holds all Bash scripts (building, running, administration, etc.)
```

### Tooling

You may execute the `tools.sh` scripts if you're on Linux. This script is invoked by running `./scripts/install_tools.sh` from the repository root. It will install

1. a complete _Rust_ installation including
    - [`rustup`](https://rustup.rs/) - the _Rust_ toolchain installer
    - [`rustc`](https://doc.rust-lang.org/rustc/what-is-rustc.html) - the _Rust_ compiler
    - [`cargo`](https://doc.rust-lang.org/cargo/) - the _Rust_ package manager
2. [`just`](https://github.com/casey/just), a command runner

The script will also check whether you have [Docker] or [Podman] installed. These tools are needed to lint the code and to work the rendered version of the documentation. The script will _not_ install the container runtimes for you. Also, make sure you have [QEMU] for your OS installed.

If you're on Windows or macOS, you will need to install these tools yourself.

### The Documentation

The documentation is written in Markdown, built with [MkDocs] and found under `documentation/`. You may build and serve the documentation locally with a container runtime (like [Docker] or [Podman]) by running `just docs` or `./scripts/documentation.sh serve`, serving it under <http://127.0.0.1:8080>. The documentation for the latest commit on `master` can be found under <https://georglauterbach.github.io/uncore/edge/>.

## Vision

_unCORE_ is not trying to invent the wheel anew. As of now, _unCORE_ is an educational project that does not run real software. We want to change this in the future. _unCORE_ shall make use of well-known and common concepts used in _UNIX_ and _GNU-Linux_. But, we acknowledge that modern software development is heavily benefitting of CI pipelines, GIT (and its platforms such as _GitHub_) and collaboration in the form of issues, pull requests, projects and other actions. While we know that mailing lists work, we belief that modern software development can do better. One aspect we heavily focus on is code quality. The motto here is: **We either do it right or not at all**. Please also read the [conventions set by this project](development.md#miscellaneous) to ensure you're up-to-date when it comes to writing real code.

## Goals

_unCORE_ makes use of the _Rust_ ecosystem, avoiding unnecessary complexity while being stable and fast. The main goals are

1. Robustness - _Rust_ provides a stable foundation with strong compile-time guarantees.
2. Safety - The code in _unCORE_ especially focuses on being safe in the sense of not allowing exploitation.

_unCORE_ has set itself some more goals, of course:

1. Performance - _Rust_ provides C++ / C performance.
2. Correctness - _unCORE_ has high test standards, see [the documentation and testing advise](./development.md#code-documentation-testing).
3. Simplicity - We want to make use of _Rust_'s high level of abstraction to write clean and concise code.

## Principles

Our vision and goals are guided by a few ((software) development) principles, such as

1. _[K.I.S.S.]_ - Keep it simple, stupid
2. _[Convention over Configuration]_ - Only specify unconventional aspects of the application
3. _[Ockham's Razor]_ - Entities should not be multiplied beyond necessity
4. _[D.R.Y.]_ - Don't repeat yourself
5. _[Law of Demeter]_ - Loose coupling
6. _[Software Craftsmanship]_ - A response by software developers to the perceived ills of the mainstream software industry
7. _[Principle of Least Surprise]_ - A component of a system should behave in a way that most users will expect it to behave

[K.I.S.S.]: https://en.wikipedia.org/wiki/KISS_principle
[Convention over Configuration]: https://en.wikipedia.org/wiki/Convention_over_configuration
[Ockham's Razor]: https://en.wikipedia.org/wiki/Occam%27s_razor
[D.R.Y.]: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself
[Law of Demeter]: https://en.wikipedia.org/wiki/Law_of_Demeter
[Software Craftsmanship]: https://en.wikipedia.org/wiki/Software_craftsmanship
[Principle of Least Surprise]: https://en.wikipedia.org/wiki/Principle_of_least_astonishment

## Architecture

_unCORE_ is neither a microkernel -- in the sense that there is no policy in the kernel -- not a complete monolithic kernel. This has several reasons: Firstly, we think that some of the ideas of microkernels are worthwhile using, such as concepts like POLA (Principle of Least Authority). On the other hand, there is currently not enough development power to make _unCORE_ a monolithic kernel.

_unCORE_ aspires to be cleanly separated into **loosely coupled modules with high cohesion**. Loose coupling ensures that it is possible to change modules themselves or for one another without major code changes in other modules. High cohesion ensures that a single module does not integrate functionality that does fall under the area of responsibility of another modules.

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
  ┌───────────┸─────────────────────┸───────────────────────┸──────────────────────┸───────────┐
  │                                                                                            │
  │ KERNEL CORE COMPONENTS                                                                     │
  │                                                                                            │
  │ ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────┐ │
  │ │                   │  │                   │  │                   │  │                   │ │
  │ │ MEMORY            │  │ SCHEDULING        │  │ PROCESSES         │  │ INTER-PROCESS     │ │
  │ │ MANAGEMENT        │  │                   │  │                   │  │ COMMUNICATION     │ │
  │ │                   │  │                   │  │                   │  │                   │ │
  │ └───────────────────┘  └───────────────────┘  └───────────────────┘  └───────────────────┘ │
  │                                                                                            │
  └───────────┰──────────────────────┰──────────────────────┰──────────────────────┰───────────┘
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

This work was and is heavily inspired by [_Phillip Oppermann_'s _BlogOS_][blog-os] project. The purpose of _unCORE_ is to explore Rust's capabilities, get a better understanding of how Rust and operating system kernels work, and to provide a kernel implementation.

!!! info "Licensing"
    This project is licensed under the [GNU General Public License v3], **except** for those parts (lines of code from used libraries) already licensed under other licenses.

[//]: # (Links)

[docs::getting-started]: #getting-started
[docs::development]: ./development.md
[docs::building-and-running]: ./building_and_running.md
[docs::kernel-architecture]: ./architecture/overview.md
[docs::testing]: ./testing.md

[operating system]: https://en.wikipedia.org/wiki/Operating_system
[kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[official Rust book]: https://doc.rust-lang.org/book/

[MkDocs]: https://www.mkdocs.org/
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/

[QEMU]: https://www.qemu.org/

[blog-os]: https://os.phil-opp.com/
[GNU General Public License v3]: https://www.gnu.org/licenses/gpl-3.0.txt
