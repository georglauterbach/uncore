/// ### Kernel Version
///
/// The `VERSION` variable contains the kernel version in the semantic
/// versioning format, the git commit id the kernel was built with and
/// the build date. If `VERSION` was not available during build-time,
/// a default value is provided, namely "testing".
const VERSION: Option<&str> = option_env!("VERSION");

/// ### The Kernel Target
///
/// The kernel target is a triple consisting of
///
/// 1. The hardware architecture
/// 2. The vendor (manufacturer) (optional and omitted in our case)
/// 3. Operating system
/// 4. ABI
///
/// The target triple reads as `ARCH-VENDOR-SYS-ABI` and you can read
/// about it [here](https://docs.rust-embedded.org/embedonomicon/custom-target.html).
///
/// The default case for `unCORE` is `x86_64-unknown-none`. This
/// target is for freestanding / bare-metal `x86-64` binaries in ELF
/// format, i.e. firmware, kernels, etc.
const TARGET: Option<&str> = option_env!("KERNEL_BUILD_TARGET");

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

/// ### Print Important Initial Information
///
/// We print out information about
///
/// - the kernel version
/// - the bootloader
///
/// on the serial interface with this function.
pub(in super::super) fn display_initial_information(boot_information: &bootloader::BootInfo)
{
	crate::log!("This is unCORE {}\n", VERSION.unwrap_or("testing"));
	crate::log_info!(
		"Target triple reads '{}'",
		TARGET.unwrap_or("x86_64-unknown-none (defaulted)")
	);
	crate::log_info!(
		"Bootloader version {}.{}.{}",
		boot_information.version_major,
		boot_information.version_minor,
		boot_information.version_patch
	);

	crate::log_trace!(
		"Printing boot information structure \n\n{:#?}\n\n",
		boot_information
	);
}

/// ## QEMU Abstractions
///
/// Contains helpers for running the kernel with QEMU.
pub mod qemu
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
