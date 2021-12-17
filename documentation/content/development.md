# Development

!!! quote "Why Rust?"
    Rust is **very performant** and **abstract**. And

    _Abstraction is not about vagueness, it is about being precise on a new semantic level._ –– **Edsger W. Dijkstra**

## Code Documentation & Testing

Code is linted against `clippy`, which tests for proper code documentation. Every `#!rust struct`, every `#!rust fn` and all other constructs need proper documentation. This documentation shall be concise and clear. Moreover, _unCORE_ tries to get a code coverage well above 90%. Make sure to write appropriate tests for the code you add.

There are various tests in place to check your code, not the least of which are lint tests. GitHub action runs the CI. You may lint parts of your code with `just lint` (to run lints not related to kernel code) or `just check-format` (to run lint tests directly related to kernel code). Especially `cargo clippy ...` can be nerve-wrecking, but running the command and the checking that your code code adheres to the norms ensures that the whole code-base can maintain a very high standard.

## Git Flow

Please sign your commits with `GPG` / `PGP` so GitHub can verify them. The `master` branch contains the current stable code base. The `development` branch contains the latest changes, which may not be as stable as `master`. For every new version, there is a `version/X.Y.Z` branch, that is first merged into `development`, and then into `master`. The `X.Y.Z` follows the semantic versioning guidelines strictly! New features are being added through `feature/name-of-the-feature` branches. Hotfixes can be merged either into a `version/` branch or into `development`, but not into `master`. The following illustration shows the order of merging a feature.

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

Rust if formatted using `rustfmt`, which is installed with [Rust] itself. You can format your code using `just format` or `cargo fmt` in the repository modules containing [Rust] code. The style definition is found under `kernel/.rustfmt.toml`. Make sure to adjust your style to the already present style. The [Rust naming convention] is strictly adhered to.

Crate-level / global `lib.rs` (or in case of the kernel, also `main.rs`) are formatted in a special way. We start by declaring crate-level attributes and crate-level documentation, then modules and exports and last but not least, global functions. You may want to have a look at the `helper/` module's `lib.rs` for a concise example.

We want to ensure, at all cost, that code in this project becomes as unreadable as some Linux kernel code. This has nothing to do with formatting taste, but with problems inherent to C and how programmers are used to writing C.

## Miscellaneous

You may run any IDE you like, of course. We recommend [Visual Studio Code] or [NeoVIM]. We do not make any assumptions about the style of working that you prefer most - use what suits you best.

[//]: # (Links)

[Rust]: https://www.rust-lang.org/
[Prettier]: https://prettier.io/
[Rust naming convention]: https://doc.rust-lang.org/1.0.0/style/style/naming/README.html
[Visual Studio Code]: https://code.visualstudio.com/
[NeoVIM]: https://neovim.io/
