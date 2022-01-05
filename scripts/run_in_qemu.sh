#! /bin/bash

# version       0.1.0
# executed by   Just, manually or in CI
# task          runs the kernel in QEMU

SCRIPT='run in QEMU'
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

QEMU_DIRECTORY=build/qemu
QEMU_VOLUME_DIRECTORY="${QEMU_DIRECTORY}/vm_volume"
KERNEL_BINARY="${QEMU_DIRECTORY}/kernel.bin"

if [[ ! -f ${KERNEL_BINARY} ]]
then
  notify 'err' 'Kernel binary not found - was it built before?'
  exit 1
fi


function prepare_qemu
{
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

  grub-mkstandalone                                    \
    -O x86_64-efi                                      \
    -o "${QEMU_VOLUME_DIRECTORY}/EFI/BOOT/BOOTX64.EFI" \
    "/boot/grub/grub.cfg=${QEMU_DIRECTORY}/grub.cfg"   \
    "/boot/kernel.bin=${KERNEL_BINARY}"
}

function run_in_qemu
{
  declare -a QEMU_ARGUMENTS

  QEMU_ARGUMENTS=(
    "-nodefaults"

    # use standard VGA for graphics
    "-vga"
    "std"

    # breaks "runs_inside_qemu()"-detection, changes CPUID
    # "-cpu"
    # "host"

    # use a modern machine, with acceleration if possible
    "-machine"
    "q35,accel=kvm:tcg"

    # allocate some memory
    "-m"
    "128M"

    # set up OVMF
    "-drive"
    "if=pflash,format=raw,readonly=on,file=${OVMF_FW_PATH}"
    "-drive"
    "if=pflash,format=raw,file=${OVMF_VARS_PATH}"

    # mount a local directory as a FAT partition
    "-drive"
    "format=raw,file=fat:rw:${QEMU_VOLUME_DIRECTORY}"

    # connect the serial port to the host. OVMF is kind enough to connect
    # the UEFI stdout and stdin to that port too.
    "-serial"
    "stdio"

    "-debugcon"
    "file:build/qemu/debugcon.txt"
    # or "/dev/stdout"

    "-monitor"
    "vc:1024x768"
  )

  notify 'inf' 'Now running in QEMU'
  notify 'tra' "Arguments are '${QEMU_ARGUMENTS[*]}'"
  qemu-system-x86_64 "${QEMU_ARGUMENTS[@]}"
}

prepare_qemu
run_in_qemu
