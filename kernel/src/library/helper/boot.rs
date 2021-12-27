/// ### Architecture Abstractions for Bootloader Information
///
/// This enum is used to abstract over the different types given
/// by different bootloaders of different architectures.
///
/// For this enum, there is a `core::convert::From<BootInformation>`
/// implementation in the `hardware` module under
/// `library::hardware::architectures::boot.rs`.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct BootInformation
{
	/// The `x86_64` bootloader information given by the
	/// bootloader provided by _Phillip Oppermann_.
	#[cfg(target_arch = "x86_64")]
	pub bootloader_information: &'static mut bootloader::BootInfo,
}
