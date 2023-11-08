// SPDX-License-Identifier: GPL-3.0-or-later

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
// As this is no ordinary program, we have a special entry-point,
// which is not the `main()` function.
#![no_main]
// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf) .
#![deny(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![deny(clippy::nursery)]
// Clippy lint target three. Enables new lints that are still
// under development
#![deny(clippy::pedantic)]
// Lint target for code documentation. This lint enforces code
// documentation on every code item.
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// Since the `x86-interrupt` calling convention is still unstable, we
// have to opt-in.
#![feature(abi_x86_interrupt)]

//! # Kernel-Stack Overflow Test
//!
//! Checks whether the kernel can handle a kernel stack overflow properly.

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use kernel::{
  library,
  prelude::*,
};

use x86_64::structures::idt;

lazy_static::lazy_static! {
  static ref TEST_IDT: idt::InterruptDescriptorTable = {
    let mut idt = idt::InterruptDescriptorTable::new();

    unsafe {
      idt.double_fault
        .set_handler_fn(test_double_fault_handler)
        .set_stack_index(0);
    }

    idt
  };
}

extern "x86-interrupt" fn test_double_fault_handler(_: idt::InterruptStackFrame, _: u64) -> ! {
  log_info!("Received double fault - nice");
  exit_kernel(kernel_types::ExitCode::Success)
}

bootloader::entry_point!(kernel_test_main);

fn kernel_test_main(_: &'static mut bootloader::BootInfo) -> ! {
  library::log::initialize(None);
  library::log::display_initial_information();

  log_info!("This is the 'stack_overflow' test");

  library::architectures::initialize();

  TEST_IDT.load();
  log_info!("Initialized new (test) IDT.");

  stack_overflow();

  log_error!("Execution continued after kernel stack overflow");
  exit_kernel(kernel_types::ExitCode::Failure)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow();
  // prevent tail call optimization
  volatile::Volatile::new(&0).read();
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic::callback(false, panic_info) }
