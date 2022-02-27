---
tags:
  - workspace
  - members
  - other crates
---

# All Workspace Members

Next to the kernel itself, there are other crates (workspace member) located under `kernel/`.

## `test_runner`

This workspace member exists to run tests with ease. It is used by `kernel/.cargo/config.toml` as the test runner. More information on tests can be found [here][docs::testing]. The binary receives the binary name of the test binary and can therefore link the bootloader to it and execute the test binary.

## `helper`

The `helper` modules provides common tasks. Like the kernel, it has both a binary and library. The library is used by other workspace members, the binary provides tasks like building the kernel, running it or starting tests.

[//]: # (Links)

[docs::testing]: ../testing.md
