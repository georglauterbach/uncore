#! /bin/bash

# version       0.2.0
# executed by   Just, manually or in CI
# task          runs kernel unit- and integration tests

# shellcheck source=scripts/lib/init.sh
source "$(dirname "$(realpath -eL "${0}")")/lib/init.sh" 'kernel'
SCRIPT='tests'

function check_kernel
{
  notify 'inf' "Running 'cargo check'"
  cargo check --target "${BUILD_TARGET}" "${KERNEL_BUILD_FLAGS[@]}"

  notify 'inf' "Running formatting and clippy checks"
  cargo fmt --all --message-format human -- --check
  cargo clippy --lib --all-features -- -D warnings
  cargo clippy --package test_runner --all-features -- -D warnings
}

function test_kernel
{
  local EXIT_CODE
  declare -a COMMAND
  COMMAND=(
    'cargo' 'test'
    '--target' "${BUILD_TARGET}"
    "${KERNEL_BUILD_FLAGS[@]}"
  )

  if [[ -z ${INTEGRATION_TEST:-} ]]
  then
    notify 'inf' 'Running all unit- and integration tests'
    "${COMMAND[@]}" --tests
  elif [[ ${INTEGRATION_TEST} == 'lib' ]]
  then
    notify 'inf' "Running only unit tests"
    "${COMMAND[@]}" --lib
  else
    notify 'inf' "Running integration test '${INTEGRATION_TEST}'"
    "${COMMAND[@]}" --test "${INTEGRATION_TEST}"
  fi

  EXIT_CODE=${?}

  if [[ ${EXIT_CODE} -eq 0 ]]
  then
    notify 'inf' 'Tests passed'
  else
    notify 'war' 'Tests did not pass' "(exit code was ${EXIT_CODE})"
    exit 1
  fi
}

function usage
{
  cat << "EOM" 
TEST_KERNEL.SH(1)

SYNOPSIS
    ./scripts/test_kernel.sh [ OPTION... ] < ACTION... >
    just < check | test >    [ OPTION... ] 

OPTIONS
    --help           Show this help message
    --is-ci          specifies that this script invocation is performed during CI
    --target TARGET  specify target triple to use when building and running the kernel
    --test TARGET    speficy the test when running integration tests

ACTIONS
    check            run linter checks
    test             run unit- and integration tests or the TEST integration test

EOM
}

function main
{
  if [[ -z ${1:-} ]]
  then
    notify 'err' 'No action specified'
    exit 1
  fi

  trap '' ERR ; set +e
  local ACTION=''
  export INTEGRATION_TEST

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
      
      ( '--test' )
        if [[ -z ${2:-} ]]
        then
          notify 'err' 'Provided test flag but no test was given afterwards'
          exit 1
        fi

        INTEGRATION_TEST="${2}"
        shift 2
        ;;

      ( 'check' )
        ACTION='check'
        shift 1
        ;;
    
      ( 'test' )
        ACTION='test'
        shift 1
        ;;

      ( * )
        notify 'err' "'${1}' is invalid (run with --help to get more information)"
        exit 1
        ;;
    esac
  done

  case "${ACTION}" in
    ( 'check' )
      check_kernel
      ;;

    ( 'test' )
      test_kernel
      ;;

    ( * )
      notify 'err' 'No action provided (run with --help to get more information)'
      exit 1
      ;;
  esac
}

main "${@}"
