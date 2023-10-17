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

//! ## TODO

use anyhow::Context;

macro_rules! log {
    ($message: expr) => {
      println!("cargo:warning={:?}", $message);
    };
}

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    log!(chrono::offset::Local::now().format("%+").to_string());

    check_dependencies().context("A required package or library seems to be missing")?;

    Ok(())
}

fn check_dependencies() -> anyhow::Result<()> {
    macro_rules! check_bin {
        ($command: tt) => {
            which::which($command).context(format!("Package {} seems to be missing", $command))?;
        };

        ($command: expr, $package: expr) => {
            which::which($command).context(format!("Package {} seems to be missing", $package))?;
        };
    }

    check_bin!("riscv64-linux-gnu-gcc", "gcc-riscv64-linux-gnu");
    check_bin!("qemu-system-riscv64");

    Ok(())
}
