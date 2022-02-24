#! /bin/bash

# version       0.2.0
# executed by   Just, manually or in CI
# task          builds the kernel

# shellcheck source=scripts/init.sh
source "$(dirname "${BASH_SOURCE[0]}")/init.sh" 'kernel' 'errors' 'log'
SCRIPT='build@bash'

function build_kernel
{
  notify 'inf' "Compiling kernel for target '${BUILD_TARGET}'"

  if ! cargo build                  \
    --target "${BUILD_TARGET_PATH}" \
    "${KERNEL_BUILD_FLAGS[@]}"
  then
    notify 'err' 'Could not compile kernel'
    exit 1
  fi

  notify 'inf' 'Finished building the kernel'
  notify 'inf' "Linking kernel with bootloader now"

  if ! cargo run --package boot --quiet
  then
    notify 'err' 'Could not link the kernel with the bootloader'
    exit 1
  fi

  notify 'deb' "Finished linking kernel with bootloader"
  notify 'deb' "Copying kernel binary to '${QEMU_KERNEL_BINARY}'"

  local BOOTLOADER_BUILD_OUTPUT
  BOOTLOADER_BUILD_OUTPUT="${ROOT_DIRECTORY}/kernel/out/qemu/boot_output/boot-uefi-kernel.efi"

  # https://stackoverflow.com/a/55409182
  if ! cp "${BOOTLOADER_BUILD_OUTPUT}" "${QEMU_KERNEL_BINARY}"
  then
    notify 'err' "Could not copy bootloader build output '${BOOTLOADER_BUILD_OUTPUT}'"
    exit 1
  fi

  notify 'inf' 'Created bootable kernel image(s)'
}

function usage
{
  cat << "EOM" 
BUILD.SH(1)

SYNOPSIS
    ./scripts/build.sh [ OPTION... ]
    just build         [ OPTION... ]

OPTIONS
    --help            Show this help message
    --target TARGET   specify target triple to use when building and running the kernel
                      currently valid options are x86_64, aarch64 and i686

EOM
}

function main
{
  while [[ -n ${1:-} ]]
  do
    case "${1:-}" in
      ( '--help' )
        usage
        exit 0
        ;;

      ( '--target' )
        set_build_target "${2:-}"
        shift 2
        ;;

      # the arguments of this scripts are a real superset of those of
      # the run script as the command runner 'Just' will call this
      # script with the same arguments first to make sure the kernel
      # binary is up to date - these arguments are immediately
      # 'shifted' away
      ( '--graphical' )
        shift 1
        ;;

      # same as above applies here, only with 2 arguments
      ( '--test' )
        shift 2
        ;;

      ( * )
        notify 'err' "'${1}' is invalid (run with --help to get more information)"
        exit 1
        ;;
    esac
  done

  build_kernel
}

main "${@}"
