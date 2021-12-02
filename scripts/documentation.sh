#! /bin/bash

# version       0.1.0
# executed by   manually or in CI
# task          builds and serves the documentation

SCRIPT='documentation'
__BASH_LOG_LEVEL=${__BASH_LOG_LEVEL:-inf}

IMAGE_NAME='uncore/documentation:latest'
DOCUMENTATION_DIRECTORY="${ROOT_DIRECTORY:-$(realpath -e -L .)}/documentation"

MKDOCS_MATERIAL_TAG='7.3.6'
MKDOCS_MATERIAL_IMAGE="docker.io/squidfunk/mkdocs-material:${MKDOCS_MATERIAL_TAG}"

CRI='docker'

# shellcheck source=lib/errors.sh
. scripts/lib/errors.sh
# shellcheck source=lib/logs.sh
. scripts/lib/logs.sh
# shellcheck source=lib/cri.sh
. scripts/lib/cri.sh

function build_documentation
{
  "${CRI}" run \
    --rm -it \
    --user "$(id -u):$(id -g)" \
    -v "${DOCUMENTATION_DIRECTORY}:/docs" \
    "${MKDOCS_MATERIAL_IMAGE}" build \
      --config-file config.yml \
      --strict
}

function cleanup_documentation_files
{
  find "${DOCUMENTATION_DIRECTORY}/site" \
    -type f \
    -name '*.min.js.map' -delete \
    -o -name '*.min.css.map' -delete

  rm -rf \
    "${DOCUMENTATION_DIRECTORY}/site/sitemap.xml.gz" \
    "${DOCUMENTATION_DIRECTORY}/site/assets/images/favicon.png" \
    "${DOCUMENTATION_DIRECTORY}/site/assets/javascripts/lunr"
}

function build_container_image
{
  "${CRI}" build -t "${IMAGE_NAME}" "${DOCUMENTATION_DIRECTORY}"
}

function serve_documentation
{
  notify 'inf' 'Serving on 127.0.0.1:8080'

  "${CRI}" run \
    --rm -it \
    --user "$(id -u):$(id -g)" \
    -v "${DOCUMENTATION_DIRECTORY}:/docs" \
    -p 8080:8080 \
    "${MKDOCS_MATERIAL_IMAGE}" serve \
      --config-file config.yml \
      --dev-addr 0.0.0.0:8080
}

function __main
{
  setup_container_runtime

  case "${1:-}" in
    ( 'build' )
      build_documentation
      cleanup_documentation_files
      ;;

    ( * )
      serve_documentation
      ;;
  esac
}

__main "${@}"
