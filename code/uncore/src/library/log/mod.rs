// SPDX-License-Identifier: GPL-3.0-or-later

//! This module implements kernel-wide logging. It uses the [`::log`] crate.

mod print;
mod env;

pub use print::{
  initialize,
  display_initial_information,
  KernelLogger,
};
