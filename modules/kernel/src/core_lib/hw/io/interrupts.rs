use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{
	instructions,
	structures::idt::InterruptStackFrame,
};

use crate::print;

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
	Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex
{
	Timer = PIC_1_OFFSET,
	Keyboard,
}

/// # Loading Programmable Interrupt Controllers
///
/// Two Programmable Interrupt Controllers (PICs) are loaded here.
/// Furthermore, the CPU is instructed to listen for interrupts.
pub fn init_pics()
{
	unsafe {
		PICS.lock().initialize();
	}

	instructions::interrupts::enable();
}

/// # End of Interrupt Signalization
///
/// Every hardware (I/O) interrupt handler must issue an "end of interrupt"
/// (EOI) signal at the end to signal that we're finished with processing the
/// interrupt.
///
/// This function provides a safe wrapper around the unsafe method.
fn notify_end_of_interrupt(interrupt_index: InterruptIndex)
{
	unsafe {
		PICS.lock().notify_end_of_interrupt(interrupt_index as u8);
	}
}

/// # Hardware Interrupt - Timer
///
/// This is the handler function for timer interrupts.
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame)
{
	print!(".");
	notify_end_of_interrupt(InterruptIndex::Timer);
}

/// # Hardware Interrupt - Keyboard
///
/// This is the handler function which reacts to keyboard input. Currently,
/// every keystroke is printed directly on the screen.
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame)
{
	use pc_keyboard::{
		layouts::Us104Key,
		DecodedKey,
		HandleControl,
		Keyboard,
		ScancodeSet1,
	};

	lazy_static! {
		static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> =
			Mutex::new(Keyboard::new(Us104Key, ScancodeSet1, HandleControl::Ignore));
	}

	let mut keyboard = KEYBOARD.lock();
	let mut port = instructions::port::Port::new(0x60);
	let scancode: u8 = unsafe { port.read() };

	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
				DecodedKey::Unicode(character) => print!("{}", character),
				DecodedKey::RawKey(key) => print!("{:?}", key),
			}
		}
	}

	notify_end_of_interrupt(InterruptIndex::Keyboard);
}
