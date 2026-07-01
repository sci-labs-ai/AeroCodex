use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Component, Path, PathBuf},
};

use super::manifest::{parse_equation_batch_manifest, EquationBatchRow};

pub const PLAN_COMMAND: &str = "equation-batch plan";
pub const PLAN_SCHEMA_VERSION: &str = "aerocodex.equation_batch.plan.v1";
pub const SAFETY_NOTICE: &str = "Dry-run planning only: this command does not compile probes, evaluate formula expressions, generate registries, execute formulas, change validation status, or make blocked formulas executable. Normal execution remains blocked unless validation_status is implementation_verified and required metadata paths are present.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanOptions {
    pub manifests: Vec<PathBuf>,
    pub all_manifests: bool,
    pub json: bool,
}

impl PlanOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        let mut manifests = Vec::new();
        let mut all_manifests = false;
        let mut json = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--manifest" => {
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
                    manifests.push(PathBuf::from(value));
                }
                "--all-manifests" => all_manifests = true,
                "--json" => json = true,
                unknown if unknown.starts_with("--") => {
                    return Err(format!(
                        "usage error: unknown equation-batch plan flag `{unknown}`"
                    ));
                }
                unexpected => {
                    return Err(format!(
                        "usage error: unexpected equation-batch plan argument `{unexpected}`"
                    ));
                }
            }
            index += 1;
        }

        if all_manifests && !manifests.is_empty() {
            return Err(
                "usage error: --manifest and --all-manifests cannot be supplied together"
                    .to_string(),
            );
        }
        if !all_manifests && manifests.is_empty() {
            return Err(
                "usage error: supply at least one --manifest PATH or use --all-manifests"
                    .to_string(),
            );
        }

        Ok(Self {
            manifests,
            all_manifests,
            json,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquationBatchPlanReport {
    pub ok: bool,
    pub command: String,
    pub schema_version: String,
    pub manifests: Vec<ManifestPlanReport>,
    pub manifest_count: usize,
    pub row_count: usize,
    pub counts_by_status: BTreeMap<String, usize>,
    pub counts_by_batch: BTreeMap<String, usize>,
    pub counts_by_test_strategy: BTreeMap<String, usize>,
    pub blocked_execution_count: usize,
    pub missing_paths: Vec<MissingMetadataPath>,
    pub safety_notice: String,
}

impl EquationBatchPlanReport {
    pub fn missing_path_count(&self) -> usize {
        self.missing_paths.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestPlanReport {
    pub path: String,
    pub batch_id: String,
    pub row_count: usize,
    pub counts_by_status: BTreeMap<String, usize>,
    pub counts_by_test_strategy: BTreeMap<String, usize>,
    pub blocked_execution_count: usize,
    pub missing_path_count: usize,
    pub rows: Vec<RowPlanReport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowPlanReport {
    pub formula_id: String,
    pub batch_id: String,
    pub validation_status: String,
    pub test_strategy: String,
    pub execution_eligible: bool,
    pub static_checks: RowStaticChecks,
    pub missing_paths: Vec<MissingRowPath>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowStaticChecks {
    pub path_status: String,
    pub contract_path: String,
    pub validation_card_path: String,
    pub source_seed_path: String,
    pub runtime_source_root: String,
    pub package_status: String,
    pub crate_status: String,
    pub symbol_status: String,
    pub static_warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkspaceIndex {
    packages: BTreeMap<String, WorkspacePackage>,
    warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkspacePackage {
    package_name: String,
    crate_name: String,
    source_dir: PathBuf,
    source_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingRowPath {
    pub field_name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingMetadataPath {
    pub manifest_path: String,
    pub formula_id: String,
    pub field_name: String,
    pub path: String,
}

pub fn plan_equation_batches(
    root: &Path,
    options: &PlanOptions,
) -> Result<EquationBatchPlanReport, String> {
    let manifest_paths = selected_manifest_paths(root, options)?;
    let workspace = load_workspace_index(root);
    let mut manifest_reports = Vec::new();
    let mut counts_by_status = BTreeMap::new();
    let mut counts_by_batch = BTreeMap::new();
    let mut counts_by_test_strategy = BTreeMap::new();
    let mut missing_paths = Vec::new();
    let mut total_row_count = 0usize;
    let mut total_blocked_execution_count = 0usize;

    for relative_path in manifest_paths {
        let absolute_path = root.join(&relative_path);
        let manifest_path = path_string(&relative_path);
        let text = fs::read_to_string(&absolute_path)
            .map_err(|error| format!("{}: {error}", manifest_path))?;
        let manifest = parse_equation_batch_manifest(&relative_path, &text)?;
        let mut rows: Vec<&EquationBatchRow> = manifest.rows.iter().collect();
        rows.sort_by(|left, right| {
            left.formula_id
                .cmp(&right.formula_id)
                .then(left.line_number.cmp(&right.line_number))
        });

        let mut manifest_rows = Vec::new();
        let mut manifest_missing_path_count = 0usize;
        let mut manifest_blocked_execution_count = 0usize;

        for row in rows {
            let mut row_missing_paths = missing_paths_for_row(root, row);
            row_missing_paths.sort_by(|left, right| {
                left.path
                    .cmp(&right.path)
                    .then(left.field_name.cmp(&right.field_name))
            });

            for missing in &row_missing_paths {
                missing_paths.push(MissingMetadataPath {
                    manifest_path: manifest_path.clone(),
                    formula_id: row.formula_id.clone(),
                    field_name: missing.field_name.clone(),
                    path: missing.path.clone(),
                });
            }

            let static_checks = static_checks_for_row(root, row, &row_missing_paths, &workspace);
            let execution_eligible =
                row.validation_status == "implementation_verified" && row_missing_paths.is_empty();
            if !execution_eligible {
                manifest_blocked_execution_count += 1;
                total_blocked_execution_count += 1;
            }
            manifest_missing_path_count += row_missing_paths.len();

            manifest_rows.push(RowPlanReport {
                formula_id: row.formula_id.clone(),
                batch_id: row.batch_id.clone(),
                validation_status: row.validation_status.clone(),
                test_strategy: row.test_strategy.clone(),
                execution_eligible,
                static_checks,
                missing_paths: row_missing_paths,
            });
        }

        for (status, count) in &manifest.validation_status_counts {
            *counts_by_status.entry(status.clone()).or_insert(0) += count;
        }
        for (strategy, count) in &manifest.test_strategy_counts {
            *counts_by_test_strategy.entry(strategy.clone()).or_insert(0) += count;
        }
        *counts_by_batch
            .entry(manifest.batch_id.clone())
            .or_insert(0) += manifest.row_count;
        total_row_count = total_row_count
            .checked_add(manifest.row_count)
            .ok_or_else(|| "equation-batch plan row count overflow".to_string())?;

        manifest_reports.push(ManifestPlanReport {
            path: manifest_path,
            batch_id: manifest.batch_id,
            row_count: manifest.row_count,
            counts_by_status: manifest.validation_status_counts,
            counts_by_test_strategy: manifest.test_strategy_counts,
            blocked_execution_count: manifest_blocked_execution_count,
            missing_path_count: manifest_missing_path_count,
            rows: manifest_rows,
        });
    }

    missing_paths.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.formula_id.cmp(&right.formula_id))
            .then(left.manifest_path.cmp(&right.manifest_path))
            .then(left.field_name.cmp(&right.field_name))
    });

    Ok(EquationBatchPlanReport {
        ok: true,
        command: PLAN_COMMAND.to_string(),
        schema_version: PLAN_SCHEMA_VERSION.to_string(),
        manifest_count: manifest_reports.len(),
        manifests: manifest_reports,
        row_count: total_row_count,
        counts_by_status,
        counts_by_batch,
        counts_by_test_strategy,
        blocked_execution_count: total_blocked_execution_count,
        missing_paths,
        safety_notice: SAFETY_NOTICE.to_string(),
    })
}

pub fn render_human(report: &EquationBatchPlanReport) -> String {
    let mut out = String::new();
    writeln!(out, "command: {}", report.command).expect("write to string");
    writeln!(out, "schema_version: {}", report.schema_version).expect("write to string");
    writeln!(out, "manifest_count: {}", report.manifest_count).expect("write to string");
    writeln!(out, "row_count: {}", report.row_count).expect("write to string");
    writeln!(out, "manifests:").expect("write to string");
    for manifest in &report.manifests {
        writeln!(out, "  - manifest_path: {}", manifest.path).expect("write to string");
        writeln!(out, "    batch_id: {}", manifest.batch_id).expect("write to string");
        writeln!(out, "    row_count: {}", manifest.row_count).expect("write to string");
        writeln!(
            out,
            "    blocked_execution_count: {}",
            manifest.blocked_execution_count
        )
        .expect("write to string");
        writeln!(
            out,
            "    missing_metadata_path_count: {}",
            manifest.missing_path_count
        )
        .expect("write to string");
    }
    writeln!(out, "counts_by_validation_status:").expect("write to string");
    write_count_lines(&mut out, &report.counts_by_status);
    writeln!(out, "counts_by_test_strategy:").expect("write to string");
    write_count_lines(&mut out, &report.counts_by_test_strategy);
    writeln!(
        out,
        "blocked_execution_count: {}",
        report.blocked_execution_count
    )
    .expect("write to string");
    writeln!(
        out,
        "missing_metadata_path_count: {}",
        report.missing_path_count()
    )
    .expect("write to string");
    if !report.missing_paths.is_empty() {
        writeln!(out, "missing_paths:").expect("write to string");
        for missing in &report.missing_paths {
            writeln!(
                out,
                "  - path: {}; formula_id: {}; field: {}; manifest: {}",
                missing.path, missing.formula_id, missing.field_name, missing.manifest_path
            )
            .expect("write to string");
        }
    }
    writeln!(out, "safety_notice: {}", report.safety_notice).expect("write to string");
    out
}

pub fn render_json(report: &EquationBatchPlanReport) -> String {
    let mut out = String::new();
    out.push('{');
    push_json_bool_field(&mut out, "ok", report.ok, true);
    push_json_string_field(&mut out, "command", &report.command, true);
    push_json_string_field(&mut out, "schema_version", &report.schema_version, true);

    out.push_str("\n  \"manifests\":[");
    for (index, manifest) in report.manifests.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        push_manifest_json(&mut out, manifest, 4);
    }
    out.push_str("\n  ],");

    push_json_usize_field(&mut out, "manifest_count", report.manifest_count, true);
    push_json_usize_field(&mut out, "row_count", report.row_count, true);
    push_json_count_map_field(&mut out, "counts_by_status", &report.counts_by_status, true);
    push_json_count_map_field(&mut out, "counts_by_batch", &report.counts_by_batch, true);
    push_json_count_map_field(
        &mut out,
        "counts_by_test_strategy",
        &report.counts_by_test_strategy,
        true,
    );
    push_json_usize_field(
        &mut out,
        "blocked_execution_count",
        report.blocked_execution_count,
        true,
    );
    push_json_usize_field(
        &mut out,
        "missing_path_count",
        report.missing_path_count(),
        true,
    );

    if report.missing_paths.is_empty() {
        out.push_str("\n  \"missing_paths\":[],");
    } else {
        out.push_str("\n  \"missing_paths\":[");
        for (index, missing) in report.missing_paths.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            push_missing_path_json(&mut out, missing, 4);
        }
        out.push_str("\n  ],");
    }

    out.push_str("\n  \"rows\":[");
    let mut first_row = true;
    for manifest in &report.manifests {
        for row in &manifest.rows {
            if !first_row {
                out.push(',');
            }
            first_row = false;
            push_row_json(&mut out, Some(&manifest.path), row, 4);
        }
    }
    out.push_str("\n  ],");

    push_json_string_field(&mut out, "safety_notice", &report.safety_notice, false);
    out.push_str("\n}\n");
    out
}

fn selected_manifest_paths(root: &Path, options: &PlanOptions) -> Result<Vec<PathBuf>, String> {
    let mut paths = if options.all_manifests {
        collect_all_manifest_paths(root)?
    } else {
        options
            .manifests
            .iter()
            .map(|path| normalize_manifest_argument(root, path))
            .collect::<Result<Vec<_>, _>>()?
    };
    paths.sort_by_key(|path| path_string(path));
    Ok(paths)
}

fn collect_all_manifest_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    let batch_dir = root.join("equation-batches");
    let entries = fs::read_dir(&batch_dir).map_err(|error| format!("equation-batches: {error}"))?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("equation-batches: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) == Some("tsv") {
            let relative = path
                .strip_prefix(root)
                .map_err(|error| format!("{}: {error}", path.display()))?
                .to_path_buf();
            paths.push(relative);
        }
    }
    if paths.is_empty() {
        return Err("equation-batch plan found no manifests under equation-batches/".to_string());
    }
    Ok(paths)
}

fn normalize_manifest_argument(root: &Path, path: &Path) -> Result<PathBuf, String> {
    let relative = if path.is_absolute() {
        path.strip_prefix(root)
            .map_err(|_| {
                format!(
                    "usage error: manifest path must be inside the repository root: {}",
                    path.display()
                )
            })?
            .to_path_buf()
    } else {
        path.to_path_buf()
    };

    if relative
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(format!(
            "usage error: manifest path must not contain parent traversal: {}",
            path.display()
        ));
    }
    if path_string(&relative).starts_with('\\')
        || has_windows_absolute_prefix(&path_string(&relative))
    {
        return Err(format!(
            "usage error: manifest path must be repository-relative: {}",
            path.display()
        ));
    }

    Ok(relative)
}

fn missing_paths_for_row(root: &Path, row: &EquationBatchRow) -> Vec<MissingRowPath> {
    let mut missing = Vec::new();
    for (field_name, value) in [
        ("contract_path", row.contract_path.as_str()),
        ("validation_card_path", row.validation_card_path.as_str()),
        ("source_seed_path", row.source_seed_path.as_str()),
    ] {
        if !root.join(value).is_file() {
            missing.push(MissingRowPath {
                field_name: field_name.to_string(),
                path: value.to_string(),
            });
        }
    }
    missing
}

fn static_checks_for_row(
    root: &Path,
    row: &EquationBatchRow,
    missing_paths: &[MissingRowPath],
    workspace: &WorkspaceIndex,
) -> RowStaticChecks {
    let contract_path = path_field_status(root, &row.contract_path);
    let validation_card_path = path_field_status(root, &row.validation_card_path);
    let source_seed_path = path_field_status(root, &row.source_seed_path);
    let mut static_warnings = workspace.warnings.clone();
    for missing in missing_paths {
        static_warnings.push(format!("missing {}: {}", missing.field_name, missing.path));
    }

    let Some(package) = workspace.packages.get(&row.package) else {
        static_warnings.push(format!("package not found in workspace: {}", row.package));
        let path_status = if missing_paths.is_empty() {
            "unknown"
        } else {
            "missing"
        }
        .to_string();
        return RowStaticChecks {
            path_status,
            contract_path,
            validation_card_path,
            source_seed_path,
            runtime_source_root: "unknown".to_string(),
            package_status: "missing".to_string(),
            crate_status: "unknown".to_string(),
            symbol_status: "unknown".to_string(),
            static_warnings,
        };
    };

    let package_status = "ok".to_string();
    let runtime_source_root = if package.source_dir.is_dir() {
        "ok".to_string()
    } else {
        static_warnings.push(format!(
            "runtime source root missing for package {}: {}",
            package.package_name,
            relative_path_string(root, &package.source_dir)
        ));
        "missing".to_string()
    };
    let path_status = if missing_paths.is_empty() && runtime_source_root == "ok" {
        "ok".to_string()
    } else {
        "missing".to_string()
    };

    let crate_status = if row.crate_name == package.crate_name {
        "ok".to_string()
    } else {
        static_warnings.push(format!(
            "crate_name mismatch for package {}: expected {}, found {}",
            package.package_name, package.crate_name, row.crate_name
        ));
        "mismatch".to_string()
    };

    let symbol_status = if crate_status != "ok" || runtime_source_root != "ok" {
        static_warnings.push(format!(
            "runtime_symbol not checked because crate_status={} and runtime_source_root={}",
            crate_status, runtime_source_root
        ));
        "unknown".to_string()
    } else {
        runtime_symbol_status(package, &row.runtime_symbol, &mut static_warnings)
    };

    RowStaticChecks {
        path_status,
        contract_path,
        validation_card_path,
        source_seed_path,
        runtime_source_root,
        package_status,
        crate_status,
        symbol_status,
        static_warnings,
    }
}

fn path_field_status(root: &Path, relative: &str) -> String {
    if root.join(relative).is_file() {
        "ok".to_string()
    } else {
        "missing".to_string()
    }
}

fn load_workspace_index(root: &Path) -> WorkspaceIndex {
    let mut warnings = Vec::new();
    let mut packages = BTreeMap::new();
    let root_cargo = root.join("Cargo.toml");
    let workspace_text = match fs::read_to_string(&root_cargo) {
        Ok(text) => text,
        Err(error) => {
            warnings.push(format!("workspace Cargo.toml could not be read: {error}"));
            return WorkspaceIndex { packages, warnings };
        }
    };

    let members = match parse_workspace_members(&workspace_text) {
        Ok(members) => members,
        Err(error) => {
            warnings.push(format!("workspace members could not be parsed: {error}"));
            Vec::new()
        }
    };

    for member in members {
        let member_path = PathBuf::from(&member);
        if member_path.is_absolute()
            || member_path
                .components()
                .any(|component| matches!(component, Component::ParentDir))
        {
            warnings.push(format!(
                "workspace member path is not repository-relative: {member}"
            ));
            continue;
        }
        let manifest_path = root.join(&member_path).join("Cargo.toml");
        let manifest_text = match fs::read_to_string(&manifest_path) {
            Ok(text) => text,
            Err(error) => {
                warnings.push(format!(
                    "workspace member Cargo.toml could not be read for {member}: {error}"
                ));
                continue;
            }
        };
        let Some(package_name) =
            parse_toml_string_assignment_in_section(&manifest_text, "package", "name")
        else {
            warnings.push(format!("workspace member has no [package] name: {member}"));
            continue;
        };
        let crate_name = parse_toml_string_assignment_in_section(&manifest_text, "lib", "name")
            .unwrap_or_else(|| package_name.replace('-', "_"));
        let lib_path = parse_toml_string_assignment_in_section(&manifest_text, "lib", "path")
            .unwrap_or_else(|| "src/lib.rs".to_string());
        let source_dir = Path::new(&lib_path)
            .parent()
            .map(|parent| root.join(&member_path).join(parent))
            .unwrap_or_else(|| root.join(&member_path));
        let mut source_files = Vec::new();
        collect_rust_source_files(&source_dir, &mut source_files);
        source_files.sort_by_key(|path| relative_path_string(root, path));
        packages.insert(
            package_name.clone(),
            WorkspacePackage {
                package_name,
                crate_name,
                source_dir,
                source_files,
            },
        );
    }

    WorkspaceIndex { packages, warnings }
}

fn parse_workspace_members(text: &str) -> Result<Vec<String>, String> {
    let mut in_workspace = false;
    let mut collecting_members = false;
    let mut members = Vec::new();

    for line in text.lines() {
        let trimmed = strip_toml_comment(line).trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_workspace = trimmed == "[workspace]";
            collecting_members = false;
            continue;
        }
        if !in_workspace {
            continue;
        }
        if collecting_members {
            members.extend(extract_quoted_values(&trimmed));
            if trimmed.contains(']') {
                collecting_members = false;
            }
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        if key.trim() != "members" {
            continue;
        }
        members.extend(extract_quoted_values(value));
        if value.contains('[') && !value.contains(']') {
            collecting_members = true;
        }
    }

    if members.is_empty() {
        Err("no [workspace] members entries found".to_string())
    } else {
        Ok(members)
    }
}

fn parse_toml_string_assignment_in_section(
    text: &str,
    target_section: &str,
    key: &str,
) -> Option<String> {
    let mut section = String::new();
    for line in text.lines() {
        let trimmed = strip_toml_comment(line).trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            section = trimmed.trim_matches(['[', ']']).to_string();
            continue;
        }
        if section != target_section {
            continue;
        }
        let Some((left, right)) = trimmed.split_once('=') else {
            continue;
        };
        if left.trim() == key {
            return extract_quoted_values(right).into_iter().next();
        }
    }
    None
}

fn strip_toml_comment(line: &str) -> String {
    let mut out = String::new();
    let mut in_quote = false;
    let mut escaped = false;
    for character in line.chars() {
        if escaped {
            out.push(character);
            escaped = false;
            continue;
        }
        match character {
            '\\' if in_quote => {
                out.push(character);
                escaped = true;
            }
            '"' => {
                in_quote = !in_quote;
                out.push(character);
            }
            '#' if !in_quote => break,
            _ => out.push(character),
        }
    }
    out
}

fn extract_quoted_values(text: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escaped = false;
    for character in text.chars() {
        if escaped {
            if in_quote {
                current.push(character);
            }
            escaped = false;
            continue;
        }
        match character {
            '\\' if in_quote => escaped = true,
            '"' if in_quote => {
                values.push(current.clone());
                current.clear();
                in_quote = false;
            }
            '"' => in_quote = true,
            _ if in_quote => current.push(character),
            _ => {}
        }
    }
    values
}

fn collect_rust_source_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_source_files(&path, out);
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn runtime_symbol_status(
    package: &WorkspacePackage,
    runtime_symbol: &str,
    static_warnings: &mut Vec<String>,
) -> String {
    let segments = runtime_symbol.split("::").collect::<Vec<_>>();
    if segments.is_empty() || segments.iter().any(|segment| !is_rust_identifier(segment)) {
        static_warnings.push(format!(
            "runtime_symbol has unsupported textual shape: {runtime_symbol}"
        ));
        return "unknown".to_string();
    }

    for module in &segments[..segments.len().saturating_sub(1)] {
        if !source_files_declare_public_item(&package.source_files, "mod", module) {
            static_warnings.push(format!(
                "runtime_symbol module path not found textually: {runtime_symbol}"
            ));
            return "missing".to_string();
        }
    }

    let symbol = segments
        .last()
        .expect("runtime symbol has at least one segment");
    if source_files_declare_public_item(&package.source_files, "fn", symbol)
        || source_files_declare_public_item(&package.source_files, "mod", symbol)
    {
        return "ok".to_string();
    }
    if source_files_contain_text(&package.source_files, symbol) {
        static_warnings.push(format!(
            "runtime_symbol appears in source but not as a conservative public function/module declaration: {runtime_symbol}"
        ));
        return "unknown".to_string();
    }

    static_warnings.push(format!(
        "runtime_symbol public function/module not found textually: {runtime_symbol}"
    ));
    "missing".to_string()
}

fn source_files_declare_public_item(files: &[PathBuf], item_kind: &str, name: &str) -> bool {
    files.iter().any(|path| {
        let Ok(text) = fs::read_to_string(path) else {
            return false;
        };
        text.lines()
            .any(|line| line_declares_public_item(line, item_kind, name))
    })
}

fn source_files_contain_text(files: &[PathBuf], needle: &str) -> bool {
    files.iter().any(|path| {
        fs::read_to_string(path)
            .map(|text| text.contains(needle))
            .unwrap_or(false)
    })
}

fn line_declares_public_item(line: &str, item_kind: &str, name: &str) -> bool {
    let trimmed = line.trim_start();
    let prefixes: &[&str] = match item_kind {
        "fn" => &[
            "pub fn ",
            "pub async fn ",
            "pub const fn ",
            "pub unsafe fn ",
        ],
        "mod" => &["pub mod ", "pub(crate) mod ", "pub(super) mod ", "pub(in "],
        _ => &[],
    };
    prefixes.iter().any(|prefix| {
        if *prefix == "pub(in " {
            if !trimmed.starts_with(prefix) {
                return false;
            }
            let Some(after_visibility) = trimmed.split_once(')') else {
                return false;
            };
            let rest = after_visibility.1.trim_start();
            return rest
                .strip_prefix("mod ")
                .map(|candidate| starts_with_identifier(candidate, name))
                .unwrap_or(false);
        }
        trimmed
            .strip_prefix(prefix)
            .map(|candidate| starts_with_identifier(candidate, name))
            .unwrap_or(false)
    })
}

fn starts_with_identifier(candidate: &str, expected: &str) -> bool {
    let mut chars = candidate.chars();
    for expected_char in expected.chars() {
        if chars.next() != Some(expected_char) {
            return false;
        }
    }
    match chars.next() {
        Some(next) => !(next == '_' || next.is_ascii_alphanumeric()),
        None => true,
    }
}

fn is_rust_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first == '_' || first.is_ascii_alphabetic()) {
        return false;
    }
    chars.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn relative_path_string(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(path_string)
        .unwrap_or_else(|_| path.display().to_string())
}

fn write_count_lines(out: &mut String, counts: &BTreeMap<String, usize>) {
    if counts.is_empty() {
        writeln!(out, "  (none)").expect("write to string");
        return;
    }
    for (key, count) in counts {
        writeln!(out, "  {key}: {count}").expect("write to string");
    }
}

fn push_manifest_json(out: &mut String, manifest: &ManifestPlanReport, indent: usize) {
    let prefix = " ".repeat(indent);
    out.push('\n');
    out.push_str(&prefix);
    out.push('{');
    push_json_string_field_at(out, indent + 2, "path", &manifest.path, true);
    push_json_string_field_at(out, indent + 2, "batch_id", &manifest.batch_id, true);
    push_json_usize_field_at(out, indent + 2, "row_count", manifest.row_count, true);
    push_json_count_map_field_at(
        out,
        indent + 2,
        "counts_by_status",
        &manifest.counts_by_status,
        true,
    );
    push_json_count_map_field_at(
        out,
        indent + 2,
        "counts_by_test_strategy",
        &manifest.counts_by_test_strategy,
        true,
    );
    push_json_usize_field_at(
        out,
        indent + 2,
        "blocked_execution_count",
        manifest.blocked_execution_count,
        true,
    );
    push_json_usize_field_at(
        out,
        indent + 2,
        "missing_path_count",
        manifest.missing_path_count,
        true,
    );
    out.push_str(&format!("\n{}  \"rows\":[", prefix));
    for (index, row) in manifest.rows.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        push_row_json(out, None, row, indent + 4);
    }
    out.push_str(&format!("\n{}  ]", prefix));
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
}

fn push_row_json(
    out: &mut String,
    manifest_path: Option<&str>,
    row: &RowPlanReport,
    indent: usize,
) {
    let prefix = " ".repeat(indent);
    out.push('\n');
    out.push_str(&prefix);
    out.push('{');
    if let Some(manifest_path) = manifest_path {
        push_json_string_field_at(out, indent + 2, "manifest_path", manifest_path, true);
    }
    push_json_string_field_at(out, indent + 2, "formula_id", &row.formula_id, true);
    push_json_string_field_at(out, indent + 2, "batch_id", &row.batch_id, true);
    push_json_string_field_at(
        out,
        indent + 2,
        "validation_status",
        &row.validation_status,
        true,
    );
    push_json_string_field_at(out, indent + 2, "test_strategy", &row.test_strategy, true);
    push_json_bool_field_at(
        out,
        indent + 2,
        "execution_eligible",
        row.execution_eligible,
        true,
    );
    push_static_checks_json(out, &row.static_checks, indent + 2, true);
    out.push_str(&format!("\n{}  \"missing_paths\":[", prefix));
    for (index, missing) in row.missing_paths.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        push_missing_row_path_json(out, missing, indent + 4);
    }
    out.push_str(&format!("\n{}  ]", prefix));
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
}

fn push_static_checks_json(out: &mut String, checks: &RowStaticChecks, indent: usize, comma: bool) {
    let prefix = " ".repeat(indent);
    let suffix = if comma { "," } else { "" };
    out.push('\n');
    out.push_str(&prefix);
    out.push_str("\"static_checks\":{");
    push_json_string_field_at(out, indent + 2, "path_status", &checks.path_status, true);
    push_json_string_field_at(
        out,
        indent + 2,
        "contract_path",
        &checks.contract_path,
        true,
    );
    push_json_string_field_at(
        out,
        indent + 2,
        "validation_card_path",
        &checks.validation_card_path,
        true,
    );
    push_json_string_field_at(
        out,
        indent + 2,
        "source_seed_path",
        &checks.source_seed_path,
        true,
    );
    push_json_string_field_at(
        out,
        indent + 2,
        "runtime_source_root",
        &checks.runtime_source_root,
        true,
    );
    push_json_string_field_at(
        out,
        indent + 2,
        "package_status",
        &checks.package_status,
        true,
    );
    push_json_string_field_at(out, indent + 2, "crate_status", &checks.crate_status, true);
    push_json_string_field_at(
        out,
        indent + 2,
        "symbol_status",
        &checks.symbol_status,
        true,
    );
    write!(
        out,
        "\n{}\"static_warnings\":{}",
        " ".repeat(indent + 2),
        json_string_array(&checks.static_warnings)
    )
    .expect("write to string");
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
    out.push_str(suffix);
}

fn push_missing_path_json(out: &mut String, missing: &MissingMetadataPath, indent: usize) {
    let prefix = " ".repeat(indent);
    out.push('\n');
    out.push_str(&prefix);
    out.push('{');
    push_json_string_field_at(
        out,
        indent + 2,
        "manifest_path",
        &missing.manifest_path,
        true,
    );
    push_json_string_field_at(out, indent + 2, "formula_id", &missing.formula_id, true);
    push_json_string_field_at(out, indent + 2, "field_name", &missing.field_name, true);
    push_json_string_field_at(out, indent + 2, "path", &missing.path, false);
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
}

fn push_missing_row_path_json(out: &mut String, missing: &MissingRowPath, indent: usize) {
    let prefix = " ".repeat(indent);
    out.push('\n');
    out.push_str(&prefix);
    out.push('{');
    push_json_string_field_at(out, indent + 2, "field_name", &missing.field_name, true);
    push_json_string_field_at(out, indent + 2, "path", &missing.path, false);
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
}

fn push_json_string_field(out: &mut String, key: &str, value: &str, comma: bool) {
    push_json_string_field_at(out, 2, key, value, comma);
}

fn push_json_string_field_at(out: &mut String, indent: usize, key: &str, value: &str, comma: bool) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\":{}{}",
        " ".repeat(indent),
        json_escape(key),
        json_string(value),
        suffix
    )
    .expect("write to string");
}

fn push_json_bool_field(out: &mut String, key: &str, value: bool, comma: bool) {
    push_json_bool_field_at(out, 2, key, value, comma);
}

fn push_json_bool_field_at(out: &mut String, indent: usize, key: &str, value: bool, comma: bool) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\":{}{}",
        " ".repeat(indent),
        json_escape(key),
        if value { "true" } else { "false" },
        suffix
    )
    .expect("write to string");
}

fn push_json_usize_field(out: &mut String, key: &str, value: usize, comma: bool) {
    push_json_usize_field_at(out, 2, key, value, comma);
}

fn push_json_usize_field_at(out: &mut String, indent: usize, key: &str, value: usize, comma: bool) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\":{}{}",
        " ".repeat(indent),
        json_escape(key),
        value,
        suffix
    )
    .expect("write to string");
}

fn push_json_count_map_field(
    out: &mut String,
    key: &str,
    counts: &BTreeMap<String, usize>,
    comma: bool,
) {
    push_json_count_map_field_at(out, 2, key, counts, comma);
}

fn push_json_count_map_field_at(
    out: &mut String,
    indent: usize,
    key: &str,
    counts: &BTreeMap<String, usize>,
    comma: bool,
) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\":{}{}",
        " ".repeat(indent),
        json_escape(key),
        json_count_map(counts),
        suffix
    )
    .expect("write to string");
}

fn json_string_array(values: &[String]) -> String {
    let mut out = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&json_string(value));
    }
    out.push(']');
    out
}

fn json_count_map(counts: &BTreeMap<String, usize>) -> String {
    let mut out = String::from("{");
    for (index, (key, value)) in counts.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        write!(out, "{}:{value}", json_string(key)).expect("write to string");
    }
    out.push('}');
    out
}

fn json_string(value: &str) -> String {
    format!("\"{}\"", json_escape(value))
}

fn json_escape(value: &str) -> String {
    let mut out = String::new();
    for character in value.chars() {
        match character {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            character if character.is_control() => {
                write!(out, "\\u{:04x}", character as u32).expect("write to string");
            }
            character => out.push(character),
        }
    }
    out
}

fn path_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn has_windows_absolute_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    use super::{plan_equation_batches, render_json, PlanOptions};

    const HEADER: &str = "schema_version\tbatch_id\tformula_id\tpackage\tcrate_name\truntime_symbol\toutput_variable\tcontract_path\tvalidation_card_path\tsource_seed_path\tvalidation_status\ttest_strategy\ttest_expression";

    fn manifest_row(
        formula_id: &str,
        status: &str,
        strategy: &str,
        contract_path: &str,
        validation_card_path: &str,
        source_seed_path: &str,
    ) -> String {
        manifest_row_with_linkage(
            formula_id,
            status,
            strategy,
            ("aero-codex-core", "aero_codex_core", "formula_symbol"),
            [contract_path, validation_card_path, source_seed_path],
        )
    }

    fn manifest_row_with_linkage(
        formula_id: &str,
        status: &str,
        strategy: &str,
        linkage: (&str, &str, &str),
        paths: [&str; 3],
    ) -> String {
        let (package, crate_name, runtime_symbol) = linkage;
        let [contract_path, validation_card_path, source_seed_path] = paths;
        [
            "aerocodex.equation_batch.v1",
            "m00-test",
            formula_id,
            package,
            crate_name,
            runtime_symbol,
            "output",
            contract_path,
            validation_card_path,
            source_seed_path,
            status,
            strategy,
            "value == 1.0",
        ]
        .join("\t")
    }

    fn temp_repo(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "aerocodex_rr006_plan_test_{}_{}",
            name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("equation-batches")).expect("create equation-batches");
        fs::create_dir_all(root.join("formula-vault/contracts")).expect("create contracts");
        fs::create_dir_all(root.join("validation/cards")).expect("create cards");
        fs::create_dir_all(root.join("validation/source_registry"))
            .expect("create source registry");
        write_workspace_package(
            &root,
            "aero-codex-core",
            "aero_codex_core",
            "pub fn formula_symbol() -> f64 { 1.0 }\n",
        );
        root
    }

    fn write_workspace_package(root: &Path, package: &str, crate_name: &str, lib_source: &str) {
        write_file(
            root,
            "Cargo.toml",
            &format!("[workspace]\nmembers = [\"crates/{package}\"]\n"),
        );
        write_file(
            root,
            &format!("crates/{package}/Cargo.toml"),
            &format!("[package]\nname = \"{package}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[lib]\nname = \"{crate_name}\"\npath = \"src/lib.rs\"\n"),
        );
        write_file(root, &format!("crates/{package}/src/lib.rs"), lib_source);
    }

    fn write_file(root: &Path, relative: &str, contents: &str) {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, contents).expect("write fixture file");
    }

    #[test]
    fn equation_batch_plan_rejects_missing_manifest_selection() {
        let err = PlanOptions::parse_args(&[]).expect_err("selection is required");

        assert!(err.contains("--manifest"));
        assert!(err.contains("--all-manifests"));
    }

    #[test]
    fn equation_batch_plan_rejects_manifest_and_all_manifests() {
        let err = PlanOptions::parse_args(&[
            "--manifest",
            "equation-batches/m00-canonical-units.tsv",
            "--all-manifests",
        ])
        .expect_err("mixed selection must fail");

        assert!(err.contains("--manifest"));
        assert!(err.contains("--all-manifests"));
        assert!(err.contains("together"));
    }

    #[test]
    fn equation_batch_plan_reports_sorted_counts_missing_paths_and_blocking() {
        let root = temp_repo("counts");
        write_file(&root, "formula-vault/contracts/a.yaml", "contract");
        write_file(&root, "formula-vault/contracts/m.yaml", "contract");
        write_file(&root, "formula-vault/contracts/z.yaml", "contract");
        write_file(&root, "validation/cards/a.yaml", "card");
        write_file(&root, "validation/cards/m.yaml", "card");
        write_file(&root, "validation/source_registry/a.yaml", "source");
        write_file(&root, "validation/source_registry/m.yaml", "source");
        write_file(&root, "validation/source_registry/z.yaml", "source");

        write_file(
            &root,
            "equation-batches/b.tsv",
            &format!(
                "{HEADER}\n{}\n{}\n",
                manifest_row(
                    "formula.z",
                    "research_required",
                    "tolerance",
                    "formula-vault/contracts/z.yaml",
                    "validation/cards/z.yaml",
                    "validation/source_registry/z.yaml",
                ),
                manifest_row(
                    "formula.m",
                    "implementation_verified",
                    "invariant",
                    "formula-vault/contracts/m.yaml",
                    "validation/cards/m.yaml",
                    "validation/source_registry/m.yaml",
                )
            ),
        );
        write_file(
            &root,
            "equation-batches/a.tsv",
            &format!(
                "{HEADER}\n{}\n",
                manifest_row(
                    "formula.a",
                    "implementation_verified",
                    "exact",
                    "formula-vault/contracts/a.yaml",
                    "validation/cards/a.yaml",
                    "validation/source_registry/a.yaml",
                )
            ),
        );

        let report = plan_equation_batches(
            &root,
            &PlanOptions {
                manifests: vec![
                    PathBuf::from("equation-batches/b.tsv"),
                    PathBuf::from("equation-batches/a.tsv"),
                ],
                all_manifests: false,
                json: false,
            },
        )
        .expect("plan succeeds");

        assert_eq!(report.command, "equation-batch plan");
        assert_eq!(report.manifest_count, 2);
        assert_eq!(report.row_count, 3);
        assert_eq!(report.manifests[0].path, "equation-batches/a.tsv");
        assert_eq!(report.manifests[1].path, "equation-batches/b.tsv");
        assert_eq!(report.manifests[1].rows[0].formula_id, "formula.m");
        assert_eq!(report.manifests[1].rows[1].formula_id, "formula.z");
        assert_eq!(
            report.counts_by_status.get("implementation_verified"),
            Some(&2)
        );
        assert_eq!(report.counts_by_status.get("research_required"), Some(&1));
        assert_eq!(report.counts_by_test_strategy.get("exact"), Some(&1));
        assert_eq!(report.counts_by_test_strategy.get("invariant"), Some(&1));
        assert_eq!(report.counts_by_test_strategy.get("tolerance"), Some(&1));
        assert_eq!(report.blocked_execution_count, 1);
        assert_eq!(report.missing_paths.len(), 1);
        assert_eq!(report.missing_paths[0].path, "validation/cards/z.yaml");
        assert_eq!(report.missing_paths[0].formula_id, "formula.z");
    }

    #[test]
    fn equation_batch_plan_reports_static_checks_for_workspace_package_row() {
        let root = temp_repo("static_ok");
        write_file(&root, "formula-vault/contracts/a.yaml", "contract");
        write_file(&root, "validation/cards/a.yaml", "card");
        write_file(&root, "validation/source_registry/a.yaml", "source");
        write_file(
            &root,
            "equation-batches/a.tsv",
            &format!(
                "{HEADER}\n{}\n",
                manifest_row(
                    "formula.a",
                    "implementation_verified",
                    "exact",
                    "formula-vault/contracts/a.yaml",
                    "validation/cards/a.yaml",
                    "validation/source_registry/a.yaml",
                )
            ),
        );

        let report = plan_equation_batches(
            &root,
            &PlanOptions {
                manifests: vec![PathBuf::from("equation-batches/a.tsv")],
                all_manifests: false,
                json: true,
            },
        )
        .expect("plan succeeds");
        let row = &report.manifests[0].rows[0];

        assert_eq!(row.static_checks.path_status, "ok");
        assert_eq!(row.static_checks.package_status, "ok");
        assert_eq!(row.static_checks.crate_status, "ok");
        assert_eq!(row.static_checks.symbol_status, "ok");
        assert!(row.static_checks.static_warnings.is_empty());

        let json = render_json(&report);
        assert!(json.contains("\"static_checks\""));
        assert!(json.contains("\"path_status\":\"ok\""));
        assert!(json.contains("\"package_status\":\"ok\""));
        assert!(json.contains("\"crate_status\":\"ok\""));
        assert!(json.contains("\"symbol_status\":\"ok\""));
        assert!(json.contains("\"static_warnings\":[]"));
    }

    #[test]
    fn equation_batch_plan_reports_static_check_failures_per_row() {
        let root = temp_repo("static_failures");
        write_file(&root, "formula-vault/contracts/a.yaml", "contract");
        write_file(&root, "formula-vault/contracts/b.yaml", "contract");
        write_file(&root, "formula-vault/contracts/c.yaml", "contract");
        write_file(&root, "validation/cards/a.yaml", "card");
        write_file(&root, "validation/cards/b.yaml", "card");
        write_file(&root, "validation/source_registry/a.yaml", "source");
        write_file(&root, "validation/source_registry/b.yaml", "source");
        write_file(&root, "validation/source_registry/c.yaml", "source");
        write_file(
            &root,
            "equation-batches/a.tsv",
            &format!(
                "{HEADER}\n{}\n{}\n{}\n",
                manifest_row_with_linkage(
                    "formula.missing_package",
                    "implementation_verified",
                    "exact",
                    ("aero-codex-missing", "aero_codex_missing", "formula_symbol"),
                    [
                        "formula-vault/contracts/a.yaml",
                        "validation/cards/a.yaml",
                        "validation/source_registry/a.yaml",
                    ],
                ),
                manifest_row_with_linkage(
                    "formula.bad_crate",
                    "implementation_verified",
                    "exact",
                    ("aero-codex-core", "wrong_crate", "formula_symbol"),
                    [
                        "formula-vault/contracts/b.yaml",
                        "validation/cards/b.yaml",
                        "validation/source_registry/b.yaml",
                    ],
                ),
                manifest_row_with_linkage(
                    "formula.missing_symbol",
                    "implementation_verified",
                    "exact",
                    ("aero-codex-core", "aero_codex_core", "not_present_symbol"),
                    [
                        "formula-vault/contracts/c.yaml",
                        "validation/cards/c.yaml",
                        "validation/source_registry/c.yaml",
                    ],
                )
            ),
        );

        let report = plan_equation_batches(
            &root,
            &PlanOptions {
                manifests: vec![PathBuf::from("equation-batches/a.tsv")],
                all_manifests: false,
                json: true,
            },
        )
        .expect("plan succeeds");
        let rows = &report.manifests[0].rows;

        assert_eq!(rows[0].formula_id, "formula.bad_crate");
        assert_eq!(rows[0].static_checks.package_status, "ok");
        assert_eq!(rows[0].static_checks.crate_status, "mismatch");
        assert_eq!(rows[0].static_checks.symbol_status, "unknown");
        assert!(rows[0]
            .static_checks
            .static_warnings
            .iter()
            .any(|warning| warning.contains("crate_name mismatch")));

        assert_eq!(rows[1].formula_id, "formula.missing_package");
        assert_eq!(rows[1].static_checks.package_status, "missing");
        assert_eq!(rows[1].static_checks.path_status, "unknown");
        assert_eq!(rows[1].static_checks.crate_status, "unknown");
        assert_eq!(rows[1].static_checks.symbol_status, "unknown");
        assert!(rows[1]
            .static_checks
            .static_warnings
            .iter()
            .any(|warning| warning.contains("package not found")));

        assert_eq!(rows[2].formula_id, "formula.missing_symbol");
        assert_eq!(rows[2].static_checks.path_status, "missing");
        assert_eq!(rows[2].static_checks.validation_card_path, "missing");
        assert_eq!(rows[2].static_checks.package_status, "ok");
        assert_eq!(rows[2].static_checks.crate_status, "ok");
        assert_eq!(rows[2].static_checks.symbol_status, "missing");
        assert!(!rows[2].execution_eligible);
    }

    #[test]
    fn equation_batch_plan_json_contains_required_deterministic_fields() {
        let root = temp_repo("json");
        write_file(&root, "formula-vault/contracts/a.yaml", "contract");
        write_file(&root, "validation/cards/a.yaml", "card");
        write_file(&root, "validation/source_registry/a.yaml", "source");
        write_file(
            &root,
            "equation-batches/a.tsv",
            &format!(
                "{HEADER}\n{}\n",
                manifest_row(
                    "formula.a",
                    "implementation_verified",
                    "exact",
                    "formula-vault/contracts/a.yaml",
                    "validation/cards/a.yaml",
                    "validation/source_registry/a.yaml",
                )
            ),
        );

        let report = plan_equation_batches(
            &root,
            &PlanOptions {
                manifests: Vec::new(),
                all_manifests: true,
                json: true,
            },
        )
        .expect("all-manifests plan succeeds");
        let json = render_json(&report);

        assert!(json.contains("\"ok\":true"));
        assert!(json.contains("\"command\":\"equation-batch plan\""));
        assert!(json.contains("\"schema_version\":\"aerocodex.equation_batch.plan.v1\""));
        assert!(json.contains("\"manifest_count\":1"));
        assert!(json.contains("\"row_count\":1"));
        assert!(json.contains("\"counts_by_status\""));
        assert!(json.contains("\"counts_by_batch\""));
        assert!(json.contains("\"counts_by_test_strategy\""));
        assert!(json.contains("\"blocked_execution_count\":0"));
        assert!(json.contains("\"missing_paths\":[]"));
        assert!(json.contains("\"safety_notice\""));
    }
}
