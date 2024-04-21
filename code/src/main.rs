// SPDX-License-Identifier: GPL-3.0-or-later

// Preventing `unsafe` code in `main.rs` completely.
#![forbid(unsafe_code)]
// In the helper, it is fine to have multiple different
// versions in transitive dependencies.
#![allow(clippy::multiple_crate_versions)]

//! ## `unCORE` Helper
//!
//! Helper program to ease building and running `unCORE`. This is the main binary in the
//! workspace, which enables a seamless integration of `cargo run --` into the workflow of
//! `unCORE`.

mod arguments;
mod command;
mod environment;
mod log;

/// A simple main function.
fn main() {
  let arguments = <arguments::Arguments as clap::Parser>::parse();

  log::initialize(arguments.get_log_level());
  if arguments.dispatch_command().is_err() {
    std::process::exit(1);
  }
}
