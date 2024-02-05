#! /usr/bin/env bash

set -eE -u -o pipefail
shopt -s inherit_errexit

[[ ${EUID} -eq 0 ]] || { echo "Run this script as root!" ; exit 1 ; }

# RUST ------------------------------------------------------------------------

if ! command -v rustup &>/dev/null || ! command -v cargo &>/dev/null; then
  RUSTUP_PARAMETERS=('--quiet' '-y' '--default-toolchain' 'none' '--profile' 'minimal' '--no-update-default-toolchain')
  [[ ${#} -eq 0 ]] || readarray -t -d ' ' RUSTUP_PARAMETERS < <(printf "%s" "${*}")

  curl -sSfL 'https://sh.rustup.rs' | sh -s -- "${RUSTUP_PARAMETERS[@]}"
fi

# MOLD ------------------------------------------------------------------------

if ! command -v mold &>/dev/null; then
  MOLD_VERSION='2.4.0'
  MOLD_DIR="mold-${MOLD_VERSION}-$(uname -m)-linux"
  MOLD_TARBALL="${MOLD_DIR}.tar.gz"

  (
    cd /tmp
    [[ -f ${MOLD_TARBALL} ]] && rm -rf "${MOLD_TARBALL}"
    curl -sSfL -o "${MOLD_TARBALL}" \
      "https://github.com/rui314/mold/releases/download/v${MOLD_VERSION}/${MOLD_DIR}.tar.gz"

    [[ -d ${MOLD_DIR} ]] && rm -rf "${MOLD_DIR}"
    tar xf "${MOLD_DIR}.tar.gz"

    cp "${MOLD_DIR}/bin/mold" /usr/local/bin/
    cp "${MOLD_DIR}/lib/mold/mold-wrapper.so" /usr/local/bin/
  )
fi
