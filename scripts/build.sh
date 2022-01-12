#! /bin/bash

# version       0.1.1
# executed by   Just, manually or in CI
# task          builds the kernel

# shellcheck disable=SC2154

source scripts/lib/init.sh 'kernel'
SCRIPT='build'

function build_kernel
{
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
}

function usage
{
  cat << "EOM" 
BUILD.SH(1)

SYNOPSIS
    ./scripts/build.sh [ OPTION... ]
    just build         [ OPTION... ]

OPTIONS
    --help           Show this help message
    --target TARGET  specify target triple to use when building and running the kernel

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
        set_build_target "${1:-}"
        shift 1
        ;;

      # the arguments of this scripts are a real superset of those of
      # the run script as the command runner 'Just' will call this
      # script with the same arguments first to make sure the kernel
      # binary is up to date - these arguments are immediately
      # 'shifted' away
      ( '--graphical' )
        shift 1
        ;;

      ( * )
        notify 'abo' "'${1}' is invalid (run with --help to get more information)"
        exit 1
        ;;
    esac
  done

  build_kernel
}

main "${@}"
