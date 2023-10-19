mod boot;
pub mod drivers;

pub fn exit_kernel(code: u32) -> ! {
  use core::arch::asm;

  const EXIT_SUCCESS: u32 = 0x5555; // Equals `exit(0)`.
  const EXIT_FAILURE_FLAG: u32 = 0x3333;
  const EXIT_FAILURE: u32 = exit_code_encode(1); // Equals `exit(1)`.
  const EXIT_RESET: u32 = 0x7777;

  /// Encode the exit code using EXIT_FAILURE_FLAG.
  const fn exit_code_encode(code: u32) -> u32 { (code << 16) | EXIT_FAILURE_FLAG }

  let code_new = match code {
    EXIT_SUCCESS | EXIT_FAILURE | EXIT_RESET => code,
    _ => exit_code_encode(code),
  };

  unsafe {
    asm!(
        "sw {0}, 0({1})",
        in(reg)code_new, in(reg)0x10_0000
    );

    // For the case that the QEMU exit attempt did not work, transition into an infinite
    // loop. Calling `panic!()` here is unfeasible, since there is a good chance
    // this function here is the last expression in the `panic!()` handler
    // itself. This prevents a possible infinite loop.
    loop {
      asm!("wfi", options(nomem, nostack));
    }
  }
}
