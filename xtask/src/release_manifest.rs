use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
    process::{Command, Output},
};

pub const RELEASE_MANIFEST_PATH: &str = "release/release-manifest.toml";
pub const OBSOLETE_RELEASE_MANIFEST_PATH: &str = "docs/release/v0.1.0-alpha.1.toml";
pub const CLI_DISPATCH_METADATA_PATH: &str = "crates/aero-codex-cli/dispatch_metadata.tsv";
const REGISTRY_PATH: &str = "generated/formula_registry.json";
const RELEASE_SCHEMA_VERSION: &str = "aerocodex.release_manifest.v3";
const RELEASE_VERSION: &str = "0.1.0-alpha.1";
const RELEASE_CHANNEL: &str = "github_releases";
const RELEASE_TIER: &str = "research_software_alpha";
const RELEASE_TIER_DISPLAY: &str = "Research Software Alpha";
const CLI_DISPATCH_SCHEMA_VERSION: &str = "aerocodex.cli_dispatch.v1";
const RELEASE_VALIDATION_RECORD: &str = "validation/equation_inventory.tsv";
const RELEASE_DOCUMENTATION: &str = "docs/release/v0.1.0-alpha.1-status.md";
const REGISTRY_FORMULA_COUNT: usize = 152;

const EXPECTED_PACKAGES: [(&str, &str); 14] = [
    ("aero-codex-core", "crates/aero-codex-core/Cargo.toml"),
    (
        "aero-codex-constants",
        "crates/aero-codex-constants/Cargo.toml",
    ),
    (
        "aero-codex-atmosphere",
        "crates/aero-codex-atmosphere/Cargo.toml",
    ),
    ("aero-codex-thermo", "crates/aero-codex-thermo/Cargo.toml"),
    (
        "aero-codex-gas-dynamics",
        "crates/aero-codex-gas-dynamics/Cargo.toml",
    ),
    (
        "aero-codex-aerodynamics",
        "crates/aero-codex-aerodynamics/Cargo.toml",
    ),
    (
        "aero-codex-propulsion",
        "crates/aero-codex-propulsion/Cargo.toml",
    ),
    (
        "aero-codex-heat-transfer",
        "crates/aero-codex-heat-transfer/Cargo.toml",
    ),
    (
        "aero-codex-structures",
        "crates/aero-codex-structures/Cargo.toml",
    ),
    (
        "aero-codex-flight-dynamics",
        "crates/aero-codex-flight-dynamics/Cargo.toml",
    ),
    (
        "aero-codex-astrodynamics",
        "crates/aero-codex-astrodynamics/Cargo.toml",
    ),
    (
        "aero-codex-life-support",
        "crates/aero-codex-life-support/Cargo.toml",
    ),
    ("aero-codex-cli", "crates/aero-codex-cli/Cargo.toml"),
    ("xtask", "xtask/Cargo.toml"),
];

const EXPECTED_FORMULA_IDS: [&str; 12] = [
    "m00.angle.deg_to_rad",
    "m00.angle.rad_to_deg",
    "m00.canonical.distance_from_canonical",
    "m00.canonical.distance_to_canonical",
    "m00.canonical.mu_from_units",
    "m00.canonical.speed_from_canonical",
    "m00.canonical.speed_to_canonical",
    "m00.canonical.speed_unit_from_du_tu",
    "m00.canonical.speed_unit_from_mu_du",
    "m00.canonical.time_from_canonical",
    "m00.canonical.time_to_canonical",
    "m00.canonical.time_unit_from_mu_du",
];

const EXPECTED_TARGETS: [(&str, &str, &str); 4] = [
    ("aarch64-apple-darwin", "tier2", "required_tier2"),
    ("x86_64-apple-darwin", "tier2", "required_tier2"),
    ("x86_64-pc-windows-msvc", "tier1", "blocking"),
    ("x86_64-unknown-linux-gnu", "tier1", "blocking"),
];

type ArtifactContract<'a> = (&'a str, &'a str, &'a str, Option<&'a str>, &'a str, &'a str);

const EXPECTED_ARTIFACTS: [ArtifactContract<'static>; 9] = [
    (
        "checksums",
        "aerocodex-0.1.0-alpha.1-SHA256SUMS",
        "checksum",
        None,
        "none",
        "R5-09",
    ),
    (
        "cli-linux-x86-64",
        "aerocodex-0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz",
        "cli_archive",
        Some("x86_64-unknown-linux-gnu"),
        "tar.gz",
        "R4-09",
    ),
    (
        "cli-macos-aarch64",
        "aerocodex-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz",
        "cli_archive",
        Some("aarch64-apple-darwin"),
        "tar.gz",
        "R4-09",
    ),
    (
        "cli-macos-x86-64",
        "aerocodex-0.1.0-alpha.1-x86_64-apple-darwin.tar.gz",
        "cli_archive",
        Some("x86_64-apple-darwin"),
        "tar.gz",
        "R4-09",
    ),
    (
        "cli-windows-x86-64",
        "aerocodex-0.1.0-alpha.1-x86_64-pc-windows-msvc.zip",
        "cli_archive",
        Some("x86_64-pc-windows-msvc"),
        "zip",
        "R4-09",
    ),
    (
        "provenance",
        "aerocodex-0.1.0-alpha.1.intoto.jsonl",
        "provenance",
        None,
        "none",
        "R5-10",
    ),
    (
        "release-manifest",
        "aerocodex-0.1.0-alpha.1-release-manifest.toml",
        "release_manifest",
        None,
        "none",
        "R4-09",
    ),
    (
        "sbom",
        "aerocodex-0.1.0-alpha.1.spdx.json",
        "sbom",
        None,
        "none",
        "R5-10",
    ),
    (
        "source-archive",
        "aerocodex-0.1.0-alpha.1-source.tar.gz",
        "source_archive",
        None,
        "tar.gz",
        "R4-09",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub schema_version: String,
    pub release_version: String,
    pub release_channel: String,
    pub release_tier: String,
    pub release_tier_display: String,
    pub program_name: String,
    pub source_revision_policy: String,
    pub base_commit: String,
    pub registry_path: String,
    pub registry_formula_count: usize,
    pub cli_dispatch_metadata: String,
    pub release_validation_record: String,
    pub release_documentation: String,
    pub release_slice_requirements: ReleaseRequirements,
    pub non_release_requirements: ReleaseRequirements,
    pub packages: Vec<ReleasePackage>,
    pub formulas: Vec<ReleaseFormula>,
    pub targets: Vec<ReleaseTarget>,
    pub artifacts: Vec<ReleaseArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseRequirements {
    pub validation_status: String,
    pub execution_policy: String,
    pub public_executable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleasePackage {
    pub name: String,
    pub manifest_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseFormula {
    pub identifier: String,
    pub runtime_symbol: String,
    pub validation_record: String,
    pub documentation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseTarget {
    pub triple: String,
    pub platform_tier: String,
    pub ci_posture: String,
    pub artifact_eligible: bool,
    pub currently_tested: bool,
    pub currently_packaged: bool,
    pub current_support: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseArtifact {
    pub identifier: String,
    pub filename: String,
    pub kind: String,
    pub target: Option<String>,
    pub archive_format: String,
    pub production_state: String,
    pub required_by_task: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliDispatchFormula {
    canonical_formula_id: String,
    runtime_symbol: String,
}

pub fn verify_release_manifest(root: &Path) -> Result<(), String> {
    verify_authority_contract(root)?;
    let manifest = load_release_manifest(root)?;
    verify_pinned_base_commit(root, &manifest.base_commit)?;

    verify_workspace_contract(root, &manifest)?;

    let dispatch_path = root.join(&manifest.cli_dispatch_metadata);
    let dispatch_text = fs::read_to_string(&dispatch_path)
        .map_err(|error| format!("cannot read {CLI_DISPATCH_METADATA_PATH}: {error}"))?;
    let dispatch = parse_cli_dispatch_metadata(&dispatch_text)?;
    validate_dispatch_matches_manifest(&manifest, &dispatch)?;

    for reference in [
        manifest.registry_path.as_str(),
        manifest.cli_dispatch_metadata.as_str(),
        manifest.release_validation_record.as_str(),
        manifest.release_documentation.as_str(),
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

    let selected_ids: BTreeSet<&str> = manifest
        .formulas
        .iter()
        .map(|formula| formula.identifier.as_str())
        .collect();
    let mut selected_count = 0usize;
    let mut non_release_count = 0usize;
    for registry_formula in &registry.formulas {
        let (requirements, scope) = if selected_ids.contains(registry_formula.formula_id.as_str()) {
            selected_count += 1;
            (&manifest.release_slice_requirements, "release slice")
        } else {
            non_release_count += 1;
            (&manifest.non_release_requirements, "non-release complement")
        };
        if registry_formula.status != requirements.validation_status {
            return Err(format!(
                "registry formula `{}` in {scope} has validation_status `{}`, expected `{}`",
                registry_formula.formula_id,
                registry_formula.status,
                requirements.validation_status
            ));
        }
        if registry_formula.execution_policy != requirements.execution_policy {
            return Err(format!(
                "registry formula `{}` in {scope} has execution_policy `{}`, expected `{}`",
                registry_formula.formula_id,
                registry_formula.execution_policy,
                requirements.execution_policy
            ));
        }
        let public_executable = matches!(
            registry_formula.execution_policy.as_str(),
            "normal_research" | "publication_supporting"
        );
        if public_executable != requirements.public_executable {
            return Err(format!(
                "registry formula `{}` in {scope} has derived public_executable={public_executable}, expected {}",
                registry_formula.formula_id, requirements.public_executable
            ));
        }
    }
    if selected_count != EXPECTED_FORMULA_IDS.len() || non_release_count != 140 {
        return Err(format!(
            "release formula partition mismatch: selected={selected_count}, non_release={non_release_count}; expected 12 and 140"
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
        if registry_formula.runtime_symbol.as_deref() != Some(formula.runtime_symbol.as_str()) {
            return Err(format!(
                "release formula `{}` runtime_symbol does not match the governed formula registry",
                formula.identifier
            ));
        }
    }

    println!(
        "verified release manifest: schema={}; version={}; channel={}; tier={}; packages={}; registry_formulas={}; release_slice={}; non_release={}; targets={}; artifacts={}; research_required=152; blocked=152; publicly_executable=0; production_state=declared_only",
        manifest.schema_version,
        manifest.release_version,
        manifest.release_channel,
        manifest.release_tier,
        manifest.packages.len(),
        manifest.registry_formula_count,
        selected_count,
        non_release_count,
        manifest.targets.len(),
        manifest.artifacts.len(),
    );
    Ok(())
}

pub fn load_release_manifest(root: &Path) -> Result<ReleaseManifest, String> {
    let path = root.join(RELEASE_MANIFEST_PATH);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {RELEASE_MANIFEST_PATH}: {error}"))?;
    parse_release_manifest(&text)
}

fn verify_authority_contract(root: &Path) -> Result<(), String> {
    if fs::symlink_metadata(root.join(OBSOLETE_RELEASE_MANIFEST_PATH)).is_ok() {
        return Err(format!(
            "obsolete release authority `{OBSOLETE_RELEASE_MANIFEST_PATH}` must be absent"
        ));
    }
    let release_metadata =
        fs::symlink_metadata(root.join(RELEASE_MANIFEST_PATH)).map_err(|error| {
            format!("sole release authority `{RELEASE_MANIFEST_PATH}` is missing: {error}")
        })?;
    if release_metadata.file_type().is_symlink() || !release_metadata.is_file() {
        return Err(format!(
            "sole release authority `{RELEASE_MANIFEST_PATH}` must be a regular file, not a symlink"
        ));
    }

    for script in [
        "scripts/friend_test_local.sh",
        "scripts/friend_test_local.ps1",
    ] {
        let text = fs::read_to_string(root.join(script))
            .map_err(|error| format!("cannot read {script}: {error}"))?;
        if !text.contains("verify-release-manifest") || !text.contains("verify-release-identity") {
            return Err(format!(
                "{script} must invoke both governed release verifiers"
            ));
        }
        if text.contains(RELEASE_MANIFEST_PATH) || text.contains(OBSOLETE_RELEASE_MANIFEST_PATH) {
            return Err(format!(
                "{script} must not contain a literal release-manifest authority path"
            ));
        }
    }

    for consumer in [
        "README.md",
        "docs/index.md",
        "docs/roadmap/research_readiness_counts.md",
    ] {
        let text = fs::read_to_string(root.join(consumer))
            .map_err(|error| format!("cannot read {consumer}: {error}"))?;
        if !text.contains(RELEASE_MANIFEST_PATH) || text.contains(OBSOLETE_RELEASE_MANIFEST_PATH) {
            return Err(format!(
                "current-authority consumer `{consumer}` must name only `{RELEASE_MANIFEST_PATH}`"
            ));
        }
    }

    let checksums = fs::read_to_string(root.join("checksums/SHA256SUMS"))
        .map_err(|error| format!("cannot read checksums/SHA256SUMS: {error}"))?;
    if !checksums
        .lines()
        .any(|line| line.ends_with(RELEASE_MANIFEST_PATH))
    {
        return Err(format!(
            "checksums/SHA256SUMS omits sole release authority `{RELEASE_MANIFEST_PATH}`"
        ));
    }
    if checksums
        .lines()
        .any(|line| line.ends_with(OBSOLETE_RELEASE_MANIFEST_PATH))
    {
        return Err(format!(
            "checksums/SHA256SUMS still names obsolete release authority `{OBSOLETE_RELEASE_MANIFEST_PATH}`"
        ));
    }
    Ok(())
}

fn verify_workspace_contract(root: &Path, manifest: &ReleaseManifest) -> Result<(), String> {
    let cargo = fs::read_to_string(root.join("Cargo.toml"))
        .map_err(|error| format!("cannot read root Cargo.toml: {error}"))?;
    let workspace_version = section_value(&cargo, "workspace.package", "version")?
        .ok_or_else(|| "root Cargo.toml is missing [workspace.package].version".to_string())?;
    if workspace_version != manifest.release_version {
        return Err(format!(
            "workspace version `{workspace_version}` does not match release manifest version `{}`",
            manifest.release_version
        ));
    }
    let members = workspace_members(&cargo)?;
    let manifest_members: Vec<&str> = manifest
        .packages
        .iter()
        .map(|package| {
            package
                .manifest_path
                .strip_suffix("/Cargo.toml")
                .unwrap_or(package.manifest_path.as_str())
        })
        .collect();
    if members.iter().map(String::as_str).collect::<Vec<_>>() != manifest_members {
        return Err(format!(
            "release package order/topology does not equal root workspace-member order: workspace={members:?}; manifest={manifest_members:?}"
        ));
    }
    for package in &manifest.packages {
        let text = fs::read_to_string(root.join(&package.manifest_path))
            .map_err(|error| format!("cannot read {}: {error}", package.manifest_path))?;
        let name = section_value(&text, "package", "name")?
            .ok_or_else(|| format!("{} is missing [package].name", package.manifest_path))?;
        if name != package.name {
            return Err(format!(
                "release package `{}` points to Cargo package `{name}` in {}",
                package.name, package.manifest_path
            ));
        }
        let inherits = section_raw_value(&text, "package", "version.workspace")?
            .is_some_and(|value| value == "true");
        let direct = section_value(&text, "package", "version")?;
        if !inherits && direct.as_deref() != Some(manifest.release_version.as_str()) {
            return Err(format!(
                "release package `{}` does not resolve to version `{}`",
                package.name, manifest.release_version
            ));
        }
    }
    Ok(())
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
    if manifest.formulas.len() != dispatch.len() {
        return Err(format!(
            "release manifest formula count {} does not match CLI dispatch metadata count {}",
            manifest.formulas.len(),
            dispatch.len()
        ));
    }

    let manifest_by_id: BTreeMap<&str, &str> = manifest
        .formulas
        .iter()
        .map(|formula| (formula.identifier.as_str(), formula.runtime_symbol.as_str()))
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
                (*manifest_symbol != *dispatch_symbol).then(|| {
                    format!("{identifier}: manifest={manifest_symbol}, dispatch={dispatch_symbol}")
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

#[derive(Debug, Default)]
struct RawTable {
    values: BTreeMap<String, String>,
    keys: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum ActiveTable {
    Top,
    ReleaseSlice,
    NonRelease,
    Package(usize),
    Formula(usize),
    Target(usize),
    Artifact(usize),
}

pub fn parse_release_manifest(text: &str) -> Result<ReleaseManifest, String> {
    for (index, byte) in text.as_bytes().iter().enumerate() {
        if *byte == b'\r' && text.as_bytes().get(index + 1) != Some(&b'\n') {
            return Err(format!(
                "release manifest byte {} contains a bare carriage return",
                index + 1
            ));
        }
    }

    let mut top = RawTable::default();
    let mut release_slice = None;
    let mut non_release = None;
    let mut packages = Vec::new();
    let mut formulas = Vec::new();
    let mut targets = Vec::new();
    let mut artifacts = Vec::new();
    let mut headers = Vec::new();
    let mut active = ActiveTable::Top;

    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            headers.push(line.to_string());
            active = match line {
                "[release_slice_requirements]" => {
                    if release_slice.replace(RawTable::default()).is_some() {
                        return Err("duplicate [release_slice_requirements] table".to_string());
                    }
                    ActiveTable::ReleaseSlice
                }
                "[non_release_requirements]" => {
                    if non_release.replace(RawTable::default()).is_some() {
                        return Err("duplicate [non_release_requirements] table".to_string());
                    }
                    ActiveTable::NonRelease
                }
                "[[packages]]" => {
                    packages.push(RawTable::default());
                    ActiveTable::Package(packages.len() - 1)
                }
                "[[formulas]]" => {
                    formulas.push(RawTable::default());
                    ActiveTable::Formula(formulas.len() - 1)
                }
                "[[targets]]" => {
                    targets.push(RawTable::default());
                    ActiveTable::Target(targets.len() - 1)
                }
                "[[artifacts]]" => {
                    artifacts.push(RawTable::default());
                    ActiveTable::Artifact(artifacts.len() - 1)
                }
                _ => {
                    return Err(format!(
                        "release manifest line {line_number} has unsupported table `{line}`"
                    ))
                }
            };
            continue;
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
        let table = match active {
            ActiveTable::Top => &mut top,
            ActiveTable::ReleaseSlice => release_slice.as_mut().expect("table exists"),
            ActiveTable::NonRelease => non_release.as_mut().expect("table exists"),
            ActiveTable::Package(index) => &mut packages[index],
            ActiveTable::Formula(index) => &mut formulas[index],
            ActiveTable::Target(index) => &mut targets[index],
            ActiveTable::Artifact(index) => &mut artifacts[index],
        };
        if table
            .values
            .insert(key.to_string(), value.to_string())
            .is_some()
        {
            return Err(format!(
                "release manifest line {line_number} duplicates key `{key}` in the same table"
            ));
        }
        table.keys.push(key.to_string());
    }

    let mut expected_headers = vec![
        "[release_slice_requirements]".to_string(),
        "[non_release_requirements]".to_string(),
    ];
    expected_headers.extend(std::iter::repeat("[[packages]]".to_string()).take(14));
    expected_headers.extend(std::iter::repeat("[[formulas]]".to_string()).take(12));
    expected_headers.extend(std::iter::repeat("[[targets]]".to_string()).take(4));
    expected_headers.extend(std::iter::repeat("[[artifacts]]".to_string()).take(9));
    if headers != expected_headers {
        return Err(format!(
            "release manifest tables are missing, extra, or noncanonical: found={headers:?}; expected={expected_headers:?}"
        ));
    }

    require_exact_keys(
        &top,
        &[
            "schema_version",
            "release_version",
            "release_channel",
            "release_tier",
            "release_tier_display",
            "program_name",
            "source_revision_policy",
            "base_commit",
            "registry_path",
            "registry_formula_count",
            "cli_dispatch_metadata",
            "release_validation_record",
            "release_documentation",
        ],
        "release top level",
    )?;

    let release_slice = parse_requirements(
        release_slice.as_ref().expect("header count requires table"),
        "release_slice_requirements",
    )?;
    let non_release = parse_requirements(
        non_release.as_ref().expect("header count requires table"),
        "non_release_requirements",
    )?;
    let manifest = ReleaseManifest {
        schema_version: required_string(&top.values, "schema_version", "release top level")?,
        release_version: required_string(&top.values, "release_version", "release top level")?,
        release_channel: required_string(&top.values, "release_channel", "release top level")?,
        release_tier: required_string(&top.values, "release_tier", "release top level")?,
        release_tier_display: required_string(
            &top.values,
            "release_tier_display",
            "release top level",
        )?,
        program_name: required_string(&top.values, "program_name", "release top level")?,
        source_revision_policy: required_string(
            &top.values,
            "source_revision_policy",
            "release top level",
        )?,
        base_commit: required_string(&top.values, "base_commit", "release top level")?,
        registry_path: required_string(&top.values, "registry_path", "release top level")?,
        registry_formula_count: required_usize(
            &top.values,
            "registry_formula_count",
            "release top level",
        )?,
        cli_dispatch_metadata: required_string(
            &top.values,
            "cli_dispatch_metadata",
            "release top level",
        )?,
        release_validation_record: required_string(
            &top.values,
            "release_validation_record",
            "release top level",
        )?,
        release_documentation: required_string(
            &top.values,
            "release_documentation",
            "release top level",
        )?,
        release_slice_requirements: release_slice,
        non_release_requirements: non_release,
        packages: packages
            .iter()
            .enumerate()
            .map(|(index, table)| parse_package(table, index + 1))
            .collect::<Result<Vec<_>, _>>()?,
        formulas: formulas
            .iter()
            .enumerate()
            .map(|(index, table)| parse_formula(table, index + 1))
            .collect::<Result<Vec<_>, _>>()?,
        targets: targets
            .iter()
            .enumerate()
            .map(|(index, table)| parse_target(table, index + 1))
            .collect::<Result<Vec<_>, _>>()?,
        artifacts: artifacts
            .iter()
            .enumerate()
            .map(|(index, table)| parse_artifact(table, index + 1))
            .collect::<Result<Vec<_>, _>>()?,
    };
    validate_release_manifest(&manifest)?;
    Ok(manifest)
}

fn parse_requirements(table: &RawTable, context: &str) -> Result<ReleaseRequirements, String> {
    require_exact_keys(
        table,
        &["validation_status", "execution_policy", "public_executable"],
        context,
    )?;
    Ok(ReleaseRequirements {
        validation_status: required_string(&table.values, "validation_status", context)?,
        execution_policy: required_string(&table.values, "execution_policy", context)?,
        public_executable: required_bool(&table.values, "public_executable", context)?,
    })
}

fn parse_package(table: &RawTable, index: usize) -> Result<ReleasePackage, String> {
    let context = format!("packages table {index}");
    require_exact_keys(table, &["name", "manifest_path"], &context)?;
    Ok(ReleasePackage {
        name: required_string(&table.values, "name", &context)?,
        manifest_path: required_string(&table.values, "manifest_path", &context)?,
    })
}

fn parse_formula(table: &RawTable, index: usize) -> Result<ReleaseFormula, String> {
    let context = format!("formulas table {index}");
    require_exact_keys(
        table,
        &[
            "identifier",
            "runtime_symbol",
            "validation_record",
            "documentation",
        ],
        &context,
    )?;
    Ok(ReleaseFormula {
        identifier: required_string(&table.values, "identifier", &context)?,
        runtime_symbol: required_string(&table.values, "runtime_symbol", &context)?,
        validation_record: required_string(&table.values, "validation_record", &context)?,
        documentation: required_string(&table.values, "documentation", &context)?,
    })
}

fn parse_target(table: &RawTable, index: usize) -> Result<ReleaseTarget, String> {
    let context = format!("targets table {index}");
    require_exact_keys(
        table,
        &[
            "triple",
            "platform_tier",
            "ci_posture",
            "artifact_eligible",
            "currently_tested",
            "currently_packaged",
            "current_support",
        ],
        &context,
    )?;
    Ok(ReleaseTarget {
        triple: required_string(&table.values, "triple", &context)?,
        platform_tier: required_string(&table.values, "platform_tier", &context)?,
        ci_posture: required_string(&table.values, "ci_posture", &context)?,
        artifact_eligible: required_bool(&table.values, "artifact_eligible", &context)?,
        currently_tested: required_bool(&table.values, "currently_tested", &context)?,
        currently_packaged: required_bool(&table.values, "currently_packaged", &context)?,
        current_support: required_string(&table.values, "current_support", &context)?,
    })
}

fn parse_artifact(table: &RawTable, index: usize) -> Result<ReleaseArtifact, String> {
    let context = format!("artifacts table {index}");
    let has_target = table.values.contains_key("target");
    let expected = if has_target {
        vec![
            "identifier",
            "filename",
            "kind",
            "target",
            "archive_format",
            "production_state",
            "required_by_task",
        ]
    } else {
        vec![
            "identifier",
            "filename",
            "kind",
            "archive_format",
            "production_state",
            "required_by_task",
        ]
    };
    require_exact_keys(table, &expected, &context)?;
    Ok(ReleaseArtifact {
        identifier: required_string(&table.values, "identifier", &context)?,
        filename: required_string(&table.values, "filename", &context)?,
        kind: required_string(&table.values, "kind", &context)?,
        target: optional_string(&table.values, "target", &context)?,
        archive_format: required_string(&table.values, "archive_format", &context)?,
        production_state: required_string(&table.values, "production_state", &context)?,
        required_by_task: required_string(&table.values, "required_by_task", &context)?,
    })
}

fn validate_release_manifest(manifest: &ReleaseManifest) -> Result<(), String> {
    validate_semver(&manifest.release_version)?;
    for (key, actual, expected) in [
        (
            "schema_version",
            manifest.schema_version.as_str(),
            RELEASE_SCHEMA_VERSION,
        ),
        (
            "release_version",
            manifest.release_version.as_str(),
            RELEASE_VERSION,
        ),
        (
            "release_channel",
            manifest.release_channel.as_str(),
            RELEASE_CHANNEL,
        ),
        ("release_tier", manifest.release_tier.as_str(), RELEASE_TIER),
        (
            "release_tier_display",
            manifest.release_tier_display.as_str(),
            RELEASE_TIER_DISPLAY,
        ),
        ("program_name", manifest.program_name.as_str(), "aerocodex"),
        (
            "source_revision_policy",
            manifest.source_revision_policy.as_str(),
            "pinned_base_commit",
        ),
        (
            "registry_path",
            manifest.registry_path.as_str(),
            REGISTRY_PATH,
        ),
        (
            "cli_dispatch_metadata",
            manifest.cli_dispatch_metadata.as_str(),
            CLI_DISPATCH_METADATA_PATH,
        ),
        (
            "release_validation_record",
            manifest.release_validation_record.as_str(),
            RELEASE_VALIDATION_RECORD,
        ),
        (
            "release_documentation",
            manifest.release_documentation.as_str(),
            RELEASE_DOCUMENTATION,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "release top-level key `{key}` has value `{actual}`, expected `{expected}`"
            ));
        }
    }
    if manifest.registry_formula_count != REGISTRY_FORMULA_COUNT {
        return Err(format!(
            "registry_formula_count={} must equal {REGISTRY_FORMULA_COUNT}",
            manifest.registry_formula_count
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
    for (context, requirements) in [
        (
            "release_slice_requirements",
            &manifest.release_slice_requirements,
        ),
        (
            "non_release_requirements",
            &manifest.non_release_requirements,
        ),
    ] {
        if requirements.validation_status != "research_required"
            || requirements.execution_policy != "blocked"
            || requirements.public_executable
        {
            return Err(format!(
                "{context} must be validation_status=research_required, execution_policy=blocked, public_executable=false; found {:?}",
                requirements
            ));
        }
    }

    let actual_packages: Vec<(&str, &str)> = manifest
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package.manifest_path.as_str()))
        .collect();
    if actual_packages != EXPECTED_PACKAGES {
        return Err(format!(
            "release packages must equal canonical workspace order/topology: found={actual_packages:?}; expected={EXPECTED_PACKAGES:?}"
        ));
    }
    for package in &manifest.packages {
        validate_repository_relative_path(&package.manifest_path).map_err(|error| {
            format!(
                "release package `{}` manifest_path is invalid: {error}",
                package.name
            )
        })?;
    }

    let mut formula_ids = BTreeSet::new();
    for formula in &manifest.formulas {
        if !formula_ids.insert(formula.identifier.as_str()) {
            return Err(format!(
                "duplicate release formula identifier `{}`",
                formula.identifier
            ));
        }
    }
    let actual_formula_ids: Vec<&str> = manifest
        .formulas
        .iter()
        .map(|formula| formula.identifier.as_str())
        .collect();
    if actual_formula_ids != EXPECTED_FORMULA_IDS {
        return Err(format!(
            "release formula set/order mismatch: found={actual_formula_ids:?}; expected={EXPECTED_FORMULA_IDS:?}"
        ));
    }
    let mut symbols = BTreeSet::new();
    for formula in &manifest.formulas {
        if formula.runtime_symbol.is_empty() || !symbols.insert(formula.runtime_symbol.as_str()) {
            return Err(format!(
                "duplicate or empty release formula runtime symbol `{}`",
                formula.runtime_symbol
            ));
        }
        validate_repository_relative_path(&formula.validation_record).map_err(|error| {
            format!(
                "release formula `{}` validation_record is invalid: {error}",
                formula.identifier
            )
        })?;
        validate_repository_relative_path(&formula.documentation).map_err(|error| {
            format!(
                "release formula `{}` documentation is invalid: {error}",
                formula.identifier
            )
        })?;
    }

    let actual_targets: Vec<(&str, &str, &str)> = manifest
        .targets
        .iter()
        .map(|target| {
            (
                target.triple.as_str(),
                target.platform_tier.as_str(),
                target.ci_posture.as_str(),
            )
        })
        .collect();
    if actual_targets != EXPECTED_TARGETS {
        return Err(format!(
            "release target set/tier/CI mapping mismatch: found={actual_targets:?}; expected={EXPECTED_TARGETS:?}"
        ));
    }
    for target in &manifest.targets {
        if !target.artifact_eligible
            || target.currently_tested
            || target.currently_packaged
            || target.current_support != "source_only"
        {
            return Err(format!(
                "target `{}` must be artifact_eligible=true, currently_tested=false, currently_packaged=false, current_support=source_only; found {:?}",
                target.triple, target
            ));
        }
    }

    let mut artifact_ids = BTreeSet::new();
    let mut artifact_filenames = BTreeSet::new();
    let target_set: BTreeSet<&str> = manifest
        .targets
        .iter()
        .map(|target| target.triple.as_str())
        .collect();
    let mut cli_targets = BTreeSet::new();
    for artifact in &manifest.artifacts {
        if !artifact_ids.insert(artifact.identifier.as_str()) {
            return Err(format!("duplicate artifact ID `{}`", artifact.identifier));
        }
        if !artifact_filenames.insert(artifact.filename.as_str()) {
            return Err(format!(
                "duplicate artifact filename `{}`",
                artifact.filename
            ));
        }
        validate_safe_artifact_basename(&artifact.filename)?;
        if artifact.production_state != "declared_only" {
            return Err(format!(
                "artifact `{}` production_state must be `declared_only`, found `{}`",
                artifact.identifier, artifact.production_state
            ));
        }
        match (artifact.kind.as_str(), artifact.target.as_deref()) {
            ("cli_archive", Some(target)) => {
                if !target_set.contains(target) {
                    return Err(format!(
                        "CLI artifact `{}` references unknown target `{target}`",
                        artifact.identifier
                    ));
                }
                if !cli_targets.insert(target) {
                    return Err(format!("duplicate CLI archive target `{target}`"));
                }
            }
            ("cli_archive", None) => {
                return Err(format!(
                    "CLI artifact `{}` is missing required target",
                    artifact.identifier
                ))
            }
            (_, Some(target)) => {
                return Err(format!(
                    "non-CLI artifact `{}` must not contain target `{target}`",
                    artifact.identifier
                ))
            }
            (_, None) => {}
        }
        validate_artifact_format(artifact)?;
    }
    if cli_targets != target_set {
        return Err(format!(
            "CLI archive targets must equal artifact-eligible targets: found={cli_targets:?}; expected={target_set:?}"
        ));
    }
    let actual_artifacts: Vec<ArtifactContract<'_>> = manifest
        .artifacts
        .iter()
        .map(|artifact| {
            (
                artifact.identifier.as_str(),
                artifact.filename.as_str(),
                artifact.kind.as_str(),
                artifact.target.as_deref(),
                artifact.archive_format.as_str(),
                artifact.required_by_task.as_str(),
            )
        })
        .collect();
    if actual_artifacts != EXPECTED_ARTIFACTS {
        return Err(format!(
            "release artifact set/contract mismatch: found={actual_artifacts:?}; expected={EXPECTED_ARTIFACTS:?}"
        ));
    }
    Ok(())
}

fn validate_safe_artifact_basename(filename: &str) -> Result<(), String> {
    if filename.is_empty()
        || filename == "."
        || filename == ".."
        || filename.starts_with('.')
        || filename.contains("..")
        || filename.contains('/')
        || filename.contains('\\')
        || has_windows_absolute_prefix(filename)
        || filename
            .bytes()
            .any(|byte| !byte.is_ascii_alphanumeric() && !matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(format!(
            "artifact filename `{filename}` must be a normalized safe ASCII basename"
        ));
    }
    Ok(())
}

fn validate_artifact_format(artifact: &ReleaseArtifact) -> Result<(), String> {
    let valid = match artifact.archive_format.as_str() {
        "tar.gz" => artifact.filename.ends_with(".tar.gz"),
        "zip" => artifact.filename.ends_with(".zip"),
        "none" => !artifact.filename.ends_with(".tar.gz") && !artifact.filename.ends_with(".zip"),
        other => {
            return Err(format!(
                "artifact `{}` has unsupported archive_format `{other}`",
                artifact.identifier
            ))
        }
    };
    if !valid {
        return Err(format!(
            "artifact `{}` kind/format/extension mismatch: kind=`{}`, archive_format=`{}`, filename=`{}`",
            artifact.identifier, artifact.kind, artifact.archive_format, artifact.filename
        ));
    }
    if artifact.kind == "cli_archive"
        && !matches!(artifact.archive_format.as_str(), "tar.gz" | "zip")
    {
        return Err(format!(
            "CLI artifact `{}` must use tar.gz or zip",
            artifact.identifier
        ));
    }
    if artifact.kind != "cli_archive"
        && !matches!(
            artifact.kind.as_str(),
            "source_archive" | "checksum" | "sbom" | "provenance" | "release_manifest"
        )
    {
        return Err(format!(
            "artifact `{}` has unsupported kind `{}`",
            artifact.identifier, artifact.kind
        ));
    }
    Ok(())
}

pub(crate) fn validate_semver(version: &str) -> Result<(), String> {
    if version.is_empty() || version.trim() != version {
        return Err(format!("release_version `{version}` is not valid SemVer"));
    }
    let (without_build, build) = match version.split_once('+') {
        Some((left, right)) if !right.is_empty() && !right.contains('+') => (left, Some(right)),
        Some(_) => return Err(format!("release_version `{version}` is not valid SemVer")),
        None => (version, None),
    };
    let (core, prerelease) = match without_build.split_once('-') {
        Some((left, right)) if !right.is_empty() => (left, Some(right)),
        Some(_) => return Err(format!("release_version `{version}` is not valid SemVer")),
        None => (without_build, None),
    };
    let core_parts: Vec<&str> = core.split('.').collect();
    if core_parts.len() != 3
        || core_parts.iter().any(|part| {
            part.is_empty()
                || !part.bytes().all(|byte| byte.is_ascii_digit())
                || (part.len() > 1 && part.starts_with('0'))
        })
    {
        return Err(format!("release_version `{version}` is not valid SemVer"));
    }
    for (identifiers, numeric_leading_zero_forbidden) in [(prerelease, true), (build, false)] {
        if let Some(identifiers) = identifiers {
            if identifiers.split('.').any(|identifier| {
                identifier.is_empty()
                    || !identifier
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                    || (numeric_leading_zero_forbidden
                        && identifier.len() > 1
                        && identifier.bytes().all(|byte| byte.is_ascii_digit())
                        && identifier.starts_with('0'))
            }) {
                return Err(format!("release_version `{version}` is not valid SemVer"));
            }
        }
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

fn require_exact_keys(table: &RawTable, expected: &[&str], context: &str) -> Result<(), String> {
    if let Some(key) = table
        .values
        .keys()
        .find(|key| !expected.contains(&key.as_str()))
    {
        return Err(format!("{context} has unknown metadata key `{key}`"));
    }
    if let Some(key) = expected
        .iter()
        .find(|key| !table.values.contains_key(**key))
    {
        return Err(format!("{context} is missing required metadata `{key}`"));
    }
    let actual: Vec<&str> = table.keys.iter().map(String::as_str).collect();
    if actual != expected {
        return Err(format!(
            "{context} keys are not in canonical order: found={actual:?}; expected={expected:?}"
        ));
    }
    Ok(())
}

fn section_raw_value(text: &str, section: &str, key: &str) -> Result<Option<String>, String> {
    let mut current = None;
    for (index, raw_line) in text.lines().enumerate() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.starts_with('[') && line.ends_with(']') {
            current = Some(line.trim_matches(&['[', ']'][..]).trim());
            continue;
        }
        if current != Some(section) || line.is_empty() {
            continue;
        }
        let Some((candidate, value)) = line.split_once('=') else {
            continue;
        };
        if candidate.trim() == key {
            return Ok(Some(value.trim().to_string()));
        }
        if candidate.trim().is_empty() {
            return Err(format!(
                "line {} in [{section}] has an empty Cargo key",
                index + 1
            ));
        }
    }
    Ok(None)
}

fn section_value(text: &str, section: &str, key: &str) -> Result<Option<String>, String> {
    section_raw_value(text, section, key)?
        .map(|raw| {
            parse_string(&raw)
                .map_err(|error| format!("[{section}] `{key}` must be a simple string: {error}"))
        })
        .transpose()
}

fn workspace_members(text: &str) -> Result<Vec<String>, String> {
    let mut in_workspace = false;
    let mut collecting = false;
    let mut members = Vec::new();
    for (index, raw_line) in text.lines().enumerate() {
        let line = raw_line.split('#').next().unwrap_or("").trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_workspace = line == "[workspace]";
            collecting = false;
            continue;
        }
        if !in_workspace {
            continue;
        }
        if !collecting {
            if line.starts_with("members") {
                let (_, value) = line.split_once('=').ok_or_else(|| {
                    format!("root Cargo.toml line {} has malformed members", index + 1)
                })?;
                if !value.trim().starts_with('[') {
                    return Err("root Cargo.toml workspace members must be an array".to_string());
                }
                collecting = true;
            } else {
                continue;
            }
        }
        for fragment in line.split(',') {
            let fragment = fragment.trim().trim_start_matches("members").trim();
            let fragment = fragment.trim_start_matches('=').trim();
            let fragment = fragment
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim();
            if fragment.is_empty() {
                continue;
            }
            members.push(parse_string(fragment).map_err(|error| {
                format!(
                    "root Cargo.toml workspace member on line {} is invalid: {error}",
                    index + 1
                )
            })?);
        }
        if line.contains(']') {
            break;
        }
    }
    if members.is_empty() {
        return Err("root Cargo.toml contains no workspace members".to_string());
    }
    Ok(members)
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

    fn canonical_manifest() -> String {
        include_str!("../../release/release-manifest.toml").to_string()
    }

    fn replace_once(text: &str, old: &str, new: &str) -> String {
        assert_eq!(
            text.matches(old).count(),
            1,
            "fixture must match once: {old}"
        );
        text.replacen(old, new, 1)
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
        let parsed =
            parse_release_manifest(&canonical_manifest()).expect("valid release manifest parses");
        assert_eq!(parsed.release_version, "0.1.0-alpha.1");
        assert_eq!(parsed.release_channel, "github_releases");
        assert_eq!(parsed.release_tier, "research_software_alpha");
        assert_eq!(parsed.packages.len(), 14);
        assert_eq!(parsed.formulas.len(), 12);
        assert_eq!(parsed.targets.len(), 4);
        assert_eq!(parsed.artifacts.len(), 9);
        assert!(!parsed.release_slice_requirements.public_executable);
        assert!(!parsed.non_release_requirements.public_executable);
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
        let text = canonical_manifest().replace("release_tier = \"research_software_alpha\"\n", "");
        let error = parse_release_manifest(&text).expect_err("missing tier must fail");
        assert!(error.contains("missing required metadata `release_tier`"));
    }

    #[test]
    fn duplicate_formula_identifiers_fail() {
        let text = replace_once(
            &canonical_manifest(),
            "identifier = \"m00.angle.rad_to_deg\"",
            "identifier = \"m00.angle.deg_to_rad\"",
        );
        let error = parse_release_manifest(&text).expect_err("duplicates must fail");
        assert!(error.contains("duplicate release formula identifier"));
    }

    #[test]
    fn duplicate_manifest_runtime_symbols_fail() {
        let text = replace_once(
            &canonical_manifest(),
            "runtime_symbol = \"m00_radians_to_degrees\"",
            "runtime_symbol = \"m00_degrees_to_radians\"",
        );
        let error = parse_release_manifest(&text).expect_err("duplicate symbols must fail");
        assert!(error.contains("duplicate or empty release formula runtime symbol"));
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
        let base = parse_release_manifest(&canonical_manifest()).expect("manifest fixture parses");
        let canonical_dispatch = parse_cli_dispatch_metadata(include_str!(
            "../../crates/aero-codex-cli/dispatch_metadata.tsv"
        ))
        .expect("checked dispatch metadata parses");
        validate_dispatch_matches_manifest(&base, &canonical_dispatch)
            .expect("canonical dispatch matches manifest");

        let mut replaced = canonical_dispatch.clone();
        replaced[0].canonical_formula_id = "fixture.replaced".to_string();
        let replaced_error = validate_dispatch_matches_manifest(&base, &replaced)
            .expect_err("replaced dispatch-linked formula must fail");
        assert!(replaced_error.contains("missing=[fixture.replaced]"));
        assert!(replaced_error.contains("extra=["), "{replaced_error}");

        let mut missing = canonical_dispatch.clone();
        missing.pop();
        assert!(validate_dispatch_matches_manifest(&base, &missing)
            .unwrap_err()
            .contains("formula count"));

        let mut extra = canonical_dispatch.clone();
        let mut record = extra[0].clone();
        record.canonical_formula_id = "fixture.extra".to_string();
        record.runtime_symbol = "fixture_extra".to_string();
        extra.push(record);
        assert!(validate_dispatch_matches_manifest(&base, &extra)
            .unwrap_err()
            .contains("formula count"));

        let mut wrong = canonical_dispatch;
        wrong[0].runtime_symbol = "wrong_symbol".to_string();
        let wrong_symbol = validate_dispatch_matches_manifest(&base, &wrong)
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
        let text = replace_first(
            &canonical_manifest(),
            "execution_policy = \"blocked\"",
            "execution_policy = \"normal_research\"",
        );
        let error = parse_release_manifest(&text)
            .expect_err("research_required cannot be normal execution");
        assert!(error.contains("release_slice_requirements must be"));
    }

    #[test]
    fn executable_formula_without_validation_reference_fails() {
        let text = replace_first(
            &canonical_manifest(),
            "public_executable = false",
            "public_executable = true",
        );
        let error = parse_release_manifest(&text)
            .expect_err("public execution in release requirements must fail");
        assert!(error.contains("public_executable=false"));
    }

    fn replace_first(text: &str, old: &str, new: &str) -> String {
        assert!(text.contains(old), "fixture must contain: {old}");
        text.replacen(old, new, 1)
    }

    #[test]
    fn canonical_v3_negative_matrix_fails_closed() {
        let base = canonical_manifest();
        let cases = vec![
            ("missing release version", replace_first(&base, "release_version = \"0.1.0-alpha.1\"\n", "")),
            ("malformed release version", replace_first(&base, "release_version = \"0.1.0-alpha.1\"", "release_version = \"0.1\"")),
            ("wrong release version", replace_first(&base, "release_version = \"0.1.0-alpha.1\"", "release_version = \"0.1.0-alpha.2\"")),
            ("missing channel", replace_first(&base, "release_channel = \"github_releases\"\n", "")),
            ("invalid channel", replace_first(&base, "release_channel = \"github_releases\"", "release_channel = \"nightly\"")),
            ("tier substituted for channel", replace_first(&base, "release_channel = \"github_releases\"", "release_channel = \"research_software_alpha\"")),
            ("missing schema", replace_first(&base, "schema_version = \"aerocodex.release_manifest.v3\"\n", "")),
            ("old schema", replace_first(&base, "aerocodex.release_manifest.v3", "aerocodex.release_manifest.v2")),
            ("unknown schema", replace_first(&base, "aerocodex.release_manifest.v3", "aerocodex.release_manifest.v99")),
            ("unknown top key", replace_first(&base, "release_documentation = \"docs/release/v0.1.0-alpha.1-status.md\"", "release_documentation = \"docs/release/v0.1.0-alpha.1-status.md\"\nunknown = \"value\"")),
            ("duplicate top key", replace_first(&base, "release_channel = \"github_releases\"", "release_channel = \"github_releases\"\nrelease_channel = \"github_releases\"")),
            ("unknown table", replace_first(&base, "[release_slice_requirements]", "[release_requirements]")),
            ("unknown nested key", replace_first(&base, "validation_status = \"research_required\"", "validation_state = \"research_required\"")),
            ("old derivative field", replace_first(&base, "registry_formula_count = 152", "registry_formula_count = 152\nregistry_blocked_formula_count = 152")),
            ("missing release requirement", replace_first(&base, "validation_status = \"research_required\"\n", "")),
            ("status requirement mismatch", replace_first(&base, "validation_status = \"research_required\"", "validation_status = \"equation_traceable\"")),
            ("execution requirement mismatch", replace_first(&base, "execution_policy = \"blocked\"", "execution_policy = \"normal_research\"")),
            ("public release requirement", replace_first(&base, "public_executable = false", "public_executable = true")),
            ("non-release requirement mismatch", base.replacen("validation_status = \"research_required\"", "validation_status = \"implementation_verified\"", 2)),
            ("duplicate formula ID", replace_first(&base, "identifier = \"m00.angle.rad_to_deg\"", "identifier = \"m00.angle.deg_to_rad\"")),
            ("duplicate runtime symbol", replace_first(&base, "runtime_symbol = \"m00_radians_to_degrees\"", "runtime_symbol = \"m00_degrees_to_radians\"")),
            ("unknown formula", replace_first(&base, "identifier = \"m00.angle.deg_to_rad\"", "identifier = \"m00.unknown\"")),
            ("formula evidence absolute", replace_first(&base, "validation_record = \"validation/cards/validation_formula_vault_m00_angle_unit_conversions.yaml\"", "validation_record = \"C:/outside.yaml\"")),
            ("formula evidence traversal", replace_first(&base, "documentation = \"docs/research_alpha/cli_formula_quickstart.md\"", "documentation = \"../outside.md\"")),
            ("missing target", replace_first(&base, "[[targets]]\ntriple = \"aarch64-apple-darwin\"\n", "")),
            ("extra target", replace_first(&base, "[[artifacts]]\nidentifier = \"checksums\"", "[[targets]]\ntriple = \"fixture\"\nplatform_tier = \"tier1\"\nci_posture = \"blocking\"\nartifact_eligible = true\ncurrently_tested = false\ncurrently_packaged = false\ncurrent_support = \"source_only\"\n\n[[artifacts]]\nidentifier = \"checksums\"")),
            ("duplicate target", replace_first(&base, "triple = \"x86_64-apple-darwin\"", "triple = \"aarch64-apple-darwin\"")),
            ("unknown target tier", replace_first(&base, "platform_tier = \"tier2\"", "platform_tier = \"tier3\"")),
            ("wrong target tier mapping", replace_first(&base, "platform_tier = \"tier2\"", "platform_tier = \"tier1\"")),
            ("wrong CI posture", replace_first(&base, "ci_posture = \"required_tier2\"", "ci_posture = \"blocking\"")),
            ("target currently tested", replace_first(&base, "currently_tested = false", "currently_tested = true")),
            ("target currently packaged", replace_first(&base, "currently_packaged = false", "currently_packaged = true")),
            ("unsupported current support", replace_first(&base, "current_support = \"source_only\"", "current_support = \"binary\"")),
            ("unknown target nested key", replace_first(&base, "current_support = \"source_only\"", "support = \"source_only\"")),
            ("artifact unknown target", replace_first(&base, "target = \"x86_64-unknown-linux-gnu\"", "target = \"unknown-target\"")),
            ("non-CLI artifact target", replace_first(&base, "kind = \"checksum\"\narchive_format", "kind = \"checksum\"\ntarget = \"x86_64-unknown-linux-gnu\"\narchive_format")),
            ("CLI artifact missing target", replace_first(&base, "target = \"x86_64-unknown-linux-gnu\"\n", "")),
            ("duplicate artifact ID", replace_first(&base, "identifier = \"cli-macos-aarch64\"", "identifier = \"cli-linux-x86-64\"")),
            ("duplicate artifact filename", replace_first(&base, "filename = \"aerocodex-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz\"", "filename = \"aerocodex-0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz\"")),
            ("unsafe artifact filename", replace_first(&base, "aerocodex-0.1.0-alpha.1-SHA256SUMS", "unsafe name")),
            ("absolute artifact filename", replace_first(&base, "aerocodex-0.1.0-alpha.1-SHA256SUMS", "/tmp/SHA256SUMS")),
            ("artifact traversal", replace_first(&base, "aerocodex-0.1.0-alpha.1-SHA256SUMS", "../SHA256SUMS")),
            ("artifact backslash", replace_first(&base, "aerocodex-0.1.0-alpha.1-SHA256SUMS", "dir\\SHA256SUMS")),
            ("artifact drive prefix", replace_first(&base, "aerocodex-0.1.0-alpha.1-SHA256SUMS", "C:/SHA256SUMS")),
            ("unsupported archive format", replace_first(&base, "archive_format = \"none\"", "archive_format = \"7z\"")),
            ("kind format mismatch", replace_first(&base, "archive_format = \"none\"", "archive_format = \"zip\"")),
            ("filename version mismatch", replace_first(&base, "aerocodex-0.1.0-alpha.1-SHA256SUMS", "aerocodex-0.1.0-alpha.2-SHA256SUMS")),
            ("filename target mismatch", replace_first(&base, "aerocodex-0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz", "aerocodex-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz")),
            ("missing artifact", replace_first(&base, "[[artifacts]]\nidentifier = \"source-archive\"", "identifier = \"source-archive\"")),
            ("extra artifact", format!("{base}\n[[artifacts]]\nidentifier = \"extra\"\nfilename = \"extra.zip\"\nkind = \"source_archive\"\narchive_format = \"zip\"\nproduction_state = \"declared_only\"\nrequired_by_task = \"R4-09\"\n")),
            ("artifact produced", replace_first(&base, "production_state = \"declared_only\"", "production_state = \"built\"")),
            ("invalid required task", replace_first(&base, "required_by_task = \"R5-09\"", "required_by_task = \"R7-01\"")),
            ("unknown artifact field", replace_first(&base, "required_by_task = \"R5-09\"", "required_by_task = \"R5-09\"\nself_hash = \"no\"")),
            ("top-level order drift", replace_first(&base, "release_version = \"0.1.0-alpha.1\"\nrelease_channel = \"github_releases\"", "release_channel = \"github_releases\"\nrelease_version = \"0.1.0-alpha.1\"")),
            ("requirement order drift", replace_first(&base, "validation_status = \"research_required\"\nexecution_policy = \"blocked\"", "execution_policy = \"blocked\"\nvalidation_status = \"research_required\"")),
            ("package key order drift", replace_first(&base, "name = \"aero-codex-core\"\nmanifest_path = \"crates/aero-codex-core/Cargo.toml\"", "manifest_path = \"crates/aero-codex-core/Cargo.toml\"\nname = \"aero-codex-core\"")),
            ("formula key order drift", replace_first(&base, "identifier = \"m00.angle.deg_to_rad\"\nruntime_symbol = \"m00_degrees_to_radians\"", "runtime_symbol = \"m00_degrees_to_radians\"\nidentifier = \"m00.angle.deg_to_rad\"")),
            ("target key order drift", replace_first(&base, "triple = \"aarch64-apple-darwin\"\nplatform_tier = \"tier2\"", "platform_tier = \"tier2\"\ntriple = \"aarch64-apple-darwin\"")),
            ("artifact key order drift", replace_first(&base, "identifier = \"checksums\"\nfilename = \"aerocodex-0.1.0-alpha.1-SHA256SUMS\"", "filename = \"aerocodex-0.1.0-alpha.1-SHA256SUMS\"\nidentifier = \"checksums\"")),
        ];
        assert!(cases.len() >= 58, "negative matrix must remain complete");
        for (name, text) in cases {
            assert!(
                parse_release_manifest(&text).is_err(),
                "negative production-parser case unexpectedly passed: {name}"
            );
        }
    }

    #[test]
    fn canonical_parser_is_platform_and_newline_stable() {
        let lf = canonical_manifest();
        assert!(!lf.contains('\r'));
        let expected = parse_release_manifest(&lf).unwrap();
        let crlf = lf.replace('\n', "\r\n");
        assert_eq!(parse_release_manifest(&crlf).unwrap(), expected);
        let mut mixed = String::new();
        for (index, line) in lf.split_inclusive('\n').enumerate() {
            let line = line.strip_suffix('\n').unwrap_or(line);
            mixed.push_str(line);
            mixed.push_str(if index % 2 == 0 { "\r\n" } else { "\n" });
        }
        assert_eq!(parse_release_manifest(&mixed).unwrap(), expected);
        assert!(parse_release_manifest(&lf.replacen('\n', "\r", 1))
            .unwrap_err()
            .contains("bare carriage return"));
    }

    #[test]
    fn dependency_free_semver_validation_rejects_malformed_values() {
        for invalid in [
            "",
            "0.1",
            "01.1.0",
            "0.1.0-",
            "0.1.0-alpha.01",
            "0.1.0+",
            "0.1.0+bad value",
        ] {
            assert!(validate_semver(invalid).is_err(), "accepted `{invalid}`");
        }
        for valid in ["0.1.0", "0.1.0-alpha.1", "1.2.3-rc.1+build.7"] {
            validate_semver(valid).unwrap();
        }
    }

    #[test]
    fn checked_repository_has_one_authority_and_friend_script_agreement() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        verify_authority_contract(root.parent().expect("xtask has repository parent")).unwrap();
    }
}
