# Repository checksum policy

`checksums/SHA256SUMS` is the governing repository checksum manifest. Every repository file is governed except the checksum manifest itself, Git administrative data under `.git/`, Cargo build output under any `target/` directory, and the intentionally untracked root `Cargo.lock` that Cargo may create during local checks.

The manifest uses lowercase SHA-256 followed by two spaces and a normalized repository-relative path. Entries are sorted lexicographically by path. UTF-8 text is hashed with CRLF converted to LF so the same committed content verifies on Windows and Unix checkouts; binary bytes are hashed without normalization.

Run the blocking, cross-platform verifier from the repository root:

```bash
cargo run -p xtask -- verify-checksums
```

The verifier rejects malformed or duplicate entries, unsafe paths, missing files, changed content, and governed files omitted from the manifest. A checksum update is permitted only after the affected file has been reviewed and the reason for the update is recorded in the applicable change or release status document. The verifier does not rewrite the manifest.

The legacy `sha256sum -c checksums/SHA256SUMS` procedure is not governing because it is unavailable in standard PowerShell and hashes checkout-specific line endings. It may still be used as an additional raw-byte diagnostic on LF-based Unix checkouts.
