use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use super::{
    build_formula_registry, render_formula_registry_json,
    rust::{self, JsonValue, GENERATED_RUST_PATH},
    GENERATED_BY, GENERATOR_VERSION, REGISTRY_JSON_PATH, REGISTRY_SCHEMA_VERSION,
    REGISTRY_SHA256_PATH,
};

const EXPECTED_FORMULA_COUNT: usize = 152;
const REPAIR_JSON_COMMAND: &str =
    "cargo run -p xtask -- formula-registry generate --out generated/formula_registry.json";
const REPAIR_RUST_COMMAND: &str =
    "cargo run -p xtask -- formula-registry generate-rust --out generated/rust/formula_registry.rs";
const CHECK_SAFETY_NOTICE: &str = "formula-registry check is a software consistency gate, not formula validation, status promotion, certification, or formula execution.";

const REQUIRED_TOP_LEVEL_FIELDS: &[&str] = &[
    "schema_version",
    "generator_version",
    "generated_by",
    "source_hash",
    "formula_count",
    "non_claims",
    "formulas",
];

const REQUIRED_FORMULA_FIELDS: &[&str] = &[
    "formula_id",
    "legacy_formula_id",
    "aliases",
    "name",
    "family",
    "batch_id",
    "status",
    "quarantine_state",
    "execution_policy",
    "source_trace",
    "inputs",
    "outputs",
    "units",
    "domain_constraints",
    "implementation_path",
    "runtime_symbol",
    "test_vectors",
    "warnings",
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CheckOptions;

impl CheckOptions {
    pub fn parse_args(args: &[&str]) -> Result<Self, String> {
        if args.is_empty() {
            return Ok(Self);
        }

        let unexpected = args[0];
        if unexpected.starts_with("--") {
            Err(format!(
                "usage error: unknown formula-registry check flag `{unexpected}`; usage: cargo run -p xtask -- formula-registry check"
            ))
        } else {
            Err(format!(
                "usage error: unexpected formula-registry check argument `{unexpected}`; usage: cargo run -p xtask -- formula-registry check"
            ))
        }
    }
}

pub fn run_check_command(root: &Path, _options: &CheckOptions) -> Result<(), String> {
    let json_path = root.join(REGISTRY_JSON_PATH);
    let sha_path = root.join(REGISTRY_SHA256_PATH);
    let rust_path = root.join(GENERATED_RUST_PATH);
    let mut failures = Vec::new();
    let temp_dir = match TemporaryCheckDir::create() {
        Ok(temp_dir) => temp_dir,
        Err(error) => return Err(failure_report(&[error])),
    };

    let mut checked_json_text = None;
    match fs::read(&json_path) {
        Ok(bytes) => {
            let expected_sidecar = expected_sha256_sidecar(&bytes);
            match fs::read_to_string(&sha_path) {
                Ok(sidecar) => {
                    if crate::checksums::canonical_text(&sidecar) != expected_sidecar {
                        failures.push(format!(
                            "{REGISTRY_SHA256_PATH} is stale: expected `{}`, found `{}`",
                            expected_sidecar.trim_end(),
                            sidecar.trim_end()
                        ));
                    }
                }
                Err(error) => failures.push(format!(
                    "{REGISTRY_SHA256_PATH} is missing or unreadable: {error}"
                )),
            }

            match String::from_utf8(bytes) {
                Ok(text) => {
                    if let Err(error) =
                        validate_registry_json_structure(&text, EXPECTED_FORMULA_COUNT)
                    {
                        failures.push(format!(
                            "{REGISTRY_JSON_PATH} is structurally invalid: {error}"
                        ));
                    }
                    checked_json_text = Some(crate::checksums::canonical_text(&text).into_owned());
                }
                Err(error) => failures.push(format!(
                    "{REGISTRY_JSON_PATH} is not valid UTF-8 registry JSON: {error}"
                )),
            }
        }
        Err(error) => failures.push(format!(
            "{REGISTRY_JSON_PATH} is missing or unreadable: {error}"
        )),
    }

    let expected_json = match build_formula_registry(root) {
        Ok(registry) => {
            if registry.formula_count != EXPECTED_FORMULA_COUNT {
                failures.push(format!(
                    "regenerated registry formula_count={} but expected {EXPECTED_FORMULA_COUNT}",
                    registry.formula_count
                ));
            }
            let rendered = render_formula_registry_json(&registry);
            if let Err(error) = validate_registry_json_structure(&rendered, EXPECTED_FORMULA_COUNT)
            {
                failures.push(format!(
                    "regenerated {REGISTRY_JSON_PATH} is structurally invalid: {error}"
                ));
            }
            match temp_dir.write_and_read("formula_registry.json", &rendered) {
                Ok(staged) => Some(staged),
                Err(error) => {
                    failures.push(format!(
                        "could not stage regenerated {REGISTRY_JSON_PATH} in a temporary file: {error}"
                    ));
                    Some(rendered)
                }
            }
        }
        Err(error) => {
            failures.push(format!(
                "could not regenerate {REGISTRY_JSON_PATH} from current sources: {error}"
            ));
            None
        }
    };

    if let (Some(checked), Some(expected)) = (&checked_json_text, &expected_json) {
        if checked != expected {
            failures.push(format!(
                "{REGISTRY_JSON_PATH} is stale; regenerated bytes differ from checked-in artifact"
            ));
        }
    }

    let expected_rust = match &expected_json {
        Some(json) => match rust::render_registry_module_from_json_text(json) {
            Ok(rendered) => match temp_dir.write_and_read("rust/formula_registry.rs", &rendered) {
                Ok(staged) => Some(staged),
                Err(error) => {
                    failures.push(format!(
                        "could not stage regenerated {GENERATED_RUST_PATH} in a temporary file: {error}"
                    ));
                    Some(rendered)
                }
            },
            Err(error) => {
                failures.push(format!(
                    "could not regenerate {GENERATED_RUST_PATH} from registry JSON: {error}"
                ));
                None
            }
        },
        None => None,
    };

    match fs::read_to_string(&rust_path) {
        Ok(checked_rust) => {
            if let Err(error) = validate_generated_rust_structure(&checked_rust) {
                failures.push(format!(
                    "{GENERATED_RUST_PATH} is structurally invalid: {error}"
                ));
            }
            if let Some(expected) = &expected_rust {
                if crate::checksums::canonical_text(&checked_rust) != expected.as_str() {
                    failures.push(format!(
                        "{GENERATED_RUST_PATH} is stale; regenerated bytes differ from checked-in artifact"
                    ));
                }
            }
        }
        Err(error) => failures.push(format!(
            "{GENERATED_RUST_PATH} is missing or unreadable: {error}"
        )),
    }

    if failures.is_empty() {
        println!(
            "formula_registry_check=PASS json={} sha256={} rust={} formula_count={}",
            REGISTRY_JSON_PATH, REGISTRY_SHA256_PATH, GENERATED_RUST_PATH, EXPECTED_FORMULA_COUNT
        );
        println!("formula_registry_check_notice={CHECK_SAFETY_NOTICE}");
        Ok(())
    } else {
        Err(failure_report(&failures))
    }
}

fn expected_sha256_sidecar(json_bytes: &[u8]) -> String {
    let (canonical, _) = crate::checksums::canonical_checksum_bytes(json_bytes);
    format!(
        "{}  {}\n",
        crate::equation_batch::generate::sha256_hex(canonical.as_ref()),
        REGISTRY_JSON_PATH
    )
}

struct TemporaryCheckDir {
    path: PathBuf,
}

impl TemporaryCheckDir {
    fn create() -> Result<Self, String> {
        let mut path = std::env::temp_dir();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("system clock before UNIX_EPOCH: {error}"))?
            .as_nanos();
        path.push(format!(
            "aerocodex_formula_registry_check_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&path).map_err(|error| {
            format!(
                "cannot create temporary check directory {}: {error}",
                path.display()
            )
        })?;
        Ok(Self { path })
    }

    fn write_and_read(&self, relative: &str, contents: &str) -> Result<String, String> {
        let path = self.path.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "cannot create temporary check directory {}: {error}",
                    parent.display()
                )
            })?;
        }
        fs::write(&path, contents).map_err(|error| {
            format!(
                "cannot write temporary check file {}: {error}",
                path.display()
            )
        })?;
        fs::read_to_string(&path).map_err(|error| {
            format!(
                "cannot read temporary check file {}: {error}",
                path.display()
            )
        })
    }
}

impl Drop for TemporaryCheckDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn validate_registry_json_structure(
    json: &str,
    expected_formula_count: usize,
) -> Result<(), String> {
    let value = rust::parse_json_value(json).map_err(|error| format!("invalid JSON: {error}"))?;
    let root = json_object(&value, "formula registry root")?;

    for field in REQUIRED_TOP_LEVEL_FIELDS {
        require_field(root, field, "formula registry root")?;
    }

    let schema_version = string_field(root, "schema_version", "formula registry root")?;
    if schema_version != REGISTRY_SCHEMA_VERSION {
        return Err(format!(
            "schema_version must be `{REGISTRY_SCHEMA_VERSION}`, found `{schema_version}`"
        ));
    }

    let generator_version = string_field(root, "generator_version", "formula registry root")?;
    if generator_version != GENERATOR_VERSION {
        return Err(format!(
            "generator_version must be `{GENERATOR_VERSION}`, found `{generator_version}`"
        ));
    }

    let generated_by = string_field(root, "generated_by", "formula registry root")?;
    if generated_by != GENERATED_BY {
        return Err(format!(
            "generated_by must be `{GENERATED_BY}`, found `{generated_by}`"
        ));
    }

    let source_hash = string_field(root, "source_hash", "formula registry root")?;
    if !source_hash.starts_with("sha256:") || source_hash.len() <= "sha256:".len() {
        return Err("source_hash must be a sha256: digest marker".to_string());
    }

    let formula_count = usize_field(root, "formula_count", "formula registry root")?;
    if formula_count != expected_formula_count {
        return Err(format!(
            "formula_count must be {expected_formula_count}, found {formula_count}"
        ));
    }

    let non_claims = array_field(root, "non_claims", "formula registry root")?;
    for (index, value) in non_claims.iter().enumerate() {
        if !matches!(value, JsonValue::String(_)) {
            return Err(format!(
                "formula registry root.non_claims[{index}] must be a string"
            ));
        }
    }

    let formulas = array_field(root, "formulas", "formula registry root")?;
    if formulas.len() != formula_count {
        return Err(format!(
            "formula_count={formula_count} but formulas length is {}",
            formulas.len()
        ));
    }

    let mut seen_formula_ids = BTreeSet::new();
    let mut seen_aliases = BTreeMap::<String, String>::new();
    let mut previous_formula_id: Option<String> = None;

    for (index, formula_value) in formulas.iter().enumerate() {
        let context = format!("formulas[{index}]");
        let formula = json_object(formula_value, &context)?;
        for field in REQUIRED_FORMULA_FIELDS {
            require_field(formula, field, &context)?;
        }

        let formula_id = string_field(formula, "formula_id", &context)?;
        if let Some(previous) = &previous_formula_id {
            if previous.as_str() > formula_id {
                return Err(format!(
                    "formula IDs must be sorted lexicographically: `{formula_id}` appears after `{previous}`"
                ));
            }
        }
        previous_formula_id = Some(formula_id.to_string());
        if !seen_formula_ids.insert(formula_id.to_string()) {
            return Err(format!("duplicate formula_id `{formula_id}`"));
        }

        optional_string_field(formula, "legacy_formula_id", &context)?;
        let aliases = string_array_field(formula, "aliases", &context)?;
        for alias in aliases {
            if let Some(existing) = seen_aliases.insert(alias.clone(), formula_id.to_string()) {
                return Err(format!(
                    "duplicate alias `{alias}` maps to both `{existing}` and `{formula_id}`"
                ));
            }
        }

        string_field(formula, "name", &context)?;
        string_field(formula, "family", &context)?;
        optional_string_field(formula, "batch_id", &context)?;
        let status = string_field(formula, "status", &context)?;
        string_field(formula, "quarantine_state", &context)?;
        let execution_policy = string_field(formula, "execution_policy", &context)?;
        if status == "research_required" && execution_policy != "blocked" {
            return Err(format!(
                "{context} status research_required must use execution_policy blocked, found `{execution_policy}`"
            ));
        }
        object_field(formula, "source_trace", &context)?;
        array_field(formula, "inputs", &context)?;
        array_field(formula, "outputs", &context)?;
        optional_object_field(formula, "units", &context)?;
        array_field(formula, "domain_constraints", &context)?;
        object_field(formula, "implementation_path", &context)?;
        optional_string_field(formula, "runtime_symbol", &context)?;
        array_field(formula, "test_vectors", &context)?;
        string_array_field(formula, "warnings", &context)?;
    }

    Ok(())
}

fn validate_generated_rust_structure(rust: &str) -> Result<(), String> {
    for marker in [
        "FORMULA_COUNT",
        "FORMULA_REGISTRY",
        "find_by_formula_id",
        "find_by_alias",
        "152",
    ] {
        if !rust.contains(marker) {
            return Err(format!("missing marker `{marker}`"));
        }
    }
    Ok(())
}

fn failure_report(failures: &[String]) -> String {
    let mut report = String::new();
    report.push_str(
        "formula-registry check failed: generated registry artifacts are stale, missing, or structurally invalid.\n",
    );
    report.push_str("This is a software consistency gate, not formula validation, status promotion, certification, or formula execution.\n\n");
    report.push_str("Failures:\n");
    for failure in failures {
        report.push_str("- ");
        report.push_str(failure);
        report.push('\n');
    }
    report.push_str("\nRepair commands:\n");
    report.push_str("  ");
    report.push_str(REPAIR_JSON_COMMAND);
    report.push('\n');
    report.push_str("  ");
    report.push_str(REPAIR_RUST_COMMAND);
    report.push('\n');
    report.push_str("\nSafety notice: ");
    report.push_str(CHECK_SAFETY_NOTICE);
    report.push('\n');
    report
}

fn require_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<&'a JsonValue, String> {
    object
        .get(key)
        .ok_or_else(|| format!("{context}.{key} is missing"))
}

fn string_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<&'a str, String> {
    match require_field(object, key, context)? {
        JsonValue::String(value) => Ok(value.as_str()),
        _ => Err(format!("{context}.{key} must be a string")),
    }
}

fn optional_string_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<(), String> {
    match require_field(object, key, context)? {
        JsonValue::String(_) | JsonValue::Null => Ok(()),
        _ => Err(format!("{context}.{key} must be a string or null")),
    }
}

fn usize_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<usize, String> {
    match require_field(object, key, context)? {
        JsonValue::Number(value) => value
            .parse::<usize>()
            .map_err(|error| format!("{context}.{key} must be a non-negative integer: {error}")),
        _ => Err(format!("{context}.{key} must be a number")),
    }
}

fn array_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<&'a [JsonValue], String> {
    match require_field(object, key, context)? {
        JsonValue::Array(values) => Ok(values),
        _ => Err(format!("{context}.{key} must be an array")),
    }
}

fn string_array_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<Vec<String>, String> {
    array_field(object, key, context)?
        .iter()
        .enumerate()
        .map(|(index, value)| match value {
            JsonValue::String(item) => Ok(item.clone()),
            _ => Err(format!("{context}.{key}[{index}] must be a string")),
        })
        .collect()
}

fn object_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<&'a BTreeMap<String, JsonValue>, String> {
    match require_field(object, key, context)? {
        JsonValue::Object(value) => Ok(value),
        _ => Err(format!("{context}.{key} must be an object")),
    }
}

fn optional_object_field(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
    context: &str,
) -> Result<(), String> {
    match require_field(object, key, context)? {
        JsonValue::Object(_) | JsonValue::Null => Ok(()),
        _ => Err(format!("{context}.{key} must be an object or null")),
    }
}

fn json_object<'a>(
    value: &'a JsonValue,
    context: &str,
) -> Result<&'a BTreeMap<String, JsonValue>, String> {
    match value {
        JsonValue::Object(object) => Ok(object),
        _ => Err(format!("{context} must be a JSON object")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_accepts_no_flags_and_rejects_unknown_flags() {
        CheckOptions::parse_args(&[]).expect("check takes no arguments");

        let error =
            CheckOptions::parse_args(&["--unknown-flag"]).expect_err("unknown flags fail closed");
        assert!(error.contains("usage error"));
        assert!(error.contains("--unknown-flag"));
    }

    #[test]
    fn structural_validation_requires_registry_v1_fields_and_expected_facts() {
        validate_registry_json_structure(fixture_registry_json(), 2)
            .expect("fixture registry is structurally valid");

        let missing_source_hash =
            fixture_registry_json().replace("  \"source_hash\": \"sha256:test\",\n", "");
        let error = validate_registry_json_structure(&missing_source_hash, 2)
            .expect_err("missing top-level required field fails");
        assert!(error.contains("source_hash"));

        let wrong_count =
            fixture_registry_json().replace("\"formula_count\": 2", "\"formula_count\": 151");
        let error = validate_registry_json_structure(&wrong_count, 2)
            .expect_err("unexpected formula_count fails");
        assert!(error.contains("formula_count"));

        let bad_policy = fixture_registry_json().replace(
            "\"execution_policy\": \"blocked\"",
            "\"execution_policy\": \"normal_research\"",
        );
        let error = validate_registry_json_structure(&bad_policy, 2)
            .expect_err("research_required must remain blocked");
        assert!(error.contains("research_required"));
        assert!(error.contains("blocked"));
    }

    #[test]
    fn structural_validation_fails_on_duplicate_aliases() {
        let duplicate_alias = fixture_registry_json().replace(
            "\"formula_vault.m00.canonical.time_unit_from_mu_du\"",
            "\"formula_vault.m00.angle.deg2rad\"",
        );
        let error = validate_registry_json_structure(&duplicate_alias, 2)
            .expect_err("duplicate alias fails closed");
        assert!(error.contains("duplicate alias"));
    }

    #[test]
    fn failure_report_names_repair_commands_and_safety_notice() {
        let report = failure_report(&["generated/formula_registry.json is stale".to_string()]);
        assert!(report.contains(REPAIR_JSON_COMMAND));
        assert!(report.contains(REPAIR_RUST_COMMAND));
        assert!(report.contains("software consistency gate"));
        assert!(report.contains("not formula validation"));
    }

    fn fixture_registry_json() -> &'static str {
        r#"{
  "schema_version": "aerocodex.formula_registry.v1",
  "generator_version": "xtask-formula-registry-v1",
  "generated_by": "cargo run -p xtask -- formula-registry generate",
  "source_hash": "sha256:test",
  "formula_count": 2,
  "non_claims": [
    "metadata only"
  ],
  "formulas": [
    {
      "formula_id": "m00.angle.deg_to_rad",
      "legacy_formula_id": "formula_vault.m00.angle.deg2rad",
      "aliases": [
        "formula_vault.m00.angle.deg2rad"
      ],
      "name": "Degrees to radians",
      "summary": "Inventory metadata only.",
      "family": "m00.angle",
      "batch_id": "m00-angle-vector",
      "status": "research_required",
      "quarantine_state": "below_execution_threshold",
      "execution_policy": "blocked",
      "source_trace": {
        "manifest_path": "equation-batches/m00-angle-vector.tsv"
      },
      "inputs": [],
      "outputs": [],
      "units": null,
      "domain_constraints": [],
      "implementation_path": {
        "package": "aero-codex-astrodynamics"
      },
      "runtime_symbol": "m00_degrees_to_radians",
      "test_vectors": [],
      "warnings": []
    },
    {
      "formula_id": "m00.canonical.time_unit_from_mu_du",
      "legacy_formula_id": "formula_vault.m00.canonical.time_unit_from_mu_du",
      "aliases": [
        "formula_vault.m00.canonical.time_unit_from_mu_du"
      ],
      "name": "Time Unit From Mu Du",
      "summary": "Inventory metadata only.",
      "family": "m00.canonical",
      "batch_id": "m00-canonical-units",
      "status": "research_required",
      "quarantine_state": "below_execution_threshold",
      "execution_policy": "blocked",
      "source_trace": {
        "manifest_path": "equation-batches/m00-canonical-units.tsv"
      },
      "inputs": [],
      "outputs": [],
      "units": null,
      "domain_constraints": [],
      "implementation_path": {
        "package": "aero-codex-astrodynamics"
      },
      "runtime_symbol": "m00_canonical_time_unit_from_mu_du",
      "test_vectors": [],
      "warnings": []
    }
  ]
}
"#
    }
}
