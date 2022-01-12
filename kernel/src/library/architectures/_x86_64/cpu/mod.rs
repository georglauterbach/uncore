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

/// ### Initialize the CPU
///
/// This function initializes the CPU.
#[allow(dead_code)]
pub(super) fn initialize()
{
	general::gdt::init();
	interrupts::init();
}
