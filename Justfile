# -----------------------------------------------
# ----  Just  -----------------------------------
# ----  https://github.com/casey/just  ----------
# -----------------------------------------------

set shell              := [ "bash", "-eu", "-o", "pipefail", "-c" ]
set dotenv-load        := false

DATE                   := `date +'%Y-%m-%d'`
GIT_REVISION_HEAD      := `git rev-parse --short HEAD`
KERNEL_VERSION         := `grep -m 1 'version*' kernel/Cargo.toml | cut -d '"' -f 2`

export ROOT_DIRECTORY  := justfile_directory()
export TOOLCHAIN       := `tr -d '\n' < kernel/rust-toolchain`
export VERSION         := KERNEL_VERSION + ' (' + GIT_REVISION_HEAD + ' ' + DATE + ')'

BUILD_TOOL             := 'cargo'
BOOTIMAGE_BUILD_TARGET := `rustc -Vv | grep 'host:' | cut -d ' ' -f 2`
KERNEL_BUILD_FLAGS_1   := ' -Z build-std=core,compiler_builtins,alloc'
KERNEL_BUILD_FLAGS_2   := ' -Z build-std-features=compiler-builtins-mem'
KERNEL_BUILD_FLAGS     := KERNEL_BUILD_FLAGS_1 + KERNEL_BUILD_FLAGS_2

export KERNEL_BUILD_TARGET := `printf "${TARGET:-x86_64-unknown-none}"`
KERNEL_BUILD_TARGET_PATH   := ROOT_DIRECTORY + '/kernel/targets/' + KERNEL_BUILD_TARGET + '.json'

# show this help message
help:
    #! /bin/bash

    if command -v rustc &>/dev/null
    then printf 'tools\n├── ' ; rustc --version
    else printf "tools\n├── 'rustc' not installed or in \$PATH\n"
    fi

    if command -v cargo &>/dev/null
    then printf '├── ' ; cargo --version
    else printf "├── 'cargo' not installed or in \$PATH\n"
    fi

    printf '└── just  %s\n' "$(cut -d ' ' -f 2 < <(just --version))"  
    printf "\nkernel\n├── toolchain %s\n└── version   %s\n\n" \
        "{{TOOLCHAIN}}" "{{VERSION}}"

    just --list
    printf '\n'

# -----------------------------------------------
# ----  Build and Test  -------------------------
# -----------------------------------------------

# compile the kernel
@build release='':
    RELEASE="{{release}}" && \
    just -- _build_kernel "${RELEASE:+--release}"

# create a bootable image
@build_image release='':
    RELEASE="{{release}}" && \
    just -- _use_bootimage "${RELEASE:+release}" --no-run

# run the kernel in QEMU
run: _use_bootimage

# compile the kernel
@_build_kernel release='':
    cd {{ROOT_DIRECTORY}}/kernel/ &&     \
    {{BUILD_TOOL}} build {{release}}     \
        --target {{KERNEL_BUILD_TARGET_PATH}} \
        {{KERNEL_BUILD_FLAGS}}

# use the bootloader tool to build or run the kernel
_use_bootimage release='' no_run='':
    #! /bin/bash

    RELEASE="{{release}}"

    just -- _build_kernel ${RELEASE:+--release} || exit ${?}
    cd {{ROOT_DIRECTORY}}/kernel/

    {{BUILD_TOOL}} run                      \
        --package boot                      \
        --target {{BOOTIMAGE_BUILD_TARGET}} \
        ${RELEASE:+--release}               \
        --                                  \
        target/{{KERNEL_BUILD_TARGET}}/${RELEASE:-debug}/kernel {{no_run}}

# remove the kernel/target/ directory
@clean:
    cd {{ROOT_DIRECTORY}}/kernel/ && {{BUILD_TOOL}} clean

# run tests workspace members
test test='':
    #! /bin/bash

    cd {{ROOT_DIRECTORY}}/kernel/

    # --tests runs all tests, i.e. the kernel library (`lib.rs`)
    # effectivly running all unit-tests, the kernel main binary
    # (`main.rs`) and all integration tests (under `tests/`)

    if [[ -z "{{test}}" ]]
    then
        {{BUILD_TOOL}} test --tests               \
            --target {{KERNEL_BUILD_TARGET_PATH}} \
            {{KERNEL_BUILD_FLAGS}}
    else
        {{BUILD_TOOL}} test --test {{test}}       \
            --target {{KERNEL_BUILD_TARGET_PATH}} \
            {{KERNEL_BUILD_FLAGS}}
    fi

    printf '\nTests passed.\n'

# -----------------------------------------------
# ----  Format and Lint  ------------------------
# -----------------------------------------------

# format the Rust code with rustfmt
@format:
    cd {{ROOT_DIRECTORY}}/kernel/ \
        && {{BUILD_TOOL}} fmt --message-format human

alias fmt := format

# lint against rustfmt and Clippy
check:
    #! /bin/bash

    cd {{ROOT_DIRECTORY}}/kernel/

    {{BUILD_TOOL}} check                      \
        --target {{KERNEL_BUILD_TARGET_PATH}} \
        {{KERNEL_BUILD_FLAGS}}

    {{BUILD_TOOL}} fmt --all --message-format human -- --check
    {{BUILD_TOOL}} clippy --lib --all-features -- -D warnings
    {{BUILD_TOOL}} clippy --package boot --all-features -- -D warnings

# generically lint the whole code base
@lint:
    - bash {{ROOT_DIRECTORY}}/scripts/lint.sh

# -----------------------------------------------
# ----  Documentation  --------------------------
# -----------------------------------------------

# build or serve the documentation
@docs action='':
    bash {{ROOT_DIRECTORY}}/scripts/documentation.sh {{action}}
