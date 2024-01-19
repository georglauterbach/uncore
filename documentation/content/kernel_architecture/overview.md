---
tags:
  - kernel architecture
  - kernel architecture
  - kernel structure
  - kernel structure
  - overview
---

# The Kernel Architecture

## :construction_worker: Architecture-Dependent Information

Architecture-specific code resides in [`code/uncore/src/library/arch/`][code::github::code/uncore/src/library/arch/]. The documentation for architecture-specific functionality is provided in separate articles:

- [RISC-V](./risc_v.md)
- Other Architectures: _unCORE_ does currently **not** support architectures other than RISC-V.

## :triangular_ruler: Architecture-Independent Information

There are aspects of the kernel code that are independent of the architecture: the [directory and file layout of the kernel source code](../development.md#about-the-workspace), mechanisms above[^1] the [HAL][www::wikipedia::hardware-abstraction], etc. The following subsections describe and explain the functionality of _unCORE_ above the HAL.

[^1]: "Above" refers to using the ([HAL][www::wikipedia::hardware-abstraction]) instead of providing functionality for it to work (which one would refer to as "below"). Drivers, for example, reside "below" the HAL.

### Memory

!!! warning "This section (and the corresponding implementation) is TODO."

The kernel does currently **not** support PVM. The heap implementation that is currently in place uses pre-defined memory (already known at link-time).

### Threads & Scheduling

!!! warning "This section (and the corresponding implementation) is TODO."

### Hardware Abstraction

!!! warning "This section (and the corresponding implementation) is TODO."

[//]: # (Links)

[code::github::code/uncore/src/library/arch/]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/src/library/arch/
[www::wikipedia::hardware-abstraction]: https://en.wikipedia.org/wiki/Hardware_abstraction
