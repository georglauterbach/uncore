# unCORE

[![Code Tests][badge::ci::code-tests]][badge::ci::code-tests::link]
[![Linting][badge::ci::linting]][badge::ci::linting::link]
[![Security Audit][badge::ci::security]][badge::ci::security::link]

## :page_with_curl: About

This project is

- an educational, modern [**operating system kernel**][www::wiki::operating-system-kernel]
- completely written in pure, idiomatic [**Rust**][www::homepage::rust] (and assembly where required),
- licensed under the [**GNU Public License v3 or later**][www::homepage::gpl-v3-license], except for those parts (lines of code from libraries used in this project) already licensed under other licenses,
- **documented** in its entirety: the code via [Doc comments][www::docs::rustdoc], the rest via [Markdown and GitHub Pages][docs::main-landing-page].

## :bulb: Getting Started

The [documentation][docs::main-landing-page] is hosted on [GitHub Pages][docs::github-pages] provides a dedicated "[Getting Started][docs::getting-started]" section. If you want to serve the documentation locally, you can run the following command from the repository root:

```bash
docker run --rm -it -v ./documentation:/docs -p 8080:8080 docker.io/squidfunk/mkdocs-material:9.4.8 serve --dev-addr 0.0.0.0:8080
```

[//]: # (Badges)

[badge::ci::code-tests]: https://github.com/georglauterbach/uncore/actions/workflows/code_tests_and_checks.yml/badge.svg?branch=master
[badge::ci::code-tests::link]: https://github.com/georglauterbach/uncore/actions/workflows/code_tests_and_checks.yml

[badge::ci::linting]: https://github.com/georglauterbach/uncore/actions/workflows/code_linting.yml/badge.svg?branch=master
[badge::ci::linting::link]: https://github.com/georglauterbach/uncore/actions/workflows/code_linting.yml

[badge::ci::security]: https://github.com/georglauterbach/uncore/actions/workflows/code_security.yml/badge.svg
[badge::ci::security::link]: https://github.com/georglauterbach/uncore/actions/workflows/code_security.yml

[//]: # (Links)

[www::wiki::operating-system-kernel]: https://en.wikipedia.org/wiki/Kernel_(operating_system)
[www::homepage::rust]: https://www.rust-lang.org/
[www::homepage::gpl-v3-license]: https://opensource.org/license/gpl-3-0/
[www::docs::rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[docs::main-landing-page]: https://georglauterbach.github.io/uncore/
[docs::github-pages]: https://pages.github.com/
[docs::getting-started]: https://georglauterbach.github.io/uncore/#getting-started
