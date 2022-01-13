#! /bin/bash

# version       0.1.2
# executed by   Just, manually or in CI
# task          runs the kernel in QEMU

# shellcheck source=scripts/lib/init.sh
source "$(dirname "$(realpath -eL "${0}")")/lib/init.sh" 'kernel'
SCRIPT='QEMU runner'

function prepare_qemu
{
  export QEMU_DIRECTORY QEMU_VOLUME_DIRECTORY
  
  QEMU_DIRECTORY="${QEMU_DIRECTORY:-build/qemu}"
  KERNEL_BINARY="${QEMU_DIRECTORY}/kernel.bin"
  QEMU_VOLUME_DIRECTORY="${QEMU_DIRECTORY}/vm_volume"

  if [[ ! -f ${KERNEL_BINARY} ]]
  then
    notify 'err' 'Kernel binary not found - was it built before?'
    exit 1
  fi

  function prepare_ovmf
  {
    local OVMF_SYSTEM_PATH="/usr/share/OVMF"
    export OVMF_FW_PATH="${QEMU_DIRECTORY}/ovmf/OVMF_CODE.fd"
    export OVMF_VARS_PATH="${QEMU_DIRECTORY}/ovmf/OVMF_VARS.fd"

    notify 'tra' 'Checking of OVMF files exist'
    rm -rf "${QEMU_DIRECTORY}/ovmf"

    if [[ ! -f "${OVMF_SYSTEM_PATH}/OVMF_CODE.fd" ]] \
    || [[ ! -f "${OVMF_SYSTEM_PATH}/OVMF_VARS.fd" ]]
    then
      notify 'inf' 'No OVMF files exist'
      return 0
    fi

    notify 'inf' 'Copying OVMF files'

    mkdir -p "${QEMU_DIRECTORY}/ovmf"
    cp "${OVMF_SYSTEM_PATH}/OVMF_VARS.fd" "${QEMU_DIRECTORY}/ovmf"
    cp "${OVMF_SYSTEM_PATH}/OVMF_CODE.fd" "${QEMU_DIRECTORY}/ovmf"
  }

  prepare_ovmf

  rm -rf "${QEMU_VOLUME_DIRECTORY}"
  mkdir -p "${QEMU_VOLUME_DIRECTORY}/EFI/BOOT"

  notify 'inf' "Creating 'BOOTX64.EFI' file"

  if ! grub-mkstandalone                               \
    -O x86_64-efi                                      \
    -o "${QEMU_VOLUME_DIRECTORY}/EFI/BOOT/BOOTX64.EFI" \
    "/boot/grub/grub.cfg=${QEMU_DIRECTORY}/grub.cfg"   \
    "/boot/kernel.bin=${KERNEL_BINARY}"
  then
    notify 'err' "Could not create 'BOOTX64.EFI' file"
    exit 1
  fi
}

function run_in_qemu
{
  declare -a QEMU_ARGUMENTS

  QEMU_ARGUMENTS+=('-nodefaults')

  QEMU_ARGUMENTS+=('-machine')
  QEMU_ARGUMENTS+=('q35,accel=kvm:tcg')

  QEMU_ARGUMENTS+=('-m')
  QEMU_ARGUMENTS+=('128M')

  # set up OVMF
  QEMU_ARGUMENTS+=('-drive')
  QEMU_ARGUMENTS+=("if=pflash,format=raw,readonly=on,file=${OVMF_FW_PATH}")
  QEMU_ARGUMENTS+=('-drive')
  QEMU_ARGUMENTS+=("format=raw,file=fat:rw:${QEMU_VOLUME_DIRECTORY}")

  # https://phip1611.de/blog/how-to-use-qemus-debugcon-feature-and-write-to-a-file/
  QEMU_ARGUMENTS+=('-debugcon')
  QEMU_ARGUMENTS+=("file:${QEMU_DIRECTORY}/debugcon.txt") # file:${QEMU_DIRECTORY}/debugcon.txt or file:/dev/stdout

  QEMU_ARGUMENTS+=('-serial')
  QEMU_ARGUMENTS+=('stdio')

  if [[ ${*} == *'graphical'* ]]
  then
    QEMU_ARGUMENTS+=('-vga')
    QEMU_ARGUMENTS+=('std')
  
    QEMU_ARGUMENTS+=('-monitor')
    QEMU_ARGUMENTS+=('vc:1024x768')
  else
    QEMU_ARGUMENTS+=('-nographic')

    QEMU_ARGUMENTS+=('-display')
    QEMU_ARGUMENTS+=('none')
  fi

  # mainly used for unit and integration tests
  QEMU_ARGUMENTS+=('-device')
  QEMU_ARGUMENTS+=('isa-debug-exit,iobase=0xf4,iosize=0x04')

  QEMU_ARGUMENTS+=('-no-reboot')
  
  notify 'inf' 'Now running in QEMU'
  notify 'tra' "Arguments are '${QEMU_ARGUMENTS[*]}'"

  qemu-system-x86_64 "${QEMU_ARGUMENTS[@]}"
}

function usage
{
  cat << "EOM" 
RUN_IN_QEMU.SH(1)

SYNOPSIS
    ./scripts/run_in_qemu.sh [ OPTION... ] [ < QEMU_OPTION... > ]
    just run                 [ OPTION... ] [ < QEMU_OPTION... > ]

OPTIONS
    --help                     Show this help message
    --target TARGET            Only relevant for build process

QEMU_OPTIONS
    --graphical                Use a dedicated window and graphics

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
        shift 2
        ;;
      
      ( '--graphical' )
        break
        ;;

      ( * )
        notify 'err' "'${1}' is invalid (run with --help to get more information)"
        exit 1
        ;;
    esac
  done

  prepare_qemu
  run_in_qemu "${@}" || return ${?}
}

main "${@}" || exit ${?}
