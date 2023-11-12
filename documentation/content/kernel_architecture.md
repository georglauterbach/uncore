---
tags:
  - kernel
  - architecture
---

# The Kernel Architecture

!!! warning "This Page is TODO"

## Directory Structure

Kernel code resides in `code/uncore/src/`. The main kernel functionality can be listed by listing the contents of `code/uncore/src/library/`:

```console
$ eza -lh code/uncore/src/library/
arch/
log/
mod.rs
prelude.rs
test.rs
```

### `arch/`

### `log/`

### `mod.rs`, `prelude.rs` and `test.rs`

`mod.rs` is simply the top-level module descriptor of `library/` and lists all other modules. `prelude.rs` provides the common Rust pattern of the [prelude](https://stackoverflow.com/questions/36384840/what-is-the-prelude). `test.rs` contains all functionality required to run unit-tests.

## Bootstrapping
