#! /usr/bin/env bash

readonly __BASE_COMMAND='cargo run -q --'

alias help="${__BASE_COMMAND} help"
alias run="${__BASE_COMMAND} run"
alias debug="${__BASE_COMMAND} run --debug"
alias utest="${__BASE_COMMAND} u-test"
alias itest="${__BASE_COMMAND} i-test"
alias check="${__BASE_COMMAND} check"
