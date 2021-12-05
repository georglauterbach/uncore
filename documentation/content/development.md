# Development

!!! quote "Why Rust?"
    Rust is **very performant** and **abstract**. And

    _Abstraction is not about vagueness, it is about being precise on a new semantic level._ –– **Edsger W. Dijkstra**

## Code Documentation & Testing

Code is linted against `clippy`, which tests for proper code documentation. Every `#!rust struct`, every `#!rust fn` and all other constructs need proper documentation. This documentation shall be concise and clear. Moreover, _unCORE_ tries to get a code coverage well above 90%. Make sure to write appropriate tests for the code you add.

There are various tests in place to check your code, not the least of which are lint tests. GitHub action runs the CI. You may lint parts of your code with `just lint` (to run lints not related to kernel code) or `just check-format` (to run lint tests directly related to kernel code). Especially `cargo clippy ...` can be nerve-wrecking, but running the command and the checking that your code code adheres to the norms ensures that the whole code-base can maintain a very high standard.

## Git Flow

The `master` branch contains the current stable code base. The `development` branch contains the latest changes, which may not be as stable as `master`. For every new version, there is a `version/X.Y.Z` branch, that is first merged into `development`, and then into `master`. The `X.Y.Z` follows the semantic versioning guidelines strictly! New features are being added through `feature/name-of-the-feature` branches. Hotfixes can be merged either into a `version/` branch or into `development`, but not into `master`. The following illustration shows the order of merging a feature.

``` TXT
feature/name-of-the-feature   ─── >   version/X.Y.Z   ─── >   development   ─── >   master
                                      │                       │
hotfix                    >   ────────┘           >   ────────┘
```

Please sign your commits with `GPG` / `PGP` so GitHub can verify them.

## Coding Style

When writing code, adhere to the style provided in the miscellaneous configuration files and to what is already written in all the files, even if this is not your preferred style. When altering files, look how it has been written and stay true to these design decisions. Make sure your IDE uses the provided `.editorconfig`.

Rust if formatted using `rustfmt`, which is installed with [Rust] itself. You can format your code using `just format` or `cargo fmt` in the repository modules containing [Rust] code. The style definition is found under `modules/.rustfmt.toml`. Make sure to adjust your style to the already present style.

When writing YAML, code is formatted with [Prettier]. When writing Bash scripts, make sure, especially with Bash, that you stick to the already present style!

## Miscellaneous

You may run any IDE you like, of course. We recommend [Visual Studio Code] or [NeoVIM].

[//]: # (Links)

[Rust]: https://www.rust-lang.org/
[Prettier]: https://prettier.io/
[Visual Studio Code]: https://code.visualstudio.com/
[NeoVIM]: https://neovim.io/
