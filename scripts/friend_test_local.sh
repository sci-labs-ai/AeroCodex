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

if ! command -v git >/dev/null 2>&1; then
  info "ERROR: git was not found on the command search path"
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
run_step "cargo run --locked -p xtask -- verify-checksums" \
  cargo run --locked -p xtask -- verify-checksums
run_step "cargo fmt --all -- --check" \
  cargo fmt --all -- --check
run_step "cargo check --locked --workspace --all-targets --all-features" \
  cargo check --locked --workspace --all-targets --all-features
run_step "cargo clippy --locked --workspace --all-targets --all-features -- -D warnings" \
  cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
run_step "cargo test --locked --workspace --all-targets --all-features" \
  cargo test --locked --workspace --all-targets --all-features
run_step "cargo run --locked -p aero-codex-cli -- version --json" \
  cargo run --locked -p aero-codex-cli -- version --json
run_step "cargo run --locked -p aero-codex-cli -- formula status-report --json" \
  cargo run --locked -p aero-codex-cli -- formula status-report --json
run_step "cargo run --locked -p aero-codex-cli -- self-check --json" \
  cargo run --locked -p aero-codex-cli -- self-check --json
run_step "cargo run --locked -p xtask -- verify --all" \
  cargo run --locked -p xtask -- verify --all
run_step "cargo run --locked -p xtask -- verify-release-manifest" \
  cargo run --locked -p xtask -- verify-release-manifest
run_step "cargo run --locked -p xtask -- verify-release-identity" \
  cargo run --locked -p xtask -- verify-release-identity
run_step "cargo run --locked -p xtask -- verify-generated" \
  cargo run --locked -p xtask -- verify-generated
run_step "cargo run --locked -p xtask -- dependency-policy" \
  cargo run --locked -p xtask -- dependency-policy
run_shell_step "RUSTDOCFLAGS=\"-D warnings\" cargo doc --locked --workspace --all-features --no-deps" \
  'RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps'

info "committed Cargo.lock was used with --locked"

info "completed all requested local checks"
info "Reminder: passing local checks does not prove physical validity, safety, certification, mission readiness, habitat safety, medical suitability, or regulated-use approval."
