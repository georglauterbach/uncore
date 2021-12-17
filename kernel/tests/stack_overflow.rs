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

use kernel::library::{
	self,
	helper::log,
};

use x86_64::structures::idt::{
	InterruptDescriptorTable,
	InterruptStackFrame,
};

lazy_static::lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
	let mut idt = InterruptDescriptorTable::new();

	unsafe {
	    idt.double_fault
		.set_handler_fn(test_double_fault_handler)
		.set_stack_index(0);
	}

	idt
    };
}

pub extern "x86-interrupt" fn test_double_fault_handler(_: InterruptStackFrame, _: u64) -> !
{
	kernel::log_info!("Received double fault. SUCCESS.");
	kernel::library::helper::qemu::exit_with_success();
	library::never_return()
}

#[no_mangle]
pub extern "C" fn _start(boot_information: &'static mut bootloader::BootInfo) -> !
{
	log::set_log_level(log::Level::Trace);
	kernel::log!("Running an integration test.");

	library::init(boot_information);
	TEST_IDT.load();
	kernel::log_info!("Initialized new (test) IDT.");

	stack_overflow();

	kernel::log_error!("Execution continued after kernel stack overflow");
	panic!()
}

#[allow(unconditional_recursion)]
fn stack_overflow()
{
	stack_overflow();
	volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { library::panic_callback(false, panic_info) }
