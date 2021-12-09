use lazy_static::lazy_static;
use x86_64::{
	instructions,
	structures::gdt::{
		Descriptor,
		GlobalDescriptorTable,
		SegmentSelector,
	},
};

/// # Double Fault Interrupt Stack Table Index
///
/// This constant defines the stack to use in the
/// `interrupt_stack_table` field on the TSS for the
/// double fault handler. The first index is chosen.
///
/// The `interrupt_stack_table` is a field in the `Task
/// State Segment` struct. It can be used to switch kernel
/// stacks.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
	static ref GDT: (GlobalDescriptorTable, Selectors) = {
		let mut gdt = GlobalDescriptorTable::new();

		let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
		let tss_selector = gdt.add_entry(Descriptor::tss_segment(&super::tss::TSS));

		(
			gdt,
			Selectors {
				code_selector,
				tss_selector,
			},
		)
	};
}

/// # GDT Selectors
///
/// This struct holds the necessary selectors which need to
/// be loaded. This makes sure the correct GDT and TSS are
/// used.
struct Selectors
{
	/// The Code Segment (`cs`) register selector
	code_selector: SegmentSelector,
	/// The TSS selector
	tss_selector:  SegmentSelector,
}

/// # Loading the GDT
///
/// The Global Descriptor Table (GDT) is loaded here.
/// Furthermore, the Code Segment register (`cs`) is
/// (re-)loaded and the correct TSS is selected.
pub fn init()
{
	GDT.0.load();

	unsafe {
		// instructions::segmentation::set_cs(GDT.1.code_selector);
		instructions::tables::load_tss(GDT.1.tss_selector);
	}
}
