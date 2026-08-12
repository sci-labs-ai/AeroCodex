use std::{
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
        "verified checksums: files={}; canonical_line_endings={}; manifest={CHECKSUM_MANIFEST_PATH}",
        result.verified_files, result.canonical_line_ending_files
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
    canonical_line_ending_files: usize,
}

fn verify_entries(
    root: &Path,
    entries: &[ChecksumEntry],
    require_complete_coverage: bool,
) -> Result<VerificationResult, String> {
    let mut failures = Vec::new();
    let mut verified_files = 0usize;
    let mut canonical_line_ending_files = 0usize;
    let listed: BTreeSet<&str> = entries.iter().map(|entry| entry.path.as_str()).collect();

    for entry in entries {
        let path = root.join(Path::new(&entry.path));
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                failures.push(format!("missing checksummed file: {}", entry.path));
                continue;
            }
            Err(error) => {
                failures.push(format!(
                    "cannot read checksummed file {}: {error}",
                    entry.path
                ));
                continue;
            }
        };
        let (canonical, normalized) = canonical_checksum_bytes(&bytes);
        let actual = crate::equation_batch::generate::sha256_hex(canonical.as_ref());
        if actual != entry.digest {
            failures.push(format!(
                "changed checksummed file: {} (expected {}, actual {})",
                entry.path, entry.digest, actual
            ));
            continue;
        }
        verified_files += 1;
        canonical_line_ending_files += usize::from(normalized);
    }

    if require_complete_coverage {
        match collect_governed_files(root) {
            Ok(files) => {
                for path in files {
                    if !listed.contains(path.as_str()) {
                        failures.push(format!("governed file is absent from checksums: {path}"));
                    }
                }
            }
            Err(error) => failures.push(error),
        }
    }

    if failures.is_empty() {
        Ok(VerificationResult {
            verified_files,
            canonical_line_ending_files,
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
        || parsed
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        || path == CHECKSUM_MANIFEST_PATH
    {
        return Err(format!(
            "malformed checksum entry at line {line_number}: `{path}` is not an allowed governed path"
        ));
    }
    Ok(())
}

pub(crate) fn canonical_checksum_bytes(bytes: &[u8]) -> (std::borrow::Cow<'_, [u8]>, bool) {
    if std::str::from_utf8(bytes).is_err() || !bytes.windows(2).any(|pair| pair == b"\r\n") {
        return (std::borrow::Cow::Borrowed(bytes), false);
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
    (std::borrow::Cow::Owned(canonical), true)
}

pub(crate) fn canonical_text(text: &str) -> std::borrow::Cow<'_, str> {
    if text.contains("\r\n") {
        std::borrow::Cow::Owned(text.replace("\r\n", "\n"))
    } else {
        std::borrow::Cow::Borrowed(text)
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
        if excluded_path(relative) {
            continue;
        }
        let file_type = entry
            .file_type()
            .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_governed_files_recursive(root, &path, files)?;
        } else if file_type.is_file() {
            files.push(path_string(relative));
        }
    }
    Ok(())
}

fn excluded_path(path: &Path) -> bool {
    path == Path::new(CHECKSUM_MANIFEST_PATH)
        || path == Path::new("Cargo.lock")
        || path.components().any(|component| {
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
        let (canonical, _) = canonical_checksum_bytes(bytes);
        ChecksumEntry {
            digest: crate::equation_batch::generate::sha256_hex(canonical.as_ref()),
            path: path.to_string(),
        }
    }

    #[test]
    fn checksum_parser_accepts_valid_entries_and_crlf_manifest() {
        let text =
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  docs/a.txt\r\n";
        let entries = parse_checksum_manifest(text).expect("valid checksum entry parses");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "docs/a.txt");
    }

    #[test]
    fn missing_checksummed_file_fails() {
        let root = test_root("missing");
        let error = verify_entries(&root, &[entry("missing.txt", b"missing")], false)
            .expect_err("missing file must fail");
        assert!(error.contains("missing checksummed file: missing.txt"));
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[test]
    fn changed_checksummed_file_fails() {
        let root = test_root("changed");
        fs::write(root.join("changed.txt"), b"actual\n").expect("write fixture");
        let error = verify_entries(&root, &[entry("changed.txt", b"expected\n")], false)
            .expect_err("changed file must fail");
        assert!(error.contains("changed checksummed file: changed.txt"));
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[test]
    fn canonical_line_endings_are_cross_platform() {
        let root = test_root("line-endings");
        fs::write(root.join("text.txt"), b"one\r\ntwo\r\n").expect("write fixture");
        let result = verify_entries(&root, &[entry("text.txt", b"one\ntwo\n")], false)
            .expect("CRLF checkout verifies against canonical LF digest");
        assert_eq!(result.canonical_line_ending_files, 1);
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[test]
    fn malformed_checksum_entries_fail() {
        for malformed in [
            "not-a-hash  docs/a.txt\n",
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD  docs/a.txt\n",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad docs/a.txt\n",
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  ../a.txt\n",
        ] {
            assert!(
                parse_checksum_manifest(malformed).is_err(),
                "malformed entry must fail: {malformed:?}"
            );
        }
    }

    #[test]
    fn unlisted_governed_file_fails_complete_coverage() {
        let root = test_root("coverage");
        fs::write(root.join("unlisted.txt"), b"content\n").expect("write fixture");
        fs::write(root.join("listed.txt"), b"listed\n").expect("write fixture");
        let error = verify_entries(&root, &[entry("listed.txt", b"listed\n")], true)
            .expect_err("unlisted governed file must fail");
        assert!(error.contains("governed file is absent from checksums: unlisted.txt"));
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }
}
