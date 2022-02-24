#! /bin/bash

# version       0.2.0
# executed by   Just, manually or in CI
# task          runs the kernel in QEMU

# shellcheck source=scripts/init.sh
source "$(dirname "${BASH_SOURCE[0]}")/init.sh" 'kernel' 'errors' 'log'
SCRIPT='QEMU runner@bash'

function run
{
  declare -a QEMU_ARGUMENTS
  local EXIT_CODE

  # QEMU_ARGUMENTS+=('-nodefaults')

  QEMU_ARGUMENTS+=('-machine')
  QEMU_ARGUMENTS+=('q35,accel=kvm:tcg')

  QEMU_ARGUMENTS+=('-m')
  QEMU_ARGUMENTS+=('128M')

  # set up OVMF
  # QEMU_ARGUMENTS+=('-bios')
  # QEMU_ARGUMENTS+=('/usr/share/ovmf/OVMF.fd')
  QEMU_ARGUMENTS+=('-drive')
  QEMU_ARGUMENTS+=('if=pflash,format=raw,file=/usr/share/OVMF/OVMF_CODE.fd,readonly=on')
  QEMU_ARGUMENTS+=('-drive')
  QEMU_ARGUMENTS+=('if=pflash,format=raw,file=/usr/share/OVMF/OVMF_VARS.fd,readonly=on')

  QEMU_ARGUMENTS+=('-drive')
  QEMU_ARGUMENTS+=("format=raw,file=fat:rw:${QEMU_VOLUME_DIRECTORY}")

  QEMU_ARGUMENTS+=('-debugcon')
  QEMU_ARGUMENTS+=("file:${QEMU_DIRECTORY}/debugcon.txt")

  if [[ ${*} == *'graphical'* ]]
  then
    QEMU_ARGUMENTS+=('-vga')
    QEMU_ARGUMENTS+=('std')
  
    QEMU_ARGUMENTS+=('-monitor')
    QEMU_ARGUMENTS+=('vc:1024x768')
  else
    # QEMU_ARGUMENTS+=('-nographic')
    QEMU_ARGUMENTS+=('-serial')
    QEMU_ARGUMENTS+=('stdio')
    QEMU_ARGUMENTS+=('-display')
    QEMU_ARGUMENTS+=('none')
  fi

  # mainly used for unit and integration tests
  QEMU_ARGUMENTS+=('-device')
  QEMU_ARGUMENTS+=('isa-debug-exit,iobase=0xf4,iosize=0x04')

  QEMU_ARGUMENTS+=('-no-reboot')
  
  notify 'inf' 'Now running in QEMU'
  notify 'deb' "Arguments are '${QEMU_ARGUMENTS[*]}'"

  qemu-system-x86_64 "${QEMU_ARGUMENTS[@]}"
  EXIT_CODE=${?}

  if [[ ${EXIT_CODE} -eq 3 ]]
  then
    notify 'inf' 'Kernel exited QEMU properly'
  elif [[ ${EXIT_CODE} -eq 0 ]]
  then
    notify 'war' 'Kernel exited QEMU unexpectedly (triple-fault, manual QEMU termination, ... ?)'
    return 1
  else
    notify 'err' 'Kernel did not exit QEMU properly' "(exit code was ${EXIT_CODE})"
    return $((EXIT_CODE + 1))
  fi
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
  
  export QEMU_DIRECTORY QEMU_VOLUME_DIRECTORY
  QEMU_DIRECTORY="${QEMU_DIRECTORY:-out/qemu}"
  QEMU_VOLUME_DIRECTORY="${QEMU_DIRECTORY}/kernel"

  run "${@}" || return ${?}
}

main "${@}" || exit ${?}
