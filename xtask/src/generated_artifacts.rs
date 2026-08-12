use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
};

const GOVERNED_GENERATED_FILES: &[&str] = &[
    "generated/equation_batch_status_report.json",
    "generated/formula_registry.json",
    "generated/formula_registry.sha256",
    "generated/rust/formula_registry.rs",
];

pub fn verify_generated_artifacts(root: &Path) -> Result<(), String> {
    for relative in GOVERNED_GENERATED_FILES {
        if !root.join(relative).is_file() {
            return Err(format!(
                "required generated artifact is missing: {relative}"
            ));
        }
    }

    verify_action_does_not_drift(root, || {
        let report_options = crate::equation_batch::report::ReportOptions::parse_args(&[
            "--all-manifests",
            "--out",
            "generated/equation_batch_status_report.json",
            "--check",
        ])?;
        crate::equation_batch::report::run_report_command(root, &report_options)?;

        let registry_options = crate::formula_registry::check::CheckOptions::parse_args(&[])?;
        crate::formula_registry::check::run_check_command(root, &registry_options)
    })?;

    println!(
        "verified generated artifacts: files={}; stale=0; missing=0; repository_drift=0",
        GOVERNED_GENERATED_FILES.len()
    );
    Ok(())
}

fn verify_action_does_not_drift<F>(root: &Path, action: F) -> Result<(), String>
where
    F: FnOnce() -> Result<(), String>,
{
    let before = repository_snapshot(root)?;
    let action_result = action();
    let after = repository_snapshot(root)?;

    let drift = describe_drift(&before, &after);
    match (action_result, drift.is_empty()) {
        (Ok(()), true) => Ok(()),
        (Err(error), true) => Err(error),
        (Ok(()), false) => Err(format!(
            "generated-artifact verification left unexpected repository drift:\n- {}",
            drift.join("\n- ")
        )),
        (Err(error), false) => Err(format!(
            "{error}\ngenerated-artifact verification also left unexpected repository drift:\n- {}",
            drift.join("\n- ")
        )),
    }
}

fn repository_snapshot(root: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut snapshot = BTreeMap::new();
    snapshot_directory(root, root, &mut snapshot)?;
    Ok(snapshot)
}

fn snapshot_directory(
    root: &Path,
    directory: &Path,
    snapshot: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot snapshot {}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot snapshot directory entry: {error}"))?;
        let path = entry.path();
        let relative = path.strip_prefix(root).map_err(|error| {
            format!(
                "cannot make {} repository-relative: {error}",
                path.display()
            )
        })?;
        if excluded_path(relative) {
            continue;
        }
        let file_type = entry
            .file_type()
            .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?;
        if file_type.is_dir() {
            snapshot_directory(root, &path, snapshot)?;
        } else if file_type.is_file() {
            let bytes = fs::read(&path)
                .map_err(|error| format!("cannot snapshot {}: {error}", path.display()))?;
            snapshot.insert(
                path_string(relative),
                crate::equation_batch::generate::sha256_hex(&bytes),
            );
        }
    }
    Ok(())
}

fn describe_drift(
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
) -> Vec<String> {
    let paths: BTreeSet<&String> = before.keys().chain(after.keys()).collect();
    paths
        .into_iter()
        .filter_map(|path| match (before.get(path), after.get(path)) {
            (Some(before_digest), Some(after_digest)) if before_digest != after_digest => {
                Some(format!("changed file: {path}"))
            }
            (Some(_), None) => Some(format!("removed file: {path}")),
            (None, Some(_)) => Some(format!("created file: {path}")),
            _ => None,
        })
        .collect()
}

fn excluded_path(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(component, Component::Normal(name) if name == ".git" || name == "target")
    })
}

fn path_string(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_root() -> PathBuf {
        let serial = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aerocodex-generated-drift-{}-{serial}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale generated drift fixture");
        }
        fs::create_dir_all(root.join("generated")).expect("create generated drift fixture");
        fs::write(root.join("generated/artifact.json"), b"{}\n")
            .expect("write generated drift fixture");
        root
    }

    #[test]
    fn unexpected_generated_file_drift_fails() {
        let root = test_root();
        let error = verify_action_does_not_drift(&root, || {
            fs::write(root.join("generated/artifact.json"), b"{\"stale\":true}\n")
                .map_err(|write_error| write_error.to_string())?;
            Ok(())
        })
        .expect_err("unexpected generated drift must fail");
        assert!(error.contains("changed file: generated/artifact.json"));
        fs::remove_dir_all(root).expect("remove generated drift fixture");
    }

    #[test]
    fn missing_required_generated_artifact_fails() {
        let root = test_root();
        let error = verify_generated_artifacts(&root)
            .expect_err("missing governed generated files must fail");
        assert!(error.contains("required generated artifact is missing"));
        fs::remove_dir_all(root).expect("remove generated drift fixture");
    }
}
