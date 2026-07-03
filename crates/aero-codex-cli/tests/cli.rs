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

#[test]
fn version_json_exposes_bounded_release_identity() {
    let output = run(&["version", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
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
    assert!(text.contains("aerocodex formula run <formula-id> name=value ... [--json]"));
}

#[test]
fn formula_run_for_non_executable_registry_row_fails_closed() {
    let output = run(&[
        "formula",
        "run",
        "aerodynamics.coefficients.drag_coefficient",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert!(text.contains("\"code\":\"execution_blocked_by_status\""));
    assert!(text.contains("\"execution_policy\":\"blocked\""));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn formula_run_emits_deterministic_machine_readable_result() {
    let output = run(&[
        "formula",
        "run",
        "m00.canonical.distance_to_canonical",
        "distance=-42",
        "distance_unit=7",
        "--json",
    ]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert!(text.contains("\"command\":\"formula run\""));
    assert!(text.contains("\"canonical_formula_id\":\"m00.canonical.distance_to_canonical\""));
    assert!(text
        .contains("\"legacy_formula_id\":\"formula_vault.m00.canonical.distance_to_canonical\""));
    assert!(text.contains("\"output_variable\":\"canonical_distance\""));
    assert!(text.contains("\"value\":-6"));
}

#[test]
fn invalid_scale_has_stable_error_code_and_exit_status() {
    let output = run(&[
        "run",
        "formula_vault.m00.canonical.distance_to_canonical",
        "distance=1",
        "distance_unit=0",
        "--json",
    ]);
    assert_eq!(output.status.code(), Some(4));
    let text = stderr(&output);
    assert!(text.contains("\"code\":\"non_positive_input\""));
    assert!(text.contains("\"validation_status\":\"research_required\""));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn unknown_formula_has_distinct_exit_status() {
    let output = run(&["formula", "describe", "no.such.formula", "--json"]);
    assert_eq!(output.status.code(), Some(3));
    let text = stderr(&output);
    assert!(text.contains("\"ok\":false"));
    assert!(text.contains("\"code\":\"formula_not_found\""));
}

#[test]
fn self_check_is_green_and_complete() {
    let output = run(&["self-check", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert!(text.contains("\"passed\":14"));
    assert!(text.contains("\"failed\":0"));
    assert!(text.contains("\"name\":\"overflow_is_rejected\""));
}
