---
hide:
  - navigation
tags:
  - home
  - introduction
  - overview
---

# The unCORE Operating System Kernel

## :wave: Introduction

Welcome to the official _unCORE_ operating system kernel documentation. _unCORE_ is

- an educational, modern [**operating system kernel**][www::wiki::operating-system-kernel]
- completely written in pure, idiomatic [**Rust**][www::homepage::rust] (and assembly where required),
- licensed under the [**GNU Public License v3 or later**][www::homepage::gpl-v3-license], except for those parts (lines of code from libraries used in this project) already licensed under other licenses,
- **documented** in its entirety: the code via [Doc comments][www::docs::rustdoc], the rest via Markdown and GitHub Pages.

Everything you need to know about how to work on _unCORE_ can be found under [Development][docs::development]. The [Kernel Architecture][docs::architecture] section contains all the information about the kernel's internal structure and composition. The [Testing][docs::testing] page contains information on unit- and integration tests.

!!! tip "The Second Half of the Documentation"

    This documentation is **only one half** of the whole documentation that is available. **The other part** is the code documentation that can be build with `#!bash cd code && cargo run -q -- doc [--help]`. The code itself is extensively documented with [doc comments](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html), so make sure to also check out the code documentation!

!!! example "Writing a Kernel Subsystem or Component From Scratch"

    _unCORE_ is (currently) suited to building kernel subsystems (scheduling, paging, etc.) or components (threads, drivers, etc.) from scratch. This is because _unCORE_ provides a straight forward documentation and _unCORE_ is very easy to get started with. The (current) code base is small and easy to grasp.

    If you always wanted to build anything inside a kernel, this is the project that gets you going! Moreover, merge requests are welcome :smile:

## :telescope: Vision

As of now, _unCORE_ is an **educational** project that does not run real software. Anyone interested in Rust, whether they are beginners or experts, can start working on this project. Its extensive documentation eases working with the code tremendously. By fully sticking to Rust (for everything, including the build), we simplify working with the code across all disciplines: building, running, debugging, testing. Rust also provides excellent abstractions and new programming concepts that older languages like C simply lack.

!!! quote "Abstraction | [Edsger Wybe Dijkstra](https://www.cs.utexas.edu/~EWD/transcriptions/EWD03xx/EWD340.html)"

    Abstraction is not about vagueness, it is about being precise on a new semantic level.

[//]: # (Links)

[www::wiki::operating-system-kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[www::homepage::rust]: https://www.rust-lang.org/
[www::homepage::gpl-v3-license]: https://opensource.org/license/gpl-3-0/
[www::docs::rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

[docs::development]: ./development.md
[docs::architecture]: ./kernel_architecture/overview.md
[docs::testing]: ./testing.md
