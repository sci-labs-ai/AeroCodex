use std::{
    collections::BTreeSet,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::{
    generate::{self, generate_probe_crate, GenerateOptions},
    manifest::parse_equation_batch_manifest,
};

pub const VERIFY_SCHEMA_VERSION: &str = "aerocodex.equation_batch.verify.v1";
pub const VERIFY_ALL_SCHEMA_VERSION: &str = "aerocodex.equation_batch.verify_all.v1";
pub const MARKER_FILE: &str = generate::MARKER_FILE;
const GENERATED_BY: &str = "xtask equation-batch verify";
const VERIFY_ALL_GENERATED_BY: &str = "xtask equation-batch verify-all";
const COMMAND_NAME: &str = "equation-batch verify";
const VERIFY_ALL_COMMAND_NAME: &str = "equation-batch verify-all";
const CARGO_STDOUT_FILE: &str = "cargo_stdout.txt";
const CARGO_STDERR_FILE: &str = "cargo_stderr.txt";
const SAFETY_NOTICE: &str = "Equation-batch verify refreshes a temporary generated probe crate and runs cargo test there only; it does not change manifests, validation files, validation status, generated registries, product CLI behavior, runtime formula code, CI wiring, or M07 materials.";
const VERIFY_ALL_SAFETY_NOTICE: &str = "Equation-batch verify-all refreshes temporary generated probe crates outside the repository and runs cargo test in each probe crate; it does not change manifests, validation files, validation status, generated registries, product CLI behavior, runtime formula code, GitHub Actions wiring, or M07 materials.";
const NON_CLAIMS: &[&str] = &[
    "Verification is compiler-check evidence for manifest test expressions only; it is not formula promotion evidence by itself.",
    "AeroCodex is not certified and not approved for regulated aviation, mission operations, habitat safety, medical/life-support decisions, or regulatory use.",
    "This command does not make formulas executable, implement formula execution, generate registries, run verify-all, or promote validation status.",
];
const VERIFY_ALL_NON_CLAIMS: &[&str] = &[
    "Verify-all is compiler-check evidence for generated probe crates only; it is not formula promotion evidence by itself.",
    "AeroCodex is not certified or approved for regulated aviation, mission operations, habitat safety, medical or life-support decisions, or regulatory use.",
    "This command does not change manifests, validation cards, M07 materials, generated registries, product CLI behavior, runtime formula code, or formula status.",
];
const FAILURE_CLASSES: &[(&str, &str)] = &[
    (
        "parse_error",
        "manifest parsing failed before probe crate generation",
    ),
    ("generation_error", "probe crate generation failed"),
    ("compile_error", "cargo test reported a Rust compile error"),
    (
        "test_failure",
        "cargo test ran and one or more generated tests failed",
    ),
    (
        "cargo_error",
        "cargo test failed before a precise compile/test class was available",
    ),
    ("json_error", "JSON rendering or parsing failed"),
    ("output_dir_error", "output directory safety checks failed"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyOptions {
    pub manifest: Option<PathBuf>,
    pub all_manifests: bool,
    pub output_dir: PathBuf,
    pub json: bool,
    pub check: bool,
    pub keep_output: bool,
}

impl VerifyOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        let mut manifest = None;
        let mut all_manifests = false;
        let mut output_dir = None;
        let mut json = false;
        let mut check = false;
        let mut keep_output = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--manifest" => {
                    if manifest.is_some() {
                        return Err(
                            "usage error: equation-batch verify requires exactly one --manifest unless --all-manifests is supplied"
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
                "--all-manifests" => {
                    if all_manifests {
                        return Err(
                            "usage error: equation-batch verify accepts --all-manifests at most once"
                                .to_string(),
                        );
                    }
                    all_manifests = true;
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
                "--check" => check = true,
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

        if all_manifests && manifest.is_some() {
            return Err(
                "usage error: --manifest and --all-manifests cannot be supplied together"
                    .to_string(),
            );
        }
        if !all_manifests && manifest.is_none() {
            return Err(
                "usage error: equation-batch verify requires exactly one --manifest or --all-manifests"
                    .to_string(),
            );
        }
        let output_dir = output_dir.ok_or_else(|| {
            "usage error: equation-batch verify requires exactly one --output-dir".to_string()
        })?;

        Ok(Self {
            manifest,
            all_manifests,
            output_dir,
            json,
            check,
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
pub struct ManifestVerifySummary {
    pub manifest: String,
    pub batch_id: String,
    pub row_count: usize,
    pub passed: usize,
    pub failed: usize,
    pub cargo_status: String,
    pub cargo_exit_code: i32,
    pub output_dir: PathBuf,
    pub stdout_path: PathBuf,
    pub stderr_path: PathBuf,
    pub failures: Vec<String>,
    pub failure_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyAllResult {
    pub ok: bool,
    pub manifest_count: usize,
    pub row_count: usize,
    pub passed_manifests: usize,
    pub failed_manifests: usize,
    pub skipped_manifests: usize,
    pub passed: usize,
    pub failed: usize,
    pub cargo_status: String,
    pub cargo_exit_code: i32,
    pub manifests: Vec<ManifestVerifySummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CargoTestSummary {
    pub passed: usize,
    pub failed: usize,
    pub failures: Vec<String>,
}

pub fn run_verify_command(root: &Path, options: &VerifyOptions) -> Result<(), String> {
    if options.all_manifests {
        let result = verify_all_equation_batches(root, options)?;
        if options.json {
            print!("{}", render_verify_all_json(&result));
        } else {
            print!("{}", render_verify_all_human(&result));
        }

        if result.ok {
            Ok(())
        } else {
            Err(format!(
                "equation-batch verify-all failed for {} manifest(s)",
                result.failed_manifests
            ))
        }
    } else {
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
}

pub fn verify_equation_batch(root: &Path, options: &VerifyOptions) -> Result<VerifyResult, String> {
    if options.all_manifests {
        return Err(
            "internal error: verify_equation_batch received all-manifests options".to_string(),
        );
    }
    let manifest = options.manifest.as_ref().ok_or_else(|| {
        "usage error: equation-batch verify requires exactly one --manifest".to_string()
    })?;
    let resolved_output_dir = generate::resolve_output_dir(root, &options.output_dir)?;
    prepare_output_dir_for_verify(&resolved_output_dir)?;
    verify_manifest_probe(root, manifest, &options.output_dir)
}

pub fn verify_all_equation_batches(
    root: &Path,
    options: &VerifyOptions,
) -> Result<VerifyAllResult, String> {
    if !options.all_manifests {
        return Err(
            "internal error: verify_all_equation_batches requires --all-manifests".to_string(),
        );
    }
    if options.manifest.is_some() {
        return Err(
            "usage error: --manifest and --all-manifests cannot be supplied together".to_string(),
        );
    }

    let output_root = generate::resolve_output_dir(root, &options.output_dir)?;
    prepare_output_dir_for_verify_all(&output_root)?;
    fs::write(
        output_root.join(MARKER_FILE),
        render_verify_all_marker().as_bytes(),
    )
    .map_err(|error| format!("{}: {error}", output_root.join(MARKER_FILE).display()))?;

    let manifests = collect_all_manifest_paths(root)?;
    let mut summaries = Vec::with_capacity(manifests.len());
    for manifest in manifests {
        let output_dir = output_root.join(sanitized_manifest_dir_name(&manifest));
        summaries.push(verify_manifest_for_all(root, &manifest, &output_dir));
    }

    Ok(aggregate_verify_all_result(summaries))
}

fn verify_manifest_probe(
    root: &Path,
    manifest: &Path,
    output_dir: &Path,
) -> Result<VerifyResult, String> {
    let generate_options = GenerateOptions {
        manifest: manifest.to_path_buf(),
        output_dir: output_dir.to_path_buf(),
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

fn verify_manifest_for_all(
    root: &Path,
    manifest: &Path,
    output_dir: &Path,
) -> ManifestVerifySummary {
    let manifest_text = match fs::read_to_string(root.join(manifest)) {
        Ok(text) => text,
        Err(error) => {
            let failure = format!("{}: {error}", path_string(manifest));
            let (stdout_path, stderr_path, mut failures) =
                write_failure_artifacts(output_dir, "", &failure);
            failures.push(failure);
            return ManifestVerifySummary {
                manifest: path_string(manifest),
                batch_id: "unknown".to_string(),
                row_count: 0,
                passed: 0,
                failed: 0,
                cargo_status: "failed".to_string(),
                cargo_exit_code: 1,
                output_dir: output_dir.to_path_buf(),
                stdout_path,
                stderr_path,
                failures,
                failure_class: "parse_error".to_string(),
            };
        }
    };
    let parsed = match parse_equation_batch_manifest(manifest, &manifest_text) {
        Ok(parsed) => parsed,
        Err(error) => {
            let (stdout_path, stderr_path, mut failures) =
                write_failure_artifacts(output_dir, "", &error);
            failures.push(error);
            return ManifestVerifySummary {
                manifest: path_string(manifest),
                batch_id: "unknown".to_string(),
                row_count: 0,
                passed: 0,
                failed: 0,
                cargo_status: "failed".to_string(),
                cargo_exit_code: 1,
                output_dir: output_dir.to_path_buf(),
                stdout_path,
                stderr_path,
                failures,
                failure_class: "parse_error".to_string(),
            };
        }
    };

    match verify_manifest_probe(root, manifest, output_dir) {
        Ok(result) => {
            let failure_class = classify_cargo_result(&result);
            ManifestVerifySummary {
                manifest: result.source_manifest,
                batch_id: parsed.batch_id,
                row_count: result.row_count,
                passed: result.passed,
                failed: result.failed,
                cargo_status: result.cargo_status,
                cargo_exit_code: result.cargo_exit_code,
                output_dir: result.output_dir,
                stdout_path: result.stdout_path,
                stderr_path: result.stderr_path,
                failure_class,
                failures: result.failures,
            }
        }
        Err(error) => {
            let failure_class = if error.contains("failed to run cargo test") {
                "cargo_error"
            } else {
                "generation_error"
            };
            let failed = parsed.row_count;
            let (stdout_path, stderr_path, mut failures) =
                write_failure_artifacts(output_dir, "", &error);
            failures.push(error);
            ManifestVerifySummary {
                manifest: path_string(manifest),
                batch_id: parsed.batch_id,
                row_count: parsed.row_count,
                passed: 0,
                failed,
                cargo_status: "failed".to_string(),
                cargo_exit_code: 1,
                output_dir: output_dir.to_path_buf(),
                stdout_path,
                stderr_path,
                failures,
                failure_class: failure_class.to_string(),
            }
        }
    }
}

fn classify_cargo_result(result: &VerifyResult) -> String {
    if result.ok {
        return "none".to_string();
    }
    let stdout = fs::read_to_string(&result.stdout_path).unwrap_or_default();
    let stderr = fs::read_to_string(&result.stderr_path).unwrap_or_default();
    let combined = format!("{stdout}\n{stderr}").to_lowercase();
    if combined.contains("error[e") || combined.contains("could not compile") {
        "compile_error".to_string()
    } else if !result.failures.is_empty() || combined.contains("test result: failed") {
        "test_failure".to_string()
    } else {
        "cargo_error".to_string()
    }
}

fn write_failure_artifacts(
    output_dir: &Path,
    stdout: &str,
    stderr: &str,
) -> (PathBuf, PathBuf, Vec<String>) {
    let mut failures = Vec::new();
    if let Err(error) = fs::create_dir_all(output_dir) {
        failures.push(format!("{}: {error}", output_dir.display()));
    }
    let marker_path = output_dir.join(MARKER_FILE);
    if let Err(error) = fs::write(&marker_path, render_verify_all_marker().as_bytes()) {
        failures.push(format!("{}: {error}", marker_path.display()));
    }
    let stdout_path = output_dir.join(CARGO_STDOUT_FILE);
    if let Err(error) = fs::write(&stdout_path, stdout.as_bytes()) {
        failures.push(format!("{}: {error}", stdout_path.display()));
    }
    let stderr_path = output_dir.join(CARGO_STDERR_FILE);
    if let Err(error) = fs::write(&stderr_path, stderr.as_bytes()) {
        failures.push(format!("{}: {error}", stderr_path.display()));
    }
    (stdout_path, stderr_path, failures)
}

fn aggregate_verify_all_result(manifests: Vec<ManifestVerifySummary>) -> VerifyAllResult {
    let manifest_count = manifests.len();
    let row_count = manifests.iter().map(|manifest| manifest.row_count).sum();
    let passed = manifests.iter().map(|manifest| manifest.passed).sum();
    let failed = manifests.iter().map(|manifest| manifest.failed).sum();
    let passed_manifests = manifests
        .iter()
        .filter(|manifest| manifest.failure_class == "none" && manifest.cargo_exit_code == 0)
        .count();
    let failed_manifests = manifest_count.saturating_sub(passed_manifests);
    let ok = failed_manifests == 0;
    let cargo_exit_code = if ok {
        0
    } else {
        manifests
            .iter()
            .find(|manifest| manifest.cargo_exit_code != 0)
            .map(|manifest| manifest.cargo_exit_code)
            .unwrap_or(1)
    };

    VerifyAllResult {
        ok,
        manifest_count,
        row_count,
        passed_manifests,
        failed_manifests,
        skipped_manifests: 0,
        passed,
        failed,
        cargo_status: if ok { "passed" } else { "failed" }.to_string(),
        cargo_exit_code,
        manifests,
    }
}

fn collect_all_manifest_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let batch_dir = root.join("equation-batches");
    let entries = fs::read_dir(&batch_dir).map_err(|error| format!("equation-batches: {error}"))?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("equation-batches: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) == Some("tsv") {
            paths.push(
                path.strip_prefix(root)
                    .map_err(|error| format!("{}: {error}", path.display()))?
                    .to_path_buf(),
            );
        }
    }
    paths.sort_by_key(|path| path_string(path));
    if paths.is_empty() {
        return Err(
            "equation-batch verify-all found no manifests under equation-batches/".to_string(),
        );
    }
    Ok(paths)
}

fn sanitized_manifest_dir_name(manifest: &Path) -> String {
    let mut out = String::new();
    for character in path_string(manifest).chars() {
        if character.is_ascii_alphanumeric() {
            out.push(character.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    while out.starts_with('_') {
        out.remove(0);
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out.is_empty() {
        "manifest".to_string()
    } else {
        out
    }
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn render_verify_all_marker() -> String {
    format!("schema_version={VERIFY_ALL_SCHEMA_VERSION}\ngenerated_by={VERIFY_ALL_GENERATED_BY}\n")
}

pub(crate) fn prepare_output_dir_for_verify_all(output_dir: &Path) -> Result<(), String> {
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
            "usage error: --output-dir must be empty, absent, or a recognized generated probe collection; refusing non-empty directory: {}",
            output_dir.display()
        ));
    }

    for entry in &entries {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            return Err(format!(
                "usage error: recognized generated probe collection contains a non-UTF-8 entry; refusing refresh: {}",
                output_dir.display()
            ));
        };
        if name == MARKER_FILE {
            continue;
        }
        let path = entry.path();
        if !path.is_dir() || !path.join(MARKER_FILE).is_file() {
            return Err(format!(
                "usage error: recognized generated probe collection contains unmarked entry `{name}`; refusing non-empty directory: {}",
                output_dir.display()
            ));
        }
    }

    for entry in entries {
        let path = entry.path();
        remove_path_if_exists(&path)?;
    }

    Ok(())
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

fn render_verify_all_human(result: &VerifyAllResult) -> String {
    let status = if result.ok { "PASS" } else { "FAIL" };
    let mut out = String::new();
    writeln!(&mut out, "equation_batch_verify_all={status}")
        .expect("writing to String cannot fail");
    writeln!(&mut out, "command={VERIFY_ALL_COMMAND_NAME}").expect("writing to String cannot fail");
    writeln!(&mut out, "manifest_count={}", result.manifest_count)
        .expect("writing to String cannot fail");
    writeln!(&mut out, "row_count={}", result.row_count).expect("writing to String cannot fail");
    for manifest in &result.manifests {
        let manifest_status = if manifest.failure_class == "none" {
            "PASS"
        } else {
            "FAIL"
        };
        let failures = if manifest.failures.is_empty() {
            "none".to_string()
        } else {
            manifest.failures.join(";")
        };
        writeln!(
            &mut out,
            "manifest_result={manifest_status} manifest={} batch_id={} row_count={} passed={} failed={} cargo_status={} cargo_exit_code={} failure_class={} output_dir={} stdout_path={} stderr_path={} failures={}",
            manifest.manifest,
            manifest.batch_id,
            manifest.row_count,
            manifest.passed,
            manifest.failed,
            manifest.cargo_status,
            manifest.cargo_exit_code,
            manifest.failure_class,
            manifest.output_dir.display(),
            manifest.stdout_path.display(),
            manifest.stderr_path.display(),
            failures
        )
        .expect("writing to String cannot fail");
    }
    writeln!(
        &mut out,
        "passed_manifests={} failed_manifests={} skipped_manifests={}",
        result.passed_manifests, result.failed_manifests, result.skipped_manifests
    )
    .expect("writing to String cannot fail");
    writeln!(
        &mut out,
        "passed={} failed={}",
        result.passed, result.failed
    )
    .expect("writing to String cannot fail");
    writeln!(
        &mut out,
        "cargo_status={} cargo_exit_code={}",
        result.cargo_status, result.cargo_exit_code
    )
    .expect("writing to String cannot fail");
    writeln!(&mut out, "check_status={}", result.cargo_status)
        .expect("writing to String cannot fail");
    writeln!(&mut out, "safety_notice={VERIFY_ALL_SAFETY_NOTICE}")
        .expect("writing to String cannot fail");
    writeln!(
        &mut out,
        "status_promotion=not_performed; verify-all does not promote formula status"
    )
    .expect("writing to String cannot fail");
    writeln!(&mut out, "non_claims={}", VERIFY_ALL_NON_CLAIMS.join(" | "))
        .expect("writing to String cannot fail");
    out
}

pub(crate) fn render_verify_all_json(result: &VerifyAllResult) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    write_json_bool_field(&mut out, 1, "ok", result.ok, true);
    write_json_field(&mut out, 1, "command", VERIFY_ALL_COMMAND_NAME, true);
    write_json_field(
        &mut out,
        1,
        "schema_version",
        VERIFY_ALL_SCHEMA_VERSION,
        true,
    );
    write_json_field(&mut out, 1, "generated_by", VERIFY_ALL_GENERATED_BY, true);
    write_json_usize_field(&mut out, 1, "manifest_count", result.manifest_count, true);
    write_json_usize_field(&mut out, 1, "row_count", result.row_count, true);
    write_json_usize_field(
        &mut out,
        1,
        "passed_manifests",
        result.passed_manifests,
        true,
    );
    write_json_usize_field(
        &mut out,
        1,
        "failed_manifests",
        result.failed_manifests,
        true,
    );
    write_json_usize_field(
        &mut out,
        1,
        "skipped_manifests",
        result.skipped_manifests,
        true,
    );
    write_json_usize_field(&mut out, 1, "passed", result.passed, true);
    write_json_usize_field(&mut out, 1, "failed", result.failed, true);
    write_json_field(&mut out, 1, "cargo_status", &result.cargo_status, true);
    write_json_i32_field(&mut out, 1, "cargo_exit_code", result.cargo_exit_code, true);
    write_failure_classes_json_field(&mut out, 1, true);
    write_manifest_summaries_json_field(&mut out, 1, &result.manifests, true);
    write_json_field(&mut out, 1, "safety_notice", VERIFY_ALL_SAFETY_NOTICE, true);
    write_json_string_array_field(
        &mut out,
        1,
        "non_claims",
        &VERIFY_ALL_NON_CLAIMS
            .iter()
            .map(|value| (*value).to_string())
            .collect::<Vec<_>>(),
        false,
    );
    out.push_str("}\n");
    out
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

fn write_failure_classes_json_field(out: &mut String, indent: usize, trailing: bool) {
    write_indent(out, indent);
    write_json_string(out, "failure_classes");
    out.push_str(": {\n");
    for (index, (key, value)) in FAILURE_CLASSES.iter().enumerate() {
        write_json_field(
            out,
            indent + 1,
            key,
            value,
            index + 1 != FAILURE_CLASSES.len(),
        );
    }
    write_indent(out, indent);
    out.push('}');
    if trailing {
        out.push(',');
    }
    out.push('\n');
}

fn write_manifest_summaries_json_field(
    out: &mut String,
    indent: usize,
    manifests: &[ManifestVerifySummary],
    trailing: bool,
) {
    write_indent(out, indent);
    write_json_string(out, "manifests");
    out.push_str(": [\n");
    for (index, manifest) in manifests.iter().enumerate() {
        write_indent(out, indent + 1);
        out.push_str("{\n");
        write_json_field(out, indent + 2, "manifest", &manifest.manifest, true);
        write_json_field(out, indent + 2, "batch_id", &manifest.batch_id, true);
        write_json_usize_field(out, indent + 2, "row_count", manifest.row_count, true);
        write_json_usize_field(out, indent + 2, "passed", manifest.passed, true);
        write_json_usize_field(out, indent + 2, "failed", manifest.failed, true);
        write_json_field(
            out,
            indent + 2,
            "cargo_status",
            &manifest.cargo_status,
            true,
        );
        write_json_i32_field(
            out,
            indent + 2,
            "cargo_exit_code",
            manifest.cargo_exit_code,
            true,
        );
        write_json_field(
            out,
            indent + 2,
            "output_dir",
            &manifest.output_dir.display().to_string(),
            true,
        );
        write_json_field(
            out,
            indent + 2,
            "stdout_path",
            &manifest.stdout_path.display().to_string(),
            true,
        );
        write_json_field(
            out,
            indent + 2,
            "stderr_path",
            &manifest.stderr_path.display().to_string(),
            true,
        );
        write_json_string_array_field(out, indent + 2, "failures", &manifest.failures, true);
        write_json_field(
            out,
            indent + 2,
            "failure_class",
            &manifest.failure_class,
            false,
        );
        write_indent(out, indent + 1);
        out.push('}');
        if index + 1 != manifests.len() {
            out.push(',');
        }
        out.push('\n');
    }
    write_indent(out, indent);
    out.push(']');
    if trailing {
        out.push(',');
    }
    out.push('\n');
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
        cargo_test_summary_from_stdout, prepare_output_dir_for_verify,
        prepare_output_dir_for_verify_all, render_json, render_verify_all_json,
        ManifestVerifySummary, VerifyAllResult, VerifyOptions, VerifyResult, MARKER_FILE,
        VERIFY_ALL_SCHEMA_VERSION, VERIFY_SCHEMA_VERSION,
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
            options
                .manifest
                .as_deref()
                .expect("single manifest present")
                .to_string_lossy(),
            "equation-batches/m00-canonical-units.tsv"
        );
        assert!(!options.all_manifests);
        assert!(!options.check);
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
    fn parse_args_supports_all_manifests_check_mode_and_rejects_manifest_mix() {
        let options = VerifyOptions::parse_args(&[
            "--all-manifests",
            "--output-dir",
            "/tmp/acx-equation-batch-probes",
            "--json",
            "--check",
        ])
        .expect("all-manifests verify args parse");

        assert!(options.all_manifests);
        assert_eq!(options.manifest, None);
        assert_eq!(
            options.output_dir.to_string_lossy(),
            "/tmp/acx-equation-batch-probes"
        );
        assert!(options.json);
        assert!(options.check);

        let mixed = VerifyOptions::parse_args(&[
            "--all-manifests",
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--output-dir",
            "/tmp/acx-probes",
        ])
        .expect_err("manifest plus all-manifests must fail");
        assert!(mixed.contains("--manifest and --all-manifests"));
    }

    #[test]
    fn prepare_verify_all_output_dir_allows_only_marked_probe_collections() {
        let dir = unique_temp_path("refresh_marked_probe_collection");
        let child = dir.join("equation_batches_m00_canonical_units_tsv");
        fs::create_dir_all(&child).expect("create child");
        fs::write(child.join(MARKER_FILE), "generated").expect("write child marker");
        fs::write(dir.join(MARKER_FILE), "generated collection").expect("write root marker");

        prepare_output_dir_for_verify_all(&dir).expect("marked collection accepted");

        fs::write(dir.join("operator-file.txt"), "do not delete").expect("write unsafe file");
        let error =
            prepare_output_dir_for_verify_all(&dir).expect_err("unmarked root entry rejected");
        assert!(error.contains("refusing non-empty directory"));
        remove_dir_if_exists(&dir);
    }

    #[test]
    fn render_verify_all_json_contains_required_aggregate_fields() {
        let result = VerifyAllResult {
            ok: true,
            manifest_count: 1,
            row_count: 10,
            passed_manifests: 1,
            failed_manifests: 0,
            skipped_manifests: 0,
            passed: 10,
            failed: 0,
            cargo_status: "passed".to_string(),
            cargo_exit_code: 0,
            manifests: vec![ManifestVerifySummary {
                manifest: "equation-batches/m00-canonical-units.tsv".to_string(),
                batch_id: "M00-CANONICAL-UNITS".to_string(),
                row_count: 10,
                passed: 10,
                failed: 0,
                cargo_status: "passed".to_string(),
                cargo_exit_code: 0,
                output_dir: PathBuf::from(
                    "/tmp/acx-probes/equation_batches_m00_canonical_units_tsv",
                ),
                stdout_path: PathBuf::from(
                    "/tmp/acx-probes/equation_batches_m00_canonical_units_tsv/cargo_stdout.txt",
                ),
                stderr_path: PathBuf::from(
                    "/tmp/acx-probes/equation_batches_m00_canonical_units_tsv/cargo_stderr.txt",
                ),
                failures: Vec::new(),
                failure_class: "none".to_string(),
            }],
        };

        let json = render_verify_all_json(&result);

        assert!(json.contains(&format!(
            "\"schema_version\": \"{}\"",
            VERIFY_ALL_SCHEMA_VERSION
        )));
        assert!(json.contains("\"command\": \"equation-batch verify-all\""));
        assert!(json.contains("\"manifest_count\": 1"));
        assert!(json.contains("\"failure_classes\""));
        assert!(json.contains("\"parse_error\""));
        assert!(json.contains("\"manifests\""));
        assert!(!json.to_lowercase().contains("flight-ready"));
        assert!(json.to_lowercase().contains("not certified"));
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
