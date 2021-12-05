/// Abstracts over the exit code a number, proving a clear
/// indication of success or failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode
{
	Success = 0x10,
	Failed  = 0x11,
}

/// Looks up `[package.metadata.bootimage]` in `Cargo.toml` to
/// use the `isa-debug-exit` device, located on port `0xf4`
/// with a 4 byte size.
pub fn exit(exit_code: ExitCode)
{
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xF4);
		port.write(exit_code as u32);
	}
}
