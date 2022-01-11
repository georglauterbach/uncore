#! /bin/bash

# version       0.1.1
# executed by   Just, manually or in CI
# task          builds the kernel

SCRIPT='build'
source scripts/lib/init.sh 'kernel'

function build_kernel
{
  local QEMU_KERNEL_BINARY
  export BUILD_TARGET COMPILATION_DATE_AND_TIME
  export KERNEL_BINARY RUST_TOOLCHAIN RUSTC_VERSION

  QEMU_KERNEL_BINARY='build/qemu/kernel.bin'
  BUILD_TARGET="${1:-x86_64-unknown-none}"

  KERNEL_BINARY="target/${BUILD_TARGET}/debug/kernel"
  RUSTC_VERSION="$(rustc --version)" ; RUSTC_VERSION=${RUSTC_VERSION#rustc }
  RUST_TOOLCHAIN="$(rustup toolchain list | grep -E '(override)' | cut -d ' ' -f 1)"
  COMPILATION_DATE_AND_TIME="$(date +'%H:%M, %d %b %Y')"

  notify 'inf' "Compiling kernel for target '${BUILD_TARGET}'"

  if ! cargo build                                       \
    --target "build/targets/${BUILD_TARGET}.json" \
    -Z build-std=core,compiler_builtins,alloc            \
    -Z build-std-features=compiler-builtins-mem
  then
    notify 'err' 'Could not compile kernel'
    exit 1
  fi

  notify 'tra' 'Checking for multiboot2 compatibility'

  if ! grub-file --is-x86-multiboot2 "${KERNEL_BINARY}"
  then
    notify 'err' 'Kernel binary is not multiboot2-compatible'
    exit 1
  else
    notify 'inf' 'Kernel is multiboot2-compatible'
  fi

  notify 'tra' "Copying kernel binary to '${QEMU_KERNEL_BINARY}'"

  if ! cp "${KERNEL_BINARY}" "${QEMU_KERNEL_BINARY}"
  then
    notify 'err' "Could not copy kernel binary '${KERNEL_BINARY}'"
    exit 1
  fi

  notify 'suc' 'Finished building the kernel'
}

build_kernel "${@}"
