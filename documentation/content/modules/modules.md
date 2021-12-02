# Modules

## Structure

All kernel modules reside in the directory `modules/`:

``` TXT
modules/
├── syscalls       # system call interface
├── kernel         # glueing everything together
├── communication  # inter-process communication
├── memory         # memory management
├── processes      # process abstractions
├── scheduling     # scheduling implementations
└── hardware       # hardware abstraction layer
```

## The Individual Modules

### `syscalls` - System Call Interface

[The `syscalls` module][docs-syscalls-module] provides a unified interface for user space to interoperate with the kernel, see [the kernel architecture][docs-architecture].

### `kernel` - The Final Binary

[The `kernel` module][docs-kernel-module] provides the actual, complete kernel. It can be compiled as a binary to run as an operating system kernel.

### `communication`

[The `communication` module][docs-communication-module] provides inter-process communication (IPC). It is a kernel core component.

### `memory`

[The `memory` module][docs-memory-module] provides main memory management functionality. It is a kernel core component.

### `processes`

[The `memory` module][docs-processes-module] provides the abstractions for processes and threads. It is a kernel core component.

### `scheduling`

[The `scheduling` module][docs-scheduling-module] provides scheduling functionality. It is a kernel core component.

### `hardware` - Hardware Abstraction Layer

[The `hardware` module][docs-hardware-module] provides the mechanisms to isolate the actual kernel code from the underlying hardware. It provides a unified interface for the kernel to work with, see [the kernel architecture][docs-architecture].


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
