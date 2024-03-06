#! /usr/bin/env bash

# RUST ------------------------------------------------------------------------

if [[ ${1} == '--container' ]]; then
  # We set a temporary value here to have the Rust installer
  # place the binaries and their configuration here. The values
  # of these variables will be changed again later; this enabled
  # caching the build, registry index and rustup components on
  # the host's disk permanently, without the need to know what
  # is needed exactly during build-time of this container.
  export RUSTUP_HOME='/rustup'
  export CARGO_HOME='/rustup'
fi

curl -sSfL 'https://sh.rustup.rs'   \
  | bash -s --                      \
    '-y'                            \
    '--no-update-default-toolchain' \
    '--profile' 'minimal'           \
    '--default-toolchain' 'none'

if [[ ${1} != '--container' ]]; then
  cargo --version
  rustc --version
fi

# MOLD ------------------------------------------------------------------------

MOLD_VERSION='2.4.0'
MOLD_DIR="mold-${MOLD_VERSION}-$(uname -m)-linux"
MOLD_TARBALL="${MOLD_DIR}.tar.gz"

[[ -f ${MOLD_TARBALL} ]] && rm -rf "${MOLD_TARBALL}"
[[ -d ${MOLD_DIR} ]] && rm -rf "${MOLD_DIR}"

curl --silent --show-error --fail --location --output "${MOLD_TARBALL}" \
  "https://github.com/rui314/mold/releases/download/v${MOLD_VERSION}/${MOLD_DIR}.tar.gz"
tar xf "${MOLD_DIR}.tar.gz"

cp "${MOLD_DIR}/bin/mold" /usr/local/bin/
cp "${MOLD_DIR}/bin/ld.mold" /usr/local/bin/
cp "${MOLD_DIR}/lib/mold/mold-wrapper.so" /usr/local/bin/

rm -rf "${MOLD_TARBALL}"
rm -rf "${MOLD_DIR}"

if [[ ${1} != '--container' ]]; then
  mold --version
fi
