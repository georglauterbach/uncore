/// ### Are We Running Tests?
///
/// Can be used to get information about whether tests are run or not.
#[allow(dead_code)]
#[cfg(test)]
pub const IS_TEST: bool = true;

/// ### Are We Running Tests?
///
/// Can be used to get information about whether tests are run or not.
#[allow(dead_code)]
#[cfg(not(test))]
pub const IS_TEST: bool = false;

/// ### A (Very) Simple Test Runner Implementation
///
/// This function is registered as the tests runner when executing
/// Cargo test's unit tests.
///
/// It will just execute all functions marked with `#[test_case]` one
/// by one.
#[allow(clippy::module_name_repetitions)]
pub fn test_runner(tests: &[&dyn Testable])
{
	crate::log_test!("Starting tests");

	for test in tests {
		test.run();
	}

	super::miscellaneous::qemu::exit_with_success();
}

/// ### Streamlining Testing
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
		crate::log_test!("Testing {}", ::core::any::type_name::<Self>());
		self();
		crate::log_test!("Last test finished successfully");
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
