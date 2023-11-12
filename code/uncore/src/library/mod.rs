// SPDX-License-Identifier: GPL-3.0-or-later

//! `unCORE` library module file that contains all other modules.

pub mod arch;
mod heap;
pub mod log;
pub mod prelude;
pub mod test;

/// `unCORE`'s panic handler. This panic handler does not implement stack-unwinding;
/// instead, it terminates execution by exiting the kernel (with [`arch::exit_kernel`]). A
/// log message is provided to indicate the reasons for the call to [`panic!`].
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  if let Some(location) = info.location() {
    info.message().map_or_else(
      || {
        ::log::error!(
          "thread 'X' panicked at {}:{}: no message provided",
          location.file(),
          location.line()
        );
      },
      |message| {
        ::log::error!(
          "thread 'X' panicked at {}:{}: {:?}",
          location.file(),
          location.line(),
          message
        );
      },
    );
  }

  arch::exit_kernel(crate::UncoreResult::Err);
}
