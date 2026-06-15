#![forbid(unsafe_code)]

use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};

const REQUIRED_CARD_FIELDS: &[&str] = &[
    "id",
    "name",
    "category",
    "status",
    "source",
    "assumptions",
    "domain",
    "inputs",
    "outputs",
    "tests",
    "failure_modes",
    "notes",
];

const REQUIRED_CARD_LIST_FIELDS: &[&str] =
    &["assumptions", "inputs", "outputs", "tests", "failure_modes"];

const REQUIRED_SOURCE_REGISTRY_FIELDS: &[&str] = &[
    "id",
    "title",
    "category",
    "status",
    "intended_use",
    "requested_details",
    "implementation_notes",
    "limits",
    "notes",
];

const REQUIRED_SOURCE_REGISTRY_LIST_FIELDS: &[&str] = &[
    "intended_use",
    "requested_details",
    "implementation_notes",
    "limits",
];

const ALLOWED_STATUSES: &[&str] = &[
    "research_required",
    "equation_traceable",
    "implementation_verified",
    "reference_validated",
    "experiment_validated",
];

const ALLOWED_CARD_CATEGORIES: &[&str] = &[
    "core",
    "constants",
    "atmosphere",
    "thermodynamics",
    "gas_dynamics",
    "aerodynamics",
    "propulsion",
    "heat_transfer",
    "structures",
    "flight_dynamics",
    "astrodynamics",
    "life_support",
    "validation",
];

const ALLOWED_SOURCE_CATEGORIES: &[&str] = &[
    "constants",
    "atmosphere",
    "thermodynamics",
    "thermodynamics_propulsion",
    "gas_dynamics",
    "aerodynamics",
    "propulsion",
    "heat_transfer",
    "structures",
    "flight_dynamics",
    "astrodynamics",
    "life_support",
    "validation",
];

const REQUIRED_SCHEMA_MARKERS: &[&str] = &[
    "\"$schema\"",
    "draft/2020-12",
    "\"additionalProperties\": false",
    "\"id\"",
    "\"name\"",
    "\"category\"",
    "\"status\"",
    "\"source\"",
    "\"assumptions\"",
    "\"domain\"",
    "\"inputs\"",
    "\"outputs\"",
    "\"tests\"",
    "\"failure_modes\"",
    "\"notes\"",
];

const FORBIDDEN_DEPENDENCY_TOKENS: &[&str] = &[
    "bindgen",
    "cc",
    "cmake",
    "pkg-config",
    "vcpkg",
    "coolprop",
    "refprop",
    "cantera",
    "blas",
    "lapack",
    "python",
    "matlab",
    "julia",
];

const FORBIDDEN_READINESS_MARKERS: &[&str] = &[
    "certified: true",
    "flight_ready: true",
    "mission_ready: true",
    "operationally_approved: true",
    "approved_for_operations: true",
    "status: certified",
    "status: flight_ready",
    "status: mission_ready",
];

const REQUIRED_DATA_REGISTRY_FIELDS: &[&str] = &[
    "id",
    "title",
    "local_path",
    "artifact_kind",
    "origin",
    "license",
    "hash_status",
    "allowed_use",
    "bundling_decision",
    "validation_status",
    "owner",
    "update_cadence",
    "notes",
];

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

    let result = match arg_refs.as_slice() {
        ["verify"] | ["verify", "--all"] => verify_all(),
        ["verify", "cards"] => {
            let root = repo_root();
            verify_schema(&root).and_then(|()| {
                let source_ids = collect_source_registry_ids(&root)?;
                verify_cards(&root, Some(&source_ids))
            })
        }
        ["verify", "source-registry"] => {
            let root = repo_root();
            verify_source_registry(&root).map(|_| ())
        }
        ["verify", "data-registry"] => {
            let root = repo_root();
            verify_data_registry(&root).map(|_| ())
        }
        ["dependency-policy"] => dependency_policy(),
        ["help"] | ["--help"] | ["-h"] => {
            print_usage();
            Ok(())
        }
        _ => {
            print_usage();
            Err("unknown or missing command".to_string())
        }
    };

    if let Err(err) = result {
        eprintln!("xtask error: {err}");
        std::process::exit(1);
    }
}

fn print_usage() {
    eprintln!(
        "usage:\n  cargo run -p xtask -- verify --all\n  cargo run -p xtask -- verify cards\n  cargo run -p xtask -- verify source-registry\n  cargo run -p xtask -- verify data-registry\n  cargo run -p xtask -- dependency-policy"
    );
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf()
}

fn verify_all() -> Result<(), String> {
    let root = repo_root();
    verify_schema(&root)?;
    let source_ids = verify_source_registry(&root)?;
    verify_cards(&root, Some(&source_ids))?;
    verify_data_registry(&root)?;
    Ok(())
}

fn verify_schema(root: &Path) -> Result<(), String> {
    let schema = root.join("validation/schema/codex_card.schema.json");
    if !schema.is_file() {
        return Err(format!("missing schema: {}", schema.display()));
    }

    let text = fs::read_to_string(&schema).map_err(|e| format!("{}: {e}", schema.display()))?;
    for marker in REQUIRED_SCHEMA_MARKERS {
        if !text.contains(marker) {
            return Err(format!(
                "{} missing schema marker `{marker}`",
                schema.display()
            ));
        }
    }
    for status in ALLOWED_STATUSES {
        if !text.contains(status) {
            return Err(format!("{} missing status `{status}`", schema.display()));
        }
    }
    for category in ALLOWED_CARD_CATEGORIES {
        if !text.contains(category) {
            return Err(format!(
                "{} missing card category `{category}`",
                schema.display()
            ));
        }
    }

    println!("verified Codex Card schema scaffold");
    Ok(())
}

fn verify_cards(root: &Path, source_ids: Option<&BTreeSet<String>>) -> Result<(), String> {
    let cards_dir = root.join("validation/cards");
    if !cards_dir.is_dir() {
        return Err(format!(
            "missing validation cards directory: {}",
            cards_dir.display()
        ));
    }

    let mut count = 0usize;
    visit_yaml(&cards_dir, &mut |path| {
        count += 1;
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        verify_no_forbidden_readiness_markers(path, &text)?;
        verify_required_top_level_fields(path, &text, REQUIRED_CARD_FIELDS)?;

        let id = require_top_level_value(path, &text, "id")?;
        if !is_valid_dotted_id(id) {
            return Err(format!("{} has invalid dotted id `{id}`", path.display()));
        }

        let name = require_top_level_value(path, &text, "name")?;
        if name.is_empty() {
            return Err(format!("{} has empty name", path.display()));
        }

        let category = require_top_level_value(path, &text, "category")?;
        require_allowed(path, "category", category, ALLOWED_CARD_CATEGORIES)?;

        let status = require_top_level_value(path, &text, "status")?;
        require_allowed(path, "status", status, ALLOWED_STATUSES)?;

        for field in REQUIRED_CARD_LIST_FIELDS {
            require_nonempty_list(path, &text, field)?;
        }

        let source_id = require_nested_value(path, &text, "source", "id")?;
        if !source_id.starts_with("source.") || !is_valid_dotted_id(source_id) {
            return Err(format!(
                "{} has invalid source id `{source_id}`",
                path.display()
            ));
        }
        if let Some(known_sources) = source_ids {
            if !known_sources.contains(source_id) {
                return Err(format!(
                    "{} references source id `{source_id}` without a matching source-registry seed",
                    path.display()
                ));
            }
        }

        let source_status = require_nested_value(path, &text, "source", "status")?;
        require_allowed(path, "source.status", source_status, ALLOWED_STATUSES)?;

        let notes = require_top_level_value(path, &text, "notes")?;
        if notes.is_empty() {
            return Err(format!("{} has empty notes", path.display()));
        }

        Ok(())
    })?;

    if count == 0 {
        return Err("no validation cards found".to_string());
    }

    println!("verified {count} validation cards");
    Ok(())
}

fn verify_source_registry(root: &Path) -> Result<BTreeSet<String>, String> {
    let registry_dir = root.join("validation/source_registry");
    if !registry_dir.is_dir() {
        return Err(format!(
            "missing source registry directory: {}",
            registry_dir.display()
        ));
    }

    let mut count = 0usize;
    let mut ids = BTreeSet::new();
    visit_yaml(&registry_dir, &mut |path| {
        count += 1;
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        verify_no_forbidden_readiness_markers(path, &text)?;
        verify_required_top_level_fields(path, &text, REQUIRED_SOURCE_REGISTRY_FIELDS)?;

        let id = require_top_level_value(path, &text, "id")?;
        if !id.starts_with("source.") || !is_valid_dotted_id(id) {
            return Err(format!(
                "{} has invalid source-registry id `{id}`",
                path.display()
            ));
        }
        if !ids.insert(id.to_string()) {
            return Err(format!("duplicate source-registry id `{id}`"));
        }

        let title = require_top_level_value(path, &text, "title")?;
        if title.is_empty() {
            return Err(format!("{} has empty title", path.display()));
        }

        let category = require_top_level_value(path, &text, "category")?;
        require_allowed(path, "category", category, ALLOWED_SOURCE_CATEGORIES)?;

        let status = require_top_level_value(path, &text, "status")?;
        require_allowed(path, "status", status, ALLOWED_STATUSES)?;

        for field in REQUIRED_SOURCE_REGISTRY_LIST_FIELDS {
            require_nonempty_list(path, &text, field)?;
        }

        Ok(())
    })?;

    if count == 0 {
        return Err("no source-registry YAML files found".to_string());
    }

    println!("verified {count} source-registry seeds");
    Ok(ids)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DataRegistryEntry {
    line: usize,
    fields: BTreeMap<String, String>,
}

impl DataRegistryEntry {
    fn get(&self, field: &str) -> Option<&str> {
        self.fields.get(field).map(String::as_str)
    }
}

fn verify_data_registry(root: &Path) -> Result<Vec<DataRegistryEntry>, String> {
    let registry = root.join("data-governance/DATA_REGISTRY.yaml");
    if !registry.is_file() {
        return Err(format!("missing data registry: {}", registry.display()));
    }

    let text = fs::read_to_string(&registry).map_err(|e| format!("{}: {e}", registry.display()))?;
    let entries = verify_data_registry_text(&registry, &text)?;
    for entry in &entries {
        let local_path = entry.get("local_path").unwrap_or_default();
        if is_external_path(local_path) {
            continue;
        }
        let candidate = root.join(local_path);
        if !candidate.exists() {
            return Err(format!(
                "{} entry `{}` local_path `{local_path}` does not exist in the repository",
                registry.display(),
                entry.get("id").unwrap_or("<missing>")
            ));
        }
    }

    println!(
        "verified {} data-governance registry entries",
        entries.len()
    );
    Ok(entries)
}

fn verify_data_registry_text(path: &Path, text: &str) -> Result<Vec<DataRegistryEntry>, String> {
    let entries = parse_data_registry_entries(path, text)?;
    let mut ids = BTreeSet::new();

    for entry in &entries {
        for field in REQUIRED_DATA_REGISTRY_FIELDS {
            match entry.get(field) {
                Some(value) if !value.is_empty() => {}
                _ => {
                    return Err(format!(
                        "{} entry starting line {} missing required field `{field}:`",
                        path.display(),
                        entry.line
                    ));
                }
            }
        }

        let id = entry.get("id").unwrap_or_default();
        if !is_valid_artifact_id(id) {
            return Err(format!(
                "{} entry starting line {} has invalid artifact id `{id}`",
                path.display(),
                entry.line
            ));
        }
        if !ids.insert(id.to_string()) {
            return Err(format!("duplicate data-registry artifact id `{id}`"));
        }

        let local_path = entry.get("local_path").unwrap_or_default();
        validate_data_registry_local_path(path, entry, local_path)?;

        let hash_status = entry.get("hash_status").unwrap_or_default();
        match entry.get("sha256") {
            Some(hash) if !hash.is_empty() => {
                if !is_valid_sha256(hash) {
                    return Err(format!(
                        "{} entry `{id}` has invalid sha256 `{hash}`",
                        path.display()
                    ));
                }
            }
            _ => {
                if !is_pending_hash_status(hash_status) {
                    return Err(format!(
                        "{} entry `{id}` missing `sha256:` without `hash_status: pending_with_reason`",
                        path.display()
                    ));
                }
            }
        }

        if is_external_archive(entry) && has_unsafe_external_archive_decision(entry) {
            return Err(format!(
                "{} entry `{id}` has unsafe external archive import decision",
                path.display()
            ));
        }
    }

    Ok(entries)
}

fn parse_data_registry_entries(path: &Path, text: &str) -> Result<Vec<DataRegistryEntry>, String> {
    let mut entries = Vec::new();
    let mut current: Option<DataRegistryEntry> = None;
    let mut saw_artifacts = false;

    for (index, raw_line) in text.lines().enumerate() {
        let line_no = index + 1;
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed == "artifacts:" {
            if saw_artifacts {
                return Err(format!(
                    "{} line {line_no}: duplicate `artifacts:` root",
                    path.display()
                ));
            }
            saw_artifacts = true;
            continue;
        }
        if !saw_artifacts {
            return Err(format!(
                "{} line {line_no}: expected top-level `artifacts:` before registry entries",
                path.display()
            ));
        }

        if let Some(rest) = trimmed.strip_prefix("- ") {
            finish_data_registry_entry(path, &mut entries, current.take())?;
            let mut entry = DataRegistryEntry {
                line: line_no,
                fields: BTreeMap::new(),
            };
            if !rest.trim().is_empty() {
                insert_data_registry_field(path, &mut entry, line_no, rest.trim())?;
            }
            current = Some(entry);
            continue;
        }

        if trimmed == "-" {
            finish_data_registry_entry(path, &mut entries, current.take())?;
            current = Some(DataRegistryEntry {
                line: line_no,
                fields: BTreeMap::new(),
            });
            continue;
        }

        if raw_line.starts_with(' ') || raw_line.starts_with('\t') {
            let entry = current.as_mut().ok_or_else(|| {
                format!(
                    "{} line {line_no}: registry field appears before an artifact entry",
                    path.display()
                )
            })?;
            insert_data_registry_field(path, entry, line_no, trimmed)?;
            continue;
        }

        return Err(format!(
            "{} line {line_no}: malformed data-registry line `{trimmed}`",
            path.display()
        ));
    }

    finish_data_registry_entry(path, &mut entries, current.take())?;
    if entries.is_empty() {
        return Err(format!("{} has no data-registry entries", path.display()));
    }

    Ok(entries)
}

fn finish_data_registry_entry(
    path: &Path,
    entries: &mut Vec<DataRegistryEntry>,
    entry: Option<DataRegistryEntry>,
) -> Result<(), String> {
    if let Some(entry) = entry {
        if entry.fields.is_empty() {
            return Err(format!(
                "{} entry starting line {} is empty or malformed",
                path.display(),
                entry.line
            ));
        }
        entries.push(entry);
    }
    Ok(())
}

fn insert_data_registry_field(
    path: &Path,
    entry: &mut DataRegistryEntry,
    line_no: usize,
    text: &str,
) -> Result<(), String> {
    let (field, raw_value) = text.split_once(':').ok_or_else(|| {
        format!(
            "{} line {line_no}: malformed registry field `{text}`",
            path.display()
        )
    })?;
    let field = field.trim();
    if field.is_empty()
        || !field
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err(format!(
            "{} line {line_no}: invalid registry field name `{field}`",
            path.display()
        ));
    }
    let value = clean_scalar(raw_value).to_string();
    if entry.fields.insert(field.to_string(), value).is_some() {
        return Err(format!(
            "{} line {line_no}: duplicate field `{field}:` in one registry entry",
            path.display()
        ));
    }
    Ok(())
}

fn validate_data_registry_local_path(
    path: &Path,
    entry: &DataRegistryEntry,
    local_path: &str,
) -> Result<(), String> {
    if is_external_path(local_path) {
        return Ok(());
    }
    let lowered = local_path.to_ascii_lowercase();
    if local_path.starts_with('/')
        || lowered.starts_with("c:\\")
        || lowered.contains("c:\\users")
        || lowered.starts_with("/mnt/")
        || local_path.split('/').any(|part| part == "..")
        || local_path.split('\\').any(|part| part == "..")
    {
        return Err(format!(
            "{} entry `{}` local_path `{local_path}` must be repo-relative or external://stage4/...",
            path.display(),
            entry.get("id").unwrap_or("<missing>")
        ));
    }
    Ok(())
}

fn is_external_path(value: &str) -> bool {
    value.starts_with("external://")
}

fn is_pending_hash_status(value: &str) -> bool {
    value
        .to_ascii_lowercase()
        .starts_with("pending_with_reason")
}

fn is_valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn is_valid_artifact_id(value: &str) -> bool {
    if value.is_empty()
        || value.starts_with(['.', '_', '-'])
        || value.ends_with(['.', '_', '-'])
        || value.contains("..")
    {
        return false;
    }
    value.chars().all(|ch| {
        ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '_' || ch == '-'
    })
}

fn is_external_archive(entry: &DataRegistryEntry) -> bool {
    let local_path = entry.get("local_path").unwrap_or_default();
    let artifact_kind = entry
        .get("artifact_kind")
        .unwrap_or_default()
        .to_ascii_lowercase();
    is_external_path(local_path)
        && (artifact_kind.contains("archive") || local_path.to_ascii_lowercase().ends_with(".zip"))
}

fn has_unsafe_external_archive_decision(entry: &DataRegistryEntry) -> bool {
    let allowed_use = entry
        .get("allowed_use")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let bundling_decision = entry
        .get("bundling_decision")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let blocked_markers = [
        "direct public api import",
        "public api import",
        "import archive into public crates",
        "import into public crates",
        "merge into public crates",
        "bundled in public crates",
        "bulk import into crates",
    ];
    [allowed_use.as_str(), bundling_decision.as_str()]
        .iter()
        .any(|value| blocked_markers.iter().any(|marker| value.contains(marker)))
}

fn dependency_policy() -> Result<(), String> {
    let root = repo_root();
    let mut tomls = Vec::new();
    visit_files(&root, &mut |path| {
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "Cargo.toml")
        {
            tomls.push(path.to_path_buf());
        }
        Ok(())
    })?;

    for toml in tomls {
        let text = fs::read_to_string(&toml).map_err(|e| format!("{}: {e}", toml.display()))?;
        let lowered = text.to_ascii_lowercase();
        for token in FORBIDDEN_DEPENDENCY_TOKENS {
            if lowered.contains(token) {
                return Err(format!(
                    "{} contains forbidden token `{token}`",
                    toml.display()
                ));
            }
        }
        for line in lowered.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name") || trimmed.starts_with('[') {
                continue;
            }
            if trimmed.contains("-sys") {
                return Err(format!(
                    "{} appears to reference a -sys crate",
                    toml.display()
                ));
            }
        }
    }
    println!("dependency policy check passed");
    Ok(())
}

fn verify_required_top_level_fields(
    path: &Path,
    text: &str,
    fields: &[&str],
) -> Result<(), String> {
    for field in fields {
        if !has_top_level_field(text, field) {
            return Err(format!(
                "{} missing required field `{field}:`",
                path.display()
            ));
        }
    }
    Ok(())
}

fn verify_no_forbidden_readiness_markers(path: &Path, text: &str) -> Result<(), String> {
    let lowered = text.to_ascii_lowercase();
    for marker in FORBIDDEN_READINESS_MARKERS {
        if lowered.contains(marker) {
            return Err(format!(
                "{} contains forbidden readiness marker `{marker}`",
                path.display()
            ));
        }
    }
    Ok(())
}

fn require_top_level_value<'a>(path: &Path, text: &'a str, field: &str) -> Result<&'a str, String> {
    top_level_value(text, field)
        .ok_or_else(|| format!("{} missing top-level value `{field}:`", path.display()))
}

fn require_nested_value<'a>(
    path: &Path,
    text: &'a str,
    section: &str,
    field: &str,
) -> Result<&'a str, String> {
    nested_value(text, section, field).ok_or_else(|| {
        format!(
            "{} missing nested value `{section}.{field}`",
            path.display()
        )
    })
}

fn require_nonempty_list(path: &Path, text: &str, field: &str) -> Result<(), String> {
    if list_has_item(text, field) {
        Ok(())
    } else {
        Err(format!(
            "{} field `{field}:` must contain at least one list item",
            path.display()
        ))
    }
}

fn require_allowed(path: &Path, field: &str, value: &str, allowed: &[&str]) -> Result<(), String> {
    if allowed.contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "{} field `{field}` has unsupported value `{value}`",
            path.display()
        ))
    }
}

fn is_top_level_line(line: &str) -> bool {
    !line.is_empty() && !line.starts_with(' ') && !line.starts_with('\t')
}

fn has_top_level_field(text: &str, field: &str) -> bool {
    let prefix = format!("{field}:");
    text.lines().any(|line| {
        let trimmed = line.trim_end();
        is_top_level_line(trimmed) && trimmed.starts_with(&prefix)
    })
}

fn top_level_value<'a>(text: &'a str, field: &str) -> Option<&'a str> {
    let prefix = format!("{field}:");
    text.lines().find_map(|line| {
        let trimmed = line.trim_end();
        if is_top_level_line(trimmed) && trimmed.starts_with(&prefix) {
            Some(clean_scalar(&trimmed[prefix.len()..]))
        } else {
            None
        }
    })
}

fn nested_value<'a>(text: &'a str, section: &str, field: &str) -> Option<&'a str> {
    let section_prefix = format!("{section}:");
    let field_prefix = format!("{field}:");
    let mut in_section = false;

    for line in text.lines() {
        let trimmed_end = line.trim_end();
        let top_level = is_top_level_line(trimmed_end);
        if top_level {
            in_section = trimmed_end.starts_with(&section_prefix);
            continue;
        }
        if in_section {
            let nested = trimmed_end.trim_start();
            if nested.starts_with(&field_prefix) {
                return Some(clean_scalar(&nested[field_prefix.len()..]));
            }
        }
    }

    None
}

fn list_has_item(text: &str, field: &str) -> bool {
    let prefix = format!("{field}:");
    let mut in_list = false;

    for line in text.lines() {
        let trimmed_end = line.trim_end();
        let top_level = is_top_level_line(trimmed_end);
        if top_level {
            in_list = trimmed_end.starts_with(&prefix);
            continue;
        }
        if in_list {
            let nested = trimmed_end.trim_start();
            if let Some(item) = nested.strip_prefix("- ") {
                if !item.trim().is_empty() {
                    return true;
                }
            }
        }
    }

    false
}

fn clean_scalar(raw: &str) -> &str {
    raw.trim().trim_matches('"').trim_matches('\'').trim()
}

fn is_valid_dotted_id(value: &str) -> bool {
    if value.is_empty() || value.starts_with('.') || value.ends_with('.') || value.contains("..") {
        return false;
    }

    value.split('.').all(|segment| {
        !segment.is_empty()
            && segment
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    })
}

fn collect_source_registry_ids(root: &Path) -> Result<BTreeSet<String>, String> {
    let registry_dir = root.join("validation/source_registry");
    let mut ids = BTreeSet::new();
    visit_yaml(&registry_dir, &mut |path| {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let id = require_top_level_value(path, &text, "id")?;
        ids.insert(id.to_string());
        Ok(())
    })?;
    Ok(ids)
}

fn visit_yaml<F>(dir: &Path, f: &mut F) -> Result<(), String>
where
    F: FnMut(&Path) -> Result<(), String>,
{
    visit_files(dir, &mut |path| {
        if path
            .extension()
            .and_then(|s| s.to_str())
            .is_some_and(|ext| ext == "yaml" || ext == "yml")
        {
            f(path)?;
        }
        Ok(())
    })
}

fn visit_files<F>(dir: &Path, f: &mut F) -> Result<(), String>
where
    F: FnMut(&Path) -> Result<(), String>,
{
    for entry in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if name == "target" || name == ".git" {
            continue;
        }
        if path.is_dir() {
            visit_files(&path, f)?;
        } else {
            f(&path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_level_value_ignores_nested_values() {
        let text = "status: research_required\nsource:\n  status: implementation_verified\n";
        assert_eq!(top_level_value(text, "status"), Some("research_required"));
        assert_eq!(
            nested_value(text, "source", "status"),
            Some("implementation_verified")
        );
    }

    #[test]
    fn list_detection_requires_nonempty_item() {
        let missing = "assumptions:\n  -   \noutputs:\n  - value\n";
        let present = "assumptions:\n  - finite scalar inputs\n";
        assert!(!list_has_item(missing, "assumptions"));
        assert!(list_has_item(present, "assumptions"));
    }

    #[test]
    fn dotted_id_validation_rejects_unsafe_shapes() {
        assert!(is_valid_dotted_id(
            "life_support.bioregenerative.closure_fraction"
        ));
        assert!(is_valid_dotted_id(
            "source.gasdynamics.naca_report_1135.research_required"
        ));
        assert!(!is_valid_dotted_id("LifeSupport.closure"));
        assert!(!is_valid_dotted_id("life_support..closure"));
        assert!(!is_valid_dotted_id("life-support.closure"));
    }

    fn minimal_data_registry_entry(id: &str) -> String {
        format!(
            "artifacts:\n  - id: {id}\n    title: Minimal registry fixture\n    local_path: data/example.txt\n    artifact_kind: repo_file\n    origin: unit_test\n    license: MIT OR Apache-2.0\n    sha256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n    hash_status: sha256_verified\n    allowed_use: documentation and source-governance fixture only\n    bundling_decision: bundled repo-relative governance fixture\n    validation_status: registered_fixture_only\n    owner: AeroCodex maintainers\n    update_cadence: per governance update\n    notes: Minimal valid dependency-free parser fixture.\n"
        )
    }

    fn assert_data_registry_error_contains(text: &str, expected: &str) {
        let err = verify_data_registry_text(Path::new("DATA_REGISTRY.yaml"), text)
            .expect_err("registry fixture should fail");
        assert!(
            err.contains(expected),
            "expected error containing `{expected}`, got `{err}`"
        );
    }

    #[test]
    fn valid_minimal_data_registry_entry_passes() {
        let text = minimal_data_registry_entry("artifact.valid_minimal");
        let entries = verify_data_registry_text(Path::new("DATA_REGISTRY.yaml"), &text)
            .expect("minimal registry entry should pass");
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn duplicate_data_registry_id_fails() {
        let text = format!(
            "{}{}",
            minimal_data_registry_entry("artifact.duplicate"),
            minimal_data_registry_entry("artifact.duplicate").replace("artifacts:\n", "")
        );
        assert_data_registry_error_contains(&text, "duplicate data-registry artifact id");
    }

    #[test]
    fn missing_data_registry_local_path_fails() {
        let text = minimal_data_registry_entry("artifact.missing_path")
            .replace("    local_path: data/example.txt\n", "");
        assert_data_registry_error_contains(&text, "missing required field `local_path:`");
    }

    #[test]
    fn missing_data_registry_hash_without_pending_reason_fails() {
        let text = minimal_data_registry_entry("artifact.missing_hash").replace(
            "    sha256: 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\n",
            "",
        );
        assert_data_registry_error_contains(
            &text,
            "missing `sha256:` without `hash_status: pending_with_reason`",
        );
    }

    #[test]
    fn missing_data_registry_license_or_status_fails() {
        let missing_license = minimal_data_registry_entry("artifact.missing_license")
            .replace("    license: MIT OR Apache-2.0\n", "");
        assert_data_registry_error_contains(&missing_license, "missing required field `license:`");

        let missing_status = minimal_data_registry_entry("artifact.missing_status")
            .replace("    validation_status: registered_fixture_only\n", "");
        assert_data_registry_error_contains(
            &missing_status,
            "missing required field `validation_status:`",
        );
    }

    #[test]
    fn external_archive_with_public_import_decision_fails() {
        let text = minimal_data_registry_entry("stage4.unsafe_external_archive")
            .replace(
                "    local_path: data/example.txt\n",
                "    local_path: external://stage4/unsafe.zip\n",
            )
            .replace(
                "    artifact_kind: repo_file\n",
                "    artifact_kind: external_archive\n",
            )
            .replace(
                "    allowed_use: documentation and source-governance fixture only\n",
                "    allowed_use: direct public API import into AeroCodex crates\n",
            )
            .replace(
                "    bundling_decision: bundled repo-relative governance fixture\n",
                "    bundling_decision: import archive into public crates\n",
            );
        assert_data_registry_error_contains(&text, "unsafe external archive import decision");
    }
}
