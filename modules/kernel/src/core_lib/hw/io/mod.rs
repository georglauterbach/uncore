/// # A Serial Device Interface
///
/// The `write` module makes heavy use of this module
/// as it `serial` provides the ability to write to
/// a serial device which is "forwarded" to `stdout`
/// via QEMU.
pub mod serial;
/// # Printing Information
///
/// This module enabled the printing of information
/// to a screen. Here, a VGA buffer is used, which
/// is displayed in `stdout` in the terminal through
/// a serial device in `QEMU`.
///
/// ## Future
///
/// This module will be re-written soon. Therefore, the
/// documentation is not of good quality.
pub mod write;

/// # Hardware Interrupts
///
/// This module handles hardware interrupts and abstracts over the hardware
/// interrupt controller.
pub mod interrupts;
