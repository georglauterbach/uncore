// SPDX-License-Identifier: GPL-3.0-or-later

//! This module holds all driver-related code for the RISC-V target.

pub mod qemu_uart;

/// Checks whether [`initialize`] has been called before.
static mut INIT_WAS_CALLED: bool = false;

/// Initializes all drivers for which code exists during startup of the kernel. This
/// function usually runs before [`crate::setup_kernel`].
///
/// #### Panics
///
/// If this function is called more than once, it panics, because initializing certain
/// drivers more than once is undefined behavior.
pub(super) fn initialize(hart: usize) {
  if hart != 0 {
    return;
  }

  assert!(
    unsafe { !INIT_WAS_CALLED },
    "called library/arch/risc_v/drivers/mod.rs:initialize more than once"
  );

  unsafe {
    INIT_WAS_CALLED = true;
  }

  qemu_uart::Uart::init();
  super::super::super::log::KernelLogger::enable_qemu_logger();
}
