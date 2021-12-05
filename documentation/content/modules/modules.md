# Modules

All kernel modules reside in the directory `modules/`.

``` BASH
modules/
├── syscalls        # (1)
├── kernel          #    (2)
├── communication   # (3)
├── memory          #    (4)
├── processes       # (5)
├── scheduling      #    (6)
└── hardware        # (7)
```

1. [The `syscalls` module][docs-syscalls-module] provides a unified interface for user space to interoperate with the kernel, see [the kernel architecture][docs-architecture].
2. [The `kernel` module][docs-kernel-module] provides the actual, complete kernel. It can be compiled as a binary to run as an operating system kernel.
3. [The `communication` module][docs-communication-module] provides inter-process communication (IPC). It is a kernel core component.
4. [The `memory` module][docs-memory-module] provides main memory management functionality. It is a kernel core component.
5. [The `processes` module][docs-processes-module] provides the abstractions for processes and threads. It is a kernel core component.
6. [The `scheduling` module][docs-scheduling-module] provides scheduling functionality. It is a kernel core component.
7. [The `hardware` module][docs-hardware-module] provides the mechanisms to isolate the actual kernel code from the underlying hardware. It provides a unified interface for the kernel to work with, see [the kernel architecture][docs-architecture].

This directory contains all kernel code (and therefore the complete [Rust] code of _unCORE_). The `modules/` directory is a Cargo workspace that contains all of the above mentioned modules as workspace members. The final kernel binary is built from the source in `modules/kernel/`.

[//]: # (Links)

[docs-syscalls-module]: ./syscalls.md
[docs-kernel-module]: ./kernel.md
[docs-communication-module]: ./communication.md
[docs-memory-module]: ./memory.md
[docs-processes-module]: ./processes.md
[docs-scheduling-module]: ./scheduling.md
[docs-hardware-module]: ./hardware.md
[docs-architecture]: ../index.md#architecture

[Hardware Abstraction Layer]: https://en.wikipedia.org/wiki/Hardware_abstraction
[Rust]: https://www.rust-lang.org/
