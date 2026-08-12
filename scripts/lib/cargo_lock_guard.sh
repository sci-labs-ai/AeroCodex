#!/usr/bin/env bash

# This file is sourced by agent_pr_check.sh and its focused test harness.
CARGO_LOCK_GUARD_STATE="uninitialized"
CARGO_LOCK_GUARD_BASELINE_SHA=""
CARGO_LOCK_GUARD_CLEANED="no"

cargo_lock_guard_log() {
  printf '[cargo-lock-guard] %s\n' "$*"
}

cargo_lock_guard_is_tracked() {
  git ls-files --error-unmatch -- "Cargo.lock" >/dev/null 2>&1
}

cargo_lock_guard_is_ignored() {
  git check-ignore -q --no-index -- "Cargo.lock"
}

cargo_lock_guard_hash() {
  sha256sum -- "Cargo.lock" | awk '{print $1}'
}

cargo_lock_guard_initialize() {
  if [[ "${CARGO_LOCK_GUARD_STATE}" != "uninitialized" ]]; then
    cargo_lock_guard_log "ERROR: guard was initialized more than once"
    return 1
  fi

  if cargo_lock_guard_is_tracked; then
    CARGO_LOCK_GUARD_STATE="tracked"
    if [[ ! -f Cargo.lock || -L Cargo.lock ]]; then
      cargo_lock_guard_log "ERROR: tracked root Cargo.lock is missing or is not a regular file; preserving repository state"
      return 1
    fi
    if ! git diff --quiet -- "Cargo.lock" || ! git diff --cached --quiet -- "Cargo.lock"; then
      CARGO_LOCK_GUARD_STATE="tracked_dirty"
      cargo_lock_guard_log "ERROR: tracked root Cargo.lock has preexisting staged or unstaged changes; preserving it and refusing to run"
      return 1
    fi
    CARGO_LOCK_GUARD_BASELINE_SHA="$(cargo_lock_guard_hash)"
    cargo_lock_guard_log "initial_state=tracked sha256=${CARGO_LOCK_GUARD_BASELINE_SHA} cleanup=preserve"
    return 0
  fi

  if [[ -e Cargo.lock || -L Cargo.lock ]]; then
    CARGO_LOCK_GUARD_STATE="preexisting_untracked"
    cargo_lock_guard_log "ERROR: preexisting untracked root Cargo.lock is ambiguous; preserving it and refusing to run"
    return 1
  fi
  if ! cargo_lock_guard_is_ignored; then
    CARGO_LOCK_GUARD_STATE="policy_error"
    cargo_lock_guard_log "ERROR: absent root Cargo.lock is not covered by the repository ignore policy; refusing to create it"
    return 1
  fi

  if ! (set -o noclobber; printf '# Transient lockfile owned by scripts/agent_pr_check.sh.\nversion = 3\n' >"Cargo.lock") 2>/dev/null; then
    CARGO_LOCK_GUARD_STATE="ownership_race"
    cargo_lock_guard_log "ERROR: could not atomically claim absent root Cargo.lock; preserving the competing path"
    return 1
  fi
  CARGO_LOCK_GUARD_STATE="owned"
  if cargo_lock_guard_is_tracked || [[ ! -f Cargo.lock || -L Cargo.lock ]] || ! cargo_lock_guard_is_ignored; then
    CARGO_LOCK_GUARD_STATE="ownership_ambiguous"
    cargo_lock_guard_log "ERROR: root Cargo.lock ownership became ambiguous immediately after creation; preserving it"
    return 1
  fi
  cargo_lock_guard_log "initial_state=absent created_owned_transient=yes cleanup=guarded_remove"
}

cargo_lock_guard_verify() {
  case "${CARGO_LOCK_GUARD_STATE}" in
    tracked)
      if [[ ! -f Cargo.lock || -L Cargo.lock ]]; then
        cargo_lock_guard_log "ERROR: tracked root Cargo.lock disappeared or changed type; preserving current state"
        return 1
      fi
      local current_sha
      current_sha="$(cargo_lock_guard_hash)"
      if [[ "${current_sha}" != "${CARGO_LOCK_GUARD_BASELINE_SHA}" ]]; then
        cargo_lock_guard_log "ERROR: tracked root Cargo.lock changed (before=${CARGO_LOCK_GUARD_BASELINE_SHA} after=${current_sha}); preserving it"
        return 1
      fi
      ;;
    owned)
      if [[ ! -f Cargo.lock || -L Cargo.lock ]]; then
        cargo_lock_guard_log "ERROR: script-owned root Cargo.lock disappeared or changed type; ownership is ambiguous, so no cleanup will occur"
        return 1
      fi
      if cargo_lock_guard_is_tracked || ! cargo_lock_guard_is_ignored; then
        cargo_lock_guard_log "ERROR: script-owned root Cargo.lock became tracked or unignored; preserving it"
        return 1
      fi
      ;;
    preexisting_untracked | tracked_dirty | policy_error | ownership_race | ownership_ambiguous)
      cargo_lock_guard_log "ERROR: root Cargo.lock guard is in non-runnable state ${CARGO_LOCK_GUARD_STATE}; preserving current state"
      return 1
      ;;
    cleaned)
      return 0
      ;;
    *)
      cargo_lock_guard_log "ERROR: root Cargo.lock guard is not initialized"
      return 1
      ;;
  esac
}

cargo_lock_guard_cleanup() {
  if [[ "${CARGO_LOCK_GUARD_CLEANED}" == "yes" ]]; then
    return 0
  fi

  case "${CARGO_LOCK_GUARD_STATE}" in
    owned)
      if ! cargo_lock_guard_verify; then
        cargo_lock_guard_log "ERROR: refusing to delete root Cargo.lock because ownership checks failed"
        return 1
      fi
      local sha
      local size
      sha="$(cargo_lock_guard_hash)"
      size="$(stat -c%s -- "Cargo.lock")"
      rm -- "Cargo.lock"
      if [[ -e Cargo.lock || -L Cargo.lock ]]; then
        cargo_lock_guard_log "ERROR: script-owned root Cargo.lock still exists after guarded removal"
        return 1
      fi
      CARGO_LOCK_GUARD_STATE="cleaned"
      CARGO_LOCK_GUARD_CLEANED="yes"
      cargo_lock_guard_log "removed_owned_transient=yes sha256=${sha} size=${size}"
      ;;
    tracked)
      cargo_lock_guard_verify || return 1
      CARGO_LOCK_GUARD_CLEANED="yes"
      cargo_lock_guard_log "preserved_tracked=yes"
      ;;
    uninitialized)
      CARGO_LOCK_GUARD_CLEANED="yes"
      ;;
    *)
      CARGO_LOCK_GUARD_CLEANED="yes"
      cargo_lock_guard_log "preserved_ambiguous_state=${CARGO_LOCK_GUARD_STATE}"
      ;;
  esac
}
