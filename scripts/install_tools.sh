#! /bin/bash

# version       0.2.4
# executed by   just or manually
# task          installs needed dependencies

# shellcheck source=scripts/lib/init.sh
source "$(dirname "$(realpath -eL "${0}")")/lib/init.sh" 'kernel'
SCRIPT='tools'

# -->                   -->                   --> START

function check_rust
{
  if ! command -v rustup &>/dev/null \
  || ! command -v rustc  &>/dev/null \
  || ! command -v cargo  &>/dev/null
  then
    notify 'inf' "Rust does not seem to be installed"
    notify 'inf' "Installing 'rustup', 'rustc' and 'cargo'"

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- -y
  else
    notify 'inf' "'rustup' is installed"
  fi

  if [[ ! ${PATH} == *.cargo/bin* ]] && [[ -e "${HOME}/.cargo/env" ]]
  then
    # shellcheck source=/dev/null
    source "${HOME}/.cargo/env"
  fi

  if ! command -v cargo &>/dev/null
  then
    notify 'war' "Could not find 'cargo'"
    notify 'err' \
      'Something went wrong installing Rust' \
      '- Please restart this script'
    return 0
  else
    local JUST_VERSION='0.10.5'
    if ! command -v just &>/dev/null && [[ $(just --version) != "just ${JUST_VERSION}" ]]
    then
      notify 'inf' "Installing Just (${JUST_VERSION}) with 'cargo'"
      cargo --quiet install just --version "${JUST_VERSION}"
    fi
  fi

  notify 'inf' 'Your Rust installation is complete'
}

function check_container_runtime
{
  if ! command -v docker &>/dev/null
  then
    if ! command -v podman &>/dev/null
    then
      notify 'inf' 'No container runtime detected'
      notify 'war' \
        'You will not be able to work' \
        'with the documentation or lint all the code locally'
      return 0
    else
      notify 'inf' 'Podman detected as container runtime'
    fi
  else
    notify 'inf' 'Docker detected as container runtime'
  fi

  notify 'inf' 'You will be able to build and serve the documentation'
  notify 'inf' 'You can lint the code'
}

function usage
{
  cat << "EOM"
INSTALL_TOOLS.SH(1)

SYNOPSIS
    ./scripts/install_tools.sh [ OPTION ]

OPTION
    --help           Show this help message

EOM
}

function main
{
  [[ ${1:-} == '--help' ]] && { usage ; exit 0 ; }

  check_rust
  check_container_runtime

  notify 'inf' 'Make sure QEMU is installed too'
}

main "${@}"
