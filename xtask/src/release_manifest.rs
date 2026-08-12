use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
};

pub const RELEASE_MANIFEST_PATH: &str = "docs/release/v0.1.0-alpha.1.toml";
const RELEASE_SCHEMA_VERSION: &str = "aerocodex.release_manifest.v1";
const RELEASE_VERSION: &str = "0.1.0-alpha.1";
const RELEASE_TIER: &str = "research_software_alpha";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub schema_version: String,
    pub release_version: String,
    pub release_tier: String,
    pub source_revision_policy: String,
    pub base_commit: String,
    pub validation_status: String,
    pub execution_policy: String,
    pub public_executable: bool,
    pub validation_record: String,
    pub documentation: String,
    pub registry_formula_count: usize,
    pub cli_dispatch_formula_count: usize,
    pub public_executable_formula_count: usize,
    pub formulas: Vec<ReleaseFormula>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseFormula {
    pub identifier: String,
    pub runtime_symbol: Option<String>,
    pub validation_status: String,
    pub execution_policy: String,
    pub public_executable: bool,
    pub validation_record: String,
    pub documentation: String,
}

pub fn verify_release_manifest(root: &Path) -> Result<(), String> {
    let path = root.join(RELEASE_MANIFEST_PATH);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {RELEASE_MANIFEST_PATH}: {error}"))?;
    let manifest = parse_release_manifest(&text)?;

    if manifest.schema_version != RELEASE_SCHEMA_VERSION {
        return Err(format!(
            "{RELEASE_MANIFEST_PATH} has schema_version `{}`, expected `{RELEASE_SCHEMA_VERSION}`",
            manifest.schema_version
        ));
    }
    if manifest.release_version != RELEASE_VERSION {
        return Err(format!(
            "{RELEASE_MANIFEST_PATH} has release_version `{}`, expected `{RELEASE_VERSION}`",
            manifest.release_version
        ));
    }
    if manifest.release_tier != RELEASE_TIER {
        return Err(format!(
            "{RELEASE_MANIFEST_PATH} has release_tier `{}`, expected `{RELEASE_TIER}`",
            manifest.release_tier
        ));
    }

    for reference in [
        manifest.validation_record.as_str(),
        manifest.documentation.as_str(),
    ]
    .into_iter()
    .chain(manifest.formulas.iter().flat_map(|formula| {
        [
            formula.validation_record.as_str(),
            formula.documentation.as_str(),
        ]
    })) {
        require_existing_repository_file(root, reference)?;
    }

    let registry = crate::formula_registry::build_formula_registry(root)?;
    if manifest.registry_formula_count != registry.formula_count {
        return Err(format!(
            "release manifest registry_formula_count={} does not match generated registry source count {}",
            manifest.registry_formula_count, registry.formula_count
        ));
    }

    for formula in &manifest.formulas {
        let registry_formula = registry
            .formulas
            .iter()
            .find(|entry| entry.formula_id == formula.identifier)
            .ok_or_else(|| {
                format!(
                    "release formula `{}` is absent from the governed formula registry",
                    formula.identifier
                )
            })?;
        if registry_formula.runtime_symbol.as_deref() != formula.runtime_symbol.as_deref() {
            return Err(format!(
                "release formula `{}` runtime_symbol does not match the governed formula registry",
                formula.identifier
            ));
        }
        if registry_formula.status != formula.validation_status {
            return Err(format!(
                "release formula `{}` validation_status `{}` does not match governed status `{}`",
                formula.identifier, formula.validation_status, registry_formula.status
            ));
        }
        if registry_formula.execution_policy != formula.execution_policy {
            return Err(format!(
                "release formula `{}` execution_policy `{}` does not match governed policy `{}`",
                formula.identifier, formula.execution_policy, registry_formula.execution_policy
            ));
        }
    }

    println!(
        "verified release manifest: version={}; tier={}; formulas={}; public_executable_formulas={}; execution_policy={}",
        manifest.release_version,
        manifest.release_tier,
        manifest.formulas.len(),
        manifest.public_executable_formula_count,
        manifest.execution_policy
    );
    Ok(())
}

pub fn parse_release_manifest(text: &str) -> Result<ReleaseManifest, String> {
    let mut release = BTreeMap::new();
    let mut formula_tables = Vec::new();
    let mut current_formula: Option<BTreeMap<String, String>> = None;

    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "[[formulas]]" {
            if let Some(formula) = current_formula.take() {
                formula_tables.push(formula);
            }
            current_formula = Some(BTreeMap::new());
            continue;
        }
        if line.starts_with('[') {
            return Err(format!(
                "release manifest line {line_number} has unsupported table `{line}`"
            ));
        }

        let (key, value) = line.split_once('=').ok_or_else(|| {
            format!("release manifest line {line_number} is not a key/value entry")
        })?;
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() {
            return Err(format!(
                "release manifest line {line_number} has an empty key or value"
            ));
        }
        let table = current_formula.as_mut().unwrap_or(&mut release);
        if table.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!(
                "release manifest line {line_number} duplicates key `{key}` in the same table"
            ));
        }
    }
    if let Some(formula) = current_formula {
        formula_tables.push(formula);
    }

    let manifest = ReleaseManifest {
        schema_version: required_string(&release, "schema_version", "release")?,
        release_version: required_string(&release, "release_version", "release")?,
        release_tier: required_string(&release, "release_tier", "release")?,
        source_revision_policy: required_string(&release, "source_revision_policy", "release")?,
        base_commit: required_string(&release, "base_commit", "release")?,
        validation_status: required_string(&release, "validation_status", "release")?,
        execution_policy: required_string(&release, "execution_policy", "release")?,
        public_executable: required_bool(&release, "public_executable", "release")?,
        validation_record: required_string(&release, "validation_record", "release")?,
        documentation: required_string(&release, "documentation", "release")?,
        registry_formula_count: required_usize(&release, "registry_formula_count", "release")?,
        cli_dispatch_formula_count: required_usize(
            &release,
            "cli_dispatch_formula_count",
            "release",
        )?,
        public_executable_formula_count: required_usize(
            &release,
            "public_executable_formula_count",
            "release",
        )?,
        formulas: formula_tables
            .iter()
            .enumerate()
            .map(|(index, table)| parse_formula(table, index + 1))
            .collect::<Result<Vec<_>, _>>()?,
    };

    require_only_keys(
        &release,
        &[
            "schema_version",
            "release_version",
            "release_tier",
            "source_revision_policy",
            "base_commit",
            "validation_status",
            "execution_policy",
            "public_executable",
            "validation_record",
            "documentation",
            "registry_formula_count",
            "cli_dispatch_formula_count",
            "public_executable_formula_count",
        ],
        "release",
    )?;
    validate_release_manifest(&manifest)?;
    Ok(manifest)
}

fn parse_formula(table: &BTreeMap<String, String>, index: usize) -> Result<ReleaseFormula, String> {
    let context = format!("formulas table {index}");
    require_only_keys(
        table,
        &[
            "identifier",
            "runtime_symbol",
            "validation_status",
            "execution_policy",
            "public_executable",
            "validation_record",
            "documentation",
        ],
        &context,
    )?;
    Ok(ReleaseFormula {
        identifier: required_string(table, "identifier", &context)?,
        runtime_symbol: optional_string(table, "runtime_symbol", &context)?,
        validation_status: required_string(table, "validation_status", &context)?,
        execution_policy: required_string(table, "execution_policy", &context)?,
        public_executable: required_bool(table, "public_executable", &context)?,
        validation_record: required_string(table, "validation_record", &context)?,
        documentation: required_string(table, "documentation", &context)?,
    })
}

fn validate_release_manifest(manifest: &ReleaseManifest) -> Result<(), String> {
    if manifest.source_revision_policy != "pinned_base_commit" {
        return Err(format!(
            "release source_revision_policy must be `pinned_base_commit`, found `{}`",
            manifest.source_revision_policy
        ));
    }
    if manifest.base_commit.len() != 40
        || !manifest
            .base_commit
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(
            "release base_commit must be a 40-character lowercase Git object ID".to_string(),
        );
    }
    validate_policy_combination(
        "release",
        &manifest.validation_status,
        &manifest.execution_policy,
        manifest.public_executable,
        &manifest.validation_record,
    )?;
    if manifest.formulas.is_empty() {
        return Err("release manifest must contain at least one formulas table".to_string());
    }
    if manifest.cli_dispatch_formula_count != manifest.formulas.len() {
        return Err(format!(
            "cli_dispatch_formula_count={} does not match formulas table count {}",
            manifest.cli_dispatch_formula_count,
            manifest.formulas.len()
        ));
    }
    let executable_count = manifest
        .formulas
        .iter()
        .filter(|formula| formula.public_executable)
        .count();
    if manifest.public_executable_formula_count != executable_count {
        return Err(format!(
            "public_executable_formula_count={} does not match executable formulas table count {executable_count}",
            manifest.public_executable_formula_count
        ));
    }
    if manifest.public_executable != (executable_count > 0) {
        return Err(
            "release public_executable must equal whether any formula is publicly executable"
                .to_string(),
        );
    }

    let mut identifiers = BTreeSet::new();
    for formula in &manifest.formulas {
        if !identifiers.insert(formula.identifier.as_str()) {
            return Err(format!(
                "duplicate release formula identifier `{}`",
                formula.identifier
            ));
        }
        if formula.identifier.trim().is_empty() {
            return Err("release formula identifier must not be empty".to_string());
        }
        if formula
            .runtime_symbol
            .as_deref()
            .is_some_and(|symbol| symbol.trim().is_empty())
        {
            return Err(format!(
                "release formula `{}` has an empty runtime_symbol",
                formula.identifier
            ));
        }
        validate_policy_combination(
            &format!("release formula `{}`", formula.identifier),
            &formula.validation_status,
            &formula.execution_policy,
            formula.public_executable,
            &formula.validation_record,
        )?;
    }
    Ok(())
}

fn validate_policy_combination(
    context: &str,
    status: &str,
    policy: &str,
    public_executable: bool,
    validation_record: &str,
) -> Result<(), String> {
    let expected_policy = match status {
        "research_required" => "blocked",
        "equation_traceable" => "preliminary_flag_required",
        "implementation_verified" => "normal_research",
        "reference_validated" => "publication_supporting",
        other => {
            return Err(format!(
                "{context} has unsupported validation_status `{other}`"
            ))
        }
    };
    if policy != expected_policy {
        return Err(format!(
            "{context} has invalid execution-policy combination: validation_status `{status}` requires `{expected_policy}`, found `{policy}`"
        ));
    }
    if public_executable && !matches!(status, "implementation_verified" | "reference_validated") {
        return Err(format!(
            "{context} cannot be public_executable with validation_status `{status}`"
        ));
    }
    if public_executable && validation_record.trim().is_empty() {
        return Err(format!(
            "{context} is public_executable but lacks a validation_record"
        ));
    }
    Ok(())
}

fn required_string(
    table: &BTreeMap<String, String>,
    key: &str,
    context: &str,
) -> Result<String, String> {
    let raw = table
        .get(key)
        .ok_or_else(|| format!("{context} is missing required metadata `{key}`"))?;
    parse_string(raw).map_err(|error| format!("{context} `{key}`: {error}"))
}

fn optional_string(
    table: &BTreeMap<String, String>,
    key: &str,
    context: &str,
) -> Result<Option<String>, String> {
    table
        .get(key)
        .map(|raw| parse_string(raw).map_err(|error| format!("{context} `{key}`: {error}")))
        .transpose()
}

fn parse_string(raw: &str) -> Result<String, String> {
    let value = raw
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| "must be a double-quoted TOML string".to_string())?;
    if value.contains('"') || value.contains('\\') {
        return Err("escapes and embedded quotes are not supported in this manifest".to_string());
    }
    Ok(value.to_string())
}

fn required_bool(
    table: &BTreeMap<String, String>,
    key: &str,
    context: &str,
) -> Result<bool, String> {
    match table
        .get(key)
        .ok_or_else(|| format!("{context} is missing required metadata `{key}`"))?
        .as_str()
    {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!(
            "{context} `{key}` must be true or false, found `{other}`"
        )),
    }
}

fn required_usize(
    table: &BTreeMap<String, String>,
    key: &str,
    context: &str,
) -> Result<usize, String> {
    table
        .get(key)
        .ok_or_else(|| format!("{context} is missing required metadata `{key}`"))?
        .parse::<usize>()
        .map_err(|_| format!("{context} `{key}` must be a nonnegative integer"))
}

fn require_only_keys(
    table: &BTreeMap<String, String>,
    allowed: &[&str],
    context: &str,
) -> Result<(), String> {
    if let Some(key) = table.keys().find(|key| !allowed.contains(&key.as_str())) {
        return Err(format!("{context} has unknown metadata key `{key}`"));
    }
    Ok(())
}

fn require_existing_repository_file(root: &Path, relative: &str) -> Result<(), String> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path.components().any(|component| {
            !matches!(component, Component::Normal(_))
                || matches!(component, Component::ParentDir | Component::RootDir)
        })
    {
        return Err(format!(
            "release manifest reference `{relative}` must be a normalized repository-relative path"
        ));
    }
    if !root.join(path).is_file() {
        return Err(format!(
            "release manifest reference `{relative}` is missing or not a file"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(formulas: &str) -> String {
        format!(
            "schema_version = \"aerocodex.release_manifest.v1\"\n\
             release_version = \"0.1.0-alpha.1\"\n\
             release_tier = \"research_software_alpha\"\n\
             source_revision_policy = \"pinned_base_commit\"\n\
             base_commit = \"ffcc2b218220cf705d4673f3b44b45c929f3a65d\"\n\
             validation_status = \"research_required\"\n\
             execution_policy = \"blocked\"\n\
             public_executable = false\n\
             validation_record = \"validation/equation_inventory.tsv\"\n\
             documentation = \"docs/release/v0.1.0-alpha.1-status.md\"\n\
             registry_formula_count = 152\n\
             cli_dispatch_formula_count = 1\n\
             public_executable_formula_count = 0\n\n\
             {formulas}"
        )
    }

    fn blocked_formula(identifier: &str) -> String {
        format!(
            "[[formulas]]\n\
             identifier = \"{identifier}\"\n\
             runtime_symbol = \"fixture_symbol\"\n\
             validation_status = \"research_required\"\n\
             execution_policy = \"blocked\"\n\
             public_executable = false\n\
             validation_record = \"validation/cards/fixture.yaml\"\n\
             documentation = \"docs/fixture.md\"\n"
        )
    }

    #[test]
    fn valid_manifest_parses_required_release_metadata() {
        let parsed = parse_release_manifest(&manifest(&blocked_formula("fixture.one")))
            .expect("valid release manifest parses");
        assert_eq!(parsed.release_version, "0.1.0-alpha.1");
        assert_eq!(parsed.release_tier, "research_software_alpha");
        assert_eq!(parsed.formulas.len(), 1);
        assert!(!parsed.public_executable);
    }

    #[test]
    fn missing_required_release_metadata_fails() {
        let text = manifest(&blocked_formula("fixture.one"))
            .replace("release_tier = \"research_software_alpha\"\n", "");
        let error = parse_release_manifest(&text).expect_err("missing tier must fail");
        assert!(error.contains("missing required metadata `release_tier`"));
    }

    #[test]
    fn duplicate_formula_identifiers_fail() {
        let formulas = format!(
            "{}{}",
            blocked_formula("fixture.same"),
            blocked_formula("fixture.same")
        );
        let text = manifest(&formulas).replace(
            "cli_dispatch_formula_count = 1",
            "cli_dispatch_formula_count = 2",
        );
        let error = parse_release_manifest(&text).expect_err("duplicates must fail");
        assert!(error.contains("duplicate release formula identifier"));
    }

    #[test]
    fn invalid_execution_policy_combination_fails() {
        let formula = blocked_formula("fixture.one").replace(
            "execution_policy = \"blocked\"",
            "execution_policy = \"normal_research\"",
        );
        let error = parse_release_manifest(&manifest(&formula))
            .expect_err("research_required cannot be normal execution");
        assert!(error.contains("invalid execution-policy combination"));
    }

    #[test]
    fn executable_formula_without_validation_reference_fails() {
        let formula = blocked_formula("fixture.one")
            .replace(
                "validation_status = \"research_required\"",
                "validation_status = \"implementation_verified\"",
            )
            .replace(
                "execution_policy = \"blocked\"",
                "execution_policy = \"normal_research\"",
            )
            .replace("public_executable = false", "public_executable = true")
            .replace(
                "validation_record = \"validation/cards/fixture.yaml\"",
                "validation_record = \"\"",
            );
        let text = manifest(&formula)
            .replacen(
                "validation_status = \"research_required\"",
                "validation_status = \"implementation_verified\"",
                1,
            )
            .replacen(
                "execution_policy = \"blocked\"",
                "execution_policy = \"normal_research\"",
                1,
            )
            .replacen("public_executable = false", "public_executable = true", 1)
            .replace(
                "public_executable_formula_count = 0",
                "public_executable_formula_count = 1",
            );
        let error = parse_release_manifest(&text)
            .expect_err("executable formula without validation evidence must fail");
        assert!(error.contains("lacks a validation_record"));
    }
}
