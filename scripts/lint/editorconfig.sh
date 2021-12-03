#! /bin/bash

# version       0.4.1
# executed by   Make
# task          linting against EditorConfig

# shellcheck source=../lib/errors.sh
. scripts/lib/errors.sh
# shellcheck source=../lib/logs.sh
. scripts/lib/logs.sh
# shellcheck source=../lib/cri.sh
. scripts/lib/cri.sh

function lint
{
  SCRIPT='EditorConfig'
  setup_container_runtime

  local VERSION IMAGE
  VERSION=latest
  IMAGE="docker.io/mstruebing/editorconfig-checker:${VERSION}"

  notify 'inf' "version ${VERSION}"

  if "${CRI}" run \
    --rm \
    --cap-drop=ALL \
    --user=999 \
    --volume "${ROOT_DIRECTORY}:/ci:ro" \
    --workdir "/ci" \
    "${IMAGE}" ec \
      -config "/ci/scripts/lint/editorconfig.json"
  then
    notify 'suc' 'No errors detected'
  else
    notify 'err' 'Errors encountered'
    exit 1
  fi
}

lint
