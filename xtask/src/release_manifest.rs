use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
    process::{Command, Output},
};

pub const RELEASE_MANIFEST_PATH: &str = "docs/release/v0.1.0-alpha.1.toml";
pub const CLI_DISPATCH_METADATA_PATH: &str = "crates/aero-codex-cli/dispatch_metadata.tsv";
const RELEASE_SCHEMA_VERSION: &str = "aerocodex.release_manifest.v2";
const RELEASE_VERSION: &str = "0.1.0-alpha.1";
const RELEASE_TIER: &str = "research_software_alpha";
const RELEASE_TIER_DISPLAY: &str = "Research Software Alpha";
const CLI_DISPATCH_SCHEMA_VERSION: &str = "aerocodex.cli_dispatch.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub schema_version: String,
    pub release_version: String,
    pub release_tier: String,
    pub release_tier_display: String,
    pub program_name: String,
    pub source_revision_policy: String,
    pub base_commit: String,
    pub validation_status: String,
    pub execution_policy: String,
    pub public_executable: bool,
    pub validation_record: String,
    pub documentation: String,
    pub registry_formula_count: usize,
    pub registry_research_required_formula_count: usize,
    pub registry_blocked_formula_count: usize,
    pub cli_dispatch_metadata: String,
    pub cli_dispatch_formula_count: usize,
    pub public_executable_formula_count: usize,
    pub packages: Vec<ReleasePackage>,
    pub formulas: Vec<ReleaseFormula>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasePackage {
    pub name: String,
    pub manifest_path: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliDispatchFormula {
    canonical_formula_id: String,
    runtime_symbol: String,
}

pub fn verify_release_manifest(root: &Path) -> Result<(), String> {
    let manifest = load_release_manifest(root)?;

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
    if manifest.release_tier_display != RELEASE_TIER_DISPLAY {
        return Err(format!(
            "{RELEASE_MANIFEST_PATH} has release_tier_display `{}`, expected `{RELEASE_TIER_DISPLAY}`",
            manifest.release_tier_display
        ));
    }
    if manifest.program_name != "aerocodex" {
        return Err(format!(
            "{RELEASE_MANIFEST_PATH} has program_name `{}`, expected `aerocodex`",
            manifest.program_name
        ));
    }

    verify_pinned_base_commit(root, &manifest.base_commit)?;

    if manifest.cli_dispatch_metadata != CLI_DISPATCH_METADATA_PATH {
        return Err(format!(
            "release manifest cli_dispatch_metadata must be `{CLI_DISPATCH_METADATA_PATH}`, found `{}`",
            manifest.cli_dispatch_metadata
        ));
    }

    let dispatch_path = root.join(&manifest.cli_dispatch_metadata);
    let dispatch_text = fs::read_to_string(&dispatch_path)
        .map_err(|error| format!("cannot read {CLI_DISPATCH_METADATA_PATH}: {error}"))?;
    let dispatch = parse_cli_dispatch_metadata(&dispatch_text)?;
    validate_dispatch_matches_manifest(&manifest, &dispatch)?;

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
    for package in &manifest.packages {
        require_existing_repository_file(root, &package.manifest_path)?;
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
        "verified release manifest: version={}; tier={}; packages={}; formulas={}; public_executable_formulas={}; execution_policy={}",
        manifest.release_version,
        manifest.release_tier,
        manifest.packages.len(),
        manifest.formulas.len(),
        manifest.public_executable_formula_count,
        manifest.execution_policy
    );
    Ok(())
}

pub fn load_release_manifest(root: &Path) -> Result<ReleaseManifest, String> {
    let path = root.join(RELEASE_MANIFEST_PATH);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {RELEASE_MANIFEST_PATH}: {error}"))?;
    parse_release_manifest(&text)
}

fn parse_cli_dispatch_metadata(text: &str) -> Result<Vec<CliDispatchFormula>, String> {
    const HEADER: &str = "schema_version\tcanonical_formula_id\tdispatch_formula_id\truntime_symbol\toutput_variable\tinputs\tsummary";
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err(format!(
            "{CLI_DISPATCH_METADATA_PATH} has an invalid header"
        ));
    }

    let mut formulas = Vec::new();
    let mut canonical_ids = BTreeSet::new();
    let mut dispatch_ids = BTreeSet::new();
    let mut runtime_symbols = BTreeSet::new();
    for (index, line) in lines.enumerate() {
        let line_number = index + 2;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 7 {
            return Err(format!(
                "{CLI_DISPATCH_METADATA_PATH} line {line_number} has {} fields, expected 7",
                fields.len()
            ));
        }
        if fields.iter().any(|field| field.trim().is_empty()) {
            return Err(format!(
                "{CLI_DISPATCH_METADATA_PATH} line {line_number} contains an empty field"
            ));
        }
        if fields[0] != CLI_DISPATCH_SCHEMA_VERSION {
            return Err(format!(
                "{CLI_DISPATCH_METADATA_PATH} line {line_number} has unsupported schema `{}`",
                fields[0]
            ));
        }
        if !canonical_ids.insert(fields[1].to_string()) {
            return Err(format!(
                "duplicate CLI dispatch canonical formula ID `{}`",
                fields[1]
            ));
        }
        if !dispatch_ids.insert(fields[2].to_string()) {
            return Err(format!("duplicate CLI dispatch formula ID `{}`", fields[2]));
        }
        if !runtime_symbols.insert(fields[3].to_string()) {
            return Err(format!(
                "duplicate CLI dispatch runtime symbol `{}`",
                fields[3]
            ));
        }
        formulas.push(CliDispatchFormula {
            canonical_formula_id: fields[1].to_string(),
            runtime_symbol: fields[3].to_string(),
        });
    }
    if formulas.is_empty() {
        return Err(format!(
            "{CLI_DISPATCH_METADATA_PATH} contains no formula records"
        ));
    }
    Ok(formulas)
}

fn validate_dispatch_matches_manifest(
    manifest: &ReleaseManifest,
    dispatch: &[CliDispatchFormula],
) -> Result<(), String> {
    if manifest.cli_dispatch_formula_count != dispatch.len() {
        return Err(format!(
            "release manifest cli_dispatch_formula_count={} does not match CLI dispatch metadata count {}",
            manifest.cli_dispatch_formula_count,
            dispatch.len()
        ));
    }

    let manifest_by_id: BTreeMap<&str, Option<&str>> = manifest
        .formulas
        .iter()
        .map(|formula| {
            (
                formula.identifier.as_str(),
                formula.runtime_symbol.as_deref(),
            )
        })
        .collect();
    let dispatch_by_id: BTreeMap<&str, &str> = dispatch
        .iter()
        .map(|formula| {
            (
                formula.canonical_formula_id.as_str(),
                formula.runtime_symbol.as_str(),
            )
        })
        .collect();

    let missing: Vec<&str> = dispatch_by_id
        .keys()
        .copied()
        .filter(|identifier| !manifest_by_id.contains_key(identifier))
        .collect();
    let extra: Vec<&str> = manifest_by_id
        .keys()
        .copied()
        .filter(|identifier| !dispatch_by_id.contains_key(identifier))
        .collect();
    let mismatched: Vec<String> = manifest_by_id
        .iter()
        .filter_map(|(identifier, manifest_symbol)| {
            dispatch_by_id.get(identifier).and_then(|dispatch_symbol| {
                (*manifest_symbol != Some(*dispatch_symbol)).then(|| {
                    format!(
                        "{identifier}: manifest={:?}, dispatch={dispatch_symbol}",
                        manifest_symbol
                    )
                })
            })
        })
        .collect();

    if missing.is_empty() && extra.is_empty() && mismatched.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "release manifest does not exactly match CLI dispatch metadata: missing=[{}]; extra=[{}]; runtime_symbol_mismatches=[{}]",
            missing.join(", "),
            extra.join(", "),
            mismatched.join(", ")
        ))
    }
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<Output, String> {
    let safe_directory = format!(
        "safe.directory={}",
        root.to_string_lossy().replace('\\', "/")
    );
    Command::new("git")
        .arg("-c")
        .arg(safe_directory)
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot execute Git for release revision verification: {error}"))
}

fn git_error(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

fn verify_pinned_base_commit(root: &Path, base_commit: &str) -> Result<(), String> {
    if base_commit.len() != 40
        || !base_commit
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(format!(
            "malformed pinned base revision `{base_commit}`: expected a 40-character lowercase Git object ID"
        ));
    }

    let worktree = git_output(root, &["rev-parse", "--is-inside-work-tree"])?;
    if !worktree.status.success() || String::from_utf8_lossy(&worktree.stdout).trim() != "true" {
        return Err(format!(
            "release revision verification requires Git metadata; source archives without .git cannot pass this gate ({})",
            git_error(&worktree)
        ));
    }

    let object_type = git_output(root, &["cat-file", "-t", base_commit])?;
    if !object_type.status.success() {
        return Err(format!(
            "pinned base revision `{base_commit}` does not resolve to a Git object: {}",
            git_error(&object_type)
        ));
    }
    let object_type_name = String::from_utf8_lossy(&object_type.stdout)
        .trim()
        .to_string();
    if object_type_name != "commit" {
        return Err(format!(
            "pinned base revision `{base_commit}` resolves to Git object type `{object_type_name}`, not a commit"
        ));
    }

    let ancestor = git_output(root, &["merge-base", "--is-ancestor", base_commit, "HEAD"])?;
    match ancestor.status.code() {
        Some(0) => Ok(()),
        Some(1) => Err(format!(
            "pinned base commit `{base_commit}` is not an ancestor of the reviewed revision HEAD"
        )),
        _ => Err(format!(
            "Git could not verify ancestry for pinned base commit `{base_commit}`: {}",
            git_error(&ancestor)
        )),
    }
}

pub fn parse_release_manifest(text: &str) -> Result<ReleaseManifest, String> {
    let mut release = BTreeMap::new();
    let mut package_tables = Vec::new();
    let mut formula_tables = Vec::new();
    let mut current_package: Option<BTreeMap<String, String>> = None;
    let mut current_formula: Option<BTreeMap<String, String>> = None;

    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "[[packages]]" {
            if let Some(formula) = current_formula.take() {
                formula_tables.push(formula);
            }
            if let Some(package) = current_package.take() {
                package_tables.push(package);
            }
            current_package = Some(BTreeMap::new());
            continue;
        }
        if line == "[[formulas]]" {
            if let Some(package) = current_package.take() {
                package_tables.push(package);
            }
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
        let previous = if let Some(table) = current_formula.as_mut() {
            table.insert(key.to_string(), value.to_string())
        } else if let Some(table) = current_package.as_mut() {
            table.insert(key.to_string(), value.to_string())
        } else {
            release.insert(key.to_string(), value.to_string())
        };
        if previous.is_some() {
            return Err(format!(
                "release manifest line {line_number} duplicates key `{key}` in the same table"
            ));
        }
    }
    if let Some(package) = current_package {
        package_tables.push(package);
    }
    if let Some(formula) = current_formula {
        formula_tables.push(formula);
    }

    let manifest = ReleaseManifest {
        schema_version: required_string(&release, "schema_version", "release")?,
        release_version: required_string(&release, "release_version", "release")?,
        release_tier: required_string(&release, "release_tier", "release")?,
        release_tier_display: required_string(&release, "release_tier_display", "release")?,
        program_name: required_string(&release, "program_name", "release")?,
        source_revision_policy: required_string(&release, "source_revision_policy", "release")?,
        base_commit: required_string(&release, "base_commit", "release")?,
        validation_status: required_string(&release, "validation_status", "release")?,
        execution_policy: required_string(&release, "execution_policy", "release")?,
        public_executable: required_bool(&release, "public_executable", "release")?,
        validation_record: required_string(&release, "validation_record", "release")?,
        documentation: required_string(&release, "documentation", "release")?,
        registry_formula_count: required_usize(&release, "registry_formula_count", "release")?,
        registry_research_required_formula_count: required_usize(
            &release,
            "registry_research_required_formula_count",
            "release",
        )?,
        registry_blocked_formula_count: required_usize(
            &release,
            "registry_blocked_formula_count",
            "release",
        )?,
        cli_dispatch_metadata: required_string(&release, "cli_dispatch_metadata", "release")?,
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
        packages: package_tables
            .iter()
            .enumerate()
            .map(|(index, table)| parse_package(table, index + 1))
            .collect::<Result<Vec<_>, _>>()?,
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
            "release_tier_display",
            "program_name",
            "source_revision_policy",
            "base_commit",
            "validation_status",
            "execution_policy",
            "public_executable",
            "validation_record",
            "documentation",
            "registry_formula_count",
            "registry_research_required_formula_count",
            "registry_blocked_formula_count",
            "cli_dispatch_metadata",
            "cli_dispatch_formula_count",
            "public_executable_formula_count",
        ],
        "release",
    )?;
    validate_release_manifest(&manifest)?;
    Ok(manifest)
}

fn parse_package(table: &BTreeMap<String, String>, index: usize) -> Result<ReleasePackage, String> {
    let context = format!("packages table {index}");
    require_only_keys(table, &["name", "manifest_path"], &context)?;
    Ok(ReleasePackage {
        name: required_string(table, "name", &context)?,
        manifest_path: required_string(table, "manifest_path", &context)?,
    })
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
        return Err("malformed pinned base revision: release base_commit must be a 40-character lowercase Git object ID".to_string());
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
    if manifest.packages.len() != 14 {
        return Err(format!(
            "release manifest must govern exactly 14 packages, found {}",
            manifest.packages.len()
        ));
    }
    let mut package_names = BTreeSet::new();
    let mut package_paths = BTreeSet::new();
    for package in &manifest.packages {
        if package.name.trim().is_empty() {
            return Err("release package name must not be empty".to_string());
        }
        if !package_names.insert(package.name.as_str()) {
            return Err(format!("duplicate release package name `{}`", package.name));
        }
        if !package_paths.insert(package.manifest_path.as_str()) {
            return Err(format!(
                "duplicate release package manifest_path `{}`",
                package.manifest_path
            ));
        }
        validate_repository_relative_path(&package.manifest_path).map_err(|error| {
            format!(
                "release package `{}` manifest_path is invalid: {error}",
                package.name
            )
        })?;
        if !package.manifest_path.ends_with("/Cargo.toml") && package.manifest_path != "Cargo.toml"
        {
            return Err(format!(
                "release package `{}` manifest_path must name Cargo.toml",
                package.name
            ));
        }
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
    let mut runtime_symbols = BTreeSet::new();
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
        if let Some(symbol) = formula.runtime_symbol.as_deref() {
            if !runtime_symbols.insert(symbol) {
                return Err(format!(
                    "duplicate release formula runtime symbol `{symbol}`"
                ));
            }
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

fn validate_repository_relative_path(relative: &str) -> Result<(), String> {
    let path = Path::new(relative);
    if path.is_absolute()
        || has_windows_absolute_prefix(relative)
        || relative.is_empty()
        || relative.contains('\\')
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(format!(
            "`{relative}` must be a normalized repository-relative path"
        ));
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
    if path.is_absolute() || has_windows_absolute_prefix(relative) {
        return Err(format!(
            "release manifest reference `{relative}` must not be an absolute path"
        ));
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(format!(
            "release manifest reference `{relative}` must not traverse outside the repository"
        ));
    }
    validate_repository_relative_path(relative).map_err(|_| {
        format!(
            "release manifest reference `{relative}` must be a normalized repository-relative path"
        )
    })?;
    let canonical_root = fs::canonicalize(root).map_err(|error| {
        format!(
            "cannot canonicalize repository root {}: {error}",
            root.display()
        )
    })?;
    let candidate = root.join(path);
    let metadata = fs::symlink_metadata(&candidate).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            format!("release manifest reference `{relative}` does not exist")
        } else {
            format!("cannot inspect release manifest reference `{relative}`: {error}")
        }
    })?;
    if metadata.file_type().is_symlink() {
        if fs::canonicalize(&candidate).is_ok_and(|resolved| !resolved.starts_with(&canonical_root))
        {
            return Err(format!(
                "release manifest reference `{relative}` is a symbolic-link escape outside the repository"
            ));
        }
        return Err(format!(
            "release manifest reference `{relative}` must not be a symbolic link"
        ));
    }
    if !metadata.is_file() {
        return Err(format!(
            "release manifest reference `{relative}` is not a regular file"
        ));
    }
    let canonical_candidate = fs::canonicalize(&candidate).map_err(|error| {
        format!("cannot canonicalize release manifest reference `{relative}`: {error}")
    })?;
    if !canonical_candidate.starts_with(&canonical_root) {
        return Err(format!(
            "release manifest reference `{relative}` resolves outside the repository"
        ));
    }
    Ok(())
}

fn has_windows_absolute_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    path.starts_with('/')
        || path.starts_with('\\')
        || (bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && matches!(bytes[2], b'/' | b'\\'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn package_tables() -> String {
        let mut tables = String::new();
        for index in 0..14 {
            tables.push_str(&format!(
                "[[packages]]\nname = \"package-{index:02}\"\nmanifest_path = \"crates/package-{index:02}/Cargo.toml\"\n\n"
            ));
        }
        tables
    }

    fn manifest(formulas: &str) -> String {
        let packages = package_tables();
        format!(
            "schema_version = \"aerocodex.release_manifest.v2\"\n\
             release_version = \"0.1.0-alpha.1\"\n\
             release_tier = \"research_software_alpha\"\n\
             release_tier_display = \"Research Software Alpha\"\n\
             program_name = \"aerocodex\"\n\
             source_revision_policy = \"pinned_base_commit\"\n\
             base_commit = \"6a94b4628e6e0821a55d6aaadf6925956a5fa2d3\"\n\
             validation_status = \"research_required\"\n\
             execution_policy = \"blocked\"\n\
             public_executable = false\n\
             validation_record = \"validation/equation_inventory.tsv\"\n\
             documentation = \"docs/release/v0.1.0-alpha.1-status.md\"\n\
             registry_formula_count = 152\n\
             registry_research_required_formula_count = 152\n\
             registry_blocked_formula_count = 152\n\
             cli_dispatch_metadata = \"crates/aero-codex-cli/dispatch_metadata.tsv\"\n\
             cli_dispatch_formula_count = 1\n\
             public_executable_formula_count = 0\n\n\
             {packages}\
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

    fn dispatch_metadata(rows: &[(&str, &str, &str)]) -> String {
        let mut text = "schema_version\tcanonical_formula_id\tdispatch_formula_id\truntime_symbol\toutput_variable\tinputs\tsummary\n".to_string();
        for (canonical_id, dispatch_id, runtime_symbol) in rows {
            text.push_str(&format!(
                "aerocodex.cli_dispatch.v1\t{canonical_id}\t{dispatch_id}\t{runtime_symbol}\toutput\tinput\tsummary\n"
            ));
        }
        text
    }

    fn parsed_dispatch(rows: &[(&str, &str, &str)]) -> Vec<CliDispatchFormula> {
        parse_cli_dispatch_metadata(&dispatch_metadata(rows)).expect("dispatch fixture parses")
    }

    fn git(root: &Path, arguments: &[&str]) -> String {
        let output = git_output(root, arguments).expect("execute fixture Git command");
        assert!(
            output.status.success(),
            "fixture Git command failed: {}",
            git_error(&output)
        );
        String::from_utf8(output.stdout)
            .expect("fixture Git output is UTF-8")
            .trim()
            .to_string()
    }

    fn git_root(name: &str) -> PathBuf {
        let serial = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aerocodex-release-manifest-{name}-{}-{serial}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale release manifest fixture");
        }
        fs::create_dir_all(&root).expect("create release manifest fixture");
        git(&root, &["init", "-b", "main"]);
        git(&root, &["config", "user.email", "tests@example.invalid"]);
        git(&root, &["config", "user.name", "AeroCodex Tests"]);
        fs::write(root.join("fixture.txt"), b"base\n").expect("write base fixture");
        git(&root, &["add", "."]);
        git(&root, &["commit", "-m", "base"]);
        root
    }

    fn plain_root(name: &str) -> PathBuf {
        let serial = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aerocodex-release-reference-{name}-{}-{serial}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale release reference fixture");
        }
        fs::create_dir_all(&root).expect("create release reference fixture");
        root
    }

    #[cfg(unix)]
    fn create_file_symlink(target: &Path, link: &Path) -> bool {
        std::os::unix::fs::symlink(target, link).expect("create release reference symlink");
        true
    }

    #[cfg(windows)]
    fn create_file_symlink(target: &Path, link: &Path) -> bool {
        use std::{io::ErrorKind, os::windows::fs::symlink_file};

        match symlink_file(target, link) {
            Ok(()) => true,
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::PermissionDenied | ErrorKind::Unsupported
                ) || error.raw_os_error() == Some(1314) =>
            {
                eprintln!("skipping release symlink assertion: {error}");
                false
            }
            Err(error) => panic!("create release reference symlink: {error}"),
        }
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
    fn repository_evidence_requires_normalized_contained_regular_files_without_git() {
        let root = plain_root("containment");
        fs::create_dir_all(root.join("docs")).expect("create evidence directory");
        fs::write(root.join("docs/evidence.md"), b"evidence\n").expect("write evidence");
        require_existing_repository_file(&root, "docs/evidence.md")
            .expect("source-archive evidence does not require .git");

        for (reference, expected) in [
            ("", "normalized repository-relative"),
            ("/absolute.md", "must not be an absolute"),
            ("C:/absolute.md", "must not be an absolute"),
            ("../outside.md", "must not traverse outside"),
            ("docs/../evidence.md", "must not traverse outside"),
            ("docs\\evidence.md", "normalized repository-relative"),
            ("docs/missing.md", "does not exist"),
            ("docs", "not a regular file"),
        ] {
            let error = require_existing_repository_file(&root, reference)
                .expect_err("invalid evidence reference must fail");
            assert!(error.contains(expected), "unexpected error: {error}");
        }
        fs::remove_dir_all(root).expect("remove release reference fixture");
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn repository_evidence_rejects_symlinks_and_symlink_escapes() {
        let root = plain_root("symlink-containment");
        let outside = plain_root("outside");
        fs::write(root.join("inside.md"), b"inside\n").expect("write inside evidence");
        fs::write(outside.join("outside.md"), b"outside\n").expect("write outside evidence");

        if !create_file_symlink(Path::new("inside.md"), &root.join("inside-link.md")) {
            fs::remove_dir_all(root).expect("remove release reference fixture");
            fs::remove_dir_all(outside).expect("remove outside fixture");
            return;
        }
        let error = require_existing_repository_file(&root, "inside-link.md")
            .expect_err("in-repository evidence symlink must fail");
        assert!(error.contains("must not be a symbolic link"));

        if create_file_symlink(&outside.join("outside.md"), &root.join("outside-link.md")) {
            let error = require_existing_repository_file(&root, "outside-link.md")
                .expect_err("outside evidence symlink must fail");
            assert!(error.contains("symbolic-link escape outside the repository"));
        }
        fs::remove_dir_all(root).expect("remove release reference fixture");
        fs::remove_dir_all(outside).expect("remove outside fixture");
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
    fn duplicate_manifest_runtime_symbols_fail() {
        let formulas = format!(
            "{}{}",
            blocked_formula("fixture.one"),
            blocked_formula("fixture.two")
        );
        let text = manifest(&formulas).replace(
            "cli_dispatch_formula_count = 1",
            "cli_dispatch_formula_count = 2",
        );
        let error = parse_release_manifest(&text).expect_err("duplicate symbols must fail");
        assert!(error.contains("duplicate release formula runtime symbol"));
    }

    #[test]
    fn dispatch_metadata_rejects_duplicate_ids_and_symbols() {
        for rows in [
            vec![
                ("fixture.one", "dispatch.one", "symbol_one"),
                ("fixture.one", "dispatch.two", "symbol_two"),
            ],
            vec![
                ("fixture.one", "dispatch.same", "symbol_one"),
                ("fixture.two", "dispatch.same", "symbol_two"),
            ],
            vec![
                ("fixture.one", "dispatch.one", "symbol_same"),
                ("fixture.two", "dispatch.two", "symbol_same"),
            ],
        ] {
            assert!(parse_cli_dispatch_metadata(&dispatch_metadata(&rows)).is_err());
        }
    }

    #[test]
    fn dispatch_comparison_rejects_replaced_missing_extra_and_wrong_symbol() {
        let base = parse_release_manifest(&manifest(&blocked_formula("fixture.one")))
            .expect("manifest fixture parses");

        let replaced = validate_dispatch_matches_manifest(
            &base,
            &parsed_dispatch(&[("fixture.replaced", "dispatch.replaced", "fixture_symbol")]),
        )
        .expect_err("replaced dispatch-linked formula must fail");
        assert!(replaced.contains("missing=[fixture.replaced]"));
        assert!(replaced.contains("extra=[fixture.one]"));

        let mut missing_manifest_formula = base.clone();
        missing_manifest_formula.cli_dispatch_formula_count = 2;
        let missing = validate_dispatch_matches_manifest(
            &missing_manifest_formula,
            &parsed_dispatch(&[
                ("fixture.one", "dispatch.one", "fixture_symbol"),
                ("fixture.two", "dispatch.two", "symbol_two"),
            ]),
        )
        .expect_err("missing manifest formula must fail");
        assert!(missing.contains("missing=[fixture.two]"));

        let mut extra_manifest_formula = base.clone();
        let mut extra = extra_manifest_formula.formulas[0].clone();
        extra.identifier = "fixture.extra".to_string();
        extra.runtime_symbol = Some("symbol_extra".to_string());
        extra_manifest_formula.formulas.push(extra);
        let extra_error = validate_dispatch_matches_manifest(
            &extra_manifest_formula,
            &parsed_dispatch(&[("fixture.one", "dispatch.one", "fixture_symbol")]),
        )
        .expect_err("extra manifest formula must fail");
        assert!(extra_error.contains("extra=[fixture.extra]"));

        let wrong_symbol = validate_dispatch_matches_manifest(
            &base,
            &parsed_dispatch(&[("fixture.one", "dispatch.one", "wrong_symbol")]),
        )
        .expect_err("wrong runtime symbol must fail");
        assert!(wrong_symbol.contains("runtime_symbol_mismatches"));
    }

    #[test]
    fn pinned_base_requires_existing_commit_ancestor() {
        let root = git_root("objects");
        let base = git(&root, &["rev-parse", "HEAD"]);
        fs::write(root.join("fixture.txt"), b"head\n").expect("write head fixture");
        git(&root, &["commit", "-am", "head"]);
        verify_pinned_base_commit(&root, &base).expect("base commit is an ancestor of HEAD");

        let nonexistent = "0000000000000000000000000000000000000000";
        let error = verify_pinned_base_commit(&root, nonexistent)
            .expect_err("nonexistent object must fail");
        assert!(error.contains("does not resolve to a Git object"));

        fs::write(root.join("blob.txt"), b"blob\n").expect("write blob fixture");
        let blob = git(&root, &["hash-object", "-w", "blob.txt"]);
        let error = verify_pinned_base_commit(&root, &blob).expect_err("blob must fail");
        assert!(error.contains("not a commit"));
        fs::remove_dir_all(root).expect("remove release manifest fixture");
    }

    #[test]
    fn pinned_base_rejects_malformed_nonancestor_and_source_archive() {
        let malformed = verify_pinned_base_commit(Path::new("."), "not-an-object-id")
            .expect_err("malformed object ID must fail before Git access");
        assert!(malformed.contains("malformed pinned base revision"));

        let root = git_root("nonancestor");
        git(&root, &["checkout", "-b", "side"]);
        fs::write(root.join("side.txt"), b"side\n").expect("write side fixture");
        git(&root, &["add", "."]);
        git(&root, &["commit", "-m", "side"]);
        let side = git(&root, &["rev-parse", "HEAD"]);
        git(&root, &["checkout", "main"]);
        fs::write(root.join("main.txt"), b"main\n").expect("write main fixture");
        git(&root, &["add", "."]);
        git(&root, &["commit", "-m", "main"]);
        let error = verify_pinned_base_commit(&root, &side).expect_err("nonancestor must fail");
        assert!(error.contains("is not an ancestor"));
        fs::remove_dir_all(&root).expect("remove release manifest fixture");

        let archive = std::env::temp_dir().join(format!(
            "aerocodex-release-archive-{}-{}",
            std::process::id(),
            TEST_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&archive).expect("create source archive fixture");
        let error = verify_pinned_base_commit(&archive, "1111111111111111111111111111111111111111")
            .expect_err("source archive must fail");
        assert!(error.contains("source archives without .git cannot pass"));
        fs::remove_dir_all(archive).expect("remove source archive fixture");
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
