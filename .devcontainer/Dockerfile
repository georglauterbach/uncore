# ! This image contains everything you need to build unCORE.
# ! Please note that unCORE officially only supports
# ! Development Containers for building and running.
#
# If you do not want to use dev-containers, you may build this image
# manually and work with this container image on a standalone basis.
#
# Moreover, if you do not want to work with a container image at all,
# you may replicate the steps taken in this file on you host.

# We use Ubuntu as the base image. Ubuntu has regular and stable releases,
# which makes it a good fit for building unCORE.
FROM ubuntu:23.10

SHELL [ "/bin/bash", "-eE", "-o", "pipefail", "-c" ]

# We start by preparing a "base" layer with common packages,
# directories, etc.
RUN <<EOM
  # First and foremost, we update the package index and install
  # required packages for building unCORE.
  export DEBIAN_FRONTEND=noninteractive
  readonly PACKAGES=(
    # 'bash-completion' 'build-essential'
    'ca-certificates' 'curl'
    # 'jq'
    # 'qemu-system-riscv64'
    'wget'
  )
  apt-get --yes update
  apt-get --yes install --no-install-recommends "${PACKAGES[@]}"

  curl -sSfL -o setup.sh https://raw.githubusercontent.com/georglauterbach/uds/main/setup.sh
  bash setup.sh --no-gui

  # This cleanup step has to happen in the same RUN statement
  # that also updates the package index and install packages;
  # do not move it out of here.
  apt-get --yes clean
  rm -rf /var/lib/apt/lists/*

  # We use `doas` instead of `sudo` (because it is smaller and
  # easier to configure): the `ubuntu` user is the user that is
  # used when working in this container (see `USER ubuntu` at
  # the bottom), but we want the user to have root privileges
  # inside the container. We also create a symbolic link so users
  # can use `sudo` transparently.
  cat >/etc/doas.conf << "EOF"
permit nopass ubuntu
EOF
  doas -C /etc/doas.conf
  ln -s "$(command -v doas)" /usr/local/bin/sudo
EOM

# In the next layer, we install Rust. This is step does not
# configure a toolchain or a build target. Such a setup is not
# required during build-time of the container because it will
# be executed when running the build container for the first
# time.
RUN <<EOM
  # These directories
  #
  # 1. contain the binaries `cargo`, `rustup`, etc.;
  # 2. contain the repository files (mounted from the host)
  #
  # respectively.
  mkdir /rustup /workspaces
  export RUSTUP_HOME='/rustup'
  export CARGO_HOME='/rustup'

  # We do not install a toolchain at this point in time because
  # on the one hand, Cargo will do so for us the first time we
  # interact with the unCORE code, on the other hand, we want to
  # keep this container image as small as possible.
  curl -sSfL 'https://sh.rustup.rs' | bash -s -- '-y' \
    '--no-update-default-toolchain'                   \
    '--default-toolchain' 'none'                      \
    '--profile' 'minimal'

  MOLD_VERSION='2.4.0'
  MOLD_DIR="mold-${MOLD_VERSION}-$(uname -m)-linux"

  curl --silent --show-error --fail --location                                               \
    "https://github.com/rui314/mold/releases/download/v${MOLD_VERSION}/${MOLD_DIR}.tar.gz" | \
    tar xvz -C /tmp

  cp "/tmp/${MOLD_DIR}/"{bin/{mold,ld.mold},lib/mold/mold-wrapper.so} /usr/local/bin/
  rm -r "/tmp/${MOLD_DIR}"
EOM

# We also adjust `PATH` to be able to run commands like `rustc`,
# `cargo` and `rustup` effortlessly.
ENV PATH="/rustup/bin:${PATH}"

# Moreover, we do not want to interfere with Cargo's output
# directory of the host (in case someone want to use both this container
# image and the hst); hence, we use a different output directory.
ENV CARGO_TARGET_DIR=dev-container/target

# This environment variable ensure Cargo stores its index in
# `/workspaces/uncore/code/.cargo/home`. This is crucial in
# ensuring we do not loose caches when the container is
# restarted (because the files are stored on the host).
ENV CARGO_HOME=/workspaces/uncore/code/dev-container/cargo-home
# We also specify where Rustup puts its components; this is
# required to also not have Rustup always download components,
# but instead store them on the host disk (see `CARGO_HOME`).
ENV RUSTUP_HOME=/workspaces/uncore/code/dev-container/rustup-home

# `/workspaces/uncore` is the directory we mount the respository root
# directory to, in order to compile and run unCORE.
WORKDIR /workspaces/uncore

USER ubuntu
