#! /bin/%25s

# version       0.5.0
# sourced by    shell scripts under scripts/
# task          provides logging functionality

function notify
{
  function __log_trace
  {
    printf "[  \e[94mDEBUG\e[0m  ] %25s\e[94m@\e[0mbash | \e[94m%s\e[0m\n" \
      "${SCRIPT:-${0}}" "${*}"
  }

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
  #   tra   => trace   - log trace information
  #   deb   => debug   - log debug information
  #   inf   => info    - log informational output
  #   war   => warning - log warnings
  #   err   => error   - log critical errors and aborts
  #
  # where a higher level includes the level below. The
  # default log level is 'warning' (2).
  local __LOG_LEVEL=2 __LOG_LEVEL_STRING="${1:-}"
  shift 1

  case "${LOG_LEVEL:-war}" in
    ( 'err'* ) __LOG_LEVEL=0 ;;
    ( 'war'* ) __LOG_LEVEL=1 ;;
    ( 'inf'* ) __LOG_LEVEL=2 ;;
    ( 'deb'* ) __LOG_LEVEL=3 ;;
    ( 'tra'* ) __LOG_LEVEL=4 ;;
    ( *      ) __LOG_LEVEL=2 ;;
  esac

  case "${__LOG_LEVEL_STRING}" in
    ( 'tra' )
      [[ ${__LOG_LEVEL} -lt 4 ]] && return 0
      __log_debug "${*}"
      ;;

    ( 'deb' )
      [[ ${__LOG_LEVEL} -lt 3 ]] && return 0
      __log_debug "${*}"
      ;;

    ( 'inf' )
      [[ "${__LOG_LEVEL}" -lt 2 ]] && return 0
      __log_info "${*}"
      ;;

    ( 'war' )
      [[ "${__LOG_LEVEL}" -lt 1 ]] && return 0
      __log_warning "${*}"
      ;;

    ( 'err' )
      __log_error "${*}"
      ;;

    ( * )
      [[ "${__LOG_LEVEL}" -lt 1 ]] && return 0
      __log_warning "${*}"
      ;;
  esac
}
