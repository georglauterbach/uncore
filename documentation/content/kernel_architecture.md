---
tags:
  - architecture
  - structure
---

# The Kernel Architecture

## General

There are aspects of the kernel code that are independent of the architecture: the file layout of the kernel source code, mechanisms above the [hardware abstraction layer (HAL)][www::wikipedia::hardware-abstraction] [^1], etc. Architecture-specific code resides in [`code/uncore/src/library/arch/`][code::github::code/uncore/src/library/arch/].

[^1]: "Above" refers to using the [hardware abstraction layer (HAL)][www::wikipedia::hardware-abstraction] instead of providing functionality for it to work (which one would refer to as "below"). Drivers, for example, reside "below" the HAL.

## RISC-V

### Bootstrapping and Runtime

The boot-flow is tied to the privilege modes on each architecture. For the purpose of simplicity, this documentation covers RISC-V exemplarily.

RISC-V has three privilege levels:

1. **Machine Mode**: This is the level with the most privilege; firmware like [OpenSBI][www::github::open-sbi] runs here.
2. **Supervisor Mode**: This privilege level is where our kernel runs.
3. **User Mode**: This is the level with the least privilege; typically, user processes run here.

!!! quote "Boot-Flow on RISC-V | [Henry Gressmann](https://blog.henrygressmann.de/rust-os/1-hello-riscv/#hello-world)"

    Compared to other CPU Architectures, RISC-V's boot process is straightforward. We're using [OpenSBI][www::github::open-sbi] as our [Supervisor Execution Environment](https://five-embeddev.com/riscv-isa-manual/latest/supervisor.html) (SEE), our Machine Mode (M-Mode) run-time firmware. [Supervisor Binary Interface](https://www.scs.stanford.edu/~zyedidia/docs/riscv/riscv-sbi.pdf) (SBI) is a standard interface for interacting with the SEE, and OpenSBI is an implementation of this standard.

When running inside QEMU, a [Jump Address][www::documentation::qemu-fw-jump] (`0x8020_0000`) is used. QEMU will load our kernel into memory at this address, then jump to address `0x800_0000` where OpenSBI is located, which will then jump to `0x8020_0000`, where our kernel is located.

??? abstract "Traditional vs QEMU RISC-V Boot Flow"

    The imagery was developed by [Henry Gressmann](https://blog.henrygressmann.de/rust-os/1-hello-riscv/).

    ---

    While a traditional boot flow looks like this:

    ```txt
        System ROM           System ROM         Disk / Network       Disk / Network
    ┌────────────────┐   ┌────────────────┐   ┌────────────────┐   ┌────────────────┐
    │ Device         │   | First Stage    │   │ Second Stage   │   │                │
    │ Specific       ├──>| Bootloader     ├──>│ Bootloader     ├──>│ Kernel         │
    │ Firmware       │   | (e.g., UEFI)   │   │ (e.g, Grub 2)  │   │                │
    └────────────────┘   └────────────────┘   └────────────────┘   └────────────────┘

                          Loads the Second     Loads the kernel
                          Stage Bootloader,    into memory, e.g.
                          e.g., from address   from disk.
                          specified in GPT.
    ```

    The QEMU RISC-V is simplified and looks like this:

    ```txt
        System ROM              RAM                  RAM
                            0x8000_0000          0x8020_0000
    ┌────────────────┐   ┌────────────────┐   ┌────────────────┐
    │ Device         │   |                │   │                │
    │ Specific       ├──>| OpenSBI        ├──>│ Kernel         │
    │ Firmware       │   |                │   │                │
    └────────────────┘   └────────────────┘   └────────────────┘

      M-Mode              M-Mode               S-Mode

      Loads OpenSBI       Loads the kernel
      into RAM.           and device tree
                          into RAM
    ```

    Such an architecture is not only simpler, but it also enables writing a single kernel for all RISC-V CPUs that implment SBI. SBI puts a layer of abstraction between the hardward and our kernel. SBI also provides functionality like printing and a Flattened Device Tree (FDT).

    Interacting with SBI is handled by the [`sbi`](https://crates.io/crates/sbi) crate. This crate utilizes the `ecall` instruction to trap into the SEE (which is OpenSBI on QEMU), where a handler will handle the trap and then return to the kernel. This is handled much in the same way that a system call is handled: first, you set up registers, then you execute `ecall`, and then you read out registers that contain return values.

_unCORE_ currently uses [`riscv-rt`][www::documentation::crate::riscv-rt]. This crate provides a run-time for RISC-V and additionally handlers for interrupts and exceptions. The linker script currently in use for RISC-V 64bit is derived from the [linker script that `riscv-rt` ships](https://github.com/rust-embedded/riscv-rt/blob/738baf93dfcc2570931d0e52d1b6ee1ccc8a6067/link-rv64.x). QEMU takes an ELF file with the `-kernel` parameter. The ELF is built according to [our linker script][code::github::code/uncore/src/library/arch/risc_v/linking.ld].

When [OpenSBI][www::github::open-sbi] and [`riscv-rt`][www::documentation::crate::riscv-rt] have finished running, _unCORE is entered_. The entry functions lives (as the only function) in [`code/uncore/src/main.rs`][code::github::code/uncore/src/main.rs]:

```rust title="unCORE Entry Function Signature" hl_lines="5"
--8<-- "https://raw.githubusercontent.com/georglauterbach/uncore/master/code/uncore/src/main.rs:16:20"
```

The entry function is called with one argument, the HART (CPU core; in RISC-V slang "hardware thread", i.e., HART) on which the setup has been called. This will prove useful because some system initialization steps need to happen only once, and some have to happen for each HART.

[//]: # (Links)

[www::wikipedia::hardware-abstraction]: https://en.wikipedia.org/wiki/Hardware_abstraction
[www::github::open-sbi]: https://github.com/riscv-software-src/opensbi
[www::documentation::qemu-fw-jump]: https://github.com/riscv-software-src/opensbi/blob/master/docs/firmware/fw_jump.md
[www::documentation::crate::riscv-rt]: https://docs.rs/riscv-rt/latest/riscv_rt/
[code::github::code/uncore/src/library/arch/]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/src/library
[code::github::code/uncore/src/library/arch/risc_v/linking.ld]: https://github.com/georglauterbach/uncore/blob/master/code/uncore/src/library/arch/risc_v/linking.ld
[code::github::code/uncore/src/main.rs]: https://github.com/georglauterbach/uncore/blob/master/code/uncore/src/main.rs
