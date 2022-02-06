# The Internal Kernel Structure

## Architecture

The bootstrapping process of _unCORE_ is explained in detail in the [bootstrapping documentation page][docs::bootstrapping]. There is no magic involved with this kernel - except for code clarity and code quality, of course - _unCORE_ puts on its trousers one leg at a time. You should be able to navigate the code base quite easily - on the one hand because it is not too large, on the other hand because the code documentation for items allows you to jump through the code of you have a proper IDE setup.

## Modules

The kernel is composed of different, so-called "modules". These big building blocks are all found in the source code under `kernel/src/library/`. Here are all modules nicely listed:

``` BASH
kernel/src/library/
├── architectures/
├── helper/
├── memory/
├── log.rs
├── mod.rs
└── prelude.rs
```

### `architectures`

The `architectures` module provides **all** the **architecture dependent** code. This includes boot code, CPU setup code or virtual memory support for this platform. Most modules

### `helper`

The `helper` module provides very generic function all other workspace members use, for example test runners, `#!rust panic` functionality and a function that does not return.

### `memory`

The `memory` module provides main memory management functionality. It is a kernel core component.

### Other Files at the Top of the Kernel Library

Next to the main modules, there are few files residing at the top of the kernel library. This includes `log.rs` for logging, `mod.rs` as the module file for the library, and `prelude.rs`, which provides the common _Rust_ pattern of the [prelude][rust::prelude].

[//]: # (Links)

[docs::bootstrapping]: ./bootstrapping.md

[rust::prelude]: https://stackoverflow.com/questions/36384840/what-is-the-prelude
