use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::{
	exceptions_handlers::{
		breakpoint_handler,
		double_fault_handler,
	},
	gdt,
	super::io::interrupts,
};

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();

		// CPU exception handlers are now registered
		idt.breakpoint.set_handler_fn(breakpoint_handler);

		unsafe {
			idt.double_fault
				.set_handler_fn(double_fault_handler)
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}

		// hardware (I/O) interrupt handlers are now registered
		idt[interrupts::InterruptIndex::Timer as usize]
			.set_handler_fn(interrupts::timer_interrupt_handler);

		idt[interrupts::InterruptIndex::Keyboard as usize]
			.set_handler_fn(interrupts::keyboard_interrupt_handler);

		idt
	};
}

/// # Loading the IDT
///
/// The Interrupt Descriptor Table (IDT) is loaded here.
///
/// ## Registered CPU Exception Handlers
///
/// Currently, these CPU exception handlers are registered:
///
/// - Double Fault Handler (without kernel stack switch)
/// - Breakpoint Handler
///
/// ## Registered Hardware (I/O) Interrupt Handlers
///
/// Currently, these hardware (I/O) interrupt handlers are registered:
///
/// -
pub fn init() { IDT.load(); }
