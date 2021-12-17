#! /bin/bash

# version       0.2.0
# executed by   just or manually
# task          installs needed dependencies

# shellcheck source=./lib/errors.sh
. scripts/lib/errors.sh
# shellcheck source=./lib/logs.sh
. scripts/lib/logs.sh

export SCRIPT='tools'
__BASH_LOG_LEVEL=${__BASH_LOG_LEVEL:-inf}

TOOLCHAIN=${TOOLCHAIN:-$(tr -d '\n' < kernel/rust-toolchain)}

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

  notify 'inf' "Setting Rust toolchain and installing additional components with 'rustup'"
  rustup --quiet override set "${TOOLCHAIN}"
  rustup --quiet component add --toolchain "${TOOLCHAIN}" \
    llvm-tools-preview rust-src
  
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
    notify 'inf' "Installing additonal packages with 'cargo'"
    cargo --quiet install cargo-xbuild bootimage
    cargo --quiet install just --version 0.10.4
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
