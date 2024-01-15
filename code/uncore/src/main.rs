// SPDX-License-Identifier: GPL-3.0-or-later

// This crate does not and cannot use the standard library.
#![no_std]
// As this is no ordinary program, we have a special entry-point,
// which is not the `main()` function.
#![no_main]

//! # The `unCORE` Operating System Kernel
//!
//! This is `unCORE`, an operating system kerne completely written in pure, idiomatic
//! Rust.

use uncore::*;

/// The RISC-V 64bit entrypoint, called by the [`riscv-rt`] runtime after SBI has set up
/// the machine.
#[cfg(target_arch = "riscv64")]
#[riscv_rt::entry]
fn riscv64_entry(hart: usize) -> ! {
  arch::initialize(hart);
  setup_kernel(hart);
  arch::exit_kernel(UncoreResult::Ok);
}
