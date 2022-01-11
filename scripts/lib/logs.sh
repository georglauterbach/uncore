#! /bin/bash

# version       0.4.1
# sourced by    shell scripts under scripts/
# task          provides logging functionality

function notify
{
  function __log_trace
  {
    printf "%-15s \e[94mTRACE\e[0m   %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_info
  {
    printf "%-15s \e[34mINFO   \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_success
  {
    printf "%-15s \e[32mSUCCESS\e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_warning
  {
    printf "%-15s \e[93mWARNING\e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_error
  {
    printf "%-15s \e[91mERROR  \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}" >&2
  }

  function __log_abort
  {
    printf "%-15s \e[31mABORT  \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}" >&2
  }

  # Log Level
  #
  # Can be one of
  #
  #   value => meaning - what to log
  #   -------------------------------------------------
  #   tra   => trace   - log debug information
  #   inf   => info    - log informational output
  #   war   => warning - log warnings
  #   err   => error   - log critical errors and aborts
  #
  # where a higher level includes the level below. The
  # default log level is 'warning' (2).
  local INTERNAL_LOG_LEVEL=2 INTERNAL_LOG_LEVEL_STRING
  case "${LOG_LEVEL:-war}" in
    ( 'err' ) INTERNAL_LOG_LEVEL=0 ;;
    ( 'war' ) INTERNAL_LOG_LEVEL=1 ;;
    ( 'inf' ) INTERNAL_LOG_LEVEL=2 ;;
    ( 'tra' ) INTERNAL_LOG_LEVEL=3 ;;
    ( * )     INTERNAL_LOG_LEVEL=2 ;;
  esac

  INTERNAL_LOG_LEVEL_STRING="${1:-}"
  shift 1

  case "${INTERNAL_LOG_LEVEL_STRING}" in
    ( 'tra' )
      [[ ${INTERNAL_LOG_LEVEL} -lt 3 ]] && return 0
      __log_trace "${*}"
      ;;

    ( 'inf' )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 2 ]] && return 0
      __log_info "${*}"
      ;;

    ( 'suc' )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 2 ]] && return 0
      __log_success "${*}"
      ;;

    ( 'war' )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 1 ]] && return 0
      __log_warning "${*}"
      ;;

    ( 'err' ) __log_error "${*}" ;;
    ( 'abo' ) __log_abort "${*}" ;;

    ( * )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 1 ]] && return 0
      __log_error "${*}"
      ;;

  esac
}
