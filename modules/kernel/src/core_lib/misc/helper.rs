/// # Providing `-> !`
///
/// This function is just a nice abstraction of
/// the call to `loop {...}`, making it absolutely
/// clear what the intend of calling this function
/// is, using its name.
///
/// We use the `hlt` instruction to "halt" the CPU to not burn through CPU
/// time, as a call to `loop {}` would do.
pub fn __never_return() -> !
{
	loop {
		x86_64::instructions::hlt();
	}
}

/// # QEMU
///
/// This module provides all necessary interaction
/// for QEMU.
pub mod qemu
{
	/// Abstracts over the exit code a number,
	/// proving a clear indication of success
	/// or failure.
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	#[repr(u32)]
	pub enum ExitCode
	{
		Success = 0x10,
		Failed  = 0x11,
	}

	/// Looks up `[package.metadata.bootimage]` in `Cargo.toml`
	/// to use the `isa-debug-exit` device, located on port
	/// `0xf4` with a 4 byte size.
	pub fn _exit(exit_code: ExitCode)
	{
		use x86_64::instructions::port::Port;

		unsafe {
			let mut port = Port::new(0xF4);
			port.write(exit_code as u32);
		}
	}
}
