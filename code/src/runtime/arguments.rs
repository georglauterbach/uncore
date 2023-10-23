// SPDX-License-Identifier: GPL-3.0-or-later

//! This module provides all types required for parsing and working with command line
//! arguments.

/// Defines which architectures can be targeted by `unCORE`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum Architecture {
  /// RISC-V 64 GC
  // (see <https://www.cnx-software.com/2019/08/27/risc-v-bases-and-extensions-explained/>)
  Riscv64,
}

/// Helper program to ease building and running `unCORE`.
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about, propagate_version = true)]
#[command(bin_name = "cargo run -q --")]
pub struct Arguments {
  /// Specify the verbosity
  #[clap(flatten)]
  verbosity:    clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
  /// Specify the architecture unCORE is built for.
  #[clap(value_enum, default_value_t=Architecture::Riscv64)]
  architecture: Architecture,
  /// Specify what to do: build the kernel, run the kernel, etc.
  #[command(subcommand)]
  command:      super::command::Command,
}

impl Arguments {
  /// Get the log level specified by the arguments.
  pub fn get_log_level(&self) -> Option<log::Level> { self.verbosity.log_level() }

  /// Dispatches individual sub-commands to the correct sub-routines that execute the
  /// sub-commands.
  pub fn dispatch_command(self) -> Result<(), ()> {
    log::debug!("Dispatching command '{}'", self.command);
    match self.command.execute(self.architecture) {
      Ok(()) => Ok(()),
      Err(error) => {
        log::error!(
          "{}",
          error
            .chain()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" - ")
        );
        Err(())
      },
    }
  }
}
