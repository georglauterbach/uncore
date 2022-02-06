// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Virtual Memory
///
/// This module handles (demand) paging, that is virtual memory, for
/// the kernel (and for the user-space in the future).
mod virtual_memory;

pub(crate) use virtual_memory::PageAllocation;
pub(crate) use virtual_memory::FrameAllocation;

/// ## A Simple Allocator
///
/// This module provides a _very_ simple and minimalistic allocator
/// _only_ used in the kernel for simple tasks.
mod kernel_heap_allocator;

use crate::prelude::*;

/// ### Default Page Size
///
/// The default page size is 4096 (= 0x1000) byte.
pub const PAGE_SIZE_DEFAULT: usize = 0x1000;

/// ### (Temporary) Kernel Heap Start
///
/// This value marks the temporary virtual start address of the kernel heap. **In the
/// future, a proper paging implementation will render this obsolete!**
pub const KERNEL_HEAP_START: usize = 0x0000_4444_4444_0000;

/// ### (Temporary) Kernel Heap Size
///
/// The size of the kernel heap. **In the future, a proper paging implementation will
/// render this obsolete!** The size of the kernel heap equals [`PAGE_SIZE_DEFAULT`]
/// (a.k.a. 4096 Byte) times 100, i.e. 800 KiB.
pub const KERNEL_HEAP_SIZE: usize = 200 * PAGE_SIZE_DEFAULT;

/// ### Initialize Kernel Memory
///
/// This function takes care of initializing
///
/// 1. virtual memory (demand paging, setting up a page table, etc.)
/// 2. a kernel heap allocator
///
/// all while abstracting over all the different architectures.
pub fn initialize(boot_information: &boot::Information)
{
	log_info!("Starting memory initialization");

	virtual_memory::initialize(boot_information);
	kernel_heap_allocator::initialize();

	log_info!("Finished memory initialization");
}

#[test_case]
fn boxing_does_not_panic()
{
	use alloc::boxed;

	log_debug!("Trying to box a value");

	let heap_value_1 = boxed::Box::new(41);
	let heap_value_2 = boxed::Box::new(13);
	assert_eq!(*heap_value_1, 41);
	assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vector()
{
	let vector_size = 1000;
	let mut vec = alloc::vec::Vec::new();

	for i in 0..vector_size {
		vec.push(i);
	}

	assert_eq!(vec.iter().sum::<u64>(), (vector_size - 1) * vector_size / 2);
}

#[test_case]
fn many_boxes()
{
	for i in 0..crate::prelude::memory::KERNEL_HEAP_SIZE {
		let x = alloc::boxed::Box::new(i);
		assert_eq!(*x, i);
	}
}
