// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// ? MODULES
// ? ---------------------------------------------------------------------

/// ## `x86_64` CPU Setup
///
/// Provides general CPU setup and exception as well as interrupt
/// handlers.
pub mod cpu;

/// ## Virtual Memory Implementation
///
/// This module contains the virtual memory / paging abstractions for
/// `x86_64`.
pub mod memory;

// ? ASSEMBLY BOOT CODE
// ? ---------------------------------------------------------------------

use core::arch::global_asm;

global_asm!(include_str!("boot/start.S"), options(att_syntax));

global_asm!(include_str!("boot/multiboot2.S"), options(att_syntax));
