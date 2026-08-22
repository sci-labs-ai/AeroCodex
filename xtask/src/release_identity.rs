use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Component, Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    formula_registry::{
        self,
        check::CheckOptions,
        rust::{CheckedFormulaIdentityRegistry, JsonValue},
    },
    release_manifest::{self, ReleaseManifest, ReleasePackage},
};

const EXPECTED_VERSION: &str = "0.1.0-alpha.1";
const EXPECTED_CHANNEL: &str = "github_releases";
const EXPECTED_TIER: &str = "research_software_alpha";
const EXPECTED_TIER_DISPLAY: &str = "Research Software Alpha";
const EXPECTED_PROGRAM: &str = "aerocodex";
const EXPECTED_PACKAGE_COUNT: usize = 14;
const EXPECTED_FORMULA_COUNT: usize = 152;
const EXPECTED_BASE_COMMIT: &str = "6a94b4628e6e0821a55d6aaadf6925956a5fa2d3";
const ORIGINAL_IMPLEMENTATION_COMMIT: &str = "04431bb787bd8219fb3df7e237769421106f368a";
const FIRST_CORRECTIVE_COMMIT: &str = "4456fabd97b2108911ed3eb218112ec73e04519f";
const SECOND_CORRECTIVE_COMMIT: &str = "75f678ba44af942e5910972d1ee34c60572ce733";
const FINAL_PARSER_CORRECTIVE_COMMIT: &str = "41d8b1d2ee277bb0f7eda7e8af7b4a00eb4c83e6";
const EVALUATED_REVISION_ENV: &str = "AEROCODEX_RELEASE_IDENTITY_REVISION";
const EVALUATED_BASE_REVISION_ENV: &str = "AEROCODEX_RELEASE_IDENTITY_BASE_REVISION";
const COMPLETE_PUBLIC_COMMAND_FIXTURE_COUNT: usize = 55;
const RELEASE_IDENTITY_TEST_FUNCTION_COUNT: usize = 29;
const FORCE_SELF_CHECK_FAILURE_ENV: &str = "AEROCODEX_TEST_FORCE_SELF_CHECK_FAILURE";

const GOVERNED_IMPLEMENTATION_COMMITS: &[&str] = &[
    EXPECTED_BASE_COMMIT,
    ORIGINAL_IMPLEMENTATION_COMMIT,
    FIRST_CORRECTIVE_COMMIT,
    SECOND_CORRECTIVE_COMMIT,
    FINAL_PARSER_CORRECTIVE_COMMIT,
];

const GOVERNED_DOCUMENTS: &[&str] = &[
    "README.md",
    "docs/index.md",
    "docs/roadmap/versioning.md",
    "docs/release/v0.1.0-alpha.1-status.md",
    "docs/research_alpha/json_contract.md",
    "docs/research_alpha/cli_formula_quickstart.md",
    "docs/beta1/cli_quickstart.md",
    "docs/beta1/release_concept.md",
];

const IDENTITY_START: &str = "<!-- aerocodex-current-identity:start -->";
const IDENTITY_END: &str = "<!-- aerocodex-current-identity:end -->";
const HISTORICAL_START: &str = "<!-- aerocodex-historical:start -->";
const HISTORICAL_END: &str = "<!-- aerocodex-historical:end -->";
const IDENTITY_BODY: &str = "Release version: `0.1.0-alpha.1`\nRelease tier: `research_software_alpha` (`Research Software Alpha`)\nWorkspace packages: `14`\nRegistry formulas: `152`\nBlocked formulas: `152`\nPublicly executable formulas: `0`";

const EVIDENCE_START: &str = "<!-- aerocodex-release-evidence:start -->";
const EVIDENCE_END: &str = "<!-- aerocodex-release-evidence:end -->";

#[derive(Debug, Clone, PartialEq, Eq)]
struct CargoPackage {
    id: String,
    name: String,
    version: String,
    manifest_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
    target_directory: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiscoveredPackage {
    name: String,
    version: String,
    manifest_path: String,
    inherits_workspace_version: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RegistrySummary {
    total: usize,
    research_required: usize,
    blocked: usize,
    publicly_executable: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvaluatedRevisionMode {
    DirectCheckout,
    SyntheticPullRequestMerge,
}

impl EvaluatedRevisionMode {
    fn name(self) -> &'static str {
        match self {
            Self::DirectCheckout => "direct_checkout",
            Self::SyntheticPullRequestMerge => "synthetic_pull_request_merge",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvaluatedReleaseRevision {
    checkout: String,
    evaluated: String,
    mode: EvaluatedRevisionMode,
}

pub fn verify_release_identity(root: &Path) -> Result<(), String> {
    let revision = evaluated_release_revision(root)?;
    verify_release_identity_at_revision(root, &revision)
}

fn verify_release_identity_at_revision(
    root: &Path,
    revision: &EvaluatedReleaseRevision,
) -> Result<(), String> {
    validate_governed_implementation_ancestry(root, &revision.evaluated)?;
    release_manifest::verify_release_manifest(root)?;
    let manifest = release_manifest::load_release_manifest(root)?;
    validate_manifest_identity(&manifest)?;

    let workspace_version = workspace_version(root)?;
    validate_workspace_and_manifest_versions(&workspace_version, &manifest.release_version)?;
    let metadata = load_cargo_metadata(root)?;
    let discovered = discover_package_manifests(root, &workspace_version)?;
    validate_package_sets(
        &manifest.packages,
        &manifest.release_version,
        &metadata.packages,
        &discovered,
    )?;

    formula_registry::check::run_check_command(root, &CheckOptions)?;
    let registry = formula_registry::rust::load_checked_formula_identity_registry(root)?;
    let registry_summary = summarize_registry(&registry)?;
    validate_registry_summary(&manifest, registry_summary)?;

    verify_governed_documents(root)?;
    verify_release_status_evidence(root)?;
    verify_compiled_cli(root, &metadata.target_directory)?;

    println!(
        "verified release identity: program={EXPECTED_PROGRAM}; version={EXPECTED_VERSION}; channel={EXPECTED_CHANNEL}; tier={EXPECTED_TIER}; tier_display={EXPECTED_TIER_DISPLAY}; packages={EXPECTED_PACKAGE_COUNT}; formulas={}; research_required={}; blocked={}; publicly_executable={}; self_check=14_passed_0_failed; external_binary=PASS",
        registry_summary.total,
        registry_summary.research_required,
        registry_summary.blocked,
        registry_summary.publicly_executable
    );
    println!(
        "release_identity_threat_model=GitHub Actions fresh runners are authoritative; local verification reduces accidental drift but is not an adversarial host-filesystem sandbox."
    );
    println!(
        "release_identity_revision=checkout:{}; evaluated:{}; mode:{}",
        revision.checkout,
        revision.evaluated,
        revision.mode.name()
    );
    Ok(())
}

fn validate_manifest_identity(manifest: &ReleaseManifest) -> Result<(), String> {
    validate_target_semver(&manifest.release_version)?;
    if manifest.release_channel != EXPECTED_CHANNEL {
        return Err(format!(
            "invalid release channel `{}`; expected `{EXPECTED_CHANNEL}`",
            manifest.release_channel
        ));
    }
    if manifest.release_tier != EXPECTED_TIER {
        return Err(format!(
            "invalid release tier `{}`; expected `{EXPECTED_TIER}`",
            manifest.release_tier
        ));
    }
    if manifest.release_tier_display != EXPECTED_TIER_DISPLAY {
        return Err(format!(
            "invalid display release tier `{}`; expected `{EXPECTED_TIER_DISPLAY}`",
            manifest.release_tier_display
        ));
    }
    if manifest.program_name != EXPECTED_PROGRAM {
        return Err(format!(
            "invalid release program_name `{}`; expected `{EXPECTED_PROGRAM}`",
            manifest.program_name
        ));
    }
    if manifest.base_commit != EXPECTED_BASE_COMMIT {
        return Err(format!(
            "release base_commit `{}` does not match the authorized base `{EXPECTED_BASE_COMMIT}`",
            manifest.base_commit
        ));
    }
    Ok(())
}

fn validate_target_semver(version: &str) -> Result<(), String> {
    let (core_and_pre, build) = version
        .split_once('+')
        .map_or((version, None), |(a, b)| (a, Some(b)));
    if build.is_some_and(|value| !valid_identifiers(value, false)) {
        return Err(format!(
            "malformed SemVer `{version}`: invalid build metadata"
        ));
    }
    let (core, prerelease) = core_and_pre
        .split_once('-')
        .map_or((core_and_pre, None), |(a, b)| (a, Some(b)));
    let numbers: Vec<&str> = core.split('.').collect();
    if numbers.len() != 3
        || numbers
            .iter()
            .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
        || numbers
            .iter()
            .any(|part| part.len() > 1 && part.starts_with('0'))
        || prerelease.is_some_and(|value| !valid_identifiers(value, true))
    {
        return Err(format!("malformed SemVer `{version}`"));
    }
    if prerelease != Some("alpha.1") {
        return Err(format!(
            "release version `{version}` must carry the exact `alpha.1` prerelease"
        ));
    }
    if version != EXPECTED_VERSION {
        return Err(format!(
            "release version `{version}` does not match target `{EXPECTED_VERSION}`"
        ));
    }
    Ok(())
}

fn valid_identifiers(value: &str, reject_numeric_leading_zero: bool) -> bool {
    !value.is_empty()
        && value.split('.').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                && !(reject_numeric_leading_zero
                    && part.len() > 1
                    && part.starts_with('0')
                    && part.bytes().all(|byte| byte.is_ascii_digit()))
        })
}

fn workspace_version(root: &Path) -> Result<String, String> {
    let path = root.join("Cargo.toml");
    let text = fs::read_to_string(&path).map_err(|error| format!("Cargo.toml: {error}"))?;
    section_value(&text, "workspace.package", "version")
        .ok_or_else(|| "Cargo.toml [workspace.package] is missing version".to_string())
        .and_then(|value| parse_toml_string(&value, "workspace package version"))
}

fn validate_workspace_and_manifest_versions(
    workspace_version: &str,
    manifest_version: &str,
) -> Result<(), String> {
    validate_target_semver(workspace_version)?;
    validate_target_semver(manifest_version)?;
    if workspace_version != manifest_version {
        return Err(format!(
            "workspace version `{workspace_version}` does not match release manifest version `{manifest_version}`"
        ));
    }
    Ok(())
}

fn load_cargo_metadata(root: &Path) -> Result<CargoMetadata, String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .current_dir(root)
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .map_err(|error| format!("cannot execute cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let text = String::from_utf8(output.stdout)
        .map_err(|error| format!("cargo metadata emitted invalid UTF-8: {error}"))?;
    parse_cargo_metadata(root, &text)
}

fn parse_cargo_metadata(root: &Path, text: &str) -> Result<CargoMetadata, String> {
    let value = formula_registry::rust::parse_json_value(text)
        .map_err(|error| format!("cannot parse cargo metadata JSON: {error}"))?;
    let object = json_object(&value, "cargo metadata root")?;
    let workspace_members = json_array_field(object, "workspace_members")?
        .iter()
        .map(|value| json_string(value, "workspace_members entry").map(str::to_string))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let package_values = json_array_field(object, "packages")?;
    let mut packages = Vec::new();
    for value in package_values {
        let package = json_object(value, "cargo metadata package")?;
        let id = json_string_field(package, "id")?.to_string();
        if !workspace_members.contains(&id) {
            continue;
        }
        packages.push(CargoPackage {
            id,
            name: json_string_field(package, "name")?.to_string(),
            version: json_string_field(package, "version")?.to_string(),
            manifest_path: repository_relative_path(
                root,
                Path::new(json_string_field(package, "manifest_path")?),
            )?,
        });
    }
    if packages.len() != workspace_members.len() {
        return Err(format!(
            "cargo metadata describes {} workspace member IDs but {} package records",
            workspace_members.len(),
            packages.len()
        ));
    }
    let target_directory = PathBuf::from(json_string_field(object, "target_directory")?);
    Ok(CargoMetadata {
        packages,
        target_directory,
    })
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

fn json_array_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    field: &str,
) -> Result<&'a [JsonValue], String> {
    match object.get(field) {
        Some(JsonValue::Array(values)) => Ok(values),
        Some(_) => Err(format!("JSON field `{field}` must be an array")),
        None => Err(format!("JSON object is missing field `{field}`")),
    }
}

fn json_string_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    field: &str,
) -> Result<&'a str, String> {
    object
        .get(field)
        .ok_or_else(|| format!("JSON object is missing field `{field}`"))
        .and_then(|value| json_string(value, field))
}

fn json_number_field<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    field: &str,
) -> Result<&'a str, String> {
    match object.get(field) {
        Some(JsonValue::Number(value)) => Ok(value),
        Some(_) => Err(format!("JSON field `{field}` must be a number")),
        None => Err(format!("JSON object is missing field `{field}`")),
    }
}

fn json_string<'a>(value: &'a JsonValue, context: &str) -> Result<&'a str, String> {
    match value {
        JsonValue::String(value) => Ok(value),
        _ => Err(format!("JSON field `{context}` must be a string")),
    }
}

fn repository_relative_path(root: &Path, path: &Path) -> Result<String, String> {
    let root = fs::canonicalize(root)
        .map_err(|error| format!("cannot canonicalize repository root: {error}"))?;
    let path = fs::canonicalize(path).map_err(|error| {
        format!(
            "cannot canonicalize package manifest {}: {error}",
            path.display()
        )
    })?;
    let relative = path.strip_prefix(&root).map_err(|_| {
        format!(
            "cargo metadata manifest {} is outside repository {}",
            path.display(),
            root.display()
        )
    })?;
    path_to_slashes(relative)
}

fn path_to_slashes(path: &Path) -> Result<String, String> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => parts.push(
                part.to_str()
                    .ok_or_else(|| format!("path is not valid UTF-8: {}", path.display()))?,
            ),
            _ => {
                return Err(format!(
                    "path is not repository-relative: {}",
                    path.display()
                ))
            }
        }
    }
    Ok(parts.join("/"))
}

fn discover_package_manifests(
    root: &Path,
    workspace_version: &str,
) -> Result<Vec<DiscoveredPackage>, String> {
    let mut manifests = Vec::new();
    collect_cargo_manifests(root, root, &mut manifests)?;
    let mut packages = Vec::new();
    for path in manifests {
        let text =
            fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let Some(name_value) = section_value(&text, "package", "name") else {
            continue;
        };
        let name = parse_toml_string(&name_value, "package name")?;
        let inherited = section_value(&text, "package", "version.workspace")
            .is_some_and(|value| value == "true");
        let version = if inherited {
            workspace_version.to_string()
        } else {
            let value = section_value(&text, "package", "version").ok_or_else(|| {
                format!("package `{name}` is missing version or version.workspace=true")
            })?;
            parse_toml_string(&value, "package version")?
        };
        packages.push(DiscoveredPackage {
            name,
            version,
            manifest_path: repository_relative_path(root, &path)?,
            inherits_workspace_version: inherited,
        });
    }
    packages.sort_by(|left, right| left.manifest_path.cmp(&right.manifest_path));
    Ok(packages)
}

fn collect_cargo_manifests(
    root: &Path,
    directory: &Path,
    manifests: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let file_name = entry.file_name();
        if path.is_dir() {
            if matches!(file_name.to_str(), Some(".git" | "target")) {
                continue;
            }
            collect_cargo_manifests(root, &path, manifests)?;
        } else if file_name == "Cargo.toml" && path != root.join("Cargo.toml") {
            manifests.push(path);
        }
    }
    Ok(())
}

fn section_value(text: &str, target_section: &str, target_key: &str) -> Option<String> {
    let mut section = "";
    for raw_line in text.lines() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.starts_with('[') && line.ends_with(']') {
            section = line.trim_matches(&['[', ']'][..]).trim();
            continue;
        }
        if section != target_section {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() == target_key {
            return Some(value.trim().to_string());
        }
    }
    None
}

fn parse_toml_string(value: &str, context: &str) -> Result<String, String> {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .filter(|value| !value.contains('"') && !value.contains('\\'))
        .map(str::to_string)
        .ok_or_else(|| format!("{context} must be a simple double-quoted TOML string"))
}

fn validate_package_sets(
    governed: &[ReleasePackage],
    expected_version: &str,
    cargo_packages: &[CargoPackage],
    discovered: &[DiscoveredPackage],
) -> Result<(), String> {
    let governed_by_path = unique_governed_packages(governed)?;
    let cargo_by_path = unique_cargo_packages(cargo_packages)?;
    let discovered_by_path = unique_discovered_packages(discovered)?;
    let governed_paths: BTreeSet<&str> = governed_by_path.keys().copied().collect();
    let cargo_paths: BTreeSet<&str> = cargo_by_path.keys().copied().collect();
    let discovered_paths: BTreeSet<&str> = discovered_by_path.keys().copied().collect();
    if governed_paths != cargo_paths || governed_paths != discovered_paths {
        return Err(format!(
            "governed package set does not exactly match Cargo metadata and discovered package manifests: cargo_missing=[{}]; cargo_extra=[{}]; discovered_missing=[{}]; discovered_extra=[{}]",
            join_set(governed_paths.difference(&cargo_paths).copied()),
            join_set(cargo_paths.difference(&governed_paths).copied()),
            join_set(governed_paths.difference(&discovered_paths).copied()),
            join_set(discovered_paths.difference(&governed_paths).copied())
        ));
    }
    for (path, package) in governed_by_path {
        let cargo = cargo_by_path[path];
        let discovered = discovered_by_path[path];
        if cargo.name != package.name || discovered.name != package.name {
            return Err(format!(
                "package name mismatch at `{path}`: manifest=`{}`, cargo=`{}`, discovered=`{}`",
                package.name, cargo.name, discovered.name
            ));
        }
        if cargo.version != expected_version || discovered.version != expected_version {
            return Err(format!(
                "package version mismatch for `{}`: expected `{expected_version}`, cargo=`{}`, manifest=`{}`",
                package.name, cargo.version, discovered.version
            ));
        }
        if !discovered.inherits_workspace_version && discovered.version != expected_version {
            return Err(format!(
                "package `{}` neither inherits nor exactly matches workspace version `{expected_version}`",
                package.name
            ));
        }
    }
    if governed.len() != EXPECTED_PACKAGE_COUNT {
        return Err(format!(
            "governed package count is {}, expected {EXPECTED_PACKAGE_COUNT}",
            governed.len()
        ));
    }
    Ok(())
}

fn unique_governed_packages(
    packages: &[ReleasePackage],
) -> Result<BTreeMap<&str, &ReleasePackage>, String> {
    let mut by_path = BTreeMap::new();
    let mut names = BTreeSet::new();
    for package in packages {
        if !names.insert(package.name.as_str()) {
            return Err(format!(
                "duplicate governed package name `{}`",
                package.name
            ));
        }
        if by_path
            .insert(package.manifest_path.as_str(), package)
            .is_some()
        {
            return Err(format!(
                "duplicate governed package path `{}`",
                package.manifest_path
            ));
        }
    }
    Ok(by_path)
}

fn unique_cargo_packages(
    packages: &[CargoPackage],
) -> Result<BTreeMap<&str, &CargoPackage>, String> {
    let mut by_path = BTreeMap::new();
    let mut names = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for package in packages {
        if !names.insert(package.name.as_str()) {
            return Err(format!("duplicate Cargo package name `{}`", package.name));
        }
        if !ids.insert(package.id.as_str()) {
            return Err(format!("duplicate Cargo package ID `{}`", package.id));
        }
        if by_path
            .insert(package.manifest_path.as_str(), package)
            .is_some()
        {
            return Err(format!(
                "duplicate Cargo manifest path `{}`",
                package.manifest_path
            ));
        }
    }
    Ok(by_path)
}

fn unique_discovered_packages(
    packages: &[DiscoveredPackage],
) -> Result<BTreeMap<&str, &DiscoveredPackage>, String> {
    let mut by_path = BTreeMap::new();
    let mut names = BTreeSet::new();
    for package in packages {
        if !names.insert(package.name.as_str()) {
            return Err(format!(
                "duplicate discovered package name `{}`",
                package.name
            ));
        }
        if by_path
            .insert(package.manifest_path.as_str(), package)
            .is_some()
        {
            return Err(format!(
                "duplicate discovered package path `{}`",
                package.manifest_path
            ));
        }
    }
    Ok(by_path)
}

fn join_set<'a>(values: impl Iterator<Item = &'a str>) -> String {
    values.collect::<Vec<_>>().join(", ")
}

fn summarize_registry(
    registry: &CheckedFormulaIdentityRegistry,
) -> Result<RegistrySummary, String> {
    let mut identifiers = BTreeSet::new();
    let mut summary = RegistrySummary {
        total: registry.formulas.len(),
        research_required: 0,
        blocked: 0,
        publicly_executable: 0,
    };
    if registry.formula_count != registry.formulas.len() {
        return Err(format!(
            "registry formula_count={} does not match parsed row count {}",
            registry.formula_count,
            registry.formulas.len()
        ));
    }
    for formula in &registry.formulas {
        if !identifiers.insert(formula.formula_id.as_str()) {
            return Err(format!("duplicate formula ID `{}`", formula.formula_id));
        }
        if formula.status == "research_required" {
            summary.research_required += 1;
        }
        if formula.execution_policy == "blocked" {
            summary.blocked += 1;
        } else if matches!(
            formula.execution_policy.as_str(),
            "normal_research" | "publication_supporting"
        ) {
            summary.publicly_executable += 1;
        }
    }
    Ok(summary)
}

fn validate_registry_summary(
    manifest: &ReleaseManifest,
    summary: RegistrySummary,
) -> Result<(), String> {
    let expected = RegistrySummary {
        total: manifest.registry_formula_count,
        research_required: manifest.registry_formula_count,
        blocked: manifest.registry_formula_count,
        publicly_executable: 0,
    };
    if summary != expected {
        return Err(format!(
            "manifest versus registry count mismatch: manifest={expected:?}; registry={summary:?}"
        ));
    }
    if summary
        != (RegistrySummary {
            total: EXPECTED_FORMULA_COUNT,
            research_required: EXPECTED_FORMULA_COUNT,
            blocked: EXPECTED_FORMULA_COUNT,
            publicly_executable: 0,
        })
    {
        return Err(format!(
            "release registry posture changed: expected 152 research_required and blocked rows with zero publicly executable formulas, found {summary:?}"
        ));
    }
    Ok(())
}

fn verify_governed_documents(root: &Path) -> Result<(), String> {
    for relative in GOVERNED_DOCUMENTS {
        let bytes = fs::read(root.join(relative))
            .map_err(|error| format!("cannot read governed document `{relative}`: {error}"))?;
        verify_identity_document_bytes(relative, &bytes)?;
    }
    println!(
        "verified governed documentation identity: documents={}; LF_CRLF=accepted; fenced_examples=non_authoritative",
        GOVERNED_DOCUMENTS.len()
    );
    Ok(())
}

fn verify_identity_document_bytes(path: &str, bytes: &[u8]) -> Result<(), String> {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(format!(
            "{path}: UTF-8 BOM at byte zero is not allowed in a governed document"
        ));
    }
    if bytes.starts_with(&[0xff, 0xfe, 0x00, 0x00]) {
        return Err(format!(
            "{path}: UTF-32LE BOM is not allowed in a governed document"
        ));
    }
    if bytes.starts_with(&[0x00, 0x00, 0xfe, 0xff]) {
        return Err(format!(
            "{path}: UTF-32BE BOM is not allowed in a governed document"
        ));
    }
    if bytes.starts_with(&[0xff, 0xfe]) {
        return Err(format!(
            "{path}: UTF-16LE BOM is not allowed in a governed document"
        ));
    }
    if bytes.starts_with(&[0xfe, 0xff]) {
        return Err(format!(
            "{path}: UTF-16BE BOM is not allowed in a governed document"
        ));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("{path}: invalid UTF-8 at byte {}", error.valid_up_to()))?;
    if text.contains('\u{FEFF}') {
        return Err(format!(
            "{path}: U+FEFF BOM marker is not allowed in governed current prose"
        ));
    }
    if text.contains('\u{FFFD}') {
        return Err(format!(
            "{path}: U+FFFD replacement character is not allowed"
        ));
    }
    for marker in ['Ã', 'Â'] {
        if text.contains(marker) {
            return Err(format!(
                "{path}: mojibake marker U+{:04X} is not allowed",
                marker as u32
            ));
        }
    }
    if text.contains("â€") || text.contains("ðŸ") {
        return Err(format!(
            "{path}: common mojibake byte sequence is not allowed"
        ));
    }
    let normalized = text.replace("\r\n", "\n");
    if normalized.contains('\r') {
        return Err(format!("{path}: lone carriage return is not allowed"));
    }
    verify_exact_block(
        path,
        &normalized,
        IDENTITY_START,
        IDENTITY_BODY,
        IDENTITY_END,
    )?;
    scan_current_identity_claims(path, &normalized)
}

fn verify_exact_block(
    path: &str,
    text: &str,
    start: &str,
    body: &str,
    end: &str,
) -> Result<(), String> {
    let start_count = text.matches(start).count();
    let end_count = text.matches(end).count();
    if start_count != 1 || end_count != 1 {
        return Err(format!(
            "{path}: identity block markers must appear exactly once (start={start_count}, end={end_count})"
        ));
    }
    let expected = format!("{start}\n{body}\n{end}");
    if !text.contains(&expected) {
        let line = text[..text.find(start).unwrap_or(0)].lines().count() + 1;
        return Err(format!(
            "{path}:{line}: current identity block does not contain the exact governed values"
        ));
    }
    Ok(())
}

fn scan_current_identity_claims(path: &str, text: &str) -> Result<(), String> {
    let mut paragraph = LogicalParagraph::default();
    let mut parser_state = MarkdownParserState::Visible;
    let mut next_fence_authoritative = false;
    let mut in_identity_block = false;
    let mut in_historical_block = false;
    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let raw_trimmed = line.trim();

        if let MarkdownParserState::Fence(open) = parser_state {
            if let Some(marker) = parse_fence_marker(line) {
                if marker.character == open.character
                    && marker.length >= open.length
                    && marker.rest.trim().is_empty()
                {
                    check_identity_paragraph(path, &paragraph)?;
                    paragraph.clear();
                    parser_state = MarkdownParserState::Visible;
                    continue;
                }
            }
            if !open.authoritative {
                continue;
            }
            if raw_trimmed.is_empty() {
                check_identity_paragraph(path, &paragraph)?;
                paragraph.clear();
            } else {
                paragraph.push(line_number, raw_trimmed);
            }
            continue;
        }

        if parser_state.is_visible() && raw_trimmed == IDENTITY_START {
            check_identity_paragraph(path, &paragraph)?;
            paragraph.clear();
            in_identity_block = true;
            continue;
        }
        if parser_state.is_visible() && raw_trimmed == IDENTITY_END {
            in_identity_block = false;
            continue;
        }
        if in_identity_block {
            continue;
        }

        if parser_state.is_visible() && raw_trimmed == HISTORICAL_START {
            check_identity_paragraph(path, &paragraph)?;
            paragraph.clear();
            if in_historical_block {
                return Err(format!(
                    "{path}:{line_number}: nested historical blocks are not allowed"
                ));
            }
            in_historical_block = true;
            continue;
        }
        if parser_state.is_visible() && raw_trimmed == HISTORICAL_END {
            if !in_historical_block {
                return Err(format!(
                    "{path}:{line_number}: historical block closes without a matching start"
                ));
            }
            in_historical_block = false;
            continue;
        }
        if in_historical_block {
            continue;
        }

        if parser_state.is_visible() && raw_trimmed == "<!-- aerocodex-authoritative-example -->" {
            check_identity_paragraph(path, &paragraph)?;
            paragraph.clear();
            next_fence_authoritative = true;
            continue;
        }

        // Fence openers have precedence over HTML-comment syntax on the same physical line.
        // Once open, the mutually exclusive parser state above prevents fenced markers from
        // opening, closing, or otherwise mutating an HTML comment.
        if parser_state.is_visible() {
            if let Some(marker) = parse_fence_marker(line) {
                check_identity_paragraph(path, &paragraph)?;
                paragraph.clear();
                parser_state = MarkdownParserState::Fence(MarkdownFence {
                    character: marker.character,
                    length: marker.length,
                    authoritative: next_fence_authoritative,
                    opening_line: line_number,
                });
                next_fence_authoritative = false;
                continue;
            }
        }

        let mut html_comment = match parser_state {
            MarkdownParserState::Visible => None,
            MarkdownParserState::HtmlComment(open) => Some(open),
            MarkdownParserState::Fence(_) => unreachable!("fence state handled above"),
        };
        let visible =
            visible_markdown_outside_html_comments(path, line, line_number, &mut html_comment)?;
        parser_state = html_comment.map_or(
            MarkdownParserState::Visible,
            MarkdownParserState::HtmlComment,
        );
        let trimmed = visible.trim();
        if trimmed.is_empty() {
            if raw_trimmed.is_empty() && parser_state.is_visible() {
                check_identity_paragraph(path, &paragraph)?;
                paragraph.clear();
            }
            continue;
        }

        if parser_state.is_visible() {
            if let Some(marker) = parse_fence_marker(&visible) {
                check_identity_paragraph(path, &paragraph)?;
                paragraph.clear();
                parser_state = MarkdownParserState::Fence(MarkdownFence {
                    character: marker.character,
                    length: marker.length,
                    authoritative: next_fence_authoritative,
                    opening_line: line_number,
                });
                next_fence_authoritative = false;
                continue;
            }
        }
        if trimmed.starts_with('#') {
            check_identity_paragraph(path, &paragraph)?;
            paragraph.clear();
            let mut heading = LogicalParagraph::default();
            heading.push(line_number, trimmed);
            check_identity_paragraph(path, &heading)?;
            continue;
        }
        paragraph.push(line_number, trimmed);
    }
    if in_historical_block {
        return Err(format!(
            "{path}: unclosed historical block beginning before end of file"
        ));
    }
    match parser_state {
        MarkdownParserState::Fence(open) => {
            return Err(format!(
                "{path}:{}: unclosed Markdown fence (delimiter={} length={})",
                open.opening_line, open.character as char, open.length
            ));
        }
        MarkdownParserState::HtmlComment(open) => {
            return Err(format!(
                "{path}:{}: unclosed HTML comment",
                open.opening_line
            ));
        }
        MarkdownParserState::Visible => {}
    }
    check_identity_paragraph(path, &paragraph)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HtmlComment {
    opening_line: usize,
}

fn visible_markdown_outside_html_comments(
    path: &str,
    line: &str,
    line_number: usize,
    state: &mut Option<HtmlComment>,
) -> Result<String, String> {
    let mut visible = String::with_capacity(line.len());
    let mut offset = 0usize;
    while offset < line.len() {
        if state.is_some() {
            if let Some(relative_end) = line[offset..].find("-->") {
                offset += relative_end + 3;
                *state = None;
                if !visible.is_empty() && !visible.ends_with(char::is_whitespace) {
                    visible.push(' ');
                }
                if visible.ends_with(char::is_whitespace) {
                    while line
                        .as_bytes()
                        .get(offset)
                        .is_some_and(u8::is_ascii_whitespace)
                    {
                        offset += 1;
                    }
                }
            } else {
                return Ok(visible);
            }
            continue;
        }

        let opening = line[offset..].find("<!--");
        let stray_close = line[offset..].find("-->");
        if let Some(relative_close) = stray_close {
            if opening.map_or(true, |relative_open| relative_close < relative_open) {
                return Err(format!(
                    "{path}:{line_number}: stray HTML comment close marker `-->`"
                ));
            }
        }
        let Some(relative_open) = opening else {
            visible.push_str(&line[offset..]);
            break;
        };
        visible.push_str(&line[offset..offset + relative_open]);
        if !visible.is_empty() && !visible.ends_with(char::is_whitespace) {
            visible.push(' ');
        }
        offset += relative_open + 4;
        *state = Some(HtmlComment {
            opening_line: line_number,
        });
    }
    Ok(visible)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MarkdownFence {
    character: u8,
    length: usize,
    authoritative: bool,
    opening_line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkdownParserState {
    Visible,
    HtmlComment(HtmlComment),
    Fence(MarkdownFence),
}

impl MarkdownParserState {
    fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }
}

#[derive(Debug, Clone, Copy)]
struct FenceMarker<'a> {
    character: u8,
    length: usize,
    rest: &'a str,
}

fn parse_fence_marker(line: &str) -> Option<FenceMarker<'_>> {
    let bytes = line.as_bytes();
    let mut offset = 0usize;
    while offset < bytes.len() && bytes[offset] == b' ' && offset < 4 {
        offset += 1;
    }
    if offset > 3 || offset >= bytes.len() || !matches!(bytes[offset], b'`' | b'~') {
        return None;
    }
    let character = bytes[offset];
    let mut end = offset;
    while end < bytes.len() && bytes[end] == character {
        end += 1;
    }
    let length = end - offset;
    if length < 3 {
        return None;
    }
    let rest = &line[end..];
    if character == b'`' && rest.as_bytes().contains(&b'`') {
        return None;
    }
    Some(FenceMarker {
        character,
        length,
        rest,
    })
}

#[derive(Debug, Default)]
struct LogicalParagraph {
    text: String,
    lines: Vec<ParagraphLine>,
}

#[derive(Debug)]
struct ParagraphLine {
    end: usize,
    line: usize,
}

impl LogicalParagraph {
    fn push(&mut self, line: usize, text: &str) {
        if !self.text.is_empty() {
            self.text.push(' ');
        }
        self.text.push_str(text);
        self.lines.push(ParagraphLine {
            end: self.text.len(),
            line,
        });
    }

    fn clear(&mut self) {
        self.text.clear();
        self.lines.clear();
    }

    fn line_for_offset(&self, offset: usize) -> usize {
        self.lines
            .iter()
            .find(|line| offset <= line.end)
            .or_else(|| self.lines.last())
            .map_or(1, |line| line.line)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClaimKind {
    SemanticVersion,
    MachineTier,
    DisplayTier,
    ProgramName,
    ReleaseChannel,
    RuntimeIdentity,
}

impl ClaimKind {
    fn name(self) -> &'static str {
        match self {
            Self::SemanticVersion => "semantic_version",
            Self::MachineTier => "machine_release_tier",
            Self::DisplayTier => "display_release_tier",
            Self::ProgramName => "program_name",
            Self::ReleaseChannel => "release_channel",
            Self::RuntimeIdentity => "runtime_identity",
        }
    }

    fn expected(self) -> &'static str {
        match self {
            Self::SemanticVersion => EXPECTED_VERSION,
            Self::MachineTier => EXPECTED_TIER,
            Self::DisplayTier => EXPECTED_TIER_DISPLAY,
            Self::ProgramName => EXPECTED_PROGRAM,
            Self::ReleaseChannel => EXPECTED_CHANNEL,
            Self::RuntimeIdentity => "AeroCodex",
        }
    }

    fn captures_words(self) -> bool {
        matches!(self, Self::DisplayTier)
    }
}

#[derive(Debug, Clone, Copy)]
struct ClaimPattern {
    needle: &'static str,
    kind: ClaimKind,
}

const CLAIM_PATTERNS: &[ClaimPattern] = &[
    ClaimPattern {
        needle: "current cargo-compatible semantic version is ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "current cargo package version is ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "current semantic version is ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "current release version is ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "current cargo version is ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "active workspace version is ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "current workspace version is ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "release version remains ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "cargo version remains ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "release version: ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "semantic version: ",
        kind: ClaimKind::SemanticVersion,
    },
    ClaimPattern {
        needle: "current machine release tier is ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "current machine tier is ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "current release tier is ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "current tier is ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "release tier remains ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "release tier: ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "machine release tier: ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "machine tier: ",
        kind: ClaimKind::MachineTier,
    },
    ClaimPattern {
        needle: "current display release tier is ",
        kind: ClaimKind::DisplayTier,
    },
    ClaimPattern {
        needle: "current display tier is ",
        kind: ClaimKind::DisplayTier,
    },
    ClaimPattern {
        needle: "display release tier: ",
        kind: ClaimKind::DisplayTier,
    },
    ClaimPattern {
        needle: "display tier is ",
        kind: ClaimKind::DisplayTier,
    },
    ClaimPattern {
        needle: "display tier: ",
        kind: ClaimKind::DisplayTier,
    },
    ClaimPattern {
        needle: "current program name is ",
        kind: ClaimKind::ProgramName,
    },
    ClaimPattern {
        needle: "program name: ",
        kind: ClaimKind::ProgramName,
    },
    ClaimPattern {
        needle: "current release name is ",
        kind: ClaimKind::ProgramName,
    },
    ClaimPattern {
        needle: "release name: ",
        kind: ClaimKind::ProgramName,
    },
    ClaimPattern {
        needle: "current release channel is ",
        kind: ClaimKind::ReleaseChannel,
    },
    ClaimPattern {
        needle: "release channel remains ",
        kind: ClaimKind::ReleaseChannel,
    },
    ClaimPattern {
        needle: "release channel: ",
        kind: ClaimKind::ReleaseChannel,
    },
    ClaimPattern {
        needle: "current runtime identity is ",
        kind: ClaimKind::RuntimeIdentity,
    },
    ClaimPattern {
        needle: "current runtime identity remains ",
        kind: ClaimKind::RuntimeIdentity,
    },
    ClaimPattern {
        needle: "runtime identity: ",
        kind: ClaimKind::RuntimeIdentity,
    },
];

#[derive(Debug, Clone, Copy)]
struct ClaimCandidate {
    start: usize,
    end: usize,
    kind: ClaimKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClaimPolarity {
    Affirmative,
    Negative,
    Ambiguous,
}

impl ClaimPolarity {
    fn name(self) -> &'static str {
        match self {
            Self::Affirmative => "affirmative",
            Self::Negative => "negative",
            Self::Ambiguous => "ambiguous",
        }
    }
}

#[derive(Debug, Clone)]
struct NegativeClaim {
    start: usize,
    end: usize,
    current_start: usize,
    affirmative_candidate_start: Option<usize>,
    kind: ClaimKind,
    asserted: String,
}

fn check_identity_paragraph(path: &str, paragraph: &LogicalParagraph) -> Result<(), String> {
    if paragraph.text.is_empty() {
        return Ok(());
    }
    let inline_spans = inline_code_spans(&paragraph.text);
    let mut masked = paragraph.text.as_bytes().to_vec();
    for &(start, end) in &inline_spans {
        for byte in &mut masked[start..end] {
            *byte = b' ';
        }
    }
    let masked = String::from_utf8(masked).expect("masking UTF-8 with ASCII spaces remains UTF-8");
    let lower = paragraph.text.to_ascii_lowercase();
    let masked_lower = masked.to_ascii_lowercase();
    let mut candidates = Vec::new();
    for pattern in CLAIM_PATTERNS {
        let mut offset = 0usize;
        while let Some(relative) = lower[offset..].find(pattern.needle) {
            let start = offset + relative;
            let end = start + pattern.needle.len();
            if !inline_spans
                .iter()
                .any(|&(inline_start, inline_end)| start >= inline_start && start < inline_end)
            {
                candidates.push(ClaimCandidate {
                    start,
                    end,
                    kind: pattern.kind,
                });
            }
            offset = end;
        }
    }
    candidates.sort_by(|left, right| {
        left.start
            .cmp(&right.start)
            .then_with(|| (right.end - right.start).cmp(&(left.end - left.start)))
    });
    let mut accepted: Vec<ClaimCandidate> = Vec::new();
    for candidate in candidates {
        if accepted
            .iter()
            .any(|prior| candidate.start < prior.end && candidate.end > prior.start)
        {
            continue;
        }
        accepted.push(candidate);
    }

    let negative_claims =
        collect_negative_claims(&paragraph.text, &masked_lower, &inline_spans, &accepted);
    validate_negative_claim_grammar(path, paragraph, &masked_lower, &negative_claims)?;

    for negative in &negative_claims {
        let expected = negative.kind.expected();
        if negative.asserted == expected {
            return Err(claim_diagnostic(
                path,
                paragraph,
                negative.start,
                negative.end,
                (negative.kind.name(), ClaimPolarity::Negative),
                &negative.asserted,
                expected,
            ));
        }
    }

    for candidate in &accepted {
        if negative_claims
            .iter()
            .any(|negative| negative.affirmative_candidate_start == Some(candidate.start))
        {
            continue;
        }
        let (asserted, value_start, value_end) = extract_asserted_value(
            &paragraph.text,
            candidate.end,
            candidate.kind.captures_words(),
        );
        let expected = candidate.kind.expected();
        if asserted != expected {
            return Err(claim_diagnostic(
                path,
                paragraph,
                candidate.start,
                value_end.max(value_start),
                (candidate.kind.name(), ClaimPolarity::Affirmative),
                &asserted,
                expected,
            ));
        }
    }

    for current_start in find_word_occurrences(&masked_lower, "current") {
        if negative_claims
            .iter()
            .any(|negative| negative.current_start == current_start)
        {
            continue;
        }
        let clause_end = clause_end(&masked_lower, current_start);
        let clause = &masked_lower[current_start..clause_end];
        let identity_noun = !find_word_occurrences(clause, "version").is_empty()
            || !find_word_occurrences(clause, "tier").is_empty()
            || [
                "runtime identity",
                "release channel",
                "program name",
                "release name",
            ]
            .iter()
            .any(|noun| clause.contains(noun));
        if identity_noun
            && !accepted
                .iter()
                .any(|candidate| candidate.start >= current_start && candidate.start < clause_end)
        {
            let end_line = paragraph.line_for_offset(clause_end.saturating_sub(1));
            return Err(format!(
                "{path}:{}-{end_line}: claim_type=ambiguous_current_identity polarity={} asserted=`{}` expected=`supported explicit current identity wording using the canonical value`",
                paragraph.line_for_offset(current_start),
                ClaimPolarity::Ambiguous.name(),
                paragraph.text[current_start..clause_end].trim()
            ));
        }
    }
    Ok(())
}

fn validate_negative_claim_grammar(
    path: &str,
    paragraph: &LogicalParagraph,
    masked_lower: &str,
    negative_claims: &[NegativeClaim],
) -> Result<(), String> {
    for current_start in find_word_occurrences(masked_lower, "current") {
        let start = clause_start(masked_lower, current_start);
        let end = clause_end(masked_lower, current_start);
        let clause = &masked_lower[start..end];
        if identity_kind_for_clause(clause).is_none() {
            continue;
        }

        let mut negations = Vec::new();
        for token in ["not", "never", "cannot", "false", "neither", "nor"] {
            negations.extend(
                find_word_occurrences(clause, token)
                    .into_iter()
                    .map(|offset| (start + offset, token)),
            );
        }
        negations.sort_by_key(|(offset, _)| *offset);
        if negations.is_empty() {
            continue;
        }

        let matching = negative_claims
            .iter()
            .filter(|claim| claim.current_start == current_start)
            .collect::<Vec<_>>();
        let supported = matching.len() == 1
            && negations.len() == 1
            && negations[0].1 == "not"
            && negations[0].0 >= matching[0].start
            && negations[0].0 < matching[0].end;
        if supported {
            continue;
        }

        let reason = if negations.len() > 1 {
            "multiple_or_nested_negation_tokens"
        } else if matching.len() > 1 {
            "multiple_supported_negative_templates_overlap"
        } else {
            "unsupported_negation_grammar"
        };
        let matched = matching.first().copied();
        let kind = matched
            .map(|claim| claim.kind)
            .or_else(|| identity_kind_for_clause(clause));
        let diagnostic_start = matched.map_or(start, |claim| claim.start);
        let diagnostic_end = matched.map_or(end, |claim| claim.end);
        let asserted = matched.map_or_else(
            || paragraph.text[start..end].trim(),
            |claim| claim.asserted.as_str(),
        );
        return Err(format!(
            "{path}:{}-{}: claim_type={} polarity={} asserted=`{asserted}` expected=`{}` reason={reason}",
            paragraph.line_for_offset(diagnostic_start),
            paragraph.line_for_offset(diagnostic_end.saturating_sub(1)),
            kind.map_or("ambiguous_current_identity", ClaimKind::name),
            ClaimPolarity::Ambiguous.name(),
            kind.map_or(
                "supported explicit current identity wording using the canonical value",
                ClaimKind::expected
            )
        ));
    }
    Ok(())
}

fn identity_kind_for_clause(clause: &str) -> Option<ClaimKind> {
    if clause.contains("version") {
        Some(ClaimKind::SemanticVersion)
    } else if clause.contains("display tier") {
        Some(ClaimKind::DisplayTier)
    } else if clause.contains("tier") {
        Some(ClaimKind::MachineTier)
    } else if clause.contains("release channel") {
        Some(ClaimKind::ReleaseChannel)
    } else if clause.contains("runtime identity") {
        Some(ClaimKind::RuntimeIdentity)
    } else if clause.contains("program name") || clause.contains("release name") {
        Some(ClaimKind::ProgramName)
    } else {
        None
    }
}

fn collect_negative_claims(
    text: &str,
    lower: &str,
    inline_spans: &[(usize, usize)],
    candidates: &[ClaimCandidate],
) -> Vec<NegativeClaim> {
    let mut claims = Vec::new();

    for candidate in candidates {
        let not_start = skip_ascii_whitespace(lower, candidate.end);
        if !word_at(lower, not_start, "not") {
            continue;
        }
        let value_offset = skip_ascii_whitespace(lower, not_start + 3);
        let (asserted, value_start, value_end) =
            extract_asserted_value(text, value_offset, candidate.kind.captures_words());
        let first_word = asserted
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .find(|word| !word.is_empty())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if asserted.is_empty()
            || matches!(
                first_word.as_str(),
                "not" | "merely" | "only" | "simply" | "exclusively" | "necessarily"
            )
        {
            continue;
        }
        claims.push(NegativeClaim {
            start: candidate.start,
            end: value_end.max(value_start),
            current_start: candidate.start,
            affirmative_candidate_start: Some(candidate.start),
            kind: candidate.kind,
            asserted,
        });
    }

    for (suffix, kind) in [
        (
            " is not the current release version",
            ClaimKind::SemanticVersion,
        ),
        (
            " is not the current cargo version",
            ClaimKind::SemanticVersion,
        ),
        (" is not the current version", ClaimKind::SemanticVersion),
        (" is not the current release tier", ClaimKind::MachineTier),
        (" is not the current machine tier", ClaimKind::MachineTier),
        (" is not the current tier", ClaimKind::MachineTier),
        (" is not the current program name", ClaimKind::ProgramName),
        (
            " is not the current release channel",
            ClaimKind::ReleaseChannel,
        ),
        (
            " is not the current runtime identity",
            ClaimKind::RuntimeIdentity,
        ),
    ] {
        let mut offset = 0usize;
        while let Some(relative) = lower[offset..].find(suffix) {
            let suffix_start = offset + relative;
            let current_start = suffix_start
                + suffix
                    .find("current")
                    .expect("negative suffixes contain current");
            let end = suffix_start + suffix.len();
            let inside_inline = inline_spans.iter().any(|&(inline_start, inline_end)| {
                current_start >= inline_start && current_start < inline_end
            });
            if !inside_inline {
                let (asserted, value_start, value_end) =
                    extract_preceding_asserted_value(text, suffix_start);
                if !asserted.is_empty() {
                    claims.push(NegativeClaim {
                        start: value_start,
                        end: end.max(value_end),
                        current_start,
                        affirmative_candidate_start: None,
                        kind,
                        asserted,
                    });
                }
            }
            offset = end;
        }
    }

    for (prefix, suffix, kind) in [
        (
            "does not claim ",
            " as the current release version",
            ClaimKind::SemanticVersion,
        ),
        (
            "does not claim ",
            " as the current version",
            ClaimKind::SemanticVersion,
        ),
        (
            "does not claim ",
            " as the current release tier",
            ClaimKind::MachineTier,
        ),
        (
            "does not claim ",
            " as the current tier",
            ClaimKind::MachineTier,
        ),
        (
            "does not claim that ",
            " is the current release version",
            ClaimKind::SemanticVersion,
        ),
        (
            "does not claim that ",
            " is the current version",
            ClaimKind::SemanticVersion,
        ),
        (
            "does not claim that ",
            " is the current release tier",
            ClaimKind::MachineTier,
        ),
        (
            "does not claim that ",
            " is the current tier",
            ClaimKind::MachineTier,
        ),
    ] {
        let mut offset = 0usize;
        while let Some(relative_prefix) = lower[offset..].find(prefix) {
            let start = offset + relative_prefix;
            let value_start = start + prefix.len();
            let search_end = clause_end(lower, value_start);
            let Some(relative_suffix) = lower[value_start..search_end].find(suffix) else {
                offset = value_start;
                continue;
            };
            let suffix_start = value_start + relative_suffix;
            let current_start = suffix_start
                + suffix
                    .find("current")
                    .expect("negative suffixes contain current");
            let end = suffix_start + suffix.len();
            let asserted = text[value_start..suffix_start]
                .trim()
                .trim_matches(|character| matches!(character, '*' | '_' | '`' | '"' | '\''))
                .to_string();
            if prefix == "does not claim "
                && asserted
                    .get(..5)
                    .is_some_and(|leading| leading.eq_ignore_ascii_case("that "))
            {
                offset = end;
                continue;
            }
            let inside_inline = inline_spans.iter().any(|&(inline_start, inline_end)| {
                current_start >= inline_start && current_start < inline_end
            });
            if !asserted.is_empty() && !inside_inline {
                claims.push(NegativeClaim {
                    start,
                    end,
                    current_start,
                    affirmative_candidate_start: None,
                    kind,
                    asserted,
                });
            }
            offset = end;
        }
    }

    claims.sort_by_key(|claim| (claim.current_start, claim.start, claim.end));
    claims.dedup_by(|left, right| {
        left.current_start == right.current_start && left.kind == right.kind
    });
    claims
}

fn skip_ascii_whitespace(text: &str, mut offset: usize) -> usize {
    while text
        .as_bytes()
        .get(offset)
        .is_some_and(u8::is_ascii_whitespace)
    {
        offset += 1;
    }
    offset
}

fn word_at(text: &str, start: usize, word: &str) -> bool {
    text.get(start..start + word.len()) == Some(word)
        && text
            .as_bytes()
            .get(start + word.len())
            .map_or(true, |byte| !byte.is_ascii_alphanumeric() && *byte != b'_')
}

fn extract_preceding_asserted_value(text: &str, offset: usize) -> (String, usize, usize) {
    let bytes = text.as_bytes();
    let mut end = offset;
    while end > 0
        && (bytes[end - 1].is_ascii_whitespace()
            || matches!(bytes[end - 1], b'*' | b'_' | b'`' | b'"' | b'\''))
    {
        end -= 1;
    }
    let mut start = end;
    while start > 0
        && !bytes[start - 1].is_ascii_whitespace()
        && !matches!(
            bytes[start - 1],
            b';' | b',' | b'!' | b'?' | b'(' | b'[' | b'{'
        )
    {
        start -= 1;
    }
    let asserted = text[start..end]
        .trim_matches(|character| matches!(character, '*' | '_' | '`' | '"' | '\''))
        .trim_end_matches('.')
        .to_string();
    (asserted, start, end)
}

fn inline_code_spans(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut spans = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'`' {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && bytes[index] == b'`' {
            index += 1;
        }
        let run = index - start;
        let mut search = index;
        let mut close = None;
        while search < bytes.len() {
            if bytes[search] != b'`' {
                search += 1;
                continue;
            }
            let close_start = search;
            while search < bytes.len() && bytes[search] == b'`' {
                search += 1;
            }
            if search - close_start == run {
                close = Some(search);
                break;
            }
        }
        if let Some(end) = close {
            spans.push((start, end));
            index = end;
        }
    }
    spans
}

fn extract_asserted_value(text: &str, offset: usize, words: bool) -> (String, usize, usize) {
    let bytes = text.as_bytes();
    let mut start = offset;
    while start < bytes.len()
        && (bytes[start].is_ascii_whitespace() || matches!(bytes[start], b'*' | b'_'))
    {
        start += 1;
    }
    if start < bytes.len() && bytes[start] == b'`' {
        let mut delimiter_end = start;
        while delimiter_end < bytes.len() && bytes[delimiter_end] == b'`' {
            delimiter_end += 1;
        }
        let run = delimiter_end - start;
        let mut search = delimiter_end;
        while search < bytes.len() {
            if bytes[search] != b'`' {
                search += 1;
                continue;
            }
            let close_start = search;
            while search < bytes.len() && bytes[search] == b'`' {
                search += 1;
            }
            if search - close_start == run {
                return (
                    text[delimiter_end..close_start].trim().to_string(),
                    delimiter_end,
                    close_start,
                );
            }
        }
    }
    let mut end = start;
    if words {
        while end < bytes.len() {
            let remaining = &text[end..];
            if remaining.starts_with('—')
                || matches!(bytes[end], b';' | b'.' | b'!' | b'?' | b')' | b']' | b'}')
            {
                break;
            }
            end += text[end..].chars().next().map_or(1, char::len_utf8);
        }
    } else {
        while end < bytes.len()
            && !bytes[end].is_ascii_whitespace()
            && !matches!(
                bytes[end],
                b';' | b',' | b'!' | b'?' | b'(' | b')' | b'[' | b']' | b'{' | b'}'
            )
        {
            end += 1;
        }
    }
    let asserted = text[start..end]
        .trim()
        .trim_matches(|character| matches!(character, '*' | '_' | '`' | '"' | '\''))
        .trim_end_matches('.')
        .trim()
        .to_string();
    (asserted, start, end)
}

fn claim_diagnostic(
    path: &str,
    paragraph: &LogicalParagraph,
    start: usize,
    end: usize,
    claim: (&str, ClaimPolarity),
    asserted: &str,
    expected: &str,
) -> String {
    let (claim_type, polarity) = claim;
    format!(
        "{path}:{}-{}: claim_type={claim_type} polarity={} asserted=`{asserted}` expected=`{expected}`",
        paragraph.line_for_offset(start),
        paragraph.line_for_offset(end),
        polarity.name()
    )
}

fn find_word_occurrences(text: &str, needle: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    let mut offset = 0usize;
    while let Some(relative) = text[offset..].find(needle) {
        let start = offset + relative;
        let before = start
            .checked_sub(1)
            .and_then(|index| text.as_bytes().get(index))
            .copied();
        let after = text.as_bytes().get(start + needle.len()).copied();
        if before.map_or(true, |byte| !byte.is_ascii_alphanumeric())
            && after.map_or(true, |byte| !byte.is_ascii_alphanumeric())
        {
            matches.push(start);
        }
        offset = start + needle.len();
    }
    matches
}

fn clause_end(text: &str, start: usize) -> usize {
    let mut end = start;
    while end < text.len() {
        let remaining = &text[end..];
        if remaining.starts_with('—') || matches!(text.as_bytes()[end], b';' | b'!' | b'?') {
            break;
        }
        if text.as_bytes()[end] == b'.'
            && text
                .as_bytes()
                .get(end + 1)
                .map_or(true, |byte| byte.is_ascii_whitespace())
        {
            break;
        }
        end += remaining.chars().next().map_or(1, char::len_utf8);
    }
    end
}

fn clause_start(text: &str, end: usize) -> usize {
    let mut start = 0usize;
    let mut offset = 0usize;
    while offset < end {
        let remaining = &text[offset..];
        if remaining.starts_with('—') {
            offset += '—'.len_utf8();
            start = offset;
            continue;
        }
        let byte = text.as_bytes()[offset];
        if matches!(byte, b';' | b'!' | b'?')
            || (byte == b'.'
                && text
                    .as_bytes()
                    .get(offset + 1)
                    .map_or(true, u8::is_ascii_whitespace))
        {
            start = offset + 1;
        }
        offset += remaining.chars().next().map_or(1, char::len_utf8);
    }
    skip_ascii_whitespace(text, start)
}

fn evaluated_release_revision(root: &Path) -> Result<EvaluatedReleaseRevision, String> {
    let supplied = revision_environment(EVALUATED_REVISION_ENV)?;
    evaluate_release_revision_with_base(root, supplied.as_deref(), || {
        revision_environment(EVALUATED_BASE_REVISION_ENV)
    })
}

fn revision_environment(name: &str) -> Result<Option<String>, String> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => {
            Err(format!("{name} must be a Unicode full Git commit SHA"))
        }
    }
}

#[cfg(test)]
fn evaluate_release_revision(
    root: &Path,
    supplied: Option<&str>,
    supplied_base: Option<&str>,
) -> Result<EvaluatedReleaseRevision, String> {
    evaluate_release_revision_with_base(root, supplied, || Ok(supplied_base.map(str::to_owned)))
}

fn evaluate_release_revision_with_base<F>(
    root: &Path,
    supplied: Option<&str>,
    load_base: F,
) -> Result<EvaluatedReleaseRevision, String>
where
    F: FnOnce() -> Result<Option<String>, String>,
{
    let checkout = resolve_git_commit(root, "HEAD", "repository checkout HEAD", false)?;
    let Some(supplied) = supplied else {
        return Ok(EvaluatedReleaseRevision {
            evaluated: checkout.clone(),
            checkout,
            mode: EvaluatedRevisionMode::DirectCheckout,
        });
    };
    let evaluated = resolve_git_commit(root, supplied, EVALUATED_REVISION_ENV, true)?;
    if evaluated == checkout {
        return Ok(EvaluatedReleaseRevision {
            checkout,
            evaluated,
            mode: EvaluatedRevisionMode::DirectCheckout,
        });
    }

    let parent_line = git_text(root, &["rev-list", "--parents", "-n", "1", &checkout])?;
    let revisions = parent_line.split_whitespace().collect::<Vec<_>>();
    if revisions.first().copied() != Some(checkout.as_str()) {
        return Err(format!(
            "cannot determine direct parents for checkout revision `{checkout}`"
        ));
    }
    let parents = &revisions[1..];
    if parents.len() != 2 {
        return Err(format!(
            "{EVALUATED_REVISION_ENV} resolved to `{evaluated}`, but checkout `{checkout}` is not a two-parent synthetic pull-request merge; an explicit differing revision is not allowed"
        ));
    }
    let supplied_base = load_base()?.ok_or_else(|| {
        format!(
            "{EVALUATED_BASE_REVISION_ENV} is required when checkout `{checkout}` differs from {EVALUATED_REVISION_ENV} `{evaluated}` for synthetic pull-request merge evaluation"
        )
    })?;
    let evaluated_base =
        resolve_git_commit(root, &supplied_base, EVALUATED_BASE_REVISION_ENV, true)?;
    if evaluated_base != parents[0] {
        return Err(format!(
            "{EVALUATED_BASE_REVISION_ENV} resolved to `{evaluated_base}`, which is not trusted base parent `{}` of synthetic merge `{checkout}`",
            parents[0]
        ));
    }
    if evaluated != parents[1] {
        return Err(format!(
            "{EVALUATED_REVISION_ENV} resolved to `{evaluated}`, which is not pull-request head parent `{}` of synthetic merge `{checkout}`",
            parents[1]
        ));
    }

    Ok(EvaluatedReleaseRevision {
        checkout,
        evaluated,
        mode: EvaluatedRevisionMode::SyntheticPullRequestMerge,
    })
}

fn resolve_git_commit(
    root: &Path,
    revision: &str,
    label: &str,
    require_full_sha: bool,
) -> Result<String, String> {
    if require_full_sha
        && (revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()))
    {
        return Err(format!(
            "{label} must be a full 40-character hexadecimal Git commit SHA; received `{revision}`"
        ));
    }
    let type_output = git_output(root, &["cat-file", "-t", revision])?;
    if !type_output.status.success() {
        return Err(format!(
            "{label} `{revision}` is not a valid existing Git object: {}",
            String::from_utf8_lossy(&type_output.stderr).trim()
        ));
    }
    let type_text = String::from_utf8(type_output.stdout)
        .map_err(|error| format!("exact object type for {label} is not UTF-8: {error}"))?;
    let object_type = type_text
        .strip_suffix("\r\n")
        .or_else(|| type_text.strip_suffix('\n'))
        .ok_or_else(|| {
            format!(
                "{label} `{revision}` produced unexpected Git object-type output: {type_text:?}"
            )
        })?;
    if object_type != "commit" {
        return Err(format!(
            "{label} `{revision}` must identify an exact commit object; exact object type is `{object_type}`"
        ));
    }
    let output = git_output(
        root,
        &["rev-parse", "--verify", "--end-of-options", revision],
    )?;
    if !output.status.success() {
        return Err(format!(
            "{label} `{revision}` cannot be resolved as the exact commit object: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let resolved = String::from_utf8(output.stdout)
        .map_err(|error| format!("resolved {label} is not UTF-8: {error}"))?
        .trim()
        .to_ascii_lowercase();
    if resolved.len() != 40 || !resolved.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!(
            "{label} `{revision}` did not resolve to a full Git commit SHA: `{resolved}`"
        ));
    }
    if require_full_sha && resolved != revision.to_ascii_lowercase() {
        return Err(format!(
            "{label} `{revision}` did not resolve to that exact Git object: `{resolved}`"
        ));
    }
    Ok(resolved)
}

fn validate_governed_implementation_ancestry(
    root: &Path,
    evaluated_revision: &str,
) -> Result<(), String> {
    for governed in GOVERNED_IMPLEMENTATION_COMMITS {
        let output = git_output(
            root,
            &["merge-base", "--is-ancestor", governed, evaluated_revision],
        )?;
        match output.status.code() {
            Some(0) => {}
            Some(1) => {
                return Err(format!(
                    "evaluated release revision `{evaluated_revision}` does not contain governed implementation commit `{governed}` in its ancestry"
                ));
            }
            code => {
                return Err(format!(
                    "cannot validate governed implementation commit `{governed}` against evaluated revision `{evaluated_revision}`: status={code:?}; stderr={}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }
        }
    }
    Ok(())
}

fn verify_release_status_evidence(root: &Path) -> Result<(), String> {
    let relative = "docs/release/v0.1.0-alpha.1-status.md";
    let text = fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))?
        .replace("\r\n", "\n");
    let start_count = text.matches(EVIDENCE_START).count();
    let end_count = text.matches(EVIDENCE_END).count();
    if start_count != 1 || end_count != 1 {
        return Err(format!(
            "{relative}: release evidence markers must appear exactly once (start={start_count}, end={end_count})"
        ));
    }
    let evidence = text
        .split_once(EVIDENCE_START)
        .and_then(|(_, tail)| tail.split_once(EVIDENCE_END).map(|(body, _)| body))
        .ok_or_else(|| format!("{relative}: release evidence markers are out of order"))?;

    for required in [
        format!("- Milestone base commit: `{EXPECTED_BASE_COMMIT}`"),
        format!("- Original implementation commit: `{ORIGINAL_IMPLEMENTATION_COMMIT}`"),
        format!("- First corrective commit: `{FIRST_CORRECTIVE_COMMIT}`"),
        format!("- Second corrective commit: `{SECOND_CORRECTIVE_COMMIT}`"),
        format!("- Final parser corrective commit: `{FINAL_PARSER_CORRECTIVE_COMMIT}`"),
        format!("- Evaluated revision environment: `{EVALUATED_REVISION_ENV}`"),
        format!("- Evaluated base revision environment: `{EVALUATED_BASE_REVISION_ENV}`"),
        "- Pull-request evaluated revision source: `github.event.pull_request.head.sha`"
            .to_string(),
        "- Pull-request evaluated base source: `github.event.pull_request.base.sha`"
            .to_string(),
        "- Revision authority: `validated commit relationships, never checkout commit subjects`"
            .to_string(),
        format!(
            "- Complete public-command fixture scenarios: `{COMPLETE_PUBLIC_COMMAND_FIXTURE_COUNT}`"
        ),
        "- Complete public-command fixture categories: `fence_comment=18; polarity=18; structure_encoding=14; claim_binding=5`".to_string(),
        format!(
            "- Release-identity test classification: `{RELEASE_IDENTITY_TEST_FUNCTION_COUNT} test functions: 11 helper/unit; 12 production-document; 3 production-loader; 1 compiled-process integration; 1 complete public-command; 1 production-command Git trust-boundary`"
        ),
        "- Scope: `minimal_release_identity`".to_string(),
        "- Registry facts: `152 research_required; 152 blocked; 0 publicly executable`"
            .to_string(),
        "- CLI self-check: `14 passed; 0 failed`".to_string(),
        "- Fresh-runner CI: `pending corrective push and rerun`".to_string(),
        "- Threat-model boundary: `local_scripts_are_conveniences_not_security_sandboxes`"
            .to_string(),
        "- Root Cargo.lock in committed Git tree: `absent`".to_string(),
        "- Pre-existing ignored local Cargo.lock: `not part of the release commit; preserve bytes; cleanup requires separate user authorization`".to_string(),
        "- Excluded work: `packaging, signing, tagging, publication, formula promotion`"
            .to_string(),
    ] {
        if !evidence.contains(&required) {
            return Err(format!(
                "{relative}: objective evidence is missing exact line `{required}`"
            ));
        }
    }

    let manifest = release_manifest::load_release_manifest(root)?;
    let expected_packages = manifest
        .packages
        .iter()
        .map(|package| {
            format!(
                "- Package: `{}` | `{}`",
                package.name, package.manifest_path
            )
        })
        .collect::<BTreeSet<_>>();
    require_exact_prefixed_evidence(relative, evidence, "- Package: `", &expected_packages)?;

    let original_files = git_lines(
        root,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            ORIGINAL_IMPLEMENTATION_COMMIT,
        ],
    )?
    .into_iter()
    .map(|path| format!("- Original file: `{path}`"))
    .collect::<BTreeSet<_>>();
    require_exact_prefixed_evidence(relative, evidence, "- Original file: `", &original_files)?;

    let original_stat = git_text(
        root,
        &[
            "show",
            "--shortstat",
            "--format=",
            ORIGINAL_IMPLEMENTATION_COMMIT,
        ],
    )?;
    require_evidence_stat(
        relative,
        evidence,
        "Original Git statistics",
        &original_stat,
    )?;

    let first_corrective_files = git_lines(
        root,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            FIRST_CORRECTIVE_COMMIT,
        ],
    )?
    .into_iter()
    .map(|path| format!("- First corrective file: `{path}`"))
    .collect::<BTreeSet<_>>();
    require_exact_prefixed_evidence(
        relative,
        evidence,
        "- First corrective file: `",
        &first_corrective_files,
    )?;

    let first_corrective_stat = git_text(
        root,
        &["show", "--shortstat", "--format=", FIRST_CORRECTIVE_COMMIT],
    )?;
    require_evidence_stat(
        relative,
        evidence,
        "First corrective Git statistics",
        &first_corrective_stat,
    )?;

    let second_corrective_files = git_lines(
        root,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            SECOND_CORRECTIVE_COMMIT,
        ],
    )?
    .into_iter()
    .map(|path| format!("- Second corrective file: `{path}`"))
    .collect::<BTreeSet<_>>();
    require_exact_prefixed_evidence(
        relative,
        evidence,
        "- Second corrective file: `",
        &second_corrective_files,
    )?;

    let second_corrective_stat = git_text(
        root,
        &["show", "--shortstat", "--format=", SECOND_CORRECTIVE_COMMIT],
    )?;
    require_evidence_stat(
        relative,
        evidence,
        "Second corrective Git statistics",
        &second_corrective_stat,
    )?;

    let final_parser_corrective_files = git_lines(
        root,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "-r",
            FINAL_PARSER_CORRECTIVE_COMMIT,
        ],
    )?
    .into_iter()
    .map(|path| format!("- Final parser corrective file: `{path}`"))
    .collect::<BTreeSet<_>>();
    require_exact_prefixed_evidence(
        relative,
        evidence,
        "- Final parser corrective file: `",
        &final_parser_corrective_files,
    )?;

    let final_parser_corrective_stat = git_text(
        root,
        &[
            "show",
            "--shortstat",
            "--format=",
            FINAL_PARSER_CORRECTIVE_COMMIT,
        ],
    )?;
    require_evidence_stat(
        relative,
        evidence,
        "Final parser corrective Git statistics",
        &final_parser_corrective_stat,
    )?;

    let cumulative_stat = git_text(
        root,
        &[
            "diff",
            "--shortstat",
            EXPECTED_BASE_COMMIT,
            FINAL_PARSER_CORRECTIVE_COMMIT,
            "--",
        ],
    )?;
    require_evidence_stat(
        relative,
        evidence,
        "Parser milestone cumulative Git statistics",
        &cumulative_stat,
    )?;
    Ok(())
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
        .map_err(|error| format!("cannot execute Git for release evidence: {error}"))
}

fn git_text(root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = git_output(root, arguments)?;
    if !output.status.success() {
        return Err(format!(
            "Git release-evidence command failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_string())
        .map_err(|error| format!("Git release-evidence output is invalid UTF-8: {error}"))
}

fn git_lines(root: &Path, arguments: &[&str]) -> Result<Vec<String>, String> {
    Ok(git_text(root, arguments)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().replace('\\', "/"))
        .collect())
}

fn require_exact_prefixed_evidence(
    path: &str,
    evidence: &str,
    prefix: &str,
    expected: &BTreeSet<String>,
) -> Result<(), String> {
    let actual = evidence
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with(prefix))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if &actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{path}: evidence inventory `{prefix}` mismatch: expected={expected:?}; actual={actual:?}"
        ))
    }
}

fn require_evidence_stat(
    path: &str,
    evidence: &str,
    label: &str,
    stat: &str,
) -> Result<(), String> {
    let normalized = stat.split_whitespace().collect::<Vec<_>>().join(" ");
    let expected = format!("- {label}: `{normalized}`");
    if evidence.contains(&expected) {
        Ok(())
    } else {
        Err(format!(
            "{path}: objective evidence is missing derived statistic `{expected}`"
        ))
    }
}

fn verify_compiled_cli(root: &Path, target_directory: &Path) -> Result<(), String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let build = Command::new(cargo)
        .current_dir(root)
        .args(["build", "-p", "aero-codex-cli"])
        .output()
        .map_err(|error| format!("cannot build production CLI: {error}"))?;
    if !build.status.success() {
        return Err(format!(
            "production CLI build failed with {}: {}",
            build.status,
            String::from_utf8_lossy(&build.stderr).trim()
        ));
    }
    let mut binary = target_directory.join("debug/aerocodex");
    if cfg!(windows) {
        binary.set_extension("exe");
    }
    if !binary.is_file() {
        return Err(format!(
            "compiled CLI binary is missing: {}",
            binary.display()
        ));
    }
    let external = ExternalCliDir::create()?;
    let copied = external.path.join(binary.file_name().unwrap_or_default());
    fs::copy(&binary, &copied).map_err(|error| {
        format!(
            "cannot copy production CLI from {} to {}: {error}",
            binary.display(),
            copied.display()
        )
    })?;
    for forbidden in [
        ".git",
        "Cargo.toml",
        "docs",
        "release",
        "release-manifest.toml",
        "v0.1.0-alpha.1.toml",
    ] {
        if external.path.join(forbidden).exists() {
            return Err(format!(
                "external CLI directory unexpectedly contains `{forbidden}`"
            ));
        }
    }

    let standard = run_cli(&copied, &external.path, &["--version"], None)?;
    require_success(&standard, "copied CLI --version")?;
    let standard_stdout = output_utf8(&standard.stdout, "--version stdout")?;
    let standard_stderr = output_utf8(&standard.stderr, "--version stderr")?;
    if standard_stdout != format!("{EXPECTED_PROGRAM} {EXPECTED_VERSION}\n")
        || !standard_stderr.is_empty()
    {
        return Err(format!(
            "standard --version contract drift: stdout={standard_stdout:?}; stderr={standard_stderr:?}"
        ));
    }

    let version = run_cli(&copied, &external.path, &["version", "--json"], None)?;
    require_success(&version, "copied CLI version --json")?;
    validate_cli_version_json(output_utf8(&version.stdout, "version JSON stdout")?)?;
    require_empty_stderr(&version, "version --json")?;

    let status = run_cli(
        &copied,
        &external.path,
        &["formula", "status-report", "--json"],
        None,
    )?;
    require_success(&status, "copied CLI formula status-report --json")?;
    validate_cli_status_json(output_utf8(&status.stdout, "status JSON stdout")?)?;
    require_empty_stderr(&status, "formula status-report --json")?;

    let self_check = run_cli(&copied, &external.path, &["self-check", "--json"], None)?;
    require_success(&self_check, "copied CLI self-check --json")?;
    validate_self_check_json(
        output_utf8(&self_check.stdout, "self-check JSON stdout")?,
        false,
    )?;
    require_empty_stderr(&self_check, "self-check --json")?;

    let forced = run_cli(
        &copied,
        &external.path,
        &["self-check", "--json"],
        Some((FORCE_SELF_CHECK_FAILURE_ENV, "1")),
    )?;
    if forced.status.code() != Some(5) {
        return Err(format!(
            "forced self-check failure returned {:?}, expected exit code 5",
            forced.status.code()
        ));
    }
    validate_self_check_json(
        output_utf8(&forced.stdout, "forced self-check stdout")?,
        true,
    )?;
    require_empty_stderr(&forced, "forced self-check")?;
    Ok(())
}

fn run_cli(
    binary: &Path,
    working_directory: &Path,
    arguments: &[&str],
    environment: Option<(&str, &str)>,
) -> Result<Output, String> {
    const MAX_ATTEMPTS: usize = 5;

    let mut command = Command::new(binary);
    command
        .current_dir(working_directory)
        .args(arguments)
        .env_remove(FORCE_SELF_CHECK_FAILURE_ENV);
    if let Some((key, value)) = environment {
        command.env(key, value);
    }
    for attempt in 1..=MAX_ATTEMPTS {
        match command.output() {
            Err(error)
                if cfg!(unix)
                    && error.raw_os_error() == Some(26)
                    && attempt < MAX_ATTEMPTS =>
            {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            result => {
                return result.map_err(|error| {
                    format!("cannot execute copied CLI {}: {error}", binary.display())
                });
            }
        }
    }
    unreachable!("bounded copied-CLI execution attempts must return")
}

fn require_success(output: &Output, context: &str) -> Result<(), String> {
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{context} failed with {}: stdout={:?}; stderr={:?}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn output_utf8<'a>(bytes: &'a [u8], context: &str) -> Result<&'a str, String> {
    std::str::from_utf8(bytes).map_err(|error| format!("{context} is invalid UTF-8: {error}"))
}

fn require_empty_stderr(output: &Output, context: &str) -> Result<(), String> {
    let stderr = output_utf8(&output.stderr, "CLI stderr")?;
    if stderr.is_empty() {
        Ok(())
    } else {
        Err(format!("{context} unexpectedly wrote stderr: {stderr:?}"))
    }
}

fn parsed_cli_object(text: &str, context: &str) -> Result<BTreeMap<String, JsonValue>, String> {
    if text.to_ascii_lowercase().contains("beta 1") || text.contains("beta1-concept") {
        return Err(format!(
            "{context} exposes legacy Beta 1 as current runtime identity"
        ));
    }
    match formula_registry::rust::parse_json_value(text)
        .map_err(|error| format!("cannot parse {context}: {error}"))?
    {
        JsonValue::Object(object) => Ok(object),
        _ => Err(format!("{context} must be a JSON object")),
    }
}

fn validate_common_cli_identity(
    object: &BTreeMap<String, JsonValue>,
    context: &str,
) -> Result<(), String> {
    for (field, expected) in [
        ("program_name", EXPECTED_PROGRAM),
        ("semantic_version", EXPECTED_VERSION),
        ("release_tier", EXPECTED_TIER),
        ("release_tier_display", EXPECTED_TIER_DISPLAY),
    ] {
        let actual = json_string_field(object, field)?;
        if actual != expected {
            return Err(format!(
                "{context} field `{field}` is `{actual}`, expected `{expected}`"
            ));
        }
    }
    Ok(())
}

fn validate_cli_version_json(text: &str) -> Result<(), String> {
    let object = parsed_cli_object(text, "CLI version JSON")?;
    validate_common_cli_identity(&object, "CLI version JSON")?;
    for (field, expected) in [
        ("workspace_package_count", "14"),
        ("registry_formula_count", "152"),
        ("blocked_formula_count", "152"),
        ("public_executable_formula_count", "0"),
    ] {
        let actual = json_number_field(&object, field)?;
        if actual != expected {
            return Err(format!(
                "CLI version JSON field `{field}` is `{actual}`, expected `{expected}`"
            ));
        }
    }
    Ok(())
}

fn validate_cli_status_json(text: &str) -> Result<(), String> {
    let object = parsed_cli_object(text, "CLI status JSON")?;
    validate_common_cli_identity(&object, "CLI status JSON")?;
    for (field, expected) in [
        ("registry_formula_count", "152"),
        ("blocked_formula_count", "152"),
        ("public_executable_formula_count", "0"),
    ] {
        let actual = json_number_field(&object, field)?;
        if actual != expected {
            return Err(format!(
                "CLI status JSON field `{field}` is `{actual}`, expected `{expected}`"
            ));
        }
    }
    Ok(())
}

fn validate_self_check_json(text: &str, forced_failure: bool) -> Result<(), String> {
    let object = parsed_cli_object(text, "CLI self-check JSON")?;
    validate_common_cli_identity(&object, "CLI self-check JSON")?;
    let expected_passed = if forced_failure { "13" } else { "14" };
    let expected_failed = if forced_failure { "1" } else { "0" };
    if json_number_field(&object, "passed")? != expected_passed
        || json_number_field(&object, "failed")? != expected_failed
    {
        return Err(format!(
            "self-check count drift: expected passed={expected_passed}, failed={expected_failed}"
        ));
    }
    if forced_failure {
        let error = object
            .get("error")
            .ok_or_else(|| "forced self-check JSON is missing error".to_string())?;
        let error = json_object(error, "forced self-check error")?;
        if json_string_field(error, "code")? != "self_check_failed" {
            return Err("forced self-check error code must be self_check_failed".to_string());
        }
        let message = json_string_field(error, "message")?;
        if !message.contains("AeroCodex self-check") || message.contains("Beta") {
            return Err(format!(
                "forced self-check wording is not current: `{message}`"
            ));
        }
    }
    Ok(())
}

struct ExternalCliDir {
    path: PathBuf,
}

impl ExternalCliDir {
    fn create() -> Result<Self, String> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("system clock before UNIX_EPOCH: {error}"))?
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "aerocodex_release_identity_cli_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir(&path).map_err(|error| {
            format!(
                "cannot create external CLI verification directory {}: {error}",
                path.display()
            )
        })?;
        Ok(Self { path })
    }
}

impl Drop for ExternalCliDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula_registry::rust::CheckedFormulaIdentityRow;

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn create(label: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("test clock should follow UNIX epoch")
                .as_nanos();
            let path = env::temp_dir().join(format!(
                "aerocodex_release_identity_test_{label}_{}_{}",
                std::process::id(),
                nonce
            ));
            fs::create_dir_all(&path).expect("temporary test directory should be created");
            Self { path }
        }

        fn write(&self, relative: &str, contents: impl AsRef<[u8]>) {
            let path = self.path.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("fixture parent should be created");
            }
            fs::write(path, contents).expect("fixture file should be written");
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask should have a repository parent")
            .to_path_buf()
    }

    fn replace_once(text: &str, old: &str, new: &str) -> String {
        assert_eq!(
            text.matches(old).count(),
            1,
            "fixture replacement must be unique: {old:?}"
        );
        text.replacen(old, new, 1)
    }

    fn replace_once_in_disposable_fixture(text: &str, old: &str, new: &str) -> String {
        let normalized_text = text.replace("\r\n", "\n");
        let normalized_old = old.replace("\r\n", "\n");
        let normalized_new = new.replace("\r\n", "\n");
        replace_once(&normalized_text, &normalized_old, &normalized_new)
    }

    fn document_with_prose(prose: &str) -> String {
        format!("{}\n{prose}\n", identity_document(IDENTITY_BODY))
    }

    fn assert_document_error(prose: &str, required: &[&str]) {
        let document = document_with_prose(prose);
        let error = verify_identity_document_bytes("fixture.md", document.as_bytes())
            .expect_err("document fixture should fail");
        for value in required {
            assert!(error.contains(value), "{error:?} did not contain {value:?}");
        }
    }

    fn copy_working_tree(source: &Path, destination: &Path) {
        let mut entries = fs::read_dir(source)
            .expect("source directory should be readable")
            .collect::<Result<Vec<_>, _>>()
            .expect("source entries should be readable");
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let name = entry.file_name();
            if matches!(name.to_str(), Some(".git" | "target" | "Cargo.lock")) {
                continue;
            }
            let source_path = entry.path();
            let destination_path = destination.join(&name);
            if source_path.is_dir() {
                fs::create_dir_all(&destination_path)
                    .expect("destination directory should be created");
                copy_working_tree(&source_path, &destination_path);
            } else {
                fs::copy(&source_path, &destination_path)
                    .expect("working-tree fixture file should be copied");
            }
        }
    }

    fn complete_repository_fixture() -> TestDirectory {
        let source = repository_root();
        let fixture = TestDirectory::create("complete_repository");
        let output = Command::new("git")
            .args(["clone", "--quiet"])
            .arg(&source)
            .arg(&fixture.path)
            .output()
            .expect("local fixture clone should execute");
        assert!(
            output.status.success(),
            "local fixture clone failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        copy_working_tree(&source, &fixture.path);
        let obsolete_manifest = fixture
            .path
            .join(release_manifest::OBSOLETE_RELEASE_MANIFEST_PATH);
        if !source
            .join(release_manifest::OBSOLETE_RELEASE_MANIFEST_PATH)
            .exists()
            && obsolete_manifest.exists()
        {
            fs::remove_file(&obsolete_manifest)
                .expect("obsolete release authority should be removed from fixture clone");
        }
        stage_and_commit_fixture_changes(&fixture.path);
        fixture
    }

    fn stage_and_commit_fixture_changes(path: &Path) -> bool {
        let add = Command::new("git")
            .arg("-C")
            .arg(path)
            .args(["add", "-A", "--", "."])
            .output()
            .expect("fixture Git add should execute");
        assert!(
            add.status.success(),
            "fixture Git add failed: stdout={} stderr={}",
            String::from_utf8_lossy(&add.stdout),
            String::from_utf8_lossy(&add.stderr)
        );

        let staged = Command::new("git")
            .arg("-C")
            .arg(path)
            .args(["diff", "--cached", "--quiet", "--exit-code"])
            .output()
            .expect("fixture staged-diff check should execute");
        match staged.status.code() {
            Some(0) => false,
            Some(1) => {
                let commit = Command::new("git")
                    .args([
                        "-c",
                        "user.name=AeroCodex fixture",
                        "-c",
                        "user.email=fixture@aerocodex.invalid",
                        "-C",
                    ])
                    .arg(path)
                    .args([
                        "commit",
                        "--quiet",
                        "-m",
                        "fixture: synchronize governed working tree",
                    ])
                    .output()
                    .expect("fixture Git commit should execute");
                assert!(
                    commit.status.success(),
                    "fixture Git commit failed: stdout={} stderr={}",
                    String::from_utf8_lossy(&commit.stdout),
                    String::from_utf8_lossy(&commit.stderr)
                );
                true
            }
            code => panic!(
                "fixture staged-diff check failed: status={code:?} stdout={} stderr={}",
                String::from_utf8_lossy(&staged.stdout),
                String::from_utf8_lossy(&staged.stderr)
            ),
        }
    }

    fn fixture_head(path: &Path) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(path)
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("fixture HEAD lookup should execute");
        assert!(
            output.status.success(),
            "fixture HEAD lookup failed: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("fixture HEAD should be UTF-8")
            .trim()
            .to_string()
    }

    fn fixture_git(path: &Path, arguments: &[&str]) -> String {
        git_text(path, arguments)
            .unwrap_or_else(|error| panic!("fixture Git command {arguments:?} failed: {error}"))
    }

    fn commit_revision_fixture(fixture: &TestDirectory, contents: &str, message: &str) -> String {
        fixture.write("revision-state.txt", contents);
        fixture_git(&fixture.path, &["add", "--", "revision-state.txt"]);
        fixture_git(&fixture.path, &["commit", "--quiet", "-m", message]);
        fixture_head(&fixture.path)
    }

    struct RevisionRelationshipFixture {
        directory: TestDirectory,
        base: String,
        pull_request_head: String,
        unrelated: String,
        synthetic_merge: String,
        wrong_synthetic_merge: String,
        non_commit: String,
    }

    fn revision_relationship_fixture() -> RevisionRelationshipFixture {
        let directory = TestDirectory::create("revision_relationships");
        fixture_git(
            &directory.path,
            &["init", "--quiet", "--initial-branch=release"],
        );
        fixture_git(
            &directory.path,
            &["config", "user.name", "AeroCodex revision fixture"],
        );
        fixture_git(
            &directory.path,
            &["config", "user.email", "revision-fixture@aerocodex.invalid"],
        );
        let base = commit_revision_fixture(&directory, "base\n", "fixture: release base");

        fixture_git(&directory.path, &["switch", "--quiet", "-c", "pr-head"]);
        let pull_request_head = commit_revision_fixture(
            &directory,
            "pull request head\n",
            "fix: verify release identity against pull request head",
        );

        fixture_git(
            &directory.path,
            &["switch", "--quiet", "-c", "unrelated", &base],
        );
        let unrelated =
            commit_revision_fixture(&directory, "unrelated\n", "fixture: unrelated commit");

        fixture_git(&directory.path, &["switch", "--quiet", "release"]);
        fixture_git(
            &directory.path,
            &[
                "merge",
                "--quiet",
                "--no-ff",
                "pr-head",
                "-m",
                "Merge pull request #49 from codex/release-identity-minimal",
            ],
        );
        let synthetic_merge = fixture_head(&directory.path);
        let non_commit = fixture_git(
            &directory.path,
            &["rev-parse", &format!("{synthetic_merge}^{{tree}}")],
        );

        fixture_git(&directory.path, &["switch", "--quiet", "--detach", &base]);
        fixture_git(
            &directory.path,
            &[
                "merge",
                "--quiet",
                "--no-ff",
                "unrelated",
                "-m",
                "Merge a different proposed head",
            ],
        );
        let wrong_synthetic_merge = fixture_head(&directory.path);

        RevisionRelationshipFixture {
            directory,
            base,
            pull_request_head,
            unrelated,
            synthetic_merge,
            wrong_synthetic_merge,
            non_commit,
        }
    }

    struct CompleteDocumentScenario {
        name: &'static str,
        contents: Vec<u8>,
        expected_error: Option<&'static str>,
    }

    impl CompleteDocumentScenario {
        fn reject_prose(name: &'static str, prose: &str, expected_error: &'static str) -> Self {
            Self {
                name,
                contents: document_with_prose(prose).into_bytes(),
                expected_error: Some(expected_error),
            }
        }

        fn reject_document(
            name: &'static str,
            contents: impl Into<Vec<u8>>,
            expected_error: &'static str,
        ) -> Self {
            Self {
                name,
                contents: contents.into(),
                expected_error: Some(expected_error),
            }
        }

        fn accept_prose(name: &'static str, prose: &str) -> Self {
            Self {
                name,
                contents: document_with_prose(prose).into_bytes(),
                expected_error: None,
            }
        }
    }

    fn write_package(root: &TestDirectory, path: &str, name: &str, version: &str) {
        let version_line = if version == "workspace" {
            "version.workspace = true".to_string()
        } else {
            format!("version = \"{version}\"")
        };
        root.write(
            &format!("{path}/Cargo.toml"),
            format!("[package]\nname = \"{name}\"\n{version_line}\nedition.workspace = true\n"),
        );
        root.write(&format!("{path}/src/lib.rs"), "pub fn fixture() {}\n");
    }

    fn package_workspace_fixture() -> (TestDirectory, Vec<ReleasePackage>) {
        let fixture = TestDirectory::create("cargo_workspace");
        let mut members = String::new();
        let mut governed = Vec::new();
        for index in 0..EXPECTED_PACKAGE_COUNT {
            let name = format!("fixture-{index:02}");
            let path = format!("crates/{name}");
            members.push_str(&format!("    \"{path}\",\n"));
            write_package(&fixture, &path, &name, "workspace");
            governed.push(package(&name, &format!("{path}/Cargo.toml")));
        }
        fixture.write(
            "Cargo.toml",
            format!(
                "[workspace]\nresolver = \"2\"\nmembers = [\n{members}]\n\n[workspace.package]\nversion = \"{EXPECTED_VERSION}\"\nedition = \"2021\"\n"
            ),
        );
        (fixture, governed)
    }

    fn validate_package_fixture(
        fixture: &TestDirectory,
        governed: &[ReleasePackage],
    ) -> Result<(), String> {
        let metadata = load_cargo_metadata(&fixture.path)?;
        let discovered = discover_package_manifests(&fixture.path, EXPECTED_VERSION)?;
        validate_package_sets(governed, EXPECTED_VERSION, &metadata.packages, &discovered)
    }

    fn registry_fixture() -> TestDirectory {
        let fixture = TestDirectory::create("registry");
        let root = repository_root();
        fixture.write(
            "generated/formula_registry.json",
            fs::read(root.join("generated/formula_registry.json"))
                .expect("checked registry should be readable"),
        );
        fixture.write(
            "generated/formula_registry.sha256",
            fs::read(root.join("generated/formula_registry.sha256"))
                .expect("checked registry sidecar should be readable"),
        );
        fixture.write(
            "release/release-manifest.toml",
            fs::read(root.join("release/release-manifest.toml"))
                .expect("release manifest should be readable"),
        );
        fixture
    }

    fn write_checked_registry(fixture: &TestDirectory, text: &str) {
        fixture.write("generated/formula_registry.json", text);
        let (canonical, _) = crate::checksums::canonical_checksum_bytes(text.as_bytes());
        let digest = crate::equation_batch::generate::sha256_hex(canonical.as_ref());
        fixture.write(
            "generated/formula_registry.sha256",
            format!("{digest}  generated/formula_registry.json\n"),
        );
    }

    fn build_cli_fixture(fixture: &TestDirectory, label: &str) -> PathBuf {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let output = Command::new(cargo)
            .current_dir(&fixture.path)
            .args(["build", "-p", "aero-codex-cli"])
            .output()
            .expect("fixture CLI build should execute");
        assert!(
            output.status.success(),
            "fixture CLI build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let mut built = fixture.path.join("target/debug/aerocodex");
        if cfg!(windows) {
            built.set_extension("exe");
        }
        let mut saved = fixture.path.join(format!("compiled-{label}"));
        if cfg!(windows) {
            saved.set_extension("exe");
        }
        fs::copy(&built, &saved).expect("compiled fixture binary should be saved");
        saved
    }

    fn fixture_commit_tree(path: &Path, tree: &str, parents: &[&str], message: &str) -> String {
        let mut arguments = vec![
            "-c".to_string(),
            "user.name=AeroCodex revision fixture".to_string(),
            "-c".to_string(),
            "user.email=revision-fixture@aerocodex.invalid".to_string(),
            "commit-tree".to_string(),
            tree.to_string(),
        ];
        for parent in parents {
            arguments.push("-p".to_string());
            arguments.push((*parent).to_string());
        }
        arguments.push("-m".to_string());
        arguments.push(message.to_string());
        let references = arguments.iter().map(String::as_str).collect::<Vec<_>>();
        fixture_git(path, &references)
    }

    fn annotated_tag_object(path: &Path, name: &str, target: &str) -> String {
        fixture_git(
            path,
            &[
                "-c",
                "user.name=AeroCodex revision fixture",
                "-c",
                "user.email=revision-fixture@aerocodex.invalid",
                "tag",
                "--annotate",
                "--message=fixture annotated tag",
                name,
                target,
            ],
        );
        fixture_git(path, &["rev-parse", &format!("refs/tags/{name}")])
    }

    fn build_xtask_fixture(root: &Path) -> PathBuf {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let output = Command::new(cargo)
            .current_dir(root)
            .args(["build", "-p", "xtask"])
            .env_remove(EVALUATED_REVISION_ENV)
            .env_remove(EVALUATED_BASE_REVISION_ENV)
            .output()
            .expect("fixture xtask build should execute");
        assert!(
            output.status.success(),
            "fixture xtask build failed: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let mut binary = root.join("target/debug/xtask");
        if cfg!(windows) {
            binary.set_extension("exe");
        }
        assert!(binary.is_file(), "fixture xtask binary should exist");
        binary
    }

    fn run_release_identity_fixture(
        binary: &Path,
        root: &Path,
        head: Option<&str>,
        base: Option<&str>,
    ) -> Output {
        let mut command = Command::new(binary);
        command
            .current_dir(root)
            .arg("verify-release-identity")
            .env_remove(EVALUATED_REVISION_ENV)
            .env_remove(EVALUATED_BASE_REVISION_ENV);
        if let Some(head) = head {
            command.env(EVALUATED_REVISION_ENV, head);
        }
        if let Some(base) = base {
            command.env(EVALUATED_BASE_REVISION_ENV, base);
        }
        command
            .output()
            .expect("fixture release-identity command should execute")
    }

    fn release_identity_fixture_output(output: &Output) -> String {
        format!(
            "stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }

    fn identity_document(body: &str) -> String {
        format!("# Current identity\n\n{IDENTITY_START}\n{body}\n{IDENTITY_END}\n")
    }

    fn package(name: &str, path: &str) -> ReleasePackage {
        ReleasePackage {
            name: name.to_string(),
            manifest_path: path.to_string(),
        }
    }

    fn cargo_package(name: &str, version: &str, path: &str) -> CargoPackage {
        CargoPackage {
            id: format!("{name}#{version}"),
            name: name.to_string(),
            version: version.to_string(),
            manifest_path: path.to_string(),
        }
    }

    fn discovered_package(name: &str, version: &str, path: &str) -> DiscoveredPackage {
        DiscoveredPackage {
            name: name.to_string(),
            version: version.to_string(),
            manifest_path: path.to_string(),
            inherits_workspace_version: true,
        }
    }

    fn registry_with(status: &str, policy: &str, ids: &[&str]) -> CheckedFormulaIdentityRegistry {
        let formulas = ids
            .iter()
            .map(|id| CheckedFormulaIdentityRow {
                formula_id: (*id).to_string(),
                status: status.to_string(),
                execution_policy: policy.to_string(),
            })
            .collect::<Vec<_>>();
        CheckedFormulaIdentityRegistry {
            formula_count: formulas.len(),
            formulas,
        }
    }

    #[test]
    fn malformed_semver_and_wrong_alpha_prerelease_fail() {
        assert!(validate_target_semver("0.1")
            .unwrap_err()
            .contains("malformed SemVer"));
        assert!(validate_target_semver("0.1.0")
            .unwrap_err()
            .contains("alpha.1"));
        assert!(validate_target_semver("0.1.0-alpha.2")
            .unwrap_err()
            .contains("alpha.1"));
        validate_target_semver(EXPECTED_VERSION).unwrap();
    }

    #[test]
    fn workspace_manifest_version_mismatch_fails() {
        let error = validate_workspace_and_manifest_versions(EXPECTED_VERSION, "0.1.0-alpha.2")
            .unwrap_err();
        assert!(error.contains("alpha.1") || error.contains("does not match"));
    }

    #[test]
    fn removed_workspace_package_and_extra_discovered_package_fail() {
        let governed = vec![package("one", "crates/one/Cargo.toml")];
        let missing = validate_package_sets(&governed, EXPECTED_VERSION, &[], &[]).unwrap_err();
        assert!(missing.contains("cargo_missing"));

        let cargo = vec![cargo_package(
            "one",
            EXPECTED_VERSION,
            "crates/one/Cargo.toml",
        )];
        let discovered = vec![
            discovered_package("one", EXPECTED_VERSION, "crates/one/Cargo.toml"),
            discovered_package("extra", EXPECTED_VERSION, "crates/extra/Cargo.toml"),
        ];
        let extra =
            validate_package_sets(&governed, EXPECTED_VERSION, &cargo, &discovered).unwrap_err();
        assert!(extra.contains("discovered_extra"));
    }

    #[test]
    fn package_name_and_version_mismatch_fail() {
        let governed = vec![package("one", "crates/one/Cargo.toml")];
        let discovered = vec![discovered_package(
            "one",
            EXPECTED_VERSION,
            "crates/one/Cargo.toml",
        )];
        let wrong_version = vec![cargo_package(
            "one",
            "0.1.0-alpha.2",
            "crates/one/Cargo.toml",
        )];
        assert!(
            validate_package_sets(&governed, EXPECTED_VERSION, &wrong_version, &discovered)
                .unwrap_err()
                .contains("version mismatch")
        );
        let wrong_name = vec![cargo_package(
            "two",
            EXPECTED_VERSION,
            "crates/one/Cargo.toml",
        )];
        assert!(
            validate_package_sets(&governed, EXPECTED_VERSION, &wrong_name, &discovered)
                .unwrap_err()
                .contains("name mismatch")
        );
    }

    #[test]
    fn production_cargo_metadata_json_parser_rejects_malformed_input() {
        let error = parse_cargo_metadata(Path::new("."), "{not-json}").unwrap_err();
        assert!(error.contains("cannot parse cargo metadata JSON"));
    }

    #[test]
    fn registry_duplicate_status_policy_and_public_changes_fail() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let manifest = release_manifest::load_release_manifest(
            root.parent().expect("xtask has a repository parent"),
        )
        .expect("checked-in release manifest should parse");
        let duplicate = registry_with("research_required", "blocked", &["same", "same"]);
        assert!(summarize_registry(&duplicate)
            .unwrap_err()
            .contains("duplicate formula ID"));
        let status =
            summarize_registry(&registry_with("equation_traceable", "blocked", &["one"])).unwrap();
        assert_eq!(status.research_required, 0);
        assert!(validate_registry_summary(&manifest, status).is_err());
        let policy = summarize_registry(&registry_with(
            "research_required",
            "preliminary_flag_required",
            &["one"],
        ))
        .unwrap();
        assert_eq!(policy.blocked, 0);
        assert!(validate_registry_summary(&manifest, policy).is_err());
        let public = summarize_registry(&registry_with(
            "implementation_verified",
            "normal_research",
            &["one"],
        ))
        .unwrap();
        assert_eq!(public.publicly_executable, 1);
        assert!(validate_registry_summary(&manifest, public).is_err());
    }

    #[test]
    fn malformed_registry_uses_production_json_parser() {
        let error = formula_registry::rust::parse_json_value("{\"formulas\":[}").unwrap_err();
        assert!(error.contains("unexpected JSON character"));
    }

    #[test]
    fn missing_registry_fails_the_production_registry_check() {
        let root = ExternalCliDir::create().expect("temporary registry fixture should be created");
        let error = formula_registry::check::run_check_command(&root.path, &CheckOptions)
            .expect_err("missing production registry must fail");
        assert!(error.contains("generated/formula_registry.json is missing or unreadable"));
    }

    #[test]
    fn invalid_release_tier_fails() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root = root.parent().expect("xtask has a repository parent");
        let mut manifest = release_manifest::load_release_manifest(root)
            .expect("checked-in release manifest should parse");
        manifest.release_tier = "production".to_string();
        assert!(validate_manifest_identity(&manifest)
            .unwrap_err()
            .contains("invalid release tier"));
    }

    #[test]
    fn exact_document_identity_accepts_lf_and_crlf() {
        let lf = identity_document(IDENTITY_BODY);
        verify_identity_document_bytes("fixture.md", lf.as_bytes()).unwrap();
        let crlf = lf.replace('\n', "\r\n");
        verify_identity_document_bytes("fixture.md", crlf.as_bytes()).unwrap();
    }

    #[test]
    fn fenced_examples_are_ignored_unless_explicitly_authoritative() {
        let ignored = format!(
            "{}\n\n```text\nThe current release version is 0.1.0-alpha.2.\n```\n",
            identity_document(IDENTITY_BODY)
        );
        verify_identity_document_bytes("fixture.md", ignored.as_bytes()).unwrap();

        let authoritative = format!(
            "{}\n\n<!-- aerocodex-authoritative-example -->\n```text\nThe current release version is 0.1.0-alpha.2.\n```\n",
            identity_document(IDENTITY_BODY)
        );
        assert!(
            verify_identity_document_bytes("fixture.md", authoritative.as_bytes())
                .unwrap_err()
                .contains("0.1.0-alpha.2")
        );
    }

    #[test]
    fn competing_current_version_tier_and_multiline_claim_fail_with_source_line() {
        let version = format!(
            "{}\n\nThe current release version\nis 0.1.0-alpha.2.\n",
            identity_document(IDENTITY_BODY)
        );
        let error = verify_identity_document_bytes("fixture.md", version.as_bytes()).unwrap_err();
        assert!(error.contains("fixture.md:") && error.contains("0.1.0-alpha.2"));

        let tier = format!(
            "{}\n\nThe current release tier is production.\n",
            identity_document(IDENTITY_BODY)
        );
        assert!(
            verify_identity_document_bytes("fixture.md", tier.as_bytes())
                .unwrap_err()
                .contains("claim_type=machine_release_tier")
        );
    }

    #[test]
    fn historical_heading_does_not_exempt_later_current_claim() {
        let text = format!(
            "{}\n\n## Historical Beta 1\n\nThe current Cargo version remains 0.0.1.\n",
            identity_document(IDENTITY_BODY)
        );
        assert!(
            verify_identity_document_bytes("fixture.md", text.as_bytes())
                .unwrap_err()
                .contains("0.0.1")
        );
    }

    #[test]
    fn legacy_beta_current_runtime_identity_fails() {
        let text = format!(
            "{}\n\nThe current runtime identity remains beta1-concept.\n",
            identity_document(IDENTITY_BODY)
        );
        assert!(
            verify_identity_document_bytes("fixture.md", text.as_bytes())
                .unwrap_err()
                .contains("asserted=`beta1-concept`")
        );
    }

    #[test]
    fn missing_duplicate_block_invalid_utf8_and_mojibake_fail() {
        assert!(verify_identity_document_bytes("fixture.md", b"# no block\n").is_err());
        let duplicate = format!(
            "{}\n{}",
            identity_document(IDENTITY_BODY),
            identity_document(IDENTITY_BODY)
        );
        assert!(
            verify_identity_document_bytes("fixture.md", duplicate.as_bytes())
                .unwrap_err()
                .contains("exactly once")
        );
        assert!(verify_identity_document_bytes("fixture.md", &[0xff, 0xfe]).is_err());
        let mojibake = identity_document(&IDENTITY_BODY.replace("Alpha", "AlphÃ"));
        assert!(
            verify_identity_document_bytes("fixture.md", mojibake.as_bytes())
                .unwrap_err()
                .contains("mojibake")
        );
    }

    #[test]
    fn fixture_commit_skips_an_empty_normalized_staged_diff() {
        let fixture = TestDirectory::create("empty_fixture_commit");
        let init = Command::new("git")
            .arg("-C")
            .arg(&fixture.path)
            .args(["init", "--quiet"])
            .output()
            .expect("fixture Git init should execute");
        assert!(init.status.success(), "fixture Git init should pass");
        let config = Command::new("git")
            .arg("-C")
            .arg(&fixture.path)
            .args(["config", "core.autocrlf", "true"])
            .output()
            .expect("fixture Git configuration should execute");
        assert!(
            config.status.success(),
            "fixture Git configuration should pass"
        );

        fixture.write("normalized.txt", "normalized\ncontent\n");
        assert!(stage_and_commit_fixture_changes(&fixture.path));
        let committed_head = fixture_head(&fixture.path);

        fixture.write("normalized.txt", "normalized\r\ncontent\r\n");
        assert!(!stage_and_commit_fixture_changes(&fixture.path));
        assert_eq!(fixture_head(&fixture.path), committed_head);
    }

    #[test]
    fn evaluated_revision_enforces_direct_and_synthetic_merge_relationships() {
        let fixture = revision_relationship_fixture();
        let root = &fixture.directory.path;

        fixture_git(
            root,
            &["switch", "--quiet", "--detach", &fixture.pull_request_head],
        );
        let direct = evaluate_release_revision(root, None, None).unwrap();
        assert_eq!(direct.checkout, fixture.pull_request_head);
        assert_eq!(direct.evaluated, fixture.pull_request_head);
        assert_eq!(direct.mode, EvaluatedRevisionMode::DirectCheckout);

        let direct_pr_head =
            evaluate_release_revision(root, Some(&fixture.pull_request_head), None).unwrap();
        assert_eq!(direct_pr_head.evaluated, fixture.pull_request_head);
        assert_eq!(direct_pr_head.mode, EvaluatedRevisionMode::DirectCheckout);

        fixture_git(
            root,
            &["switch", "--quiet", "--detach", &fixture.synthetic_merge],
        );
        let synthetic =
            evaluate_release_revision(root, Some(&fixture.pull_request_head), Some(&fixture.base))
                .unwrap();
        assert_eq!(synthetic.checkout, fixture.synthetic_merge);
        assert_eq!(synthetic.evaluated, fixture.pull_request_head);
        assert_eq!(
            synthetic.mode,
            EvaluatedRevisionMode::SyntheticPullRequestMerge
        );

        let base_error =
            evaluate_release_revision(root, Some(&fixture.base), Some(&fixture.base)).unwrap_err();
        assert!(
            base_error.contains("not pull-request head parent"),
            "{base_error}"
        );

        let unrelated_error =
            evaluate_release_revision(root, Some(&fixture.unrelated), Some(&fixture.base))
                .unwrap_err();
        assert!(
            unrelated_error.contains("not pull-request head parent"),
            "{unrelated_error}"
        );

        let nonexistent = "1111111111111111111111111111111111111111";
        let nonexistent_error =
            evaluate_release_revision(root, Some(nonexistent), Some(&fixture.base)).unwrap_err();
        assert!(
            nonexistent_error.contains("not a valid existing Git object"),
            "{nonexistent_error}"
        );

        let object_error =
            evaluate_release_revision(root, Some(&fixture.non_commit), Some(&fixture.base))
                .unwrap_err();
        assert!(object_error.contains("exact object type"), "{object_error}");

        fixture_git(
            root,
            &[
                "switch",
                "--quiet",
                "--detach",
                &fixture.wrong_synthetic_merge,
            ],
        );
        let wrong_head_error =
            evaluate_release_revision(root, Some(&fixture.pull_request_head), Some(&fixture.base))
                .unwrap_err();
        assert!(
            wrong_head_error.contains("not pull-request head parent"),
            "{wrong_head_error}"
        );

        fixture_git(
            root,
            &["switch", "--quiet", "--detach", &fixture.synthetic_merge],
        );
        let ordinary_post_merge = evaluate_release_revision(root, None, None).unwrap();
        assert_eq!(ordinary_post_merge.evaluated, fixture.synthetic_merge);
        assert_eq!(
            ordinary_post_merge.mode,
            EvaluatedRevisionMode::DirectCheckout
        );
        assert!(
            fixture_git(root, &["log", "-1", "--format=%s"]).starts_with("Merge pull request #49"),
            "the regression fixture must use a realistic synthetic merge subject"
        );

        let pr_49_regression =
            evaluate_release_revision(root, Some(&fixture.pull_request_head), Some(&fixture.base))
                .unwrap();
        assert_eq!(
            pr_49_regression.mode,
            EvaluatedRevisionMode::SyntheticPullRequestMerge
        );
    }

    #[test]
    fn production_command_authenticates_exact_objects_and_both_merge_parents() {
        let fixture = complete_repository_fixture();
        let root = &fixture.path;
        let fixture_head = fixture_head(root);
        let reviewed_head = "c751919c021fbff5caff19f5285bf662f84184b8";
        let historical_head = FINAL_PARSER_CORRECTIVE_COMMIT;
        let base = EXPECTED_BASE_COMMIT;
        let tree = fixture_git(root, &["rev-parse", &format!("{fixture_head}^{{tree}}")]);
        let blob = fixture_git(root, &["rev-parse", &format!("{fixture_head}:Cargo.toml")]);
        let unrelated = fixture_commit_tree(root, &tree, &[], "fixture: unrelated root");
        let descendant = fixture_commit_tree(
            root,
            &tree,
            &[reviewed_head],
            "fixture: descendant outside the reviewed merge",
        );
        let reviewed_merge = fixture_commit_tree(
            root,
            &tree,
            &[base, reviewed_head],
            "subject text is deliberately non-authoritative",
        );
        let historical_merge = fixture_commit_tree(
            root,
            &tree,
            &[base, historical_head],
            "Merge pull request #49 from codex/release-identity-minimal",
        );
        let wrong_first_parent = fixture_commit_tree(
            root,
            &tree,
            &[&unrelated, reviewed_head],
            "fixture: unauthorized first parent",
        );
        let wrong_second_parent = fixture_commit_tree(
            root,
            &tree,
            &[base, &unrelated],
            "fixture: unauthorized second parent",
        );
        let three_parent_merge = fixture_commit_tree(
            root,
            &tree,
            &[base, reviewed_head, &unrelated],
            "fixture: three parents",
        );
        let head_tag = annotated_tag_object(root, "reviewed-head-object", reviewed_head);
        let base_tag = annotated_tag_object(root, "reviewed-base-object", base);
        fixture_git(root, &["tag", "reviewed-head-lightweight", reviewed_head]);

        let binary = build_xtask_fixture(root);
        let direct = run_release_identity_fixture(&binary, root, None, None);
        assert!(
            direct.status.success(),
            "direct production command failed: {}",
            release_identity_fixture_output(&direct)
        );
        assert!(
            release_identity_fixture_output(&direct).contains("mode:direct_checkout"),
            "{}",
            release_identity_fixture_output(&direct)
        );
        let explicit = run_release_identity_fixture(&binary, root, Some(&fixture_head), Some(""));
        assert!(
            explicit.status.success(),
            "explicit direct production command failed: {}",
            release_identity_fixture_output(&explicit)
        );

        fixture_git(root, &["switch", "--quiet", "--detach", reviewed_head]);
        let direct_tag = run_release_identity_fixture(&binary, root, Some(&head_tag), None);
        assert!(!direct_tag.status.success());
        assert!(
            release_identity_fixture_output(&direct_tag).contains("exact object type is `tag`"),
            "{}",
            release_identity_fixture_output(&direct_tag)
        );

        fixture_git(root, &["switch", "--quiet", "--detach", &reviewed_merge]);
        let synthetic =
            run_release_identity_fixture(&binary, root, Some(reviewed_head), Some(base));
        assert!(
            synthetic.status.success(),
            "trusted synthetic production command failed: {}",
            release_identity_fixture_output(&synthetic)
        );
        assert!(
            release_identity_fixture_output(&synthetic)
                .contains("mode:synthetic_pull_request_merge"),
            "{}",
            release_identity_fixture_output(&synthetic)
        );

        let reject = |head: Option<&str>, base_input: Option<&str>, required: &str| {
            let output = run_release_identity_fixture(&binary, root, head, base_input);
            assert!(
                !output.status.success(),
                "fixture should be rejected: {}",
                release_identity_fixture_output(&output)
            );
            assert!(
                release_identity_fixture_output(&output).contains(required),
                "{} did not contain {required:?}",
                release_identity_fixture_output(&output)
            );
        };

        reject(Some(&head_tag), Some(base), "exact object type is `tag`");
        reject(
            Some(reviewed_head),
            Some(&base_tag),
            "exact object type is `tag`",
        );
        reject(Some(reviewed_head), None, "is required");
        reject(
            Some(reviewed_head),
            Some(""),
            "must be a full 40-character hexadecimal Git commit SHA",
        );
        reject(
            Some(reviewed_head),
            Some("not-a-sha"),
            "must be a full 40-character hexadecimal Git commit SHA",
        );
        reject(
            Some(reviewed_head),
            Some(&base[..12]),
            "must be a full 40-character hexadecimal Git commit SHA",
        );
        reject(
            Some(reviewed_head),
            Some("1111111111111111111111111111111111111111"),
            "not a valid existing Git object",
        );
        reject(Some(&blob), Some(base), "exact object type is `blob`");
        reject(Some(&tree), Some(base), "exact object type is `tree`");
        reject(
            Some(reviewed_head),
            Some(&blob),
            "exact object type is `blob`",
        );
        reject(
            Some(reviewed_head),
            Some(&tree),
            "exact object type is `tree`",
        );
        for invalid in [
            "HEAD",
            "@",
            "reviewed-head-object",
            "reviewed-head-lightweight",
            "c751919c021fbff5caff19f5285bf662f84184b8^{commit}",
        ] {
            reject(
                Some(invalid),
                Some(base),
                "must be a full 40-character hexadecimal Git commit SHA",
            );
            reject(
                Some(reviewed_head),
                Some(invalid),
                "must be a full 40-character hexadecimal Git commit SHA",
            );
        }
        reject(
            Some(&reviewed_head[..12]),
            Some(base),
            "must be a full 40-character hexadecimal Git commit SHA",
        );
        reject(Some(&unrelated), Some(base), "not pull-request head parent");
        reject(
            Some(historical_head),
            Some(base),
            "not pull-request head parent",
        );
        reject(
            Some(&descendant),
            Some(base),
            "not pull-request head parent",
        );
        reject(Some(base), Some(reviewed_head), "not trusted base parent");
        reject(
            Some(reviewed_head),
            Some(reviewed_head),
            "not trusted base parent",
        );

        fixture_git(
            root,
            &["switch", "--quiet", "--detach", &wrong_first_parent],
        );
        reject(Some(reviewed_head), Some(base), "not trusted base parent");

        fixture_git(
            root,
            &["switch", "--quiet", "--detach", &wrong_second_parent],
        );
        reject(
            Some(reviewed_head),
            Some(base),
            "not pull-request head parent",
        );

        fixture_git(
            root,
            &["switch", "--quiet", "--detach", &three_parent_merge],
        );
        reject(
            Some(reviewed_head),
            Some(base),
            "not a two-parent synthetic pull-request merge",
        );

        fixture_git(root, &["switch", "--quiet", "--detach", &reviewed_merge]);
        let post_merge = run_release_identity_fixture(&binary, root, None, None);
        assert!(
            post_merge.status.success(),
            "post-merge direct production command failed: {}",
            release_identity_fixture_output(&post_merge)
        );

        fixture_git(root, &["switch", "--quiet", "--detach", &historical_merge]);
        let historical =
            run_release_identity_fixture(&binary, root, Some(historical_head), Some(base));
        assert!(
            historical.status.success(),
            "historical PR #49 parent shape failed: {}",
            release_identity_fixture_output(&historical)
        );
    }

    #[test]
    fn runtime_version_and_tier_drift_fail_typed_json_validation() {
        let good = format!(
            "{{\"program_name\":\"aerocodex\",\"semantic_version\":\"{EXPECTED_VERSION}\",\"release_tier\":\"{EXPECTED_TIER}\",\"release_tier_display\":\"{EXPECTED_TIER_DISPLAY}\",\"workspace_package_count\":14,\"registry_formula_count\":152,\"blocked_formula_count\":152,\"public_executable_formula_count\":0}}"
        );
        validate_cli_version_json(&good).unwrap();
        assert!(
            validate_cli_version_json(&good.replace(EXPECTED_VERSION, "0.1.0-alpha.2"))
                .unwrap_err()
                .contains("semantic_version")
        );
        assert!(
            validate_cli_version_json(&good.replace(EXPECTED_TIER, "production"))
                .unwrap_err()
                .contains("release_tier")
        );
    }

    // Production-document fixtures: these call the same governed-document scanner used by
    // `verify-release-identity`, with no test-only parser or token shortcut.
    #[test]
    fn markdown_fence_character_length_and_line_rules_are_fail_closed() {
        for prose in [
            "```text\nThe current release version is 0.1.0-alpha.2.\n```",
            "~~~text\nThe current release version is 0.1.0-alpha.2.\n~~~",
            "```text <!--\nThe current release version is 0.1.0-alpha.2.\n-->\n```\nThe current release version is 0.1.0-alpha.1.",
            "~~~text <!--\nThe current release tier is production.\n-->\n~~~\nThe current release tier is research_software_alpha.",
            "```text\n<!-- comment markers stay fenced -->\n``` <!-- not a valid close\nThe current release version is 0.1.0-alpha.2.\n```\nThe current release version is 0.1.0-alpha.1.",
            "```text\n~~~\nThe current release version is 0.1.0-alpha.2.\n```",
            "~~~text\n```\nThe current release version is 0.1.0-alpha.2.\n~~~",
            "````text\n```\nThe current release version is 0.1.0-alpha.2.\n````",
            "````text\nThe current release version is 0.1.0-alpha.2.\n`````",
            "   ```text\nThe current release version is 0.1.0-alpha.2.\n   ```",
            "Inline ````` fence-like code does not open a block; the current release version is 0.1.0-alpha.1.",
        ] {
            verify_identity_document_bytes(
                "fixture.md",
                document_with_prose(prose).as_bytes(),
            )
            .unwrap();
        }

        assert_document_error(
            "````text\n```\nThe current release version is 0.1.0-alpha.2.\n````\nThe current release version is 0.1.0-alpha.2.",
            &["claim_type=semantic_version", "asserted=`0.1.0-alpha.2`"],
        );
        assert_document_error(
            "```text\nThe current release version is 0.1.0-alpha.2.\n```\nThe current release version is 0.1.0-alpha.2.",
            &["claim_type=semantic_version", "asserted=`0.1.0-alpha.2`"],
        );
        assert_document_error(
            "```text\n~~~\nThe current release version is 0.1.0-alpha.2.\n```\nThe current release version is 0.1.0-alpha.2.",
            &["claim_type=semantic_version", "asserted=`0.1.0-alpha.2`"],
        );
        assert_document_error(
            "```text <!--\nbenign fenced content\n```\nThe current release version is 0.1.0-alpha.2.\n-->",
            &["stray HTML comment close marker"],
        );
        assert_document_error(
            "~~~text <!--\nbenign fenced content\n~~~\nThe current release tier is production.\n-->",
            &["stray HTML comment close marker"],
        );
        assert_document_error(
            "```text <!--\nbenign fenced content\n```\nThe current release version is 0.1.0-alpha.2.",
            &["claim_type=semantic_version", "asserted=`0.1.0-alpha.2`"],
        );
        assert_document_error(
            "~~~text <!--\nbenign fenced content\n~~~\nThe current release tier is production.",
            &["claim_type=machine_release_tier", "asserted=`production`"],
        );
        assert_document_error(
            "````text\nThe current release version is 0.1.0-alpha.2.\n```",
            &["unclosed Markdown fence", "delimiter=`", "length=4"],
        );
        assert_document_error(
            "````text <!--\n<!-- fenced marker -->\n```",
            &["unclosed Markdown fence", "delimiter=`", "length=4"],
        );

        let crlf = document_with_prose(
            "~~~text\nThe current release version is 0.1.0-alpha.2.\n~~~\nThe current release version is 0.1.0-alpha.1.",
        )
        .replace('\n', "\r\n");
        verify_identity_document_bytes("fixture.md", crlf.as_bytes()).unwrap();

        let reproduced_crlf = document_with_prose(
            "```text <!--\nbenign fenced content\n```\nThe current release version is 0.1.0-alpha.2.\n-->",
        )
        .replace('\n', "\r\n");
        let error =
            verify_identity_document_bytes("fixture.md", reproduced_crlf.as_bytes()).unwrap_err();
        assert!(error.contains("stray HTML comment close marker"), "{error}");
    }

    #[test]
    fn historical_scope_never_exempts_a_current_claim() {
        for prose in [
            "Historical Beta 1 used 0.0.1; the current release version is 0.1.0-alpha.1.",
            "Previously the tier was beta1-concept. The current release version is 0.1.0-alpha.1.",
            "Historical Beta 1 used 0.0.1 — the current release tier is research_software_alpha.",
            "(Historical version: 0.0.1.) The current display tier is Research Software Alpha.",
            "The current release version is 0.1.0-alpha.1; the historical Beta 1 version was 0.0.1.",
            "The current runtime identity is AeroCodex; Beta 1 is a historical reference.",
            "## Historical Beta 1\n\nThe current release version is 0.1.0-alpha.1.",
            "<!-- aerocodex-historical:start -->\nThe current release version was 0.0.1.\n<!-- aerocodex-historical:end -->\n\nThe current release version is 0.1.0-alpha.1.",
        ] {
            verify_identity_document_bytes(
                "docs/beta1/fixture.md",
                document_with_prose(prose).as_bytes(),
            )
            .unwrap();
        }

        for (prose, asserted) in [
            (
                "Historical Beta 1 used 0.0.1; the current release version is 0.1.0-alpha.2.",
                "0.1.0-alpha.2",
            ),
            (
                "The historical tier was beta1-concept; the current release version is 0.2.",
                "0.2",
            ),
            (
                "The historical version was 0.0.1; the current release tier is production.",
                "production",
            ),
            (
                "Historical Beta 1 used 0.0.1. The current runtime identity is beta1-concept.",
                "beta1-concept",
            ),
            (
                "This is not historical but the current release carries version 0.1.0-alpha.2.",
                "This is not historical but the current release carries version 0.1.0-alpha.2",
            ),
        ] {
            assert_document_error(prose, &["claim_type=", &format!("asserted=`{asserted}`")]);
        }
    }

    #[test]
    fn claims_are_bound_to_their_immediate_asserted_values() {
        for prose in [
            "The current release version is 0.1.0-alpha.1.",
            "The current machine tier is research_software_alpha.",
            "The current display tier is Research Software Alpha.",
            "The current program name is aerocodex; the current release channel is github_releases.",
            "The current Cargo package version is `0.1.0-alpha.1`.",
            "The active workspace version is 0.1.0-alpha.1.",
            "The current release version\nis 0.1.0-alpha.1; the current release tier\nis research_software_alpha.",
            "The current release version is 0.1.0-alpha.1; example: `the current release version is 9.9.9`.",
        ] {
            verify_identity_document_bytes(
                "fixture.md",
                document_with_prose(prose).as_bytes(),
            )
            .unwrap();
        }

        for asserted in [
            "0.1",
            "0.2",
            "0.1.0",
            "0.1.0-alpha",
            "0.1.0-alpha.2",
            "v0.1.0-alpha.1",
        ] {
            assert_document_error(
                &format!("The current release version is {asserted}."),
                &[
                    "claim_type=semantic_version",
                    &format!("asserted=`{asserted}`"),
                    "expected=`0.1.0-alpha.1`",
                ],
            );
        }
        for (prose, claim, asserted, expected) in [
            (
                "Current release version is 0.1.0-alpha.2; expected example: 0.1.0-alpha.1.",
                "semantic_version",
                "0.1.0-alpha.2",
                EXPECTED_VERSION,
            ),
            (
                "Current release tier is production; identifier example: research_software_alpha.",
                "machine_release_tier",
                "production",
                EXPECTED_TIER,
            ),
            (
                "Current display tier is Production; machine identifier: research_software_alpha.",
                "display_release_tier",
                "Production",
                EXPECTED_TIER_DISPLAY,
            ),
            (
                "Example: `0.1.0-alpha.1`. The current release version is 0.1.0-alpha.2.",
                "semantic_version",
                "0.1.0-alpha.2",
                EXPECTED_VERSION,
            ),
        ] {
            assert_document_error(
                prose,
                &[
                    &format!("claim_type={claim}"),
                    &format!("asserted=`{asserted}`"),
                    &format!("expected=`{expected}`"),
                ],
            );
        }
        assert_document_error(
            "The current release carries version 0.1.0-alpha.1.",
            &["claim_type=ambiguous_current_identity", "fixture.md:"],
        );
    }

    #[test]
    fn html_comments_preserve_visible_prose_and_fail_closed() {
        for prose in [
            "The current release version is 0.1.0-alpha.1. <!-- harmless note -->",
            "The current <!-- harmless note --> release version is 0.1.0-alpha.1.",
            "Visible prefix <!-- one --> and suffix; the current release tier is research_software_alpha.",
            "```text\n<!-- comment markers stay inside the fence\n-->\nThe current release version is 0.1.0-alpha.2.\n```\nThe current release version is 0.1.0-alpha.1.",
            "<!-- before fence\ncomment only -->\n```text <!-- fenced info\n<!-- fenced comment-like content -->\n```\n<!-- after fence\ncomment only -->\nThe current release version is 0.1.0-alpha.1.",
            "<!-- before -->\n~~~text <!-- fenced info\n```\nThe current release tier is production.\n~~~\n<!-- after --> The current release tier is research_software_alpha.",
        ] {
            verify_identity_document_bytes(
                "fixture.md",
                document_with_prose(prose).as_bytes(),
            )
            .unwrap();
        }

        for (prose, expected) in [
            (
                "<!-- harmless review note --> The current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            (
                "<!-- harmless review note --> The current release tier is production.",
                "asserted=`production`",
            ),
            (
                "The current release version is 0.1.0-alpha.2. <!-- harmless review note -->",
                "asserted=`0.1.0-alpha.2`",
            ),
            (
                "Visible prefix <!-- harmless review note --> the current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            (
                "<!-- first --> The current release tier is production. <!-- second -->",
                "asserted=`production`",
            ),
            (
                "<!-- multiline\ncomment -->\nThe current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            (
                "<!-- multiline\ncomment --> The current release tier is production.",
                "asserted=`production`",
            ),
            (
                "```text\nbenign content\n```\n<!-- note --> The current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            (
                "<!-- note --> The current release\nversion is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            (
                "<!-- historical Beta 1 comment --> The current release tier is production.",
                "asserted=`production`",
            ),
        ] {
            assert_document_error(prose, &[expected]);
        }

        let crlf = document_with_prose(
            "<!-- review note --> The current release version is 0.1.0-alpha.2.",
        )
        .replace('\n', "\r\n");
        let error = verify_identity_document_bytes("fixture.md", crlf.as_bytes()).unwrap_err();
        assert!(error.contains("asserted=`0.1.0-alpha.2`"), "{error}");

        assert_document_error(
            "Visible prose <!-- unclosed comment",
            &["fixture.md:", "unclosed HTML comment"],
        );
        assert_document_error(
            "Visible prose --> remains visible",
            &["fixture.md:", "stray HTML comment close marker"],
        );
        assert_document_error(
            "```text <!-- fenced opener\nbenign content\n```\nVisible prose <!-- unclosed comment",
            &["fixture.md:", "unclosed HTML comment"],
        );
    }

    #[test]
    fn claim_polarity_uses_only_narrow_unambiguous_negatives() {
        for prose in [
            "0.1.0-alpha.2 is not the current release version.",
            "The current release tier is not production.",
            "This document does not claim production as the current tier.",
            "This document does not claim that production is the current release tier.",
            "Historical note: 0.1.0-alpha.2 is not the current release version; the current release version is 0.1.0-alpha.1.",
            "The current release tier is not\nproduction.",
            "The current release tier is not  \nproduction.",
            "It is not uncommon to discuss releases; the current release tier is research_software_alpha.",
            "0.1.0-alpha.2 is not the current release version; the current release tier is research_software_alpha.",
            "```text\n0.1.0-alpha.1 is not the current release version.\n```\nThe current release version is 0.1.0-alpha.1.",
            "Example: `0.1.0-alpha.1 is not the current release version.` The current release version is 0.1.0-alpha.1.",
        ] {
            verify_identity_document_bytes(
                "fixture.md",
                document_with_prose(prose).as_bytes(),
            )
            .unwrap();
        }

        for prose in [
            "It is not merely the current release version 0.1.0-alpha.2.",
            "It is not only the current release tier production.",
            "It is not true that 0.1.0-alpha.2 is not the current release version.",
            "It is not false that 0.1.0-alpha.2 is the current release version.",
            "It cannot be said that 0.1.0-alpha.2 is not the current release version.",
            "It is not simply the current machine tier production_ready.",
            "It is not historical but the current release version is 0.1.0-alpha.2.",
            "Never merely the current release tier is Beta 1.",
            "0.1.0-alpha.2 is not the current release version; the current release version is 0.1.0-alpha.2.",
            "It is not merely the current release\nversion 0.1.0-alpha.2.",
            "`not` is an example; the current release version is 0.1.0-alpha.2.",
            "The historical tier was not production; the current release tier is production.",
            "The current release tier is not not production.",
            "It is not uncommon; the current release version is 0.1.0-alpha.2.",
            "The current release tier is not production; the current release version is 0.1.0-alpha.2.",
            "It is not true that 0.1.0-alpha.2 is not the current release\nversion.",
            "Canonical token: 0.1.0-alpha.1; it is not true that 0.1.0-alpha.2 is not the current release version.",
            "The current release tier is not production; the current release tier is production.",
        ] {
            let error = verify_identity_document_bytes(
                "fixture.md",
                document_with_prose(prose).as_bytes(),
            )
            .expect_err("ambiguous or affirmative noncanonical claim should fail closed");
            assert!(error.contains("polarity="), "{error}");
            assert!(error.contains("expected=`"), "{error}");
        }

        for prose in [
            "It is not true that 0.1.0-alpha.2 is not the current release version.",
            "It is not false that 0.1.0-alpha.2 is the current release version.",
            "It cannot be said that 0.1.0-alpha.2 is not the current release version.",
        ] {
            let error =
                verify_identity_document_bytes("fixture.md", document_with_prose(prose).as_bytes())
                    .unwrap_err();
            for field in [
                "fixture.md:",
                "claim_type=",
                "polarity=ambiguous",
                "asserted=`",
                "expected=`",
                "reason=",
            ] {
                assert!(error.contains(field), "{error}");
            }
        }

        let crlf = document_with_prose(
            "It is not true that 0.1.0-alpha.2 is not the current release version.",
        )
        .replace('\n', "\r\n");
        let error = verify_identity_document_bytes("fixture.md", crlf.as_bytes()).unwrap_err();
        assert!(error.contains("polarity=ambiguous"), "{error}");
        assert!(
            error.contains("reason=multiple_or_nested_negation_tokens"),
            "{error}"
        );
    }

    #[test]
    fn governed_document_encoding_rejects_boms_and_preserves_scientific_unicode() {
        let valid = document_with_prose(
            "Scientific notation remains valid UTF-8: Δv = 1.0×10⁻³ m·s⁻¹, μ = 3.986×10¹⁴.",
        );
        verify_identity_document_bytes("fixture.md", valid.as_bytes()).unwrap();

        let mut utf8_bom = vec![0xef, 0xbb, 0xbf];
        utf8_bom.extend_from_slice(valid.as_bytes());
        let error = verify_identity_document_bytes("fixture.md", &utf8_bom).unwrap_err();
        assert!(error.contains("fixture.md") && error.contains("UTF-8 BOM"));

        let middle = valid.replace("Scientific", "Scien\u{FEFF}tific");
        assert!(
            verify_identity_document_bytes("fixture.md", middle.as_bytes())
                .unwrap_err()
                .contains("U+FEFF")
        );
        assert!(
            verify_identity_document_bytes("fixture.md", &[0xff, 0xfe, 0x23, 0x00])
                .unwrap_err()
                .contains("UTF-16LE")
        );
        assert!(
            verify_identity_document_bytes("fixture.md", &[0xfe, 0xff, 0x00, 0x23])
                .unwrap_err()
                .contains("UTF-16BE")
        );

        let mut crlf_bom = vec![0xef, 0xbb, 0xbf];
        crlf_bom.extend_from_slice(valid.replace('\n', "\r\n").as_bytes());
        assert!(verify_identity_document_bytes("fixture.md", &crlf_bom)
            .unwrap_err()
            .contains("UTF-8 BOM"));
    }

    // Complete public-command fixtures: a real repository clone traverses Git, manifest,
    // Cargo metadata, registry, governed-document, evidence, and compiled-CLI boundaries.
    #[test]
    fn complete_public_verifier_rejects_reported_document_bypasses() {
        let fixture = complete_repository_fixture();
        let target = "docs/beta1/release_concept.md";
        let mut utf8_bom = vec![0xef, 0xbb, 0xbf];
        utf8_bom.extend_from_slice(document_with_prose("Benign prose.").as_bytes());
        let mut invalid_utf8 = document_with_prose("Benign prose.").into_bytes();
        invalid_utf8.push(0xff);

        let scenarios = vec![
            // Fences and HTML comments.
            CompleteDocumentScenario::reject_prose(
                "fence_comment/exact_backtick_opener_comment_leak",
                "```text <!--\nbenign fenced content\n```\nThe current release version is 0.1.0-alpha.2.\n-->",
                "stray HTML comment close marker",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/backtick_contains_tilde_marker",
                "```text\n~~~\nstill fenced\n```\nThe current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/tilde_contains_backtick_marker",
                "~~~text\n```\nstill fenced\n~~~\nThe current release tier is production.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/four_backticks_shorter_apparent_close",
                "````text\n```\nstill fenced\n````\nThe current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/same_line_comment_then_invalid_visible_prose",
                "<!-- note --> The current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/multiline_comment_then_invalid_visible_prose",
                "<!-- multiline\ncomment -->\nThe current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/unclosed_html_comment",
                "Visible prose <!-- unclosed",
                "unclosed HTML comment",
            ),
            CompleteDocumentScenario::accept_prose(
                "fence_comment/comment_markers_inside_valid_fence",
                "```text\n<!-- markers remain fenced -->\nThe current release tier is production.\n```\nThe current release tier is research_software_alpha.",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/backtick_info_comment_then_invalid_version",
                "```text <!--\nbenign fenced content\n```\nThe current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/tilde_info_comment_then_invalid_tier",
                "~~~text <!--\nbenign fenced content\n~~~\nThe current release tier is production.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/fenced_content_comment_then_invalid_visible_version",
                "```text\n<!-- fenced marker -->\n```\nThe current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/comment_like_apparent_close_then_true_close",
                "```text\n``` <!-- not a valid close\nThe current release version is 0.1.0-alpha.2.\n```\nThe current release tier is production.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::accept_prose(
                "fence_comment/html_comment_before_fence",
                "<!-- before fence -->\n```text <!-- info\nbenign\n```\nThe current release version is 0.1.0-alpha.1.",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/html_comment_after_fence",
                "```text\nbenign\n```\n<!-- after fence --> The current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::accept_prose(
                "fence_comment/multiline_comments_surround_closed_fence",
                "<!-- before\ncomment -->\n```text <!-- info\nbenign\n```\n<!-- after\ncomment -->\nThe current release version is 0.1.0-alpha.1.",
            ),
            CompleteDocumentScenario::reject_document(
                "fence_comment/crlf_exact_opener_comment_leak",
                document_with_prose("```text <!--\nbenign fenced content\n```\nThe current release version is 0.1.0-alpha.2.\n-->").replace('\n', "\r\n"),
                "stray HTML comment close marker",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/unclosed_fence_with_comment_markers",
                "````text <!--\n<!-- fenced marker -->\n```",
                "unclosed Markdown fence",
            ),
            CompleteDocumentScenario::reject_prose(
                "fence_comment/unclosed_comment_after_closed_fence",
                "```text <!-- info\nbenign\n```\nVisible prose <!-- unclosed",
                "unclosed HTML comment",
            ),
            // Negation and polarity.
            CompleteDocumentScenario::reject_prose(
                "polarity/exact_not_true_not_current",
                "It is not true that 0.1.0-alpha.2 is not the current release version.",
                "reason=multiple_or_nested_negation_tokens",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/not_false_current",
                "It is not false that 0.1.0-alpha.2 is the current release version.",
                "reason=multiple_or_nested_negation_tokens",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/cannot_be_said_not_current",
                "It cannot be said that 0.1.0-alpha.2 is not the current release version.",
                "polarity=ambiguous",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/not_merely",
                "It is not merely the current release version 0.1.0-alpha.2.",
                "polarity=ambiguous",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/not_only",
                "It is not only the current release tier production.",
                "polarity=ambiguous",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/double_negation_soft_wrap",
                "It is not true that 0.1.0-alpha.2 is not the current release\nversion.",
                "reason=multiple_or_nested_negation_tokens",
            ),
            CompleteDocumentScenario::reject_document(
                "polarity/double_negation_crlf",
                document_with_prose("It is not true that 0.1.0-alpha.2 is not the current release version.").replace('\n', "\r\n"),
                "reason=multiple_or_nested_negation_tokens",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/supported_negative_then_invalid_affirmative",
                "0.1.0-alpha.2 is not the current release version; the current release version is 0.1.0-alpha.2.",
                "polarity=affirmative",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/historical_negative_then_invalid_current",
                "The historical tier was not production; the current release tier is production.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/double_negation_beside_canonical_token",
                "Canonical token: 0.1.0-alpha.1; it is not true that 0.1.0-alpha.2 is not the current release version.",
                "reason=multiple_or_nested_negation_tokens",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/multiple_current_claims_conflicting_polarity",
                "The current release tier is not production; the current release tier is production.",
                "polarity=affirmative",
            ),
            CompleteDocumentScenario::reject_prose(
                "polarity/never_merely",
                "Never merely the current release tier is Beta 1.",
                "polarity=ambiguous",
            ),
            CompleteDocumentScenario::accept_prose(
                "polarity/supported_simple_negative_version",
                "0.1.0-alpha.2 is not the current release version.",
            ),
            CompleteDocumentScenario::accept_prose(
                "polarity/supported_simple_negative_tier",
                "The current release tier is not production.",
            ),
            CompleteDocumentScenario::accept_prose(
                "polarity/supported_does_not_claim",
                "This document does not claim production as the current tier.",
            ),
            CompleteDocumentScenario::accept_prose(
                "polarity/supported_negative_then_correct_identity",
                "0.1.0-alpha.2 is not the current release version; the current release version is 0.1.0-alpha.1.",
            ),
            CompleteDocumentScenario::accept_prose(
                "polarity/negative_example_in_fence",
                "```text\n0.1.0-alpha.1 is not the current release version.\n```\nThe current release version is 0.1.0-alpha.1.",
            ),
            CompleteDocumentScenario::accept_prose(
                "polarity/negative_inline_example_with_valid_prose",
                "Example: `0.1.0-alpha.1 is not the current release version.` The current release version is 0.1.0-alpha.1.",
            ),
            // Document structure and encoding.
            CompleteDocumentScenario::reject_prose(
                "structure_encoding/lf_invalid_version",
                "The current release version is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_document(
                "structure_encoding/crlf_invalid_tier",
                document_with_prose("The current release tier is production.").replace('\n', "\r\n"),
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_document(
                "structure_encoding/mixed_newlines_invalid_version",
                document_with_prose("The current release version is 0.1.0-alpha.2.").replacen('\n', "\r\n", 5),
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "structure_encoding/multiline_soft_break",
                "The current release\nversion is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "structure_encoding/multiline_hard_break",
                "The current release  \nversion is 0.1.0-alpha.2.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "structure_encoding/historical_clause_invalid_current",
                "Historical Beta 1 used 0.0.1; the current release tier is production.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_prose(
                "structure_encoding/historical_section_invalid_current",
                "## Historical Beta 1\n\nThe old tier was beta1-concept.\n\n## Current release\n\nThe current release tier is production.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_document(
                "structure_encoding/utf8_bom",
                utf8_bom,
                "UTF-8 BOM",
            ),
            CompleteDocumentScenario::reject_prose(
                "structure_encoding/embedded_ufeff",
                "Embedded U+FEFF: \u{FEFF}.",
                "U+FEFF",
            ),
            CompleteDocumentScenario::reject_document(
                "structure_encoding/invalid_utf8",
                invalid_utf8,
                "invalid UTF-8",
            ),
            CompleteDocumentScenario::reject_prose(
                "structure_encoding/mojibake",
                "Mojibake marker: AlphÃ.",
                "mojibake",
            ),
            CompleteDocumentScenario::reject_document(
                "structure_encoding/duplicate_identity_block",
                format!("{}\n{}", identity_document(IDENTITY_BODY), identity_document(IDENTITY_BODY)),
                "identity block markers must appear exactly once",
            ),
            CompleteDocumentScenario::reject_document(
                "structure_encoding/missing_identity_block",
                identity_document(IDENTITY_BODY).replace(IDENTITY_START, ""),
                "identity block markers must appear exactly once",
            ),
            CompleteDocumentScenario::reject_document(
                "structure_encoding/malformed_identity_block",
                identity_document(&IDENTITY_BODY.replacen(EXPECTED_VERSION, "0.1.0-alpha.2", 1)),
                "current identity block does not contain the exact governed values",
            ),
            // Immediate claim/value binding.
            CompleteDocumentScenario::reject_prose(
                "claim_binding/invalid_version_with_canonical_elsewhere",
                "Current release version is 0.1.0-alpha.2; expected example: 0.1.0-alpha.1.",
                "asserted=`0.1.0-alpha.2`",
            ),
            CompleteDocumentScenario::reject_prose(
                "claim_binding/invalid_machine_tier_with_canonical_elsewhere",
                "Current release tier is production; identifier example: research_software_alpha.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_prose(
                "claim_binding/invalid_display_tier_with_machine_token",
                "Current display tier is Production; machine identifier: research_software_alpha.",
                "asserted=`Production`",
            ),
            CompleteDocumentScenario::reject_prose(
                "claim_binding/inline_example_beside_invalid_visible_claim",
                "Example: `the current release tier is research_software_alpha`; the current release tier is production.",
                "asserted=`production`",
            ),
            CompleteDocumentScenario::reject_prose(
                "claim_binding/multiple_current_claims_one_invalid",
                "The current release version is 0.1.0-alpha.1; the current release tier is production.",
                "asserted=`production`",
            ),
        ];

        let mut names = BTreeSet::new();
        let mut category_counts = BTreeMap::new();
        for scenario in &scenarios {
            assert!(
                names.insert(scenario.name),
                "duplicate complete public-command scenario name: {}",
                scenario.name
            );
            let category = scenario
                .name
                .split_once('/')
                .expect("scenario names must use category/name")
                .0;
            *category_counts.entry(category).or_insert(0usize) += 1;
        }
        assert_eq!(
            scenarios.len(),
            COMPLETE_PUBLIC_COMMAND_FIXTURE_COUNT,
            "documented complete public-command scenario count must stay synchronized"
        );
        assert_eq!(
            category_counts,
            BTreeMap::from([
                ("claim_binding", 5usize),
                ("fence_comment", 18usize),
                ("polarity", 18usize),
                ("structure_encoding", 14usize),
            ])
        );
        let status =
            fs::read_to_string(repository_root().join("docs/release/v0.1.0-alpha.1-status.md"))
                .expect("release status should be readable for matrix synchronization");
        assert!(
            status.contains(&format!(
                "- Complete public-command fixture scenarios: `{}`",
                scenarios.len()
            )),
            "release status must report the derived complete public-command scenario count"
        );
        assert!(
            status.contains("- Complete public-command fixture categories: `fence_comment=18; polarity=18; structure_encoding=14; claim_binding=5`"),
            "release status must report the derived complete public-command categories"
        );

        for scenario in scenarios {
            fixture.write(target, scenario.contents);
            match scenario.expected_error {
                Some(expected) => {
                    let error = match verify_release_identity(&fixture.path) {
                        Ok(()) => panic!(
                            "scenario {} should be rejected by the complete public verifier",
                            scenario.name
                        ),
                        Err(error) => error,
                    };
                    assert!(
                        error.contains(expected),
                        "scenario {}: {error:?} did not contain {expected:?}",
                        scenario.name
                    );
                }
                None => verify_release_identity(&fixture.path)
                    .unwrap_or_else(|error| panic!("scenario {}: {error}", scenario.name)),
            }
        }
    }

    // Production-loader fixtures: real Cargo workspaces and real `cargo metadata` output.
    #[test]
    fn production_cargo_loader_rejects_package_topology_drift() {
        let (removed, governed) = package_workspace_fixture();
        let root_manifest = fs::read_to_string(removed.path.join("Cargo.toml")).unwrap();
        removed.write(
            "Cargo.toml",
            replace_once(&root_manifest, "    \"crates/fixture-13\",\n", ""),
        );
        assert!(validate_package_fixture(&removed, &governed)
            .unwrap_err()
            .contains("cargo_missing"));

        let (extra_member, governed) = package_workspace_fixture();
        write_package(&extra_member, "crates/extra", "extra", "workspace");
        let root_manifest = fs::read_to_string(extra_member.path.join("Cargo.toml")).unwrap();
        extra_member.write(
            "Cargo.toml",
            replace_once(
                &root_manifest,
                "]\n\n[workspace.package]",
                "    \"crates/extra\",\n]\n\n[workspace.package]",
            ),
        );
        assert!(validate_package_fixture(&extra_member, &governed)
            .unwrap_err()
            .contains("cargo_extra"));

        let (outside, governed) = package_workspace_fixture();
        write_package(&outside, "crates/outside", "outside", "workspace");
        assert!(validate_package_fixture(&outside, &governed)
            .unwrap_err()
            .contains("discovered_extra"));

        let (version, governed) = package_workspace_fixture();
        write_package(&version, "crates/fixture-00", "fixture-00", "0.1.0-alpha.2");
        assert!(validate_package_fixture(&version, &governed)
            .unwrap_err()
            .contains("version mismatch"));

        let (name, governed) = package_workspace_fixture();
        write_package(&name, "crates/fixture-00", "renamed", "workspace");
        assert!(validate_package_fixture(&name, &governed)
            .unwrap_err()
            .contains("name mismatch"));

        let (duplicate, governed) = package_workspace_fixture();
        write_package(&duplicate, "outside/duplicate", "fixture-00", "workspace");
        assert!(validate_package_fixture(&duplicate, &governed)
            .unwrap_err()
            .contains("duplicate discovered package name"));

        let (excluded_without_reason, governed) = package_workspace_fixture();
        write_package(
            &excluded_without_reason,
            "crates/excluded",
            "excluded",
            "workspace",
        );
        let root_manifest =
            fs::read_to_string(excluded_without_reason.path.join("Cargo.toml")).unwrap();
        excluded_without_reason.write(
            "Cargo.toml",
            replace_once(
                &root_manifest,
                "]\n\n[workspace.package]",
                "]\nexclude = [\"crates/excluded\"]\n\n[workspace.package]",
            ),
        );
        assert!(
            validate_package_fixture(&excluded_without_reason, &governed)
                .unwrap_err()
                .contains("discovered_extra")
        );
    }

    // Production-loader fixtures: checked registry JSON, SHA-256 sidecar, and parser.
    #[test]
    fn production_registry_loader_rejects_artifact_and_policy_drift() {
        let missing_registry = TestDirectory::create("missing_registry");
        let error =
            formula_registry::check::run_check_command(&missing_registry.path, &CheckOptions)
                .unwrap_err();
        assert!(error.contains("generated/formula_registry.json is missing or unreadable"));

        let missing_sidecar = registry_fixture();
        fs::remove_file(
            missing_sidecar
                .path
                .join("generated/formula_registry.sha256"),
        )
        .unwrap();
        assert!(
            formula_registry::check::run_check_command(&missing_sidecar.path, &CheckOptions)
                .unwrap_err()
                .contains("generated/formula_registry.sha256 is missing or unreadable")
        );

        let wrong_sidecar = registry_fixture();
        wrong_sidecar.write(
            "generated/formula_registry.sha256",
            "0000000000000000000000000000000000000000000000000000000000000000  generated/formula_registry.json\n",
        );
        assert!(
            formula_registry::rust::load_checked_formula_identity_registry(&wrong_sidecar.path)
                .unwrap_err()
                .contains("stale sha256 sidecar")
        );

        let malformed = registry_fixture();
        write_checked_registry(&malformed, "{\"formulas\":[}");
        assert!(
            formula_registry::rust::load_checked_formula_identity_registry(&malformed.path)
                .unwrap_err()
                .contains("unexpected JSON character")
        );

        let duplicate = registry_fixture();
        let text =
            fs::read_to_string(duplicate.path.join("generated/formula_registry.json")).unwrap();
        let parsed =
            formula_registry::rust::load_checked_formula_identity_registry(&duplicate.path)
                .unwrap();
        let first = &parsed.formulas[0].formula_id;
        let second = &parsed.formulas[1].formula_id;
        let duplicate_text = replace_once(
            &text,
            &format!("\"formula_id\": \"{second}\""),
            &format!("\"formula_id\": \"{first}\""),
        );
        write_checked_registry(&duplicate, &duplicate_text);
        assert!(
            formula_registry::rust::load_checked_formula_identity_registry(&duplicate.path)
                .unwrap_err()
                .contains("duplicate formula_id")
        );

        let manifest = release_manifest::load_release_manifest(&registry_fixture().path)
            .expect("production manifest parser should load the fixture");
        for (old, new, expected) in [
            (
                "\"status\": \"research_required\"",
                "\"status\": \"equation_traceable\"",
                "research_required",
            ),
            (
                "\"execution_policy\": \"blocked\"",
                "\"execution_policy\": \"preliminary_flag_required\"",
                "blocked",
            ),
            (
                "\"execution_policy\": \"blocked\"",
                "\"execution_policy\": \"normal_research\"",
                "publicly_executable",
            ),
        ] {
            let fixture = registry_fixture();
            let text =
                fs::read_to_string(fixture.path.join("generated/formula_registry.json")).unwrap();
            write_checked_registry(&fixture, &text.replacen(old, new, 1));
            let registry =
                formula_registry::rust::load_checked_formula_identity_registry(&fixture.path)
                    .unwrap();
            let summary = summarize_registry(&registry).unwrap();
            let error = validate_registry_summary(&manifest, summary).unwrap_err();
            assert!(error.contains(expected), "{error:?}");
        }

        let count_mismatch = registry_fixture();
        let manifest_text =
            fs::read_to_string(count_mismatch.path.join("release/release-manifest.toml")).unwrap();
        count_mismatch.write(
            "release/release-manifest.toml",
            manifest_text.replacen(
                "registry_formula_count = 152",
                "registry_formula_count = 151",
                1,
            ),
        );
        assert!(
            release_manifest::load_release_manifest(&count_mismatch.path)
                .unwrap_err()
                .contains("registry_formula_count")
        );
    }

    // Compiled-process integration fixtures: each drift is built into and observed from a real
    // CLI executable. The canonical build also proves external-CWD and repository independence.
    #[test]
    fn compiled_cli_process_detects_identity_count_and_legacy_drift() {
        let fixture = TestDirectory::create("compiled_cli");
        copy_working_tree(&repository_root(), &fixture.path);
        let main_path = fixture.path.join("crates/aero-codex-cli/src/main.rs");
        let registry_path = fixture.path.join("generated/rust/formula_registry.rs");
        let main_baseline = fs::read_to_string(&main_path).unwrap();
        let registry_baseline = fs::read_to_string(&registry_path).unwrap();

        for (label, main_old, main_new, registry_old, registry_new, expected) in [
            (
                "semantic-version",
                "fn package_version() -> &'static str {\n    env!(\"CARGO_PKG_VERSION\")\n}",
                "fn package_version() -> &'static str {\n    \"0.1.0-alpha.2\"\n}",
                "",
                "",
                "semantic_version",
            ),
            (
                "machine-tier",
                "fn release_tier() -> &'static str {\n    \"research_software_alpha\"\n}",
                "fn release_tier() -> &'static str {\n    \"production\"\n}",
                "",
                "",
                "release_tier",
            ),
            (
                "display-tier",
                "fn release_tier_display() -> &'static str {\n    \"Research Software Alpha\"\n}",
                "fn release_tier_display() -> &'static str {\n    \"Production\"\n}",
                "",
                "",
                "release_tier_display",
            ),
            (
                "formula-count",
                "",
                "",
                "pub const FORMULA_COUNT: usize = 152;",
                "pub const FORMULA_COUNT: usize = 151;",
                "registry_formula_count",
            ),
            (
                "legacy-beta",
                "const PROGRAM_NAME: &str = \"aerocodex\";",
                "const PROGRAM_NAME: &str = \"beta1-concept\";",
                "",
                "",
                "legacy Beta 1",
            ),
        ] {
            let changed_main = if main_old.is_empty() {
                main_baseline.clone()
            } else {
                replace_once_in_disposable_fixture(&main_baseline, main_old, main_new)
            };
            let changed_registry = if registry_old.is_empty() {
                registry_baseline.clone()
            } else {
                replace_once_in_disposable_fixture(&registry_baseline, registry_old, registry_new)
            };
            fs::write(&main_path, changed_main).unwrap();
            fs::write(&registry_path, changed_registry).unwrap();
            let binary = build_cli_fixture(&fixture, label);
            let external = ExternalCliDir::create().unwrap();
            let output = run_cli(&binary, &external.path, &["version", "--json"], None).unwrap();
            require_success(&output, "compiled drift fixture").unwrap();
            let error =
                validate_cli_version_json(output_utf8(&output.stdout, "drift JSON").unwrap())
                    .unwrap_err();
            assert!(error.contains(expected), "{label}: {error:?}");
        }

        fs::write(&main_path, &main_baseline).unwrap();

        let crlf_registry = registry_baseline
            .replace("\r\n", "\n")
            .replace('\n', "\r\n");
        assert!(
            crlf_registry.contains("\r\n"),
            "explicit CRLF fixture must be host-independent"
        );
        let crlf_changed = replace_once_in_disposable_fixture(
            &crlf_registry,
            "pub const FORMULA_COUNT: usize = 152;",
            "pub const FORMULA_COUNT: usize = 151;",
        );
        assert_eq!(
            crlf_changed
                .matches("pub const FORMULA_COUNT: usize = 151;")
                .count(),
            1
        );
        assert_eq!(
            crlf_changed
                .matches("pub const FORMULA_COUNT: usize = 152;")
                .count(),
            0
        );
        fs::write(&registry_path, crlf_changed).unwrap();
        let binary = build_cli_fixture(&fixture, "formula-count-crlf");
        let external = ExternalCliDir::create().unwrap();
        let output = run_cli(&binary, &external.path, &["version", "--json"], None).unwrap();
        require_success(&output, "compiled CRLF drift fixture").unwrap();
        let error =
            validate_cli_version_json(output_utf8(&output.stdout, "CRLF drift JSON").unwrap())
                .unwrap_err();
        assert!(error.contains("registry_formula_count"), "{error:?}");

        fs::write(&main_path, main_baseline).unwrap();
        fs::write(&registry_path, registry_baseline).unwrap();
        let metadata = load_cargo_metadata(&fixture.path).unwrap();
        verify_compiled_cli(&fixture.path, &metadata.target_directory).unwrap();
    }
}
