// SPDX-License-Identifier: GPL-3.0-or-later

/// TODO
// #[cfg(target_arch = "riscv64")]
mod risc_v;
// #[cfg(target_arch = "riscv64")]
use risc_v as architecture;

pub use architecture::{
  drivers,
  exit_kernel,
  initialize,
};
