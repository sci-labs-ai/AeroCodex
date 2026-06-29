#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

TOTAL_STEPS=13
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

if ! command -v git >/dev/null 2>&1; then
  info "ERROR: git was not found on the command search path"
  exit 127
fi

if ! command -v sha256sum >/dev/null 2>&1; then
  info "ERROR: sha256sum was not found on the command search path"
  info "Install GNU coreutils or use the PowerShell friend-test script on Windows."
  exit 127
fi

if command -v rustc >/dev/null 2>&1; then
  info "rustc: $(rustc --version)"
else
  info "rustc: not found on the command search path"
fi
info "cargo: $(cargo --version)"

if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  info "git commit: $(git log -1 --format=%h)"
fi

run_step "git status --short" \
  git status --short
run_step "git diff --check" \
  git diff --check
run_step "sha256sum -c checksums/SHA256SUMS" \
  sha256sum -c checksums/SHA256SUMS
run_step "cargo fmt --all -- --check" \
  cargo fmt --all -- --check
run_step "cargo check --workspace --all-targets --all-features" \
  cargo check --workspace --all-targets --all-features
run_step "cargo clippy --workspace --all-targets --all-features -- -D warnings" \
  cargo clippy --workspace --all-targets --all-features -- -D warnings
run_step "cargo test --workspace --all-targets --all-features" \
  cargo test --workspace --all-targets --all-features
run_step "cargo run -p aero-codex-cli -- version --json" \
  cargo run -p aero-codex-cli -- version --json
run_step "cargo run -p aero-codex-cli -- run canonical distance smoke" \
  cargo run -p aero-codex-cli -- run formula_vault.m00.canonical.distance_to_canonical distance=-42 distance_unit=7 --json
run_step "cargo run -p aero-codex-cli -- self-check --json" \
  cargo run -p aero-codex-cli -- self-check --json
run_step "cargo run -p xtask -- verify --all" \
  cargo run -p xtask -- verify --all
run_step "cargo run -p xtask -- dependency-policy" \
  cargo run -p xtask -- dependency-policy
run_shell_step "RUSTDOCFLAGS=\"-D warnings\" cargo doc --workspace --all-features --no-deps" \
  'RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps'

if [[ -f Cargo.lock ]]; then
  info "NOTE: a root Cargo.lock exists after the run. Do not submit it unless project policy changes."
fi

info "completed all requested local checks"
info "Reminder: passing local checks does not prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval."
