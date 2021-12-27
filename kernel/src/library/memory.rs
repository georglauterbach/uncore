/// ### Generic Memory Initialization
///
/// This function is called to initialize virtual memory correctly. It
/// uses the `init` function provided by the `hardware` module to use
/// conditional compilation to initialize the memory correctly for
/// each target platform.
pub fn init(boot_information: &crate::library::BootInformation)
{
	crate::log_info!("Initializing virtual memory");
	crate::library::hardware::memory::init(boot_information.bootloader_information);
}
