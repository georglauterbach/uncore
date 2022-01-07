#! /bin/bash

# version       0.1.0
# executed by   Just, manually or in CI
# task          builds the kernel

SCRIPT='build'
__BASH_LOG_LEVEL=${__BASH_LOG_LEVEL:-inf}

GUESSED_ROOT_DIRECTORY="$(realpath -e -L "$(dirname "$(realpath -e -L "${0}")")/..")"
ROOT_DIRECTORY=${ROOT_DIRECTORY:-${GUESSED_ROOT_DIRECTORY}}

if ! cd "${ROOT_DIRECTORY}" &>/dev/null
then
  echo "ABORT Could not change into kernel directory '${ROOT_DIRECTORY}'"
  exit 1
fi

source scripts/lib/errors.sh
source scripts/lib/logs.sh

if ! cd "${ROOT_DIRECTORY}/kernel"
then
  notify 'abo' 'Could not change into kernel directory (root directory wrong?)'
  exit 1
fi

function build_kernel
{
  local KERNEL_BINARY RUSTC_VERSION RUST_TOOLCHAIN COMPILATION_DATE_AND_TIME
  local KERNEL_BUILD_TARGET QEMU_KERNEL_BINARY

  QEMU_KERNEL_BINARY='build/qemu/kernel.bin'
  KERNEL_BUILD_TARGET="${1:-x86_64-unknown-none}"
  KERNEL_BINARY="target/${KERNEL_BUILD_TARGET}/debug/kernel"
  RUSTC_VERSION="$(rustc --version)"
  RUST_TOOLCHAIN="$(rustup toolchain list | grep -E '(override)' | cut -d ' ' -f 1)"
  COMPILATION_DATE_AND_TIME="$(date +'%H:%M, %d %b %Y')"

  notify 'inf' "Compiling kernel for target '${KERNEL_BUILD_TARGET}'"

  if ! VERSION="${VERSION:-testing}"                         \
    BUILD_TARGET="${KERNEL_BUILD_TARGET}"                    \
    RUSTC_VERSION="${RUSTC_VERSION}"                         \
    RUST_TOOLCHAIN="${RUST_TOOLCHAIN}"                       \
    COMPILATION_DATE_AND_TIME="${COMPILATION_DATE_AND_TIME}" \
    cargo build                                              \
    --target "build/targets/${KERNEL_BUILD_TARGET}.json"     \
    -Z build-std=core,compiler_builtins,alloc                \
    -Z build-std-features=compiler-builtins-mem
  then
    notify 'err' 'Could not compile kernel'
    exit 1
  fi

  notify 'tra' "Copying kernel binary to '${QEMU_KERNEL_BINARY}'"

  if ! cp "${KERNEL_BINARY}" "${QEMU_KERNEL_BINARY}"
  then
    notify 'err' "Could not copy kernel binary '${KERNEL_BINARY}'"
    exit 1
  fi

  notify 'tra' 'Checking for multiboot2 compatibility'

  if ! grub-file --is-x86-multiboot2 "${QEMU_KERNEL_BINARY}"
  then
    notify 'war' 'Kernel binary is not multiboot2-compatible'
  else
    notify 'inf' 'Kernel is multiboot2-compatible'
  fi

  notify 'suc' 'Finished building the kernel'
}

build_kernel "${@}"
