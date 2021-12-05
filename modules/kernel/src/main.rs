#![no_std]
#![no_main]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::core_lib::tests::testing::test_runner)]
#![reexport_test_harness_main = "__start_tests"]

/// # Important Information
///
/// ## Kernel Parameters
///
/// As this is an absolutely freestanding project,
/// we cannot rely on the standard-library, and we
/// will need to use our own entrypoint, not `main()`.
///
/// Therefore,
///
/// ``` RUST
/// #![no_std]
/// #![no_main]
/// ```
///
/// are used.
///
/// ## Linting Targets
///
/// A very strict set of rules is employed to guarantee
/// clear, robust and idiomatic code.
///
/// Therefore,
///
/// ``` RUST
/// #![deny(clippy::all)]
/// #![deny(clippy::nursery)]
/// #![deny(clippy::pedantic)]
/// ```
///
/// are used.
///
/// ## Custom Tests
///
/// As we cannot use the standard-library, we will
/// need to use our own tests framework.
///
/// Therefore,
///
/// ``` RUST
/// #![feature(custom_test_frameworks)]
/// #![test_runner(kernel::core_lib::tests::test_runner)]
/// #![reexport_test_harness_main = "__start_tests"]
/// ```
///
/// are used.
///
/// ## Naming Convention
///
/// The Rust naming convention found under
/// <https://doc.rust-lang.org/1.0.0/style/style/naming/README.html>
/// is strictly adhered to. The only exception are functions
/// which serve as a helper, such as `__never_return`. These
/// are prefixed with two underscores.

/// # Imports
///
/// The `kernel::core_lib` is used here explicitly with the
/// `use` statement, and not with the `mod` statement. As
/// `kernel::core_lib` is already used in `lib.rs`, we do not
/// want to re-import it here and possibly confuse Cargo.
///
/// The only exceptions so far is the `init()` function called
/// at the beginning of `_start`. It is called vi a`kernel::init()`
/// which is perfectly fine.
///
/// ## Macros
///
/// We will need to re-import all needed macros, as per definition
/// they reside in `crate`, which to be exact is `lib.rs`'s root
/// and **not** `main.rs`'s root.
///
/// Make sure to **always** use `core_lib::` instead of `crate::lib::` or
/// `lib::` or something else.
use kernel::core_lib;

/// # Entrypoint
///
/// The `_start` function is the entrypoint which is directly "called"
/// after booting. The bootloader will set up a stack and call this
/// function.
#[no_mangle]
pub extern "C" fn _start() -> !
{
	kernel::init();

	#[cfg(test)]
	__start_tests();

	core_lib::misc::helper::__never_return()
}

/// # Panic Handler
///
/// This function uses a conditionally compiled function
/// depending on whether running the kernel or the tests
/// suite.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { core_lib::misc::panic::panic(info) }
