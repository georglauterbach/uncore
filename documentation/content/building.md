# Building _unCORE_

The `kernel/` directory contains all kernel code (and therefore the complete [Rust] code of _unCORE_). It is, at the same time, package and a Cargo workspace. That means, the `kernel/` directory contains a binary -- the kernel with all its source code located under `kernel/src/` -- and other workspace member, such as `boot`, located under `kernel/boot`. The final binary is obviously built from the source code located at `kernel/src/`, but `boot` handles the creation of an actually bootable image.

We **highly recommend using [Just]** for working with _unCORE_. The following steps will just (no pun intended) explain what [Just] does in the background. With [Just] installed, you do not need to run all these long and tedious and error-prone commands yourself.

!!! tip "Getting the Bigger Picture"
    This operating system kernel bases on and is heavily inspired by _Phillip Opperman_'s _BlogOS_. You can read all about it in [his blog](https://os.phil-opp.com/).

## Compiling the Kernel

The kernel is compiled against a special target. This target is located under `kernel/x64_64-uncore.json`. This target does not provide a standard library, as it is a custom target.

??? danger "`.cargo/config.toml` And Its Fallacies"
    Note that we do not actually want to use a `kernel/.cargo/config.toml` file. Using this file can mess with the defaults for build / run targets and this may lead to very unpleasant outputs. We rather write the target for each compilation explicitly when writing the command -- or we let [Just] do this for us. **But** we currently cannot circumvent using because of the way tests are executed. When `cargo test ...` runs tests, we need to tell `cargo` every time which runner to use. We (currently) cannot do this explicitly on the command line.

First of all, if you're using [Just] (which is recommended, remember), make yourself familiar with all recipes by running `#!bash just help`. The kernel itself is compiled by running

``` CONSOLE
$ pwd
/uncore/kernel
$ cargo build --target x86_64-uncore.json -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem
    Compiling kernel v0.1.0 (/uncore/kernel)
    Building [=======================>   ] 20/22: kernel
    ...
$ # (1)
```

1. See how this is a very long and tedious command? Just let [Just] do this for you.

We specify the target and on top of that, which built-in function (that is, into the compiler `rustc`) we need. That's all. This will create a debug binary of our kernel. In case you want to build a release version, add `--release` after `cargo build`.

The equivalent for this step with [Just] is

``` BASH
just build
```

## Creating a Bootable Image

### The `boot` Workspace Member

The `kernel/boot/` workspace member is responsible for creating a bootable image from out kernel binary (which itself is not bootable). It does this by creating a UEFI bootloader and then linking it to our binary. _unCORE_ will, on purpose, not look into the details of how this works. If you are curious, have a look at [this blog post by _Phillip Opperman_](https://os.phil-opp.com/minimal-rust-kernel/#creating-a-bootimage).

### Correctly Working With the `boot` Workspace Member

This is where it gets just a little bit more complicated. The `boot` workspace member is not a `#!rust #![no_std]` crate, and does consequently not use the same target as the kernel - it does not even use nightly. Running it correctly, you will need to specify the location of your kernel binary:

``` CONSOLE
$ pwd
/uncore/kernel
$ cargo run --package boot \
  --target <YOUR DEFAULT STABLE TARGET WITH STD> \
  -- \
  target/x86_64-uncore/<DEBUG or RELEASE>/kernel [--no-run]

Creating disk image...   [ok]
Created disk image at    '/kernel/target/x86_64-uncore/debug/boot-bios-kernel.img'
```

Here, [Just] will try to find your default target and use it. On Linux, this is most likely `x86_64-unknown-linux-gnu`. After this process has finished, we have an image located at `kernel/target/x86_64-uncore/debug/boot-bios-kernel.img`.

The equivalent for this step with [Just] is

``` BASH
just build_image
```

## Running in QEMU

We can now run the image created in the step above:

``` CONSOLE
$ pwd
/uncore/kernel
$ qemu-system-x86_64 \
  --no-reboot -s \
  -drive format=raw,file=/kernel/target/x86_64-uncore/debug/boot-uefi-kernel.img
```

Note how this requires you to have `qemu-system-x86_64` installed. Executing the command above will open a new window. We'd much rather like to stay on our beloved command line. We can add more options to do so:

``` CONSOLE
$ pwd
/uncore/kernel
$ qemu-system-x86_64          \
  --no-reboot -s              \
  -serial stdio -display none \
  -drive format=raw,file=/kernel/target/x86_64-uncore/debug/boot-uefi-kernel.img
```

The equivalent for this step with [Just] is

``` BASH
just run
```

[//]: # (Links)

[Rust]: https://www.rust-lang.org/
[Just]: https://github.com/casey/just
