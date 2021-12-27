# Testing the Kernel

_unCORE_ provides unit- and integration-tests. All unit-test are located "inside" the kernel itself, all integration tests are found under `kernel/tests/`. Note that linting the kernel is an important part of code quality analysis.

??? hint "Using a Pre-Commit Hook"
    You may run a pre-commit hook to verify your code before committing. If you are using [Just], the hook can be created like this:

    ``` BASH
    cat >.git/hooks/pre-commit << "EOM"
    #! /bin/bash

    set -euEo pipefail
    just fmt check test

    EOM
    ```

    If you are not using [Just], you may copy the targets from the `Justfile` manually into the pre-commit hook script.

!!! missing "Missing Documentation"
    **This documentation is missing major parts**. You could contribute here yourself.

## Unit Tests

All unit tests for the kernel are associated with `lib.rs` and not with `main.rs`. Therefore, zero tests run when testing `main.rs`. `main.rs` is tested because it is easier to just use `cargo test --tests` to run all tests instead of running each tests individually.

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

A simple test runner implementation (remember, we are in a `#!rust #[no_std]` environment), will just execute all tests one after another.

## Integration Tests

Integration tests reside under `kernel/tests/`.

## Running Tests

We are running tests with `cargo test ...`. This requires us to use a `.cargo/config.toml` file, as we need to specify the test runner (which is the `boot` workspace member) explicitly.

[//]: # (Links)

[Just]: https://github.com/casey/just
