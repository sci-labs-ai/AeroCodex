use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use super::plan::{self, PlanOptions, RowPlanReport};

pub const REPORT_COMMAND: &str = "equation-batch report";
pub const REPORT_SCHEMA_VERSION: &str = "aerocodex.equation_batch.status_report.v1";
pub const GENERATED_BY: &str = "xtask equation-batch report";
pub const SAFETY_NOTICE: &str = "AeroCodex equation-batch status reporting is deterministic inventory evidence for research and preliminary-design review only; it is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval, and it does not make formulas executable or promote validation status.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportOptions {
    pub all_manifests: bool,
    pub out: PathBuf,
    pub check: bool,
}

impl ReportOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        let mut all_manifests = false;
        let mut out: Option<PathBuf> = None;
        let mut check = false;
        let mut index = 0usize;

        while index < args.len() {
            match args[index] {
                "--all-manifests" => {
                    if all_manifests {
                        return Err(
                            "usage error: --all-manifests was supplied more than once".to_string()
                        );
                    }
                    all_manifests = true;
                }
                "--out" => {
                    if out.is_some() {
                        return Err("usage error: --out was supplied more than once".to_string());
                    }
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "usage error: --out requires a path".to_string())?;
                    if value.starts_with("--") {
                        return Err("usage error: --out requires a path".to_string());
                    }
                    out = Some(PathBuf::from(value));
                }
                "--check" => {
                    if check {
                        return Err("usage error: --check was supplied more than once".to_string());
                    }
                    check = true;
                }
                unknown if unknown.starts_with("--") => {
                    return Err(format!(
                        "usage error: unknown equation-batch report flag `{unknown}`"
                    ));
                }
                unexpected => {
                    return Err(format!(
                        "usage error: unexpected equation-batch report argument `{unexpected}`"
                    ));
                }
            }
            index += 1;
        }

        if !all_manifests {
            return Err("usage error: equation-batch report requires --all-manifests".to_string());
        }

        let out = out
            .ok_or_else(|| "usage error: equation-batch report requires --out PATH".to_string())?;

        Ok(Self {
            all_manifests,
            out,
            check,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquationBatchStatusReport {
    pub ok: bool,
    pub command: String,
    pub schema_version: String,
    pub generated_by: String,
    pub manifest_count: usize,
    pub row_count: usize,
    pub counts_by_status: BTreeMap<String, usize>,
    pub counts_by_family: BTreeMap<String, usize>,
    pub counts_by_batch: BTreeMap<String, usize>,
    pub counts_by_test_strategy: BTreeMap<String, usize>,
    pub counts_by_static_symbol_status: BTreeMap<String, usize>,
    pub counts_by_static_path_status: BTreeMap<String, usize>,
    pub counts_by_static_package_status: BTreeMap<String, usize>,
    pub counts_by_static_crate_status: BTreeMap<String, usize>,
    pub counts_by_static_runtime_source_root_status: BTreeMap<String, usize>,
    pub execution_eligible_count: usize,
    pub metadata_complete_count: usize,
    pub blocked_execution_count: usize,
    pub missing_metadata_path_count: usize,
    pub static_warning_count: usize,
    pub manifests: Vec<ManifestStatusReport>,
    pub missing_metadata_paths: Vec<StatusMissingMetadataPath>,
    pub static_warnings: Vec<StatusStaticWarning>,
    pub non_claims: Vec<String>,
    pub safety_notice: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestStatusReport {
    pub path: String,
    pub batch_id: String,
    pub row_count: usize,
    pub counts_by_status: BTreeMap<String, usize>,
    pub counts_by_family: BTreeMap<String, usize>,
    pub counts_by_test_strategy: BTreeMap<String, usize>,
    pub counts_by_static_symbol_status: BTreeMap<String, usize>,
    pub counts_by_static_path_status: BTreeMap<String, usize>,
    pub counts_by_static_package_status: BTreeMap<String, usize>,
    pub counts_by_static_crate_status: BTreeMap<String, usize>,
    pub counts_by_static_runtime_source_root_status: BTreeMap<String, usize>,
    pub execution_eligible_count: usize,
    pub metadata_complete_count: usize,
    pub blocked_execution_count: usize,
    pub missing_metadata_path_count: usize,
    pub static_warning_count: usize,
    pub rows: Vec<RowStatusReport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowStatusReport {
    pub formula_id: String,
    pub family: String,
    pub batch_id: String,
    pub validation_status: String,
    pub test_strategy: String,
    pub metadata_complete: bool,
    pub execution_eligible: bool,
    pub execution_readiness: String,
    pub blocked_execution: bool,
    pub static_checks: RowStatusStaticChecks,
    pub missing_paths: Vec<StatusMissingRowPath>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowStatusStaticChecks {
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
pub struct StatusMissingRowPath {
    pub field_name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusMissingMetadataPath {
    pub manifest_path: String,
    pub formula_id: String,
    pub field_name: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusStaticWarning {
    pub manifest_path: String,
    pub formula_id: String,
    pub warning: String,
}

pub fn run_report_command(root: &Path, options: &ReportOptions) -> Result<(), String> {
    let report = build_status_report(root)?;
    let expected = render_status_report_json(&report);
    let out_path = output_path(root, &options.out);

    if options.check {
        let existing = fs::read_to_string(&out_path).map_err(|error| {
            format!(
                "equation-batch status report check failed; cannot read {}: {error}",
                out_path.display()
            )
        })?;
        if existing != expected {
            return Err(format!(
                "equation-batch status report check failed; output is missing or stale: {}",
                out_path.display()
            ));
        }
        println!(
            "equation_batch_status_report_check=PASS path={}",
            out_path.display()
        );
        return Ok(());
    }

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{}: {error}", parent.display()))?;
    }
    fs::write(&out_path, expected).map_err(|error| format!("{}: {error}", out_path.display()))?;
    println!("wrote_equation_batch_status_report={}", out_path.display());
    Ok(())
}

pub fn build_status_report(root: &Path) -> Result<EquationBatchStatusReport, String> {
    let plan_options = PlanOptions {
        manifests: Vec::new(),
        all_manifests: true,
        json: true,
    };
    let plan_report = plan::plan_equation_batches(root, &plan_options)?;

    let mut counts_by_family = BTreeMap::new();
    let mut counts_by_static_symbol_status = BTreeMap::new();
    let mut counts_by_static_path_status = BTreeMap::new();
    let mut counts_by_static_package_status = BTreeMap::new();
    let mut counts_by_static_crate_status = BTreeMap::new();
    let mut counts_by_static_runtime_source_root_status = BTreeMap::new();
    let mut execution_eligible_count = 0usize;
    let mut metadata_complete_count = 0usize;
    let mut static_warning_count = 0usize;
    let mut static_warnings = Vec::new();
    let mut manifests = Vec::new();

    for manifest in &plan_report.manifests {
        let mut manifest_counts_by_family = BTreeMap::new();
        let mut manifest_counts_by_static_symbol_status = BTreeMap::new();
        let mut manifest_counts_by_static_path_status = BTreeMap::new();
        let mut manifest_counts_by_static_package_status = BTreeMap::new();
        let mut manifest_counts_by_static_crate_status = BTreeMap::new();
        let mut manifest_counts_by_static_runtime_source_root_status = BTreeMap::new();
        let mut manifest_execution_eligible_count = 0usize;
        let mut manifest_metadata_complete_count = 0usize;
        let mut manifest_static_warning_count = 0usize;
        let mut rows = Vec::new();

        for row in &manifest.rows {
            let status_row = status_row_from_plan_row(row);
            increment(&mut counts_by_family, &status_row.family);
            increment(&mut manifest_counts_by_family, &status_row.family);
            increment(
                &mut counts_by_static_symbol_status,
                &status_row.static_checks.symbol_status,
            );
            increment(
                &mut manifest_counts_by_static_symbol_status,
                &status_row.static_checks.symbol_status,
            );
            increment(
                &mut counts_by_static_path_status,
                &status_row.static_checks.path_status,
            );
            increment(
                &mut manifest_counts_by_static_path_status,
                &status_row.static_checks.path_status,
            );
            increment(
                &mut counts_by_static_package_status,
                &status_row.static_checks.package_status,
            );
            increment(
                &mut manifest_counts_by_static_package_status,
                &status_row.static_checks.package_status,
            );
            increment(
                &mut counts_by_static_crate_status,
                &status_row.static_checks.crate_status,
            );
            increment(
                &mut manifest_counts_by_static_crate_status,
                &status_row.static_checks.crate_status,
            );
            increment(
                &mut counts_by_static_runtime_source_root_status,
                &status_row.static_checks.runtime_source_root,
            );
            increment(
                &mut manifest_counts_by_static_runtime_source_root_status,
                &status_row.static_checks.runtime_source_root,
            );

            if status_row.execution_eligible {
                execution_eligible_count += 1;
                manifest_execution_eligible_count += 1;
            }
            if status_row.metadata_complete {
                metadata_complete_count += 1;
                manifest_metadata_complete_count += 1;
            }
            static_warning_count += status_row.static_checks.static_warnings.len();
            manifest_static_warning_count += status_row.static_checks.static_warnings.len();
            for warning in &status_row.static_checks.static_warnings {
                static_warnings.push(StatusStaticWarning {
                    manifest_path: manifest.path.clone(),
                    formula_id: status_row.formula_id.clone(),
                    warning: warning.clone(),
                });
            }
            rows.push(status_row);
        }

        manifests.push(ManifestStatusReport {
            path: manifest.path.clone(),
            batch_id: manifest.batch_id.clone(),
            row_count: manifest.row_count,
            counts_by_status: manifest.counts_by_status.clone(),
            counts_by_family: manifest_counts_by_family,
            counts_by_test_strategy: manifest.counts_by_test_strategy.clone(),
            counts_by_static_symbol_status: manifest_counts_by_static_symbol_status,
            counts_by_static_path_status: manifest_counts_by_static_path_status,
            counts_by_static_package_status: manifest_counts_by_static_package_status,
            counts_by_static_crate_status: manifest_counts_by_static_crate_status,
            counts_by_static_runtime_source_root_status:
                manifest_counts_by_static_runtime_source_root_status,
            execution_eligible_count: manifest_execution_eligible_count,
            metadata_complete_count: manifest_metadata_complete_count,
            blocked_execution_count: manifest.blocked_execution_count,
            missing_metadata_path_count: manifest.missing_path_count,
            static_warning_count: manifest_static_warning_count,
            rows,
        });
    }

    let mut missing_metadata_paths = plan_report
        .missing_paths
        .iter()
        .map(|missing| StatusMissingMetadataPath {
            manifest_path: missing.manifest_path.clone(),
            formula_id: missing.formula_id.clone(),
            field_name: missing.field_name.clone(),
            path: missing.path.clone(),
        })
        .collect::<Vec<_>>();
    missing_metadata_paths.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.formula_id.cmp(&right.formula_id))
            .then(left.manifest_path.cmp(&right.manifest_path))
            .then(left.field_name.cmp(&right.field_name))
    });
    static_warnings.sort_by(|left, right| {
        left.warning
            .cmp(&right.warning)
            .then(left.formula_id.cmp(&right.formula_id))
            .then(left.manifest_path.cmp(&right.manifest_path))
    });

    Ok(EquationBatchStatusReport {
        ok: true,
        command: REPORT_COMMAND.to_string(),
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        generated_by: GENERATED_BY.to_string(),
        manifest_count: plan_report.manifest_count,
        row_count: plan_report.row_count,
        counts_by_status: plan_report.counts_by_status,
        counts_by_family,
        counts_by_batch: plan_report.counts_by_batch,
        counts_by_test_strategy: plan_report.counts_by_test_strategy,
        counts_by_static_symbol_status,
        counts_by_static_path_status,
        counts_by_static_package_status,
        counts_by_static_crate_status,
        counts_by_static_runtime_source_root_status,
        execution_eligible_count,
        metadata_complete_count,
        blocked_execution_count: plan_report.blocked_execution_count,
        missing_metadata_path_count: missing_metadata_paths.len(),
        static_warning_count,
        manifests,
        missing_metadata_paths,
        static_warnings,
        non_claims: non_claims(),
        safety_notice: SAFETY_NOTICE.to_string(),
    })
}

fn status_row_from_plan_row(row: &RowPlanReport) -> RowStatusReport {
    let mut static_warnings = row.static_checks.static_warnings.clone();
    static_warnings.sort();
    let mut missing_paths = row
        .missing_paths
        .iter()
        .map(|missing| StatusMissingRowPath {
            field_name: missing.field_name.clone(),
            path: missing.path.clone(),
        })
        .collect::<Vec<_>>();
    missing_paths.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.field_name.cmp(&right.field_name))
    });

    let metadata_complete = missing_paths.is_empty();
    let execution_readiness = execution_readiness(row, metadata_complete);
    RowStatusReport {
        formula_id: row.formula_id.clone(),
        family: family_from_formula_id(&row.formula_id),
        batch_id: row.batch_id.clone(),
        validation_status: row.validation_status.clone(),
        test_strategy: row.test_strategy.clone(),
        metadata_complete,
        execution_eligible: row.execution_eligible,
        execution_readiness,
        blocked_execution: !row.execution_eligible,
        static_checks: RowStatusStaticChecks {
            path_status: row.static_checks.path_status.clone(),
            contract_path: row.static_checks.contract_path.clone(),
            validation_card_path: row.static_checks.validation_card_path.clone(),
            source_seed_path: row.static_checks.source_seed_path.clone(),
            runtime_source_root: row.static_checks.runtime_source_root.clone(),
            package_status: row.static_checks.package_status.clone(),
            crate_status: row.static_checks.crate_status.clone(),
            symbol_status: row.static_checks.symbol_status.clone(),
            static_warnings,
        },
        missing_paths,
    }
}

fn execution_readiness(row: &RowPlanReport, metadata_complete: bool) -> String {
    if row.execution_eligible {
        "eligible".to_string()
    } else if row.validation_status != "implementation_verified" {
        "blocked_by_validation_status".to_string()
    } else if !metadata_complete {
        "blocked_by_missing_metadata".to_string()
    } else {
        "blocked_by_static_or_policy_gate".to_string()
    }
}

fn family_from_formula_id(formula_id: &str) -> String {
    let mut parts = formula_id.split('.');
    match parts.next() {
        Some("formula_vault") => parts.next().unwrap_or("unknown").to_string(),
        Some(first) if !first.is_empty() => first.to_string(),
        _ => "unknown".to_string(),
    }
}

fn increment(counts: &mut BTreeMap<String, usize>, key: &str) {
    *counts.entry(key.to_string()).or_insert(0) += 1;
}

fn output_path(root: &Path, out: &Path) -> PathBuf {
    if out.is_absolute() {
        out.to_path_buf()
    } else {
        root.join(out)
    }
}

fn non_claims() -> Vec<String> {
    vec![
        "AeroCodex is research and preliminary-design software, not certified operational aerospace software.".to_string(),
        "This report is inventory/status evidence only; it does not validate, certify, or make any equation publication-ready.".to_string(),
        "This report is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.".to_string(),
        "Blocked or research_required formulas remain non-executable through the normal CLI and library paths.".to_string(),
        "Static path, package, crate, and symbol checks are textual readiness indicators, not runtime execution or probe compilation evidence.".to_string(),
        "M07 rows are excluded unless they are present in equation-batches/*.tsv; M07 quarantine reporting belongs to separate tasks.".to_string(),
        "The report does not evaluate manifest test expressions, compile runtime code, generate probes, generate registries, or promote validation status.".to_string(),
    ]
}

pub fn render_status_report_json(report: &EquationBatchStatusReport) -> String {
    let mut out = String::new();
    out.push('{');
    push_bool_field(&mut out, 2, "ok", report.ok, true);
    push_string_field(&mut out, 2, "command", &report.command, true);
    push_string_field(&mut out, 2, "schema_version", &report.schema_version, true);
    push_string_field(&mut out, 2, "generated_by", &report.generated_by, true);
    push_usize_field(&mut out, 2, "manifest_count", report.manifest_count, true);
    push_usize_field(&mut out, 2, "row_count", report.row_count, true);
    push_count_map_field(
        &mut out,
        2,
        "counts_by_status",
        &report.counts_by_status,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_family",
        &report.counts_by_family,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_batch",
        &report.counts_by_batch,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_test_strategy",
        &report.counts_by_test_strategy,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_static_symbol_status",
        &report.counts_by_static_symbol_status,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_static_path_status",
        &report.counts_by_static_path_status,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_static_package_status",
        &report.counts_by_static_package_status,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_static_crate_status",
        &report.counts_by_static_crate_status,
        true,
    );
    push_count_map_field(
        &mut out,
        2,
        "counts_by_static_runtime_source_root_status",
        &report.counts_by_static_runtime_source_root_status,
        true,
    );
    push_usize_field(
        &mut out,
        2,
        "execution_eligible_count",
        report.execution_eligible_count,
        true,
    );
    push_usize_field(
        &mut out,
        2,
        "metadata_complete_count",
        report.metadata_complete_count,
        true,
    );
    push_usize_field(
        &mut out,
        2,
        "blocked_execution_count",
        report.blocked_execution_count,
        true,
    );
    push_usize_field(
        &mut out,
        2,
        "missing_metadata_path_count",
        report.missing_metadata_path_count,
        true,
    );
    push_usize_field(
        &mut out,
        2,
        "static_warning_count",
        report.static_warning_count,
        true,
    );
    push_manifests_json(&mut out, &report.manifests, true);
    push_missing_metadata_paths_json(&mut out, &report.missing_metadata_paths, true);
    push_static_warnings_json(&mut out, &report.static_warnings, true);
    push_string_array_field(&mut out, 2, "non_claims", &report.non_claims, true);
    push_string_field(&mut out, 2, "safety_notice", &report.safety_notice, false);
    out.push_str("\n}\n");
    out
}

fn push_manifests_json(out: &mut String, manifests: &[ManifestStatusReport], comma: bool) {
    let suffix = if comma { "," } else { "" };
    out.push_str("\n  \"manifests\": [");
    for (index, manifest) in manifests.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        push_manifest_json(out, manifest, 4);
    }
    out.push_str("\n  ]");
    out.push_str(suffix);
}

fn push_manifest_json(out: &mut String, manifest: &ManifestStatusReport, indent: usize) {
    let prefix = " ".repeat(indent);
    out.push('\n');
    out.push_str(&prefix);
    out.push('{');
    push_string_field(out, indent + 2, "path", &manifest.path, true);
    push_string_field(out, indent + 2, "batch_id", &manifest.batch_id, true);
    push_usize_field(out, indent + 2, "row_count", manifest.row_count, true);
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_status",
        &manifest.counts_by_status,
        true,
    );
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_family",
        &manifest.counts_by_family,
        true,
    );
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_test_strategy",
        &manifest.counts_by_test_strategy,
        true,
    );
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_static_symbol_status",
        &manifest.counts_by_static_symbol_status,
        true,
    );
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_static_path_status",
        &manifest.counts_by_static_path_status,
        true,
    );
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_static_package_status",
        &manifest.counts_by_static_package_status,
        true,
    );
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_static_crate_status",
        &manifest.counts_by_static_crate_status,
        true,
    );
    push_count_map_field(
        out,
        indent + 2,
        "counts_by_static_runtime_source_root_status",
        &manifest.counts_by_static_runtime_source_root_status,
        true,
    );
    push_usize_field(
        out,
        indent + 2,
        "execution_eligible_count",
        manifest.execution_eligible_count,
        true,
    );
    push_usize_field(
        out,
        indent + 2,
        "metadata_complete_count",
        manifest.metadata_complete_count,
        true,
    );
    push_usize_field(
        out,
        indent + 2,
        "blocked_execution_count",
        manifest.blocked_execution_count,
        true,
    );
    push_usize_field(
        out,
        indent + 2,
        "missing_metadata_path_count",
        manifest.missing_metadata_path_count,
        true,
    );
    push_usize_field(
        out,
        indent + 2,
        "static_warning_count",
        manifest.static_warning_count,
        true,
    );
    push_rows_json(out, &manifest.rows, indent + 2, false);
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
}

fn push_rows_json(out: &mut String, rows: &[RowStatusReport], indent: usize, comma: bool) {
    let prefix = " ".repeat(indent);
    let suffix = if comma { "," } else { "" };
    out.push_str(&format!("\n{prefix}\"rows\": ["));
    for (index, row) in rows.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        push_row_json(out, row, indent + 2);
    }
    out.push_str(&format!("\n{prefix}]"));
    out.push_str(suffix);
}

fn push_row_json(out: &mut String, row: &RowStatusReport, indent: usize) {
    let prefix = " ".repeat(indent);
    out.push('\n');
    out.push_str(&prefix);
    out.push('{');
    push_string_field(out, indent + 2, "formula_id", &row.formula_id, true);
    push_string_field(out, indent + 2, "family", &row.family, true);
    push_string_field(out, indent + 2, "batch_id", &row.batch_id, true);
    push_string_field(
        out,
        indent + 2,
        "validation_status",
        &row.validation_status,
        true,
    );
    push_string_field(out, indent + 2, "test_strategy", &row.test_strategy, true);
    push_bool_field(
        out,
        indent + 2,
        "metadata_complete",
        row.metadata_complete,
        true,
    );
    push_bool_field(
        out,
        indent + 2,
        "execution_eligible",
        row.execution_eligible,
        true,
    );
    push_string_field(
        out,
        indent + 2,
        "execution_readiness",
        &row.execution_readiness,
        true,
    );
    push_bool_field(
        out,
        indent + 2,
        "blocked_execution",
        row.blocked_execution,
        true,
    );
    push_static_checks_json(out, &row.static_checks, indent + 2, true);
    push_missing_row_paths_json(out, &row.missing_paths, indent + 2, false);
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
}

fn push_static_checks_json(
    out: &mut String,
    checks: &RowStatusStaticChecks,
    indent: usize,
    comma: bool,
) {
    let prefix = " ".repeat(indent);
    let suffix = if comma { "," } else { "" };
    out.push_str(&format!("\n{prefix}\"static_checks\": {{"));
    push_string_field(out, indent + 2, "path_status", &checks.path_status, true);
    push_string_field(
        out,
        indent + 2,
        "contract_path",
        &checks.contract_path,
        true,
    );
    push_string_field(
        out,
        indent + 2,
        "validation_card_path",
        &checks.validation_card_path,
        true,
    );
    push_string_field(
        out,
        indent + 2,
        "source_seed_path",
        &checks.source_seed_path,
        true,
    );
    push_string_field(
        out,
        indent + 2,
        "runtime_source_root",
        &checks.runtime_source_root,
        true,
    );
    push_string_field(
        out,
        indent + 2,
        "package_status",
        &checks.package_status,
        true,
    );
    push_string_field(out, indent + 2, "crate_status", &checks.crate_status, true);
    push_string_field(
        out,
        indent + 2,
        "symbol_status",
        &checks.symbol_status,
        true,
    );
    push_string_array_field(
        out,
        indent + 2,
        "static_warnings",
        &checks.static_warnings,
        false,
    );
    out.push('\n');
    out.push_str(&prefix);
    out.push('}');
    out.push_str(suffix);
}

fn push_missing_row_paths_json(
    out: &mut String,
    missing_paths: &[StatusMissingRowPath],
    indent: usize,
    comma: bool,
) {
    let prefix = " ".repeat(indent);
    let suffix = if comma { "," } else { "" };
    out.push_str(&format!("\n{prefix}\"missing_paths\": ["));
    for (index, missing) in missing_paths.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let item_prefix = " ".repeat(indent + 2);
        out.push('\n');
        out.push_str(&item_prefix);
        out.push('{');
        push_string_field(out, indent + 4, "field_name", &missing.field_name, true);
        push_string_field(out, indent + 4, "path", &missing.path, false);
        out.push('\n');
        out.push_str(&item_prefix);
        out.push('}');
    }
    out.push_str(&format!("\n{prefix}]"));
    out.push_str(suffix);
}

fn push_missing_metadata_paths_json(
    out: &mut String,
    missing_paths: &[StatusMissingMetadataPath],
    comma: bool,
) {
    let suffix = if comma { "," } else { "" };
    out.push_str("\n  \"missing_metadata_paths\": [");
    for (index, missing) in missing_paths.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str("\n    {");
        push_string_field(out, 6, "manifest_path", &missing.manifest_path, true);
        push_string_field(out, 6, "formula_id", &missing.formula_id, true);
        push_string_field(out, 6, "field_name", &missing.field_name, true);
        push_string_field(out, 6, "path", &missing.path, false);
        out.push_str("\n    }");
    }
    out.push_str("\n  ]");
    out.push_str(suffix);
}

fn push_static_warnings_json(out: &mut String, warnings: &[StatusStaticWarning], comma: bool) {
    let suffix = if comma { "," } else { "" };
    out.push_str("\n  \"static_warnings\": [");
    for (index, warning) in warnings.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str("\n    {");
        push_string_field(out, 6, "manifest_path", &warning.manifest_path, true);
        push_string_field(out, 6, "formula_id", &warning.formula_id, true);
        push_string_field(out, 6, "warning", &warning.warning, false);
        out.push_str("\n    }");
    }
    out.push_str("\n  ]");
    out.push_str(suffix);
}

fn push_string_field(out: &mut String, indent: usize, key: &str, value: &str, comma: bool) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\": {}{}",
        " ".repeat(indent),
        json_escape(key),
        json_string(value),
        suffix
    )
    .expect("write to string");
}

fn push_string_array_field(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &[String],
    comma: bool,
) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\": {}{}",
        " ".repeat(indent),
        json_escape(key),
        json_string_array(values),
        suffix
    )
    .expect("write to string");
}

fn push_bool_field(out: &mut String, indent: usize, key: &str, value: bool, comma: bool) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\": {}{}",
        " ".repeat(indent),
        json_escape(key),
        if value { "true" } else { "false" },
        suffix
    )
    .expect("write to string");
}

fn push_usize_field(out: &mut String, indent: usize, key: &str, value: usize, comma: bool) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\": {}{}",
        " ".repeat(indent),
        json_escape(key),
        value,
        suffix
    )
    .expect("write to string");
}

fn push_count_map_field(
    out: &mut String,
    indent: usize,
    key: &str,
    counts: &BTreeMap<String, usize>,
    comma: bool,
) {
    let suffix = if comma { "," } else { "" };
    write!(
        out,
        "\n{}\"{}\": {}{}",
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
            out.push_str(", ");
        }
        write!(out, "{}: {value}", json_string(key)).expect("write to string");
    }
    out.push('}');
    out
}

fn json_string_array(values: &[String]) -> String {
    let mut out = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&json_string(value));
    }
    out.push(']');
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

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::{
        build_status_report, render_status_report_json, ReportOptions, REPORT_SCHEMA_VERSION,
    };

    const HEADER: &str = "schema_version\tbatch_id\tformula_id\tpackage\tcrate_name\truntime_symbol\toutput_variable\tcontract_path\tvalidation_card_path\tsource_seed_path\tvalidation_status\ttest_strategy\ttest_expression";

    fn temp_repo(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "aerocodex_rr008_report_test_{}_{}",
            name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("equation-batches")).expect("create equation-batches");
        write_file(
            &root,
            "Cargo.toml",
            "[workspace]\nmembers = [\"crates/aero-codex-core\"]\n",
        );
        write_file(
            &root,
            "crates/aero-codex-core/Cargo.toml",
            "[package]\nname = \"aero-codex-core\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[lib]\nname = \"aero_codex_core\"\npath = \"src/lib.rs\"\n",
        );
        write_file(
            &root,
            "crates/aero-codex-core/src/lib.rs",
            "pub fn formula_symbol() -> f64 { 1.0 }\n",
        );
        write_file(&root, "formula-vault/contracts/a.yaml", "contract");
        write_file(&root, "formula-vault/contracts/z.yaml", "contract");
        write_file(&root, "validation/cards/a.yaml", "card");
        write_file(&root, "validation/source_registry/a.yaml", "source");
        write_file(&root, "validation/source_registry/z.yaml", "source");
        root
    }

    fn write_file(root: &std::path::Path, relative: &str, contents: &str) {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent");
        }
        fs::write(path, contents).expect("write fixture file");
    }

    fn row(formula_id: &str, status: &str, strategy: &str, card_path: &str) -> String {
        [
            "aerocodex.equation_batch.v1",
            "m00-test",
            formula_id,
            "aero-codex-core",
            "aero_codex_core",
            "formula_symbol",
            "output",
            "formula-vault/contracts/a.yaml",
            card_path,
            "validation/source_registry/a.yaml",
            status,
            strategy,
            "value == 1.0",
        ]
        .join("\t")
    }

    #[test]
    fn equation_batch_report_rejects_missing_out_and_missing_all_manifests() {
        let missing_out =
            ReportOptions::parse_args(&["--all-manifests"]).expect_err("--out is required");
        assert!(missing_out.contains("--out"));

        let missing_all = ReportOptions::parse_args(&["--out", "generated/report.json"])
            .expect_err("--all-manifests is required");
        assert!(missing_all.contains("--all-manifests"));
    }

    #[test]
    fn equation_batch_report_rejects_unknown_flags() {
        let err = ReportOptions::parse_args(&["--unknown-flag"]).expect_err("unknown flags fail");

        assert!(err.contains("unknown"));
        assert!(err.contains("--unknown-flag"));
    }

    #[test]
    fn equation_batch_report_builds_deterministic_status_summary() {
        let root = temp_repo("summary");
        write_file(
            &root,
            "equation-batches/z.tsv",
            &format!(
                "{HEADER}\n{}\n{}\n",
                row(
                    "formula_vault.m00.angle.zeta",
                    "research_required",
                    "tolerance",
                    "validation/cards/z.yaml",
                ),
                row(
                    "formula_vault.a4.atmosphere.alpha",
                    "implementation_verified",
                    "exact",
                    "validation/cards/a.yaml",
                )
            ),
        );

        let report = build_status_report(&root).expect("report builds");

        assert!(report.ok);
        assert_eq!(report.command, "equation-batch report");
        assert_eq!(report.schema_version, REPORT_SCHEMA_VERSION);
        assert_eq!(report.manifest_count, 1);
        assert_eq!(report.row_count, 2);
        assert_eq!(
            report.counts_by_status.get("implementation_verified"),
            Some(&1)
        );
        assert_eq!(report.counts_by_status.get("research_required"), Some(&1));
        assert_eq!(report.counts_by_family.get("a4"), Some(&1));
        assert_eq!(report.counts_by_family.get("m00"), Some(&1));
        assert_eq!(report.counts_by_batch.get("m00-test"), Some(&2));
        assert_eq!(report.counts_by_test_strategy.get("exact"), Some(&1));
        assert_eq!(report.counts_by_static_symbol_status.get("ok"), Some(&2));
        assert_eq!(report.counts_by_static_path_status.get("missing"), Some(&1));
        assert_eq!(report.blocked_execution_count, 1);
        assert_eq!(report.missing_metadata_path_count, 1);
        assert_eq!(
            report.manifests[0].rows[0].formula_id,
            "formula_vault.a4.atmosphere.alpha"
        );
        assert_eq!(
            report.manifests[0].rows[1].formula_id,
            "formula_vault.m00.angle.zeta"
        );
        assert!(report
            .non_claims
            .iter()
            .any(|claim| claim.contains("research")));
        assert!(report.safety_notice.contains("not certified"));
    }

    #[test]
    fn equation_batch_report_json_is_stable_pretty_and_excludes_test_expression() {
        let root = temp_repo("json");
        write_file(
            &root,
            "equation-batches/a.tsv",
            &format!(
                "{HEADER}\n{}\n",
                row(
                    "formula_vault.m00.angle.alpha",
                    "research_required",
                    "exact",
                    "validation/cards/a.yaml",
                )
            ),
        );
        let report = build_status_report(&root).expect("report builds");
        let first = render_status_report_json(&report);
        let second = render_status_report_json(&report);

        assert_eq!(first, second);
        assert!(first.contains("\"schema_version\": \"aerocodex.equation_batch.status_report.v1\""));
        assert!(first.contains("\"row_count\": 1"));
        assert!(first.contains("\"research_required\": 1"));
        assert!(first.contains("\"non_claims\""));
        assert!(!first.contains("test_expression"));
    }
}
