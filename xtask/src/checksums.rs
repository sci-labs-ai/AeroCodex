use std::{
    borrow::Cow,
    collections::BTreeSet,
    ffi::OsStr,
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

pub fn generate_checksums(root: &Path) -> Result<(), String> {
    for entry in generate_checksum_entries(root)? {
        println!("{}  {}", entry.digest, entry.path);
    }
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
    // Discover and validate the native governed path set before hashing any manifest entry.
    // This makes an unrepresentable repository filename a fail-closed precondition rather
    // than a path that can be omitted from, or aliased into, the UTF-8 manifest.
    let governed = require_exact_coverage
        .then(|| collect_governed_files(root))
        .transpose()?
        .map(|files| files.into_iter().collect::<BTreeSet<_>>());
    let mut failures = Vec::new();
    let mut verified_files = 0usize;
    let mut normalized_text_files = 0usize;
    let listed: BTreeSet<String> = entries.iter().map(|entry| entry.path.clone()).collect();

    for entry in entries {
        let path = root.join(Path::new(&entry.path));
        let (identity, normalized) = match checksummed_identity(&path, Path::new(&entry.path)) {
            Ok(identity) => identity,
            Err(error) => {
                failures.push(format!(
                    "cannot read checksummed file {}: {error}",
                    entry.path
                ));
                continue;
            }
        };
        let actual = crate::equation_batch::generate::sha256_hex(&identity);
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

    if let Some(governed) = governed {
        for path in governed.difference(&listed) {
            failures.push(format!("governed file is absent from checksums: {path}"));
        }
        for path in listed.difference(&governed) {
            failures.push(format!(
                "checksum entry is not a governed repository file: {path}"
            ));
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
    {
        return Err(format!(
            "malformed checksum entry at line {line_number}: `{path}` is not a canonical repository-relative path"
        ));
    }
    let canonical = governed_path_string(parsed)
        .map_err(|error| format!("malformed checksum entry at line {line_number}: {error}"))?;
    if canonical != path {
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

fn checksummed_identity(path: &Path, repository_path: &Path) -> Result<(Vec<u8>, bool), String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| error.to_string())?;
    if metadata.file_type().is_symlink() {
        let target = fs::read_link(path).map_err(|error| error.to_string())?;
        return Ok((
            crate::fs_identity::symlink_identity(target.as_os_str()),
            false,
        ));
    }
    if !metadata.is_file() {
        return Err("path is neither a regular file nor a symbolic link".to_string());
    }
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    let (canonical, normalized) = if is_intentional_text(repository_path) {
        canonical_checksum_bytes(&bytes)
    } else {
        (Cow::Borrowed(bytes.as_slice()), false)
    };
    Ok((
        crate::fs_identity::regular_file_identity(canonical.as_ref()),
        normalized,
    ))
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

fn generate_checksum_entries(root: &Path) -> Result<Vec<ChecksumEntry>, String> {
    let paths = collect_governed_files(root)?;
    paths
        .into_iter()
        .map(|path| {
            let repository_path = Path::new(&path);
            let (identity, _) = checksummed_identity(&root.join(repository_path), repository_path)
                .map_err(|error| format!("cannot read governed file {path}: {error}"))?;
            Ok(ChecksumEntry {
                digest: crate::equation_batch::generate::sha256_hex(&identity),
                path,
            })
        })
        .collect()
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
            files.push(governed_path_string(relative)?);
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

fn governed_path_string(path: &Path) -> Result<String, String> {
    let mut components = Vec::new();
    for component in path.components() {
        let Component::Normal(value) = component else {
            return Err(format!(
                "governed repository path is not canonical: {}",
                native_path_diagnostic(path.as_os_str())
            ));
        };
        let value = value.to_str().ok_or_else(|| {
            format!(
                "governed repository path is not valid UTF-8: {}",
                native_path_diagnostic(path.as_os_str())
            )
        })?;
        components.push(value);
    }
    Ok(components.join("/"))
}

#[cfg(unix)]
fn native_path_diagnostic(path: &OsStr) -> String {
    use std::os::unix::ffi::OsStrExt;

    format_hex_units(
        "unix-bytes",
        path.as_bytes().iter().copied().map(u32::from),
        2,
    )
}

#[cfg(windows)]
fn native_path_diagnostic(path: &OsStr) -> String {
    use std::os::windows::ffi::OsStrExt;

    format_hex_units("windows-utf16", path.encode_wide().map(u32::from), 4)
}

#[cfg(not(any(unix, windows)))]
fn native_path_diagnostic(path: &OsStr) -> String {
    format_hex_units(
        "encoded-bytes",
        path.as_encoded_bytes().iter().copied().map(u32::from),
        2,
    )
}

fn format_hex_units(label: &str, units: impl IntoIterator<Item = u32>, width: usize) -> String {
    let encoded = units
        .into_iter()
        .map(|unit| format!("{unit:0width$x}"))
        .collect::<Vec<_>>()
        .join(" ");
    format!("{label}=[{encoded}]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::OsStr,
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
        let (canonical, _) = if is_intentional_text(Path::new(path)) {
            canonical_checksum_bytes(bytes)
        } else {
            (Cow::Borrowed(bytes), false)
        };
        ChecksumEntry {
            digest: crate::equation_batch::generate::sha256_hex(
                &crate::fs_identity::regular_file_identity(canonical.as_ref()),
            ),
            path: path.to_string(),
        }
    }

    fn symlink_entry(path: &str, target: &OsStr) -> ChecksumEntry {
        ChecksumEntry {
            digest: crate::equation_batch::generate::sha256_hex(
                &crate::fs_identity::symlink_identity(target),
            ),
            path: path.to_string(),
        }
    }

    #[cfg(unix)]
    fn create_file_symlink(target: &OsStr, link: &Path) -> bool {
        use std::os::unix::fs::symlink;

        symlink(target, link).expect("create checksum symlink fixture");
        true
    }

    #[cfg(windows)]
    fn create_file_symlink(target: &OsStr, link: &Path) -> bool {
        use std::{io::ErrorKind, os::windows::fs::symlink_file};

        match symlink_file(target, link) {
            Ok(()) => true,
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::PermissionDenied | ErrorKind::Unsupported
                ) || error.raw_os_error() == Some(1314) =>
            {
                eprintln!("skipping symlink filesystem assertion: {error}");
                false
            }
            Err(error) => panic!("create checksum symlink fixture: {error}"),
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
    fn governed_path_conversion_accepts_ascii_and_unicode_utf8() {
        assert_eq!(
            governed_path_string(Path::new("docs/normal.txt")),
            Ok("docs/normal.txt".to_string())
        );
        assert_eq!(
            governed_path_string(Path::new("docs/aerodynamik-α.txt")),
            Ok("docs/aerodynamik-α.txt".to_string())
        );
    }

    #[test]
    fn checksum_generation_accepts_ascii_and_unicode_utf8_filenames() {
        let root = test_root("utf8-filenames");
        fs::write(root.join("normal.txt"), b"normal\n").expect("write ASCII fixture");
        fs::write(root.join("aerodynamik-α.txt"), b"unicode\n").expect("write Unicode fixture");
        let entries = generate_checksum_entries(&root).expect("UTF-8 paths generate exactly");
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.path.as_str())
                .collect::<Vec<_>>(),
            vec!["aerodynamik-α.txt", "normal.txt"]
        );
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[cfg(unix)]
    #[test]
    fn invalid_utf8_governed_filename_fails_generation_and_verification_without_aliasing() {
        use std::os::unix::ffi::OsStrExt;

        let root = test_root("invalid-governed-path");
        let invalid_name = OsStr::from_bytes(b"artifact-\xff.txt");
        fs::write(root.join(invalid_name), b"raw-name\n").expect("write raw filename fixture");
        fs::write(root.join("artifact-�.txt"), b"lossy-alias\n")
            .expect("write replacement-character alias fixture");
        let expected = concat!(
            "governed repository path is not valid UTF-8: ",
            "unix-bytes=[61 72 74 69 66 61 63 74 2d ff 2e 74 78 74]"
        );

        let generation_error = generate_checksum_entries(&root)
            .expect_err("generation must not omit an invalid native filename");
        assert_eq!(generation_error, expected);
        assert_eq!(
            generate_checksum_entries(&root)
                .expect_err("diagnostic must be deterministic on repeated discovery"),
            expected
        );

        let verification_error =
            verify_entries(&root, &[entry("artifact-�.txt", b"lossy-alias\n")], true)
                .expect_err("a valid replacement-character entry cannot alias the raw filename");
        assert_eq!(verification_error, expected);
        assert!(
            !verification_error.contains("governed file is absent"),
            "discovery must fail before a lossy set comparison can occur"
        );
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[cfg(windows)]
    #[test]
    fn invalid_windows_utf16_path_has_deterministic_diagnostic() {
        use std::{ffi::OsString, os::windows::ffi::OsStringExt};

        let path = PathBuf::from(OsString::from_wide(&[0x0061, 0xd800, 0x0062]));
        let expected = concat!(
            "governed repository path is not valid UTF-8: ",
            "windows-utf16=[0061 d800 0062]"
        );
        assert_eq!(governed_path_string(&path).unwrap_err(), expected);
        assert_eq!(governed_path_string(&path).unwrap_err(), expected);
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

    #[cfg(any(unix, windows))]
    #[test]
    fn symlinks_hash_the_unresolved_exact_target_and_not_file_contents() {
        let root = test_root("symlink-targets");
        fs::write(root.join("ordinary.txt"), b"ordinary\n").expect("write ordinary fixture");
        let relative_target = OsStr::new("ordinary.txt");
        if !create_file_symlink(relative_target, &root.join("relative-link")) {
            fs::remove_dir_all(root).expect("remove checksum test directory");
            return;
        }
        verify_entries(
            &root,
            &[symlink_entry("relative-link", relative_target)],
            false,
        )
        .expect("relative link target is hashed without following it");
        assert!(verify_entries(&root, &[entry("relative-link", b"ordinary\n")], false).is_err());
        assert!(
            verify_entries(
                &root,
                &[symlink_entry("relative-link", OsStr::new("./ordinary.txt"))],
                false,
            )
            .is_err(),
            "lexically different link targets must have different identities"
        );

        let absolute_target = root.join("ordinary.txt");
        if create_file_symlink(absolute_target.as_os_str(), &root.join("absolute-link")) {
            verify_entries(
                &root,
                &[symlink_entry("absolute-link", absolute_target.as_os_str())],
                false,
            )
            .expect("absolute link target is hashed exactly");
        }
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }

    #[cfg(unix)]
    #[test]
    fn non_utf8_symlink_targets_are_hashed_byte_exact() {
        use std::os::unix::ffi::OsStrExt;

        let root = test_root("non-utf8-symlink");
        let target = OsStr::from_bytes(b"target-\xff");
        create_file_symlink(target, &root.join("link"));
        verify_entries(&root, &[symlink_entry("link", target)], false)
            .expect("non-UTF-8 link target is lossless");
        assert!(verify_entries(
            &root,
            &[symlink_entry("link", OsStr::from_bytes(b"target-\xfe"))],
            false,
        )
        .is_err());
        fs::remove_dir_all(root).expect("remove checksum test directory");
    }
}
