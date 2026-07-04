#![forbid(unsafe_code)]

use std::{
    env,
    ffi::OsStr,
    path::PathBuf,
    process::{Command, Output},
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

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be valid text")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be valid text")
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
    assert!(text.contains("\"package_version\":\"0.0.1\""));
    assert!(text.contains("\"release_channel\":\"beta1-concept\""));
    assert!(text.contains("\"build_commit\":"));
    assert!(text.contains("\"build_target\":"));
    assert!(text.contains("\"build_profile\":"));
    assert!(text.contains("\"supported_formula_count\":10"));
    assert!(text.contains("\"registry_formula_count\":152"));
    assert!(text.contains("\"validation_status\":\"research_required\""));
    assert!(text.contains("\"safety_notice\":"));
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
    assert!(
        text.contains("aerocodex formula run <formula-id> [--preliminary] name=value ... [--json]")
    );
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
fn execution_gate_blocks_task_card_angle_before_flag_parser() {
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
        "RR-025 gate must fire before unsupported RR-022/RR-023 parser or dispatch paths: {text}"
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
    assert!(text.contains("\"name\":\"overflow_is_rejected\""));
}
