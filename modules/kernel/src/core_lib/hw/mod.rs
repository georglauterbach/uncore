/// # Central Processing Unit Interaction
///
/// This module handles CPU interaction. This includes CPU exceptions, fault
/// handling or setup of descriptor tables.
pub mod cpu;

/// # Hardware Initialization
///
/// This module's sole propose is to provide the `run()` function
/// as the main initialization wrapper for all hardware components.
pub mod init;

/// # I/O Device Management
///
/// This modules handles input and output devices and their interrupts.
///
/// ## Macros
///
/// The macros for `print!()` and `println!()` are provided in `io::write`.
pub mod io;
