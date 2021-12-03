#! /bin/bash

# version       0.2.0
# executed by   Make
# task          linting YAML files

# shellcheck source=../lib/errors.sh
. scripts/lib/errors.sh
# shellcheck source=../lib/logs.sh
. scripts/lib/logs.sh
# shellcheck source=../lib/cri.sh
. scripts/lib/cri.sh

function lint
{
  SCRIPT='YAMLLint'
  setup_container_runtime

  local VERSION IMAGE
  VERSION=1.26-0.9
  IMAGE="docker.io/cytopia/yamllint:${VERSION}"

  notify 'inf' "version ${VERSION}"

  if "${CRI}" run \
    --rm \
    --cap-drop=ALL \
    --user=999 \
    --volume "${ROOT_DIRECTORY}/.github:/data/.github" \
    --volume "${ROOT_DIRECTORY}/documentation:/data/documentation" \
    --volume "${ROOT_DIRECTORY}/scripts:/data/scripts" \
    "${IMAGE}" \
      --strict \
      --config-file "/data/scripts/lint/yamllint.yml" \
      -- .
  then
    notify 'suc' 'No errors detected'
  else
    notify 'err' 'Errors encountered'
    exit 1
  fi
}

lint
