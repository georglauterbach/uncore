// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use clap::Parser;

/// ### Arguments
///
/// Holds arguments for this binary.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments
{
	/// Indicates whether a test binary is provided.
	#[clap(short, long)]
	pub test: Option<String>,
}

impl Arguments
{
	/// ### Let `clap` Create a New Instance
	///
	/// This is a simple wrapper for `clap`'s `::parse()` method.
	pub fn new() -> Self { Self::parse() }
}
