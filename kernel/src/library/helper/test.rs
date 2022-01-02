use bootloader as x86_64_bootloader;

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
		crate::log_test!("Most recent test PASSED");
	}
}

/// ### A (Very) Simple Test Runner Implementation
///
/// This function is registered as the tests runner when executing
/// Cargo test's unit tests.
///
/// It will just execute all functions marked with `#[test_case]` one
/// by one.
#[allow(clippy::module_name_repetitions)]
pub fn runner(tests: &[&dyn Testable])
{
	crate::log_test!("Starting tests");

	for test in tests {
		test.run();
	}

	crate::log_test!("Last test finished. SUCCESS.");
	super::miscellaneous::qemu::exit_with_success();
}

/// ### `x86_64` Test Wrapper
///
/// This `main()` function exists to provide a uniform setup for
/// integration tests running on `x86_64`.
///
/// This function initializes the kernel. It takes care of
///
/// - printing important initial information
/// - calling the hardware initialization subroutine
pub fn main(
	log_level: Option<super::super::log::Level>,
	boot_information: &'static x86_64_bootloader::BootInfo,
)
{
	use super::super::{
		hardware,
		log,
	};

	crate::log!("Running an integration test.");

	log::init(log_level, boot_information);

	crate::log_info!("Kernel initialization for tests started");

	hardware::init();
	hardware::memory::init(boot_information);

	crate::log_info!("Kernel initialization for tests finished");
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
