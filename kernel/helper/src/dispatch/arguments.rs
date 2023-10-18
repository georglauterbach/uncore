// SPDX-License-Identifier: GPL-3.0-or-later

// see https://github.com/clap-rs/clap/blob/v3.1.2/examples/derive_ref/README.md
// for the clap derive reference

#[allow(clippy::missing_docs_in_private_items)]
#[derive(clap::Parser, Debug)]
#[clap(author, version, about, name = "unCORE Helper Binary")]
pub struct Arguments
{
	/// path to target file; currently valid options are x86_64, aarch64 and i686
	#[clap(short, long, default_value = "x86_64")]
	pub target:    String,
	#[clap(flatten)]
	pub verbosity: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
	#[clap(subcommand)]
	command:       AvailableSubCommands,
}

impl Arguments
{
	/// ### Query the Log Level from the Arguments
	///
	/// Returns an instance of [`log::Level`] parsed from the derived from the given
	/// arguments.
	pub fn get_log_level(&self) -> log::Level { self.verbosity.log_level().unwrap_or(log::Level::Error) }

	/// ### Execute the Correct Command Given the Arguments
	///
	/// Checks [`self`] and executes the appropriate function.
	pub fn execute_command(self)
	{
		match self.command {
			AvailableSubCommands::Check { is_ci } => super::test::check(is_ci),
			AvailableSubCommands::Test { test, is_ci } => super::test::test(test, is_ci),
		}
	}
}

#[allow(clippy::missing_docs_in_private_items)]
#[derive(clap::Subcommand, Debug)]
enum AvailableSubCommands
{
	/// Build the kernel
	#[clap(
		author,
		version,
		long_about = "This command enabled one to easily build unCORE and supply needed variables \
		              with the CLI."
	)]
	Build,
	/// Check all code with clippy and rustfmt
	#[clap(
		version,
		long_about = "Checks against clippy and rustfmt, and uses rustdoc to check the \
		              documentation as well. If the --is-ci option is provided, the run will return \
		              with a non-zero exit code if there were errors."
	)]
	Check
	{
		/// If you need the binary to return with a non-zero exit code on when
		/// error occurred
		#[clap(long)]
		is_ci: bool,
	},
	/// Run the kernel with QEMU
	#[clap(
		author,
		version,
		long_about = "Easily run the kernel in QEMU. This command will not rebuild the kernel \
		              before it is run automatically, though, if you're using Just, this is already \
		              taken care of."
	)]
	Run
	{
		/// If you want a separate QEMU window to open. Otherwise, all is logged
		/// to the console (default).
		#[clap(long)]
		graphical: bool,
	},
	/// Run unit and integration tests
	#[clap(
		version,
		long_about = "Unit and integration testing framework to easily run all tests or specific \
		              tests with --test. When specifying '--test lib', only unit tests are run."
	)]
	Test
	{
		/// Specify a specific test to run (under kernel/tests) ('--test lib' for
		/// unit tests)
		#[clap(long)]
		test:  Option<String>,
		// If you need the binary to return with a non-zero exit code on when
		/// error occurred
		#[clap(long)]
		is_ci: bool,
	},
}
