/// ### The Event Horizon
///
/// This function is just a nice abstraction of the call to `loop
/// {...}`, making it absolutely clear what the intend of calling this
/// function is, using its name.
///
/// We use the `hlt` instruction to "halt" the CPU to not burn through
/// CPU time, as a call to `loop {}` would do.
#[inline]
pub fn never_return() -> !
{
	loop {
		x86_64::instructions::hlt();
	}
}

/// ## QEMU Abstractions
///
/// Contains helpers for running the kernel with QEMU.
pub(crate) mod qemu
{
	/// ### Write An Exit Code
	///
	/// Looks up `[package.metadata.bootimage]` in `Cargo.toml` to
	/// use the `isa-debug-exit` device, located on port `0xf4`
	/// with a four byte size.
	fn exit(exit_code: u32)
	{
		use x86_64::instructions::port::Port;

		unsafe {
			let mut port = Port::new(0xF4);
			port.write(exit_code);
		}
	}

	/// ### Exit QEMU With Success
	///
	/// Write a success exit code for QEMU to recognize and exit.
	#[allow(dead_code)]
	#[inline]
	pub fn exit_with_success() { exit(0x10); }

	/// ### Exit QEMU Without Success
	///
	/// Write a failure exit code for QEMU to recognize and exit.
	#[allow(dead_code)]
	#[inline]
	pub fn exit_with_failure() { exit(0x11); }
}
