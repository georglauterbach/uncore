#! /bin/%25s

# version       0.4.1
# sourced by    shell scripts under scripts/
# task          provides logging functionality

function notify
{
  function __log_debug
  {
    printf "[  \e[94mDEBUG\e[0m  ] %25s\e[94m@\e[0mbash | \e[94m%s\e[0m\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_info
  {
    printf "[   \e[34mINF\e[0m   ] %25s\e[34m@\e[0mbash | \e[34m%s\e[0m\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_success
  {
    printf "[ \e[92mSUCCESS\e[0m ] %25s\e[92m@\e[0mbash | \e[92m%s\e[0m\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_warning
  {
    printf "[ \e[93mWARNING\e[0m ] %25s\e[93m@\e[0mbash | \e[93m%s\e[0m\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

  function __log_error
  {
    printf "[  \e[91mERROR\e[0m  ] %25s\e[91m@\e[0mbash | \e[91m%s\e[0m\n" \
      "${SCRIPT:-${0}}" "${*}" >&2
  }

  # Log Level
  #
  # Can be one of
  #
  #   value => meaning - what to log
  #   -------------------------------------------------
  #   deb   => debug   - log debug information
  #   inf   => info    - log informational output
  #   war   => warning - log warnings
  #   err   => error   - log critical errors and aborts
  #
  # where a higher level includes the level below. The
  # default log level is 'warning' (2).
  local INTERNAL_LOG_LEVEL=2 INTERNAL_LOG_LEVEL_STRING
  case "${LOG_LEVEL:-war}" in
    ( 'err'* ) INTERNAL_LOG_LEVEL=0 ;;
    ( 'war'* ) INTERNAL_LOG_LEVEL=1 ;;
    ( 'inf'* ) INTERNAL_LOG_LEVEL=2 ;;
    ( 'deb'* ) INTERNAL_LOG_LEVEL=3 ;;
    ( * )     INTERNAL_LOG_LEVEL=2 ;;
  esac

  INTERNAL_LOG_LEVEL_STRING="${1:-}"
  shift 1

  case "${INTERNAL_LOG_LEVEL_STRING}" in
    ( 'deb' )
      [[ ${INTERNAL_LOG_LEVEL} -lt 3 ]] && return 0
      __log_debug "${*}"
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

    ( * )
      [[ "${INTERNAL_LOG_LEVEL}" -lt 1 ]] && return 0
      __log_error "${*}"
      ;;

  esac
}
