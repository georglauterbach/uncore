# -----------------------------------------------
# ----  Just  -----------------------------------
# ----  https://github.com/casey/just  ----------
# -----------------------------------------------

set shell             := [ "bash", "-eu", "-o", "pipefail", "-c" ]
set dotenv-load       := false

export ROOT_DIRECTORY := justfile_directory()
CARGO                 := 'cargo'
KERNEL_DIRECTORY      := ROOT_DIRECTORY + '/kernel'

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
@build *arguments:
	bash "{{ROOT_DIRECTORY}}/scripts/build.sh" {{arguments}}

# run the kernel in QEMU
run *arguments: (build arguments)
	#! /bin/bash

	bash "{{ROOT_DIRECTORY}}/scripts/run_in_qemu.sh" {{arguments}}
	EXIT_CODE=${?}
	[[ ${EXIT_CODE} -gt 1 ]] && exit $((EXIT_CODE - 1)) 
	exit 0

# remove the kernel/target/ directory
@clean:
	cd {{KERNEL_DIRECTORY}} && {{CARGO}} clean
	rm -rf {{KERNEL_DIRECTORY}}/out/

# run tests workspace members
@test *arguments:
	- bash "{{ROOT_DIRECTORY}}/scripts/test_kernel.sh" {{arguments}} test

# -----------------------------------------------
# ----  Format and Lint  ------------------------
# -----------------------------------------------

# format the Rust code with rustfmt
@format:
	cd {{KERNEL_DIRECTORY}} && {{CARGO}} fmt --message-format human

alias fmt := format

# lint against rustfmt and Clippy
@check *arguments: format
	- bash "{{ROOT_DIRECTORY}}/scripts/test_kernel.sh" {{arguments}} check

# generically lint the whole code base
@lint *arguments:
	- bash "{{ROOT_DIRECTORY}}/scripts/lint.sh" {{arguments}}

# -----------------------------------------------
# ----  Documentation  --------------------------
# -----------------------------------------------

# build or serve the documentation
@docs action='serve':
	bash {{ROOT_DIRECTORY}}/scripts/documentation.sh {{action}}

@cargo_doc arguments='':
	cd {{KERNEL_DIRECTORY}} && cargo doc --lib --document-private-items {{arguments}}

alias doc := docs
alias cargo_docs := cargo_doc
