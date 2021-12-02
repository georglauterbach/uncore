#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(uncore::core_lib::tests::testing::test_runner)]
#![reexport_test_harness_main = "__start_tests"]

/// # Imports
///
/// The `uncore::core_lib` is used here explicitly with the
/// `use` statement, and not with the `mod` statement. As
/// `uncore::core_lib` is already used in `lib.rs`, we do not
/// want to re-import it here and possibly confuse Cargo.
///
/// ## Macros
///
/// We will need to re-import all needed macros, as per definition
/// they reside in `crate`, which to be exact is `lib.rs`'s root
/// and **not** `main.rs`'s root.
///
/// Make sure to **always** use `core_lib::` instead of `crate::lib::` or
/// `lib::` or something else.
use uncore::core_lib;
use uncore::println;

#[no_mangle]
pub extern "C" fn _start() -> !
{
	__start_tests();
	core_lib::misc::helper::__never_return()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { core_lib::misc::panic::test_panic_handler(info) }

#[test_case]
fn println_does_not_panic()
{
	println!("test_println output");
}
