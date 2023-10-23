// SPDX-License-Identifier: GPL-3.0-or-later

//! Hols all functionality required for building, running, etc. unCORE.

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
  /// Debug the kernel by allowing GDB to attach
  Debug,
}

impl Command {
  /// Actually dispatches the given subcommand by matching on `Self`.
  pub fn execute(self, architecture: super::arguments::Architecture) -> anyhow::Result<()> {
    check_dependencies(architecture, self == Self::Debug)?;
    build(&architecture.into())?;

    match self {
      Self::Build => {},
      Self::Run => run(&architecture.into())?,
      Self::Test => anyhow::bail!("The test sub-command is not yet implemented"),
      Self::Debug => debug(&mut architecture.into())?,
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
    match val {
      crate::runtime::arguments::Architecture::Riscv64 => Self {
        target:             "riscv64gc-unknown-none-elf",
        qemu_command:       "qemu-system-riscv64",
        linker_script_path: std::env::var("CARGO_MANIFEST_DIR").expect("msg")
          + "/uncore/src/arch/risc_v/boot/qemu.ld",
        qemu_arguments:     vec![
          "-machine".to_string(),
          "virt".to_string(),
          "-cpu".to_string(),
          "rv64".to_string(),
          "-smp".to_string(),
          "4".to_string(),
          "-m".to_string(),
          "128M".to_string(),
          "-nographic".to_string(),
          "-serial".to_string(),
          "mon:stdio".to_string(),
          "-device".to_string(),
          "virtio-rng-device".to_string(),
          "-device".to_string(),
          "virtio-gpu-device".to_string(),
          "-device".to_string(),
          "virtio-net-device".to_string(),
          "-device".to_string(),
          "virtio-tablet-device".to_string(),
          "-device".to_string(),
          "virtio-keyboard-device".to_string(),
          "-bios".to_string(),
          "none".to_string(),
          "-kernel".to_string(),
          std::env::var("CARGO_MANIFEST_DIR").expect("msg")
            + "/target/riscv64gc-unknown-none-elf/debug/uncore",
        ],
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
        log::trace!("  -> not a debug session - not checking debug dependencies");
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
      0 => {
        log::trace!("  -> success");
        Ok(())
      },
      _ => anyhow::bail!("  -> failure: command exited with status code {}", status),
    }
  } else {
    anyhow::bail!("  -> failure: could not determine exit status - terminated by signal?")
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
  run_command_and_check!(
    env!("CARGO"),
    [
      "build",
      "--package",
      "uncore",
      "--target",
      arch_specification.target,
    ],
    [
      (
        "RUSTFLAGS",
        format!("-Clink-arg=-T{}", arch_specification.linker_script_path),
      ),
      (
        "__UNCORE__BUILD_TIME",
        chrono::offset::Local::now().format("%+").to_string()
      )
    ]
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
