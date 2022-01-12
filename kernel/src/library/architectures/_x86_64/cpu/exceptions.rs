// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ### Double Fault Interrupt Stack Table Index
///
/// This constant defines the stack to use in the Interrupt Stack
/// Table (IST) field on the TSS for the double fault handler. The
/// first index is chosen.
///
/// The `interrupt_stack_table` is a field in the Task State Segment
/// (TSS) struct. It can be used to switch kernel stacks.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// ## CPU Exception Handlers
///
/// This module contains all CPU exception handler callback functions.
/// They are registered in the Interrupt Descriptor Table (IDT).
pub(super) mod handlers
{
	use crate::prelude::*;

	use x86_64::structures::idt::{
		InterruptStackFrame,
		PageFaultErrorCode,
	};

	/// ### CPU Exception - Breakpoint Handler
	///
	/// This is the handler callback function for the Breakpoint
	/// CPU Exception.
	pub extern "x86-interrupt" fn breakpoint(_stack_frame: InterruptStackFrame)
	{
		crate::log_info!("CPU exception occurred (breakpoint, no abort)");
	}

	/// ### CPU Exception - Double Fault Handler
	///
	/// This is the handler callback function for the Double Fault
	/// CPU Exception
	///
	/// #### Trivia
	///
	/// One difference to the breakpoint handler is that the
	/// double fault handler is diverging. The reason is that
	/// the `x86_64` architecture does not permit returning from
	/// a double fault exception.
	pub extern "x86-interrupt" fn double_fault(
		stack_frame: InterruptStackFrame,
		_error_code: u64,
	) -> !
	{
		log_error!(
			"Fatal CPU exception occurred (double fault, abort through panic)\n{:#?}\n",
			stack_frame
		);
		panic!("received fatal double fault exception");
	}

	/// ### CPU Exception - Page Fault Handler
	///
	/// This is the handler callback function for the page fault
	/// CPU exception.
	pub extern "x86-interrupt" fn page_fault(
		_stack_frame: InterruptStackFrame,
		error_code: PageFaultErrorCode,
	)
	{
		crate::log_warning!("CPU exception occurred (page fault, no abort)");
		crate::log_info!(
			"page fault information: accessed address = {:?} | error code = {:?}\n",
			x86_64::registers::control::Cr2::read(),
			error_code
		);
		// crate::library::never_return();
	}

	#[test_case]
	fn breakpoint_exception() { x86_64::instructions::interrupts::int3(); }
}
