#![no_std]
#![no_main]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![feature(abi_x86_interrupt)]

/// # Imports
///
/// The `uncore::core_lib` is used here explicitly with the
/// `use` statement, and not with the `mod` statement. As
/// `uncore::core_lib` is already used in `lib.rs`, we do not
/// want to re-import it here and possibly confuse Cargo.
///
/// ## Macros
///
/// We will need to re-import all needed macros, as per definition
/// they reside in `crate`, which to be exact is `lib.rs`'s root
/// and **not** `main.rs`'s root.
///
/// Make sure to **always** use `core_lib::` instead of `crate::lib::` or
/// `lib::` or something else.
use uncore::core_lib;
use uncore::{
	serial_print,
	serial_println,
};

#[no_mangle]
pub extern "C" fn _start() -> !
{
	serial_print!("stack_overflow::stack_overflow...\t");

	core_lib::hw::cpu::gdt::init();
	init_test_idt();

	// trigger a stack overflow
	stack_overflow();

	panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow()
{
	// for each recursion, the return address is pushed
	stack_overflow();

	// prevent tail recursion optimizations
	volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { misc::panic::test_panic_handler(info) }

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
	static ref TEST_IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();

		unsafe {
			idt.double_fault
				.set_handler_fn(test_double_fault_handler)
				.set_stack_index(core_lib::hw::cpu::gdt::DOUBLE_FAULT_IST_INDEX);
		}

		idt
	};
}

pub fn init_test_idt() { TEST_IDT.load(); }

use uncore::core_lib::misc::helper::qemu::{
	_exit,
	ExitCode,
};
use x86_64::structures::idt::InterruptStackFrame;
use uncore::core_lib::misc;

extern "x86-interrupt" fn test_double_fault_handler(
	_stack_frame: &mut InterruptStackFrame,
	_error_code: u64,
) -> !
{
	serial_println!("[ok]");
	_exit(ExitCode::Success);

	misc::helper::__never_return()
}
