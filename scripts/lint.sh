#! /bin/bash

# version       0.3.1
# executed by   just or manually
# task          lints the codebase against various linters

SCRIPT='linting'

source scripts/lib/init.sh
source scripts/lib/cri.sh

# -->                   -->                   --> START

function lint_shellcheck
{  
  local TAG IMAGE FILES ARGUMENTS

  TAG='0.8.0'
  IMAGE="docker.io/koalaman/shellcheck:v${TAG}"
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

  TAG='slim-v4.8.5'
  # TAG='slim-latest'
  IMAGE="ghcr.io/github/super-linter:${TAG}"

  notify 'inf' "Running GitHub Super Linter (${TAG})"

  if ${CRI} run \
    -e RUN_LOCAL=true \
    -e LOG_LEVEL=ERROR \
    -e SUPPRESS_POSSUM=true \
    -e VALIDATE_ALL_CODEBASE=true \
    -e IGNORE_GITIGNORED_FILES=true \
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

  if [[ -n ${1:-} ]]
  then
    case "${1}" in
      ( 'shellcheck' | 'sc' )
        lint_shellcheck || ERROR_OCCURRED=true
        ;;
      
      ( 'github-super-linter' | 'gsl' )
        lint_github_super_linter || ERROR_OCCURRED=true
        ;;
      
      ( * )
        notify 'err' "'${1}' is not a valid linter ('sh' or 'gsl' are valid)"
        exit 1
        ;;
    esac
  else
    lint_github_super_linter || ERROR_OCCURRED=true
  fi

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
