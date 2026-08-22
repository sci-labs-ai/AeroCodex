use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
    process::{Command, Output},
};

const GOVERNED_GENERATED_FILES: &[&str] = &[
    "generated/equation_batch_status_report.json",
    "generated/formula_registry.json",
    "generated/formula_registry.sha256",
    "generated/release_slice_validation_summary.json",
    "generated/rust/formula_registry.rs",
];
const GENERATOR_OWNED_DIRECTORIES: &[&str] = &["generated"];

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepositoryState {
    porcelain_status: Vec<u8>,
    tracked_diff: Vec<u8>,
    untracked_entries: BTreeMap<String, String>,
}

impl RepositoryState {
    fn is_clean(&self) -> bool {
        self.porcelain_status.is_empty()
            && self.tracked_diff.is_empty()
            && self.untracked_entries.is_empty()
    }
}

pub fn verify_generated_artifacts(root: &Path) -> Result<(), String> {
    for relative in GOVERNED_GENERATED_FILES {
        let path = root.join(relative);
        let is_regular_file = fs::symlink_metadata(&path)
            .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink());
        if !is_regular_file {
            return Err(format!(
                "required generated artifact is missing or not a regular file: {relative}"
            ));
        }
    }

    let baseline_clean = verify_action_does_not_drift(root, || {
        let report_options = crate::equation_batch::report::ReportOptions::parse_args(&[
            "--all-manifests",
            "--out",
            "generated/equation_batch_status_report.json",
            "--check",
        ])?;
        crate::equation_batch::report::run_report_command(root, &report_options)?;

        crate::release_slice_validation::verify_release_slice_validation(root)?;

        let registry_options = crate::formula_registry::check::CheckOptions::parse_args(&[])?;
        crate::formula_registry::check::run_check_command(root, &registry_options)
    })?;

    println!(
        "verified generated artifacts: files={}; stale=0; missing=0; repository_drift=0; baseline={}; policy=exact_git_state_delta",
        GOVERNED_GENERATED_FILES.len(),
        if baseline_clean { "clean" } else { "dirty" }
    );
    Ok(())
}

fn verify_action_does_not_drift<F>(root: &Path, action: F) -> Result<bool, String>
where
    F: FnOnce() -> Result<(), String>,
{
    let before = repository_state(root)?;
    let baseline_clean = before.is_clean();
    let action_result = action();
    let after = repository_state(root)?;

    let drift = describe_drift(&before, &after);
    match (action_result, drift.is_empty()) {
        (Ok(()), true) => Ok(baseline_clean),
        (Err(error), true) => Err(error),
        (Ok(()), false) => Err(format!(
            "generated-artifact verification left unexpected Git-aware repository drift:\n- {}",
            drift.join("\n- ")
        )),
        (Err(error), false) => Err(format!(
            "{error}\ngenerated-artifact verification also left unexpected Git-aware repository drift:\n- {}",
            drift.join("\n- ")
        )),
    }
}

fn repository_state(root: &Path) -> Result<RepositoryState, String> {
    let worktree = git_output(root, &["rev-parse", "--is-inside-work-tree"])?;
    if !worktree.status.success() || String::from_utf8_lossy(&worktree.stdout).trim() != "true" {
        return Err(format!(
            "generated-artifact drift verification requires Git metadata; source archives without .git cannot pass this gate ({})",
            git_error(&worktree)
        ));
    }

    let porcelain_status = successful_git_output(
        root,
        &[
            "-c",
            "core.quotepath=false",
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=no",
            "--ignored=no",
        ],
    )?;
    let tracked_diff = successful_git_output(
        root,
        &[
            "-c",
            "core.quotepath=false",
            "diff",
            "--binary",
            "--no-ext-diff",
            "--full-index",
            "HEAD",
            "--",
        ],
    )?;
    let untracked = successful_git_output(
        root,
        &["-c", "core.quotepath=false", "ls-files", "--others", "-z"],
    )?;
    let untracked_entries = snapshot_untracked(root, &untracked)?;

    Ok(RepositoryState {
        porcelain_status,
        tracked_diff,
        untracked_entries,
    })
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<Output, String> {
    let root = root.to_str().ok_or_else(|| {
        format!("generated-artifact Git verification requires a UTF-8 repository root: {root:?}")
    })?;
    let safe_directory = format!("safe.directory={}", root.replace('\\', "/"));
    Command::new("git")
        .arg("--no-optional-locks")
        .arg("-c")
        .arg(safe_directory)
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot execute Git for generated-artifact drift: {error}"))
}

fn successful_git_output(root: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let output = git_output(root, arguments)?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "Git command for generated-artifact drift failed: {}",
            git_error(&output)
        ))
    }
}

fn git_error(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

fn snapshot_untracked(root: &Path, output: &[u8]) -> Result<BTreeMap<String, String>, String> {
    let mut entries = BTreeMap::new();
    for raw_path in output
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
    {
        let relative = std::str::from_utf8(raw_path)
            .map_err(|_| "Git reported a non-UTF-8 untracked path".to_string())?;
        let parsed = Path::new(relative);
        if parsed.is_absolute()
            || parsed
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(format!("Git reported unsafe untracked path `{relative}`"));
        }
        if untracked_path_is_excluded(parsed) {
            continue;
        }
        let path = root.join(parsed);
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("cannot inspect untracked path {relative}: {error}"))?;
        let value = if metadata.file_type().is_symlink() {
            let target = fs::read_link(&path)
                .map_err(|error| format!("cannot read untracked symlink {relative}: {error}"))?;
            format!(
                "identity:{}",
                crate::equation_batch::generate::sha256_hex(&crate::fs_identity::symlink_identity(
                    target.as_os_str()
                ))
            )
        } else if metadata.is_file() {
            let bytes = fs::read(&path)
                .map_err(|error| format!("cannot read untracked file {relative}: {error}"))?;
            format!(
                "identity:{}:{}",
                file_mode(&metadata),
                crate::equation_batch::generate::sha256_hex(
                    &crate::fs_identity::regular_file_identity(&bytes)
                )
            )
        } else {
            return Err(format!(
                "untracked path {relative} is neither a regular file nor a symbolic link"
            ));
        };
        entries.insert(relative.replace('\\', "/"), value);
    }
    Ok(entries)
}

fn untracked_path_is_excluded(path: &Path) -> bool {
    if GENERATOR_OWNED_DIRECTORIES
        .iter()
        .any(|directory| path.starts_with(directory))
    {
        return false;
    }
    if path == Path::new("Cargo.lock") {
        return true;
    }
    if path.components().any(|component| {
        matches!(component, Component::Normal(name) if name == ".git" || name == "target")
    }) {
        return true;
    }
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    name == ".DS_Store" || name.ends_with(".tmp") || name.ends_with(".rs.bk")
}

#[cfg(unix)]
fn file_mode(metadata: &fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    metadata.permissions().mode() & 0o777
}

#[cfg(not(unix))]
fn file_mode(_metadata: &fs::Metadata) -> u32 {
    0
}

fn describe_drift(before: &RepositoryState, after: &RepositoryState) -> Vec<String> {
    let mut drift = Vec::new();
    if before.porcelain_status != after.porcelain_status {
        drift.push("Git staged/unstaged/untracked status changed".to_string());
    }
    if before.tracked_diff != after.tracked_diff {
        drift.push(
            "tracked content, executable mode, or symlink diff against HEAD changed".to_string(),
        );
    }
    let paths: BTreeSet<&String> = before
        .untracked_entries
        .keys()
        .chain(after.untracked_entries.keys())
        .collect();
    for path in paths {
        match (
            before.untracked_entries.get(path),
            after.untracked_entries.get(path),
        ) {
            (Some(before_value), Some(after_value)) if before_value != after_value => {
                drift.push(format!("changed untracked path: {path}"));
            }
            (Some(_), None) => drift.push(format!("removed untracked path: {path}")),
            (None, Some(_)) => drift.push(format!("created untracked path: {path}")),
            _ => {}
        }
    }
    drift
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn git(root: &Path, arguments: &[&str]) -> String {
        let output = git_output(root, arguments).expect("execute fixture Git command");
        assert!(
            output.status.success(),
            "fixture Git command failed: {}",
            git_error(&output)
        );
        String::from_utf8(output.stdout)
            .expect("Git fixture output is UTF-8")
            .trim()
            .to_string()
    }

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
        fs::write(root.join(".gitignore"), b"/target/\n").expect("write ignore policy");
        fs::write(root.join("generated/artifact.json"), b"{}\n")
            .expect("write generated drift fixture");
        git(&root, &["init", "-b", "main"]);
        git(&root, &["config", "user.email", "tests@example.invalid"]);
        git(&root, &["config", "user.name", "AeroCodex Tests"]);
        git(&root, &["add", "."]);
        git(&root, &["commit", "-m", "fixture"]);
        root
    }

    #[test]
    fn exact_dirty_baseline_without_delta_passes() {
        let root = test_root();
        fs::write(root.join("generated/artifact.json"), b"dirty baseline\n")
            .expect("write dirty baseline");
        let clean = verify_action_does_not_drift(&root, || Ok(()))
            .expect("an unchanged dirty baseline must pass");
        assert!(!clean);
        fs::remove_dir_all(root).expect("remove generated drift fixture");
    }

    #[test]
    fn clean_before_and_after_action_passes() {
        let root = test_root();
        let clean = verify_action_does_not_drift(&root, || Ok(()))
            .expect("a no-op action in a clean repository must pass");
        assert!(clean);
        fs::remove_dir_all(root).expect("remove generated drift fixture");
    }

    #[test]
    fn unstaged_staged_and_untracked_deltas_fail() {
        for kind in ["unstaged", "staged", "untracked"] {
            let root = test_root();
            let error = verify_action_does_not_drift(&root, || {
                match kind {
                    "unstaged" => fs::write(root.join("generated/artifact.json"), b"unstaged\n")
                        .map_err(|error| error.to_string())?,
                    "staged" => {
                        fs::write(root.join("generated/artifact.json"), b"staged\n")
                            .map_err(|error| error.to_string())?;
                        git(&root, &["add", "generated/artifact.json"]);
                    }
                    _ => fs::write(root.join("created.txt"), b"untracked\n")
                        .map_err(|error| error.to_string())?,
                }
                Ok(())
            })
            .expect_err("every Git-visible delta must fail");
            assert!(error.contains("Git-aware repository drift"));
            fs::remove_dir_all(root).expect("remove generated drift fixture");
        }
    }

    #[test]
    fn ignored_target_output_does_not_count_as_drift() {
        let root = test_root();
        verify_action_does_not_drift(&root, || {
            fs::create_dir_all(root.join("target/debug")).map_err(|error| error.to_string())?;
            fs::write(root.join("target/debug/output"), b"ignored")
                .map_err(|error| error.to_string())?;
            fs::write(root.join("scratch.tmp"), b"temporary").map_err(|error| error.to_string())?;
            fs::write(root.join("scratch.rs.bk"), b"backup").map_err(|error| error.to_string())?;
            fs::write(root.join(".DS_Store"), b"metadata").map_err(|error| error.to_string())
        })
        .expect("narrow build and transient exclusions must not count as repository drift");
        fs::remove_dir_all(root).expect("remove generated drift fixture");
    }

    #[test]
    fn generated_paths_bypass_repository_info_and_global_ignore_rules() {
        for source in ["repository", "info", "global"] {
            let root = test_root();
            let pattern = "/generated/ignored.json\n";
            match source {
                "repository" => {
                    fs::write(root.join(".gitignore"), format!("/target/\n{pattern}"))
                        .expect("write repository ignore fixture");
                    git(&root, &["add", ".gitignore"]);
                    git(&root, &["commit", "-m", "add repository ignore"]);
                }
                "info" => fs::write(root.join(".git/info/exclude"), pattern)
                    .expect("write info exclude fixture"),
                "global" => {
                    let excludes = root.join(".git/test-global-ignore");
                    fs::write(&excludes, pattern).expect("write global ignore fixture");
                    git(
                        &root,
                        &[
                            "config",
                            "core.excludesFile",
                            excludes.to_str().expect("UTF-8 fixture path"),
                        ],
                    );
                }
                _ => unreachable!(),
            }

            let created = verify_action_does_not_drift(&root, || {
                fs::write(root.join("generated/ignored.json"), b"created\n")
                    .map_err(|error| error.to_string())
            })
            .expect_err("ignored generated creation must fail");
            assert!(created.contains("created untracked path: generated/ignored.json"));
            assert_eq!(
                git(&root, &["check-ignore", "generated/ignored.json"]),
                "generated/ignored.json",
                "fixture must exercise the configured {source} ignore source"
            );

            let changed = verify_action_does_not_drift(&root, || {
                fs::write(root.join("generated/ignored.json"), b"changed\n")
                    .map_err(|error| error.to_string())
            })
            .expect_err("ignored generated modification must fail");
            assert!(changed.contains("changed untracked path: generated/ignored.json"));
            fs::remove_dir_all(root).expect("remove generated drift fixture");
        }
    }

    #[test]
    fn generator_owned_paths_do_not_receive_transient_filename_exclusions() {
        let root = test_root();
        let error = verify_action_does_not_drift(&root, || {
            fs::write(root.join("generated/unexpected.tmp"), b"unexpected\n")
                .map_err(|error| error.to_string())
        })
        .expect_err("new transient-looking file beneath generated must fail");
        assert!(error.contains("created untracked path: generated/unexpected.tmp"));
        fs::remove_dir_all(root).expect("remove generated drift fixture");
    }

    #[test]
    fn source_archive_without_git_metadata_fails() {
        let serial = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aerocodex-generated-archive-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("create source archive fixture");
        let error = repository_state(&root).expect_err("source archive must fail");
        assert!(error.contains("requires Git metadata"));
        fs::remove_dir_all(root).expect("remove source archive fixture");
    }

    #[cfg(unix)]
    #[test]
    fn executable_mode_and_symlink_target_deltas_fail() {
        use std::os::unix::fs::{symlink, PermissionsExt};

        let root = test_root();
        let created_error = verify_action_does_not_drift(&root, || {
            symlink("generated/artifact.json", root.join("created-link"))
                .map_err(|error| error.to_string())
        })
        .expect_err("created symlink drift must fail");
        assert!(created_error.contains("created untracked path"));
        fs::remove_file(root.join("created-link")).expect("remove created symlink fixture");

        fs::write(root.join("script.sh"), b"#!/bin/sh\n").expect("write script");
        symlink("generated/artifact.json", root.join("artifact-link"))
            .expect("create fixture symlink");
        git(&root, &["add", "script.sh", "artifact-link"]);
        git(&root, &["commit", "-m", "add modes"]);

        let mode_error = verify_action_does_not_drift(&root, || {
            let mut permissions = fs::metadata(root.join("script.sh"))
                .map_err(|error| error.to_string())?
                .permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(root.join("script.sh"), permissions)
                .map_err(|error| error.to_string())
        })
        .expect_err("executable mode drift must fail");
        assert!(mode_error.contains("executable mode"));

        git(&root, &["checkout", "--", "script.sh"]);
        let symlink_error = verify_action_does_not_drift(&root, || {
            fs::remove_file(root.join("artifact-link")).map_err(|error| error.to_string())?;
            symlink("script.sh", root.join("artifact-link")).map_err(|error| error.to_string())
        })
        .expect_err("symlink target drift must fail");
        assert!(symlink_error.contains("symlink"));

        git(&root, &["checkout", "--", "artifact-link"]);
        let removed_error = verify_action_does_not_drift(&root, || {
            fs::remove_file(root.join("artifact-link")).map_err(|error| error.to_string())
        })
        .expect_err("removed symlink drift must fail");
        assert!(removed_error.contains("symlink"));
        fs::remove_dir_all(root).expect("remove generated drift fixture");
    }

    #[cfg(unix)]
    #[test]
    fn untracked_symlink_state_is_lossless_and_detects_type_changes() {
        use std::{ffi::OsStr, os::unix::ffi::OsStrExt, os::unix::fs::symlink};

        let root = test_root();
        let link = root.join("generated/untracked-link");
        symlink(OsStr::from_bytes(b"target-\xff"), &link).expect("create non-UTF-8 link");

        let retargeted = verify_action_does_not_drift(&root, || {
            fs::remove_file(&link).map_err(|error| error.to_string())?;
            symlink(OsStr::from_bytes(b"target-\xfe"), &link).map_err(|error| error.to_string())
        })
        .expect_err("non-UTF-8 symlink retarget must fail");
        assert!(retargeted.contains("changed untracked path: generated/untracked-link"));

        let replaced = verify_action_does_not_drift(&root, || {
            fs::remove_file(&link).map_err(|error| error.to_string())?;
            fs::write(&link, b"regular file\n").map_err(|error| error.to_string())
        })
        .expect_err("untracked link-to-file replacement must fail");
        assert!(replaced.contains("changed untracked path: generated/untracked-link"));

        let removed = verify_action_does_not_drift(&root, || {
            fs::remove_file(&link).map_err(|error| error.to_string())
        })
        .expect_err("untracked file removal must fail");
        assert!(removed.contains("removed untracked path: generated/untracked-link"));
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
