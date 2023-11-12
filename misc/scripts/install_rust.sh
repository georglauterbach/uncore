#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

RUSTUP_PARAMETERS=('--quiet' '-y' '--default-toolchain' 'none' '--profile' 'minimal' '--no-update-default-toolchain')
[[ ${#} -eq 0 ]] || readarray -t -d ' ' RUSTUP_PARAMETERS < <(printf "%s" "${*}")

curl -sSfL 'https://sh.rustup.rs' | sh -s -- "${RUSTUP_PARAMETERS[@]}"
