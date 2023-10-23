/// TODO
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::arch::drivers::uart::Uart::new(0x1000_0000), $($args)+);
			});
}

/// TODO
#[macro_export]
macro_rules! println
{
  () => ({
    crate::print!("\r\n")
		   });
	($fmt:expr) => ({
    crate::print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			crate::print!(concat!($fmt, "\r\n"), $($args)+)
			});
}
