#![forbid(unsafe_code)]

use std::{
    collections::BTreeSet,
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
        "usage:\n  cargo run -p xtask -- verify --all\n  cargo run -p xtask -- verify cards\n  cargo run -p xtask -- verify source-registry\n  cargo run -p xtask -- dependency-policy"
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
}
