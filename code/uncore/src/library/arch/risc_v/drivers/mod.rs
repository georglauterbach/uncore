//! TODO

pub mod uart;

/// TODO
pub fn init() -> Result<(), &'static str> {
  // TODO make this function callable only once!
  uart::Uart::init();

  Ok(())
}
