# The _unCORE_ Operating System Kernel

## Introduction

Welcome to the official _unCORE_ operating system kernel documentation. _unCORE_ is an [operating system] [kernel] completely written in pure, idiomatic [Rust]. _unCORE_ makes use of the [Rust] ecosystem, avoiding unnecessary complexity while being stable and performant. If you're new to this project, we highly recommend reading the [Getting Started][docs-getting-started] section. Everything you need to know about development guidelines can be found under [Development][docs-development]. The [Building][docs-building] site contains information on how to build and run (with QEMU) the kernel. The [Structure][docs-structure] section contains all the information about the kernel's internal structure and composition. This documentation is only one half of the whole documentation that is available. The other part is the code documentation which can be built with `#!bash cargo doc --open`.

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

## Vision

_unCORE_ is not trying to invent the wheel anew. As of now, _unCORE_ is an educational project that does not run real software. We want to change this in the future. _unCORE_ shall make use of well-known and common concepts used in _UNIX_ / _GNU-Linux_. But, we acknowledge that modern software development is heavily benefitting of CI pipelines, GIT platforms (such as _GitHub_) and collaboration in the form of issues, pull requests, projects and other actions. While we know that mailing lists work, we belief that modern software development can do better. One aspect we heavily focus on is code quality (in the same way that _Linus Torvalds_ has ensured the code quality in the _Linux_ kernel). We are using  automated CI to achieve this goal too. You will, when you start out, notice that CI is very restrictive. This may get on your nerves, but ensures all code in this project is as clean as possible. The motto here is: **We either do it right or not at all**. Please also read the [conventions set in this project](development.md#miscellaneous) to ensure you're up-to-date when it comes to writing real code.

## Goals

_unCORE_ makes use of the [Rust] ecosystem, avoiding unnecessary complexity while being stable and fast. The main goals are

1. Robustness - [Rust] provides a stable foundation with strong compile-time guarantees.
2. Safety - The code in _unCORE_ especially focuses on being safe in the sense of not allowing exploitation.

_unCORE_ has set itself some more goals, of course:

1. Performance - [Rust] provides C++ / C performance.
2. Correctness - _unCORE_ has high test standards, see [the documentation and testing advise](./development.md#code-documentation-testing).
3. Simplicity - We want to make use of [Rust]'s high level of abstraction to write clean and concise code.

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
    This project is licensed under the [GNU General Public License v3], except for those parts (lines of code from used libraries) already licensed under other licenses. Moreover, code taken from [_Phillip Oppermann_'s _BlogOS_ project][blog-os] is not covered by the license of this project as well.

[//]: # (Links)

[docs-getting-started]: #getting-started
[docs-development]: ./development.md
[docs-building]: ./building.md
[docs-structure]: ./kernel_structure.md

[operating system]: https://en.wikipedia.org/wiki/Operating_system
[kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[Rust]: https://www.rust-lang.org/
[official Rust book]: https://doc.rust-lang.org/book/

[QEMU]: https://www.qemu.org/
[Docker]: https://www.docker.com/
[Podman]: https://podman.io/

[blog-os]: https://os.phil-opp.com/
[GNU General Public License v3]: https://www.gnu.org/licenses/gpl-3.0.txt
