// SPDX-License-Identifier: GPL-3.0-or-later

//! The RISC-V 64bit architecture module file. This module file contains all other modules
//! for the RISC-V 64bit target.
//!
//! The QEMU variant is based on this code:
//! <https://github.com/qemu/qemu/blob/v8.1.2/hw/riscv/virt.c>.

pub mod drivers;
pub mod heap;
mod interrupts;

/// Architecture-specific functionality before the kernel setup in [`crate::setup_kernel`]
/// should run.
///
/// #### Attention
///
/// No logging is available here yet.
pub fn initialize(hart: usize) { drivers::initialize(hart); }

/// Architecture-specific kernel exit. This function uses [`sbi`] to stop the machine. In
/// case of an error, we need to use the SiFive-Test device because the [`sbi`] crate does
/// not exit QEMU in a way that an error is produced.
pub fn exit_kernel(condition: crate::UncoreResult) -> ! {
  use sbi::system_reset;

  if condition == crate::UncoreResult::Ok {
    log::info!("Terminating unCORE - execution successful");
    let _ = system_reset::system_reset(
      system_reset::ResetType::Shutdown,
      system_reset::ResetReason::NoReason,
    );
  } else {
    log::warn!("Terminating unCORE - execution unsuccessful");
    unsafe {
      core::arch::asm!(
          "sw {0}, 0({1})",
          in(reg)(1 << 16) | 0x3333, in(reg)0x10_0000
      );
    }
  };

  // Happens on when there is a bug in the SBI implementation or when SBI is not present
  // We need to ensure we do not panic again.
  log::error!("Shutdown failed - this is undefined behavior");

  // For the case that the QEMU exit attempt did not work, transition into an infinite
  // loop. Calling `panic!()` here is unfeasible, since there is a good chance
  // this function here is the last expression in the `panic!()` handler
  // itself. This prevents a possible infinite loop.
  unsafe {
    core::arch::asm!("wfi", options(nomem, nostack));
    core::hint::unreachable_unchecked();
  }
}
