#! /bin/bash

# version       0.1.1
# executed by   Just, manually or in CI
# task          runs kernel unit- and integration tests

source scripts/lib/init.sh 'kernel'
SCRIPT='tests'

function check_kernel
{
  notify 'inf' "Running 'cargo check'"
  cargo check                                     \
    --target "build/targets/${BUILD_TARGET}.json" \
    "${KERNEL_BUILD_FLAGS[@]}"

  notify 'inf' "Running formatting and clippy checks"
  cargo fmt --all --message-format human -- --check
  cargo clippy --lib --all-features -- -D warnings
}

function test_kernel
{
  # FIXME tests do not currently run
  notify 'abo' 'Unit- and integration tests are unimplemented'
  return 1

  local INTEGRATION_TEST
  INTEGRATION_TEST="${1:-}"

  if [[ -z ${INTEGRATION_TEST} ]]
  then
  notify 'inf' 'Running unit- and integration tests'
    cargo test --tests                              \
      --target "build/targets/${BUILD_TARGET}.json" \
      "${KERNEL_BUILD_FLAGS[@]}"
  else
    notify 'inf' "Running integration test '${INTEGRATION_TEST}'"
    cargo test --test "${INTEGRATION_TEST}"       \
      --target "build/targets/${BUILD_TARGET}.json" \
      "${KERNEL_BUILD_FLAGS[@]}"
  fi

  # shellcheck disable=SC2181
  if [[ ${?} -eq 0 ]]
  then
    notify 'suc' 'Tests passed'
  else
    notify 'war' 'Tests did not pass'
  fi
}

function usage
{
  cat << "EOM" 
TEST_KERNEL.SH(1)

SYNOPSIS
    ./scripts/test_kernel.sh [ OPTION... ] < ACTION... >
    just check               [ OPTION... ] 
    just test                [ [ '--help' ] | [ TARGET ] [ TEST ] ]

OPTIONS
    --help           Show this help message
    --is-ci          specifies that this script invocation is performed during CI
    --target TARGET  specify target triple to use when building and running the kernel

ACTIONS
    check            run linter checks
    test [ TEST ]    run unit- and integration tests or the TEST integration test

EOM
}

function main
{
  trap '' ERR ; set +e

  if [[ -z ${1:-} ]]
  then
    notify 'abo' 'No action specified'
    exit 1
  fi

  while [[ -n ${1:-} ]]
  do
    case "${1:-}" in
      ( '--help' )
        usage
        exit 0
        ;;

      ( '--is-ci' )
        set -e
        shift 1
        ;;

      ( '--target' )
        set_build_target "${2:-}"
        shift 2
        ;;

      ( 'check' )
        check_kernel
        shift 1
        ;;
    
      ( 'test' )
        test_kernel "${2:-}"
        [[ -n ${2:-} ]] && shift 1
        shift 1
        ;;

      ( * )
        notify 'abo' "'${1}' is invalid (run with --help to get more information)"
        exit 1
        ;;
    esac
  done
}

main "${@}"
