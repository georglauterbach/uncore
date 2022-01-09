#! /bin/bash

# version       0.2.2
# executed by   just or manually
# task          installs needed dependencies

source scripts/lib/errors.sh
source scripts/lib/logs.sh

export SCRIPT='tools'
__BASH_LOG_LEVEL=${__BASH_LOG_LEVEL:-inf}

# -->                   -->                   --> START

function check_rust
{
  if ! command -v rustup &>/dev/null
  then
    notify 'inf' "'rustup' is not installed or not in \${PATH}"
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
    local JUST_VERSION=0.10.5
    if ! command -v just &>/dev/null && [[ $(just --version) != "just ${JUST_VERSION}" ]]
    then
      notify 'inf' "Installing Just (${JUST_VERSION}) with 'cargo'"
      cargo --quiet install just --version "${JUST_VERSION}"
    fi
  fi

  notify 'suc' 'Your Rust installation is complete'
}

function check_container_runtime
{
  if ! command -v docker &>/dev/null
  then
    if ! command -v podman &>/dev/null
    then
      notify 'inf' 'No container runtime detected'
      notify 'war' 'You will not be able to work with the documentation or lint all the code'
      return 0
    else
      notify 'inf' 'Podman detected as container runtime'
    fi
  else
    notify 'inf' 'Docker detected as container runtime'
  fi

  notify 'suc' 'You will be able to build and serve the documentation'
  notify 'suc' 'You can lint the code'
}

function _main
{
  check_rust
  check_container_runtime

  notify 'inf' 'Make sure QEMU is installed too'
}

_main "${@}"
