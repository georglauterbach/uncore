# The Internal Kernel Structure

## Architecture

The boot process of _unCORE_ is explained in detail in the [boot documentation page][docs-boot].

## Modules

The kernel is composed of different, so-called "modules". These big building blocks are all found in the source code under `kernel/src/library/`. Here are all modules nicely listed:

``` BASH
kernel/src/library/
├── helper
├── memory
└── hardware
```

### `boot`

This module contains architecture independent boot code - mostly for [multiboot2] and [UEFI].

### `hardware`

The `hardware` module provides **all** the **architecture dependent** code. This includes boot code, CPU setup code or virtual memory support for this platform. Most modules

### `helper`

The `helper` module provides very generic function all other workspace members use, for example test runners, `#!rust panic` functionality and a function that does not return.

### `memory`

The `memory` module provides main memory management functionality. It is a kernel core component.

[//]: # (Links)

[docs-boot]: ./architecture/boot.md

[Hardware Abstraction Layer]: https://en.wikipedia.org/wiki/Hardware_abstraction
[Rust]: https://www.rust-lang.org/

[multiboot2]: https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
[GRUB2]: https://en.wikipedia.org/wiki/GNU_GRUB
[UEFI]: https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface
