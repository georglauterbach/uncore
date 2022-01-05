# -----------------------------------------------
# ----  Just  -----------------------------------
# ----  https://github.com/casey/just  ----------
# -----------------------------------------------

set shell         := [ "bash", "-eu", "-o", "pipefail", "-c" ]
set dotenv-load   := false

DATE              := `date +'%Y-%m-%d'`
GIT_REVISION_HEAD := `git rev-parse --short HEAD`
DEFAULT_TARGET    := `rustc -Vv | grep 'host:' | cut -d ' ' -f 2`
BUILD_TOOL        := 'cargo'

export ROOT_DIRECTORY := justfile_directory()
export KERNEL_VERSION := `grep -m 1 'version*' kernel/Cargo.toml | cut -d '"' -f 2`
export VERSION        := KERNEL_VERSION + ' (' + GIT_REVISION_HEAD + ' ' + DATE + ')'

KERNEL_DEFAULT_TARGET_PATH := 'build/targets/x86_64-unknown-none.json'
KERNEL_BUILD_FLAGS_1       := ' -Z build-std=core,compiler_builtins,alloc'
KERNEL_BUILD_FLAGS_2       := ' -Z build-std-features=compiler-builtins-mem'
KERNEL_BUILD_FLAGS         := KERNEL_BUILD_FLAGS_1 + KERNEL_BUILD_FLAGS_2
KERNEL_DIRECTORY           := ROOT_DIRECTORY + '/kernel'

# show a dedicated help message
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

    TOOLCHAIN=$(cd {{KERNEL_DIRECTORY}} && rustup toolchain list | grep 'override')
    [[ -z ${TOOLCHAIN} ]] && TOOLCHAIN='not properly set in the kernel directory'

    printf '└── just  %s\n' "$(cut -d ' ' -f 2 < <(just --version))"  
    printf "\nkernel\n├── toolchain %s\n└── version   %s\n\n" \
        "${TOOLCHAIN}" "{{VERSION}}"

    just --list
    printf '\n'

# -----------------------------------------------
# ----  Build and Test  -------------------------
# -----------------------------------------------

# compile the kernel
@build target='':
    bash "{{ROOT_DIRECTORY}}/scripts/build.sh" {{target}}

# run the kernel for x86_64 in QEMU
@run target='': (build target)
    bash "{{ROOT_DIRECTORY}}/scripts/run_in_qemu.sh"

# remove the kernel/target/ directory
@clean:
    cd {{KERNEL_DIRECTORY}} && {{BUILD_TOOL}} clean

# FIXME tests do not currently run
# run tests workspace members
test test='':
    #! /bin/bash

    echo "CURRENTLY TEST DO NOT WORK BECAUSE WE NEED TO RUN THEM IN QEMU" >&2
    exit 1

    cd {{KERNEL_DIRECTORY}}

    # --tests runs all tests, i.e. the kernel library (`lib.rs`)
    # effectivly running all unit-tests, the kernel main binary
    # (`main.rs`) effectively running zero unit tests and all
    # integration tests (under `tests/`)

    if [[ -z "{{test}}" ]]
    then
        {{BUILD_TOOL}} test --tests               \
            --target {{KERNEL_DEFAULT_TARGET_PATH}} \
            {{KERNEL_BUILD_FLAGS}}
    else
        {{BUILD_TOOL}} test --test {{test}}       \
            --target {{KERNEL_DEFAULT_TARGET_PATH}} \
            {{KERNEL_BUILD_FLAGS}}
    fi

    printf '\nINFO    | Tests passed.\n'

# -----------------------------------------------
# ----  Format and Lint  ------------------------
# -----------------------------------------------

# format the Rust code with rustfmt
@format:
    cd {{KERNEL_DIRECTORY}} && {{BUILD_TOOL}} fmt --message-format human

alias fmt := format

# lint against rustfmt and Clippy
check:
    #! /bin/bash

    cd {{KERNEL_DIRECTORY}}

    {{BUILD_TOOL}} check                                \
        --target {{KERNEL_DEFAULT_TARGET_PATH}} \
        {{KERNEL_BUILD_FLAGS}}

    {{BUILD_TOOL}} fmt --all --message-format human -- --check
    {{BUILD_TOOL}} clippy --lib --all-features -- -D warnings
    {{BUILD_TOOL}} clippy --package init --all-features -- -D warnings

# generically lint the whole code base
@lint:
    - bash {{ROOT_DIRECTORY}}/scripts/lint.sh

# -----------------------------------------------
# ----  Documentation  --------------------------
# -----------------------------------------------

# build or serve the documentation
@docs action='':
    bash {{ROOT_DIRECTORY}}/scripts/documentation.sh {{action}}

alias doc := docs
