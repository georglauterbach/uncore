#![no_std]
#![no_main]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![feature(custom_test_frameworks)]
#![test_runner(helper::test::test_runner)]
#![reexport_test_harness_main = "__start_tests"]
#![feature(abi_x86_interrupt)]

/// # Hardware Abstractions for Different Architectures
///
/// The `library` module contains initialization routines as well as
/// access routines for different architectures. It is conditionally
/// compiled, depending on the architecture chosen for compilation.
/// Currently, only our special "none" architecture for use with QEMU
/// is really supported.
mod library;

/// # Hardware Initialization
///
/// This method wraps all initialization in the hardware module.
///
/// ## Caller
///
/// It is called during the initialization of the kernel, as the first
/// subroutine. In the `kernel` module, the caller is also called
/// `init()`, located under `library/mod.rs`.
///
/// ## Callees
///
/// The following structures are initialized
///
/// 1. Global Descriptor Table (GDT)
/// 2. Interrupt Descriptor Table (IDT)
/// 3. Process Interrupt Controllers (PIC)
#[inline]
pub fn init() { library::init(); }
