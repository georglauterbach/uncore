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

## Integration Tests

## Running Tests

We are running tests with `cargo test ...`. This requires us to use a `.cargo/config.toml` file, as we need to specify the test runner (which is the `boot` workspace member) explicitly.

[//]: # (Links)

[Just]: https://github.com/casey/just
