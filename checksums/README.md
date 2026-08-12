# Repository checksum policy

`checksums/SHA256SUMS` is the governing repository checksum manifest. Its paths must be the exact set of governed files: no missing files, extra entries, duplicates, alternate spellings, or excluded paths are accepted. Paths use canonical repository-relative forward slashes and lowercase SHA-256 digests followed by two spaces.

Governed content includes regular files, generated files, and symbolic links. Every digest covers a versioned binary identity frame containing an object-type marker, an unsigned 64-bit payload length, and the payload. The marker makes a regular file and a symbolic link with otherwise identical bytes unambiguously different; length framing prevents ambiguous concatenation.

Regular-file payloads are file contents after the narrow text normalization below. Symbolic-link payloads are the unresolved target's exact operating-system representation, never the target file's contents: Unix uses raw `OsStr` bytes, while Windows losslessly and deterministically encodes the native UTF-16 target as WTF-8, including unpaired surrogates. The verifier does not follow links, canonicalize targets, or rewrite slash direction, so relative and absolute targets and every lexical retargeting remain distinct. The only exclusions are:

- `checksums/SHA256SUMS` itself;
- Git administrative data in any `.git/` path;
- Cargo build output in any `target/` path;
- the root `Cargo.lock`, which this repository intentionally leaves untracked for its current library-oriented workspace policy; and
- `.DS_Store`, `*.tmp`, and `*.rs.bk` temporary/editor files.

The `Cargo.lock` exclusion is a repository policy, not a packaging decision. Any future decision to track the workspace lockfile must update `.gitignore`, this policy, the verifier, and the checksum manifest together.

Line-ending normalization is intentionally narrow. UTF-8 files with these extensions are hashed after CRLF-to-LF normalization: `.bib`, `.csv`, `.json`, `.md`, `.ps1`, `.rs`, `.sh`, `.sha256`, `.tex`, `.toml`, `.tsv`, `.txt`, `.yaml`, and `.yml`. The extensionless text files `.gitignore`, `.gitattributes`, `LICENSE`, `LICENSE-APACHE`, `LICENSE-MIT`, and `NOTICE` receive the same treatment. Invalid UTF-8 in a listed text format, and every file outside this allowlist, is hashed byte-for-byte.

Run the blocking, cross-platform verifier from the repository root:

```bash
cargo run -p xtask -- verify-checksums
```

The verifier rejects malformed hashes, unsafe or noncanonical paths, excluded entries, duplicate paths, missing or changed files, and either side of a checksum/governed-file set difference. A checksum update is permitted only after the affected file has been reviewed and the reason is recorded in the applicable change or release status document. The verifier does not rewrite the manifest.

The legacy `sha256sum -c checksums/SHA256SUMS` procedure is not governing because it neither applies the identity frame nor the text policy, is unavailable in standard PowerShell, and hashes checkout-specific line endings. It is not a substitute for the Rust verifier.
