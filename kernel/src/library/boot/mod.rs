// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// Include order possibly matters for the linker. Make sure to
// pick the correct sequence.

// * x86_64
// * ---------------------------------------------------------------------

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64/start.S"), options(att_syntax));

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64/multiboot2.S"), options(att_syntax));
