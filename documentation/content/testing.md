---
tags:
  - testing
  - unit tests
  - integration tests
  - linting
---

# Testing the Kernel

!!! abstract

    _unCORE_ provides unit- and integration-tests. Integration tests are found under `code/uncore/tests/`. Unit-test are located in the kernel source code as part of `code/uncore/src/lib.rs` (which is the library part of _unCORE_ that contains almost all code of _unCORE_ and that acts as a crate in and of itself). Note that [linting](https://stackoverflow.com/questions/8503559/what-is-linting) (the kernel but also all other parts of this project) is an important part of code quality enforcement. Hence, we lint the whole codebase during [CI](#ci).

## Unit Tests

Unit tests for the kernel are associated with `lib.rs` and not with `main.rs`. Unit tests are declared via the `#!rust #[test_case]` directive above the test:

```rust linenums="1" hl_lines="5"
/// ### Sanity Check
///
/// This tests is just here for sanity's sake to make
/// sure tests behave correctly at the most basic level.
#[test_case]
fn trivial_assertion() {
  const ONE: u8 = 1;
  assert_eq!(1, ONE);
  assert_eq!(ONE, 1);
}
```

A simple test runner implementation (located in `code/uncore/src/library/test.rs`) executes all tests one after another when the unit-test binary is run in QEMU. Conditional compilation (with `#!rust #[cfg(test)]`) indicates code that only runs when the unit-test binary is created. Because the library part of _unCORE_ runs the unit tests, it has a pseudo entry function that acts like `main()`:

```rust hl_lines="5"
/// The unit-test entry point of `lib.rs`. This function
/// is run when unit tests for `lib.rs` are run.
#[cfg(all(target_arch = "riscv64", test))]
#[riscv_rt::entry]
fn riscv64_entry() -> ! { ... }
```

To run unit tests, use `cargo run -- u-test`.

## Integration Tests

Integration tests reside under `code/uncore/tests/`. They test bigger parts of the whole kernel to make sure all parts work together nicely. Some integration tests do not use a [test harness](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

To run unit tests, use `cargo run -- i-test`.

### Running Tests Inside an Integration Test

If you want to run "unit-tests" inside an integration test, you require a test runner. The library part of _unCORE_ provides such a runner:

```rust linenums="1" hl_lines="3 6 9"
// Use custom test runners. Since we cannot use the standard
// library, we have to use our own test framework.
#![feature(custom_test_frameworks)]
// With our own test framework, we have to define which function
// runs our tests.
#![test_runner(uncore::test::runner)]
// We will have to re-export the actual test runner above with
// a new name so cargo is not confused.
#![reexport_test_harness_main = "__test_runner"]
```

You can then call `#!rust __test_runner();` to run all tests marked with `#!rust #[test_case]`.

## How Test are Implemented

Running kernel tests is a bit more tricky than you might think. We will need to run them inside QEMU, and on top of that, `cargo` does not (yet) provide a nice interface to list the files it created for the tests. The trick is to supply the `--no-run` and `--message-output=json` flags when running `cargo test ...` and then parse the binary file paths with `jq`. These file paths can then be used in conjunction with QEMU. Relying on special files like `.cargo/config.toml` would be infeasible as they introduce other pitfalls and have critical downsides (like forcing the whole workspace to a target).

## CI

Continuous Integration (CI) is a critical part of modern software development. This project uses [GitHub Actions](https://docs.github.com/en/actions). When you open a PR or when pushing on a branch, action(s) are run to check and test your code. These checks consist of linting as well as unit- and integration tests.

!!! tip "Praise be Linters"

    A linter that is probably going to be very annoying, nerve-wrecking, but also essential in the end is [`clippy`](https://github.com/rust-lang/rust-clippy). You may have noticed the many `#!rust #![deny(clippy::LINT_TARGET)]` lines in `kernel/src/lib.rs`. These lines enable linting targets for `clippy` for the whole kernel. If you do not want `clippy` to eat you alive during GitHub's CI, **fix the lints locally**. You can run `cargo run -- check` to check for all kinds of linting issues.
