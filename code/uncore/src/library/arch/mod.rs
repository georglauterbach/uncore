// SPDX-License-Identifier: GPL-3.0-or-later

//! This module contains all architecture-specific functionality.
//!
//! The trick to having a uniform interface is to use `pub use architecture::{stuff, to,
//! re-export}` where `architecture` is a local re-export of a specific architecture
//! (.e.g, `use risc_v as architecture`) guarded by conditional compilation (e.g.,
//! `#[cfg(target_arch = "riscv64`) so that only one architecture is enabled at any given
//! time.

/// Re-exported intra-kernel API that any implementation of an architecture must satisfy.
///
/// #### Attention
///
/// Initialization functionality may not have a working logger available when the
/// functions are called at run-time.
pub use architecture::{
  drivers,
  heap,
  exit_kernel,
  initialize,
};

#[cfg(target_arch = "riscv64")]
mod risc_v;
#[cfg(target_arch = "riscv64")] use risc_v as architecture;
