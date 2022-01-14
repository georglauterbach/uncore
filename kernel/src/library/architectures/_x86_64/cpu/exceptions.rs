// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## CPU Exception Handlers
///
/// This module contains all CPU exception handler callback functions.
/// They are registered in the Interrupt Descriptor Table (IDT).
pub(super) mod handlers
{
	use crate::prelude::*;
	use x86_64::structures::idt;

	/// ### CPU Exception - Breakpoint Handler
	///
	/// This is the handler callback function for the Breakpoint
	/// CPU Exception.
	#[allow(dead_code)]
	pub extern "x86-interrupt" fn breakpoint(_stack_frame: idt::InterruptStackFrame)
	{
		crate::log_info!("CPU exception occurred (type: breakpoint)");
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
		stack_frame: idt::InterruptStackFrame,
		error_code: u64,
	) -> !
	{
		log_error!(
			"CPU exception occurred (type: double fault (fatal))\n\nError code: \
			 {}\n{:#?}\n",
			error_code,
			stack_frame,
		);

		panic!("fatal double fault CPU exception");
	}

	/// ### CPU Exception - Page Fault Handler
	///
	/// This is the handler callback function for the page fault
	/// CPU exception.
	pub extern "x86-interrupt" fn page_fault(
		_stack_frame: idt::InterruptStackFrame,
		_error_code: idt::PageFaultErrorCode,
	)
	{
		crate::log_warning!("CPU exception occurred (type: page fault)");
		// crate::log_info!(
		// 	"page fault information: accessed address = {:?} | error code =
		// {:?}\n", 	x86_64::registers::control::Cr2::read(),
		// 	error_code
		// );
		panic!("page fault are not yet implemented");
	}

	#[test_case]
	fn breakpoint_exception_does_not_panic() { x86_64::instructions::interrupts::int3(); }
}
