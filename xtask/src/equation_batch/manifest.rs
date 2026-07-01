use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

pub const SCHEMA_VERSION: &str = "aerocodex.equation_batch.v1";
pub const EQUATION_BATCH_HEADER: &str = "schema_version\tbatch_id\tformula_id\tpackage\tcrate_name\truntime_symbol\toutput_variable\tcontract_path\tvalidation_card_path\tsource_seed_path\tvalidation_status\ttest_strategy\ttest_expression";

const FIELD_NAMES: [&str; 13] = [
    "schema_version",
    "batch_id",
    "formula_id",
    "package",
    "crate_name",
    "runtime_symbol",
    "output_variable",
    "contract_path",
    "validation_card_path",
    "source_seed_path",
    "validation_status",
    "test_strategy",
    "test_expression",
];

pub const SUPPORTED_VALIDATION_STATUSES: &[&str] = &[
    "research_required",
    "equation_traceable",
    "implementation_verified",
    "reference_validated",
    "experiment_validated",
];

pub const SUPPORTED_TEST_STRATEGIES: &[&str] = &["exact", "tolerance", "invariant"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquationBatchRow {
    pub line_number: usize,
    pub schema_version: String,
    pub batch_id: String,
    pub formula_id: String,
    pub package: String,
    pub crate_name: String,
    pub runtime_symbol: String,
    pub output_variable: String,
    pub contract_path: String,
    pub validation_card_path: String,
    pub source_seed_path: String,
    pub validation_status: String,
    pub test_strategy: String,
    pub test_expression: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquationBatchManifest {
    pub path: PathBuf,
    pub batch_id: String,
    pub rows: Vec<EquationBatchRow>,
    pub row_count: usize,
    pub validation_status_counts: BTreeMap<String, usize>,
    pub test_strategy_counts: BTreeMap<String, usize>,
}

pub fn parse_equation_batch_manifest(
    path: impl AsRef<Path>,
    text: &str,
) -> Result<EquationBatchManifest, String> {
    let path = path.as_ref().to_path_buf();
    let mut lines = text
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty());

    let (header_index, header) = lines
        .next()
        .ok_or_else(|| line_error(&path, 1, "is empty; expected equation-batch header"))?;
    let header_line = header_index + 1;
    if header != EQUATION_BATCH_HEADER {
        return Err(line_error(
            &path,
            header_line,
            "has an unsupported equation-batch header",
        ));
    }

    let mut batch_id: Option<String> = None;
    let mut rows = Vec::new();
    let mut validation_status_counts = BTreeMap::new();
    let mut test_strategy_counts = BTreeMap::new();

    for (line_index, line) in lines {
        let line_number = line_index + 1;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != FIELD_NAMES.len() {
            return Err(line_error(
                &path,
                line_number,
                format!(
                    "has {} fields; expected {}",
                    fields.len(),
                    FIELD_NAMES.len()
                ),
            ));
        }

        for (field_name, value) in FIELD_NAMES.iter().zip(fields.iter()) {
            if value.trim().is_empty() {
                return Err(line_error(
                    &path,
                    line_number,
                    format!("{field_name} must be non-empty"),
                ));
            }
        }

        if fields[0] != SCHEMA_VERSION {
            return Err(line_error(
                &path,
                line_number,
                format!(
                    "schema_version must be {SCHEMA_VERSION}; found {}",
                    fields[0]
                ),
            ));
        }

        if !SUPPORTED_VALIDATION_STATUSES.contains(&fields[10]) {
            return Err(line_error(
                &path,
                line_number,
                format!(
                    "validation_status `{}` is not in the supported vocabulary: {}",
                    fields[10],
                    SUPPORTED_VALIDATION_STATUSES.join(", ")
                ),
            ));
        }

        if !SUPPORTED_TEST_STRATEGIES.contains(&fields[11]) {
            return Err(line_error(
                &path,
                line_number,
                format!(
                    "test_strategy `{}` is not supported; expected one of: {}",
                    fields[11],
                    SUPPORTED_TEST_STRATEGIES.join(", ")
                ),
            ));
        }

        for (field_name, value) in [
            ("contract_path", fields[7]),
            ("validation_card_path", fields[8]),
            ("source_seed_path", fields[9]),
        ] {
            validate_repository_relative_path(&path, line_number, field_name, value)?;
        }

        match &batch_id {
            Some(expected) if expected != fields[1] => {
                return Err(line_error(
                    &path,
                    line_number,
                    format!("batch_id must match manifest batch_id `{expected}`"),
                ));
            }
            None => batch_id = Some(fields[1].to_string()),
            _ => {}
        }

        *validation_status_counts
            .entry(fields[10].to_string())
            .or_insert(0) += 1;
        *test_strategy_counts
            .entry(fields[11].to_string())
            .or_insert(0) += 1;

        rows.push(EquationBatchRow {
            line_number,
            schema_version: fields[0].to_string(),
            batch_id: fields[1].to_string(),
            formula_id: fields[2].to_string(),
            package: fields[3].to_string(),
            crate_name: fields[4].to_string(),
            runtime_symbol: fields[5].to_string(),
            output_variable: fields[6].to_string(),
            contract_path: fields[7].to_string(),
            validation_card_path: fields[8].to_string(),
            source_seed_path: fields[9].to_string(),
            validation_status: fields[10].to_string(),
            test_strategy: fields[11].to_string(),
            test_expression: fields[12].to_string(),
        });
    }

    let batch_id = batch_id.ok_or_else(|| {
        line_error(
            &path,
            header_line,
            "contains no equation rows after the header",
        )
    })?;
    let row_count = rows.len();

    Ok(EquationBatchManifest {
        path,
        batch_id,
        rows,
        row_count,
        validation_status_counts,
        test_strategy_counts,
    })
}

fn validate_repository_relative_path(
    path: &Path,
    line_number: usize,
    field_name: &str,
    value: &str,
) -> Result<(), String> {
    let candidate = Path::new(value);
    if candidate.is_absolute() || has_windows_absolute_prefix(value) || value.starts_with('\\') {
        return Err(line_error(
            path,
            line_number,
            format!("{field_name} must be a repository-relative path"),
        ));
    }

    if candidate
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
        || value.split(['/', '\\']).any(|component| component == "..")
    {
        return Err(line_error(
            path,
            line_number,
            format!("{field_name} must not contain parent traversal"),
        ));
    }

    Ok(())
}

fn has_windows_absolute_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

fn line_error(path: &Path, line_number: usize, message: impl std::fmt::Display) -> String {
    format!("{} line {} {message}", path.display(), line_number)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::parse_equation_batch_manifest;

    const HEADER: &str = "schema_version\tbatch_id\tformula_id\tpackage\tcrate_name\truntime_symbol\toutput_variable\tcontract_path\tvalidation_card_path\tsource_seed_path\tvalidation_status\ttest_strategy\ttest_expression";

    fn row(overrides: &[(&str, &str)]) -> String {
        let mut fields = vec![
            ("schema_version", "aerocodex.equation_batch.v1"),
            ("batch_id", "m00-minimal"),
            ("formula_id", "formula_vault.m00.angle.deg_to_rad"),
            ("package", "aero-codex-astrodynamics"),
            ("crate_name", "aero_codex_astrodynamics"),
            ("runtime_symbol", "m00_angle_deg_to_rad"),
            ("output_variable", "radians"),
            (
                "contract_path",
                "formula-vault/contracts/m00_angle_contract.yaml",
            ),
            (
                "validation_card_path",
                "validation/cards/validation_formula_vault_m00_angle.yaml",
            ),
            (
                "source_seed_path",
                "validation/source_registry/source_formula_vault_m00_angle.yaml",
            ),
            ("validation_status", "research_required"),
            ("test_strategy", "exact"),
            ("test_expression", "value == 0.0"),
        ];
        for (name, value) in overrides {
            let (_, slot) = fields
                .iter_mut()
                .find(|(field_name, _)| field_name == name)
                .expect("test override field exists");
            *slot = value;
        }
        fields
            .into_iter()
            .map(|(_, value)| value)
            .collect::<Vec<_>>()
            .join("\t")
    }

    fn manifest_text(overrides: &[(&str, &str)]) -> String {
        format!("{}\n{}\n", HEADER, row(overrides))
    }

    fn parse(text: &str) -> Result<super::EquationBatchManifest, String> {
        parse_equation_batch_manifest(
            Path::new("xtask/tests/fixtures/equation_batches/minimal.tsv"),
            text,
        )
    }

    #[test]
    fn equation_batch_good_minimal_row_parses_with_summary_counts() {
        let manifest = parse(&manifest_text(&[])).expect("minimal manifest parses");

        assert_eq!(
            manifest.path,
            Path::new("xtask/tests/fixtures/equation_batches/minimal.tsv")
        );
        assert_eq!(manifest.batch_id, "m00-minimal");
        assert_eq!(manifest.row_count, 1);
        assert_eq!(manifest.rows.len(), 1);
        assert_eq!(
            manifest.rows[0].formula_id,
            "formula_vault.m00.angle.deg_to_rad"
        );
        assert_eq!(
            manifest.validation_status_counts.get("research_required"),
            Some(&1)
        );
        assert_eq!(manifest.test_strategy_counts.get("exact"), Some(&1));
    }

    #[test]
    fn equation_batch_rejects_bad_header_with_path_and_line() {
        let err = parse("bad\theader\n").expect_err("bad header is rejected");

        assert!(err.contains("xtask/tests/fixtures/equation_batches/minimal.tsv"));
        assert!(err.contains("line 1"));
        assert!(err.contains("header"));
    }

    #[test]
    fn equation_batch_rejects_bad_field_count_with_path_and_line() {
        let err =
            parse(&format!("{}\nonly\ttwo\n", HEADER)).expect_err("bad field count is rejected");

        assert!(err.contains("xtask/tests/fixtures/equation_batches/minimal.tsv"));
        assert!(err.contains("line 2"));
        assert!(err.contains("fields"));
    }

    #[test]
    fn equation_batch_rejects_empty_formula_id() {
        let err =
            parse(&manifest_text(&[("formula_id", "")])).expect_err("empty formula_id rejected");

        assert!(err.contains("line 2"));
        assert!(err.contains("formula_id"));
        assert!(err.contains("non-empty"));
    }

    #[test]
    fn equation_batch_rejects_absolute_path() {
        let err = parse(&manifest_text(&[("contract_path", "/tmp/contract.yaml")]))
            .expect_err("absolute paths rejected");

        assert!(err.contains("line 2"));
        assert!(err.contains("contract_path"));
        assert!(err.contains("repository-relative"));
    }

    #[test]
    fn equation_batch_rejects_parent_traversal_path() {
        let err = parse(&manifest_text(&[("source_seed_path", "../source.yaml")]))
            .expect_err("parent traversal rejected");

        assert!(err.contains("line 2"));
        assert!(err.contains("source_seed_path"));
        assert!(err.contains("parent traversal"));
    }

    #[test]
    fn equation_batch_rejects_unsupported_schema() {
        let err = parse(&manifest_text(&[(
            "schema_version",
            "aerocodex.equation_batch.v0",
        )]))
        .expect_err("unsupported schema rejected");

        assert!(err.contains("line 2"));
        assert!(err.contains("schema_version"));
        assert!(err.contains("aerocodex.equation_batch.v1"));
    }

    #[test]
    fn equation_batch_rejects_unsupported_test_strategy() {
        let err = parse(&manifest_text(&[("test_strategy", "approx")]))
            .expect_err("unsupported strategy rejected");

        assert!(err.contains("line 2"));
        assert!(err.contains("test_strategy"));
        assert!(err.contains("exact, tolerance, invariant"));
    }

    #[test]
    fn equation_batch_rejects_unsupported_validation_status() {
        let err = parse(&manifest_text(&[("validation_status", "promoted")]))
            .expect_err("unsupported validation status rejected");

        assert!(err.contains("line 2"));
        assert!(err.contains("validation_status"));
        assert!(err.contains("supported vocabulary"));
    }
}
