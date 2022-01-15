// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

/// ## CPU Exception Callbacks
///
/// This module contains the callback functions executed when a CPU
/// exception is thrown.
mod exceptions;

/// ## General CPU Initialization
///
/// This module contains code for general setup code, such as Global
/// Descriptor Table (GDT) setup.
mod general;

/// ## CPU Interrupt Callbacks
///
/// This module contains the callback functions executed when CPU
/// interrupts arrive.
mod interrupts;

/// ### Initialize `x86_64`
///
/// This function initializes interrupts, exceptions, GDT, IDT and
/// more.
pub fn initialize()
{
	crate::prelude::log_debug!("Initializing CPU");

	general::gdt::load();
	general::idt::load();

	interrupts::setup_and_enable();
}