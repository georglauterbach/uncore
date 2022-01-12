# -----------------------------------------------
# ----  Just  -----------------------------------
# ----  https://github.com/casey/just  ----------
# -----------------------------------------------

set shell         := [ "bash", "-eu", "-o", "pipefail", "-c" ]
set dotenv-load   := false

export ROOT_DIRECTORY := justfile_directory()

BUILD_TOOL       := 'cargo'
KERNEL_DIRECTORY := ROOT_DIRECTORY + '/kernel'

# show a dedicated help message
help:
    #! /bin/bash
    RUST_INSTALLED=true

    if command -v rustc &>/dev/null
    then printf 'tools\n├── ' ; rustc --version
    else
        printf "tools\n├── 'rustc' not installed or in \$PATH\n"
        RUST_INSTALLED=false
    fi

    if command -v cargo &>/dev/null
    then printf '├── ' ; cargo --version
    else printf "├── 'cargo' not installed or in \$PATH\n"
    fi

    printf '└── just  %s\n\n' "$(cut -d ' ' -f 2 < <(just --version))"  

    if ${RUST_INSTALLED}
    then
        source scripts/lib/init.sh 'kernel'
        printf "kernel\n├── toolchain %s\n└── version   %s\n\n" \
            "${RUST_TOOLCHAIN}" "${KERNEL_VERSION}"
    fi

    just --list
    printf '\n'

# -----------------------------------------------
# ----  Build and Test  -------------------------
# -----------------------------------------------

# compile the kernel
@build target='':
    bash "{{ROOT_DIRECTORY}}/scripts/build.sh" {{target}}

# run the kernel for x86_64 in QEMU
@run *options: (build options)
    bash "{{ROOT_DIRECTORY}}/scripts/run_in_qemu.sh" {{options}}

# remove the kernel/target/ directory
@clean:
    cd {{KERNEL_DIRECTORY}} && {{BUILD_TOOL}} clean
    cd {{KERNEL_DIRECTORY}}/build/qemu/ && find . ! -name "grub.cfg" -delete

# run tests workspace members
test target='' test='':
    #! /bin/bash
    if [[ -z '{{target}}' ]]
    then
        bash "{{ROOT_DIRECTORY}}/scripts/test_kernel.sh" test {{test}}
    elif [[ {{target}} == '--help' ]]
    then
        bash "{{ROOT_DIRECTORY}}/scripts/test_kernel.sh" --help
    else
        bash "{{ROOT_DIRECTORY}}/scripts/test_kernel.sh" \
            --target {{target}}                          \
            test {{test}}
    fi

# -----------------------------------------------
# ----  Format and Lint  ------------------------
# -----------------------------------------------

# format the Rust code with rustfmt
@format:
    cd {{KERNEL_DIRECTORY}} && {{BUILD_TOOL}} fmt --message-format human

alias fmt := format

# lint against rustfmt and Clippy
@check *arguments: format
    - bash "{{ROOT_DIRECTORY}}/scripts/test_kernel.sh" {{arguments}} 'check'

# generically lint the whole code base
@lint linter='':
    - bash {{ROOT_DIRECTORY}}/scripts/lint.sh {{linter}}

# -----------------------------------------------
# ----  Documentation  --------------------------
# -----------------------------------------------

# build or serve the documentation
@docs action='':
    bash {{ROOT_DIRECTORY}}/scripts/documentation.sh {{action}}

alias doc := docs
