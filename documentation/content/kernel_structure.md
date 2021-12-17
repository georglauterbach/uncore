# The Internal Kernel Structure

The kernel is composed of different, so-called "modules". These big building blocks are all found in the source code under `kernel/src/library/`. Here are all modules nicely listed:

``` BASH
kernel/src/library/
├── syscalls
├── helpers
├── communication
├── memory
├── processes
├── scheduling
└── hardware
```

Some of these modules are so-called kernel core components. This is just a fancy way of saying that they are very important to the kernel's main functionality, and without them, the kernel would not work. Core components include (non-exhaustivly) `helper`, `processes`, `scheduling`, ... There are dedicated pages for every module: Each module is listed to the right of this text in a navigation menu. The interplay of all modules is somewhat obvious, and all modules have a rather concise name so everyone knows what their purpose is.

[//]: # (Links)

[docs-syscalls-module]: ./modules/syscalls.md
[docs-kernel-module]: ./modules/kernel.md
[docs-communication-module]: ./modules/communication.md
[docs-memory-module]: ./modules/memory.md
[docs-processes-module]: ./modules/processes.md
[docs-scheduling-module]: ./modules/scheduling.md
[docs-hardware-module]: ./modules/hardware.md
[docs-architecture]: ./index.md#architecture

[Hardware Abstraction Layer]: https://en.wikipedia.org/wiki/Hardware_abstraction
[Rust]: https://www.rust-lang.org/
