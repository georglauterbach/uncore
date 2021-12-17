pub fn init(boot_information: &bootloader::BootInfo)
{
	crate::log_info!("Initializing virtual memory");
	super::architectures::memory::init(boot_information);
}
