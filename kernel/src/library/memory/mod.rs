// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## A (Simple) Kernel Heap Allocator
///
/// This module provides a _very_ simple and minimalistic allocator
/// _only_ used in the kernel for simple tasks.
pub mod heap;

/// ## Physical Memory
///
/// Holds structures and functions needed when interacting with physical addresses.
pub mod physical_memory;

/// ## Virtual Memory
///
/// This module handles (demand) paging, that is virtual memory, for
/// the kernel (and for the user-space in the future).
pub mod virtual_memory;

use crate::prelude::*;

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
	use crate::library::architectures::memory as architecture_memory;

	log_info!("Starting memory initialization");

	log_debug!("Initializing virtual memory");
	let (mut kernel_page_table, kernel_frame_allocator) =
		architecture_memory::initialize(boot_information.0);

	unsafe {
		physical_memory::KERNEL_FRAME_ALLOCATOR.call_once(|| {
			architecture_memory::physical_memory::FrameAllocator::new(kernel_frame_allocator)
		});
	}
	log_debug!("Finished initializing virtual memory");

	architecture_memory::initialize_kernel_heap(&mut kernel_page_table);

	heap::initialize();

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
	for i in 0..heap::KERNEL_HEAP_SIZE {
		let x = alloc::boxed::Box::new(i);
		assert_eq!(*x, i);
	}
}
