// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## A (Simple) Kernel Heap Allocator
///
/// This module provides a _very_ simple and minimalistic allocator
/// _only_ used in the kernel for simple tasks.
mod heap;

/// ## Physical Memory
///
/// Holds structures and functions needed when interacting with physical addresses.
mod physical;

pub use physical::{
	Frame,
	FrameAllocation,
};
pub(crate) use physical::get_frame_allocator;

/// ## Virtual Memory
///
/// This module handles (demand) paging, that is virtual memory, for
/// the kernel (and for the user-space in the future).
mod virtual_;

pub use virtual_::{
	addresses::{
		PhysicalAddress,
		VirtualAddress,
	},
	chunks::{
		ChunkSize,
		ChunkSizeDefault,
		ChunkSizeHuge,
		ChunkSizeGiant,
	},
	paging,
};

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

	let (kernel_page_table, kernel_frame_allocator) = architecture_memory::initialize(boot_information.0);
	unsafe {
		physical::KERNEL_FRAME_ALLOCATOR
			.call_once(|| architecture_memory::FrameAllocator::new(kernel_frame_allocator));

		virtual_::KERNEL_PAGE_TABLE
			.lock()
			.call_once(|| architecture_memory::PageTable::new(kernel_page_table));
	}

	log_debug!("Finished initializing virtual memory");
	log_debug!("Initializing a simple global memory allocator");

	unsafe {
		if let Err(error) = heap::ALLOCATOR.lock().initialize() {
			log_error!("Failed to initialize the kernel heap (error: {:?}", error);
			exit_kernel(kernel_types::ExitCode::Failure);
		};
	}

	log_debug!("Initialized allocator");
	log_info!("Finished memory initialization");
}
