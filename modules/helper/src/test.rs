/// ### A (Very) Simple Test Runner Implementation
///
/// This function is registered as the tests runner when executing
/// Cargo test's unit tests.
///
/// It will just execute all functions marked with `#[test_case]` one
/// by one.
pub fn test_runner(tests: &[&dyn Testable])
{
	for test in tests {
		test.run();
	}

	crate::miscellaneous::qemu::exit_with_success();
}

/// # Streamlining Testing
///
/// This trait provides the tests runner with the ability to `.run`
/// tests. This is done for all functions in the `impl` block, so they
/// can be "parsed" to reduce boilerplate code.
pub trait Testable
{
	/// ### Run Tests
	///
	/// The `run` function will literally just execute the
	/// function it contains, as `Testable` is implemented for all
	/// generics that implement `Fn()`.
	fn run(&self);
}

impl<T> Testable for T
where
	T: Fn(),
{
	fn run(&self)
	{
		// use core::any::type_name;
		// serial_print!("{}  ", type_name::<Self>());

		self();
	}
}

/// ### Sanity Check
///
/// This tests is just here for sanity's sake to make
/// sure tests behave correctly at the most basic level.
#[test_case]
fn trivial_assertion()
{
	const ONE: u8 = 1;
	assert_eq!(1, ONE);
	assert_eq!(ONE, 1);
}
