#! /bin/bash

# version       0.2.0
# executed by   shell scripts under scripts/
# task          performs script initialization

# shellcheck source=scripts/libbash/load
source "$(dirname "${BASH_SOURCE[0]}")/libbash/load" 'errors' 'log' "${@}"
SCRIPT='uncore initialization@bash'

function basic_setup
{
  notify 'tra' 'Starting basic script intialization'

  local GUESSED_ROOT_DIRECTORY SOURCE
  export ROOT_DIRECTORY

  if [[ -n ${BASH_SOURCE[0]+set} ]]
  then  
    SOURCE="$(realpath -eL "${BASH_SOURCE[0]}")"
  else
    SOURCE="$(realpath -eL "${0}")"
  fi

  GUESSED_ROOT_DIRECTORY="$(realpath -eL "$(dirname "${SOURCE}")/..")"
  ROOT_DIRECTORY=${ROOT_DIRECTORY:-${GUESSED_ROOT_DIRECTORY}}

  notify 'tra' 'Performed basic script intialization'
}

function setup_kernel_environment
{
  notify 'tra' 'Changing into kernel directory'

  if ! cd "${ROOT_DIRECTORY}/kernel"
  then
    notify 'err' 'Could not change into kernel directory'
    exit 1
  fi

  mkdir -p                     \
    out/qemu/kernel/EFI/BOOT/  \
    out/qemu/boot_output/      \
    out/tests/kernel/EFI/BOOT/ \
    out/tests/boot_output/

  notify 'deb' 'Setting kernel environment variables'

  export                         \
    COMPILATION_DATE_AND_TIME    \
    KERNEL_VERSION               \
    RUST_DEFAULT_TARGET          \
    RUST_TOOLCHAIN               \
    RUSTC_VERSION

  COMPILATION_DATE_AND_TIME="$(date +'%H:%M, %d %b %Y')"
  GIT_REVISION_HEAD="$(git rev-parse --short HEAD)"
  KERNEL_VERSION="$(grep -m 1 'version*' Cargo.toml | cut -d '"' -f 2)"
  KERNEL_VERSION+=" (${GIT_REVISION_HEAD})"
  RUST_DEFAULT_TARGET="$(rustc -Vv | grep 'host:' | cut -d ' ' -f 2)"
  RUST_TOOLCHAIN="$(grep 'channel' rust-toolchain.toml | cut -d ' ' -f 3 | tr -d '"')"
  RUSTC_VERSION="$(rustc --version)" ; RUSTC_VERSION=${RUSTC_VERSION#rustc }

  notify 'tra' 'Finished kernel environment setup'
}

basic_setup
setup_kernel_environment
