#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
GUARD_LIBRARY="${REPO_ROOT}/scripts/lib/cargo_lock_guard.sh"
TEST_ROOT="$(mktemp -d -t 'aerocodex cargo lock guard.XXXXXX')"
trap 'rm -rf -- "${TEST_ROOT}"' EXIT

PASS_COUNT=0
SKIP_COUNT=0

fail() {
  printf '[cargo-lock-guard-test] ERROR: %s\n' "$*" >&2
  exit 1
}

pass_case() {
  PASS_COUNT=$((PASS_COUNT + 1))
  printf '[cargo-lock-guard-test] PASS case=%s\n' "$1"
}

skip_case() {
  SKIP_COUNT=$((SKIP_COUNT + 1))
  printf '[cargo-lock-guard-test] SKIP case=%s reason=%s\n' "$1" "$2"
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
    trap 'exit 130' INT
    trap 'exit 143' TERM
    cargo_lock_guard_initialize
    case "${mode}" in
      failure) exit 42 ;;
      int) kill -INT "${BASHPID}" ;;
      term) kill -TERM "${BASHPID}" ;;
      *) fail "unknown exit mode ${mode}" ;;
    esac
  )
}

run_regular_replacement_case() {
  local name="$1"
  local replacement_mode="$2"
  local root
  local expected
  local rc
  root="$(new_repo "${name}")"
  expected="${TEST_ROOT}/${name}.expected"
  printf 'user replacement for %s\nsecond line\n' "${name}" >"${expected}"

  set +e
  (
    cd "${root}"
    source "${GUARD_LIBRARY}"
    cargo_lock_guard_initialize
    case "${replacement_mode}" in
      recreate)
        rm -- "Cargo.lock"
        cp -- "${expected}" "Cargo.lock"
        ;;
      atomic)
        cp -- "${expected}" "replacement.lock"
        mv -f -- "replacement.lock" "Cargo.lock"
        ;;
      *) fail "unknown replacement mode ${replacement_mode}" ;;
    esac
    cargo_lock_guard_cleanup
  )
  rc=$?
  set -e

  [[ "${rc}" -ne 0 ]] || fail "${name} replacement cleanup unexpectedly succeeded"
  [[ -f "${root}/Cargo.lock" && ! -L "${root}/Cargo.lock" ]] || fail "${name} replacement was not preserved as a regular file"
  cmp -- "${expected}" "${root}/Cargo.lock" || fail "${name} replacement bytes changed"
  pass_case "${name}_fails_and_preserves_bytes"
}

owned_root="$(new_repo owned-same-instance)"
(
  cd "${owned_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
  printf '# cargo updated the owned lock in place\nversion = 3\n' >"Cargo.lock"
  cargo_lock_guard_verify
  cargo_lock_guard_cleanup
  [[ ! -e Cargo.lock && ! -L Cargo.lock ]] || fail 'owned transient was not removed'
)
pass_case 'owned_same_instance_normal_cleanup'

run_regular_replacement_case 'deleted-recreated' recreate
run_regular_replacement_case 'atomic-rename-over' atomic

symlink_root="$(new_repo replacement-symlink)"
printf 'symlink target user data\n' >"${symlink_root}/replacement-target.txt"
if (
  cd "${symlink_root}"
  ln -s -- "replacement-target.txt" "symlink-capability-probe" 2>/dev/null
  [[ -L symlink-capability-probe ]]
); then
  rm -- "${symlink_root}/symlink-capability-probe"
  set +e
  (
    cd "${symlink_root}"
    source "${GUARD_LIBRARY}"
    cargo_lock_guard_initialize
    rm -- "Cargo.lock"
    ln -s -- "replacement-target.txt" "Cargo.lock"
    cargo_lock_guard_cleanup
  )
  symlink_rc=$?
  set -e
  [[ "${symlink_rc}" -ne 0 ]] || fail 'symlink replacement cleanup unexpectedly succeeded'
  [[ -L "${symlink_root}/Cargo.lock" ]] || fail 'symlink replacement was not preserved'
  [[ "$(readlink -- "${symlink_root}/Cargo.lock")" == 'replacement-target.txt' ]] || fail 'symlink replacement target changed'
  [[ "$(cat "${symlink_root}/Cargo.lock")" == 'symlink target user data' ]] || fail 'symlink replacement data changed'
  pass_case 'replacement_symlink_fails_and_is_preserved'
else
  rm -f -- "${symlink_root}/symlink-capability-probe"
  skip_case 'replacement_symlink_fails_and_is_preserved' 'native symbolic-link creation unavailable on this host; Ubuntu CI executes it'
fi

directory_root="$(new_repo replacement-directory)"
set +e
(
  cd "${directory_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
  rm -- "Cargo.lock"
  mkdir -- "Cargo.lock"
  printf 'directory replacement user data\n' >"Cargo.lock/user-data.txt"
  cargo_lock_guard_cleanup
)
directory_rc=$?
set -e
[[ "${directory_rc}" -ne 0 ]] || fail 'directory replacement cleanup unexpectedly succeeded'
[[ -d "${directory_root}/Cargo.lock" && ! -L "${directory_root}/Cargo.lock" ]] || fail 'directory replacement was not preserved'
[[ "$(cat "${directory_root}/Cargo.lock/user-data.txt")" == 'directory replacement user data' ]] || fail 'directory replacement data changed'
pass_case 'replacement_directory_fails_and_is_preserved'

identity_root="$(new_repo identity-unavailable)"
set +e
(
  cd "${identity_root}"
  source "${GUARD_LIBRARY}"
  cargo_lock_guard_initialize
  before="$(sha256sum -- "Cargo.lock")"
  CARGO_LOCK_GUARD_STAT_COMMAND='aerocodex-deliberately-missing-stat'
  cleanup_rc=0
  cargo_lock_guard_cleanup || cleanup_rc=$?
  after="$(sha256sum -- "Cargo.lock")"
  [[ "${before}" == "${after}" ]] || fail 'identity-unavailable path changed Cargo.lock bytes'
  exit "${cleanup_rc}"
)
identity_rc=$?
set -e
[[ "${identity_rc}" -ne 0 ]] || fail 'identity-unavailable cleanup unexpectedly succeeded'
[[ -f "${identity_root}/Cargo.lock" && ! -L "${identity_root}/Cargo.lock" ]] || fail 'identity-unavailable Cargo.lock was not preserved'
pass_case 'identity_unavailable_fails_and_preserves'

failure_root="$(new_repo command-failure)"
set +e
run_exit_case "${failure_root}" failure
failure_rc=$?
set -e
[[ "${failure_rc}" -eq 42 ]] || fail "failure exit code changed to ${failure_rc}"
[[ ! -e "${failure_root}/Cargo.lock" ]] || fail 'owned lock survived command failure cleanup'
pass_case 'command_failure_cleanup'

int_root="$(new_repo interruption-int)"
set +e
run_exit_case "${int_root}" int
int_rc=$?
set -e
[[ "${int_rc}" -eq 130 ]] || fail "INT exit code changed to ${int_rc}"
[[ ! -e "${int_root}/Cargo.lock" ]] || fail 'owned lock survived INT cleanup'
pass_case 'int_cleanup'

term_root="$(new_repo interruption-term)"
set +e
run_exit_case "${term_root}" term
term_rc=$?
set -e
[[ "${term_rc}" -eq 143 ]] || fail "TERM exit code changed to ${term_rc}"
[[ ! -e "${term_root}/Cargo.lock" ]] || fail 'owned lock survived TERM cleanup'
pass_case 'term_cleanup'

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
pass_case 'preexisting_untracked_preserved'

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
pass_case 'preexisting_tracked_preserved'

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
pass_case 'preexisting_dirty_tracked_preserved'

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
pass_case 'tracked_modified_during_run_preserved'

printf '[cargo-lock-guard-test] PASS cases=%s skipped=%s paths_with_spaces=yes\n' "${PASS_COUNT}" "${SKIP_COUNT}"
