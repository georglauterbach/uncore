#! /bin/bash

# version       0.4.1
# sourced by    shell scripts under scripts/
# task          provides logging functionality

function notify
{
  function __log_trace
  {
    printf "%-12s \e[94mTRACE\e[0m   %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_info
  {
    printf "%-12s \e[34mINFO   \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_success
  {
    printf "%-12s \e[32mSUCCESS\e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_warning
  {
    printf "%-12s \e[93mWARNING\e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_error
  {
    printf "%-12s \e[91mERROR  \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}" >&2
  }

  function __log_abort
  {
    printf "%-12s \e[31mABORT  \e[0m %s\n" \
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
  local INTERNAL_LOG_LEVEL=2
  case "${LOG_LEVEL:-warning}" in
    ( 'err' ) INTERNAL_LOG_LEVEL=0 ;;
    ( 'war' ) INTERNAL_LOG_LEVEL=1 ;;
    ( 'inf' ) INTERNAL_LOG_LEVEL=2 ;;
    ( 'tra' ) INTERNAL_LOG_LEVEL=3 ;;
    ( * )     INTERNAL_LOG_LEVEL=2 ;;
  esac

  case "${1:-}" in
    ( 'tra' )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 3 ]] && return 0
      shift
      __log_trace "${*}"
      ;;

    ( 'inf' )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 2 ]] && return 0
      shift
      __log_info "${*}"
      ;;

    ( 'suc' )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 2 ]] && return 0
      shift
      __log_success "${*}"
      ;;

    ( 'war' )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 1 ]] && return 0
      shift
      __log_warning "${*}"
      ;;

    ( 'err' ) shift ; __log_error "${*}" ;;
    ( 'abo' ) shift ; __log_abort "${*}" ;;

    ( * )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 1 ]] && return 0
      shift
      __log_error "${*}"
      ;;

  esac
}
