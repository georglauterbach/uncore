# ! This image contains everything you need to build unCORE.
# ! Please note that unCORE officially only supports
# ! Development Containers for building and running.
#
# If you do not want to use dev-containers, you may build this image
# manually and work with this container image on a standalone basis.
#
# Moreover, if you do not want to work with a container image at all,
# you may replicate the steps taken in this file on your host.

# We use a custom base image (that itself is based on Ubuntu). The
# image contains additional helpers and programs that ease working
# inside a Development Container. The image does *not* contain
# programs or configuration files that you could not install manually
# by yourself.
FROM ghcr.io/georglauterbach/dev-container-base:1.0.0

SHELL [ "/bin/bash", "-eE", "-u", "-o", "pipefail", "-c" ]

USER root

# Refer to `devcontainer.json` on why `WORKSPACE_DIR_ARG´ exists.
ARG WORKSPACE_DIR_ARG=/workspaces/uncore
# Because `ENV` is required to have a fixed value and it cannot
# be overwritten like `ARG`, we use a preceding `ARG` to set the
# ENV. This is done to have the environment variable available
# inside the container later, during "run time" of the container.
ENV WORKSPACE_DIR="${WORKSPACE_DIR_ARG}"
# This environment variable is a shortcut to files we cache
# inside the Development Container.
ENV DEV_CONTAINER_FILES_DIR="${WORKSPACE_DIR}/code/.dev_container"

# We start by preparing a "base" layer with common packages,
# directories, etc.
RUN <<EOM
  # First and foremost, we update the package index and install
  # required packages for building unCORE.
  #
  # We do not need a cleanup because we only build locally anyway,
  # and there is no pushing to a registry involved. Image size
  # is hence not a concern.
  export DEBIAN_FRONTEND=noninteractive
  apt-get --yes update
  apt-get --yes install --no-install-recommends \
    'build-essential' 'gdb-multiarch' 'jq' 'qemu-system-riscv64'
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
  mkdir -p /rustup "${DEV_CONTAINER_FILES_DIR}"
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
ENV CARGO_TARGET_DIR="${DEV_CONTAINER_FILES_DIR}/target"

# This environment variable ensure Cargo stores its index in
# a directory we prepared exclusively for this task (which is also
# persisted on disk but ignored by git).This is crucial in
# ensuring we do not loose caches when the container is
# restarted (because the files are stored on the host).
ENV CARGO_HOME="${DEV_CONTAINER_FILES_DIR}/cargo_home"
# We also specify where Rustup puts its components; this is
# required to also not have Rustup always download components,
# but instead store them on the host disk (see `CARGO_HOME`) (i.e.
# this directory is also persisted on disk but ignored by git))
ENV RUSTUP_HOME="${DEV_CONTAINER_FILES_DIR}/rustup_home"

# This is the location where we mount the repository into and also
# where we will work.
WORKDIR "${WORKSPACE_DIR}"

# We do not need not do we want to be `root`. The Development Containers
# standard translates the UID of the host user so that no file conflicts
# arise.
USER ubuntu
