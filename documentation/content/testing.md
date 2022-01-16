# Testing the Kernel

_unCORE_ provides unit- and integration-tests. All unit-test are located "inside" the kernel itself (as members of the `lib.rs` "crate"), all integration tests are found under `kernel/tests/`. Note that linting the kernel is an important part of code quality analysis as well - your code is checked against the guidelines set in `kernel/.rustfmt.toml`.

## Unit Tests

All unit tests for the kernel are associated with `lib.rs` and not with `main.rs`. Therefore, only one test runs when testing `main.rs` (a trivial assertion). `main.rs` is tested too when running all tests because it is easier to just use `cargo test --tests` to run all tests instead of running each tests individually.

Unit tests run via the `#!rust #[test_case]` directive above the test:

``` RUST
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
```

A (simple) test runner implementation (that we built our own - remember, we are in a `#!rust #[no_std]` environment), will just execute all tests one after another.

## Integration Tests

Integration tests reside under `kernel/tests/`. These test bigger parts of the whole kernel to make sure all parts work together nicely. They have a common structure. Some of the integration tests [do not use a test harness][cargo-tests]. When writing new integration tests, the top part of the test file always looks the same:

``` RUST
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright 2022 The unCORE Kernel Organization

// ? GLOBAL CRATE ATTRIBUTES AND DOCUMENTATION
// ? ---------------------------------------------------------------------

// This crate does not and cannot use the standard library.
#![no_std]
// As this is no ordinary program, we have a special entry-point,
// which is not the `main()` function.
#![no_main]
// Clippy lint target one. Enables all lints that are on by
// default (correctness, suspicious, style, complexity, perf) .
#![deny(clippy::all)]
// Clippy lint target two. Enables lints which are rather strict
// or have occasional false positives.
#![deny(clippy::nursery)]
// Clippy lint target three. Enables new lints that are still
// under development
#![deny(clippy::pedantic)]
```

If a test uses a test harness, you'll need to go on with

``` RUST
// Use custom test runners. Since we cannot use the standard
// library, we have to use our own test framework.
#![feature(custom_test_frameworks)]
// With our own test framework, we have to define which function
// runs our tests.
#![test_runner(test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]
```

and then

``` RUST

// ? MODULES and GLOBAL / CRATE-LEVEL FUNCTIONS
// ? ---------------------------------------------------------------------

use kernel::{
        library,
        prelude::*,
};

#[no_mangle]
pub extern "C" fn efi_main(
        uefi_image_handle: uefi::Handle,
        uefi_system_table_boot: library::boot::UEFISystemTableBootTime,
) -> !
{
        library::log::init(Some(log::Level::Trace));
        library::log::display_initial_information();

        let (_uefi_system_table_runtime, uefi_memory_map) = library::boot::exit_boot_services(
                uefi_image_handle,
                uefi_system_table_boot,
        );

        kernel_main(uefi_memory_map)
}

fn kernel_main(_: library::boot::UEFIMemoryMap) -> !
{
        log_info!("This is the 'TEST_NAME' test");
    ...
}

#[panic_handler]
fn panic(panic_info: &::core::panic::PanicInfo) -> ! { panic_callback(false, panic_info) }
```

## Running Tests

Running kernel tests ist a bit more tricky than you might think. We will need to run them inside QEMU, and on top of that, `cargo` does not (yet) provide a nice interface to list the files it created for the tests.

`cargo` creates a new binary for each integration test, i.e. for `main.rs`, for `lib.rs` and so on, and it does not tell us the file names in an easy way. We therefore rely on `kernel/.cargo/config.toml`. We can provide a workspace member (`test_runner`) that will receive the produced binary as an argument. And because this repository has some fine Bash scripts in place, the workspace member is "just" a nice wrapper for running the `scripts/run_in_qemu.sh` script. Note that the wrapper does some very important things like testing for timeouts and checking whether the correct exit code (`0x3` for success) was provided.

The whole test invocation is again wrapped by another script (mostly for convenience but also to provide the `ROOT_DIRECTORY` environment variable), namely `scripts/test_kernel.sh`. The whole "call-stack" looks like this:

``` BASH
[just test] ──> scripts/test_kernel.sh ─────────┐
                                                │

                                            cargo test ... # (1)

                                                │
scripts/run_in_qemu.sh <── kernel/test_runner <─┘
```

1. This command invokes the next programs multiple times, once for each test executable

This looks overcomplicated, but integrates nicely with existing (shell) code and is currently the easiest approach. When using [Just] you can just run

``` BASH
# this will run all tests
$ just test
[   INF   ]                     tests@bash | Running all unit- and integration tests

# or run a single test
$ just test --test basic_boot
[   INF   ]                     tests@bash | Running integration test 'basic_boot'

# or just the test belonging to `lib.rs` (a.k.a. unit-tests)
$ just test --test lib
[   INF   ]                     tests@bash | Running only unit tests
```

[//]: # (Links)

[Just]: https://github.com/casey/just
[cargo-tests]: https://doc.rust-lang.org/cargo/commands/cargo-test.html
