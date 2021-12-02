---
hide:
  - navigation
---

# Development

!!! quote "Why Rust?"
    Rust is **very performant** and **abstract**. And we know that

    _Abstraction is not about vagueness, it is about being precise on a new semantic level._ –– **Edsger W. Dijkstra**

## Miscellaneous

You may run any IDE you like, of course. We recommend [Visual Studio Code] or [NeoVIM].

## Code Documentation & Testing

TODO.

## Coding Style

When writing code, adhere to the style provided in the miscellaneous configuration files and to what is already written in all the files, even if this is not your preferred style. When altering files, look how it has been written and stay true to these design decisions. Make sure your IDE uses the provided `.editorconfig`.

Rust if formatted using `rustfmt`, which is installed with [Rust] itself. You can format your code using `just format` or `cargo fmt` in the repository modules containing [Rust] code. The style definition is found in `.rustfmt.toml` in the repository's root. Make sure to adjust your style to the already present style.

When writing YAML, code is formatted with [Prettier]. When writing Bash scripts, make sure, especially with Bash, that you stick to the already present style!

[//]: # (Links)

[Visual Studio Code]: https://code.visualstudio.com/
[NeoVIM]: https://neovim.io/
[Rust]: https://www.rust-lang.org/
[Prettier]: https://prettier.io/
