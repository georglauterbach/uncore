/// ## Global Descriptor Table Setup
///
/// This module handles the setup of the Global Descriptor Table (GDT)
/// and relates structures such as the Task State Segment (TSS) and
/// Interrupt Stack Table (IST).
pub(super) mod gdt
{
	use super::super::exceptions;

	use x86_64::{
		instructions::{
			tables,
			segmentation::{
				self,
				Segment,
			},
		},
		structures::{
			gdt::{
				Descriptor,
				GlobalDescriptorTable,
				SegmentSelector,
			},
			tss,
		},
	};

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
			tss.interrupt_stack_table[exceptions::DOUBLE_FAULT_IST_INDEX as usize] = {
				/// The size of the stack used during
				/// the CPU double fault exception.
				const STACK_SIZE: usize = 4096 * 5;

				/// Size-aligned representation of the stack used
				/// during the CPU double fault exception.
				#[repr(align(16))]
				struct Stack([u8; STACK_SIZE]);

				/// The stack representation of the actual stack
				/// used during the  CPU double fault exception.
				static mut STACK: Stack = Stack([0; STACK_SIZE]);

				// on x86_64 the stack grows downwards, therefore, the
				// "start" is the lowest address and we return the
				// "end" address which is the highest
				let stack_start = x86_64::VirtAddr::from_ptr(unsafe { &STACK });
				stack_start + STACK_SIZE
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
		static ref GDT: (GlobalDescriptorTable, Selectors) = {
			let mut gdt = GlobalDescriptorTable::new();

			let code_segment = gdt.add_entry(Descriptor::kernel_code_segment());
			let tss_segment = gdt.add_entry(Descriptor::tss_segment(&TSS));
			let stack_segment = SegmentSelector(0);

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
	/// used.
	struct Selectors
	{
		/// The Code Segment (`cs`) register selector
		code_segment:  SegmentSelector,
		/// The Stack Segment (`ss`) register selector
		stack_segment: SegmentSelector,
		/// The [`TSS`] selector
		tss_segment:   SegmentSelector,
	}

	/// ### Loading the GDT
	///
	/// The Global Descriptor Table (GDT) is loaded here.
	/// Furthermore, the Code Segment register (`cs`) is set, the
	/// Stack Segment register (`ss`) is loaded and the correct
	/// TSS is selected.
	pub(in super::super) fn init()
	{
		crate::log_trace!(
			"Initializing Global Descriptor Table (GDT) and Task State Segment (TSS)"
		);
		GDT.0.load();

		unsafe {
			segmentation::CS::set_reg(GDT.1.code_segment);
			segmentation::SS::set_reg(GDT.1.stack_segment);

			tables::load_tss(GDT.1.tss_segment);
		}
	}
}
