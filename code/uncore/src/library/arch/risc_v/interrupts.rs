// SPDX-License-Identifier: GPL-3.0-or-later

//! Contains all interrupt handlers. These handlers are set up by [`riscv-rt`].

/// This function is used by [`riscv-rt`] to provide a default interrupt handler. This
/// handler reports the interrupt and exists the kernel.
#[export_name = "DefaultHandler"]
pub fn default_handler() {
  todo!("Default Interrupt handler is todo");
}

/// todo
#[export_name = "UserSoft"]
pub fn user_software() {
  todo!("User Software Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "SupervisorSoft"]
pub fn supervisor_software() {
  todo!("Supervisor Software Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "MachineSoft"]
pub fn machine_software() {
  todo!("Machine Software Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "UserTimer"]
pub fn user_timer() {
  todo!("User Timer Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "SupervisorTimer"]
pub fn supervisor_timer() {
  todo!("User Timer Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "MachineTimer"]
pub fn machine_timer() {
  todo!("User Timer Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "UserExternal"]
pub fn user_external() {
  todo!("User External Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "SupervisorExternal"]
pub fn supervisor_external() {
  todo!("Supervisor External Interrupt triggered but interrupt handler is todo");
}

/// todo
#[export_name = "MachineExternal"]
pub fn machine_external() {
  todo!("Machine External Interrupt triggered but interrupt handler is todo");
}

/// This function is used by [`riscv-rt`] to provide an exception handler.
#[export_name = "ExceptionHandler"]
fn exception_handler(_trap_frame: &riscv_rt::TrapFrame) -> ! {
  todo!("Exception occurred but handler has not been written");
}
