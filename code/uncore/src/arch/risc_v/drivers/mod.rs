pub mod uart;

pub fn init() {
  let mut my_uart = uart::Uart::new(0x1_000_0000);
  my_uart.init();
}
