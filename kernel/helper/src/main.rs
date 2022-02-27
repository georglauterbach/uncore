// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

#![feature(is_some_with)]

mod dispatch;

fn main()
{
	use clap::Parser;

	// parse arguments
	let arguments = dispatch::arguments::Arguments::parse();
	log::trace!("Arguments:\n{:#?}\n", arguments);

	// set up log
	let log_level = arguments.get_log_level();
	helper::environment::set_log_level(helper::logger::Logger::level_to_string(&log_level));
	helper::logger::initialize(Some(log_level));

	// run the specified command
	helper::build::set_target(&arguments.target);
	arguments.execute_command();
}
