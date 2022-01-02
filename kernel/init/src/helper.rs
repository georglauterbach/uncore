/// ### Print Abort Message and Exit
///
/// If there was an error, print the given message that describes the
/// error and exit the whole process.
pub fn print_abort_message_and_exit<F>(message: F) -> !
where
	F: std::fmt::Display,
{
	println!("[not ok]");
	eprintln!("ERROR   | {}", message);
	std::process::exit(1)
}
