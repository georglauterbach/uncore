use lazy_static::lazy_static;
use x86_64::{
	structures::tss::TaskStateSegment,
	VirtAddr,
};

use super::gdt;

lazy_static! {
	pub static ref TSS: TaskStateSegment = {
		let mut tss = TaskStateSegment::new();

		// we now define the kernel stack to use when a double
		// fault exception occurs to prevent fatal triple fault
		// exceptions (e.g. due to hitting the guard page)
		tss.interrupt_stack_table[gdt::DOUBLE_FAULT_IST_INDEX as usize] = {
			const STACK_SIZE: usize = 4096 * 5;
			// the mut is important, as the bootloader would otherwise
			// map this to a read-only page
			static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

			// on x86_64 the stack grows downwards, therefore, the
			// "start" is the lowest address and we return the "end"
			// address which is the highest
			let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
			stack_start + STACK_SIZE // = "end"
		};
		tss
	};
}
