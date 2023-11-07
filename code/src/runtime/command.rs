// SPDX-License-Identifier: GPL-3.0-or-later

//! Hols all functionality required for building, running, etc. `unCORE`.

/// Specifies which sub-command are available, i.e. whether the user wants to build the
/// kernel, run the kernel, etc.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, clap::Subcommand)]
pub enum Command {
  /// Build the kernel
  Build,
  /// Run the kernel
  Run,
  /// Test the kernel by running unit tests
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
    let architecture: &super::arguments::ArchitectureSpecification = &architecture.into();

    match self {
      Self::Build => build(architecture)?,
      Self::Run => {
        build(architecture)?;
        run(architecture)?;
      },
      Self::Test => {
        test(architecture)?;
      },
      Self::Debug => {
        build(architecture)?;
        debug(architecture)?;
      },
      Self::Check => {
        check(architecture)?;
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

  check_bin!("jq");

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

/// Runs a given command, taking arguments and environment variables if necessary, and
/// evaluates the exit status in the end.
macro_rules! run_command_and_check {
  ($command_name:expr, $arguments:expr) => {
    run_command_and_check!($command_name, $arguments, [("__unused", "")])
  };

  ($command_name:expr, $arguments:expr, $envs:expr) => {{
    let __special: anyhow::Result<()> = if let Some(status) = std::process::Command::new($command_name)
      .args($arguments)
      .envs($envs)
      .status()?
      .code()
    {
      if status == 0 {
        Ok(())
      } else {
        anyhow::bail!("Command exited with status code {}", status)
      }
    } else {
      anyhow::bail!("Failure: could not determine exit status - terminated by signal?")
    };
    __special
  }};
}

/// Builds the kernel.
fn build(arch_specification: &super::arguments::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Building unCORE");

  let mut environment = super::environment::get_all_environment_variables_for_build()?;
  environment.insert(
    "RUSTFLAGS",
    format!("-C link-arg=-T{}", arch_specification.linker_script_path),
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
  )
}

/// Runs the kernel given an [`super::arguments::ArchitectureSpecification`].
fn run(arch_specification: &super::arguments::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Running unCORE now");

  run_command_and_check!(
    arch_specification.qemu_command,
    &arch_specification.qemu_arguments_with_kernel()
  )
}

/// Runs the kernel given an [`super::arguments::ArchitectureSpecification`] with debug
/// attributes.
fn debug(arch_specification: &super::arguments::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Debugging unCORE");
  log::debug!("You may use 'gdb-multiarch -q -x misc/gdb/init.txt' to attach now");
  log::debug!("Remember: 'Ctrl-A x' will exit QEMU");

  let mut arguments = arch_specification.qemu_arguments_with_kernel();
  arguments.append(&mut vec!["-s", "-S"]);

  run_command_and_check!(arch_specification.qemu_command, arguments)
}

/// TODO
fn test(arch_specification: &super::arguments::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Testing unCORE now");

  let mut environment = super::environment::get_all_environment_variables_for_build()?;
  environment.insert(
    "RUSTFLAGS",
    format!("-C link-arg=-T{}", arch_specification.linker_script_path),
  );

  let cargo = std::process::Command::new(env!("CARGO"))
    .args([
      "test",
      "--lib",
      "--package",
      "uncore",
      "--target",
      arch_specification.target,
      "--no-run",
      "--message-format=json",
    ])
    .envs(&environment)
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::null())
    .spawn()?;

  let jq = std::process::Command::new("jq")
    .args(["-r", "select(.profile.test == true) | .filenames[]"])
    .stdin(std::process::Stdio::from(cargo.stdout.unwrap()))
    .stdout(std::process::Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  let kernel_test_binary = std::str::from_utf8(&jq.stdout)?.trim();
  if kernel_test_binary.is_empty() {
    anyhow::bail!("Cargo did not create a test binary");
  }

  let mut arguments = arch_specification.qemu_arguments();
  arguments.append(&mut vec!["-kernel", std::str::from_utf8(&jq.stdout)?.trim()]);

  run_command_and_check!(arch_specification.qemu_command, arguments)
}

/// Performs miscellaneous code (quality) checks, like running Clippy, formatting,
/// documentation, etc.
fn check(arch_specification: &super::arguments::ArchitectureSpecification) -> anyhow::Result<()> {
  /// A simple wrapper around [`run_command_and_check`] to ease calling checks.
  macro_rules! check {
    ($arguments:expr) => {{
      run_command_and_check!(env!("CARGO"), $arguments)?;
    }};
  }

  // clippy
  check!(&["clippy", "-q", "--all-features", "--", "-D", "warnings"]);
  check!(&[
    "clippy",
    "-q",
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
  check!(&["doc", "-q", "--document-private-items"]);
  check!(&[
    "doc",
    "-q",
    "--lib",
    "--package",
    "uncore",
    "--document-private-items"
  ]);

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
