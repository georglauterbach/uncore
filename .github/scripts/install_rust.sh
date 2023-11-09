#! /usr/bin/env bash

# version       0.2.0
# executed by   during CI
# task          installs Rust

set -eE -u -o pipefail
shopt -s inherit_errexit

function exit_with_error() {
  echo "${1}" >&2
  exit 1
}

function usage() {
  local BOLD=$(echo -e '\e[1m')
  local UNDERLINED=$(echo -e '\e[4m')
  local RESET=$(echo -e '\e[0m')
  echo -e "Build or serve the documentation

${BOLD}${UNDERLINED}Usage:${RESET} .github/scripts/documentation.sh <COMMAND>

${BOLD}${UNDERLINED}Commands:${RESET}
  build                   Build the documentation (create HTML/CSS/JS)
  update_versions_json    (CI only) update a special versioning file
  serve                   Serve locally under 'http://127.0.0.1:8080'

"
}

function parse_options_and_arguments() {
  RUSTUP_PARAMETERS=('--quiet' '-y' '--default-toolchain' 'none' '--profile' 'minimal' '--no-update-default-toolchain')

  while [[ ${#} -gt 0 ]]; do
    case "${1}" in
      ( '--rustup-parameters' )
        [[ -n ${2:-} ]] || exit_with_error "No parameters provided after '--additional-rustup-parameters' option"
        readarray -t -d ' ' RUSTUP_PARAMETERS <<< "${2:-}"
        ;;

      ( * )
        usage
        exit_with_error "Unknown option(s) '${1}' ${2:+"and '${2}'"}"
        ;;
    esac

    shift 2
  done
}

function install_rust() {
  curl -sSfL 'https://sh.rustup.rs' | sh -s -- "${RUSTUP_PARAMETERS[@]}"

  rustup --version
  cargo version
  rustc --version
}

parse_options_and_arguments "${@}"
install_rust
