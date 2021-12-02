use super::{
	cpu,
	io,
};

/// # Hardware Initialization
///
/// This method wraps all initialization in the `hw` (hardware) module.
///
/// ## Caller
///
/// It is called in the global `init()` function found in `crate::lib.rs`.
///
/// ## Callees
///
/// The following structures are initialized
///
/// 1. Global Descriptor Table (GDT)
/// 2. Interrupt Descriptor Table (IDT)
/// 3. Process Interrupt Controllers (PIC)
pub fn run()
{
	cpu::gdt::init();
	cpu::idt::init();

	io::interrupts::init_pics();
}
