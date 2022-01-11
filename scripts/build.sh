#! /bin/bash

# version       0.1.1
# executed by   Just, manually or in CI
# task          builds the kernel

# shellcheck disable=SC2154

SCRIPT='build'
source scripts/lib/init.sh 'kernel'

[[ -n ${1:-} ]] && set_build_target "${1:-}"

notify 'inf' "Compiling kernel for target '${BUILD_TARGET}'"

if ! cargo build                                       \
  --target "build/targets/${BUILD_TARGET}.json" \
  "${KERNEL_BUILD_FLAGS[@]}"
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
