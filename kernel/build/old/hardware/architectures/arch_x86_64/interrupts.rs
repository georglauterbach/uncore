// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static::lazy_static! {

	/// ### Interrupt Descriptor Table (IDT)
	///
	/// "In order to catch and handle exceptions, we have to set up a
	/// so-called Interrupt Descriptor Table (IDT). In this table we
	/// can specify a handler function for each CPU exception." [1]
	///
	/// Moreover, interrupt handler callback functions are also
	/// registered in the IDT.
	///
	/// [1]: https://os.phil-opp.com/cpu-exceptions/#the-interrupt-descriptor-table
	static ref IDT: InterruptDescriptorTable = {
		use super::exceptions;

		let mut idt = InterruptDescriptorTable::new();

		// register exception handler callback functions
		idt.breakpoint.set_handler_fn(
			exceptions::handlers::breakpoint
		);

		idt.page_fault.set_handler_fn(
			exceptions::handlers::page_fault
		);

		unsafe {
			idt.double_fault
				.set_handler_fn(exceptions::handlers::double_fault)
				.set_stack_index(exceptions::DOUBLE_FAULT_IST_INDEX);
		}

		// register interrupt handler callback functions
		idt[handlers::InterruptIndex::Timer as usize]
			.set_handler_fn(handlers::timer);

		idt[handlers::InterruptIndex::Keyboard as usize]
			.set_handler_fn(handlers::keyboard);

		idt
	};
}

/// ### Initializing Interrupts
///
/// This function initializes the Interrupt Descriptor Table (IDT),
/// all programmable interrupt controllers (PICs) and enabled
/// interrupts in the end.
pub(super) fn init()
{
	crate::log_trace!("Initializing Interrupt Descriptor Table (IDT)");
	IDT.load();

	crate::log_trace!("Initializing programmable interrupt controllers (PICs)");
	unsafe {
		handlers::PICS.lock().initialize();
	}

	crate::log_trace!("Enabling interrupts");
	x86_64::instructions::interrupts::enable();
}

/// ## CPU Interrupt Handlers
///
/// This module contains all CPU interrupt handler callback functions.
/// They are registered in the Interrupt Descriptor Table (IDT).
///
/// We use programmable interrupt controllers (PICs) to notify out
/// kernel of these hardware interrupts.
mod handlers
{
	use pic8259::ChainedPics;
	use spin::Mutex;
	use x86_64::structures::idt::InterruptStackFrame;

	/// ### Offset for PIC No. 1
	///
	/// The default configuration of the PICs is not usable,
	/// because it sends interrupt vector numbers in the range
	/// 0–15 to the CPU. These numbers are already occupied by CPU
	/// exceptions, for example number 8 corresponds to a double
	/// fault. To fix this overlapping issue, we need to remap the
	/// PIC interrupts to different numbers. The actual range
	/// doesn't matter as long as it does not overlap with the
	/// exceptions, but typically the range 32–47 is chosen,
	/// because these are the first free numbers after the 32
	/// exception slots.
	const PIC_1_OFFSET: u8 = 32;

	/// ### Offset for PIC No. 2
	///
	/// Adds on top of [`PIC_1_OFFSET`]
	const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

	/// ### Interrupt Indices
	///
	/// Instead of hardcoding interrupt indices, we store them in
	/// this enum for each interrupt.
	#[derive(Debug, Clone, Copy)]
	#[repr(u8)]
	pub enum InterruptIndex
	{
		/// The timer interrupt index for the chained PIC
		Timer = PIC_1_OFFSET,
		/// The keyboard interrupt index for the chained PIC
		Keyboard,
	}

	/// ### Chained PICs
	///
	/// This structure provides access to the two chained PICs we
	/// use for notifying the CPU that an interrupt has occurred.
	pub(super) static PICS: Mutex<ChainedPics> =
		Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

	/// ### End of Interrupt Signalization
	///
	/// Every hardware (I/O) interrupt handler must issue an "end
	/// of interrupt" (EOI) signal at the end to signal that we're
	/// finished with processing the interrupt.
	///
	/// This function provides a safe wrapper around the unsafe
	/// method.
	fn notify_end_of_interrupt(interrupt_index: InterruptIndex)
	{
		unsafe {
			PICS.lock().notify_end_of_interrupt(interrupt_index as u8);
		}
	}

	/// ### CPU Interrupt - Timer Handler
	///
	/// This is the handler function for timer interrupts.
	pub extern "x86-interrupt" fn timer(_stack_frame: InterruptStackFrame)
	{
		notify_end_of_interrupt(InterruptIndex::Timer);
	}

	/// ### CPU Interrupt - Keyboard Handler
	///
	/// This is the handler function which reacts to keyboard
	/// input. Currently, every keystroke is printed directly on
	/// the screen.
	pub extern "x86-interrupt" fn keyboard(_stack_frame: InterruptStackFrame)
	{
		// use pc_keyboard::{
		// 	layouts::Us104Key,
		// 	DecodedKey,
		// 	HandleControl,
		// 	Keyboard,
		// 	ScancodeSet1,
		// };

		// lazy_static::lazy_static! {

		// 	/// ### The Keyboard Representation
		// 	///
		// 	/// This
		// 	static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> =
		// 		Mutex::new(Keyboard::new(Us104Key, ScancodeSet1,
		// HandleControl::Ignore)); }

		// let mut keyboard = KEYBOARD.lock();
		// let mut port = x86_64::instructions::port::Port::new(0x60);
		// let scancode: u8 = unsafe { port.read() };

		// if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		// 	if let Some(key) = keyboard.process_keyevent(key_event) {
		// 		match key {
		// 			DecodedKey::Unicode(character) => {
		// 				crate::log!("{}", character);
		// 			},
		// 			DecodedKey::RawKey(key) => {
		// 				crate::log!("{:?}", key);
		// 			},
		// 		}
		// 	}
		// }

		crate::log_info!("Keyboard interrupt occurred");

		notify_end_of_interrupt(InterruptIndex::Keyboard);
	}
}
