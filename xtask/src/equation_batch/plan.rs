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
    pub missing_paths: Vec<MissingRowPath>,
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
        [
            "aerocodex.equation_batch.v1",
            "m00-test",
            formula_id,
            "aero-codex-core",
            "aero_codex_core",
            "formula_symbol",
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
        root
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
