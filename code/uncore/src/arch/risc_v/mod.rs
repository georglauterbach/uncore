mod boot;
pub mod drivers;

#[no_mangle]
extern "C" fn eh_personality() {}

/// TODO
#[no_mangle]
#[inline(always)]
pub unsafe fn abort() -> ! {
  loop {
    unsafe {
      core::arch::asm!("wfi", options(nomem, nostack));
    }
  }
}

/// TODO
pub fn exit_kernel(code: u32) -> ! {
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
      abort();
    }
  }
}

/// TODO
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  if let Some(p) = info.location() {
    crate::println!(
      "thread 'X' paniced at {}:{}: {}",
      p.file(),
      p.line(),
      if let Some(message) = info.message() {
        message.as_str().unwrap_or("no message provided")
      } else {
        "no message provided"
      }
    );
  } else {
    crate::println!("aborting due to panic (no information available)");
  }

  exit_kernel(10);
}
