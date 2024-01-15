// SPDX-License-Identifier: GPL-3.0-or-later

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
// As this is no ordinary program, we have a special entry-point,
// which is not the `main()` function.
#![no_main]

//! This integration test tests whether unCORE can boot and bootstrap itself.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use uncore::*;

/// The RISC-V 64bit entrypoint, called by the [`riscv-rt`] runtime after SBI has set up
/// the machine.
#[cfg(target_arch = "riscv64")]
#[riscv_rt::entry]
fn riscv64_entry(hart: usize) -> ! {
  arch::initialize(hart);
  setup_kernel(hart);

  ::log::warn!("This is an integration test!");
  ::log::info!("This integration test is called 'basic_boot'");

  arch::exit_kernel(UncoreResult::Ok);
}
