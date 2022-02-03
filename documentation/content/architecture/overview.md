# The Internal Kernel Structure

## Architecture

The bootstrapping process of _unCORE_ is explained in detail in the [bootstrapping documentation page][docs-bootstrapping]. As we compile against UEFI targets, the entrypoint for our kernel is `#!rust fn efi_main()`, which calls `#!rust fn main()` in the end. There is no magic involved with this kernel - except for code clarity and code quality, of course - _unCORE_ also puts on its trousers one leg at a time. You should be able to navigate the code base quite easily - on the one hand because it is not too large, on the other hand because the code documentation for items allows you to jump through the code of you have a proper IDE setup.

## Modules

The kernel is composed of different, so-called "modules". These big building blocks are all found in the source code under `kernel/src/library/`. Here are all modules nicely listed:

``` BASH
kernel/src/library/
├── helper
├── memory
└── hardware
```

### `architectures`

The `architectures` module provides **all** the **architecture dependent** code. This includes boot code, CPU setup code or virtual memory support for this platform. Most modules

### `boot`

This module contains architecture independent boot code - mostly for [multiboot2] and [UEFI].

### `helper`

The `helper` module provides very generic function all other workspace members use, for example test runners, `#!rust panic` functionality and a function that does not return.

### `memory`

The `memory` module provides main memory management functionality. It is a kernel core component.

[//]: # (Links)

[docs-bootstrapping]: ./architecture/bootstrapping.md

[Hardware Abstraction Layer]: https://en.wikipedia.org/wiki/Hardware_abstraction
[Rust]: https://www.rust-lang.org/

[multiboot2]: https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
[GRUB2]: https://en.wikipedia.org/wiki/GNU_GRUB
[UEFI]: https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface