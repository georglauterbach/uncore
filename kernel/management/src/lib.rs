// SPDX-License-Identifier: GPL-3.0-or-later

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// // Clippy lint target one. Enables all lints that are on by
// // default (correctness, suspicious, style, complexity, perf) .
// #![deny(clippy::all)]
// // Clippy lint target two. Enables lints which are rather strict
// // or have occasional false positives.
// #![deny(clippy::nursery)]
// // Clippy lint target three. Enables new lints that are still
// // under development
// #![deny(clippy::pedantic)]
// // Clippy lint target four. Enable lints for the cargo manifest
// // file, a.k.a. Cargo.toml.
// #![deny(clippy::cargo)]
// #![allow(clippy::multiple_crate_versions)]
// // Lint target for code documentation. This lint enforces code
// // documentation on every code item.
// #![deny(missing_docs)]
// #![deny(clippy::missing_docs_in_private_items)]
// // Lint target for code documentation. When running `rustdoc`,
// // show an error when using broken links.
// #![deny(rustdoc::broken_intra_doc_links)]

//! # The `unCORE` Kernel -- Management Crate
//!
//! This crate allows building and running the kernel while staying Rust-native.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

pub mod rush;
