// SPDX-License-Identifier: GPL-3.0-or-later

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
// As this is no ordinary program, we have a special entry-point,
// which is not the `main()` function.
#![no_main]
// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf) .
#![deny(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![deny(clippy::nursery)]
// Clippy lint target three. Enables new lints that are still
// under development
#![deny(clippy::pedantic)]
// Clippy lint target four. Enable lints for the cargo manifest
// file, a.k.a. Cargo.toml.
#![deny(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_docs_in_private_items)]

//! TODO

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use uncore::*;

/// The RISC-V 64bit entrypoint, called by the [`riscv-rt`] runtime after SBI has set up
/// the machine.
#[cfg(target_arch = "riscv64")]
#[riscv_rt::entry]
fn riscv64_entry() -> ! {
  arch::initialize();
  setup_kernel();

  ::log::warn!("This is an integration test!");
  ::log::info!("This integration test is called 'basic_boot'");

  arch::exit_kernel(Condition::Success);
}
