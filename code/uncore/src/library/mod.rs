// SPDX-License-Identifier: GPL-3.0-or-later

//! `unCORE` library module file that contains all other modules.

pub mod arch;
pub mod mem;
pub mod log;
pub mod prelude;
pub mod test;

/// `unCORE`'s panic handler. This panic handler does not implement stack-unwinding;
/// instead, it terminates execution by exiting the kernel (with [`arch::exit_kernel`]). A
/// log message is provided to indicate the reasons for the call to [`panic!`].
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  if let Some(location) = info.location() {
    let message = info.message().as_str().unwrap_or("no message available");
    ::log::error!(
      "Panicked in file {}, line {}, column {}: {message}",
      location.file(),
      location.line(),
      location.column()
    );
  } else {
    ::log::error!("Panic without location information - you are out of luck!");
  }

  arch::exit_kernel(crate::UncoreResult::Err);
}
