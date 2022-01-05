#! /bin/bash

# version       0.1.0
# executed by   Just, manually or in CI
# task          builds the kernel

SCRIPT='build'
__BASH_LOG_LEVEL=${__BASH_LOG_LEVEL:-inf}

GUESSES_ROOT_DIRECTORY="$(realpath -e -L "$(dirname "$(realpath -e -L "${0}")")/..")"
ROOT_DIRECTORY=${ROOT_DIRECTORY:-${GUESSES_ROOT_DIRECTORY}}

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
  local TARGET KERNEL_BINARY

  TARGET=${1:-x86_64-unknown-none}
  KERNEL_BINARY="target/${TARGET}/debug/kernel"

  notify 'inf' "Compiling kernel for target '${TARGET}'"

  if ! VERSION=${VERSION:-testing} cargo build \
    --target "build/targets/${TARGET}.json" \
    -Z build-std=core,compiler_builtins,alloc \
    -Z build-std-features=compiler-builtins-mem
  then
    notify 'err' 'Could not compile kernel'
    exit 1
  fi

  notify 'tra' 'Checking for multiboot2 compatibility'

  if ! grub-file --is-x86-multiboot2 build/qemu/kernel.bin
  then
    notify 'war' 'Kernel binary is not multiboot2-compatible'
  else
    notify 'inf' 'Kernel is multiboot2-compatible'
  fi

  notify 'tra' "Copying kernel binary to 'build/qemu/kernel.bin'"

  if ! cp "${KERNEL_BINARY}" build/qemu/kernel.bin
  then
    notify 'err' "Could not copy kernel binary '${KERNEL_BINARY}'"
    exit 1
  fi

  notify 'suc' 'Finished building the kernel'
}

build_kernel "${@}"
