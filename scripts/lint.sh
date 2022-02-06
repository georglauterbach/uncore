#! /bin/bash

# version       0.3.1
# executed by   just or manually
# task          lints the codebase against various linters

# shellcheck source=scripts/lib/init.sh
source "$(dirname "$(realpath -eL "${0}")")/lib/init.sh"
source scripts/lib/cri.sh
SCRIPT='linting'

# shellcheck disable=SC2154

# -->                   -->                   --> START

function lint_editorconfig
{
  local VERSION IMAGE
  VERSION=latest
  IMAGE="docker.io/mstruebing/editorconfig-checker:${VERSION}"

  notify 'deb' "Running EditorConfig lint (${VERSION})"

  if "${CRI}" run \
    --rm \
    --cap-drop=ALL \
    --user=999 \
    --volume "${ROOT_DIRECTORY}:/ci:ro" \
    --workdir "/ci" \
    "${IMAGE}" ec \
      -config "/ci/.github/linters/.ecrc"
  then
    notify 'inf' 'EditorConfig lint succeeded'
    return 0
  else
    notify 'err' 'EditorConfig lint reported problems'
    return 1
  fi
}

function lint_shellcheck
{  
  declare -a ARGUMENTS
  local TAG IMAGE FILES

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

  notify 'deb' "Running ShellCheck (${TAG})"

  # shellcheck disable=SC2154
  if "${CRI}" run \
    --rm \
    --cap-drop=ALL \
    --user=999 \
    --volume "${ROOT_DIRECTORY}/scripts:/ci/scripts:ro" \
    --workdir "/ci" \
    "${IMAGE}" \
      "${ARGUMENTS[@]}" \
      "${FILES[@]}"
  then
    notify 'inf' 'ShellCheck succeeded'
    return 0
  else
    notify 'err' 'ShellCheck reported problems'
    return 1
  fi
}

function lint_yamllint
{
  local VERSION IMAGE
  VERSION=1.26-0.9
  IMAGE="docker.io/cytopia/yamllint:${VERSION}"

  notify 'deb' "Running YAMLLint (${VERSION})"

  if "${CRI}" run \
    --rm \
    --cap-drop=ALL \
    --user=999 \
    --volume "${ROOT_DIRECTORY}/.github:/data/.github" \
    --volume "${ROOT_DIRECTORY}/documentation:/data/documentation" \
    --volume "${ROOT_DIRECTORY}/scripts:/data/scripts" \
    "${IMAGE}" \
      --strict \
      --config-file "/data/.github/linters/.yaml-lint.yml" \
      --format colored \
      -- .
  then
    notify 'inf' 'YAMLLint succeeded'
    return 0
  else
    notify 'err' 'YAMLLint reported problems'
    return 1
  fi
}

function usage
{
  cat << "EOM" 
LINT.SH(1)

SYNOPSIS
    ./scripts/lint.sh [ OPTION... ] < ACTION... >
    just lint         [ OPTION... ] < ACTION... >

OPTIONS
    --help                     Show this help message

ACTIONS
    editorcinfig | ec          Run the EditorConfig linter
    shellcheck   | sc          Run the ShellCheck linter
    yamllint     | yl          Run the YAMLLint linter

EOM
}

function main
{
  setup_container_runtime
  local ERROR_OCCURRED=false

  notify 'inf' 'Starting the linting process'

  if [[ -n ${1:-} ]]
  then
    case "${1}" in
      ( '--help' )
        usage
        exit 0
        ;;

      ( 'editorconfig' | 'ec' )
        lint_editorconfig || ERROR_OCCURRED=true
        ;;

      ( 'shellcheck' | 'sc' )
        lint_shellcheck || ERROR_OCCURRED=true
        ;;

      ( 'yamllint' | 'yl' )
        lint_yamllint || ERROR_OCCURRED=true
        ;;

      ( * )
        notify 'err' "'${1}' is not a valid linter ('sh' or 'gsl' are valid)"
        exit 1
        ;;
    esac
  else
    lint_editorconfig || ERROR_OCCURRED=true
    lint_shellcheck || ERROR_OCCURRED=true
    lint_yamllint || ERROR_OCCURRED=true
  fi

  if ${ERROR_OCCURRED}
  then
    notify 'err' 'Linting not successful'
    return 1
  else
    notify 'inf' 'Linting successful'
    return 0
  fi
}

main "${@}" || exit ${?}
