// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use multiboot2::BootInformation;

const MULTIBOOT2_MAGIC: u32 = 0x36d76289;

// pub static MULTIBOOT2_INFO_STRUCTURE: spin::Mutex<Option<BootInformation>> = spin::Mutex::new(None);

pub fn handle_uefi(
	_multiboot2_magic_value: u32,
	multiboot2_boot_information_pointer: u32,
)
{
	let mb2_boot_info: multiboot2::BootInformation =
            unsafe { multiboot2::load(multiboot2_boot_information_pointer as usize) }.expect("Couldn't load MBI");

	    
}
