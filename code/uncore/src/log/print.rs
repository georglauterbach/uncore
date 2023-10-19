#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::arch::drivers::uart::Uart::new(0x1000_0000), $($args)+);
			});
}

#[macro_export]
macro_rules! println
{
  () => ({
    use crate::print;
    print!("\r\n")
		   });
	($fmt:expr) => ({
    use crate::print;
    print!(concat!($fmt, "\r\n"))
			});
	($fmt:expr, $($args:tt)+) => ({
			print!(concat!($fmt, "\r\n"), $($args)+)
			});
}


