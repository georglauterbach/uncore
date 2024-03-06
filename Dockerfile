# We use Ubuntu as the base image. Ubuntu has regular and stable releases,
# which makes it a good fit for building unCORE.
FROM ubuntu:23.10

SHELL [ "/bin/bash", "-e", "-o", "pipefail", "-c" ]

# We start by preparing a "base" layer with common packages,
# directories, etc.
RUN <<EOM
  # First and foremost, we update the package index and install
  # required packages for building unCORE.
  export DEBIAN_FRONTEND=noninteractive
  apt-get --yes update
  apt-get --yes install --no-install-recommends \
    build-essential                             \
    ca-certificates curl                        \
    jq

  apt-get --yes install --no-install-recommends \
    qemu-system-riscv64

  # This cleanup step has to happen in the same RUN statement
  # that also updates the package index and install packages;
  # do not move it out of here.
  apt-get --yes clean
  rm -rf /var/lib/apt/lists/*

  # This directory will contain the binaries `cargo`,
  # `rustup`, etc.
  mkdir /rustup
  touch /IS_CONTAINER
EOM

# In the next layer, we install Rust. This is step does not
# configure a toolchain or a build target. Such a setup is not
# required during build-time of the container because it will
# be executed when running the build container for the first
# time.
COPY misc/scripts/install_rust_and_mold.sh /
RUN bash /install_rust_and_mold.sh --container

# We also adjust `PATH` to be able to run commands like `rustc`,
# `cargo` and `rustup` effortlessly.
ENV PATH="/rustup/bin:${PATH}"

# Moreover, we do not want to interfere with Cargo's output
# directory of the host; hence, we use a different output
# directory.
ENV CARGO_TARGET_DIR=target-build-container

# This environment variable ensure Cargo stores its index in
# `/tmp/build`, the directory we use the build directory. This
# is crucial in ensuring Cargo does not need to update the
# index every time we invoke the container to build unCORE.
ENV CARGO_HOME=/tmp/build/.cargo/home
# We also specify where Rustup puts its components; this is
# required to also not have Rustup always download components,
# but instead store them on the host disk.
ENV RUSTUP_HOME=/tmp/build/.cargo/rustup

# `/tmp/build` is the directory we mount the `code` directory
# in to compile and run unCORE.
WORKDIR /tmp/build

ENTRYPOINT [ "/rustup/bin/cargo", "run", "--" ]
CMD [ "build" ]
