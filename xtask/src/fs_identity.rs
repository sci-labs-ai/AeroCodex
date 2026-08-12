use std::ffi::OsStr;

const IDENTITY_HEADER: &[u8] = b"aerocodex.fs-identity.v1\0";
const REGULAR_FILE_MARKER: u8 = 1;
const SYMBOLIC_LINK_MARKER: u8 = 2;

/// Frames regular-file contents so an object kind and payload boundary are part of its identity.
pub(crate) fn regular_file_identity(contents: &[u8]) -> Vec<u8> {
    framed_identity(REGULAR_FILE_MARKER, contents)
}

/// Frames a symbolic link's target without resolving it or rewriting path separators.
pub(crate) fn symlink_identity(target: &OsStr) -> Vec<u8> {
    let target = symlink_target_bytes(target);
    framed_identity(SYMBOLIC_LINK_MARKER, &target)
}

fn framed_identity(kind: u8, payload: &[u8]) -> Vec<u8> {
    let payload_len =
        u64::try_from(payload.len()).expect("filesystem identity payload fits in u64");
    let mut identity = Vec::with_capacity(IDENTITY_HEADER.len() + 1 + 8 + payload.len());
    identity.extend_from_slice(IDENTITY_HEADER);
    identity.push(kind);
    identity.extend_from_slice(&payload_len.to_be_bytes());
    identity.extend_from_slice(payload);
    identity
}

#[cfg(unix)]
fn symlink_target_bytes(target: &OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;

    target.as_bytes().to_vec()
}

#[cfg(windows)]
fn symlink_target_bytes(target: &OsStr) -> Vec<u8> {
    use std::os::windows::ffi::OsStrExt;

    // WTF-8 is an injective, deterministic byte representation of Windows' native UTF-16:
    // valid surrogate pairs use ordinary UTF-8 and unpaired surrogates remain distinguishable.
    let wide: Vec<u16> = target.encode_wide().collect();
    windows_wide_to_wtf8(&wide)
}

#[cfg(not(any(unix, windows)))]
fn symlink_target_bytes(target: &OsStr) -> Vec<u8> {
    target.as_encoded_bytes().to_vec()
}

#[cfg(windows)]
fn windows_wide_to_wtf8(wide: &[u16]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(wide.len() * 3);
    let mut index = 0usize;
    while index < wide.len() {
        let first = wide[index];
        if (0xd800..=0xdbff).contains(&first)
            && wide
                .get(index + 1)
                .is_some_and(|second| (0xdc00..=0xdfff).contains(second))
        {
            let second = wide[index + 1];
            let code_point =
                0x1_0000 + ((u32::from(first) - 0xd800) << 10) + (u32::from(second) - 0xdc00);
            push_utf8_code_point(code_point, &mut encoded);
            index += 2;
        } else {
            push_utf8_code_point(u32::from(first), &mut encoded);
            index += 1;
        }
    }
    encoded
}

#[cfg(windows)]
fn push_utf8_code_point(code_point: u32, output: &mut Vec<u8>) {
    if code_point <= 0x7f {
        output.push(code_point as u8);
    } else if code_point <= 0x7ff {
        output.push((0xc0 | (code_point >> 6)) as u8);
        output.push((0x80 | (code_point & 0x3f)) as u8);
    } else if code_point <= 0xffff {
        output.push((0xe0 | (code_point >> 12)) as u8);
        output.push((0x80 | ((code_point >> 6) & 0x3f)) as u8);
        output.push((0x80 | (code_point & 0x3f)) as u8);
    } else {
        output.push((0xf0 | (code_point >> 18)) as u8);
        output.push((0x80 | ((code_point >> 12) & 0x3f)) as u8);
        output.push((0x80 | ((code_point >> 6) & 0x3f)) as u8);
        output.push((0x80 | (code_point & 0x3f)) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_kind_and_payload_length_are_framed() {
        let regular = regular_file_identity(b"same");
        let link = symlink_identity(OsStr::new("same"));
        assert_ne!(regular, link);
        assert_ne!(regular_file_identity(b"ab"), regular_file_identity(b"a"));
        assert!(regular.ends_with(b"same"));
        assert_eq!(
            &regular[IDENTITY_HEADER.len() + 1..IDENTITY_HEADER.len() + 9],
            4u64.to_be_bytes()
        );
    }

    #[cfg(unix)]
    #[test]
    fn unix_symlink_identity_preserves_non_utf8_target_bytes() {
        use std::os::unix::ffi::OsStrExt;

        let left = symlink_identity(OsStr::from_bytes(b"target-\xff"));
        let right = symlink_identity(OsStr::from_bytes(b"target-\xfe"));
        assert_ne!(left, right);
        assert!(left.ends_with(b"target-\xff"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_wtf8_encoding_is_lossless_and_deterministic() {
        assert_eq!(windows_wide_to_wtf8(&[0x0061, 0x20ac]), b"a\xe2\x82\xac");
        assert_eq!(windows_wide_to_wtf8(&[0xd83d, 0xde80]), b"\xf0\x9f\x9a\x80");
        assert_eq!(windows_wide_to_wtf8(&[0xd800]), b"\xed\xa0\x80");
        assert_ne!(
            windows_wide_to_wtf8(&[0xd800]),
            windows_wide_to_wtf8(&[0xdc00])
        );
    }
}
