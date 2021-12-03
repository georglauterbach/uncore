#! /bin/sh

# version       0.3.1-stable
# sourced by    shell scripts under scripts/
# task          provides logging functionality

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
# where a higher level includes the level below.
__BASH_LOG_LEVEL=${__BASH_LOG_LEVEL:-warning}

notify()
{
  __log_trace()
  {
    printf "%-12s \e[94mTRACE\e[0m   %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  __log_info()
  {
    printf "%-12s \e[34mINFO   \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  __log_success()
  {
    printf "%-12s \e[32mSUCCESS\e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  __log_warning()
  {
    printf "%-12s \e[93mWARNING\e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  __log_error()
  {
    printf "%-12s \e[91mERROR  \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}" >&2
  }

  __log_abort()
  {
    printf "%-12s \e[31mABORT  \e[0m %s\n" \
      "${SCRIPT:-${0}}" "${*}" >&2
  }

  LOG_LEVEL=2
  case "${__BASH_LOG_LEVEL}" in
    ( 'err' ) LOG_LEVEL=0 ;;
    ( 'war' ) LOG_LEVEL=1 ;;
    ( 'inf' ) LOG_LEVEL=2 ;;
    ( 'tra' ) LOG_LEVEL=3 ;;
    ( * )     LOG_LEVEL=2 ;;
  esac

  case "${1:-}" in
    ( 'tra' )
      [ "${LOG_LEVEL}" -lt 3 ] && return 0
      shift
      __log_trace "${*}"
      ;;

    ( 'inf' )
      [ "${LOG_LEVEL}" -lt 2 ] && return 0
      shift
      __log_info "${*}"
      ;;

    ( 'suc' )
      [ "${LOG_LEVEL}" -lt 2 ] && return 0
      shift
      __log_success "${*}"
      ;;

    ( 'war' )
      [ "${LOG_LEVEL}" -lt 1 ] && return 0
      shift
      __log_warning "${*}"
      ;;

    ( 'err' ) shift ; __log_error "${*}" ;;
    ( 'abo' ) shift ; __log_abort "${*}" ;;

    ( * )
      [ "${LOG_LEVEL}" -lt 1 ] && return 0
      shift
      __log_error "${*}"
      ;;

  esac
}
