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

	/// ### CPU Exception - Alignment Check
	///
	/// This is the handler callback function for the alignment
	/// check CPU Exception.
	pub extern "x86-interrupt" fn alignment_check(stack_frame: idt::InterruptStackFrame, error_code: u64)
	{
		log_warning!(
			"CPU exception occurred (type: alignment check)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame
		);
	}

	/// ### CPU Exception - Bound Range Exceeded
	///
	/// This is the handler callback function for the bound range
	/// exceeded CPU Exception.
	pub extern "x86-interrupt" fn bound_range_exceeded(stack_frame: idt::InterruptStackFrame)
	{
		log_error!(
			"CPU exception occurred (type: bound range exceeded)\n\n{:#?}\n",
			stack_frame
		);

		panic!("fatal 'bound range exceeded' CPU exception occurred");
	}

	/// ### CPU Exception - Breakpoint Handler
	///
	/// This is the handler callback function for the Breakpoint
	/// CPU Exception.
	pub extern "x86-interrupt" fn breakpoint(_stack_frame: idt::InterruptStackFrame)
	{
		log_info!("CPU exception occurred (type: breakpoint)");
	}

	/// ### CPU Exception - Bound Range Exceeded
	///
	/// This is the handler callback function for the bound range
	/// exceeded CPU Exception.
	pub extern "x86-interrupt" fn debug(stack_frame: idt::InterruptStackFrame)
	{
		log_info!("CPU exception occurred (type: debug)\n\n{:#?}\n", stack_frame);
	}

	/// ### CPU Exception - Device Not Available
	///
	/// This is the handler callback function for the device not
	/// available CPU Exception.
	pub extern "x86-interrupt" fn device_not_available(stack_frame: idt::InterruptStackFrame)
	{
		log_warning!(
			"CPU exception occurred (type: device not available)\n\n{:#?}\n",
			stack_frame
		);
	}

	/// ### CPU Exception - Divide by Zero
	///
	/// This is the handler callback function for the divide by
	/// zero CPU Exception.
	pub extern "x86-interrupt" fn divide_by_zero(stack_frame: idt::InterruptStackFrame)
	{
		log_error!(
			"CPU exception occurred (type: divide by zero)\n\n{:#?}\n",
			stack_frame,
		);

		panic!("fatal 'divide by zero' CPU exception occurred");
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
			"CPU exception occurred (type: double fault)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame,
		);

		panic!("fatal 'double fault' CPU exception occurred");
	}

	/// ### CPU Exception - General Protection Fault
	///
	/// This is the handler callback function for the general
	/// protection fault CPU Exception.
	pub extern "x86-interrupt" fn general_protection_fault(
		stack_frame: idt::InterruptStackFrame,
		error_code: u64,
	)
	{
		log_error!(
			"CPU exception occurred (type: general protection fault)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame
		);

		panic!("fatal 'general protection fault' CPU exception occurred");
	}

	/// ### CPU Exception - Invalid Opcode
	///
	/// This is the handler callback function for the invalid
	/// opcode CPU Exception.
	pub extern "x86-interrupt" fn invalid_opcode(stack_frame: idt::InterruptStackFrame)
	{
		log_error!(
			"CPU exception occurred (type: invalid opcode)\n\n{:#?}\n",
			stack_frame
		);

		panic!("fatal 'invalid opcode' CPU exception occurred");
	}

	/// ### CPU Exception - Invalid TSS
	///
	/// This is the handler callback function for the invalid TSS
	/// CPU Exception.
	pub extern "x86-interrupt" fn invalid_tss(stack_frame: idt::InterruptStackFrame, error_code: u64)
	{
		log_error!(
			"CPU exception occurred (type: invalid TSS)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame,
		);

		panic!("fatal 'invalid TSS' CPU exception occurred");
	}

	/// ### CPU Exception - Machine Check
	///
	/// This is the handler callback function for the machine
	/// check CPU Exception.
	pub extern "x86-interrupt" fn machine_check(stack_frame: idt::InterruptStackFrame) -> !
	{
		log_error!(
			"CPU exception occurred (type: machine check)\n\n{:#?}\n",
			stack_frame
		);

		panic!("fatal 'machine check' CPU exception occurred")
	}

	/// ### CPU Exception - Non-Maskable Interrupt
	///
	/// This is the handler callback function for the non-maskable
	/// interrupt CPU Exception.
	pub extern "x86-interrupt" fn non_maskable_interrupt(stack_frame: idt::InterruptStackFrame)
	{
		log_warning!(
			"CPU exception occurred (type: non-maskable interrupt)\n\n{:#?}\n",
			stack_frame
		);
	}

	/// ### CPU Exception - Overflow
	///
	/// This is the handler callback function for the overflow CPU
	/// Exception.
	pub extern "x86-interrupt" fn overflow(stack_frame: idt::InterruptStackFrame)
	{
		log_warning!("CPU exception occurred (type: overflow)\n\n{:#?}\n", stack_frame);
	}

	/// ### CPU Exception - Page Fault Handler
	///
	/// This is the handler callback function for the page fault
	/// CPU exception.
	pub extern "x86-interrupt" fn page_fault(
		_stack_frame: idt::InterruptStackFrame,
		error_code: idt::PageFaultErrorCode,
	)
	{
		log_debug!("CPU exception occurred (type: page fault)");
		log_trace!(
			"page fault information: accessed address = {:?} | error code = {:?}\n",
			x86_64::registers::control::Cr2::read(),
			error_code
		);
		unimplemented!();
	}

	/// ### CPU Exception - Security
	///
	/// This is the handler callback function for the security CPU
	/// Exception.
	pub extern "x86-interrupt" fn security(stack_frame: idt::InterruptStackFrame, error_code: u64)
	{
		log_error!(
			"CPU exception occurred (type: security)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame,
		);

		panic!("fatal 'security' CPU exception occurred");
	}

	/// ### CPU Exception - Segment not Present
	///
	/// This is the handler callback function for the segment not
	/// present CPU Exception.
	pub extern "x86-interrupt" fn segment_not_present(
		stack_frame: idt::InterruptStackFrame,
		error_code: u64,
	)
	{
		log_error!(
			"CPU exception occurred (type: segment not present)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame,
		);

		panic!("fatal 'segment not present' CPU exception occurred");
	}

	/// ### CPU Exception - SIMD Floating Point
	///
	/// This is the handler callback function for the SIMD
	/// floating point CPU Exception.
	pub extern "x86-interrupt" fn simd_floating_point(stack_frame: idt::InterruptStackFrame)
	{
		log_error!(
			"CPU exception occurred (type: SIMD floating point exception)\n\n{:#?}\n",
			stack_frame,
		);

		panic!("fatal SIMD floating point CPU exception");
	}

	/// ### CPU Exception - Stack Segment Fault
	///
	/// This is the handler callback function for the stack
	/// segment fault Exception.
	pub extern "x86-interrupt" fn stack_segment_fault(
		stack_frame: idt::InterruptStackFrame,
		error_code: u64,
	)
	{
		log_error!(
			"CPU exception occurred (type: stack segment fault)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame,
		);

		panic!("fatal 'stack segment fault' CPU exception occurred");
	}

	/// ### CPU Exception - Virtualization
	///
	/// This is the handler callback function for the machine
	/// check CPU Exception.
	pub extern "x86-interrupt" fn virtualization(stack_frame: idt::InterruptStackFrame)
	{
		log_error!(
			"CPU exception occurred (type: virtualization)\n\n{:#?}\n",
			stack_frame,
		);

		panic!("virtualization");
	}

	/// ### CPU Exception - VMM Communication
	///
	/// This is the handler callback function for the VMM
	/// communication CPU Exception.
	pub extern "x86-interrupt" fn vmm_communication(
		stack_frame: idt::InterruptStackFrame,
		error_code: u64,
	)
	{
		log_error!(
			"CPU exception occurred (type: VMM communication)\n\nError code: {}\n{:#?}\n",
			error_code,
			stack_frame
		);

		panic!("fatal 'VMM communication' CPU exception occurred");
	}

	/// ### CPU Exception - x87 Floating Point
	///
	/// This is the handler callback function for the x87 floating
	/// point CPU Exception.
	pub extern "x86-interrupt" fn x87_floating_point(stack_frame: idt::InterruptStackFrame)
	{
		log_error!(
			"CPU exception occurred (type: x87 floating point exception)\n\n{:#?}\n",
			stack_frame,
		);

		panic!("fatal 'x87 floating point' CPU exception occurred");
	}

	#[test_case]
	fn breakpoint_exception_does_not_panic() { x86_64::instructions::interrupts::int3(); }
}
