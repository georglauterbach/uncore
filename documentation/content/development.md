---
tags:
  - development
  - guidelines
  - workflow
---

# Development

!!! info "Preliminary Information"

    We expect you to have _some_ experience with _Rust_: you need not be an expert, but _unCORE_ requires you to understand the basics of the programming language. If you are a complete beginner, we highly recommend you to read the [official Rust book](https://doc.rust-lang.org/book/).

## :rocket: Getting Started

After you have forked the repository, you can clone it. All Rust code resides in [`code/`][code::github::code/]. The documentation lives in [`documentation/`][code::github::documentation/]. [`misc/`][code::github::misc/] contains miscellaneous files, e.g., GDB initialization files, shell aliases, etc. In the [`.github/`][code::github::.github/] directory you can find CI/CD and GitHub-related configuration files. The [`code/`][code::github::code/] directory is a [_Cargo_ Workspace][www::docs::cargo-workspace].

If you want to start working on _unCORE_, go ahead and install Rust by running [`./misc/scripts/install_rust.sh`][code::github::misc/scripts/install-rust.sh]. When you later work on this project, you will be told if you're missing other dependencies (like [`qemu-system-riscv64`][www::homepage::qemu-riscv], [`jq`][www::homepage::jq] or [`mold`][www::github::mold]).

## :toolbox: Workflow

### About the Workspace

The workspace that lives inside [`code/`][code::github::code/] has a "main" binary and "proper" workspace members. The main binary lives in [`code/src/`][code::github::code/src]. An example for a workspace member is [`code/uncore/`][code::github::code/uncore]; this is where the kernel code resides. When using `#!bash cargo run -- <COMMAND>`, the main binary is invoked, which contains code to handle the other workspace members; and this is the trick to using only Rust and no build system (other than _Cargo_). The default binary, when invoked, likely invokes _Cargo_ again with all the correct arguments and options to properly build the kernel. It also performs required checks (e.g., on dependencies) beforehand. With such a workspace, _unCORE_ does not require a build system or additional build configuration in files like [`.cargo/config.toml`][www::documentation::cargo::configuration] that are inflexible.

The actual kernel code lives in [`code/uncore/src/`][code::github::code/uncore/src]. In this directory, you fill find [`main.rs`][code::github::code/uncore/src/main.rs], [`lib.rs`][code::github::code/uncore/src/lib.rs] and [`library/`][code::github::code/uncore/src/library/]. [`main.rs`][code::github::code/uncore/src/main.rs] is recognized by _Cargo_ automatically as a binary. It contains the kernel's entry point (called by a bootloader). [`lib.rs`][code::github::code/uncore/src/lib.rs] is automatically recognized by _Cargo_ as a library target. This is useful because we can put all kernel code in the library, whose root is [code::github::code/uncore/src/lib.rs], and just call it from binaries - such binaries are not only [`main.rs`][code::github::code/uncore/src/main.rs], but integration tests (in [`code/uncore/tests/`][code::github::code/uncore/tests/]) as well! [`library/`][code::github::code/uncore/src/library/] is the top-level directory that is used by [`lib.rs`][code::github::code/uncore/src/lib.rs] as a module (i.e., with `#!rust mod library;`); [`library/`][code::github::code/uncore/src/library/] exists in order to not have all top-level modules in the top-level directory.

### Working with _unCORE_

When working on _unCORE_, you use the workspace's main binary. You run it by executing `#!bash cargo run`, and you provide all arguments to it in the same command. To see which commands and options the binary supports, run the following commands:

```console
$ cd code
$ cargo run -- help # (1)
Compiling uncore-helper v1.0.0-alpha1 (/path/to/uncore/code)
Finished dev [unoptimized + debuginfo] target(s) in 1.56s
Running `target/debug/uncore-helper help`

Workspace member that eases working with unCORE.
...
```

1. The `--` is used to separate arguments for _Cargo_ from those that we want to pass to our binary. In this case, `run` is an argument to _Cargo_, `help` is an argument to our binary.

There are different commands available: The `#!bash run` command will run _unCORE_; when you use `#!bash run --debug`, you can attach GDB; when you use `u-test`, you run unit-tests. Using the `#!bash help` (or `#!bash --help` or `#!bash -h`) command will always show which commands you can run. These patterns are used ubiquitously; _unCORE_'s [CI][docs::ci] also makes use of these commands.

To further ease the process, aliases are defined in [`code/.cargo/config.toml`][code::github::code/.cargo/config.toml]. Hence, to run the kernel, you may use `#!bash cargo _run`. Have a look at the file to see which other aliases are defined.

### Build-Time

As mentioned [earlier](#about-the-workspace), the kernel is actually built by the main workspace binary (residing in [`code/src/`][code::github::code/src]). The function that invokes _Cargo_ is [`code/src/command.rs:build`][code::github::code/src/command.rs:build]. _Cargo_ then builds the kernel whose code resides in [`code/uncore/src/`][code::github::code/uncore/src].

The "heavy lifting" is done by _Cargo_. The workspace main binary "only" takes care of checking dependencies and invoking _Cargo_ correctly, i.e., with the correct target (architecture), environment variables used when building, linker script (and linker), etc.

## :compass: Conventions

Please stick to the style and naming conventions you encounter in this project. [Clippy][www::github::clippy] is used to check and lint the Rust code in this project. [rustfmt][www::github::rustfmt] is used to format (and check) the code. An appropriate [`code/.rustfmt.toml`][code::github::code/.rustfmt.toml] is already provided. Similarly, we use [EditorConfig][www::homepage::editorconfig]. Conventions are enforced by our [CI][docs::ci].

## :fire_extinguisher: Debugging

The [workspace main binary](#about-the-workspace) provides an easy way to debug _unCORE_. Debugging is supported for running the plain kernel binary, the unit-, and the integration-tests. All you need to do is add the `--debug` flag to these targets:

```console
$ cargo run -q -- -vv run --debug
...
DEBUG Checking run-time dependencies
TRACE Checking run-time dependencies required for debugging
...
INFO  Debugging unCORE
DEBUG You may use 'gdb-multiarch -q -x ../misc/gdb/init.txt' to attach now
TRACE Remember: 'Ctrl-A x' will exit QEMU
...
```

You can then attach to QEMU with [GDB][www::homepage::gdb]. An example initialization script for GDB can found at [`misc/gdb/init.gdb`][code::github::misc/gdb/init.txt].

!!! note

    The command also works for unit-tests: `cargo run -q -- -vv u-test --debug`. For integration tests, you will need to specify the test name in conjunction with the `--debug` flag: `cargo run -q -- -vv i-test --debug --test <TEST NAME>`.

[//]: # (Links)

[code::github::code/]: https://github.com/georglauterbach/uncore/blob/master/code/
[code::github::documentation/]: https://github.com/georglauterbach/uncore/blob/master/documentation/
[code::github::misc/]: https://github.com/georglauterbach/uncore/blob/master/misc/
[code::github::.github/]: https://github.com/georglauterbach/uncore/blob/master/.github/
[www::docs::cargo-workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[code::github::misc/scripts/install-rust.sh]: https://github.com/georglauterbach/uncore/blob/master/misc/scripts/install_rust.sh
[www::homepage::qemu-riscv]: https://www.qemu.org/docs/master/system/target-riscv.html
[www::homepage::jq]: https://jqlang.github.io/jq/
[www::github::mold]: https://github.com/rui314/mold
[code::github::code/src]: https://github.com/georglauterbach/uncore/tree/master/code/src/
[code::github::code/uncore]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/
[www::documentation::cargo::configuration]: https://doc.rust-lang.org/cargo/reference/config.html
[code::github::code/uncore/src]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/src/
[code::github::code/uncore/src/main.rs]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/src/main.rs
[code::github::code/uncore/src/lib.rs]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/src/lib.rs
[code::github::code/uncore/src/library/]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/src/library/
[code::github::code/uncore/tests/]: https://github.com/georglauterbach/uncore/tree/master/code/uncore/tests/
[code::github::code/.cargo/config.toml]: https://github.com/georglauterbach/uncore/blob/master/code/.cargo/config.toml
[www::github::clippy]: https://github.com/rust-lang/rust-clippy
[www::github::rustfmt]: https://github.com/rust-lang/rustfmt
[code::github::code/.rustfmt.toml]: https://github.com/georglauterbach/uncore/blob/master/code/.rustfmt.toml
[www::homepage::editorconfig]: https://editorconfig.org/
[docs::ci]: ./testing.md#continuous-integration-ci
[www::homepage::gdb]: https://www.sourceware.org/gdb/
[code::github::misc/gdb/init.txt]: https://github.com/georglauterbach/uncore/blob/master/misc/gdb/init.txt
[code::github::code/src/command.rs:build]: https://github.com/georglauterbach/uncore/blob/master/code/src/command.rs#L210
