#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

TOTAL_STEPS=16
CURRENT_STEP=0

info() {
  printf '[friend-test] %s\n' "$*"
}

run_step() {
  CURRENT_STEP=$((CURRENT_STEP + 1))
  local label="$1"
  shift
  info "step ${CURRENT_STEP}/${TOTAL_STEPS}: ${label}"
  "$@"
}

run_shell_step() {
  CURRENT_STEP=$((CURRENT_STEP + 1))
  local label="$1"
  local command_text="$2"
  info "step ${CURRENT_STEP}/${TOTAL_STEPS}: ${label}"
  bash -lc "${command_text}"
}

info "AeroCodex local friend-test package"
info "repository root: ${REPO_ROOT}"

if ! command -v cargo >/dev/null 2>&1; then
  info "ERROR: cargo was not found on the command search path"
  info "Install Rust with cargo, rustfmt, and clippy before running the friend-test package."
  exit 127
fi

if command -v rustc >/dev/null 2>&1; then
  info "rustc: $(rustc --version)"
else
  info "rustc: not found on the command search path"
fi
info "cargo: $(cargo --version)"

IN_GIT_CHECKOUT=0
if command -v git >/dev/null 2>&1 && git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  IN_GIT_CHECKOUT=1
  info "git commit: $(git log -1 --format=%h)"
else
  info "source archive mode: Git metadata is not present"
fi

FRIEND_BINARY="${AEROCODEX_FRIEND_BINARY:-}"
EXPECTED_COMMIT="${AEROCODEX_EXPECTED_COMMIT:-}"
if [[ -n "${FRIEND_BINARY}" ]]; then
  if [[ ! -x "${FRIEND_BINARY}" ]]; then
    info "ERROR: AEROCODEX_FRIEND_BINARY is not executable: ${FRIEND_BINARY}"
    exit 126
  fi
  info "downloaded binary: ${FRIEND_BINARY}"
fi

source_status() {
  if [[ "${IN_GIT_CHECKOUT}" -eq 1 ]]; then
    git status --short
  else
    test -f Cargo.lock
    test -f release/release-manifest.sha256
    info "source archive contains the committed lockfile and manifest sidecar"
  fi
}

source_diff_check() {
  if [[ "${IN_GIT_CHECKOUT}" -eq 1 ]]; then
    git diff --check
  else
    info "git diff is not applicable to a source archive; governed checksums run next"
  fi
}

run_public_cli() {
  if [[ -n "${FRIEND_BINARY}" ]]; then
    "${FRIEND_BINARY}" "$@"
  else
    cargo run --locked -p aero-codex-cli -- "$@"
  fi
}

verify_all_or_source_archive() {
  if [[ "${IN_GIT_CHECKOUT}" -eq 1 ]]; then
    cargo run --locked -p xtask -- verify --all
  else
    cargo run --locked -p xtask -- verify-source-archive
  fi
}

test_workspace_or_source_archive() {
  if [[ "${IN_GIT_CHECKOUT}" -eq 1 ]]; then
    cargo test --locked --workspace --all-targets --all-features
  else
    info "source archive omits Git history; skipping the two Git-history-only xtask fixtures"
    cargo test --locked --workspace --all-targets --all-features -- \
      --skip release_identity::tests::complete_public_verifier_rejects_reported_document_bypasses \
      --skip release_identity::tests::production_command_authenticates_exact_objects_and_both_merge_parents
  fi
}

verify_release_identity_or_archive() {
  if [[ "${IN_GIT_CHECKOUT}" -eq 1 ]]; then
    cargo run --locked -p xtask -- verify-release-identity
    return
  fi
  if [[ -z "${FRIEND_BINARY}" || -z "${EXPECTED_COMMIT}" ]]; then
    info "ERROR: source archive mode requires AEROCODEX_FRIEND_BINARY and AEROCODEX_EXPECTED_COMMIT"
    return 2
  fi
  local version_json actual_commit actual_manifest expected_manifest
  version_json="$("${FRIEND_BINARY}" version --json)"
  actual_commit="$(printf '%s' "${version_json}" | sed -n 's/.*"build_commit":"\([^"]*\)".*/\1/p')"
  actual_manifest="$(printf '%s' "${version_json}" | sed -n 's/.*"release_manifest_sha256":"\([0-9a-f]*\)".*/\1/p')"
  expected_manifest="$(sed -n 's/^\([0-9a-f]\{64\}\)  release\/release-manifest.toml$/\1/p' release/release-manifest.sha256)"
  if [[ "${actual_commit}" != "${EXPECTED_COMMIT}" ]]; then
    info "ERROR: downloaded binary build_commit=${actual_commit:-missing}, expected ${EXPECTED_COMMIT}"
    return 1
  fi
  if [[ "${actual_manifest}" != "${expected_manifest}" ]]; then
    info "ERROR: downloaded binary manifest hash=${actual_manifest:-missing}, expected ${expected_manifest}"
    return 1
  fi
  info "downloaded binary commit and manifest hash match the source archive"
}

verify_release_manifest_or_archive() {
  if [[ "${IN_GIT_CHECKOUT}" -eq 1 ]]; then
    cargo run --locked -p xtask -- verify-release-manifest
  else
    cargo run --locked -p xtask -- verify-release-manifest-source-archive
  fi
}

verify_generated_or_source_archive() {
  if [[ "${IN_GIT_CHECKOUT}" -eq 1 ]]; then
    cargo run --locked -p xtask -- verify-generated
  else
    cargo run --locked -p xtask -- equation-batch report --all-manifests \
      --out generated/equation_batch_status_report.json --check
    cargo run --locked -p xtask -- verify-release-slice-validation
    cargo run --locked -p xtask -- formula-registry check
    cargo run --locked -p xtask -- verify-checksums
  fi
}

run_step "git status --short" \
  source_status
run_step "git diff --check" \
  source_diff_check
run_step "cargo run --locked -p xtask -- verify-checksums" \
  cargo run --locked -p xtask -- verify-checksums
run_step "cargo fmt --all -- --check" \
  cargo fmt --all -- --check
run_step "cargo check --locked --workspace --all-targets --all-features" \
  cargo check --locked --workspace --all-targets --all-features
run_step "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings" \
  cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
run_step "cargo test --locked --workspace --all-targets --all-features" \
  test_workspace_or_source_archive
run_step "cargo run --locked -p aero-codex-cli -- version --json" \
  run_public_cli version --json
run_step "cargo run --locked -p aero-codex-cli -- formula status-report --json" \
  run_public_cli formula status-report --json
run_step "cargo run --locked -p aero-codex-cli -- self-check --json" \
  run_public_cli self-check --json
run_step "cargo run --locked -p xtask -- verify --all" \
  verify_all_or_source_archive
run_step "cargo run --locked -p xtask -- verify-release-manifest" \
  verify_release_manifest_or_archive
run_step "cargo run --locked -p xtask -- verify-release-identity" \
  verify_release_identity_or_archive
run_step "cargo run --locked -p xtask -- verify-generated" \
  verify_generated_or_source_archive
run_step "cargo run --locked -p xtask -- dependency-policy" \
  cargo run --locked -p xtask -- dependency-policy
run_shell_step "RUSTDOCFLAGS=\"-D warnings\" cargo doc --locked --workspace --all-features --no-deps" \
  'RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps'

info "committed Cargo.lock was used with --locked"

info "completed all requested local checks"
info "Reminder: passing local checks does not prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval."
