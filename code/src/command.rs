// SPDX-License-Identifier: GPL-3.0-or-later

//! Holds all functionality required for building, running, debugging and testing
//! `unCORE`.

use super::arguments;

/// Specifies which sub-command are available, i.e. whether the user wants to build the
/// kernel, run the kernel, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, clap::Subcommand)]
pub enum Command {
  /// Build the kernel
  Build,
  /// Run the kernel
  Run {
    /// Specify whether you want to debug the kernel
    #[clap(short, long)]
    debug: bool,
  },
  /// Test the kernel by running unit tests
  UTest {
    /// Specify whether you want to debug a test (only works when a specific test is
    /// supplied)
    #[clap(short, long)]
    debug: bool,
  },
  /// Test the kernel by running integration tests
  ITest {
    /// Specify whether you want to debug a test (only works when a specific test is
    /// supplied)
    #[clap(short, long)]
    debug: bool,
    /// Specify which test to run
    #[clap(short, long)]
    test:  Option<String>,
  },
  /// Check the code (e.g. with `clippy`)
  Check,
}

impl Command {
  /// Dispatches the given subcommand, thereby executing the correct action (building,
  /// running, debugging, etc.).
  pub fn execute(arguments: &arguments::Arguments) -> anyhow::Result<()> {
    let architecture = arguments.architecture;
    check_build_time_dependencies(architecture)?;
    let architecture_specification: &arguments::ArchitectureSpecification = &arguments.architecture.into();

    match &arguments.command {
      Self::Build => build(architecture_specification)?,
      Self::Run { debug } => {
        check_run_time_dependencies(architecture)?;
        build(architecture_specification)?;
        run(architecture_specification, *debug)?;
      },
      Self::UTest { debug } => {
        check_run_time_dependencies(architecture)?;
        run_unit_tests(architecture_specification, *debug)?;
      },
      Self::ITest { debug, test } => {
        check_run_time_dependencies(architecture)?;
        run_integration_tests(architecture_specification, *debug, test)?;
      },
      Self::Check => {
        check(architecture_specification)?;
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

use anyhow::Context;

/// Short-hand for calling [`which`].
macro_rules! check_bin {
  ($command:tt) => {
    which::which($command).context(format!("Package '{}' seems to be missing", $command))?;
  };

  ($command:expr, $package:expr) => {
    which::which($command).context(format!("Package '{}' seems to be missing", $package))?;
  };
}

/// Checks all dependencies required to build `unCORE`.
#[allow(clippy::unnecessary_wraps)]
fn check_build_time_dependencies(_architecture: arguments::Architecture) -> anyhow::Result<()> {
  log::debug!("Checking build-time dependencies");

  log::trace!("Build-time dependencies are satisfied");
  Ok(())
}

/// Checks all dependencies required to run `unCORE`.
fn check_run_time_dependencies(architecture: arguments::Architecture) -> anyhow::Result<()> {
  log::debug!("Checking build-time dependencies");

  check_bin!("jq");

  match architecture {
    arguments::Architecture::Riscv64 => {
      check_bin!("qemu-system-riscv64");
      log::trace!("Checking dependencies required for debugging");
      check_bin!("gdb-multiarch");
    },
  }

  log::trace!("Build-time dependencies are satisfied");
  Ok(())
}

/// Run a given command, taking arguments and environment variables if necessary, and
/// evaluates the exit status in the end.
macro_rules! run_command_and_check {
  ($command_name:expr, $arguments:expr) => {
    run_command_and_check!($command_name, $arguments, [("__UNUSED", "")])
  };

  ($command_name:expr, $arguments:expr, $envs:expr) => {{
    let __special: anyhow::Result<()> = if std::process::Command::new($command_name)
      .args($arguments)
      .envs($envs)
      .status()?
      .success()
    {
      Ok(())
    } else {
      anyhow::bail!("Failure: could not determine exit status - terminated by signal?")
    };
    __special
  }};
}

/// Build the kernel.
fn build(arch_specification: &arguments::ArchitectureSpecification) -> anyhow::Result<()> {
  log::info!("Building unCORE");

  let cargo_build_environment =
    super::environment::get_all_environment_variables_for_build(&arch_specification.linker_script_path)?;

  run_command_and_check!(
    env!("CARGO"),
    [
      "build",
      "--package",
      "uncore",
      "--target",
      arch_specification.target,
    ],
    cargo_build_environment
  )
}

/// Run the kernel.
fn run(arch_specification: &arguments::ArchitectureSpecification, is_debug: bool) -> anyhow::Result<()> {
  let mut arguments = arch_specification.qemu_arguments_with_kernel();
  if is_debug {
    log::info!("Debugging unCORE");
    log::debug!("You may use 'gdb-multiarch -q -x misc/gdb/init.txt' to attach now");
    log::trace!("Remember: 'Ctrl-A x' will exit QEMU");
    arguments.append(&mut vec!["-s", "-S"]);
  } else {
    log::info!("Running unCORE");
  };

  run_command_and_check!(arch_specification.qemu_command, arguments)
}

/// Builds test binaries. Depending on the input, this function builds unit or integration
/// test binaries (or a single binary). The output is a list of binaries that are to be
/// run.
fn create_test_binaries<I, S>(
  arch_specification: &arguments::ArchitectureSpecification,
  extra_cargo_arguments: I,
) -> anyhow::Result<Vec<String>>
where
  I: Clone + IntoIterator<Item = S>,
  S: AsRef<std::ffi::OsStr>,
{
  // Prepare the environment for building the test binary
  let cargo_build_environment =
    super::environment::get_all_environment_variables_for_build(&arch_specification.linker_script_path)?;

  let cargo_arguments = [
    "test",
    "--package",
    "uncore",
    "--target",
    arch_specification.target,
    "--no-run",
  ];

  // Build the integration test binary; do not run it, and produce JSON output that we can
  // parse later with jq
  let cargo = std::process::Command::new(env!("CARGO"))
    .args(cargo_arguments)
    .args(extra_cargo_arguments.clone())
    .arg("--message-format=json")
    .envs(&cargo_build_environment)
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::null())
    .output()?;

  // In case building the integration test binary was unsuccessful, we want to provide the
  // user with what has happened; therefore, we run Cargo again, but this time we do not
  // produce JSON output.
  if !cargo.status.success() {
    let _ = std::process::Command::new(env!("CARGO"))
      .args(cargo_arguments)
      .args(extra_cargo_arguments)
      .envs(&cargo_build_environment)
      .status()?;
    anyhow::bail!("Could not build test binaries")
  }

  // Parse which binaries we can use
  let mut jq = std::process::Command::new("jq")
    .args(["-r", "select(.profile.test == true) | .filenames[]"])
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .spawn()?;
  let stdin = jq.stdin.take();
  if let Some(mut stdin) = stdin {
    use std::io::Write;
    stdin.write_all(std::str::from_utf8(&cargo.stdout)?.as_bytes())?;
  } else {
    anyhow::bail!("Could not acquire stdin of jq");
  }

  // Create a list of all binaries to run
  let jq = jq.wait_with_output()?;
  let test_binaries: Vec<&str> = std::str::from_utf8(&jq.stdout)?.trim().split('\n').collect();
  if test_binaries.is_empty() {
    anyhow::bail!("Cargo did not create a test binary?!");
  }

  Ok(
    test_binaries
      .into_iter()
      .map(std::string::ToString::to_string)
      .collect(),
  )
}

/// TODO
fn parse_binary_name(binary_name: &str) -> anyhow::Result<String> {
  let regex = regex::Regex::new(r".*/(.+)-.+")?;
  regex
    .captures(binary_name)
    .map_or_else(|| Ok(String::from("unknown")), |name| Ok(name[1].to_string()))
}

/// TODO
fn create_gdb_init_file(binary_location: &String, test_name: &String) -> anyhow::Result<()> {
  use std::io::Write;
  /// TODO
  const FILE_LOCATION: &str = "/tmp/init.gdb";
  let mut w = std::fs::File::create(FILE_LOCATION)?;
  writeln!(
    &mut w,
    "set mi-async
set architecture riscv
set pagination off
set print asm-demangle on

file {binary_location}
symbol-file {binary_location}

layout asm
layout regs
focus cmd

br {test_name}::__risc_v_rt__main

define kq
  kill
  quit 5
end

target remote :1234
continue"
  )?;

  log::info!(
    "You can now attach to QEMU via 'gdb-multiarch -q -x {}'",
    FILE_LOCATION
  );
  Ok(())
}

/// Runs all unit tests. In case of `unCORE`, only the library part contains unit tests
/// (i.e. only files associated with `lib.rs`, which are all files except for `main.rs`;
/// hence, all files belong to the library). This makes running the tests and extracting
/// the binary to run easy as there is only one.
fn run_unit_tests(
  arch_specification: &arguments::ArchitectureSpecification,
  is_debug: bool,
) -> anyhow::Result<()> {
  log::info!("Building unit test binary");
  let test_binaries = create_test_binaries(arch_specification, ["--lib"])?;
  let test_binary = test_binaries.get(0);
  if let Some(binary) = test_binary {
    let mut qemu_arguments = arch_specification.qemu_arguments();
    qemu_arguments.append(&mut vec!["-kernel", binary]);

    if is_debug {
      log::info!("Debugging unCORE unit tests");
      qemu_arguments.append(&mut vec!["-s", "-S"]);
      create_gdb_init_file(&parse_binary_name(binary)?, binary)?;
    } else {
      log::info!("Running unCORE unit tests");
      log::trace!("The unit test binary file is '{}'", binary);
    }

    run_command_and_check!(arch_specification.qemu_command, qemu_arguments)?;
  } else {
    // This part is unreachable because [`test_helper`] does already check whether the vector
    // contains at least one element and returns an error otherwise; which is caught by the
    // `?` operator in the line above.
    unreachable!();
  }

  log::info!("Unit tests finished successfully");
  Ok(())
}

/// Runs all or a specific integration test. When `is_debug` is `true`, then QEMU can be
/// attached to debug the test. If `test` is `Some(test_name)`, then the integration test
/// with the name `test_name` is built and run.
fn run_integration_tests(
  arch_specification: &arguments::ArchitectureSpecification,
  is_debug: bool,
  test: &Option<String>,
) -> anyhow::Result<()> {
  log::info!("Building integration test binaries");
  let mut qemu_arguments = arch_specification.qemu_arguments();
  let test_binaries = if let Some(test) = test {
    // If a test name is supplied, we may debug it
    if is_debug {
      qemu_arguments.append(&mut vec!["-s", "-S"]);
    }
    create_test_binaries(arch_specification, ["--test", test])?
  } else {
    create_test_binaries(arch_specification, ["--test", "*"])?
  };

  // Run every test in the list of integration tests
  for binary in test_binaries {
    let test_name = parse_binary_name(&binary)?;
    if is_debug {
      log::info!("Debugging integration test '{}'", test_name);
      create_gdb_init_file(&binary, &test_name)?;
    } else {
      log::info!("Running integration test '{}'", test_name);
    }
    log::trace!("The integration test binary file is '{}'", binary);
    let mut current_arguments = qemu_arguments.clone();
    current_arguments.append(&mut vec!["-kernel", binary.as_str()]);
    run_command_and_check!(arch_specification.qemu_command, current_arguments)?;
    log::info!("Integration test '{}' finished successfully", test_name);
  }

  Ok(())
}

/// Perform miscellaneous code (quality) checks:
///
/// - `cargo clippy`: general code quality
/// - `cargo fmt`: formatting
/// - `cargo doc`: documentation
fn check(arch_specification: &arguments::ArchitectureSpecification) -> anyhow::Result<()> {
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
    "--target",
    arch_specification.target,
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
