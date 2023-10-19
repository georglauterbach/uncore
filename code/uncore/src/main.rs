// Steve Operating System
// Stephen Marz
// 21 Sep 2019
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use crate::arch::{drivers, exit_kernel};

mod arch;
mod log;

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  print!("Aborting: ");
  if let Some(p) = info.location() {
    println!(
      "line {}, file {}: {}",
      p.line(),
      p.file(),
      info.message().unwrap()
    );
  } else {
    println!("no information available.");
  }
  abort();
}

#[no_mangle]
extern "C" fn abort() -> ! {
  loop {
    unsafe {
      core::arch::asm!("wfi");
    }
  }
}

#[no_mangle]
extern "C" fn _main() {
  drivers::init();

  // Now test println! macro!
  println!("This is my operating system! Juhu!");
  println!("I'm so awesome. If you start typing something, I'll show you what you typed!");

  exit_kernel(1);

  // drivers::uart::Uart::read_loop();
}
