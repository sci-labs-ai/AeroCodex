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
const EXPECTED_TIER: &str = "research_software_alpha";
const EXPECTED_TIER_DISPLAY: &str = "Research Software Alpha";
const EXPECTED_PROGRAM: &str = "aerocodex";
const EXPECTED_PACKAGE_COUNT: usize = 14;
const EXPECTED_FORMULA_COUNT: usize = 152;
const EXPECTED_BASE_COMMIT: &str = "6a94b4628e6e0821a55d6aaadf6925956a5fa2d3";
const FORCE_SELF_CHECK_FAILURE_ENV: &str = "AEROCODEX_TEST_FORCE_SELF_CHECK_FAILURE";

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
const IDENTITY_BODY: &str = "Release version: `0.1.0-alpha.1`\nRelease tier: `research_software_alpha` (`Research Software Alpha`)\nWorkspace packages: `14`\nRegistry formulas: `152`\nBlocked formulas: `152`\nPublicly executable formulas: `0`";

const EVIDENCE_START: &str = "<!-- aerocodex-release-evidence:start -->";
const EVIDENCE_END: &str = "<!-- aerocodex-release-evidence:end -->";
const EVIDENCE_BODY: &str = "- Base commit: `6a94b4628e6e0821a55d6aaadf6925956a5fa2d3`\n- Scope: `minimal_release_identity`\n- Semantic-version authority: `Cargo.toml [workspace.package].version`\n- Release-tier authority: `docs/release/v0.1.0-alpha.1.toml`\n- Governed package count: `14`\n- Registry facts: `152 research_required; 152 blocked; 0 publicly executable`\n- CLI self-check: `14 passed; 0 failed`\n- CI authority: `fresh_github_actions_runners`\n- Threat-model boundary: `local_scripts_are_conveniences_not_security_sandboxes`\n- Excluded work: `packaging, signing, tagging, publication, formula promotion`";

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

pub fn verify_release_identity(root: &Path) -> Result<(), String> {
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
        "verified release identity: program={EXPECTED_PROGRAM}; version={EXPECTED_VERSION}; tier={EXPECTED_TIER}; tier_display={EXPECTED_TIER_DISPLAY}; packages={EXPECTED_PACKAGE_COUNT}; formulas={}; research_required={}; blocked={}; publicly_executable={}; self_check=14_passed_0_failed; external_binary=PASS",
        registry_summary.total,
        registry_summary.research_required,
        registry_summary.blocked,
        registry_summary.publicly_executable
    );
    println!(
        "release_identity_threat_model=GitHub Actions fresh runners are authoritative; local verification reduces accidental drift but is not an adversarial host-filesystem sandbox."
    );
    Ok(())
}

fn validate_manifest_identity(manifest: &ReleaseManifest) -> Result<(), String> {
    validate_target_semver(&manifest.release_version)?;
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
        research_required: manifest.registry_research_required_formula_count,
        blocked: manifest.registry_blocked_formula_count,
        publicly_executable: manifest.public_executable_formula_count,
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
    let text = std::str::from_utf8(bytes)
        .map_err(|error| format!("{path}: invalid UTF-8 at byte {}", error.valid_up_to()))?;
    for marker in ['\u{FFFD}', 'Ã', 'Â'] {
        if text.contains(marker) {
            return Err(format!("{path}: mojibake marker `{marker}` is not allowed"));
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
    let mut paragraph = Vec::new();
    let mut paragraph_line = 1usize;
    let mut in_fence = false;
    let mut authoritative_fence = false;
    let mut next_fence_authoritative = false;
    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim();
        if trimmed == "<!-- aerocodex-authoritative-example -->" {
            check_identity_paragraph(path, paragraph_line, &paragraph.join(" "))?;
            paragraph.clear();
            next_fence_authoritative = true;
            continue;
        }
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            check_identity_paragraph(path, paragraph_line, &paragraph.join(" "))?;
            paragraph.clear();
            if in_fence {
                in_fence = false;
                authoritative_fence = false;
            } else {
                in_fence = true;
                authoritative_fence = next_fence_authoritative;
                next_fence_authoritative = false;
            }
            continue;
        }
        if in_fence && !authoritative_fence {
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("<!--") {
            check_identity_paragraph(path, paragraph_line, &paragraph.join(" "))?;
            paragraph.clear();
            if trimmed.starts_with('#') {
                check_identity_paragraph(path, line_number, trimmed)?;
            }
            continue;
        }
        if paragraph.is_empty() {
            paragraph_line = line_number;
        }
        paragraph.push(trimmed);
    }
    check_identity_paragraph(path, paragraph_line, &paragraph.join(" "))
}

fn check_identity_paragraph(path: &str, line: usize, paragraph: &str) -> Result<(), String> {
    if paragraph.is_empty() {
        return Ok(());
    }
    let lower = paragraph.to_ascii_lowercase();
    let current = [
        "current",
        "currently",
        " now ",
        " remains",
        "active workspace version",
        "runtime identity",
        "release version",
        "release tier",
        "cargo version",
        "cargo package version",
    ]
    .iter()
    .any(|cue| lower.contains(cue));
    if !current {
        return Ok(());
    }
    let historical = [
        "historical",
        "formerly",
        "previous",
        "legacy",
        "compatibility alias",
    ]
    .iter()
    .any(|cue| lower.contains(cue));
    let versions = semver_tokens(paragraph);
    for version in versions {
        if version != EXPECTED_VERSION && !historical {
            return Err(format!(
                "{path}:{line}: competing current semantic version `{version}`; expected `{EXPECTED_VERSION}`"
            ));
        }
    }
    let tier_claim = [
        "release tier is",
        "release tier:",
        "release tier remains",
        "release tier now",
    ]
    .iter()
    .any(|cue| lower.contains(cue));
    if tier_claim
        && !paragraph.contains(EXPECTED_TIER)
        && !paragraph.contains(EXPECTED_TIER_DISPLAY)
        && !historical
    {
        return Err(format!(
            "{path}:{line}: competing or incomplete current release-tier claim"
        ));
    }
    if (lower.contains("beta1-concept") || lower.contains("beta 1"))
        && !historical
        && (lower.contains("current")
            || lower.contains("runtime identity")
            || lower.contains("remains"))
    {
        return Err(format!(
            "{path}:{line}: Beta 1 is presented as current identity rather than historical material or a compatibility alias"
        ));
    }
    Ok(())
}

fn semver_tokens(text: &str) -> Vec<&str> {
    text.split(|character: char| {
        !(character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '+'))
    })
    .map(|token| token.trim_matches('.'))
    .filter(|token| {
        let core = token.split(['-', '+']).next().unwrap_or("");
        let parts: Vec<&str> = core.split('.').collect();
        parts.len() == 3
            && parts
                .iter()
                .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))
    })
    .collect()
}

fn verify_release_status_evidence(root: &Path) -> Result<(), String> {
    let relative = "docs/release/v0.1.0-alpha.1-status.md";
    let text = fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))?
        .replace("\r\n", "\n");
    verify_exact_block(relative, &text, EVIDENCE_START, EVIDENCE_BODY, EVIDENCE_END)
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
    for forbidden in [".git", "Cargo.toml", "docs", "v0.1.0-alpha.1.toml"] {
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
    let mut command = Command::new(binary);
    command
        .current_dir(working_directory)
        .args(arguments)
        .env_remove(FORCE_SELF_CHECK_FAILURE_ENV);
    if let Some((key, value)) = environment {
        command.env(key, value);
    }
    command
        .output()
        .map_err(|error| format!("cannot execute copied CLI {}: {error}", binary.display()))
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
                .contains("release-tier")
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
                .contains("Beta 1")
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
}
