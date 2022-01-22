// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

use crate::library;

/// ## Virtual Memory
///
/// This module handles (demand) paging, that is virtual memory, for
/// the kernel (and for the user-space in the future).
mod virtual_memory;

/// ## A Simple Allocator
///
/// This module provides a _very_ simple and minimalistic allocator
/// _only_ used in the kernel for simple tasks.
mod kernel_heap_allocator;

/// ### Initialize Kernel Memory
///
/// This function takes care of initializing
///
/// 1. virtual memory (demand paging, setting up a page table, etc.)
/// 2. a kernel heap allocator
///
/// all while abstracting over all the different architectures.
pub fn initialize(uefi_memory_map: library::boot::UEFIMemoryMap)
{
	use crate::prelude::*;
	log_info!("Starting memory initialization");

	virtual_memory::initialize(uefi_memory_map);
	kernel_heap_allocator::_initialize();

	log_error!("Reached an unfinished state here");
}

#[test_case]
fn boxing_does_not_panic()
{
	use crate::prelude::*;

	log_debug!("Trying to box a value");
	let x = alloc::boxed::Box::new(42);
	log_debug!("The boxed value reads {:?}", x);
}
