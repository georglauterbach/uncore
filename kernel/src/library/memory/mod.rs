// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## Virtual Memory
///
/// This module handles (demand) paging, that is virtual memory, for
/// the kernel (and for the user-space in the future).
mod paging;

/// ## A Simple Static Allocator
///
/// This module provides a _very_ simple and minimalistic allocator
/// _only_ used in the kernel for simple tasks. It uses a
/// pre-allocated array.
mod simple_static;

// TODO copy and run the test from Phillip as well

/// TODO
/// 
pub fn initialize()
{
	use crate::prelude::*;
	log_info!("Starting memory initialization");

	// https://github.com/rust-osdev/bootloader/blob/main/src/bin/uefi.rs#L37

	simple_static::initialize();

	log_info!("Finished memory initialization");
}

#[test_case]
fn boxing_does_not_panic()
{
	use crate::prelude::*;

	log_debug!("Trying to box a value");
	let x = alloc::boxed::Box::new(42);
	log_debug!("The boxed value reads {:?}", x);
}
