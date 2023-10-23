//! TODO

pub mod uart;

/// TODO
pub fn init() -> Result<(), &'static str> {
  let mut my_uart = uart::Uart::new(0x1_000_0000);
  my_uart.init();

  Ok(())
}
