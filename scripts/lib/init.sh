#! /bin/bash

# version       0.1.0
# executed by   shell scripts under scripts/
# task          performs script initialization

export LOG_LEVEL ROOT_DIRECTORY SCRIPT

LOG_LEVEL=${LOG_LEVEL:-inf}
GUESSED_ROOT_DIRECTORY="$(realpath -e -L "$(dirname "$(realpath -e -L "${0}")")/..")"
ROOT_DIRECTORY=${ROOT_DIRECTORY:-${GUESSED_ROOT_DIRECTORY}}
SCRIPT="${SCRIPT:-${0}}"

if ! cd "${ROOT_DIRECTORY}"
then
   printf "%-12s \e[31mABORT  \e[0m %s%s\n"             \
      "${SCRIPT:-${0}}"                                 \
      'Could not change into repository root directory' \
      "${ROOT_DIRECTORY}" >&2
  exit 1
fi

source scripts/lib/errors.sh
source scripts/lib/logs.sh

notify 'tra' 'Performed basic script intialization'

if [[ ${1:-} == 'kernel' ]]
then
  notify 'tra' 'Changing into kernel directory'
  if ! cd "${ROOT_DIRECTORY}/kernel"
  then
    notify 'abo' 'Could not change into kernel directory'
    exit 1
  fi
fi

notify 'tra' 'Finished script intialization'
