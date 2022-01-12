#! /bin/bash

# version       0.1.1
# executed by   shell scripts under scripts/
# task          performs script initialization
# parameters    ${1} - 'kernel' for kernel setup scripts (optional)
#               ${2} - kernel build target (optional)

SCRIPT='initialization'

function basic_setup
{
  export LOG_LEVEL ROOT_DIRECTORY SCRIPT

  LOG_LEVEL=${LOG_LEVEL:-inf}
  GUESSED_ROOT_DIRECTORY="$(realpath -e -L "$(dirname "$(realpath -e -L "${0}")")/..")"
  ROOT_DIRECTORY=${ROOT_DIRECTORY:-${GUESSED_ROOT_DIRECTORY}}

  if ! cd "${ROOT_DIRECTORY}"
  then
    printf "%-12s \e[31mABORT  \e[0m %s%s\n"             \
      "${SCRIPT:-${0}}"                                 \
      'Could not change into repository root directory' \
      "${ROOT_DIRECTORY}" >&2
    exit 1
  fi

  source scripts/lib/logs.sh
  source scripts/lib/errors.sh

  notify 'tra' 'Performed basic script intialization'
}

function setup_kernel_environment
{
  notify 'tra' 'Changing into kernel directory'

  if ! cd "${ROOT_DIRECTORY}/kernel"
  then
    notify 'abo' 'Could not change into kernel directory'
    exit 1
  fi

  notify 'tra' 'Setting kernel environment variables'

  export BUILD_TARGET COMPILATION_DATE_AND_TIME
  export GIT_REVISION_HEAD
  export KERNEL_BINARY KERNEL_VERSION
  export QEMU_KERNEL_BINARY
  export RUST_DEFAULT_TARGET RUST_TOOLCHAIN RUSTC_VERSION

  declare -g -a KERNEL_BUILD_FLAGS

  BUILD_TARGET='x86_64-unknown-none'
  COMPILATION_DATE_AND_TIME="$(date +'%H:%M, %d %b %Y')"
  GIT_REVISION_HEAD="$(git rev-parse --short HEAD)"
  KERNEL_VERSION="$(grep -m 1 'version*' Cargo.toml | cut -d '"' -f 2)"
  KERNEL_VERSION+=" (${GIT_REVISION_HEAD})"
  KERNEL_BINARY="target/${BUILD_TARGET}/debug/kernel"
  KERNEL_BUILD_FLAGS+=('-Z')
  KERNEL_BUILD_FLAGS+=('build-std=core,compiler_builtins,alloc')
  KERNEL_BUILD_FLAGS+=('-Z')
  KERNEL_BUILD_FLAGS+=('build-std-features=compiler-builtins-mem')
  QEMU_KERNEL_BINARY='build/qemu/kernel.bin'

  RUST_DEFAULT_TARGET="$(rustc -Vv | grep 'host:' | cut -d ' ' -f 2)"
  RUSTC_VERSION="$(rustc --version)" ; RUSTC_VERSION=${RUSTC_VERSION#rustc }
  RUST_TOOLCHAIN="$(rustup toolchain list | grep -E '(override)' | cut -d ' ' -f 1)"
}

function set_build_target
{
  export BUILD_TARGET KERNEL_BINARY

  BUILD_TARGET="${1}"
  KERNEL_BINARY="target/${BUILD_TARGET}/debug/kernel"

  if [[ -z ${BUILD_TARGET} ]]
  then
    notify 'abo' 'Specified build target is empty'
    exit 1
  fi

  if [[ ! -f "build/targets/${BUILD_TARGET}.json"  ]] \
  && [[ ! -f "kernel/build/targets/${BUILD_TARGET}.json"  ]]
  then
    notify 'abo'                                                     \
      "The build target '${BUILD_TARGET}' does not seem to be valid" \
      "(is it in the 'kernel/build/targets/' directory?)"
    exit 1
  fi

  notify 'inf' "Set build target to '${BUILD_TARGET}'"
}

function main
{
  basic_setup

  while [[ -n ${1:-} ]]
  do
    case "${1:-}" in
      ( 'kernel' )
        setup_kernel_environment
        shift 1
        ;;
      
      ( * )
        notify 'abo' "Option '${1:-}' is invalid"
        exit 1
        ;;
    esac
  done

  export -f set_build_target
  notify 'tra' 'Finished script intialization'
}

main "${@}"
