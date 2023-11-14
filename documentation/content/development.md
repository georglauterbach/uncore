---
tags:
  - development
  - guidelines
  - workflow
---

# Development

!!! info "Preliminary Information"

    We expect you to have _some_ experience with _Rust_; you need not be an expert, but _unCORE_ requires you to understand the basics of the programming language. If you are a complete beginner, we highly recommend you to read the [official Rust book](https://doc.rust-lang.org/book/).

## Getting Started

After you have forked the repository, you can clone it. All Rust code resides in `code/`. The documentation lives in `documentation/`. `misc/` contains miscellaneous files, e.g., GDB initialization files, shell aliases, etc. In the `.github/` directory you can find CI/CD and GitHub-related configuration files. When entering the `code/` directory, you can see that it is a [Cargo Workspace][www::docs::cargo-workspace]. The default binary (in `code/src/`) acts as a pseudo-build-system and manages building, running, debugging and testing _unCORE_ for you. The actual kernel code lives in `code/uncore/`.

When you want to start working on _unCORE_, go ahead and install Rust by running `./misc/scripts/install_rust.sh`. When you later work on this project, you will be told if you're missing other dependencies (like `qemu-system-riscv64` or `jq`).

## Workflow

### Working with _unCORE_

When working on _unCORE_, you use the workspace's main binary. You run it by executing `cargo run`, and you provide all arguments to it in the same command. To see which commands and options the binary supports, run the following commands:

```console
$ cd code
$ cargo run -- help
Compiling uncore-helper v1.0.0-alpha1 (/path/to/uncore/code)
Finished dev [unoptimized + debuginfo] target(s) in 1.56s
Running `target/debug/uncore-helper help`

Workspace member that eases working with unCORE.
...
```

There are different commands available: The `run` command will run _unCORE_; when you use `run --debug`, you can attach GDB; when you use `u-test`, you run unit-tests. Using the `help` (or `--help` or `-h`) command will always show which commands you can run. These patterns are used ubiquitously; [CI][docs::ci] also makes use of these commands.

To further ease the process, aliases are defined in [`code/.cargo/config.toml`][code::github::cargo-aliases]. Hence, to run the kernel, you may use `#!bash cargo _run`. Have a look at the file to see which other aliases are defined.

### About the Workspace

The workspace that lives inside `code/` has two members: the default binary in `code/src/` and the "proper" member in `src/uncore/` where the kernel code resides. When using `cargo run -- <COMMAND>`, you invoke the default binary, which contains code to handle the other workspace members; and this is the trick to using only Rust and no build system. The default binary then likely invokes Cargo again with all the correct arguments and options to properly build the kernel. It also performs required checks (e.g., on dependencies) beforehand. With such a workspace, _unCORE_ does not require a build system or additional configuration files like `.cargo/config.toml` that are inflexible.

!!! tip "Documentation of Workspace Main Binary"

    You can open the main workspace binary's documentation with `#!bash cd code && cargo doc --open`

## Conventions

Please stick to the style and naming conventions you encounter in this project. [Clippy][www::github::clippy] is used to check and lint the Rust code in this project. [rustfmt][www::github::rustfmt] is used to format (and check) the code. An appropriate `.rustfmt.toml` is already provided. Similarly, we use [EditorConfig][www::homepage::editorconfig]. Conventions are enforced by our [CI][docs::ci].

## Debugging

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

You can then attach to QEMU with [GDB][www::homepage::gdb]. An example initialization script for GDB can found at [`misc/gdb/init.gdb`][code::github::gdb-init].

!!! note

    The command also works for unit-tests: `cargo run -q -- -vv u-test --debug`. For integration tests, you will need to specify the test name in conjunction with the `--debug` flag: `cargo run -q -- -vv i-test --debug --test <TEST NAME>`.

[//]: # (Links)

[www::docs::cargo-workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[www::github::clippy]: https://github.com/rust-lang/rust-clippy
[www::github::rustfmt]: https://github.com/rust-lang/rustfmt
[www::homepage::editorconfig]: https://editorconfig.org/
[www::homepage::gdb]: https://www.sourceware.org/gdb/
[code::github::gdb-init]: https://github.com/georglauterbach/uncore/blob/master/misc/gdb/init.txt
[code::github::cargo-aliases]: https://github.com/georglauterbach/uncore/blob/master/code/.cargo/config.toml

[docs::ci]: ./testing.md#ci
