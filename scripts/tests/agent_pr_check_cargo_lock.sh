#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
GUARD_LIBRARY="${REPO_ROOT}/scripts/lib/cargo_lock_guard.sh"
TEST_ROOT="$(mktemp -d -t 'aerocodex cargo lock guard.XXXXXX')"
trap 'rm -rf -- "${TEST_ROOT}"' EXIT

fail() {
  printf '[cargo-lock-guard-test] ERROR: %s\n' "$*" >&2
  exit 1
}

new_repo() {
  local name="$1"
  local root="${TEST_ROOT}/repository fixtures/${name}"
  mkdir -p -- "${root}"
  git -C "${root}" init -q
  git -C "${root}" config user.email tests@example.invalid
  git -C "${root}" config user.name 'AeroCodex Tests'
  git -C "${root}" config core.autocrlf false
  printf '/Cargo.lock\n' >"${root}/.gitignore"
  printf 'fixture\n' >"${root}/fixture.txt"
  git -C "${root}" add .gitignore fixture.txt
  git -C "${root}" commit -q -m fixture
  printf '%s\n' "${root}"
}

run_exit_case() {
  local root="$1"
  local mode="$2"
  (
    cd "${root}"
    # shellcheck source=../lib/cargo_lock_guard.sh
    source "${GUARD_LIBRARY}"
    on_exit() {
      local rc=$?
      trap - EXIT
      if ! cargo_lock_guard_cleanup; then
        [[ "${rc}" -ne 0 ]] || rc=1
      fi
      exit "${rc}"
    }
    trap on_exit EXIT
    trap 'exit 143' TERM
    cargo_lock_guard_initialize
    if [[ "${mode}" == "failure" ]]; then
      exit 42
    fi
    kill -TERM "${BASHPID}"
  )
}

absent_root="$(new_repo absent)"
(
  cd "${absent_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
  printf '# cargo updated the owned lock\nversion = 3\n' >"Cargo.lock"
  cargo_lock_guard_verify
  cargo_lock_guard_cleanup
  [[ ! -e Cargo.lock && ! -L Cargo.lock ]] || fail 'owned transient was not removed'
)

untracked_root="$(new_repo preexisting-untracked)"
printf 'user-owned\n' >"${untracked_root}/Cargo.lock"
set +e
(
  cd "${untracked_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
)
untracked_rc=$?
set -e
[[ "${untracked_rc}" -ne 0 ]] || fail 'preexisting untracked lock was accepted'
[[ "$(cat "${untracked_root}/Cargo.lock")" == 'user-owned' ]] || fail 'preexisting untracked lock was changed'

tracked_root="$(new_repo tracked)"
printf 'tracked\n' >"${tracked_root}/Cargo.lock"
git -C "${tracked_root}" add -f Cargo.lock
git -C "${tracked_root}" commit -q -m 'track lock'
(
  cd "${tracked_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
  cargo_lock_guard_cleanup
)
[[ "$(cat "${tracked_root}/Cargo.lock")" == 'tracked' ]] || fail 'tracked lock was changed or removed'

dirty_root="$(new_repo tracked-dirty-at-start)"
printf 'tracked\n' >"${dirty_root}/Cargo.lock"
git -C "${dirty_root}" add -f Cargo.lock
git -C "${dirty_root}" commit -q -m 'track lock'
printf 'preexisting dirty\n' >"${dirty_root}/Cargo.lock"
set +e
(
  cd "${dirty_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
)
dirty_rc=$?
set -e
[[ "${dirty_rc}" -ne 0 ]] || fail 'preexisting dirty tracked lock was accepted'
[[ "$(cat "${dirty_root}/Cargo.lock")" == 'preexisting dirty' ]] || fail 'preexisting dirty tracked lock was changed'

modified_root="$(new_repo tracked-modified)"
printf 'tracked\n' >"${modified_root}/Cargo.lock"
git -C "${modified_root}" add -f Cargo.lock
git -C "${modified_root}" commit -q -m 'track lock'
set +e
(
  cd "${modified_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
  printf 'changed\n' >"Cargo.lock"
  cargo_lock_guard_cleanup
)
modified_rc=$?
set -e
[[ "${modified_rc}" -ne 0 ]] || fail 'modified tracked lock cleanup did not fail'
[[ "$(cat "${modified_root}/Cargo.lock")" == 'changed' ]] || fail 'modified tracked lock was not preserved'

failure_root="$(new_repo command-failure)"
set +e
run_exit_case "${failure_root}" failure
failure_rc=$?
set -e
[[ "${failure_rc}" -eq 42 ]] || fail "failure exit code changed to ${failure_rc}"
[[ ! -e "${failure_root}/Cargo.lock" ]] || fail 'owned lock survived command failure'

interrupt_root="$(new_repo interruption)"
set +e
run_exit_case "${interrupt_root}" interruption
interrupt_rc=$?
set -e
[[ "${interrupt_rc}" -eq 143 ]] || fail "interruption exit code changed to ${interrupt_rc}"
[[ ! -e "${interrupt_root}/Cargo.lock" ]] || fail 'owned lock survived interruption'

printf '[cargo-lock-guard-test] PASS cases=7 paths_with_spaces=yes\n'
