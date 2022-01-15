#! /bin/bash

# version       0.2.0
# executed by   Just, manually or in CI
# task          builds the kernel

# shellcheck source=scripts/lib/init.sh
source "$(dirname "$(realpath -eL "${0}")")/lib/init.sh" 'kernel'
SCRIPT='build'

function build_kernel
{
  notify 'inf' "Compiling kernel for target '${BUILD_TARGET}'"

  if ! cargo build --target "${BUILD_TARGET}" "${KERNEL_BUILD_FLAGS[@]}"
  then
    notify 'err' 'Could not compile kernel'
    exit 1
  fi

  notify 'suc' 'Finished building the kernel'
  notify 'deb' "Copying kernel binary to '${QEMU_KERNEL_BINARY}'"

  # https://stackoverflow.com/a/55409182
  if ! cp "${KERNEL_BINARY}" "${QEMU_KERNEL_BINARY}"
  then
    notify 'err' "Could not copy kernel binary '${KERNEL_BINARY}'"
    exit 1
  fi
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
