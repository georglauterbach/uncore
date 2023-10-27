// SPDX-License-Identifier: GPL-3.0-or-later

//! Hols all functionality required for building, running, etc. `unCORE`.

use core::arch;

/// Specifies which sub-command are available, i.e. whether the user wants to build the
/// kernel, run the kernel, etc.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, clap::Subcommand)]
pub enum Command {
  /// Build the kernel
  Build,
  /// Run the kernel
  Run,
  /// Test the kernel
  Test,
  /// Debug the kernel (by allowing GDB to attach)
  Debug,
  /// Check the code (e.g. with `clippy`)
  Check,
}

impl Command {
  /// Actually dispatches the given subcommand by matching on `Self`.
  pub fn execute(self, architecture: super::arguments::Architecture) -> anyhow::Result<()> {
    check_dependencies(architecture, self == Self::Debug)?;

    match self {
      Self::Build => build(&architecture.into())?,
      Self::Run => {
        build(&architecture.into())?;
        run(&architecture.into())?;
      },
      Self::Test => {
        anyhow::bail!("The test sub-command is not yet implemented");
        // build(&architecture.into())?;
      },
      Self::Debug => {
        build(&architecture.into())?;
        debug(&mut architecture.into())?;
      },
      Self::Check => {
        check(&architecture.into())?;
      },
    }
    Ok(())
  }
}

impl std::fmt::Display for Command {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", format!("{self:?}").to_lowercase())
  }
}

/// Holds information about all architecture-specific files and commands.
pub struct ArchitectureSpecification {
  /// The target triple
  pub target:             &'static str,
  /// The QEMU command to execute
  pub qemu_command:       &'static str,
  /// Path to the linker script
  pub linker_script_path: String,
  /// The parameters of the QEMU command to execute
  pub qemu_arguments:     Vec<String>,
}

impl From<crate::runtime::arguments::Architecture> for ArchitectureSpecification {
  fn from(val: crate::runtime::arguments::Architecture) -> Self {
    let base_dir = std::env::var("CARGO_MANIFEST_DIR").expect("LOL");
    match val {
      crate::runtime::arguments::Architecture::Riscv64 => Self {
        target:             "riscv64gc-unknown-none-elf",
        qemu_command:       "qemu-system-riscv64",
        linker_script_path: base_dir.clone() + "/uncore/src/library/arch/risc_v/boot/memory.x",
        qemu_arguments:     format!(
          "-m 2G -machine virt -nographic -serial mon:stdio -kernel {}",
           base_dir + "/target/riscv64gc-unknown-none-elf/debug/uncore"
        )
        .split(' ')
        .map(|x| x.to_string())
        .collect(),
      },
    }
  }
}

/// Check all dependencies (libraries and binaries) given a specific architecture.
fn check_dependencies(architecture: super::arguments::Architecture, is_debug: bool) -> anyhow::Result<()> {
  use anyhow::Context;

  log::debug!("Checking dependencies");

  /// Short-hand for calling [`which`].
  macro_rules! check_bin {
    ($command:tt) => {
      which::which($command).context(format!("Package '{}' seems to be missing", $command))?;
    };

    ($command:expr, $package:expr) => {
      which::which($command).context(format!("Package '{}' seems to be missing", $package))?;
    };
  }

  match architecture {
    super::arguments::Architecture::Riscv64 => {
      check_bin!("qemu-system-riscv64");
      if is_debug {
        log::trace!("  -> debug session detected - also checking debug dependencies");
        check_bin!("gdb-multiarch");
      } else {
        log::trace!("Not a debug session - not checking debug dependencies");
      }
    },
  }

  Ok(())
}

/// A wrapper that taken a [`std::process::ExitStatus`] and evaluates it properly. This
/// function is used in the macro [`run_command_and_check`].
fn evaluate_exit_status(exit_status: std::process::ExitStatus) -> anyhow::Result<()> {
  if let Some(status) = exit_status.code() {
    match status {
      0 => Ok(()),
      _ => anyhow::bail!("Failure: command exited with status code {}", status),
    }
  } else {
    anyhow::bail!("Failure: could not determine exit status - terminated by signal?")
  }
}

/// Runs a given command, taking arguments and environment variables if necessary, and
/// evaluates the exit status in the end using [`evaluate_exit_status`].
macro_rules! run_command_and_check {
  ($command_name:expr, $arguments:expr) => {{
    let exit_status = std::process::Command::new($command_name)
      .args($arguments)
      .status()?;

    evaluate_exit_status(exit_status)?;
  }};

  ($command_name:expr, $arguments:expr, $envs:expr) => {{
    let exit_status = std::process::Command::new($command_name)
      .args($arguments)
      .envs($envs)
      .status()?;

    evaluate_exit_status(exit_status)?;
  }};
}

/// Builds the kernel.
fn build(arch_specification: &super::command::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Building unCORE");

  let mut environment = super::environment::get_all_environment_variables_for_build()?;
  environment.insert(
    "RUSTFLAGS",
    format!("-Clink-arg=-T{} -Clink-arg=-Tlink.x", arch_specification.linker_script_path),
  );

  run_command_and_check!(
    env!("CARGO"),
    [
      "build",
      "--package",
      "uncore",
      "--target",
      arch_specification.target,
    ],
    environment
  );

  Ok(())
}

/// Runs the kernel given an [`ArchitectureSpecification`].
fn run(arch_specification: &super::command::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Running unCORE");
  run_command_and_check!(
    arch_specification.qemu_command,
    &arch_specification.qemu_arguments
  );
  Ok(())
}

/// Runs the kernel given an [`ArchitectureSpecification`] with debug attributes.
fn debug(arch_specification: &mut super::command::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Debugging unCORE");
  log::debug!("You may use 'gdb-multiarch -q -x code/uncore/gdb/init.gdb' to attach now");
  log::debug!("Remeber: 'Ctrl-A x' will exit QEMU");

  arch_specification.qemu_arguments.push("-s".to_string());
  arch_specification.qemu_arguments.push("-S".to_string());

  run_command_and_check!(
    arch_specification.qemu_command,
    &arch_specification.qemu_arguments
  );
  Ok(())
}

/// Performs miscellaneous code (quality) checks, like running Clippy, formatting,
/// documentation, etc.
fn check(arch_specification: &super::command::ArchitectureSpecification) -> anyhow::Result<()> {
  /// A simple wrapper around [`run_command_and_check`] to ease calling checks.
  macro_rules! check {
    ($arguments:expr) => {{
      run_command_and_check!(env!("CARGO"), $arguments);
    }};
  }

  // clippy
  check!(&["clippy", "--all-features", "--", "-D", "warnings"]);
  check!(&[
    "clippy",
    "--lib",
    "--target",
    arch_specification.target,
    "--package",
    "uncore",
    "--all-features",
    "--",
    "-D",
    "warnings"
  ]);

  // documentation
  check!(&["doc", "--document-private-items"]);
  check!(&["doc", "--lib", "--package", "uncore", "--document-private-items"]);

  // formatting
  check!(&["fmt", "--all", "--message-format", "human", "--", "--check"]);
  check!(&[
    "fmt",
    "--package",
    "uncore",
    "--all",
    "--message-format",
    "human",
    "--",
    "--check",
  ]);

  Ok(())
}
