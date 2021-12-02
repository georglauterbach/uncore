# -----------------------------------------------
# ----  Just  -----------------------------------
# ----  https://github.com/casey/just  ----------
# -----------------------------------------------

set shell              := [ "bash", "-eu", "-o", "pipefail", "-c" ]
set dotenv-load        := false

export ROOT_DIRECTORY  := justfile_directory()
export TOOLCHAIN       := `tr -d '\n' < rust-toolchain`

BUILD_TOOL             := "cargo"
DATE                   := `date +'%Y-%m-%d'`
GIT_REVISION_HEAD      := `git rev-parse --short HEAD`
KERNEL_VERSION         := `grep -m 1 'version*' modules/kernel/Cargo.toml | cut -d '"' -f 2`

export VERSION         := KERNEL_VERSION + ' (' + GIT_REVISION_HEAD + ' ' + DATE + ')'

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

# build all (or a dedicated) package members
build package='""':
    #! /bin/bash

    cd {{ROOT_DIRECTORY}}/modules/
    if [[ -z {{package}} ]]
    then
        {{BUILD_TOOL}} build
    else
        {{BUILD_TOOL}} build --package {{package}}
    fi

# build the kernel with optimizations
release:
    #! /bin/bash

    cd {{ROOT_DIRECTORY}}/modules
    RUSTFLAGS="-C target-cpu=native" cargo build \
        --package kernel                         \
        --bin kernel                             \
        --release

# clean output produced during building and tetsing
@clean:
    cd {{ROOT_DIRECTORY}}/modules/ && {{BUILD_TOOL}} clean

# run all tests for all package members
test package='""':
    #! /bin/bash

    cd {{ROOT_DIRECTORY}}/modules/
    if [[ {{package}} == 'kernel' ]]
    then
        cd {{ROOT_DIRECTORY}}/modules/kernel/
        {{BUILD_TOOL}} test --package {{package}} --bin kernel
        {{BUILD_TOOL}} test --package {{package}} --lib
        {{BUILD_TOOL}} test --package {{package}} --test '*'
    elif [[ -n {{package}} ]]
    then
        cd {{package}}
        {{BUILD_TOOL}} test --package {{package}}
    else
        {{BUILD_TOOL}} test
    fi

# create the complete QEMU boot image
@bootimage:
    cd {{ROOT_DIRECTORY}}/modules/kernel/ && {{BUILD_TOOL}} bootimage

# -----------------------------------------------
# ----  Format and Lint  ------------------------
# -----------------------------------------------

# format all package members
format package='""':
    #! /bin/bash

    cd {{ROOT_DIRECTORY}}/modules/
    if [[ -z {{package}} ]]
    then
        {{BUILD_TOOL}} fmt --message-format human
    else
        {{BUILD_TOOL}} fmt --message-format human --package {{package}}
    fi

# lint against rustfmt and Clippy everwhere
check-format package='""':
    #! /bin/bash

    cd {{ROOT_DIRECTORY}}/modules/
    if [[ -z {{package}} ]]
    then
        {{BUILD_TOOL}} fmt --message-format human -- --check
        {{BUILD_TOOL}} clippy \
            --all-targets --all-features -- -D warnings
    else
        {{BUILD_TOOL}} fmt --package {{package}} \
            --message-format human -- --check
        {{BUILD_TOOL}} clippy --package {{package}} \
            --all-targets --all-features -- -D warnings
    fi

# lint against EditorConfig, ShellCheck and YAMLLint
@lint:
    - bash {{ROOT_DIRECTORY}}/scripts/lint/editorconfig.sh
    - bash {{ROOT_DIRECTORY}}/scripts/lint/shellcheck.sh
    - bash {{ROOT_DIRECTORY}}/scripts/lint/yamllint.sh

# -----------------------------------------------
# ----  Documentation  --------------------------
# -----------------------------------------------

# build or serve the documentation
@docs action='':
    bash {{ROOT_DIRECTORY}}/scripts/documentation.sh {{action}}
