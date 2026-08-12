use std::{
    borrow::Cow,
    collections::BTreeSet,
    fs,
    path::{Component, Path},
};

pub const CHECKSUM_MANIFEST_PATH: &str = "checksums/SHA256SUMS";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumEntry {
    pub digest: String,
    pub path: String,
}

pub fn verify_checksums(root: &Path) -> Result<(), String> {
    let manifest_path = root.join(CHECKSUM_MANIFEST_PATH);
    let text = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("cannot read {CHECKSUM_MANIFEST_PATH}: {error}"))?;
    let entries = parse_checksum_manifest(&text)?;
    let result = verify_entries(root, &entries, true)?;
    println!(
        "verified checksums: files={}; normalized_text_files={}; manifest={CHECKSUM_MANIFEST_PATH}",
        result.verified_files, result.normalized_text_files
    );
    Ok(())
}

pub fn parse_checksum_manifest(text: &str) -> Result<Vec<ChecksumEntry>, String> {
    let mut entries = Vec::new();
    let mut paths = BTreeSet::new();
    let mut previous_path: Option<String> = None;

    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if line.is_empty() {
            return Err(format!(
                "malformed checksum entry at line {line_number}: blank lines are not allowed"
            ));
        }
        if line.len() < 67 || line.as_bytes().get(64..66) != Some(b"  ") {
            return Err(format!(
                "malformed checksum entry at line {line_number}: expected `<64 lowercase hex>  <repository-relative path>`"
            ));
        }
        let digest = &line[..64];
        if !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(format!(
                "malformed checksum entry at line {line_number}: digest must be 64 lowercase hexadecimal characters"
            ));
        }
        let path = &line[66..];
        validate_manifest_path(path, line_number)?;
        if !paths.insert(path.to_string()) {
            return Err(format!(
                "duplicate checksum path `{path}` at line {line_number}"
            ));
        }
        if previous_path
            .as_deref()
            .is_some_and(|previous| previous >= path)
        {
            return Err(format!(
                "checksum entries are not in lexicographic path order at line {line_number}: `{path}` follows `{}`",
                previous_path.as_deref().unwrap_or_default()
            ));
        }
        previous_path = Some(path.to_string());
        entries.push(ChecksumEntry {
            digest: digest.to_string(),
            path: path.to_string(),
        });
    }

    if entries.is_empty() {
        return Err("checksum manifest contains no entries".to_string());
    }
    Ok(entries)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VerificationResult {
    verified_files: usize,
    normalized_text_files: usize,
}

fn verify_entries(
    root: &Path,
    entries: &[ChecksumEntry],
    require_exact_coverage: bool,
) -> Result<VerificationResult, String> {
    let mut failures = Vec::new();
    let mut verified_files = 0usize;
    let mut normalized_text_files = 0usize;
    let listed: BTreeSet<String> = entries.iter().map(|entry| entry.path.clone()).collect();

    for entry in entries {
        let path = root.join(Path::new(&entry.path));
        let bytes = match checksummed_bytes(&path) {
            Ok(bytes) => bytes,
            Err(error) => {
                failures.push(format!(
                    "cannot read checksummed file {}: {error}",
                    entry.path
                ));
                continue;
            }
        };
        let (canonical, normalized) = checksum_bytes_for_path(Path::new(&entry.path), &bytes);
        let actual = crate::equation_batch::generate::sha256_hex(canonical.as_ref());
        if actual != entry.digest {
            failures.push(format!(
                "changed checksummed file: {} (expected {}, actual {})",
                entry.path, entry.digest, actual
            ));
            continue;
        }
        verified_files += 1;
        normalized_text_files += usize::from(normalized);
    }

    if require_exact_coverage {
        match collect_governed_files(root) {
            Ok(files) => {
                let governed: BTreeSet<String> = files.into_iter().collect();
                for path in governed.difference(&listed) {
                    failures.push(format!("governed file is absent from checksums: {path}"));
                }
                for path in listed.difference(&governed) {
                    failures.push(format!(
                        "checksum entry is not a governed repository file: {path}"
                    ));
                }
            }
            Err(error) => failures.push(error),
        }
    }

    if failures.is_empty() {
        Ok(VerificationResult {
            verified_files,
            normalized_text_files,
        })
    } else {
        Err(format!(
            "checksum verification failed with {} problem(s):\n- {}",
            failures.len(),
            failures.join("\n- ")
        ))
    }
}

fn validate_manifest_path(path: &str, line_number: usize) -> Result<(), String> {
    if path.is_empty() || path.contains('\\') {
        return Err(format!(
            "malformed checksum entry at line {line_number}: path must use repository-relative forward slashes"
        ));
    }
    let parsed = Path::new(path);
    if parsed.is_absolute()
        || has_windows_drive_prefix(path)
        || parsed
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        || path_string(parsed) != path
    {
        return Err(format!(
            "malformed checksum entry at line {line_number}: `{path}` is not a canonical repository-relative path"
        ));
    }
    if let Some(reason) = exclusion_reason(parsed) {
        return Err(format!(
            "malformed checksum entry at line {line_number}: excluded path `{path}` cannot appear in the manifest ({reason})"
        ));
    }
    Ok(())
}

fn has_windows_drive_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/'
}

fn checksummed_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| error.to_string())?;
    if metadata.file_type().is_symlink() {
        let target = fs::read_link(path).map_err(|error| error.to_string())?;
        return Ok(target.to_string_lossy().replace('\\', "/").into_bytes());
    }
    if !metadata.is_file() {
        return Err("path is neither a regular file nor a symbolic link".to_string());
    }
    fs::read(path).map_err(|error| error.to_string())
}

fn checksum_bytes_for_path<'a>(path: &Path, bytes: &'a [u8]) -> (Cow<'a, [u8]>, bool) {
    if is_intentional_text(path) {
        canonical_checksum_bytes(bytes)
    } else {
        (Cow::Borrowed(bytes), false)
    }
}

fn is_intentional_text(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    if matches!(
        name,
        ".gitignore" | ".gitattributes" | "LICENSE" | "LICENSE-APACHE" | "LICENSE-MIT" | "NOTICE"
    ) {
        return true;
    }
    matches!(
        path.extension().and_then(|value| value.to_str()),
        Some(
            "bib"
                | "csv"
                | "json"
                | "md"
                | "ps1"
                | "rs"
                | "sh"
                | "sha256"
                | "tex"
                | "toml"
                | "tsv"
                | "txt"
                | "yaml"
                | "yml"
        )
    )
}

pub(crate) fn canonical_checksum_bytes(bytes: &[u8]) -> (Cow<'_, [u8]>, bool) {
    if std::str::from_utf8(bytes).is_err() || !bytes.windows(2).any(|pair| pair == b"\r\n") {
        return (Cow::Borrowed(bytes), false);
    }

    let mut canonical = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index..].starts_with(b"\r\n") {
            canonical.push(b'\n');
            index += 2;
        } else {
            canonical.push(bytes[index]);
            index += 1;
        }
    }
    (Cow::Owned(canonical), true)
}

pub(crate) fn canonical_text(text: &str) -> Cow<'_, str> {
    if text.contains("\r\n") {
        Cow::Owned(text.replace("\r\n", "\n"))
    } else {
        Cow::Borrowed(text)
    }
}

fn collect_governed_files(root: &Path) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    collect_governed_files_recursive(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_governed_files_recursive(
    root: &Path,
    directory: &Path,
    files: &mut Vec<String>,
) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot inspect directory entry: {error}"))?;
        let path = entry.path();
        let relative = path.strip_prefix(root).map_err(|error| {
            format!(
                "cannot make {} repository-relative: {error}",
                path.display()
            )
        })?;
        if exclusion_reason(relative).is_some() {
            continue;
        }
        let file_type = entry
            .file_type()
            .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_governed_files_recursive(root, &path, files)?;
        } else if file_type.is_file() || file_type.is_symlink() {
            files.push(path_string(relative));
        }
    }
    Ok(())
}

fn exclusion_reason(path: &Path) -> Option<&'static str> {
    if path == Path::new(CHECKSUM_MANIFEST_PATH) {
        return Some("the checksum manifest cannot checksum itself");
    }
    if path == Path::new("Cargo.lock") {
        return Some("the workspace intentionally does not track the root Cargo.lock");
    }
    if path.components().any(|component| {
        matches!(component, Component::Normal(name) if name == ".git" || name == "target")
    }) {
        return Some("Git metadata and Cargo target output are excluded");
    }
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    if name == ".DS_Store" || name.ends_with(".tmp") || name.ends_with(".rs.bk") {
        return Some("temporary/editor files are excluded");
    }
    None
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
    const HASH: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    fn test_root(name: &str) -> PathBuf {
        let serial = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aerocodex-checksums-{name}-{}-{serial}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale checksum test directory");
        }
        fs::create_dir_all(root.join("checksums")).expect("create checksum test directory");
        root
    }

    fn entry(path: &str, bytes: &[u8]) -> ChecksumEntry {
        let (canonical, _) = checksum_bytes_for_path(Path::new(path), bytes);
        ChecksumEntry {
            digest: crate::equation_batch::generate::sha256_hex(canonical.as_ref()),
            path: path.to_string(),
        }
    }

    #[test]
    fn checksum_parser_accepts_valid_entries_and_crlf_manifest() {
        let text = format!("{HASH}  docs/a.txt\r\n");
        let entries = parse_checksum_manifest(&text).expect("valid checksum entry parses");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "docs/a.txt");
    }

    #[test]
    fn malformed_duplicate_aliased_and_excluded_entries_fail() {
        for malformed in [
            "not-a-hash  docs/a.txt\n".to_string(),
            format!("{}  docs/a.txt\n", HASH.to_uppercase()),
            format!("{HASH} docs/a.txt\n"),
            format!("{HASH}  ../a.txt\n"),
            format!("{HASH}  /absolute.txt\n"),
            format!("{HASH}  C:/absolute.txt\n"),
            format!("{HASH}  docs\\a.txt\n"),
            format!("{HASH}  docs//a.txt\n"),
            format!("{HASH}  Cargo.lock\n"),
            format!("{HASH}  .git/config\n"),
            format!("{HASH}  target/output\n"),
            format!("{HASH}  checksums/SHA256SUMS\n"),
            format!("{HASH}  docs/a.txt\n{HASH}  docs/a.txt\n"),
        ] {
            assert!(
                parse_checksum_manifest(&malformed).is_err(),
                "malformed entry must fail: {malformed:?}"
            );
        }
    }

    #[test]
    fn missing_and_changed_checksummed_files_fail() {
        let root = test_root("missing-changed");
        fs::write(root.join("changed.txt"), b"actual\n").expect("write fixture");
        let error = verify_entries(
            &root,
            &[
                entry("changed.txt", b"expected\n"),
                entry("missing.txt", b"missing"),
            ],
            false,
        )
        .expect_err("missing and changed files must fail");
        assert!(error.contains("changed checksummed file: changed.txt"));
        assert!(error.contains("cannot read checksummed file missing.txt"));
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[test]
    fn text_crlf_is_normalized_but_binary_crlf_is_not() {
        let root = test_root("line-endings");
        fs::write(root.join("text.txt"), b"one\r\ntwo\r\n").expect("write text fixture");
        fs::write(root.join("binary.bin"), b"one\r\ntwo\r\n").expect("write binary fixture");
        let result = verify_entries(
            &root,
            &[
                entry("binary.bin", b"one\r\ntwo\r\n"),
                entry("text.txt", b"one\ntwo\n"),
            ],
            false,
        )
        .expect("only intentional text formats normalize CRLF");
        assert_eq!(result.normalized_text_files, 1);
        assert!(verify_entries(&root, &[entry("binary.bin", b"one\ntwo\n")], false).is_err());
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[test]
    fn invalid_utf8_in_text_extension_is_hashed_byte_exact() {
        let root = test_root("invalid-utf8");
        let bytes = [0xff, b'\r', b'\n'];
        fs::write(root.join("invalid.txt"), bytes).expect("write invalid UTF-8 fixture");
        verify_entries(&root, &[entry("invalid.txt", &bytes)], false)
            .expect("invalid UTF-8 must be hashed without normalization");
        assert!(verify_entries(&root, &[entry("invalid.txt", &[0xff, b'\n'])], false).is_err());
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[test]
    fn exact_coverage_rejects_missing_and_extra_entries() {
        let root = test_root("coverage");
        fs::write(root.join("unlisted.txt"), b"content\n").expect("write fixture");
        fs::write(root.join("listed.txt"), b"listed\n").expect("write fixture");
        let error = verify_entries(
            &root,
            &[
                entry("listed.txt", b"listed\n"),
                entry("nonexistent.txt", b"none\n"),
            ],
            true,
        )
        .expect_err("set differences must fail");
        assert!(error.contains("governed file is absent from checksums: unlisted.txt"));
        assert!(error.contains("checksum entry is not a governed repository file: nonexistent.txt"));
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[test]
    fn excluded_outputs_do_not_change_governed_set() {
        let root = test_root("excluded");
        fs::write(root.join("kept.txt"), b"kept\n").expect("write fixture");
        fs::write(root.join("Cargo.lock"), b"lock\n").expect("write excluded lockfile");
        fs::create_dir_all(root.join("target/debug")).expect("create target fixture");
        fs::write(root.join("target/debug/output"), b"generated").expect("write excluded output");
        verify_entries(&root, &[entry("kept.txt", b"kept\n")], true)
            .expect("documented excluded outputs must not be governed");
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }
}
