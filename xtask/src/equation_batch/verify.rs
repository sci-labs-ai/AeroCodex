use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::generate::{self, generate_probe_crate, GenerateOptions};

pub const VERIFY_SCHEMA_VERSION: &str = "aerocodex.equation_batch.verify.v1";
pub const MARKER_FILE: &str = generate::MARKER_FILE;
const GENERATED_BY: &str = "xtask equation-batch verify";
const COMMAND_NAME: &str = "equation-batch verify";
const CARGO_STDOUT_FILE: &str = "cargo_stdout.txt";
const CARGO_STDERR_FILE: &str = "cargo_stderr.txt";
const SAFETY_NOTICE: &str = "Equation-batch verify refreshes a temporary generated probe crate and runs cargo test there only; it does not change manifests, validation files, validation status, generated registries, product CLI behavior, runtime formula code, CI wiring, or M07 materials.";
const NON_CLAIMS: &[&str] = &[
    "Verification is compiler-check evidence for manifest test expressions only; it is not formula promotion evidence by itself.",
    "AeroCodex is not certified and not approved for regulated aviation, mission operations, habitat safety, medical/life-support decisions, or regulatory use.",
    "This command does not make formulas executable, implement formula execution, generate registries, run verify-all, or promote validation status.",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyOptions {
    pub manifest: PathBuf,
    pub output_dir: PathBuf,
    pub json: bool,
    pub keep_output: bool,
}

impl VerifyOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        let mut manifest = None;
        let mut output_dir = None;
        let mut json = false;
        let mut keep_output = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--manifest" => {
                    if manifest.is_some() {
                        return Err(
                            "usage error: equation-batch verify requires exactly one --manifest"
                                .to_string(),
                        );
                    }
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        "usage error: --manifest requires a repository-relative TSV path"
                            .to_string()
                    })?;
                    if value.starts_with("--") {
                        return Err(
                            "usage error: --manifest requires a repository-relative TSV path"
                                .to_string(),
                        );
                    }
                    manifest = Some(PathBuf::from(value));
                }
                "--output-dir" => {
                    if output_dir.is_some() {
                        return Err(
                            "usage error: equation-batch verify requires exactly one --output-dir"
                                .to_string(),
                        );
                    }
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        "usage error: --output-dir requires an output directory path".to_string()
                    })?;
                    if value.starts_with("--") {
                        return Err(
                            "usage error: --output-dir requires an output directory path"
                                .to_string(),
                        );
                    }
                    output_dir = Some(PathBuf::from(value));
                }
                "--json" => json = true,
                "--keep-output" => keep_output = true,
                unknown if unknown.starts_with("--") => {
                    return Err(format!(
                        "usage error: unknown equation-batch verify flag `{unknown}`"
                    ));
                }
                unexpected => {
                    return Err(format!(
                        "usage error: unexpected equation-batch verify argument `{unexpected}`"
                    ));
                }
            }
            index += 1;
        }

        let manifest = manifest.ok_or_else(|| {
            "usage error: equation-batch verify requires exactly one --manifest".to_string()
        })?;
        let output_dir = output_dir.ok_or_else(|| {
            "usage error: equation-batch verify requires exactly one --output-dir".to_string()
        })?;

        Ok(Self {
            manifest,
            output_dir,
            json,
            keep_output,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyResult {
    pub ok: bool,
    pub source_manifest: String,
    pub source_manifest_hash: String,
    pub output_dir: PathBuf,
    pub row_count: usize,
    pub passed: usize,
    pub failed: usize,
    pub cargo_status: String,
    pub cargo_exit_code: i32,
    pub stdout_path: PathBuf,
    pub stderr_path: PathBuf,
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CargoTestSummary {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<String>,
}

pub fn run_verify_command(root: &Path, options: &VerifyOptions) -> Result<(), String> {
    let result = verify_equation_batch(root, options)?;
    if options.json {
        print!("{}", render_json(&result));
    } else {
        print!("{}", render_human(&result, options.keep_output));
    }

    if result.ok {
        Ok(())
    } else {
        Err(format!(
            "equation-batch verify cargo test failed with exit code {}",
            result.cargo_exit_code
        ))
    }
}

pub fn verify_equation_batch(root: &Path, options: &VerifyOptions) -> Result<VerifyResult, String> {
    let resolved_output_dir = generate::resolve_output_dir(root, &options.output_dir)?;
    prepare_output_dir_for_verify(&resolved_output_dir)?;

    let generate_options = GenerateOptions {
        manifest: options.manifest.clone(),
        output_dir: options.output_dir.clone(),
        json: false,
    };
    let generated = generate_probe_crate(root, &generate_options)?;

    let cargo_output = Command::new("cargo")
        .arg("test")
        .current_dir(&generated.output_dir)
        .output()
        .map_err(|error| {
            format!(
                "failed to run cargo test in generated probe crate {}: {error}",
                generated.output_dir.display()
            )
        })?;

    let stdout_text = String::from_utf8_lossy(&cargo_output.stdout).into_owned();
    let stderr_text = String::from_utf8_lossy(&cargo_output.stderr).into_owned();
    let stdout_path = generated.output_dir.join(CARGO_STDOUT_FILE);
    let stderr_path = generated.output_dir.join(CARGO_STDERR_FILE);
    fs::write(&stdout_path, stdout_text.as_bytes())
        .map_err(|error| format!("{}: {error}", stdout_path.display()))?;
    fs::write(&stderr_path, stderr_text.as_bytes())
        .map_err(|error| format!("{}: {error}", stderr_path.display()))?;

    let cargo_success = cargo_output.status.success();
    let summary = cargo_test_summary_from_stdout(&stdout_text, generated.row_count, cargo_success);
    let cargo_exit_code = cargo_output.status.code().unwrap_or(-1);
    let ok = cargo_success && summary.failed == 0;
    let cargo_status = if ok { "passed" } else { "failed" }.to_string();

    Ok(VerifyResult {
        ok,
        source_manifest: generated.source_manifest,
        source_manifest_hash: generated.source_manifest_hash,
        output_dir: generated.output_dir,
        row_count: generated.row_count,
        passed: summary.passed,
        failed: summary.failed,
        cargo_status,
        cargo_exit_code,
        stdout_path,
        stderr_path,
        failures: summary.failures,
    })
}

pub(crate) fn prepare_output_dir_for_verify(output_dir: &Path) -> Result<(), String> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .map_err(|error| format!("{}: {error}", output_dir.display()))?;
        return Ok(());
    }
    if !output_dir.is_dir() {
        return Err(format!(
            "usage error: --output-dir exists but is not a directory: {}",
            output_dir.display()
        ));
    }

    let entries = fs::read_dir(output_dir)
        .map_err(|error| format!("{}: {error}", output_dir.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("{}: {error}", output_dir.display()))?;
    if entries.is_empty() {
        return Ok(());
    }

    let marker = output_dir.join(MARKER_FILE);
    if !marker.is_file() {
        return Err(format!(
            "usage error: --output-dir must be empty, absent, or a recognized generated probe directory; refusing non-empty directory: {}",
            output_dir.display()
        ));
    }

    let allowed: BTreeSet<&str> = [
        "Cargo.toml",
        "Cargo.lock",
        "src",
        "tests",
        "manifest_summary.json",
        MARKER_FILE,
        ".aerocodex-repo",
        CARGO_STDOUT_FILE,
        CARGO_STDERR_FILE,
        "target",
    ]
    .into_iter()
    .collect();

    for entry in &entries {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            return Err(format!(
                "usage error: recognized generated probe directory contains a non-UTF-8 entry; refusing refresh: {}",
                output_dir.display()
            ));
        };
        if !allowed.contains(name) {
            return Err(format!(
                "usage error: recognized generated probe directory contains unexpected entry `{name}`; refusing refresh: {}",
                output_dir.display()
            ));
        }
    }

    for name in [
        "target",
        "src",
        "tests",
        ".aerocodex-repo",
        "Cargo.toml",
        "Cargo.lock",
        "manifest_summary.json",
        CARGO_STDOUT_FILE,
        CARGO_STDERR_FILE,
        MARKER_FILE,
    ] {
        let path = output_dir.join(name);
        remove_path_if_exists(&path)?;
    }

    Ok(())
}

fn remove_path_if_exists(path: &Path) -> Result<(), String> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    let file_type = metadata.file_type();
    if file_type.is_symlink() || file_type.is_file() {
        fs::remove_file(path).map_err(|error| format!("{}: {error}", path.display()))
    } else if file_type.is_dir() {
        fs::remove_dir_all(path).map_err(|error| format!("{}: {error}", path.display()))
    } else {
        Err(format!(
            "usage error: generated probe entry is neither file, symlink, nor directory; refusing refresh: {}",
            path.display()
        ))
    }
}

pub(crate) fn cargo_test_summary_from_stdout(
    stdout: &str,
    row_count: usize,
    cargo_success: bool,
) -> CargoTestSummary {
    let mut best_counts = None::<(usize, usize, usize)>;
    for line in stdout.lines() {
        if !line.contains("test result:") {
            continue;
        }
        let Some(passed) = count_before_token(line, " passed") else {
            continue;
        };
        let Some(failed) = count_before_token(line, " failed") else {
            continue;
        };
        let total = passed.saturating_add(failed);
        if best_counts
            .map(|(best_total, _, _)| total > best_total)
            .unwrap_or(true)
        {
            best_counts = Some((total, passed, failed));
        }
    }

    let (mut passed, mut failed) = match best_counts {
        Some((_, passed, failed)) => (passed, failed),
        None if cargo_success => (row_count, 0),
        None => (0, row_count),
    };

    if !cargo_success && failed == 0 {
        failed = row_count.saturating_sub(passed).max(1);
    }
    if cargo_success && passed.saturating_add(failed) == 0 && row_count > 0 {
        passed = row_count;
    }

    let mut failures = failure_names_from_stdout(stdout);
    if !cargo_success && failures.is_empty() {
        failures.push(format!(
            "cargo test exited nonzero; inspect {CARGO_STDOUT_FILE} and {CARGO_STDERR_FILE}"
        ));
    }

    CargoTestSummary {
        passed,
        failed,
        failures,
    }
}

fn count_before_token(line: &str, token: &str) -> Option<usize> {
    let before = line.get(..line.find(token)?)?;
    before.split_whitespace().last()?.parse().ok()
}

fn failure_names_from_stdout(stdout: &str) -> Vec<String> {
    let mut failures = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("---- ")
            .and_then(|value| value.strip_suffix(" stdout ----"))
        {
            failures.push(rest.to_string());
        }
    }
    failures.sort();
    failures.dedup();
    failures
}

fn render_human(result: &VerifyResult, keep_output: bool) -> String {
    let status = if result.ok { "PASS" } else { "FAIL" };
    let failures = if result.failures.is_empty() {
        "none".to_string()
    } else {
        result.failures.join(";")
    };
    let keep_output_text = if keep_output { "true" } else { "true(default)" };
    format!(
        "equation_batch_verify={status}\ncommand={COMMAND_NAME}\nsource_manifest={}\nsource_manifest_hash={}\noutput_dir={}\nrow_count={}\ncargo_status={}\ncargo_exit_code={}\npassed={}\nfailed={}\nstdout_path={}\nstderr_path={}\nfailures={}\nkeep_output={}\nsafety_notice={}\nstatus_promotion=not_performed; verification does not promote formula status\nnon_claims={}\n",
        result.source_manifest,
        result.source_manifest_hash,
        result.output_dir.display(),
        result.row_count,
        result.cargo_status,
        result.cargo_exit_code,
        result.passed,
        result.failed,
        result.stdout_path.display(),
        result.stderr_path.display(),
        failures,
        keep_output_text,
        SAFETY_NOTICE,
        NON_CLAIMS.join(" | ")
    )
}

pub(crate) fn render_json(result: &VerifyResult) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    write_json_bool_field(&mut out, 1, "ok", result.ok, true);
    write_json_field(&mut out, 1, "command", COMMAND_NAME, true);
    write_json_field(&mut out, 1, "schema_version", VERIFY_SCHEMA_VERSION, true);
    write_json_field(&mut out, 1, "generated_by", GENERATED_BY, true);
    write_json_field(
        &mut out,
        1,
        "source_manifest",
        &result.source_manifest,
        true,
    );
    write_json_field(
        &mut out,
        1,
        "source_manifest_hash",
        &result.source_manifest_hash,
        true,
    );
    write_json_field(
        &mut out,
        1,
        "output_dir",
        &result.output_dir.display().to_string(),
        true,
    );
    write_json_usize_field(&mut out, 1, "row_count", result.row_count, true);
    write_json_usize_field(&mut out, 1, "passed", result.passed, true);
    write_json_usize_field(&mut out, 1, "failed", result.failed, true);
    write_json_field(&mut out, 1, "cargo_status", &result.cargo_status, true);
    write_json_i32_field(&mut out, 1, "cargo_exit_code", result.cargo_exit_code, true);
    write_json_field(
        &mut out,
        1,
        "stdout_path",
        &result.stdout_path.display().to_string(),
        true,
    );
    write_json_field(
        &mut out,
        1,
        "stderr_path",
        &result.stderr_path.display().to_string(),
        true,
    );
    write_json_string_array_field(&mut out, 1, "failures", &result.failures, true);
    write_json_field(&mut out, 1, "safety_notice", SAFETY_NOTICE, true);
    write_json_string_array_field(
        &mut out,
        1,
        "non_claims",
        &NON_CLAIMS
            .iter()
            .map(|value| (*value).to_string())
            .collect::<Vec<_>>(),
        false,
    );
    out.push_str("}\n");
    out
}

fn write_json_field(out: &mut String, indent: usize, key: &str, value: &str, trailing: bool) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": ");
    write_json_string(out, value);
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_json_bool_field(out: &mut String, indent: usize, key: &str, value: bool, trailing: bool) {
    write_indent(out, indent);
    write_json_string(out, key);
    write!(out, ": {value}").expect("writing to String cannot fail");
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_json_usize_field(
    out: &mut String,
    indent: usize,
    key: &str,
    value: usize,
    trailing: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    write!(out, ": {value}").expect("writing to String cannot fail");
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_json_i32_field(out: &mut String, indent: usize, key: &str, value: i32, trailing: bool) {
    write_indent(out, indent);
    write_json_string(out, key);
    write!(out, ": {value}").expect("writing to String cannot fail");
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_json_string_array_field(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &[String],
    trailing: bool,
) {
    write_indent(out, indent);
    write_json_string(out, key);
    out.push_str(": [");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        write_json_string(out, value);
    }
    out.push(']');
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}

fn write_json_string(out: &mut String, value: &str) {
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            character if character.is_control() => {
                write!(out, "\\u{:04x}", character as u32).expect("writing to String cannot fail");
            }
            character => out.push(character),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::{
        cargo_test_summary_from_stdout, prepare_output_dir_for_verify, render_json, VerifyOptions,
        VerifyResult, MARKER_FILE, VERIFY_SCHEMA_VERSION,
    };

    #[test]
    fn parse_args_requires_exactly_one_manifest_and_output_dir() {
        let options = VerifyOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--output-dir",
            "/tmp/acx-m00-probe",
            "--json",
            "--keep-output",
        ])
        .expect("valid verify args parse");

        assert_eq!(
            options.manifest.to_string_lossy(),
            "equation-batches/m00-canonical-units.tsv"
        );
        assert_eq!(options.output_dir.to_string_lossy(), "/tmp/acx-m00-probe");
        assert!(options.json);
        assert!(options.keep_output);
    }

    #[test]
    fn parse_args_rejects_missing_duplicate_and_unknown_flags() {
        let missing_manifest = VerifyOptions::parse_args(&["--output-dir", "/tmp/probe"])
            .expect_err("missing manifest must fail");
        assert!(missing_manifest.contains("usage error"));
        assert!(missing_manifest.contains("--manifest"));

        let missing_output =
            VerifyOptions::parse_args(&["--manifest", "equation-batches/m00-canonical-units.tsv"])
                .expect_err("missing output dir must fail");
        assert!(missing_output.contains("usage error"));
        assert!(missing_output.contains("--output-dir"));

        let duplicate_manifest = VerifyOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--output-dir",
            "/tmp/probe",
        ])
        .expect_err("duplicate manifest must fail");
        assert!(duplicate_manifest.contains("exactly one --manifest"));

        let duplicate_output = VerifyOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--output-dir",
            "/tmp/probe-a",
            "--output-dir",
            "/tmp/probe-b",
        ])
        .expect_err("duplicate output dir must fail");
        assert!(duplicate_output.contains("exactly one --output-dir"));

        let unknown = VerifyOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--output-dir",
            "/tmp/probe",
            "--unknown-flag",
        ])
        .expect_err("unknown flags must fail");
        assert!(unknown.contains("usage error"));
        assert!(unknown.contains("unknown"));
    }

    #[test]
    fn cargo_test_summary_parses_final_test_result_line() {
        let stdout = "\nrunning 10 tests\n\ntest result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n\nrunning 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n";
        let summary = cargo_test_summary_from_stdout(stdout, 10, true);
        assert_eq!(summary.passed, 10);
        assert_eq!(summary.failed, 0);
        assert!(summary.failures.is_empty());
    }

    #[test]
    fn prepare_output_dir_refreshes_only_marked_probe_dirs() {
        let dir = unique_temp_path("refresh_marked_probe");
        fs::create_dir_all(dir.join("src")).expect("create src");
        fs::write(dir.join("src/lib.rs"), "").expect("write lib");
        fs::write(dir.join("Cargo.toml"), "").expect("write cargo toml");
        fs::write(dir.join(MARKER_FILE), "generated").expect("write marker");
        fs::write(dir.join("cargo_stdout.txt"), "old").expect("write stdout");

        prepare_output_dir_for_verify(&dir).expect("marked probe dir refreshes");

        assert!(dir.is_dir());
        assert!(fs::read_dir(&dir).expect("read dir").next().is_none());
        remove_dir_if_exists(&dir);
    }

    #[test]
    fn prepare_output_dir_rejects_unmarked_non_empty_dirs() {
        let dir = unique_temp_path("reject_unmarked_probe");
        fs::create_dir_all(&dir).expect("create dir");
        fs::write(dir.join("not-generated.txt"), "operator file").expect("write file");

        let error = prepare_output_dir_for_verify(&dir).expect_err("unmarked non-empty dir fails");
        assert!(error.contains("refusing non-empty directory"));
        remove_dir_if_exists(&dir);
    }

    #[test]
    fn render_json_contains_required_stable_fields() {
        let result = VerifyResult {
            ok: true,
            source_manifest: "equation-batches/m00-canonical-units.tsv".to_string(),
            source_manifest_hash: "abc123".to_string(),
            output_dir: PathBuf::from("/tmp/acx-m00-probe"),
            row_count: 10,
            passed: 10,
            failed: 0,
            cargo_status: "passed".to_string(),
            cargo_exit_code: 0,
            stdout_path: PathBuf::from("/tmp/acx-m00-probe/cargo_stdout.txt"),
            stderr_path: PathBuf::from("/tmp/acx-m00-probe/cargo_stderr.txt"),
            failures: Vec::new(),
        };

        let json = render_json(&result);

        assert!(json.contains(&format!(
            "\"schema_version\": \"{}\"",
            VERIFY_SCHEMA_VERSION
        )));
        assert!(json.contains("\"command\": \"equation-batch verify\""));
        assert!(json.contains("\"source_manifest_hash\": \"abc123\""));
        assert!(json.contains("\"cargo_exit_code\": 0"));
        assert!(json.contains("\"non_claims\""));
        assert!(!json.to_lowercase().contains("flight-ready"));
        assert!(json.to_lowercase().contains("not certified"));
    }

    fn unique_temp_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "aerocodex_verify_test_{}_{}",
            std::process::id(),
            label
        ))
    }

    fn remove_dir_if_exists(path: &std::path::Path) {
        if path.exists() {
            fs::remove_dir_all(path).expect("remove temp dir");
        }
    }
}
