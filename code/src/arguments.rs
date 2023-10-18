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
  command:      subcommands::Command,
}

impl Arguments {
  /// Get the log level specified by the arguments.
  pub fn get_log_level(&self) -> Option<log::Level> { self.verbosity.log_level() }

  /// Dispatches individual sub-commands to the correct sub-routines that execute the
  /// sub-commands.
  pub fn dispatch_command(self) -> anyhow::Result<()> {
    self.command.execute(self.architecture)?;
    Ok(())
  }
}

/// This module is responsible for properly executing individual sub-commands for a
/// given architecture.
mod subcommands {
  /// Specifies which sub-command are available, i.e. whether the user wants to build the
  /// kernel, run the kernel, etc.
  #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, clap::Subcommand)]
  pub(super) enum Command {
    /// Build the kernel
    Build,
    /// Run the kernel
    Run,
    /// Test the kernel
    Test,
  }

  impl Command {
    /// TODO
    pub(super) fn execute(self, architecture: super::Architecture) -> anyhow::Result<()> {
      check_dependencies(architecture)?;
      match self {
        Self::Build => build(&architecture.into())?,
        Self::Run => run(&architecture.into())?,
        Self::Test => anyhow::bail!("The test sub-command is not yet implemented"),
      }
      Ok(())
    }
  }

  /// TODO
  struct ArchitectureSpecification {
    /// TODO
    pub target:             &'static str,
    /// TODO
    pub qemu_command:       &'static str,
    /// TODO
    pub linker_script_path: String,
    /// TODO
    pub qemu_arguments:     Vec<String>,
  }

  impl From<crate::arguments::Architecture> for ArchitectureSpecification {
    fn from(val: crate::arguments::Architecture) -> Self {
      match val {
        crate::arguments::Architecture::Riscv64 => Self {
          target:             "riscv64gc-unknown-none-elf",
          qemu_command:       "qemu-system-riscv64",
          linker_script_path: std::env::var("CARGO_MANIFEST_DIR").expect("msg")
            + "/uncore/src/arch/risc-v/qemu.ld",
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

  /// TODO
  fn check_dependencies(architecture: super::Architecture) -> anyhow::Result<()> {
    use anyhow::Context;

    /// TODO
    macro_rules! check_bin {
      ($command:tt) => {
        which::which($command).context(format!("Package {} seems to be missing", $command))?;
      };

      ($command:expr, $package:expr) => {
        which::which($command).context(format!("Package {} seems to be missing", $package))?;
      };
    }

    match architecture {
      super::Architecture::Riscv64 => {
        check_bin!("qemu-system-riscv64");
      },
    }

    Ok(())
  }

  /// TODO
  fn build(arch_specification: &ArchitectureSpecification) -> anyhow::Result<()> {
    std::process::Command::new(env!("CARGO"))
      .args([
        "build",
        "--package",
        "uncore",
        "--target",
        arch_specification.target,
      ])
      .env(
        "RUSTFLAGS",
        format!("-Clink-arg=-T{}", arch_specification.linker_script_path),
      )
      .status()?;

    Ok(())
  }

  /// TODO
  fn run(arch_specification: &ArchitectureSpecification) -> anyhow::Result<()> {
    std::process::Command::new(arch_specification.qemu_command)
      .args(&arch_specification.qemu_arguments)
      .status()?;

    Ok(())
  }
}
