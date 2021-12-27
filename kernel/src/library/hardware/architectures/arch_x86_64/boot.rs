impl core::convert::From<&'static mut bootloader::BootInfo> for crate::library::BootInformation
{
	fn from(bootloader_information: &'static mut bootloader::BootInfo) -> Self
	{
		Self {
			bootloader_information,
		}
	}
}
