// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

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
// Since the `x86-interrupt` calling convention is still unstable, we
// have to opt-in.
#![feature(abi_x86_interrupt)]

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
				.set_stack_index(1);
		}

		idt
	};
}

pub extern "x86-interrupt" fn test_double_fault_handler(_: idt::InterruptStackFrame, _: u64) -> !
{
	log_info!("Received double fault. SUCCESS.");
	test::qemu::exit_with_success();
	never_return()
}

// TODO [FIXME]
// ! THIS TEST DOES NOT CURRENTLY WORK
// it seems that the stack overflow does not yet result in the double
// fault handler being called properly. Why? I don't know. The kernel
// itself can call the double fault handler properly (one can test
// this by uncommenting `x86_64::instructions::interrupts::int3();` in
// line 81 - then you see that the test succeeds). I'm currently
// thinking about stack guard pages (but they are set up by the OS -
// i.e. we have not set up one), and I don't know why no exception is
// triggered!!! I BET however that this has something to do with our
// stack setup in out boot code (I'm looking at you, `start.S`) :D

#[no_mangle]
pub extern "C" fn efi_main(
	uefi_handle: uefi::Handle,
	uefi_system_table_boot: library::boot::UEFISystemTableBootTime,
) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	main(library::boot::exit_boot_services(
		uefi_handle,
		uefi_system_table_boot,
	))
}

fn main(_: library::boot::UEFIMemoryMap) -> !
{
	library::log::init(Some(log::Level::Trace));
	library::log::display_initial_information();

	log_info!("This is the 'stack_overflow' test");

	library::architectures::cpu::initialize();

	TEST_IDT.load();
	log_info!("Initialized new (test) IDT.");

	x86_64::instructions::interrupts::int3();
	stack_overflow();

	log_error!("Execution continued after kernel stack overflow");
	test::qemu::exit_with_failure();

	never_return()
}

#[allow(unconditional_recursion)]
fn stack_overflow()
{
	stack_overflow();
	// prevent tail call optimization
	volatile::Volatile::new(&0).read();
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(false, panic_info) }
