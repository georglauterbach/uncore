use crate::println;
use x86_64::structures::idt::InterruptStackFrame;

/// # Printing Exception Information
///
/// This function provides an interface for all CPU exception handlers to
/// uniformly print information about the exception that happened.
fn print_information(
	exception_type: &str,
	abort_through_panic: bool,
	stack_frame: &mut InterruptStackFrame,
)
{
	println!("\n{{{{ CPU EXCEPTION }}}}\n");
	println!("  :: EXCEPTION TYPE      = {}", exception_type);
	println!("  :: ABORT THROUGH PANIC = {}", abort_through_panic);
	println!("  :: STACK FRAME STATE   = \n\n{:#?}", stack_frame);
}

/// # CPU Exception - Double Fault Handler
///
/// This is the handler for the Double Fault
/// CPU Exception
///
/// ## Trivia
///
/// One difference to the breakpoint handler is that the
/// double fault handler is diverging. The reason is that the `x86_64`
/// architecture does not permit returning from a double fault exception.
pub extern "x86-interrupt" fn double_fault_handler(
	stack_frame: &mut InterruptStackFrame,
	_error_code: u64,
) -> !
{
	print_information("DOUBLE_FAULT", true, stack_frame);

	panic!("[FATAL] DOUBLE FAULT");
}

/// # CPU Exception - Breakpoint Handler
///
/// This is the handler for the Breakpoint CPU Exception.
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame)
{
	print_information("BREAKPOINT", false, stack_frame);
}

#[test_case]
fn test_breakpoint_exception() { x86_64::instructions::interrupts::int3(); }
