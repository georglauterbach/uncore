// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
pub struct Arguments
{
	/// path to target file; currently valid options are x86_64, aarch64 and i686
	#[clap(short, long, default_value = "x86_64")]
	pub target:    String,
	#[clap(flatten)]
	pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
	#[clap(subcommand)]
	command:       SubCommands,
}

impl Arguments
{
	pub fn get_log_level(&self) -> log::Level { self.verbosity.log_level().unwrap_or(log::Level::Error) }

	pub fn execute_command(self)
	{
		match self.command {
			SubCommands::Build => super::build::build(),
			SubCommands::Check { is_ci } => super::test::check(is_ci),
			SubCommands::Run { graphical } => super::run::run(graphical),
			SubCommands::Test { test, is_ci } => super::test::test(test, is_ci),
		}
	}
}

#[derive(clap::Subcommand, Debug)]
enum SubCommands
{
	Build,
	Check
	{
		#[clap(long)]
		is_ci: bool,
	},
	Run
	{
		#[clap(long)]
		graphical: bool,
	},
	Test
	{
		#[clap(long)]
		test:  Option<String>,
		#[clap(long)]
		is_ci: bool,
	},
}
