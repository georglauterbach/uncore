#! /bin/bash

# version       0.4.1
# executed by   Make
# task          linting scripts against ShellCheck

# shellcheck source=../lib/errors.sh
. scripts/lib/errors.sh
# shellcheck source=../lib/logs.sh
. scripts/lib/logs.sh
# shellcheck source=../lib/cri.sh
. scripts/lib/cri.sh

function lint
{
  SCRIPT='ShellCheck'
  setup_container_runtime
  
  local VERSION IMAGE FILES ARGUMENTS
  VERSION=0.7.2
  IMAGE="docker.io/koalaman/shellcheck-alpine:v${VERSION}"
  readarray -d '' FILES < <(find . -type f -iname "*.sh" -print0)

  ARGUMENTS=(
    --enable=all
    --severity=style
    --color=auto
    --wiki-link-count=50
    --exclude=SC2154
    --check-sourced
    --external-sources
    --source-path=SCRIPTDIR
    --format=tty
  )

  notify 'inf' "version ${VERSION}"

  if "${CRI}" run \
    --rm \
    --cap-drop=ALL \
    --user=999 \
    --volume "${ROOT_DIRECTORY}/scripts:/ci/scripts:ro" \
    --workdir "/ci" \
    "${IMAGE}" shellcheck \
      "${ARGUMENTS[@]}" \
      "${FILES[@]}"
  then
    notify 'suc' 'No errors detected'
  else
    notify 'err' 'Errors encountered'
    exit 1
  fi
}

lint
