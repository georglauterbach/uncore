#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

function exit_with_error() { echo "${1}" >&2 ; exit 1 ; }

function parse_options_and_arguments() {
  RUSTUP_PARAMETERS=('--quiet' '-y' '--default-toolchain' 'none' '--profile' 'minimal' '--no-update-default-toolchain')

  while [[ ${#} -gt 0 ]]; do
    case "${1}" in
      ( '--rustup-parameters' )
        [[ -n ${2:-} ]] || exit_with_error "No parameters provided after '--additional-rustup-parameters' option"
        readarray -t -d ' ' RUSTUP_PARAMETERS <<< "${2:-}"
        ;;

      ( * ) exit_with_error "Unknown option(s) '${1}' ${2:+"and '${2}'"}" ;;
    esac

    shift 2
  done
}

function install_rust() {
  curl -sSfL 'https://sh.rustup.rs' | sh -s -- "${RUSTUP_PARAMETERS[@]}"
  rustup --version ; cargo version ; rustc --version ;
}

parse_options_and_arguments "${@}"
install_rust
