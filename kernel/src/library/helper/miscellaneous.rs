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
/// 2. The vendor (manufacturer) (optional)
/// 3. Operating system
/// 4. ABI (omitted in our case)
///
/// The target triple reads as `ARCH-VENDOR-SYS-ABI` and you can read
/// about it [here](https://docs.rust-embedded.org/embedonomicon/custom-target.html).
///
/// The default case for `unCORE` is `x86_64-unknown-none`. This
/// target is for freestanding / bare-metal `x86-64` binaries in ELF
/// format, i.e. firmware, kernels, etc.
const TARGET: Option<&str> = option_env!("KERNEL_BUILD_TARGET");

/// ### Kernel `main()` Function
///
/// This function is the architecture independent entrypoint for the
/// kernel.
///
/// This function initializes the whole kernel. It takes care of
///
/// - printing important initial information
/// - calling the hardware initialization subroutine
pub fn main(boot_information: &super::BootInformation) -> !
{
	use super::super::{
		hardware,
		log,
		memory,
	};

	log::set_log_level(log::Level::Trace);
	display_initial_information(boot_information);

	crate::log_info!("Kernel initialization started");

	hardware::init();
	memory::init(boot_information);

	crate::log_info!("Kernel initialization finished");

	// It is fine to have this here. If unit tests are run for `lib.rs`,
	// this will make sure all is initialized beforehand. This does not
	// affect `main.rs` since we use `crate::` which refers to `lib.rs`
	// and is therefore not called when running unit tests for `main.rs`.
	#[cfg(test)]
	crate::__test_runner();

	never_return()
}

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
		#[cfg(target_arch = "x86_64")]
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
pub(super) fn display_initial_information(boot_information: &super::BootInformation)
{
	crate::log!("This is unCORE {}\n", VERSION.unwrap_or("testing"));
	crate::log_info!(
		"Target triple reads '{}'",
		TARGET.unwrap_or("x86_64-unknown-none (defaulted)")
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
