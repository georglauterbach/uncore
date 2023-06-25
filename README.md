# unCORE

[![CI][badge::ci::kernel-code-tests]][badge::ci::kernel-code-tests::link] [![CI Linting][badge::ci::security]][badge::ci::security::link] [![CI Linting][badge::ci::linting]][badge::ci::linting::link]

## :page_with_curl: About

This project is

- an educational, modern [**operating system kernel**][www::wiki::operating-system-kernel];
- completely written in pure, idiomatic [**Rust**][www::homepage::rust] (and assembly where required);
- licensed under the [**MIT License**][www::homepage::mit-license], except for those parts (lines of code from libraries used in this project) already licensed under other licenses;
- **documented** in its entirety: the code via [Doc Comments][www::docs::rustdoc], the rest via [Markdown and GitHub Pages][docs::main-landing-page];
- focused around [a few simple **concepts and principles**][docs::principles].

## :bulb: Getting Started

The [documentation][docs::main-landing-page] provides a dedicated "[Getting Started][docs::getting-started]" section.

[//]: # (Badges)

[badge::ci::linting]: https://img.shields.io/github/actions/workflow/status/georglauterbach/uncore/linting.yml?branch=master&label=CI%20-%20Linting&logo=github&logoColor=white&style=for-the-badge
[badge::ci::linting::link]: https://github.com/georglauterbach/uncore/actions/workflows/linting.yml

[badge::ci::security]: https://img.shields.io/github/actions/workflow/status/georglauterbach/uncore/security.yml?branch=master&label=CI%20-%20Security%20Audit&logo=github&logoColor=white&style=for-the-badge
[badge::ci::security::link]: https://github.com/georglauterbach/uncore/actions/workflows/security.yml

[badge::ci::kernel-code-tests]: https://img.shields.io/github/actions/workflow/status/georglauterbach/uncore/kernel_tests.yml?branch=master&label=CI%20-%20Kernel%20Code%20Tests&logo=github&logoColor=white&style=for-the-badge
[badge::ci::kernel-code-tests::link]: https://github.com/georglauterbach/uncore/actions/workflows/kernel-tests.yml

[//]: # (Links)

[www::wiki::operating-system-kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[www::homepage::rust]: https://www.rust-lang.org/
[www::homepage::mit-license]: https://opensource.org/license/mit/
[www::docs::rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[docs::main-landing-page]: https://georglauterbach.github.io/uncore/edge/
[docs::principles]: https://georglauterbach.github.io/uncore/edge/#principles
[docs::getting-started]: https://georglauterbach.github.io/uncore/edge/#getting-started
