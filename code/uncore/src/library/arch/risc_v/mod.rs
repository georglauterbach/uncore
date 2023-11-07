// SPDX-License-Identifier: GPL-3.0-or-later

pub mod drivers;
mod ld;

/// Architecture-specific functionality before the actual main function [`crate::main`]
/// can run.
pub fn main() -> ! {
  drivers::initialize();
  crate::main();
}

/// TODO
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  if let Some(location) = info.location() {
    info.message().map_or_else(
      || {
        log::error!(
          "thread 'X' panicked at {}:{}: no message provided",
          location.file(),
          location.line()
        );
      },
      |message| {
        log::error!(
          "thread 'X' panicked at {}:{}: {:?}",
          location.file(),
          location.line(),
          message
        );
      },
    );
  }

  exit_kernel(10);
}

/// TODO
pub fn exit_kernel(code: u32) -> ! {
  log::info!("Exiting unCORE with exit code {}", code);

  unsafe {
    core::arch::asm!(
        "sw {0}, 0({1})",
        in(reg)(code << 16) | 0x3333, in(reg)0x10_0000
    );

    // For the case that the QEMU exit attempt did not work, transition into an infinite
    // loop. Calling `panic!()` here is unfeasible, since there is a good chance
    // this function here is the last expression in the `panic!()` handler
    // itself. This prevents a possible infinite loop.
    loop {
      core::arch::asm!("wfi", options(nomem, nostack));
    }
  }
}
