#! /bin/bash

# shellcheck source=scripts/lib/logs.sh
. scripts/lib/logs.sh

function setup_container_runtime
{
  command -v 'docker' &>/dev/null && export CRI='docker' && return 0
  command -v 'podman' &>/dev/null && export CRI='podman' && return 0

  notify 'err' \
    'Could not identify Container Runtime.' \
    "Is 'docker' or 'podman' in \${PATH}?"
  exit 1
}
