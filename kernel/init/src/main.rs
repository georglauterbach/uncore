// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

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
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// Lint target for code documentation. When running `rustdoc`,
// show an error when using broken links.
#![deny(rustdoc::broken_intra_doc_links)]

//! # The `unCORE` Boot-Image Creation
//!
//! This small application builds the boot-image for `unCORE`, and if
//! demanded, runs it with QEMU too.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

/// ## Various Helper Functions
///
/// Logging functionality and more.
mod helper;

/// ## Compile, Boot and Run on `x86_64`
#[cfg(target_arch = "x86_64")]
mod x86_64;

/// ### Entrypoint
///
/// Parses arguments and runs the boot-image creation subroutine for
/// each individual architecture. Additionally, QEMU can be run.
fn main()
{
	println!("\nINFO    | Starting to build the unCORE kernel image. This may take some time.");

	let arguments = std::env::args();

	#[cfg(target_arch = "x86_64")]
	{
		println!("INFO    | Compiling for x86_64");
		x86_64::main(arguments.skip(1));
	}
}
