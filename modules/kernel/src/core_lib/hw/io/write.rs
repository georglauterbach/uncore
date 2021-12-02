use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::core_lib::VERSION;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color
{
	Black      = 0,
	Blue       = 1,
	Green      = 2,
	Cyan       = 3,
	Red        = 4,
	Magenta    = 5,
	Brown      = 6,
	LightGray  = 7,
	DarkGray   = 8,
	LightBlue  = 9,
	LightGreen = 10,
	LightCyan  = 11,
	LightRed   = 12,
	Pink       = 13,
	Yellow     = 14,
	White      = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode
{
	const fn new(foreground: Color, background: Color) -> Self
	{
		Self((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar
{
	ascii_character: u8,
	color_code:      ColorCode,
}

#[repr(transparent)]
struct Buffer
{
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer
{
	column_position: usize,
	color_code:      ColorCode,
	buffer:          &'static mut Buffer,
}

impl Writer
{
	fn write_byte(&mut self, byte: u8)
	{
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;

				let color_code = self.color_code;
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					color_code,
				});
				self.column_position += 1;
			},
		}
	}

	fn write_string(&mut self, s: &str)
	{
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or newline
				0x20..=0x7E | b'\n' => self.write_byte(byte),
				// not part of printable ASCII range
				_ => self.write_byte(0xFE),
			}
		}
	}

	fn new_line(&mut self)
	{
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	fn clear_row(&mut self, row: usize)
	{
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code:      self.color_code,
		};
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

impl fmt::Write for Writer
{
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		self.write_string(s);
		Ok(())
	}
}

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code:      ColorCode::new(Color::Yellow, Color::Black),
		buffer:          unsafe { &mut *(0xB8000 as *mut Buffer) },
	});
}

pub fn print_init()
{
	crate::println!("unCore\n\nversion :: {}", VERSION);
}

#[doc(hidden)]
pub fn _print_write_args(args: fmt::Arguments)
{
	use core::fmt::Write;
	x86_64::instructions::interrupts::without_interrupts(|| {
		WRITER.lock().write_fmt(args).unwrap();
	});
}

/// Prints a line of text on the screen
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => (
		$crate::core_lib::hw::io::write::_print_write_args(
			format_args!($($arg)*)
		);
	);
}

/// Prints text on the screen appending a newline.
#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[test_case]
fn println_does_not_panic()
{
	println!("test_println_simple output");
}

#[test_case]
fn println_does_not_panic_complex()
{
	for _ in 0..200 {
		println!("test_println_many output");
	}
}

#[test_case]
fn test_println_output()
{
	use core::fmt::Write;

	let s = "Some tests string that fits on a single line";

	x86_64::instructions::interrupts::without_interrupts(|| {
		let mut writer = WRITER.lock();
		writeln!(writer, "\n{}", s).expect("writeln failed");
		for (i, c) in s.chars().enumerate() {
			let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
			assert_eq!(char::from(screen_char.ascii_character), c);
		}
	});
}
