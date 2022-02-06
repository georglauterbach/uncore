# Building and Running _unCORE_

The `kernel/` directory contains all kernel code (and therefore the complete _Rust_ code of _unCORE_). It is, at the same time, package and a Cargo workspace. That means, the `kernel/` directory contains a binary -- the kernel with all its source code located under `kernel/src/` -- and **possibly** other workspace members in the future. The final binary is obviously built from the source code located at `kernel/src/`.

We highly recommend using [Just] when working with _unCORE_. The following steps will just (no pun intended) explain what [Just] does in the background. With [Just] installed, you do not need to run all these long and tedious and error-prone commands yourself.

??? tip "The Bigger Picture"
    This operating system kernel bases on and is heavily inspired by _Phillip Opperman_'s _BlogOS_ (version one and two). You can read all about it in [his blog](https://os.phil-opp.com/). **However**, due to _unCORE_'s goals of being concise and easy to understand, looking at the code base and the code comments will also help you in understanding how the kernel works and what it does.

## Compiling the Kernel

### Introductory Knowledge

The kernel is compiled against special custom targets located under `kernel/.cargo/targets/`. As of now, only the `x86_64-unknown-uncore.json` target resides there and works. This target is designed for

1. the `x86_64` architecture
2. where the vendor is set to `unknown`
3. and the operating system specification is `uncore`
4. (and no ABI specification, i.e. `none` as well, but this is omitted)

The LLVM target triple is explained again in `kernel/src/library/helper/miscellaneous.rs`. To get an overview of (all) targets, you may visit [the `rustc` target specification page on GitHub][rustc-target-specification].

!!! danger "`.cargo/config.toml` And Its Fallacies"
    Note that we use a `kernel/.cargo/config.toml` file. Using this file generally messes with the defaults for build and run targets and this _may_ lead to very unpleasant outputs. You will therefore, and of course, for convenience, use the scripts under `scripts/` or [Just] to build, run and test your code. More on this is explained on the [Testing][docs::testing] page.

### Actually Running the Kernel

First of all, if you're using [Just], make yourself familiar with all recipes by running `#!bash just help`. The kernel itself is compiled by running

``` CONSOLE
$ pwd
/uncore/kernel
$ cargo build --target .cargo/targets/x86_64-unknown-uncore.json -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem
    Compiling kernel v0.1.0 (/uncore/kernel)
    Building [=======================>   ] 20/22: kernel
    ...
```

We specify the target and on top of that, which built-in function (that is, into the compiler `rustc`) we need. That's all. This will create a debug binary of our kernel. In case you want to build a release version, add `--release` after `cargo build`.

The equivalent for this step with [Just] is

``` CONSOLE
$ just build
    Compiling kernel v0.1.0 (/uncore/kernel)
    Building [=======================>   ] 20/22: kernel
    ...
```

or use the script located under `scripts/`:

``` CONSOLE
$ pwd
/uncore
$ ./scripts/build.sh [target]
    Compiling kernel v0.1.0 (/uncore/kernel)
    Building [=======================>   ] 20/22: kernel
    ...
```

## Running in QEMU

We can now use the kernel binary we built in the step above to run it in QEMU with UEFI. Using [Just] will automatically rebuilt the kernel - so you can just use `just run` to build and run the kernel. This has the additional advantage that you need not think about being on the most up-to-date build while developing.

``` CONSOLE
$ just run
...
```

Note how this requires you to have `qemu-system-x86_64` installed, and you must also have `ovmf` installed for UEFI to work. Executing the command above will open a new window. To achieve the same with a script, you can also run

``` CONSOLE
$ pwd
/uncore
$ ./scripts/run_in_qemu.sh
...
```

Running these commands in your terminal will not open a new window unless you specify `graphical` as a parameter to [Just] or the `run_in_qemu.sh` script.

[//]: # (Links)

[docs::testing]: ./testing.md

[Just]: https://github.com/casey/just

[rustc-target-specification]: https://github.com/rust-lang/rust/tree/1.57.0/compiler/rustc_target/src/spec
