/// # Providing `-> !`
///
/// This function is just a nice abstraction of the call to `loop
/// {...}`, making it absolutely clear what the intend of calling this
/// function is, using its name.
///
/// We use the `hlt` instruction to "halt" the CPU to not burn through
/// CPU time, as a call to `loop {}` would do.
pub fn __never_return() -> !
{
	loop {
		x86_64::instructions::hlt();
	}
}

#[panic_handler]
pub fn __panic(panic_info: &core::panic::PanicInfo) -> ! { crate::panic::__panic(panic_info); }
