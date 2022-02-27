// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

mod dispatch;

fn main()
{
	use clap::Parser;

	let arguments = dispatch::arguments::Arguments::parse();
	log::trace!("Arguments:\n{:#?}\n", arguments);

	helper::logger::initialize(arguments.get_log_level());
	dispatch::build::set_target(&arguments.target);
	arguments.execute_command();
}
