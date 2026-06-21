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
    assert!(text.contains("\"validation_status\":\"research_required\""));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn formula_catalog_is_bounded_and_labeled() {
    let output = run(&["formulas", "--json"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert!(text.contains("\"count\":10"));
    assert!(text.contains("\"validation_status\":\"research_required\""));
    assert!(text.contains("\"safety_notice\":"));
}

#[test]
fn formula_run_emits_deterministic_machine_readable_result() {
    let output = run(&[
        "run",
        "formula_vault.m00.canonical.distance_to_canonical",
        "distance=-42",
        "distance_unit=7",
        "--json",
    ]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
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
    let output = run(&["describe", "formula_vault.m00.canonical.unknown", "--json"]);
    assert_eq!(output.status.code(), Some(3));
    let text = stderr(&output);
    assert!(text.contains("\"code\":\"unknown_formula\""));
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
