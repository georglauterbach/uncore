// SPDX-License-Identifier: GPL-3.0-or-later

// Preventing `unsafe` code in `main.rs` completely.
#![forbid(unsafe_code)]
// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf).
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
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
// All other, generic lint targets that were not
// covered previously
#![deny(missing_debug_implementations)]

//! ## `unCORE` Helper
//!
//! Helper program to ease building and running `unCORE`. This is the main binary in the
//! workspace, which enabled a seamless integration of `cargo run --` into the workflow of
//! `unCORE`.

mod arguments;
mod logger;

/// A simple main function.
fn main() -> anyhow::Result<()> {
  let arguments = <arguments::Arguments as clap::Parser>::parse();

  logger::initialize(arguments.get_log_level());
  // log::info!("{}", chrono::offset::Local::now().format("%+").to_string());
  arguments.dispatch_command()?;

  Ok(())
}
