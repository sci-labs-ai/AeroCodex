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
        cell::Cell,
        collections::hash_map::RandomState,
        hash::{BuildHasher, Hash, Hasher},
        io::{self, ErrorKind},
        panic::{catch_unwind, AssertUnwindSafe},
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn canonical_manifest() -> String {
        include_str!("../../release/release-manifest.toml").to_string()
    }

    fn canonical_manifest_with_newline(newline: &str) -> String {
        assert!(matches!(newline, "\n" | "\r\n"));
        canonical_manifest().replace("\r\n", "\n").replace('\n', newline)
    }

    fn fixture_fragment_with_text_newline(text: &str, fragment: &str) -> String {
        let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
        fragment.replace("\r\n", "\n").replace('\n', newline)
    }

    fn replace_text_occurrence(
        text: &str,
        old: &str,
        new: &str,
        expected_pre_count: usize,
        occurrence_index: usize,
        expected_post_count: usize,
    ) -> String {
        assert!(!old.is_empty(), "fixture replacement needle must not be empty");
        let actual_pre_count = text.matches(old).count();
        assert_eq!(
            actual_pre_count, expected_pre_count,
            "fixture replacement expected {expected_pre_count} pre-mutation occurrences of `{old}`, found {actual_pre_count}"
        );
        assert!(
            occurrence_index < actual_pre_count,
            "fixture replacement occurrence index {occurrence_index} is out of range for {actual_pre_count} occurrences of `{old}`"
        );

        let start = text
            .match_indices(old)
            .nth(occurrence_index)
            .expect("validated fixture occurrence index")
            .0;
        let end = start + old.len();
        let mut candidate = String::with_capacity(text.len() - old.len() + new.len());
        candidate.push_str(&text[..start]);
        candidate.push_str(new);
        candidate.push_str(&text[end..]);

        assert_ne!(
            candidate.as_bytes(),
            text.as_bytes(),
            "fixture replacement must change bytes"
        );

        let actual_post_count = candidate.matches(old).count();
        assert_eq!(
            actual_post_count, expected_post_count,
            "fixture replacement expected {expected_post_count} post-mutation occurrences of `{old}`, found {actual_post_count}"
        );
        candidate
    }

    fn replace_manifest_text_occurrence(
        text: &str,
        old: &str,
        new: &str,
        expected_pre_count: usize,
        occurrence_index: usize,
        expected_post_count: usize,
    ) -> String {
        let old = fixture_fragment_with_text_newline(text, old);
        let new = fixture_fragment_with_text_newline(text, new);
        replace_text_occurrence(
            text,
            &old,
            &new,
            expected_pre_count,
            occurrence_index,
            expected_post_count,
        )
    }

    fn replace_once(text: &str, old: &str, new: &str, expected_post_count: usize) -> String {
        replace_text_occurrence(text, old, new, 1, 0, expected_post_count)
    }

    struct ParserNegativeCase {
        name: &'static str,
        old: &'static str,
        new: &'static str,
        expected_pre_count: usize,
        expected_post_count: usize,
        expected: &'static [&'static str],
    }

    #[derive(Clone, Copy)]
    enum SemanticMutation {
        RemoveTarget(&'static str),
        AddTarget,
        SetArtifactFilename {
            identifier: &'static str,
            filename: &'static str,
        },
        RemoveArtifact(&'static str),
        AddArtifact,
    }

    struct SemanticNegativeCase {
        name: &'static str,
        mutation: SemanticMutation,
        expected: &'static [&'static str],
    }

    fn assert_discriminating_error(name: &str, error: &str, expected: &[&str]) {
        assert!(
            !expected.is_empty(),
            "negative case `{name}` must declare a discriminating diagnostic"
        );
        for fragment in expected {
            assert!(
                error.contains(fragment),
                "negative case `{name}` reached the wrong production guard: expected fragment `{fragment}`, actual error: {error}"
            );
        }
    }

    fn repository_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask has repository parent")
            .to_path_buf()
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

    const FIXTURE_CREATE_ATTEMPTS: usize = 64;

    struct OwnedFixtureRoot {
        root: PathBuf,
        cleanup_armed: bool,
    }

    impl OwnedFixtureRoot {
        fn create(name: &str) -> io::Result<Self> {
            let parent = std::env::temp_dir();
            let serial = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or_default();
            let nonce = Self::unique_component(name, serial, now);
            Self::create_with_candidate_source(|attempt| {
                parent.join(format!(
                    "aerocodex-release-fixture-{name}-{nonce}-{attempt:02}"
                ))
            })
        }

        fn unique_component(name: &str, serial: usize, now: u128) -> String {
            fn keyed_hash(domain: u64, name: &str, serial: usize, now: u128) -> u64 {
                let mut hasher = RandomState::new().build_hasher();
                domain.hash(&mut hasher);
                name.hash(&mut hasher);
                std::process::id().hash(&mut hasher);
                serial.hash(&mut hasher);
                now.hash(&mut hasher);
                hasher.finish()
            }

            format!(
                "{:016x}{:016x}",
                keyed_hash(0x4f574e45445f524f, name, serial, now),
                keyed_hash(0x4f4f545f4e4f4e43, name, serial, now)
            )
        }

        fn create_with_candidate_source<F>(mut candidate_source: F) -> io::Result<Self>
        where
            F: FnMut(usize) -> PathBuf,
        {
            for attempt in 0..FIXTURE_CREATE_ATTEMPTS {
                let candidate = candidate_source(attempt);
                match fs::create_dir(&candidate) {
                    Ok(()) => {
                        return Ok(Self {
                            root: candidate,
                            cleanup_armed: true,
                        })
                    }
                    Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                    Err(error) => return Err(error),
                }
            }
            Err(io::Error::new(
                ErrorKind::AlreadyExists,
                format!(
                    "could not atomically create an owned fixture root after {FIXTURE_CREATE_ATTEMPTS} collision retries"
                ),
            ))
        }

        fn root(&self) -> &Path {
            &self.root
        }

        fn path(&self, relative: &str) -> PathBuf {
            self.root.join(relative)
        }

        fn cleanup(self) -> io::Result<()> {
            self.cleanup_with(|path| fs::remove_dir_all(path))
        }

        fn cleanup_with<F>(mut self, remove: F) -> io::Result<()>
        where
            F: FnOnce(&Path) -> io::Result<()>,
        {
            match remove(&self.root) {
                Ok(()) => {
                    self.cleanup_armed = false;
                    Ok(())
                }
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    self.cleanup_armed = false;
                    Ok(())
                }
                Err(error) => Err(error),
            }
        }
    }

    impl Drop for OwnedFixtureRoot {
        fn drop(&mut self) {
            if self.cleanup_armed {
                match fs::remove_dir_all(&self.root) {
                    Ok(()) => self.cleanup_armed = false,
                    Err(error) if error.kind() == ErrorKind::NotFound => {
                        self.cleanup_armed = false;
                    }
                    Err(_) => {}
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct FixtureTreeSnapshot {
        entries: Vec<FixtureTreeEntry>,
        directory_count: usize,
        file_count: usize,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct FixtureTreeEntry {
        relative_path: String,
        file_type: String,
        length: u64,
        sha256: Option<String>,
        readonly: bool,
        created_nanos: Option<u128>,
        modified_nanos: Option<u128>,
        symlink_target: Option<PathBuf>,
    }

    fn fixture_tree_snapshot(root: &Path) -> io::Result<FixtureTreeSnapshot> {
        fn system_time_nanos(value: io::Result<SystemTime>) -> Option<u128> {
            value
                .ok()
                .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_nanos())
        }

        fn visit(
            root: &Path,
            current: &Path,
            entries: &mut Vec<FixtureTreeEntry>,
        ) -> io::Result<()> {
            let metadata = fs::symlink_metadata(current)?;
            let file_type = metadata.file_type();
            let kind = if file_type.is_dir() {
                "directory"
            } else if file_type.is_file() {
                "file"
            } else if file_type.is_symlink() {
                "symlink"
            } else {
                "other"
            };
            let relative_path = if current == root {
                ".".to_string()
            } else {
                current
                    .strip_prefix(root)
                    .map_err(|error| io::Error::new(ErrorKind::InvalidInput, error))?
                    .to_string_lossy()
                    .replace('\\', "/")
            };
            entries.push(FixtureTreeEntry {
                relative_path,
                file_type: kind.to_string(),
                length: metadata.len(),
                sha256: if file_type.is_file() {
                    Some(crate::equation_batch::generate::sha256_hex(&fs::read(
                        current,
                    )?))
                } else {
                    None
                },
                readonly: metadata.permissions().readonly(),
                created_nanos: system_time_nanos(metadata.created()),
                modified_nanos: system_time_nanos(metadata.modified()),
                symlink_target: if file_type.is_symlink() {
                    Some(fs::read_link(current)?)
                } else {
                    None
                },
            });

            if file_type.is_dir() {
                let mut children = fs::read_dir(current)?
                    .map(|entry| entry.map(|entry| entry.path()))
                    .collect::<io::Result<Vec<_>>>()?;
                children.sort();
                for child in children {
                    visit(root, &child, entries)?;
                }
            }
            Ok(())
        }

        let mut entries = Vec::new();
        visit(root, root, &mut entries)?;
        Ok(FixtureTreeSnapshot {
            directory_count: entries
                .iter()
                .filter(|entry| entry.file_type == "directory")
                .count(),
            file_count: entries
                .iter()
                .filter(|entry| entry.file_type == "file")
                .count(),
            entries,
        })
    }

    fn git_root(name: &str) -> OwnedFixtureRoot {
        let fixture = OwnedFixtureRoot::create(&format!("git-{name}"))
            .expect("atomically create owned release manifest Git fixture");
        git(fixture.root(), &["init", "-b", "main"]);
        git(
            fixture.root(),
            &["config", "user.email", "tests@example.invalid"],
        );
        git(fixture.root(), &["config", "user.name", "AeroCodex Tests"]);
        fs::write(fixture.path("fixture.txt"), b"base\n").expect("write base fixture");
        git(fixture.root(), &["add", "."]);
        git(fixture.root(), &["commit", "-m", "base"]);
        fixture
    }

    struct AuthorityFixture {
        owner: OwnedFixtureRoot,
    }

    impl AuthorityFixture {
        fn new(name: &str) -> Self {
            let fixture = Self {
                owner: OwnedFixtureRoot::create(&format!("authority-{name}"))
                    .expect("atomically create owned authority fixture"),
            };
            for relative in [
                RELEASE_MANIFEST_PATH,
                "scripts/friend_test_local.sh",
                "scripts/friend_test_local.ps1",
                "README.md",
                "docs/index.md",
                "docs/roadmap/research_readiness_counts.md",
                "checksums/SHA256SUMS",
            ] {
                fixture.copy_from_repository(relative);
            }
            fixture
        }

        fn root(&self) -> &Path {
            self.owner.root()
        }

        fn path(&self, relative: &str) -> PathBuf {
            self.owner.path(relative)
        }

        fn copy_from_repository(&self, relative: &str) {
            let destination = self.path(relative);
            fs::create_dir_all(destination.parent().expect("fixture path has parent"))
                .expect("create disposable authority repository directory");
            fs::copy(repository_root().join(relative), &destination)
                .unwrap_or_else(|error| panic!("copy authority fixture `{relative}`: {error}"));
        }

        fn read(&self, relative: &str) -> String {
            fs::read_to_string(self.path(relative))
                .unwrap_or_else(|error| panic!("read authority fixture `{relative}`: {error}"))
        }

        fn write(&self, relative: &str, text: &str) {
            fs::write(self.path(relative), text)
                .unwrap_or_else(|error| panic!("write authority fixture `{relative}`: {error}"));
        }

        fn create_parent(&self, relative: &str) {
            fs::create_dir_all(
                self.path(relative)
                    .parent()
                    .expect("fixture path has parent"),
            )
            .unwrap_or_else(|error| {
                panic!("create authority fixture parent for `{relative}`: {error}")
            });
        }

        fn cleanup(self) -> io::Result<()> {
            self.owner.cleanup()
        }
    }

    #[test]
    fn owned_fixture_root_retries_collision_without_touching_preexisting_path() {
        let regression = OwnedFixtureRoot::create("collision-regression")
            .expect("atomically create collision regression parent");
        let sentinel = regression.path("sentinel-collision");
        fs::create_dir(&sentinel).expect("atomically create owned collision sentinel");
        fs::create_dir(sentinel.join("nested")).expect("create nested sentinel directory");
        fs::write(sentinel.join("nested/preserved.bin"), b"preserve me\n")
            .expect("write nested sentinel content");
        let before = fixture_tree_snapshot(&sentinel).expect("snapshot collision sentinel");

        let retry = regression.path("owned-retry");
        let attempts = Cell::new(0usize);
        let fixture = OwnedFixtureRoot::create_with_candidate_source(|attempt| {
            attempts.set(attempts.get() + 1);
            match attempt {
                0 => sentinel.clone(),
                1 => retry.clone(),
                _ => regression.path(&format!("unexpected-retry-{attempt}")),
            }
        })
        .expect("retry collision without changing the pre-existing candidate");

        assert_eq!(attempts.get(), 2, "allocator must retry exactly once");
        assert_ne!(fixture.root(), sentinel.as_path());
        assert_eq!(fixture.root(), retry.as_path());
        fs::write(fixture.path("owned.txt"), b"owned\n")
            .expect("write only inside newly owned fixture root");
        let owned_root = fixture.root().to_path_buf();
        fixture
            .cleanup()
            .expect("explicitly remove newly owned retry root");
        assert!(!owned_root.exists(), "owned retry root must be removed");

        assert!(sentinel.is_dir(), "collision sentinel must remain present");
        let after = fixture_tree_snapshot(&sentinel).expect("resnapshot collision sentinel");
        assert_eq!(after, before, "collision sentinel changed during retry");
        fs::remove_dir_all(&sentinel).expect("remove regression-owned collision sentinel");
        regression
            .cleanup()
            .expect("remove collision regression parent");
    }

    #[test]
    fn owned_fixture_root_cleanup_is_non_panicking_during_unwind() {
        const SENTINEL_PANIC: &str = "owned fixture unwind sentinel";

        let unrelated = OwnedFixtureRoot::create("unwind-unrelated")
            .expect("atomically create unrelated preservation fixture");
        fs::write(unrelated.path("preserved.txt"), b"unrelated\n")
            .expect("write unrelated sentinel content");
        let unrelated_before =
            fixture_tree_snapshot(unrelated.root()).expect("snapshot unrelated fixture");

        let fixture =
            OwnedFixtureRoot::create("unwind").expect("atomically create owned unwind fixture");
        fs::create_dir(fixture.path("nested")).expect("create nested unwind directory");
        fs::write(fixture.path("nested/file.txt"), b"owned\n").expect("write nested unwind file");
        let root = fixture.root().to_path_buf();
        let panic = catch_unwind(AssertUnwindSafe(|| {
            let _fixture = fixture;
            std::panic::panic_any(SENTINEL_PANIC);
        }))
        .expect_err("sentinel panic must escape the fixture scope");

        assert_eq!(
            panic.downcast_ref::<&str>().copied(),
            Some(SENTINEL_PANIC),
            "fixture Drop replaced the exact sentinel panic payload"
        );
        assert!(
            !root.exists(),
            "Drop must remove the exact owned unwind root"
        );
        assert_eq!(
            fixture_tree_snapshot(unrelated.root()).expect("resnapshot unrelated fixture"),
            unrelated_before,
            "unwind cleanup touched an unrelated owned path"
        );
        unrelated
            .cleanup()
            .expect("remove unrelated preservation fixture");
    }

    #[test]
    fn owned_fixture_root_explicit_cleanup_surfaces_errors() {
        let successful = OwnedFixtureRoot::create("cleanup-success")
            .expect("atomically create successful cleanup fixture");
        fs::write(successful.path("nested.txt"), b"owned\n")
            .expect("write successful cleanup content");
        let successful_root = successful.root().to_path_buf();
        successful.cleanup().expect("explicit cleanup must succeed");
        assert!(!successful_root.exists());

        let unrelated = OwnedFixtureRoot::create("cleanup-unrelated")
            .expect("atomically create cleanup preservation fixture");
        fs::write(unrelated.path("preserved.txt"), b"unrelated\n")
            .expect("write cleanup preservation content");
        let unrelated_before =
            fixture_tree_snapshot(unrelated.root()).expect("snapshot cleanup preservation fixture");

        let failing = OwnedFixtureRoot::create("cleanup-error")
            .expect("atomically create failing cleanup fixture");
        fs::write(failing.path("nested.txt"), b"owned\n").expect("write failing cleanup content");
        let failing_root = failing.root().to_path_buf();
        let error = failing
            .cleanup_with(|path| {
                assert_eq!(path, failing_root.as_path());
                Err(io::Error::new(
                    ErrorKind::PermissionDenied,
                    "injected normal-cleanup failure",
                ))
            })
            .expect_err("normal cleanup error must be returned");
        assert_eq!(error.kind(), ErrorKind::PermissionDenied);
        assert_eq!(error.to_string(), "injected normal-cleanup failure");
        assert!(
            !failing_root.exists(),
            "best-effort Drop must remain non-panicking after returned cleanup error"
        );
        assert_eq!(
            fixture_tree_snapshot(unrelated.root())
                .expect("resnapshot cleanup preservation fixture"),
            unrelated_before,
            "cleanup error handling touched an unrelated owned path"
        );
        unrelated
            .cleanup()
            .expect("remove cleanup preservation fixture");
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
        let fixture = OwnedFixtureRoot::create("containment")
            .expect("atomically create owned containment fixture");
        fs::create_dir_all(fixture.path("docs")).expect("create evidence directory");
        fs::write(fixture.path("docs/evidence.md"), b"evidence\n").expect("write evidence");
        require_existing_repository_file(fixture.root(), "docs/evidence.md")
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
            let error = require_existing_repository_file(fixture.root(), reference)
                .expect_err("invalid evidence reference must fail");
            assert!(error.contains(expected), "unexpected error: {error}");
        }
        fixture
            .cleanup()
            .expect("remove owned release reference fixture");
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn repository_evidence_rejects_symlinks_and_symlink_escapes() {
        let fixture = OwnedFixtureRoot::create("symlink-containment")
            .expect("atomically create owned symlink-containment fixture");
        let root = fixture.path("repository");
        let outside = fixture.path("outside");
        fs::create_dir(&root).expect("create simulated repository root");
        fs::create_dir(&outside).expect("create outside-repository fixture path");
        fs::write(root.join("inside.md"), b"inside\n").expect("write inside evidence");
        fs::write(outside.join("outside.md"), b"outside\n").expect("write outside evidence");

        if !create_file_symlink(Path::new("inside.md"), &root.join("inside-link.md")) {
            fixture
                .cleanup()
                .expect("remove owned symlink-containment fixture");
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
        fixture
            .cleanup()
            .expect("remove owned symlink-containment fixture");
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
            0,
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
            0,
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
        let fixture = git_root("objects");
        let base = git(fixture.root(), &["rev-parse", "HEAD"]);
        fs::write(fixture.path("fixture.txt"), b"head\n").expect("write head fixture");
        git(fixture.root(), &["commit", "-am", "head"]);
        verify_pinned_base_commit(fixture.root(), &base)
            .expect("base commit is an ancestor of HEAD");

        let nonexistent = "0000000000000000000000000000000000000000";
        let error = verify_pinned_base_commit(fixture.root(), nonexistent)
            .expect_err("nonexistent object must fail");
        assert!(error.contains("does not resolve to a Git object"));

        fs::write(fixture.path("blob.txt"), b"blob\n").expect("write blob fixture");
        let blob = git(fixture.root(), &["hash-object", "-w", "blob.txt"]);
        let error = verify_pinned_base_commit(fixture.root(), &blob).expect_err("blob must fail");
        assert!(error.contains("not a commit"));
        fixture
            .cleanup()
            .expect("remove owned release manifest Git fixture");
    }

    #[test]
    fn pinned_base_rejects_malformed_nonancestor_and_source_archive() {
        let malformed = verify_pinned_base_commit(Path::new("."), "not-an-object-id")
            .expect_err("malformed object ID must fail before Git access");
        assert!(malformed.contains("malformed pinned base revision"));

        let fixture = git_root("nonancestor");
        git(fixture.root(), &["checkout", "-b", "side"]);
        fs::write(fixture.path("side.txt"), b"side\n").expect("write side fixture");
        git(fixture.root(), &["add", "."]);
        git(fixture.root(), &["commit", "-m", "side"]);
        let side = git(fixture.root(), &["rev-parse", "HEAD"]);
        git(fixture.root(), &["checkout", "main"]);
        fs::write(fixture.path("main.txt"), b"main\n").expect("write main fixture");
        git(fixture.root(), &["add", "."]);
        git(fixture.root(), &["commit", "-m", "main"]);
        let error =
            verify_pinned_base_commit(fixture.root(), &side).expect_err("nonancestor must fail");
        assert!(error.contains("is not an ancestor"));
        fixture
            .cleanup()
            .expect("remove owned release manifest Git fixture");

        let archive = OwnedFixtureRoot::create("source-archive")
            .expect("atomically create owned source archive fixture");
        let error =
            verify_pinned_base_commit(archive.root(), "1111111111111111111111111111111111111111")
                .expect_err("source archive must fail");
        assert!(error.contains("source archives without .git cannot pass"));
        archive
            .cleanup()
            .expect("remove owned source archive fixture");
    }

    #[test]
    fn invalid_execution_policy_combination_fails() {
        let text = replace_first(
            &canonical_manifest(),
            "execution_policy = \"blocked\"",
            "execution_policy = \"normal_research\"",
            2,
            1,
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
            2,
            1,
        );
        let error = parse_release_manifest(&text)
            .expect_err("public execution in release requirements must fail");
        assert!(error.contains("public_executable=false"));
    }

    fn replace_first(
        text: &str,
        old: &str,
        new: &str,
        expected_pre_count: usize,
        expected_post_count: usize,
    ) -> String {
        replace_manifest_text_occurrence(
            text,
            old,
            new,
            expected_pre_count,
            0,
            expected_post_count,
        )
    }

    fn captured_panic_message<F>(action: F) -> String
    where
        F: FnOnce(),
    {
        let payload = catch_unwind(AssertUnwindSafe(action))
            .expect_err("fixture replacement control must panic");
        if let Some(message) = payload.downcast_ref::<String>() {
            message.clone()
        } else if let Some(message) = payload.downcast_ref::<&str>() {
            (*message).to_string()
        } else {
            "non-string panic payload".to_string()
        }
    }

    #[test]
    fn fixture_replacement_contract_is_newline_stable_and_count_complete() {
        for message in [
            captured_panic_message(|| {
                replace_text_occurrence("value\n", "value", "value", 1, 0, 1);
            }),
            captured_panic_message(|| {
                replace_manifest_text_occurrence(
                    "value\n", "value\n", "value\n", 1, 0, 1,
                );
            }),
            captured_panic_message(|| {
                replace_manifest_text_occurrence(
                    "value\r\n",
                    "value\n",
                    "value\n",
                    1,
                    0,
                    1,
                );
            }),
            captured_panic_message(|| {
                replace_manifest_text_occurrence("value", "value", "value", 1, 0, 1);
            }),
        ] {
            assert!(
                message.contains("fixture replacement must change bytes"),
                "stable no-op diagnostic changed: {message}"
            );
        }

        assert_eq!(
            replace_manifest_text_occurrence("value\n", "value\n", "other\n", 1, 0, 0),
            "other\n"
        );
        assert_eq!(
            replace_manifest_text_occurrence(
                "value\r\n",
                "value\n",
                "other\n",
                1,
                0,
                0,
            ),
            "other\r\n"
        );
        assert_eq!(
            replace_manifest_text_occurrence("value", "value", "other", 1, 0, 0),
            "other"
        );

        let overmatch = captured_panic_message(|| {
            replace_text_occurrence("value\n", "value", "valuevalue", 1, 0, 0);
        });
        assert!(
            overmatch.contains(
                "fixture replacement expected 0 post-mutation occurrences of `value`, found 2"
            ),
            "post-mutation overmatch reached the wrong guard: {overmatch}"
        );

        assert_eq!(
            replace_text_occurrence("value\n", "value", "value!", 1, 0, 1),
            "value!\n",
            "a byte-changing replacement may legitimately preserve the needle count"
        );

        for (text, expected) in [("other\n", 0usize), ("value value\n", 2usize)] {
            let message = captured_panic_message(|| {
                replace_text_occurrence(text, "value", "other", 1, 0, 0);
            });
            assert!(
                message.contains(&format!(
                    "fixture replacement expected 1 pre-mutation occurrences of `value`, found {expected}"
                )),
                "pre-mutation count diagnostic changed: {message}"
            );
        }
    }

    #[test]
    fn canonical_v3_negative_matrix_fails_closed() {
        let parser_cases = vec![
            ParserNegativeCase { name: "missing release version", old: "release_version = \"0.1.0-alpha.1\"\n", new: "", expected_pre_count: 1, expected_post_count: 0, expected: &["release top level is missing required metadata `release_version`"] },
            ParserNegativeCase { name: "malformed release version", old: "release_version = \"0.1.0-alpha.1\"", new: "release_version = \"0.1\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release_version `0.1` is not valid SemVer"] },
            ParserNegativeCase { name: "wrong release version", old: "release_version = \"0.1.0-alpha.1\"", new: "release_version = \"0.1.0-alpha.2\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release top-level key `release_version`", "value `0.1.0-alpha.2`"] },
            ParserNegativeCase { name: "missing channel", old: "release_channel = \"github_releases\"\n", new: "", expected_pre_count: 1, expected_post_count: 0, expected: &["release top level is missing required metadata `release_channel`"] },
            ParserNegativeCase { name: "invalid channel", old: "release_channel = \"github_releases\"", new: "release_channel = \"nightly\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release top-level key `release_channel`", "value `nightly`"] },
            ParserNegativeCase { name: "tier substituted for channel", old: "release_channel = \"github_releases\"", new: "release_channel = \"research_software_alpha\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release top-level key `release_channel`", "value `research_software_alpha`"] },
            ParserNegativeCase { name: "missing schema", old: "schema_version = \"aerocodex.release_manifest.v3\"\n", new: "", expected_pre_count: 1, expected_post_count: 0, expected: &["release top level is missing required metadata `schema_version`"] },
            ParserNegativeCase { name: "old schema", old: "aerocodex.release_manifest.v3", new: "aerocodex.release_manifest.v2", expected_pre_count: 1, expected_post_count: 0, expected: &["release top-level key `schema_version`", "aerocodex.release_manifest.v2"] },
            ParserNegativeCase { name: "unknown schema", old: "aerocodex.release_manifest.v3", new: "aerocodex.release_manifest.v99", expected_pre_count: 1, expected_post_count: 0, expected: &["release top-level key `schema_version`", "aerocodex.release_manifest.v99"] },
            ParserNegativeCase { name: "unknown top key", old: "release_documentation = \"docs/release/v0.1.0-alpha.1-status.md\"", new: "release_documentation = \"docs/release/v0.1.0-alpha.1-status.md\"\nunknown = \"value\"", expected_pre_count: 1, expected_post_count: 1, expected: &["release top level has unknown metadata key `unknown`"] },
            ParserNegativeCase { name: "duplicate top key", old: "release_channel = \"github_releases\"", new: "release_channel = \"github_releases\"\nrelease_channel = \"github_releases\"", expected_pre_count: 1, expected_post_count: 2, expected: &["duplicates key `release_channel` in the same table"] },
            ParserNegativeCase { name: "unknown table", old: "[release_slice_requirements]", new: "[release_requirements]", expected_pre_count: 1, expected_post_count: 0, expected: &["unsupported table `[release_requirements]`"] },
            ParserNegativeCase { name: "unknown nested key", old: "validation_status = \"research_required\"", new: "validation_state = \"research_required\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release_slice_requirements has unknown metadata key `validation_state`"] },
            ParserNegativeCase { name: "old derivative field", old: "registry_formula_count = 152", new: "registry_formula_count = 152\nregistry_blocked_formula_count = 152", expected_pre_count: 1, expected_post_count: 1, expected: &["release top level has unknown metadata key `registry_blocked_formula_count`"] },
            ParserNegativeCase { name: "missing release requirement", old: "validation_status = \"research_required\"\n", new: "", expected_pre_count: 2, expected_post_count: 1, expected: &["release_slice_requirements is missing required metadata `validation_status`"] },
            ParserNegativeCase { name: "status requirement mismatch", old: "validation_status = \"research_required\"", new: "validation_status = \"equation_traceable\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release_slice_requirements must be", "validation_status: \"equation_traceable\""] },
            ParserNegativeCase { name: "execution requirement mismatch", old: "execution_policy = \"blocked\"", new: "execution_policy = \"normal_research\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release_slice_requirements must be", "execution_policy: \"normal_research\""] },
            ParserNegativeCase { name: "public release requirement", old: "public_executable = false", new: "public_executable = true", expected_pre_count: 2, expected_post_count: 1, expected: &["release_slice_requirements must be", "public_executable: true"] },
            ParserNegativeCase { name: "non-release requirement mismatch", old: "[non_release_requirements]\nvalidation_status = \"research_required\"", new: "[non_release_requirements]\nvalidation_status = \"implementation_verified\"", expected_pre_count: 1, expected_post_count: 0, expected: &["non_release_requirements must be", "validation_status: \"implementation_verified\""] },
            ParserNegativeCase { name: "duplicate formula ID", old: "identifier = \"m00.angle.rad_to_deg\"", new: "identifier = \"m00.angle.deg_to_rad\"", expected_pre_count: 1, expected_post_count: 0, expected: &["duplicate release formula identifier `m00.angle.deg_to_rad`"] },
            ParserNegativeCase { name: "duplicate runtime symbol", old: "runtime_symbol = \"m00_radians_to_degrees\"", new: "runtime_symbol = \"m00_degrees_to_radians\"", expected_pre_count: 1, expected_post_count: 0, expected: &["duplicate or empty release formula runtime symbol `m00_degrees_to_radians`"] },
            ParserNegativeCase { name: "unknown formula", old: "identifier = \"m00.angle.deg_to_rad\"", new: "identifier = \"m00.unknown\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release formula set/order mismatch", "m00.unknown"] },
            ParserNegativeCase { name: "formula evidence absolute", old: "validation_record = \"validation/cards/validation_formula_vault_m00_angle_unit_conversions.yaml\"", new: "validation_record = \"C:/outside.yaml\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release formula `m00.angle.deg_to_rad` validation_record is invalid", "C:/outside.yaml"] },
            ParserNegativeCase { name: "formula evidence traversal", old: "documentation = \"docs/research_alpha/cli_formula_quickstart.md\"", new: "documentation = \"../outside.md\"", expected_pre_count: 12, expected_post_count: 11, expected: &["release formula `m00.angle.deg_to_rad` documentation is invalid", "../outside.md"] },
            ParserNegativeCase { name: "duplicate target", old: "triple = \"x86_64-apple-darwin\"", new: "triple = \"aarch64-apple-darwin\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release target set/tier/CI mapping mismatch"] },
            ParserNegativeCase { name: "unknown target tier", old: "platform_tier = \"tier2\"", new: "platform_tier = \"tier3\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release target set/tier/CI mapping mismatch", "tier3"] },
            ParserNegativeCase { name: "wrong target tier mapping", old: "platform_tier = \"tier2\"", new: "platform_tier = \"tier1\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release target set/tier/CI mapping mismatch"] },
            ParserNegativeCase { name: "wrong CI posture", old: "ci_posture = \"required_tier2\"", new: "ci_posture = \"blocking\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release target set/tier/CI mapping mismatch"] },
            ParserNegativeCase { name: "target currently tested", old: "currently_tested = false", new: "currently_tested = true", expected_pre_count: 4, expected_post_count: 3, expected: &["target `aarch64-apple-darwin` must be", "currently_tested: true"] },
            ParserNegativeCase { name: "target currently packaged", old: "currently_packaged = false", new: "currently_packaged = true", expected_pre_count: 4, expected_post_count: 3, expected: &["target `aarch64-apple-darwin` must be", "currently_packaged: true"] },
            ParserNegativeCase { name: "unsupported current support", old: "current_support = \"source_only\"", new: "current_support = \"binary\"", expected_pre_count: 4, expected_post_count: 3, expected: &["target `aarch64-apple-darwin` must be", "current_support: \"binary\""] },
            ParserNegativeCase { name: "unknown target nested key", old: "current_support = \"source_only\"", new: "support = \"source_only\"", expected_pre_count: 4, expected_post_count: 3, expected: &["targets table 1 has unknown metadata key `support`"] },
            ParserNegativeCase { name: "artifact unknown target", old: "target = \"x86_64-unknown-linux-gnu\"", new: "target = \"unknown-target\"", expected_pre_count: 1, expected_post_count: 0, expected: &["CLI artifact `cli-linux-x86-64` references unknown target `unknown-target`"] },
            ParserNegativeCase { name: "non-CLI artifact target", old: "kind = \"checksum\"\narchive_format", new: "kind = \"checksum\"\ntarget = \"x86_64-unknown-linux-gnu\"\narchive_format", expected_pre_count: 1, expected_post_count: 0, expected: &["non-CLI artifact `checksums` must not contain target `x86_64-unknown-linux-gnu`"] },
            ParserNegativeCase { name: "CLI artifact missing target", old: "target = \"x86_64-unknown-linux-gnu\"\n", new: "", expected_pre_count: 1, expected_post_count: 0, expected: &["CLI artifact `cli-linux-x86-64` is missing required target"] },
            ParserNegativeCase { name: "duplicate artifact ID", old: "identifier = \"cli-macos-aarch64\"", new: "identifier = \"cli-linux-x86-64\"", expected_pre_count: 1, expected_post_count: 0, expected: &["duplicate artifact ID `cli-linux-x86-64`"] },
            ParserNegativeCase { name: "duplicate artifact filename", old: "filename = \"aerocodex-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz\"", new: "filename = \"aerocodex-0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz\"", expected_pre_count: 1, expected_post_count: 0, expected: &["duplicate artifact filename `aerocodex-0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz`"] },
            ParserNegativeCase { name: "unsafe artifact filename", old: "aerocodex-0.1.0-alpha.1-SHA256SUMS", new: "unsafe name", expected_pre_count: 1, expected_post_count: 0, expected: &["artifact filename `unsafe name` must be a normalized safe ASCII basename"] },
            ParserNegativeCase { name: "absolute artifact filename", old: "aerocodex-0.1.0-alpha.1-SHA256SUMS", new: "/tmp/SHA256SUMS", expected_pre_count: 1, expected_post_count: 0, expected: &["artifact filename `/tmp/SHA256SUMS` must be a normalized safe ASCII basename"] },
            ParserNegativeCase { name: "artifact traversal", old: "aerocodex-0.1.0-alpha.1-SHA256SUMS", new: "../SHA256SUMS", expected_pre_count: 1, expected_post_count: 0, expected: &["artifact filename `../SHA256SUMS` must be a normalized safe ASCII basename"] },
            ParserNegativeCase { name: "artifact drive prefix", old: "aerocodex-0.1.0-alpha.1-SHA256SUMS", new: "C:/SHA256SUMS", expected_pre_count: 1, expected_post_count: 0, expected: &["artifact filename `C:/SHA256SUMS` must be a normalized safe ASCII basename"] },
            ParserNegativeCase { name: "unsupported archive format", old: "archive_format = \"none\"", new: "archive_format = \"7z\"", expected_pre_count: 4, expected_post_count: 3, expected: &["artifact `checksums` has unsupported archive_format `7z`"] },
            ParserNegativeCase { name: "kind format mismatch", old: "archive_format = \"none\"", new: "archive_format = \"zip\"", expected_pre_count: 4, expected_post_count: 3, expected: &["artifact `checksums` kind/format/extension mismatch"] },
            ParserNegativeCase { name: "filename version mismatch", old: "aerocodex-0.1.0-alpha.1-SHA256SUMS", new: "aerocodex-0.1.0-alpha.2-SHA256SUMS", expected_pre_count: 1, expected_post_count: 0, expected: &["release artifact set/contract mismatch", "aerocodex-0.1.0-alpha.2-SHA256SUMS"] },
            ParserNegativeCase { name: "filename target mismatch", old: "aerocodex-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz", new: "aerocodex-0.1.0-alpha.1-aarch64-apple-darwin-v2.tar.gz", expected_pre_count: 1, expected_post_count: 0, expected: &["release artifact set/contract mismatch", "cli-macos-aarch64", "aerocodex-0.1.0-alpha.1-aarch64-apple-darwin-v2.tar.gz", "aerocodex-0.1.0-alpha.1-aarch64-apple-darwin.tar.gz"] },
            ParserNegativeCase { name: "artifact produced", old: "production_state = \"declared_only\"", new: "production_state = \"built\"", expected_pre_count: 9, expected_post_count: 8, expected: &["artifact `checksums` production_state must be `declared_only`, found `built`"] },
            ParserNegativeCase { name: "invalid required task", old: "required_by_task = \"R5-09\"", new: "required_by_task = \"R7-01\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release artifact set/contract mismatch", "R7-01"] },
            ParserNegativeCase { name: "unknown artifact field", old: "required_by_task = \"R5-09\"", new: "required_by_task = \"R5-09\"\nself_hash = \"no\"", expected_pre_count: 1, expected_post_count: 1, expected: &["artifacts table 1 has unknown metadata key `self_hash`"] },
            ParserNegativeCase { name: "top-level order drift", old: "release_version = \"0.1.0-alpha.1\"\nrelease_channel = \"github_releases\"", new: "release_channel = \"github_releases\"\nrelease_version = \"0.1.0-alpha.1\"", expected_pre_count: 1, expected_post_count: 0, expected: &["release top level keys are not in canonical order"] },
            ParserNegativeCase { name: "requirement order drift", old: "validation_status = \"research_required\"\nexecution_policy = \"blocked\"", new: "execution_policy = \"blocked\"\nvalidation_status = \"research_required\"", expected_pre_count: 2, expected_post_count: 1, expected: &["release_slice_requirements keys are not in canonical order"] },
            ParserNegativeCase { name: "package key order drift", old: "name = \"aero-codex-core\"\nmanifest_path = \"crates/aero-codex-core/Cargo.toml\"", new: "manifest_path = \"crates/aero-codex-core/Cargo.toml\"\nname = \"aero-codex-core\"", expected_pre_count: 1, expected_post_count: 0, expected: &["packages table 1 keys are not in canonical order"] },
            ParserNegativeCase { name: "formula key order drift", old: "identifier = \"m00.angle.deg_to_rad\"\nruntime_symbol = \"m00_degrees_to_radians\"", new: "runtime_symbol = \"m00_degrees_to_radians\"\nidentifier = \"m00.angle.deg_to_rad\"", expected_pre_count: 1, expected_post_count: 0, expected: &["formulas table 1 keys are not in canonical order"] },
            ParserNegativeCase { name: "target key order drift", old: "triple = \"aarch64-apple-darwin\"\nplatform_tier = \"tier2\"", new: "platform_tier = \"tier2\"\ntriple = \"aarch64-apple-darwin\"", expected_pre_count: 1, expected_post_count: 0, expected: &["targets table 1 keys are not in canonical order"] },
            ParserNegativeCase { name: "artifact key order drift", old: "identifier = \"checksums\"\nfilename = \"aerocodex-0.1.0-alpha.1-SHA256SUMS\"", new: "filename = \"aerocodex-0.1.0-alpha.1-SHA256SUMS\"\nidentifier = \"checksums\"", expected_pre_count: 1, expected_post_count: 0, expected: &["artifacts table 1 keys are not in canonical order"] },
        ];

        let semantic_cases = [
            SemanticNegativeCase {
                name: "missing target",
                mutation: SemanticMutation::RemoveTarget("aarch64-apple-darwin"),
                expected: &[
                    "release target set/tier/CI mapping mismatch",
                    "aarch64-apple-darwin",
                ],
            },
            SemanticNegativeCase {
                name: "extra target",
                mutation: SemanticMutation::AddTarget,
                expected: &[
                    "release target set/tier/CI mapping mismatch",
                    "fixture-target",
                ],
            },
            SemanticNegativeCase {
                name: "artifact backslash",
                mutation: SemanticMutation::SetArtifactFilename {
                    identifier: "checksums",
                    filename: "dir\\SHA256SUMS",
                },
                expected: &[
                    "artifact filename `dir\\SHA256SUMS` must be a normalized safe ASCII basename",
                ],
            },
            SemanticNegativeCase {
                name: "missing artifact",
                mutation: SemanticMutation::RemoveArtifact("source-archive"),
                expected: &["release artifact set/contract mismatch", "source-archive"],
            },
            SemanticNegativeCase {
                name: "extra artifact",
                mutation: SemanticMutation::AddArtifact,
                expected: &["release artifact set/contract mismatch", "extra.zip"],
            },
        ];

        assert_eq!(parser_cases.len(), 54, "parser negative case count drifted");
        assert_eq!(
            semantic_cases.len(),
            5,
            "semantic negative case count drifted"
        );
        assert_eq!(
            parser_cases.len() + semantic_cases.len(),
            59,
            "complete negative matrix count drifted"
        );

        for case in parser_cases {
            for (line_ending, newline) in [("LF", "\n"), ("CRLF", "\r\n")] {
                let text = replace_first(
                    &canonical_manifest_with_newline(newline),
                    case.old,
                    case.new,
                    case.expected_pre_count,
                    case.expected_post_count,
                );
                let error = match parse_release_manifest(&text) {
                    Err(error) => error,
                    Ok(_) => panic!(
                        "negative production-parser case unexpectedly passed: {} ({line_ending})",
                        case.name
                    ),
                };
                assert_discriminating_error(
                    &format!("{} ({line_ending})", case.name),
                    &error,
                    case.expected,
                );
            }
        }

        for case in semantic_cases {
            let mut manifest = parse_release_manifest(&canonical_manifest())
                .expect("fresh canonical manifest must parse before semantic mutation");
            match case.mutation {
                SemanticMutation::RemoveTarget(triple) => {
                    let before = manifest.targets.len();
                    manifest.targets.retain(|target| target.triple != triple);
                    assert_eq!(
                        manifest.targets.len() + 1,
                        before,
                        "case `{}` must remove exactly one complete target record",
                        case.name
                    );
                }
                SemanticMutation::AddTarget => manifest.targets.push(ReleaseTarget {
                    triple: "fixture-target".to_string(),
                    platform_tier: "tier1".to_string(),
                    ci_posture: "blocking".to_string(),
                    artifact_eligible: true,
                    currently_tested: false,
                    currently_packaged: false,
                    current_support: "source_only".to_string(),
                }),
                SemanticMutation::SetArtifactFilename {
                    identifier,
                    filename,
                } => {
                    manifest
                        .artifacts
                        .iter_mut()
                        .find(|artifact| artifact.identifier == identifier)
                        .unwrap_or_else(|| {
                            panic!("case `{}` missing artifact `{identifier}`", case.name)
                        })
                        .filename = filename.to_string();
                }
                SemanticMutation::RemoveArtifact(identifier) => {
                    let before = manifest.artifacts.len();
                    manifest
                        .artifacts
                        .retain(|artifact| artifact.identifier != identifier);
                    assert_eq!(
                        manifest.artifacts.len() + 1,
                        before,
                        "case `{}` must remove exactly one complete artifact record",
                        case.name
                    );
                }
                SemanticMutation::AddArtifact => manifest.artifacts.push(ReleaseArtifact {
                    identifier: "extra".to_string(),
                    filename: "extra.zip".to_string(),
                    kind: "source_archive".to_string(),
                    target: None,
                    archive_format: "zip".to_string(),
                    production_state: "declared_only".to_string(),
                    required_by_task: "R4-09".to_string(),
                }),
            }
            let error = match validate_release_manifest(&manifest) {
                Err(error) => error,
                Ok(()) => panic!(
                    "negative production-semantic case unexpectedly passed: {}",
                    case.name
                ),
            };
            assert_discriminating_error(case.name, &error, case.expected);
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
        verify_authority_contract(&repository_root()).unwrap();
    }

    #[test]
    fn release_manifest_authority_contract_negative_matrix_is_discriminating() {
        #[derive(Clone, Copy)]
        enum AuthorityMutation {
            OldOnly,
            BothAuthorities,
            OmitCanonicalChecksum,
            FriendScriptDisagreement,
            ObsoleteCurrentConsumer,
        }

        struct AuthorityNegativeCase {
            name: &'static str,
            mutation: AuthorityMutation,
            expected: &'static [&'static str],
        }

        let control = AuthorityFixture::new("control");
        verify_authority_contract(control.root())
            .expect("canonical new-authority-only disposable repository must pass");
        control
            .cleanup()
            .expect("remove owned canonical authority fixture");

        let cases = [
            AuthorityNegativeCase {
                name: "old authority only",
                mutation: AuthorityMutation::OldOnly,
                expected: &[
                    "obsolete release authority `docs/release/v0.1.0-alpha.1.toml`",
                    "must be absent",
                ],
            },
            AuthorityNegativeCase {
                name: "both authorities present",
                mutation: AuthorityMutation::BothAuthorities,
                expected: &[
                    "obsolete release authority `docs/release/v0.1.0-alpha.1.toml`",
                    "must be absent",
                ],
            },
            AuthorityNegativeCase {
                name: "canonical authority omitted from checksums",
                mutation: AuthorityMutation::OmitCanonicalChecksum,
                expected: &[
                    "checksums/SHA256SUMS omits sole release authority",
                    "release/release-manifest.toml",
                ],
            },
            AuthorityNegativeCase {
                name: "Bash and PowerShell friend scripts disagree",
                mutation: AuthorityMutation::FriendScriptDisagreement,
                expected: &[
                    "scripts/friend_test_local.sh must invoke both governed release verifiers",
                ],
            },
            AuthorityNegativeCase {
                name: "current consumer retains obsolete authority path",
                mutation: AuthorityMutation::ObsoleteCurrentConsumer,
                expected: &[
                    "current-authority consumer `README.md` must name only",
                    "release/release-manifest.toml",
                ],
            },
        ];
        assert_eq!(cases.len(), 5, "authority negative case count drifted");

        for case in cases {
            let fixture = AuthorityFixture::new(&case.name.replace(' ', "-"));
            match case.mutation {
                AuthorityMutation::OldOnly => {
                    fixture.create_parent(OBSOLETE_RELEASE_MANIFEST_PATH);
                    fs::rename(
                        fixture.path(RELEASE_MANIFEST_PATH),
                        fixture.path(OBSOLETE_RELEASE_MANIFEST_PATH),
                    )
                    .expect("replace canonical authority with obsolete authority");
                    assert!(!fixture.path(RELEASE_MANIFEST_PATH).exists());
                    assert!(fixture.path(OBSOLETE_RELEASE_MANIFEST_PATH).is_file());
                }
                AuthorityMutation::BothAuthorities => {
                    fixture.create_parent(OBSOLETE_RELEASE_MANIFEST_PATH);
                    fs::copy(
                        fixture.path(RELEASE_MANIFEST_PATH),
                        fixture.path(OBSOLETE_RELEASE_MANIFEST_PATH),
                    )
                    .expect("add competing obsolete authority");
                    assert!(fixture.path(RELEASE_MANIFEST_PATH).is_file());
                    assert!(fixture.path(OBSOLETE_RELEASE_MANIFEST_PATH).is_file());
                }
                AuthorityMutation::OmitCanonicalChecksum => {
                    let mut checksums = fixture.read("checksums/SHA256SUMS");
                    let entry = checksums
                        .lines()
                        .find(|line| line.ends_with(RELEASE_MANIFEST_PATH))
                        .expect("canonical authority checksum entry exists")
                        .to_string();
                    assert_eq!(
                        checksums
                            .lines()
                            .filter(|line| *line == entry.as_str())
                            .count(),
                        1,
                        "canonical authority checksum entry must be unique"
                    );
                    let start = checksums
                        .find(&entry)
                        .expect("checksum entry offset exists");
                    let mut end = start + entry.len();
                    if checksums.as_bytes().get(end) == Some(&b'\r') {
                        end += 1;
                    }
                    if checksums.as_bytes().get(end) == Some(&b'\n') {
                        end += 1;
                    }
                    checksums.replace_range(start..end, "");
                    fixture.write("checksums/SHA256SUMS", &checksums);
                }
                AuthorityMutation::FriendScriptDisagreement => {
                    let script = fixture.read("scripts/friend_test_local.sh");
                    assert_eq!(script.matches("verify-release-identity").count(), 2);
                    fixture.write(
                        "scripts/friend_test_local.sh",
                        &script.replace("verify-release-identity", "verify-generated"),
                    );
                }
                AuthorityMutation::ObsoleteCurrentConsumer => {
                    let readme = fixture.read("README.md");
                    fixture.write(
                        "README.md",
                        &replace_first(
                            &readme,
                            RELEASE_MANIFEST_PATH,
                            OBSOLETE_RELEASE_MANIFEST_PATH,
                            2,
                            1,
                        ),
                    );
                }
            }

            let error = match verify_authority_contract(fixture.root()) {
                Err(error) => error,
                Ok(()) => panic!("negative authority case unexpectedly passed: {}", case.name),
            };
            assert_discriminating_error(case.name, &error, case.expected);
            fixture
                .cleanup()
                .expect("remove owned negative authority fixture");
        }
    }
}
