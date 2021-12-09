/// # CPU Exceptions & Interrupt Handlers
///
/// This module contains all CPU exception handlers which
/// can be registered in the IDT.
mod exceptions_handlers;

/// # Global Descriptor Table
///
/// This modules provides the integration for the Global
/// Descriptor Table (GDT).
pub mod gdt;

/// # Interrupt Descriptor Table
///
/// The Interrupt Descriptor Table (IDT) is initialized
/// here.
pub mod idt;

/// # Task State Segment
///
/// The Task State Segment (TSS) is initialized here.
mod tss;
