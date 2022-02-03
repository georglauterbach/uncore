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
const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// ### Generic Stacks
///
/// This module contains a struct which can represent a stack (with
/// proper alignment, etc.).
mod stack
{
	/// ### Just a Stack
	///
	/// The `PseudoStack` represent a proper stack that is
	/// allocated like a normal stack but as a stack object.
	#[repr(align(16))]
	pub(super) struct PseudoStack<'a>(pub &'a [u8]);
}

/// ## Global Descriptor Table Setup
///
/// This module handles the setup of the Global Descriptor Table (GDT)
/// and relates structures such as the Task State Segment (TSS) and
/// Interrupt Stack Table (IST).
pub(super) mod gdt
{
	use crate::prelude::*;
	use super::{
		stack,
		DOUBLE_FAULT_IST_INDEX,
	};
	use x86_64::{
		instructions::{
			tables,
			segmentation,
		},
		structures::{
			gdt,
			tss,
		},
	};

	/// ### Stack Size for Double Fault Handler
	///
	/// The size of the stack used during the CPU double fault
	/// exception.
	const DOUBLE_FAULT_STACK_SIZE: usize = 0x1000 * 5;

	lazy_static::lazy_static! {

		/// ### Task State Segment (TSS)
		///
		/// On x86_64, the TSS holds two stack tables (the IST is one of them).
		/// It is used to setup the IST to switch to a new table so exceptions
		/// can be handled properly on a new stack.
		static ref TSS: tss::TaskStateSegment = {
			let mut tss = tss::TaskStateSegment::new();

			// we now define the kernel stack to use when a double
			// fault exception occurs to prevent fatal triple fault
			// exceptions (e.g. due to hitting the guard page)
			tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
				/// ### The Double Fault Handler Stack Memory
				///
				/// This variable contains the memory for the CPU
				/// double fault exception handler stack.
				static mut STACK: stack::PseudoStack =
					stack::PseudoStack(&[0; DOUBLE_FAULT_STACK_SIZE]);

				// on `x86_64`, the stack grows downwards, therefore,
				// the "start" is the lowest address and we return
				// the "end" address which is the highest address
				let stack_start = x86_64::VirtAddr::from_ptr(unsafe { &STACK });
				stack_start + DOUBLE_FAULT_STACK_SIZE
			};

			tss
		};

		/// ### Global Descriptor Table (GDT)
		///
		/// The GDT is a relict that was used for memory segmentation before
		/// paging became the de facto standard. It is still needed in 64-bit
		/// mode for various things such as kernel/user mode configuration or
		/// TSS loading.
		///
		/// While segmentation is no longer supported in 64-bit mode, the GDT
		/// still exists. It is mostly used for two things: Switching between
		/// kernel space and user space, and loading a TSS structure.
		static ref GDT: (gdt::GlobalDescriptorTable, Selectors) = {
			let mut gdt = gdt::GlobalDescriptorTable::new();

			let code_segment = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
			let tss_segment = gdt.add_entry(gdt::Descriptor::tss_segment(&TSS));
			let stack_segment = gdt::SegmentSelector(0);

			(
				gdt,
				Selectors {
					code_segment,
					stack_segment,
					tss_segment,
				},
			)
		};
	}

	/// ### GDT Selectors
	///
	/// This struct holds the necessary selectors which need to
	/// be loaded. This makes sure the correct GDT and TSS are
	/// used by putting these values in the corresponding regis-
	/// ters.
	struct Selectors
	{
		/// The Code Segment (`cs`) register selector
		code_segment:  gdt::SegmentSelector,
		/// The Stack Segment (`ss`) register selector
		stack_segment: gdt::SegmentSelector,
		/// The [`TSS`] selector
		tss_segment:   gdt::SegmentSelector,
	}

	/// ### Loading the GDT
	///
	/// The Global Descriptor Table (GDT) is loaded here.
	/// Furthermore, the Code Segment register (`cs`) is set, the
	/// Stack Segment register (`ss`) is loaded and the correct
	/// TSS is selected.
	pub(in super::super) fn load()
	{
		use x86_64::instructions::segmentation::Segment;

		log_debug!("Loading Global Descriptor Table (GDT) and Task State Segment (TSS)");
		GDT.0.load();

		log_trace!("Setting registers for the GDT");
		unsafe {
			segmentation::CS::set_reg(GDT.1.code_segment);
			segmentation::SS::set_reg(GDT.1.stack_segment);
			tables::load_tss(GDT.1.tss_segment);
		}

		log_trace!("Finished GDT setup");
	}
}

/// ## Interrupt Descriptor Table Setup
///
/// This module initializes and loads the interrupt descriptor table,
/// hooking up CPU exception and interrupt handler callbacks.
pub(super) mod idt
{
	use crate::prelude::*;
	use x86_64::structures::idt;

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
		/// [2]: https://wiki.osdev.org/Exceptions
		static ref IDT: idt::InterruptDescriptorTable = {
			use super::super::exceptions;

			let mut idt = idt::InterruptDescriptorTable::new();

			idt.alignment_check.set_handler_fn(
				exceptions::handlers::alignment_check
			);

			idt.bound_range_exceeded.set_handler_fn(
				exceptions::handlers::bound_range_exceeded
			);

			idt.breakpoint.set_handler_fn(
				exceptions::handlers::breakpoint
			);

			idt.vmm_communication_exception.set_handler_fn(
				exceptions::handlers::vmm_communication
			);

			idt.debug.set_handler_fn(
				exceptions::handlers::debug
			);

			idt.device_not_available.set_handler_fn(
				exceptions::handlers::device_not_available
			);

			idt.divide_error.set_handler_fn(
				exceptions::handlers::divide_by_zero
			);

			idt.general_protection_fault.set_handler_fn(
				exceptions::handlers::general_protection_fault
			);

			unsafe {
				idt.double_fault
					.set_handler_fn(exceptions::handlers::double_fault)
					.set_stack_index(super::DOUBLE_FAULT_IST_INDEX);
			}

			idt.invalid_tss.set_handler_fn(
				exceptions::handlers::invalid_tss
			);

			idt.invalid_opcode.set_handler_fn(
				exceptions::handlers::invalid_opcode
			);

			idt.machine_check.set_handler_fn(
				exceptions::handlers::machine_check
			);

			idt.non_maskable_interrupt.set_handler_fn(
				exceptions::handlers::non_maskable_interrupt
			);

			idt.overflow.set_handler_fn(
				exceptions::handlers::overflow
			);

			idt.page_fault.set_handler_fn(
				exceptions::handlers::page_fault
			);

			idt.security_exception.set_handler_fn(
				exceptions::handlers::security
			);

			idt.segment_not_present.set_handler_fn(
				exceptions::handlers::segment_not_present
			);

			idt.simd_floating_point.set_handler_fn(
				exceptions::handlers::simd_floating_point
			);

			idt.stack_segment_fault.set_handler_fn(
				exceptions::handlers::stack_segment_fault
			);

			idt.virtualization.set_handler_fn(
				exceptions::handlers::virtualization
			);

			idt.x87_floating_point.set_handler_fn(
				exceptions::handlers::x87_floating_point
			);

			idt
		};
	}

	/// ### Loading the IDT
	///
	/// The Interrupt Stack Table is loaded here.
	pub(in super::super) fn load()
	{
		log_debug!("Loading Interrupt Descriptor Table (IDT)");
		IDT.load();
		log_trace!("Finished IDT setup");
	}
}
