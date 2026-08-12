#!/usr/bin/env bash

# This file is sourced by agent_pr_check.sh and its focused test harness.
CARGO_LOCK_GUARD_STATE="uninitialized"
CARGO_LOCK_GUARD_BASELINE_SHA=""
CARGO_LOCK_GUARD_OWNED_IDENTITY=""
CARGO_LOCK_GUARD_PRIVATE_DIR=""
CARGO_LOCK_GUARD_ANCHOR=""
CARGO_LOCK_GUARD_CREATED_TARGET="no"
CARGO_LOCK_GUARD_CLEANED="no"
CARGO_LOCK_GUARD_STAT_COMMAND="${CARGO_LOCK_GUARD_STAT_COMMAND:-stat}"

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

cargo_lock_guard_capture_identity() {
  local destination="$1"
  local path="$2"
  local identity

  if ! command -v "${CARGO_LOCK_GUARD_STAT_COMMAND}" >/dev/null 2>&1; then
    return 1
  fi
  if ! identity="$("${CARGO_LOCK_GUARD_STAT_COMMAND}" -c '%d:%i' -- "${path}" 2>/dev/null)"; then
    return 1
  fi
  if [[ ! "${identity}" =~ ^[0-9]+:[0-9]+$ ]]; then
    return 1
  fi
  printf -v "${destination}" '%s' "${identity}"
}

cargo_lock_guard_prepare_anchor() {
  local path_identity
  local anchor_identity

  if [[ -L target || ( -e target && ! -d target ) ]]; then
    cargo_lock_guard_log "ERROR: target is not a local regular directory; file-instance anchoring is unavailable, so Cargo.lock is preserved"
    return 1
  fi
  if [[ ! -d target ]]; then
    mkdir -- "target" || return 1
    CARGO_LOCK_GUARD_CREATED_TARGET="yes"
  fi
  if ! CARGO_LOCK_GUARD_PRIVATE_DIR="$(mktemp -d -p "target" '.aerocodex-cargo-lock-guard.XXXXXX')"; then
    cargo_lock_guard_log "ERROR: cannot create the private file-instance anchor directory; Cargo.lock is preserved"
    return 1
  fi
  CARGO_LOCK_GUARD_ANCHOR="${CARGO_LOCK_GUARD_PRIVATE_DIR}/owned-instance"
  if ! ln -- "Cargo.lock" "${CARGO_LOCK_GUARD_ANCHOR}"; then
    cargo_lock_guard_log "ERROR: cannot create a hard-link anchor for the owned Cargo.lock instance; Cargo.lock is preserved"
    return 1
  fi
  if ! cargo_lock_guard_capture_identity path_identity "Cargo.lock" ||
    ! cargo_lock_guard_capture_identity anchor_identity "${CARGO_LOCK_GUARD_ANCHOR}"; then
    cargo_lock_guard_log "ERROR: filesystem file-instance identity is unavailable; Cargo.lock and its private anchor are preserved"
    return 1
  fi
  if [[ "${path_identity}" != "${CARGO_LOCK_GUARD_OWNED_IDENTITY}" ||
    "${anchor_identity}" != "${CARGO_LOCK_GUARD_OWNED_IDENTITY}" ]]; then
    cargo_lock_guard_log "ERROR: Cargo.lock changed instance while its ownership anchor was established; current paths are preserved"
    return 1
  fi
}

cargo_lock_guard_release_private_files() {
  local quarantined_path="${1:-}"
  local private_identity

  if [[ -n "${quarantined_path}" && ( -e "${quarantined_path}" || -L "${quarantined_path}" ) ]]; then
    if [[ ! -f "${quarantined_path}" || -L "${quarantined_path}" ]] ||
      ! cargo_lock_guard_capture_identity private_identity "${quarantined_path}" ||
      [[ "${private_identity}" != "${CARGO_LOCK_GUARD_OWNED_IDENTITY}" ]]; then
      return 1
    fi
  fi
  if [[ -n "${CARGO_LOCK_GUARD_ANCHOR}" && ( -e "${CARGO_LOCK_GUARD_ANCHOR}" || -L "${CARGO_LOCK_GUARD_ANCHOR}" ) ]]; then
    if [[ ! -f "${CARGO_LOCK_GUARD_ANCHOR}" || -L "${CARGO_LOCK_GUARD_ANCHOR}" ]] ||
      ! cargo_lock_guard_capture_identity private_identity "${CARGO_LOCK_GUARD_ANCHOR}" ||
      [[ "${private_identity}" != "${CARGO_LOCK_GUARD_OWNED_IDENTITY}" ]]; then
      return 1
    fi
  fi
  if [[ -n "${quarantined_path}" && ( -e "${quarantined_path}" || -L "${quarantined_path}" ) ]]; then
    rm -- "${quarantined_path}" || return 1
  fi
  if [[ -n "${CARGO_LOCK_GUARD_ANCHOR}" && ( -e "${CARGO_LOCK_GUARD_ANCHOR}" || -L "${CARGO_LOCK_GUARD_ANCHOR}" ) ]]; then
    rm -- "${CARGO_LOCK_GUARD_ANCHOR}" || return 1
  fi
  if [[ -n "${CARGO_LOCK_GUARD_PRIVATE_DIR}" && -d "${CARGO_LOCK_GUARD_PRIVATE_DIR}" ]]; then
    rmdir -- "${CARGO_LOCK_GUARD_PRIVATE_DIR}" || return 1
  fi
  if [[ "${CARGO_LOCK_GUARD_CREATED_TARGET}" == "yes" && -d target && ! -L target ]]; then
    rmdir -- "target" 2>/dev/null || true
  fi
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
  CARGO_LOCK_GUARD_STATE="ownership_ambiguous"
  if cargo_lock_guard_is_tracked || [[ ! -f Cargo.lock || -L Cargo.lock ]] || ! cargo_lock_guard_is_ignored; then
    cargo_lock_guard_log "ERROR: root Cargo.lock ownership became ambiguous immediately after creation; preserving it"
    return 1
  fi
  if ! cargo_lock_guard_capture_identity CARGO_LOCK_GUARD_OWNED_IDENTITY "Cargo.lock"; then
    cargo_lock_guard_log "ERROR: filesystem file-instance identity is unavailable immediately after Cargo.lock creation; preserving it"
    return 1
  fi
  if ! cargo_lock_guard_prepare_anchor; then
    return 1
  fi
  CARGO_LOCK_GUARD_STATE="owned"
  cargo_lock_guard_log "initial_state=absent created_owned_transient=yes identity=${CARGO_LOCK_GUARD_OWNED_IDENTITY} anchor=hard_link cleanup=quarantine_then_remove"
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
      if [[ ! -f "${CARGO_LOCK_GUARD_ANCHOR}" || -L "${CARGO_LOCK_GUARD_ANCHOR}" ]]; then
        cargo_lock_guard_log "ERROR: script-owned Cargo.lock file-instance anchor is missing or changed type; preserving the current path"
        return 1
      fi
      local current_identity
      local anchor_identity
      if ! cargo_lock_guard_capture_identity current_identity "Cargo.lock" ||
        ! cargo_lock_guard_capture_identity anchor_identity "${CARGO_LOCK_GUARD_ANCHOR}"; then
        cargo_lock_guard_log "ERROR: filesystem file-instance identity cannot be verified; preserving the current Cargo.lock"
        return 1
      fi
      if [[ "${current_identity}" != "${CARGO_LOCK_GUARD_OWNED_IDENTITY}" ||
        "${anchor_identity}" != "${CARGO_LOCK_GUARD_OWNED_IDENTITY}" ]]; then
        cargo_lock_guard_log "ERROR: script-owned Cargo.lock was replaced (owned=${CARGO_LOCK_GUARD_OWNED_IDENTITY} current=${current_identity} anchor=${anchor_identity}); preserving the replacement"
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
      local quarantined_path
      local quarantined_identity
      sha="$(cargo_lock_guard_hash)"
      size="$(stat -c%s -- "Cargo.lock")"
      quarantined_path="${CARGO_LOCK_GUARD_PRIVATE_DIR}/quarantined-instance"
      if ! mv -- "Cargo.lock" "${quarantined_path}"; then
        cargo_lock_guard_log "ERROR: could not quarantine the verified Cargo.lock instance; current paths are preserved"
        return 1
      fi
      if [[ ! -f "${quarantined_path}" || -L "${quarantined_path}" ]] ||
        ! cargo_lock_guard_capture_identity quarantined_identity "${quarantined_path}" ||
        [[ "${quarantined_identity}" != "${CARGO_LOCK_GUARD_OWNED_IDENTITY}" ]]; then
        if [[ ! -e Cargo.lock && ! -L Cargo.lock ]]; then
          mv -- "${quarantined_path}" "Cargo.lock" || true
        fi
        cargo_lock_guard_log "ERROR: quarantined Cargo.lock is not the owned file instance; the moved path was preserved/restored and no deletion occurred"
        return 1
      fi
      if [[ -e Cargo.lock || -L Cargo.lock ]]; then
        if ! cargo_lock_guard_release_private_files "${quarantined_path}"; then
          cargo_lock_guard_log "ERROR: a replacement appeared after quarantine and owned private files could not be finalized; the replacement was preserved"
          return 1
        fi
        CARGO_LOCK_GUARD_STATE="ownership_lost"
        cargo_lock_guard_log "ERROR: a replacement Cargo.lock appeared after quarantine; it was preserved and cleanup failed"
        return 1
      fi
      if ! cargo_lock_guard_release_private_files "${quarantined_path}"; then
        if [[ ! -e Cargo.lock && ! -L Cargo.lock ]]; then
          if [[ -e "${quarantined_path}" && ! -L "${quarantined_path}" ]]; then
            mv -- "${quarantined_path}" "Cargo.lock" || true
          elif [[ -e "${CARGO_LOCK_GUARD_ANCHOR}" && ! -L "${CARGO_LOCK_GUARD_ANCHOR}" ]]; then
            ln -- "${CARGO_LOCK_GUARD_ANCHOR}" "Cargo.lock" || true
          fi
        fi
        cargo_lock_guard_log "ERROR: verified owned Cargo.lock was quarantined but private cleanup failed"
        return 1
      fi
      if [[ -e Cargo.lock || -L Cargo.lock ]]; then
        CARGO_LOCK_GUARD_STATE="ownership_lost"
        cargo_lock_guard_log "ERROR: a replacement Cargo.lock appeared during finalization; it was preserved and cleanup failed"
        return 1
      fi
      CARGO_LOCK_GUARD_STATE="cleaned"
      CARGO_LOCK_GUARD_CLEANED="yes"
      cargo_lock_guard_log "removed_owned_transient=yes identity=${CARGO_LOCK_GUARD_OWNED_IDENTITY} quarantine_verified=yes sha256=${sha} size=${size}"
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
