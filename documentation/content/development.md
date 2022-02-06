# Development

!!! quote "Why Rust?"
    Rust is **very performant** and **abstract**. It does not suffer from legacy problems that C or C++ bring with them. Of course, C and C++ are much older languages, but this is more or less a bad justification.

    _Abstraction is not about vagueness, it is about being precise on a new semantic level._ –– **Edsger W. Dijkstra**

## Git Flow

Please sign your commits so GitHub can verify them. The `master` branch contains the current stable code base. The `development` branch contains the latest changes, which may not be as stable as `master`. For every new version, there is a `version/X.Y.Z` branch, that is first merged into `development`, and then into `master`. The `X.Y.Z` follows the semantic versioning guidelines strictly! New features are being added through `feature/name-of-the-feature` branches. Hotfixes can be merged either into a `version/` branch or into `development`, but not into `master`. The following illustration shows the order of merging a feature.

``` TXT
feature/name-of-the-feature   ─── >   version/X.Y.Z   ─── >   development   ─── >   master
                                      │                       │
hotfix                    >   ────────┘           >   ────────┘
```

One can then rebase onto `development` if something has been merged into development (e.g. a new version).

## Coding Style

### General

When writing code, adhere to the style provided in the miscellaneous configuration files and to what is already written in all the files, even if this is not your preferred style. When altering files, look how it has been written and stay true to these design decisions. Make sure your IDE uses the provided `.editorconfig`. When writing YAML, code is formatted with [Prettier]. When writing Bash scripts, make sure, especially with Bash, that you stick to the already present style!

**Avoid** the following under all circumstances:

1. C/C++ idiosyncrasies, such as stacked `#!c typedef`s or preprocessor chaos with `#!c define`s
2. Using abbreviations everywhere, which makes code unreadable for others; we are trying to write _concise_ code, not necessarily short code - concise and short are not always the same
3. Using `#!rust type` to define type aliases for very simple types; only use this when absolutely appropriate, otherwise, write out the whole type

### Rust Conventions

Rust if formatted using `rustfmt`, which is installed with _Rust_ itself. You can format your code using `just format` or `cargo fmt` in the repository modules containing _Rust_ code. The style definition is found under `kernel/.rustfmt.toml`. Make sure to adjust your style to the already present style. The [Rust naming convention] is strictly adhered to.

---

#### Goal

We want to ensure, at all cost, that code in this project becomes as unreadable as some Linux kernel code. This has nothing to do with formatting taste, but with problems inherent to C and how programmers are used to writing C. When it comes to formatting taste, just adhere to the style defined by `.rustfmt.toml` and do not complain.

---

#### Imports

When importing modules, do not use a fully-qualified named, but only the module:

``` RUST
// bad
use super::some_module::SomeStructure;
fn foo() -> SomeStructure { ... }

// good
use super::some_module;
fn foo() -> some_module::SomeStructure { ... }
```

The only exceptions to this rule are

1. the `prelude` module
2. imports in function bodies for very frequently used items (as scope is very limited, it is fine)

``` RUST
fn foo()
{
  // okay, but only for very frequently used items
  use super::some_module::SomeStructure;
}
```

---

#### Variable Names

Write variable names in their long form and **do not use abbreviations**. Abbreviations are unnecessary as they do not impact final binary size and they clutter the readability of the code. There is simply no reason to write

``` RUST
// bad
pub static MB2_INF: ...
```

when

``` RUST
// good
pub static MULTIBOOT2_INFORMATION: ...
```

is more readable just for the sake of laziness or inappropriate shortness.

---

#### Code Documentation

Every `#!rust struct`, every `#!rust fn` and all other constructs need proper documentation. This documentation shall be concise and clear. Your best friend [`clippy`][rust-clippy] will not pass until every last item is properly documented.

## CI/CD and Testing

### Praise be Linters

The kernel uses several checks to determine of the code satisfies the high quality standards. GitHub actions test the kernel with unit- and integration tests. Moreover, proper formatting ist checked with `rustfmt`. A linter that is probably going to be very annoying, nerve-wrecking but essential in the end is [`clippy`][rust-clippy]. You may have noticed the many `#!rust #![deny(clippy::LINT_TARGET)]` lines in `kernel/src/lib.rs`. These lines enable linting targets for clippy for the whole kernel. All dependencies are checked by [`cargo audit`][cargo-audit] for security vulnerabilities. The whole repository is checked by [`shellcheck`][shellcheck] for Bash scripts and we use the [GitHub Super Linter] to check various linting issues.

### A Word of Advice

If you do not want [`clippy`][rust-clippy] to eat you alive during GitHub's CI, _fix the lints locally_. You can run `just check` to check formatting and clippy errors. With `just test`, you run the unit- and integration tests. With `just lint`, you run generic linters, i.e. [`shellcheck`][shellcheck] and the [GitHub Super Linter] (not necessarily related to kernel code). You will need a container runtime to be installed for the generic lints to work. It's is considered okay to let GitHub's CI check the generic lints for you as the [GitHub Super Linter] is rather slow.

[//]: # (Links)

[Prettier]: https://prettier.io/
[Rust naming convention]: https://doc.rust-lang.org/1.0.0/style/style/naming/README.html

[rust-clippy]: https://github.com/rust-lang/rust-clippy
[cargo-audit]: https://github.com/rustsec/rustsec
[shellcheck]: https://github.com/koalaman/shellcheck
[GitHub Super Linter]: https://github.com/github/super-linter
