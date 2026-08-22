#![forbid(unsafe_code)]

use std::{
    env,
    ffi::OsStr,
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn binary_path() -> PathBuf {
    let mut path = env::current_exe().expect("test executable path should be available");
    path.pop();
    if path.file_name() == Some(OsStr::new("deps")) {
        path.pop();
    }
    path.push("aerocodex");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

fn run(arguments: &[&str]) -> Output {
    Command::new(binary_path())
        .args(arguments)
        .output()
        .expect("aerocodex binary should execute")
}

fn run_with_env(arguments: &[&str], key: &str, value: &str) -> Output {
    Command::new(binary_path())
        .args(arguments)
        .env(key, value)
        .output()
        .expect("aerocodex binary should execute")
}

fn output_with_executable_busy_retry(command: &mut Command) -> std::io::Result<Output> {
    const MAX_ATTEMPTS: usize = 5;

    for attempt in 1..=MAX_ATTEMPTS {
        match command.output() {
            Err(error)
                if cfg!(unix)
                    && error.raw_os_error() == Some(26)
                    && attempt < MAX_ATTEMPTS =>
            {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            result => return result,
        }
    }
    unreachable!("bounded copied-CLI execution attempts must return")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be valid text")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be valid text")
}

fn assert_json_semantics_with_python(text: &str, script: &str) {
    let python = if cfg!(windows) { "python" } else { "python3" };
    let mut parser = Command::new(python)
        .arg("-c")
        .arg(script)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("python3 should be available for CLI JSON semantic checks");
    {
        use std::io::Write as _;
        parser
            .stdin
            .as_mut()
            .expect("parser stdin should be open")
            .write_all(text.as_bytes())
            .expect("JSON text should be writable to parser stdin");
    }
    let parsed = parser
        .wait_with_output()
        .expect("python3 JSON semantic parser should exit");
    assert!(
        parsed.status.success(),
        "JSON semantic check failed; stdout={} stderr={} json={text}",
        String::from_utf8_lossy(&parsed.stdout),
        String::from_utf8_lossy(&parsed.stderr)
    );
}

fn assert_success_json_envelope(text: &str, command: &str) {
    assert!(
        text.starts_with("{\"ok\":true"),
        "missing ok=true envelope in {text}"
    );
    assert!(
        text.contains(&format!("\"command\":\"{command}\"")),
        "missing command `{command}` in {text}"
    );
    assert!(
        text.contains("\"safety_notice\":"),
        "missing safety_notice in {text}"
    );
    assert!(
        text.contains("\"registry_schema_version\":"),
        "missing registry_schema_version in {text}"
    );
    assert!(text.contains("\"warnings\":"), "missing warnings in {text}");
    assert!(
        text.contains("\"error\":null"),
        "missing error=null in {text}"
    );
}

fn assert_error_json_envelope(text: &str, command: &str, code: &str) {
    assert!(
        text.starts_with("{\"ok\":false"),
        "missing ok=false envelope in {text}"
    );
    assert!(
        text.contains(&format!("\"command\":\"{command}\"")),
        "missing command `{command}` in {text}"
    );
    assert!(
        text.contains(&format!("\"code\":\"{code}\"")),
        "missing error code `{code}` in {text}"
    );
    assert!(
        text.contains("\"message\":"),
        "missing error.message in {text}"
    );
    assert!(
        text.contains("\"safety_notice\":"),
        "missing safety_notice in {text}"
    );
}

#[test]
fn version_json_exposes_bounded_release_identity() {
    let output = run(&["version", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "version");
    assert!(text.contains("\"package_version\":\"0.1.0-alpha.1\""));
    assert!(text.contains("\"program_name\":\"aerocodex\""));
    assert!(text.contains("\"semantic_version\":\"0.1.0-alpha.1\""));
    assert!(text.contains("\"release_tier\":\"research_software_alpha\""));
    assert!(text.contains("\"release_tier_display\":\"Research Software Alpha\""));
    assert!(text.contains("\"workspace_package_count\":14"));
    assert!(text.contains("\"build_commit\":"));
    assert!(text.contains("\"build_target\":"));
    assert!(text.contains("\"build_profile\":"));
    assert!(text.contains("\"supported_formula_count\":12"));
    assert!(text.contains("\"registry_formula_count\":152"));
    assert!(text.contains("\"blocked_formula_count\":152"));
    assert!(text.contains("\"public_executable_formula_count\":0"));
    assert!(text.contains("\"validation_status\":\"research_required\""));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn standard_version_uses_cargo_package_version() {
    let output = run(&["--version"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "aerocodex 0.1.0-alpha.1\n");
    assert!(stderr(&output).is_empty());
}

#[test]
fn formula_catalog_is_registry_backed_and_labeled() {
    let output = run(&["formula", "list", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "formula_list");
    assert!(text.contains("\"command\":\"formula_list\""));
    assert!(text.contains("\"count\":152"));
    assert!(text.contains("\"registry_formula_count\":152"));
    assert!(text.contains("\"registry_schema_version\":\"aerocodex.formula_registry.v1\""));
    assert!(text.contains("\"source_hash\":\"sha256:b5a16a99f20a0bea420b6c3842894c316e958a8934b34556bfef2e1799586c3e\""));
    assert!(text.contains("\"formula_id\":\"m00.canonical.distance_to_canonical\""));
    assert!(text.contains("\"formula_id\":\"aerodynamics.coefficients.drag_coefficient\""));
    assert!(text.contains("\"status\":\"research_required\""));
    assert!(text.contains("\"execution_policy\":\"blocked\""));
    assert!(text.contains("\"family\":\"m00\""));
    assert!(text.contains("\"family\":\"aerodynamics\""));
    assert!(text.contains("\"output\":"));
    assert!(text.contains("\"runtime_symbol\":"));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn formula_catalog_family_filter_is_registry_backed() {
    let output = run(&["formula", "list", "--family", "m00", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "formula_list");
    assert!(text.contains("\"command\":\"formula_list\""));
    assert!(text.contains("\"filters\":{\"family\":\"m00\""));
    assert!(text.contains("\"formula_id\":\"m00.canonical.distance_to_canonical\""));
    assert!(!text.contains("\"formula_id\":\"aerodynamics.coefficients.drag_coefficient\""));
}

#[test]
fn formula_catalog_unknown_flags_fail_closed() {
    let output = run(&["formula", "list", "--definitely-unknown-rr020-flag"]);
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert!(text.contains("unknown formula list option"));
}

#[test]
fn legacy_formula_catalog_json_is_marked_as_deprecated_alias() {
    let output = run(&["formulas", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "formulas");
    assert!(text.contains("\"command\":\"formulas\""));
    assert!(text.contains("\"count\":152"));
    assert!(text.contains("\"deprecated_alias\":true"));
    assert!(text.contains("\"migration_command\":\"aerocodex formula list\""));
    assert!(text.contains("\"validation_status\":\"research_required\""));
    assert!(text.contains("\"execution_policy\":\"blocked\""));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn formula_namespace_describes_checked_in_registry_formula_id() {
    let output = run(&[
        "formula",
        "describe",
        "aerodynamics.coefficients.drag_coefficient",
        "--json",
    ]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "formula describe");
    assert!(text.contains("\"command\":\"formula describe\""));
    assert!(text.contains("\"formula_id\":\"aerodynamics.coefficients.drag_coefficient\""));
    assert!(text.contains("\"status\":\"research_required\""));
    assert!(text.contains("\"execution_policy\":\"blocked\""));
    assert!(text.contains("\"family\":\"aerodynamics.coefficients\""));
    assert!(text.contains("\"source_trace\":{"));
    assert!(text.contains("\"contract_path\":"));
    assert!(text.contains("\"validation_card_path\":"));
    assert!(text.contains("\"source_seed_path\":"));
    assert!(text.contains("\"domain_constraints\":"));
    assert!(text.contains("\"implementation_path\":"));
    assert!(text.contains("\"warnings\":"));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn formula_describe_human_output_includes_traceability_fields() {
    let output = run(&["formula", "describe", "m00.canonical.time_unit_from_mu_du"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    for term in [
        "formula_id=",
        "legacy_formula_id=",
        "family=",
        "status=",
        "execution_policy=",
        "quarantine_state=",
        "inputs=",
        "outputs=",
        "units=",
        "domain_constraints=",
        "implementation_path=",
        "contract_path=",
        "validation_card_path=",
        "source_seed_path=",
        "warnings=",
    ] {
        assert!(text.contains(term), "missing `{term}` in:\n{text}");
    }
}

#[test]
fn legacy_describe_json_preserves_alias_traceability() {
    let output = run(&[
        "describe",
        "formula_vault.m00.canonical.distance_to_canonical",
        "--json",
    ]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "describe");
    assert!(text.contains("\"command\":\"describe\""));
    assert!(text.contains("\"deprecated_alias\":true"));
    assert!(text.contains("\"canonical_formula_id\":\"m00.canonical.distance_to_canonical\""));
    assert!(text.contains("\"alias_used\":\"formula_vault.m00.canonical.distance_to_canonical\""));
    assert!(text.contains("\"migration_command\":\"aerocodex formula describe <formula-id>\""));
    for key in [
        "\"legacy_formula_id\":",
        "\"family\":",
        "\"status\":",
        "\"execution_policy\":",
        "\"quarantine_state\":",
        "\"inputs\":",
        "\"outputs\":",
        "\"units\":",
        "\"domain_constraints\":",
        "\"implementation_path\":",
        "\"source_trace\":",
        "\"contract_path\":",
        "\"validation_card_path\":",
        "\"source_seed_path\":",
        "\"warnings\":",
    ] {
        assert!(text.contains(key), "missing `{key}` in {text}");
    }
}

#[test]
fn formula_status_report_human_is_registry_backed_and_honest() {
    let output = run(&["formula", "status-report"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    for term in [
        "Formula status report",
        "command=formula status-report",
        "registry_formula_count=152",
        "total_formula_count=152",
        "status.research_required=152",
        "execution_policy.blocked=152",
        "blocked_formula_count=152",
        "preliminary_only_formula_count=0",
        "executable_formula_count=0",
        "m07_candidate_count=0",
        "promotion_candidate_count=0",
        "by_family.life=50",
        "safety_notice=",
    ] {
        assert!(text.contains(term), "missing `{term}` in:\n{text}");
    }
}

#[test]
fn formula_status_report_json_includes_consistent_registry_counts() {
    let output = run(&["formula", "status-report", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "formula status-report");
    assert_json_semantics_with_python(
        &text,
        r#"
import json, sys
report = json.load(sys.stdin)
assert report["ok"] is True
assert report["command"] == "formula status-report"
assert report["error"] is None
assert isinstance(report["warnings"], list)
assert report["registry_schema_version"] == "aerocodex.formula_registry.v1"
assert report["registry_formula_count"] == 152
assert report["program_name"] == "aerocodex"
assert report["semantic_version"] == "0.1.0-alpha.1"
assert report["release_tier"] == "research_software_alpha"
assert report["release_tier_display"] == "Research Software Alpha"
assert report["total_formula_count"] == 152
assert report["inventory_formula_count"] == 152
assert report["total_formula_count"] == sum(report["counts_by_status"].values())
assert report["total_formula_count"] == sum(report["counts_by_execution_policy"].values())
assert report["execution_policy_bucket_total"] == report["total_formula_count"]
assert report["counts_by_status"] == {"research_required": 152}
assert report["counts_by_execution_policy"] == {"blocked": 152}
assert report["blocked_formula_count"] == 152
assert report["normal_executable_count"] == 0
assert report["executable_formula_count"] == 0
assert report["public_executable_formula_count"] == 0
assert report["preliminary_only_formula_count"] == 0
assert report["m07_candidate_count"] == 0
assert report["promotion_candidate_count"] == 0
for category in ["blocked_formulas", "normal_executable_formulas", "preliminary_only_formulas", "m07_candidates", "promotion_candidates"]:
    assert category in report["categories"]
assert report["categories"]["blocked_formulas"]["count"] == 152
assert report["categories"]["m07_candidates"]["count"] == 0
assert report["categories"]["promotion_candidates"]["count"] == 0
assert report["by_family"]["life"] == 50
assert "safety_notice" in report and report["safety_notice"]
"#,
    );
    for term in [
        "\"registry_schema_version\":\"aerocodex.formula_registry.v1\"",
        "\"source_hash\":\"sha256:b5a16a99f20a0bea420b6c3842894c316e958a8934b34556bfef2e1799586c3e\"",
        "\"registry_formula_count\":152",
        "\"total_formula_count\":152",
        "\"counts_by_status\":{\"research_required\":152}",
        "\"counts_by_execution_policy\":{\"blocked\":152}",
        "\"blocked_formula_count\":152",
        "\"preliminary_only_formula_count\":0",
        "\"executable_formula_count\":0",
        "\"m07_candidate_count\":0",
        "\"promotion_candidate_count\":0",
        "\"by_family\":{",
        "\"life\":50",
        "\"categories\":{",
        "\"m07_candidates\":{\"count\":0",
        "\"promotion_candidates\":{\"count\":0",
    ] {
        assert!(text.contains(term), "missing `{term}` in {text}");
    }
}

#[test]
fn formula_help_prioritizes_namespace_and_lists_legacy_aliases() {
    let output = run(&["--help"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    let namespace = text
        .find("aerocodex formula list [--family <family>] [--status <status>] [--executable] [--json]")
        .expect("formula namespace should be documented");
    let legacy = text
        .find("legacy aliases")
        .expect("legacy aliases should be documented");
    assert!(
        namespace < legacy,
        "help should show formula namespace first"
    );
    assert!(text.contains("aerocodex formula describe <formula-id> [--json]"));
    assert!(text.contains("aerocodex formula status-report [--json]"));
    assert!(text.contains(
        "aerocodex formula run <formula-id> [--preliminary] [--input-name <value> ...] [--json]"
    ));
    assert!(text.contains("legacy name=value assignments remain compatibility syntax"));
}

fn assert_execution_gate_blocked_json(
    text: &str,
    command: &str,
    formula_id: &str,
    code: &str,
    status: &str,
    execution_policy: &str,
) {
    assert_error_json_envelope(text, command, code);
    assert!(
        text.contains(&format!("\"formula_id\":\"{formula_id}\"")),
        "missing gated formula id `{formula_id}` in {text}"
    );
    assert!(
        text.contains(&format!("\"status\":\"{status}\"")),
        "missing status `{status}` in {text}"
    );
    assert!(
        text.contains(&format!("\"execution_policy\":\"{execution_policy}\"")),
        "missing execution_policy `{execution_policy}` in {text}"
    );
    assert!(
        !text.contains("\"error\":null"),
        "blocked execution must not emit error=null: {text}"
    );
}

#[test]
fn execution_gate_blocks_research_required_registry_row_json() {
    let output = run(&[
        "formula",
        "run",
        "aerodynamics.coefficients.drag_coefficient",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "formula run",
        "aerodynamics.coefficients.drag_coefficient",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
}

#[test]
fn execution_gate_blocks_registry_backed_m00_default_run_before_dispatch() {
    let output = run(&[
        "formula",
        "run",
        "m00.canonical.distance_to_canonical",
        "distance=-42",
        "distance_unit=7",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "formula run",
        "m00.canonical.distance_to_canonical",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
    assert!(
        !text.contains("\"value\":"),
        "status gate must run before dispatch: {text}"
    );
}

#[test]
fn execution_gate_blocks_legacy_alias_through_same_policy() {
    let output = run(&[
        "run",
        "formula_vault.m00.canonical.distance_to_canonical",
        "distance=1",
        "distance_unit=0",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "run",
        "m00.canonical.distance_to_canonical",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
    assert!(text.contains("\"code\":\"execution_blocked_by_status\""));
    assert!(
        !text.contains("non_positive_input"),
        "status gate must run before equation/domain dispatch: {text}"
    );
}

#[test]
fn formula_run_flag_style_input_parses_and_then_respects_status_gate() {
    let output = run(&[
        "formula",
        "run",
        "m00.angle.deg_to_rad",
        "--degrees",
        "180",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "formula run",
        "m00.angle.deg_to_rad",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
    assert!(
        !text.contains("invalid_assignment") && !text.contains("usage_error"),
        "RR-022 flag-style parser must accept scalar flags before the RR-025 status gate blocks dispatch: {text}"
    );
}

#[test]
fn formula_run_flag_style_negative_number_reaches_status_gate() {
    let output = run(&[
        "formula",
        "run",
        "m00.angle.deg_to_rad",
        "--degrees",
        "-180",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "formula run",
        "m00.angle.deg_to_rad",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
    assert!(
        !text.contains("invalid_assignment") && !text.contains("usage_error"),
        "negative scalar flag values must not be reinterpreted as flags: {text}"
    );
}

#[test]
fn formula_run_duplicate_flag_input_fails_closed_with_json_error() {
    let output = run(&[
        "formula",
        "run",
        "m00.angle.deg_to_rad",
        "--degrees",
        "180",
        "--degrees",
        "90",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert_error_json_envelope(&text, "formula run", "duplicate_input");
    assert!(text.contains("input `degrees` was provided more than once"));
}

#[test]
fn formula_run_missing_flag_input_fails_closed_with_json_error() {
    let output = run(&["formula", "run", "m00.angle.deg_to_rad", "--json"]);
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert_error_json_envelope(&text, "formula run", "missing_input");
    assert!(text.contains("requires input `degrees`"));
}

#[test]
fn formula_run_unknown_flag_input_fails_closed_with_json_error() {
    let output = run(&[
        "formula",
        "run",
        "m00.angle.deg_to_rad",
        "--radians",
        "3.14159",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert_error_json_envelope(&text, "formula run", "unexpected_input");
    assert!(text.contains("does not accept input `radians`"));
}

#[test]
fn formula_run_invalid_flag_number_fails_closed_with_json_error() {
    let output = run(&[
        "formula",
        "run",
        "m00.angle.deg_to_rad",
        "--degrees",
        "not-a-number",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(2));
    let text = stderr(&output);
    assert_error_json_envelope(&text, "formula run", "invalid_number");
    assert!(text.contains("input `degrees` has invalid f64 value `not-a-number`"));
}

#[test]
fn rr023_m00_angle_run_accepts_preliminary_flag_but_preserves_status_gate() {
    let output = run(&[
        "formula",
        "run",
        "m00.angle.deg_to_rad",
        "--degrees",
        "180",
        "--preliminary",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "formula run",
        "m00.angle.deg_to_rad",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
    assert!(
        !text.contains("\"value\":"),
        "RR-023 dispatch must stay behind the existing RR-025 status gate while M00 angle rows remain research_required: {text}"
    );
}

#[test]
fn rr023_m00_angle_legacy_alias_is_mapped_to_readable_registry_id() {
    let output = run(&[
        "formula",
        "run",
        "formula_vault.m00.angle.rad2deg",
        "--radians",
        "3.141592653589793",
        "--preliminary",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "formula run",
        "m00.angle.rad_to_deg",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
}

#[test]
fn execution_gate_preliminary_flag_does_not_bypass_research_required() {
    let output = run(&[
        "formula",
        "run",
        "m00.canonical.distance_to_canonical",
        "--preliminary",
        "distance=-42",
        "distance_unit=7",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert_execution_gate_blocked_json(
        &text,
        "formula run",
        "m00.canonical.distance_to_canonical",
        "execution_blocked_by_status",
        "research_required",
        "blocked",
    );
}

#[test]
fn unknown_formula_has_distinct_exit_status() {
    let output = run(&["formula", "describe", "no.such.formula", "--json"]);
    assert_eq!(output.status.code(), Some(3));
    let text = stderr(&output);
    assert_error_json_envelope(&text, "formula describe", "formula_not_found");
    assert!(text.contains("\"formula_id\":\"no.such.formula\""));
    assert!(text.contains("\"ok\":false"));
    assert!(text.contains("\"code\":\"formula_not_found\""));
}

#[test]
fn self_check_is_green_and_complete() {
    let output = run(&["self-check", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_success_json_envelope(&text, "self-check");
    assert!(text.contains("\"passed\":14"));
    assert!(text.contains("\"failed\":0"));
    assert!(text.contains("\"semantic_version\":\"0.1.0-alpha.1\""));
    assert!(text.contains("\"release_tier\":\"research_software_alpha\""));
    assert!(text.contains("\"name\":\"overflow_is_rejected\""));
}

#[test]
fn forced_self_check_failure_uses_real_process_contract() {
    let output = run_with_env(
        &["self-check", "--json"],
        "AEROCODEX_TEST_FORCE_SELF_CHECK_FAILURE",
        "1",
    );
    assert_eq!(output.status.code(), Some(5));
    assert!(stderr(&output).is_empty(), "{}", stderr(&output));
    let text = stdout(&output);
    assert_error_json_envelope(&text, "self-check", "self_check_failed");
    assert!(text.contains("\"passed\":13"));
    assert!(text.contains("\"failed\":1"));
    assert!(text.contains("AeroCodex self-check reported 1 failing checks"));
    assert!(!text.contains("Beta 1"));
    assert!(!text.contains("beta1-concept"));
}

#[test]
fn copied_binary_runs_from_external_directory_without_repository_context() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should follow UNIX_EPOCH")
        .as_nanos();
    let directory = env::temp_dir().join(format!(
        "aerocodex_cli_external_test_{}_{}",
        std::process::id(),
        nonce
    ));
    fs::create_dir(&directory).expect("external test directory should be created");
    let source = binary_path();
    let destination = directory.join(source.file_name().expect("binary should have a file name"));
    fs::copy(&source, &destination).expect("CLI binary should copy outside the repository");
    for forbidden in [
        ".git",
        "Cargo.toml",
        "docs",
        "release",
        "release-manifest.toml",
        "v0.1.0-alpha.1.toml",
    ] {
        assert!(!directory.join(forbidden).exists());
    }
    let mut command = Command::new(&destination);
    command
        .current_dir(&directory)
        .args(["version", "--json"]);
    let output = output_with_executable_busy_retry(&mut command)
        .expect("copied CLI should execute from external directory");
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert!(text.contains("\"semantic_version\":\"0.1.0-alpha.1\""));
    assert!(text.contains("\"release_tier\":\"research_software_alpha\""));
    fs::remove_file(&destination).expect("copied CLI should be removable");
    fs::remove_dir(&directory).expect("empty external test directory should be removable");
}
