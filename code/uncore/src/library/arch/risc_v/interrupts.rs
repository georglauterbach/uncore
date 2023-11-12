// SPDX-License-Identifier: GPL-3.0-or-later

//! Contains all interrupt handlers. These handlers are set up by [`riscv-rt`].

/// This function is used by [`riscv-rt`] to provide a default interrupt handler. This
/// handler reports the interrupt and exists the kernel.
#[export_name = "DefaultHandler"]
pub fn default_handler() {
  log::error!("Interrupt without defined interrupt handler occurred");
  crate::arch::exit_kernel(crate::UncoreResult::Err);
}

/// This function is used by [`riscv-rt`] to provide an exception handler.
#[export_name = "ExceptionHandler"]
fn exception_handler(_trap_frame: &riscv_rt::TrapFrame) -> ! {
  panic!("Exception occurred but handler has not been written");
}
