#! /bin/bash

# version       0.1.2
# executed by   shell scripts under scripts/
# task          performs script initialization

SCRIPT='initialization'

function basic_setup
{
  export LOG_LEVEL ROOT_DIRECTORY SCRIPT

  LOG_LEVEL=${LOG_LEVEL:-inf}
  GUESSED_ROOT_DIRECTORY="$(realpath -eL "$(dirname "$(realpath -eL "${0}")")/..")"
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

  notify 'deb' 'Performed basic script intialization'
}

function setup_kernel_environment
{
  notify 'deb' 'Changing into kernel directory'

  if ! cd "${ROOT_DIRECTORY}/kernel"
  then
    notify 'err' 'Could not change into kernel directory'
    exit 1
  fi

  notify 'deb' 'Setting kernel environment variables'

  export BUILD_TARGET COMPILATION_DATE_AND_TIME
  export GIT_REVISION_HEAD
  export KERNEL_BINARY KERNEL_VERSION
  export QEMU_KERNEL_BINARY
  export RUST_DEFAULT_TARGET RUST_TOOLCHAIN RUSTC_VERSION

  declare -g -a KERNEL_BUILD_FLAGS

  BUILD_TARGET='x86_64-unknown-uefi'
  COMPILATION_DATE_AND_TIME="$(date +'%H:%M, %d %b %Y')"
  GIT_REVISION_HEAD="$(git rev-parse --short HEAD)"
  KERNEL_VERSION="$(grep -m 1 'version*' Cargo.toml | cut -d '"' -f 2)"
  KERNEL_VERSION+=" (${GIT_REVISION_HEAD})"
  KERNEL_BINARY="target/${BUILD_TARGET}/debug/kernel.efi"
  KERNEL_BUILD_FLAGS+=('-Z')
  KERNEL_BUILD_FLAGS+=('build-std=core,compiler_builtins,alloc')
  KERNEL_BUILD_FLAGS+=('-Z')
  KERNEL_BUILD_FLAGS+=('build-std-features=compiler-builtins-mem')
  QEMU_KERNEL_BINARY='build/qemu/kernel/EFI/BOOT/BOOTX64.EFI'

  mkdir -p build/qemu/kernel/EFI/BOOT/ build/tests/kernel/EFI/BOOT/

  RUST_DEFAULT_TARGET="$(rustc -Vv | grep 'host:' | cut -d ' ' -f 2)"
  RUSTC_VERSION="$(rustc --version)" ; RUSTC_VERSION=${RUSTC_VERSION#rustc }
  RUST_TOOLCHAIN="$(rustup toolchain list | grep -E '(override)' | cut -d ' ' -f 1)"
}

function set_build_target
{
  if [[ -z ${1:-} ]]
  then
    notify 'err' 'Build target is empty'
    exit 1
  fi

  declare -a VALID_TARGETS
  VALID_TARGETS=(
    'aarch64'
    'i686'
    'x86_64'
  )

  for VALID_TARGET in "${VALID_TARGETS[@]}"
  do
    if [[ ${1} == "${VALID_TARGET}" ]]
    then
        export BUILD_TARGET KERNEL_BINARY

        BUILD_TARGET="${1}-unknown-uefi"
        KERNEL_BINARY="target/${BUILD_TARGET}/debug/kernel.efi"

        return 0
    fi
  done

  notify 'err' "Build target '${1}' is invalid"
  exit 1
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
        shift 1
        continue
        ;;
    esac
  done

  export -f set_build_target
  notify 'deb' 'Finished script intialization'
}

main "${@}"
