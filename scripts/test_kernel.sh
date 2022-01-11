#! /bin/bash

# version       0.1.1
# executed by   Just, manually or in CI
# task          runs kernel unit- and integration tests
# parameters    ${1} - kernel build target (optional)
#               ${2} - 'check' for `cargo check` (optional)
#               ${3} - if running tests, which test (optional)

SCRIPT='tests'
source scripts/lib/init.sh 'kernel' "${1:-}"

if [[ ${2:-} == 'check' ]]
then
  cargo check                                     \
    --target "build/targets/${BUILD_TARGET}.json" \
    "${KERNEL_BUILD_FLAGS[@]}"

  cargo fmt --all --message-format human -- --check
  cargo clippy --lib --all-features -- -D warnings
else
  # FIXME tests do not currently run
  notify 'abo' 'Unimplemented'
  exit 1

  if [[ -z ${3:-} ]]
  then
    cargo test --tests                              \
      --target "build/targets/${BUILD_TARGET}.json" \
      "${KERNEL_BUILD_FLAGS[@]}"
  else
    cargo test --test "${3}"       \
      --target "build/targets/${BUILD_TARGET}.json" \
      "${KERNEL_BUILD_FLAGS[@]}"
  fi

  # shellcheck disable=SC2181
  if [[ ${?} -eq 0 ]]
  then
    notify 'suc' 'Tests passed'
  else
    notify 'war' 'Tests did not pass'
  fi
fi
