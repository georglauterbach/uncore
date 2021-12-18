#! /bin/bash

# version       0.2.0
# sourced by    shell scripts under scripts/ that can use /bin/bash
# task          provides error handlers

set -euEo pipefail
shopt -s inherit_errexit

trap '__log_uerror "${FUNCNAME[0]:-none / global}" "${BASH_COMMAND:-?}" "${LINENO:-?}" "${?:-?}"' ERR

function __log_uerror
{
  local MESSAGE
  MESSAGE='\n--- \e[1m\e[31mUNCHECKED ERROR\e[0m\n'
  MESSAGE+="  - script     = ${SCRIPT:-${0}}\n"
  MESSAGE+="  - function   = ${1}\n"
  MESSAGE+="  - command    = ${2}\n"
  MESSAGE+="  - line       = ${3}\n"
  MESSAGE+="  - exit code  = ${4}\n"

  echo -e "${MESSAGE}" >&2
}
