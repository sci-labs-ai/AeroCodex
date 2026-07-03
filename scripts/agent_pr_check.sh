#!/usr/bin/env bash
set -euo pipefail

TOTAL_STEPS=14
CURRENT_STEP=0
PRE_ROOT_CARGO_LOCK="absent"

info() {
  printf '[agent-pr-check] %s\n' "$*"
}

fail() {
  info "ERROR: $*"
  exit 1
}

print_command() {
  printf '[agent-pr-check] +'
  printf ' %q' "$@"
  printf '\n'
}

cleanup_cargo_lock() {
  if [[ "${PRE_ROOT_CARGO_LOCK}" == "absent" && -e Cargo.lock ]]; then
    if git status --porcelain --untracked-files=all -- Cargo.lock | grep -q '^?? Cargo.lock$'; then
      local sha
      local size
      sha="$(sha256sum Cargo.lock | awk '{print $1}')"
      size="$(stat -c%s Cargo.lock)"
      rm Cargo.lock
      info "removed_generated_untracked_root_Cargo_lock=yes sha256=${sha} size=${size}"
    fi
  fi
}

trap 'cleanup_cargo_lock || true' EXIT

run_step() {
  local label="$1"
  shift
  CURRENT_STEP=$((CURRENT_STEP + 1))
  info "== step ${CURRENT_STEP}/${TOTAL_STEPS}: ${label} =="
  print_command "$@"
  "$@"
  cleanup_cargo_lock
}

run_shell_step() {
  local label="$1"
  local command_text="$2"
  CURRENT_STEP=$((CURRENT_STEP + 1))
  info "== step ${CURRENT_STEP}/${TOTAL_STEPS}: ${label} =="
  info "+ ${command_text}"
  bash -o pipefail -c "${command_text}"
  cleanup_cargo_lock
}

require_command() {
  local command_name="$1"
  if ! command -v "${command_name}" >/dev/null 2>&1; then
    fail "required local command not found on PATH: ${command_name}"
  fi
}

preflight_required_xtask_commands() {
  [[ -f xtask/src/main.rs ]] || fail "missing xtask/src/main.rs; cannot verify local handoff commands"

  if ! grep -F '["verify"] | ["verify", "--all"]' xtask/src/main.rs >/dev/null; then
    fail "stop condition: xtask verify --all command is not present yet; leave this script staged/draft until the prerequisite merges"
  fi

  if ! grep -F '["formula-registry", "check"' xtask/src/main.rs >/dev/null; then
    fail "stop condition: xtask formula-registry check command is not present yet; RR-017 must be merged before this script is blocking"
  fi

  if ! grep -F '["equation-batch", "plan"' xtask/src/main.rs >/dev/null; then
    fail "stop condition: xtask equation-batch plan command is not present yet"
  fi

  if ! grep -F '["equation-batch", "report"' xtask/src/main.rs >/dev/null; then
    fail "stop condition: xtask equation-batch report command is not present yet"
  fi

  if ! grep -F '["dependency-policy"]' xtask/src/main.rs >/dev/null; then
    fail "stop condition: xtask dependency-policy command is not present yet"
  fi

  info "xtask_command_preflight=PASS"
}

forbidden_claim_scan() {
  local forbidden_claim_pattern
  local log_file
  local err_file
  local grep_rc
  local candidates
  local scan_paths=()

  forbidden_claim_pattern='NASA-ready|flight-ready|mission-ready|certified for|life-support certified|life-support safe|habitat-safe|operationally approved|regulatory approved'
  log_file="$(mktemp -t aerocodex_forbidden_claim_matches.XXXXXX)"
  err_file="$(mktemp -t aerocodex_forbidden_claim_errors.XXXXXX)"
  candidates=(scripts docs README.md .github xtask crates tests formula-schemas schemas)

  for candidate in "${candidates[@]}"; do
    if [[ -e "${candidate}" ]]; then
      scan_paths+=("${candidate}")
    fi
  done

  if [[ "${#scan_paths[@]}" -eq 0 ]]; then
    fail "forbidden-claim scan has no existing paths to review"
  fi

  grep_rc=0
  grep -RniE "${forbidden_claim_pattern}" "${scan_paths[@]}" >"${log_file}" 2>"${err_file}" || grep_rc=$?
  if [[ "${grep_rc}" -gt 1 ]]; then
    info "forbidden_claim_grep_errors_begin"
    cat "${err_file}"
    info "forbidden_claim_grep_errors_end"
    fail "forbidden-claim grep failed with exit code ${grep_rc}"
  fi

  info "forbidden_claim_matches_begin"
  if [[ -s "${log_file}" ]]; then
    cat "${log_file}"
  else
    info "no forbidden-claim phrase matches found"
  fi
  info "forbidden_claim_matches_end"

  python3 - "${log_file}" <<'PY'
from pathlib import Path
import re
import sys

log_path = Path(sys.argv[1])
phrases = [
    "nasa-ready",
    "flight-ready",
    "mission-ready",
    "certified for",
    "life-support certified",
    "life-support safe",
    "habitat-safe",
    "operationally approved",
    "regulatory approved",
]
allow_cues = [
    " not ",
    " no ",
    " never ",
    " without ",
    " does not ",
    " do not ",
    " must not ",
    " should not ",
    " cannot ",
    " can not ",
    " forbidden",
    " prohibited",
    " disallow",
    " blocked",
    " guardrail",
    " non-claim",
    " nonclaim",
    " negative",
    " caveat",
    " avoid",
    " not certified",
    "not formula validation",
    "not certification",
    "does not prove",
    "does not replace",
    "required_public_forbidden_claim_phrases",
    "forbidden_claim_pattern",
    "forbidden_claim_scan",
]

failures = []
reviewed = 0
for raw in log_path.read_text(encoding="utf-8", errors="replace").splitlines():
    if not raw.strip():
        continue
    reviewed += 1
    match = re.match(r"^(.*?):(\d+):(.*)$", raw)
    if not match:
        failures.append((raw, "could not parse grep output"))
        continue
    rel_path, line_text = match.group(1), match.group(3)
    line_no = int(match.group(2))
    path = Path(rel_path)
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        start = max(1, line_no - 8)
        end = min(len(lines), line_no + 4)
        context = "\n".join(lines[start - 1:end])
        block_start = line_no - 1
        while block_start > 0 and lines[block_start - 1].strip():
            block_start -= 1
        block_end = line_no - 1
        while block_end < len(lines) and lines[block_end].strip():
            block_end += 1
        block_context = "\n".join(lines[block_start:block_end])
    except OSError:
        context = line_text
        block_context = line_text
    normalized_context = " " + re.sub(r"[^a-z0-9_./-]+", " ", (context + "\n" + block_context).lower()) + " "
    normalized_line = " " + re.sub(r"[^a-z0-9_./-]+", " ", line_text.lower()) + " "
    has_phrase = any(phrase in normalized_line for phrase in phrases)
    allowed = has_phrase and any(cue in normalized_context for cue in allow_cues)
    if not allowed:
        failures.append((raw, "no nearby negative/prohibition/non-claim cue"))

if failures:
    print("forbidden_claim_review=FAIL")
    for raw, reason in failures:
        print(f"positive_or_ambiguous_forbidden_claim: {reason}: {raw}")
    sys.exit(1)

print(f"forbidden_claim_review=PASS reviewed_matches={reviewed}")
PY

  rm -f "${log_file}" "${err_file}"
}

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[[ -n "${REPO_ROOT}" ]] || fail "not inside a Git repository; run from an AeroCodex checkout"
cd "${REPO_ROOT}"

info "AeroCodex local agent PR handoff check"
info "repository_root=${REPO_ROOT}"
info "command_shape=bash scripts/agent_pr_check.sh"
info "local_only=yes"
info "network_github_auth_pr_merge_branch_delete_behavior=none"

for command_name in cargo git grep python3 sha256sum stat awk mktemp; do
  require_command "${command_name}"
done

if [[ -e Cargo.lock ]]; then
  PRE_ROOT_CARGO_LOCK="present"
fi
info "pre_root_Cargo_lock=${PRE_ROOT_CARGO_LOCK}"

run_step "preflight required local xtask command contracts" preflight_required_xtask_commands
run_step "git diff --check" git diff --check
run_step "cargo fmt --check" cargo fmt --check
run_step "cargo check --workspace --all-targets --all-features" cargo check --workspace --all-targets --all-features
run_step "cargo clippy --all-targets --all-features -- -D warnings" cargo clippy --all-targets --all-features -- -D warnings
run_step "cargo test --all" cargo test --all
run_step "cargo doc --no-deps" cargo doc --no-deps
run_step "cargo run -p xtask -- verify --all" cargo run -p xtask -- verify --all
run_shell_step "equation-batch plan JSON check" 'cargo run -p xtask -- equation-batch plan --all-manifests --json > /tmp/equation_batch_plan.json && python3 -m json.tool /tmp/equation_batch_plan.json >/dev/null'
run_step "equation-batch status report check" cargo run -p xtask -- equation-batch report --all-manifests --out generated/equation_batch_status_report.json --check
run_step "cargo run -p xtask -- formula-registry check" cargo run -p xtask -- formula-registry check
run_step "cargo run -p xtask -- dependency-policy" cargo run -p xtask -- dependency-policy
run_step "cargo run -p aero-codex-cli -- self-check --json" cargo run -p aero-codex-cli -- self-check --json
run_step "forbidden-claim grep/review gate" forbidden_claim_scan

cleanup_cargo_lock
if [[ "${PRE_ROOT_CARGO_LOCK}" == "absent" && -e Cargo.lock ]]; then
  fail "root Cargo.lock exists after cleanup; it was absent before the run and was not safely removable as an untracked transient file"
fi

info "root_Cargo_lock_final=$([[ -e Cargo.lock ]] && printf present || printf absent)"
info "agent_pr_check=PASS"
