// SPDX-License-Identifier: GPL-3.0-or-later

//! Contains all interrupt handlers. These handlers are set up by [`riscv-rt`].

/// This function is used by [`riscv-rt`] to provide an exception handler.
#[export_name = "ExceptionHandler"]
fn default_exception_handler(_trap_frame: &riscv_rt::TrapFrame) -> ! {
  todo!("Exception occurred but handler has not been written");
}

/// This function is used by [`riscv-rt`] to provide an interrupt handler.
#[export_name = "DefaultHandler"]
fn default_interrupt_handler(_trap_frame: &riscv_rt::TrapFrame) -> ! {
  todo!("Exception occurred but handler has not been written");
}
