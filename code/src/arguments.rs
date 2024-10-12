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

/// Workspace member that eases working with `unCORE`.
#[derive(Debug, clap::Parser)]
#[command(
  author,
  version,
  about="Workspace member that eases working with unCORE.",
  long_about=None,
  propagate_version=true)]
#[command(bin_name = "cargo run -q --")]
pub struct Arguments {
  /// Specify the verbosity
  #[clap(flatten)]
  verbosity:        clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,
  /// Specify the architecture unCORE is built for.
  #[clap(short, long, value_enum, default_value_t=Architecture::Riscv64)]
  pub architecture: Architecture,
  /// Specify what to do: build the kernel, run the kernel, etc.
  #[command(subcommand)]
  pub command:      super::command::Command,
}

impl Arguments {
  /// Get the log level specified by the arguments.
  pub fn get_log_level(&self) -> Option<log::Level> { self.verbosity.log_level() }

  /// Dispatches individual sub-commands to the correct sub-routines that execute the
  /// sub-commands.
  pub fn dispatch_command(self) -> Result<(), ()> {
    log::debug!("Dispatching command '{}'", self.command);
    if crate::environment::is_inside_container() {
      log::debug!("Running inside a container");
    } else {
      log::debug!("Running outside a container");
    }
    match super::command::Command::execute(&self) {
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

/// Holds information about all architecture-specific files and commands.
pub struct ArchitectureSpecification {
  /// The target triple
  pub target:             &'static str,
  /// The QEMU command to execute
  pub qemu_command:       &'static str,
  /// Path to the linker script
  pub linker_script_path: String,
  /// The parameters of the QEMU command to execute
  qemu_arguments:         Vec<&'static str>,
  /// Default path to the kernel binary when `main.rs` is used (i.e. no tests are run)
  kernel_binary_path:     String,
}

impl ArchitectureSpecification {
  /// Uses the environment variable `CARGO_MANIFEST_DIR` to construct an absolute path.
  fn append_to_base_dir(suffix: &str) -> String {
    std::env::var("CARGO_MANIFEST_DIR").expect("Could not obtain value from 'CARGO_MANIFEST_DIR'") + suffix
  }

  /// Returns the default set of arguments required to run QEMU.
  pub fn qemu_arguments(&self) -> Vec<&str> { self.qemu_arguments.clone() }

  /// Like [`Self::qemu_arguments`], but implicitly adds the standard kernel binary to the
  /// set of arguments.
  pub fn qemu_arguments_with_kernel(&self) -> Vec<&str> {
    let mut vec = self.qemu_arguments.clone();
    vec.push("-kernel");
    vec.push(self.kernel_binary_path.as_str());
    vec
  }

  /// Returns an instance of [`ArchitectureSpecification`] that is suitable for RISC-V
  /// 64bit.
  fn riscv64() -> Self {
    let mut kernel_binary_path = Self::append_to_base_dir("/");
    kernel_binary_path.push_str("target/riscv64gc-unknown-none-elf/debug/uncore");

    Self {
      target: "riscv64gc-unknown-none-elf",
      qemu_command: "qemu-system-riscv64",
      linker_script_path: Self::append_to_base_dir("/uncore/src/library/arch/risc_v/linking.ld"),
      qemu_arguments: vec![
        "-machine",
        "virt",
        "-cpu",
        "rv64",
        "-smp",
        "1",
        "-m",
        "128M",
        "-nographic",
        "-serial",
        "mon:stdio",
        "-device",
        "virtio-rng-device",
        "-device",
        "virtio-gpu-device",
        "-device",
        "virtio-net-device",
        "-device",
        "virtio-keyboard-device",
      ],
      kernel_binary_path,
    }
  }
}

impl From<Architecture> for ArchitectureSpecification {
  fn from(val: crate::arguments::Architecture) -> Self {
    match val {
      crate::arguments::Architecture::Riscv64 => Self::riscv64(),
    }
  }
}
