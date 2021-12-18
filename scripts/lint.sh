#! /bin/bash

# version       0.3.0
# executed by   just or manually
# task          lints the codebase against various linters

source scripts/lib/errors.sh
source scripts/lib/logs.sh
source scripts/lib/cri.sh

export SCRIPT='linting'
__BASH_LOG_LEVEL=${__BASH_LOG_LEVEL:-inf}

# -->                   -->                   --> START

function lint_shellcheck
{  
  local TAG IMAGE FILES ARGUMENTS

  TAG=0.8.0
  IMAGE=docker.io/koalaman/shellcheck:v${TAG}
  readarray -d '' FILES < <(find . -type f -iname "*.sh" -print0)

  ARGUMENTS=(
    '--shell=bash'
    '--enable=all'
    '--severity=style'
    '--color=auto'
    '--wiki-link-count=5'
    '--check-sourced'
    '--external-sources'
    '--exclude=SC2310'
    '--exclude=SC2312'
    "--source-path=${ROOT_DIRECTORY}"
  )

  notify 'inf' "Running ShellCheck (${TAG})"

  if ${CRI} run \
    --rm \
    --cap-drop=ALL \
    --user=999 \
    --volume "${ROOT_DIRECTORY}/scripts:/ci/scripts:ro" \
    --workdir "/ci" \
    "${IMAGE}" \
      "${ARGUMENTS[@]}" \
      "${FILES[@]}"
  then
    notify 'suc' 'ShellCheck succeeded'
    return 0
  else
    notify 'err' 'ShellCheck reported problems'
    return 1
  fi
}

function lint_github_super_linter
{
  local TAG IMAGE

  TAG='slim-v4.8.4'
  # TAG='slim-latest'
  IMAGE="github/super-linter:${TAG}"

  notify 'inf' "Running GitHub Super Linter (${TAG})"

  if ${CRI} run \
    -e RUN_LOCAL=true \
    -e LOG_LEVEL=ERROR \
    -e SUPPRESS_POSSUM=true \
    -e VALIDATE_ALL_CODEBASE=true \
    -e ERROR_ON_MISSING_EXEC_BIT=true \
    -e VALIDATE_JSCPD_ALL_CODEBASE=false \
    -e VALIDATE_BASH=true \
    -e VALIDATE_BASH_EXEC=true \
    -e VALIDATE_EDITORCONFIG=true \
    -e VALIDATE_GITHUB_ACTIONS=true \
    -e VALIDATE_JSCPD=true \
    -e VALIDATE_JSON=true \
    -e VALIDATE_MARKDOWN=true \
    -e VALIDATE_YAML=true \
    --volume "$(pwd):/uncore:ro" \
    --workdir "/uncore" \
    -e DEFAULT_WORKSPACE=/uncore \
    -e LOG_FILE=../../dev/null \
    "${IMAGE}" >/dev/null
  then
    notify 'suc' 'GitHub Super Linter succeeded'
    return 0
  else
    notify 'err' 'GitHub Super Linte reported problems'
    return 1
  fi
}

function __main
{
  setup_container_runtime
  local ERROR_OCCURRED=false

  notify 'inf' 'Starting the linting process'

  # lint_shellcheck || ERROR_OCCURRED=true
  lint_github_super_linter || ERROR_OCCURRED=true

  if ${ERROR_OCCURRED}
  then
    notify 'err' 'Linting not successful'
    return 1
  else
    notify 'suc' 'Linting successful'
    return 0
  fi
}

__main "${@}" || exit ${?}
