---
tags:
  - bootstrapping
  - boot
---

# Bootstrapping _unCORE_

!!! missing "Incomplete Documentation"
    Major parts of this documentation are still missing. You could be the first!

The boot process is neat and tidy, although it may not seem like it at first glance. This is because the heavy lifting is done (on `x86_64`) by the `rust-osdev/bootloader` crate. _unCORE_ uses UEFI and UEFI only - legacy shall not be supported.

## The Kernel Part

The kernel uses the `bootloader` crate (for `x86_64`) to define an entry point and a macro that calls this entry point (and typechecking it). Since the entrypoint is somewhat tied to the architecture and the corresponding bootloader, _unCORE_ has chosen to go the following way:

1. There is an architecture specific entrypoint in the `kernel/src/library/architectures/<ARCHITECTURE>/mod.rs` file called `#!rust fn kernel_main(...) -> !`. This function will take care of porting the boot information given to it by the bootloader into a universal format (by utilizing `boot::BootInformation`, a generic enumeration which uses conditional compilation and which can be passed around to various setup functions).
2. The function from 1. calls `#!rust fn kernel_main() -> !` in `kernel/src/lib.rs`. This function has the same name, and starts the kernel-wide setup.

This integrates nicely with tests as well, because we do not need much redundant code for the kernel-wide setup. Just another architecture specific `#!rust fn kernel_main(...) -> !` annotated with `#!rust #[cfg(test)]`. The `boot::BootInformation` enumeration is then used to initialize memory, etc.

This procedure makes use of _Rust_'s ecosystem, heavily benefitting from compiler features for conditional compilation.

## The `boot` Workspace Member

Next to the kernel under `kernel/`, there is a workspace member with the name `boot`. We need this because we need to link to bootloader crates to the kernel, but the finished binary will need to be linked to an actual bootloader in order to allow the kernel to boot properly. This is exactly what the `boot` workspace member does. When the kernel is built, a

``` CONSOLE
$ file kernel/target/x86_64-unknown-uncore/debug/kernel
ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, with debug_info, not stripped
```

file was created. When linked with the bootloader, more than one file is created under `kernel/out/qemu/boot_output/`. The one file we're interested in is `boot-uefi-kernel.efi`:

``` CONSOLE
$ file kernel/out/qemu/boot_output/boot-uefi-kernel.efi
PE32+ executable (EFI application) x86-64, for MS Windows
```

This file is copied to `kernel/out/qemu/kernel/EFI/BOOT/BOOTX64.EFI` so we're able to run QEMU on the `kernel/out/qemu/kernel/` directory. The other files created under `kernel/out/qemu/boot_output/` may be used in the future for proper disk image creation.

[//]: # (Links)
