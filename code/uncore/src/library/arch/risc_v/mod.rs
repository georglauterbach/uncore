
pub mod drivers;
mod ld;

#[no_mangle]
extern "C" fn eh_personality() {}

/// TODO
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  if let Some(p) = info.location() {
    if let Some(message) = info.message() {
      log::error!("thread 'X' panicked at {}:{}: {:?}", p.file(), p.line(), message);
    } else {
      log::error!(
        "thread 'X' panicked at {}:{}: no message provided",
        p.file(),
        p.line(),
      );
    };
  } else {
    log::error!("thread 'X' panicked: no information available");
  }

  exit_kernel(10);
}

/// TODO
#[no_mangle]
#[inline(always)]
unsafe fn __abort() -> ! {
  loop {
    unsafe {
      core::arch::asm!("wfi", options(nomem, nostack));
    }
  }
}

/// TODO
pub fn exit_kernel(code: u32) -> ! {
  log::info!("Exiting unCORE with exit code {}", code);
  log::warn!("Heap size set to {:?}", ld::get_heap_bottom_and_size());

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
      __abort();
    }
  }
}
