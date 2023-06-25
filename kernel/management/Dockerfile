# syntax=docker.io/docker/dockerfile:1

ARG IMAGE_TAG=1.70.0-bookworm

FROM rust:${IMAGE_TAG}

ARG RUST_TOOLCHAIN=nightly-2023-06-25
ARG RUST_TARGET=riscv64imac-unknown-none-elf

RUN <<EOM
  # Rust toolchain releases can be found under
  # https://rust-lang.github.io/rustup-components-history/
  #
  # A reference for `rust-toolchain.toml` can be found under
  # https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
  #
  # ! Make sure to propagate version and component changes here to
  # `.github/workflows/kernel_tests.yml` and to
  # `.github/workflows/security.yml`!
  rustup toolchain add \
    --component cargo,llvm-tools-preview,rustc,rustfmt,rust-std,rust-src \
    ${RUST_TOOLCHAIN}

  rustup default ${RUST_TOOLCHAIN}
  rustup target add ${RUST_TARGET}
EOM

WORKDIR /uncore

ENTRYPOINT ["/usr/local/cargo/bin/cargo"]
CMD ["build", "--target", "/uncore/.cargo/targets/x86_64-unknown-uncore.json", "-Z", "build-std=core,compiler_builtins,alloc", "-Z", "build-std-features=compiler-builtins-mem"]
