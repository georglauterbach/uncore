---
hide:
  - navigation
tags:
  - home
  - introduction
  - overview
---

# The unCORE Operating System Kernel

## Introduction

Welcome to the official _unCORE_ operating system kernel documentation. _unCORE_ is

- an educational, modern [**operating system kernel**][www::wiki::operating-system-kernel]
- completely written in pure, idiomatic [**Rust**][www::homepage::rust] (and assembly where required),
- licensed under the [**GNU Public License v3 or later**][www::homepage::gpl-v3-license], except for those parts (lines of code from libraries used in this project) already licensed under other licenses,
- **documented** in its entirety: the code via [Doc comments][www::docs::rustdoc], the rest via Markdown and GitHub Pages.

Everything you need to know about how to work on _unCORE_ can be found under [Development][docs::development]. The [Kernel Architecture][docs::architecture] section contains all the information about the kernel's internal structure and composition. The [Testing][docs::testing] page contains information on unit- and integration tests.

!!! note "The Second Half of the Documentation"

    This documentation is only one half of the whole documentation that is available. The other part is the code documentation that can be build with `#!bash cd code && cargo doc --package uncore --open`.

## Vision

As of now, _unCORE_ is an **educational** project that does not run real software. Anyone interested in Rust, whether they are beginners or experts, can start working on this project. Its extensive documentation eases working with the code tremendously. By fully sticking to Rust (for everything, including the build), we simplify working with the code across all disciplines: building, running, debugging, testing. Rust also provides excellent abstractions and new programming concepts that older languages like C simply lack.

!!! quote "Abstraction | Edsger Wybe Dijkstra"

    Abstraction is not about vagueness, it is about being precise on a new semantic level.

## Disclaimers

This project was and is heavily inspired by the following projects:

1. _Phillip Oppermann_ | [_BlogOS_](https://os.phil-opp.com/)
2. _Matthias Totschnig_ | [_Hello, RISC-V and QEMU_](https://mth.st/blog/riscv-qemu/)
3. _Stephen Marz_ | [RISC-V OS using Rust](https://osblog.stephenmarz.com/ch1.html)
4. _Henry Gressmann_ | [Operating Systems in Rust](https://blog.henrygressmann.de/rust-os/1-hello-riscv/#hello-world)

[//]: # (Links)

[www::wiki::operating-system-kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[www::homepage::rust]: https://www.rust-lang.org/
[www::homepage::gpl-v3-license]: https://opensource.org/license/gpl-3-0/
[www::docs::rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

[docs::development]: ./development.md
[docs::architecture]: ./architecture/overview.md
[docs::testing]: ./testing.md
