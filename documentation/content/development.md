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

To further ease the process, you can run `source misc/scripts/uncore_commands.sh`, which provides aliases like `run` (for `cargo run -q -- run`) that shorten the command invocations.

## Conventions

Please stick to the style and naming conventions you encounter in this project. [`rustfmt`][www::github::rustfmt] is used to format (and check) the code. An appropriate `.rustfmt.toml` is already provided. Similarly, we use [EditorConfig][www::homepage::editorconfig]. Conventions are enforced by our [CI][docs::ci].

## Debugging

!!! warning "This section is TODO"

[//]: # (Links)

[www::docs::cargo-workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[www::github::rustfmt]: https://github.com/rust-lang/rustfmt
[www::homepage::editorconfig]: https://editorconfig.org/

[docs::ci]: ./testing.md#ci
