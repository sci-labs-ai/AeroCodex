# Beta 1 release-candidate testing

Status: `research_required`

This procedure creates a testable AeroCodex Beta 1 concept binary archive from one clean Git commit. It is a software release-engineering gate, not an aerospace assurance or certification gate.

The current candidate surface remains exactly ten governed M00 canonical-unit formulas. The 1,000+ equation backlog is outside this release-candidate scope.

## Prerequisites

- Git
- Python 3.11 or later
- Rust and Cargo compatible with the workspace `rust-version`
- `rustfmt` and Clippy for the normal repository gate

A root `Cargo.lock` remains intentionally uncommitted while every Cargo dependency is workspace-local and path-only. The packaging script proves that dependency condition and builds with Cargo `--offline`. If any registry, Git, or version-resolved external dependency is introduced, release packaging must fail until the lockfile policy is deliberately revised.

## Build a candidate archive

Run from a clean committed checkout:

```bash
python scripts/package_beta1_release.py \
  --repo . \
  --output-dir dist/beta1
```

The script:

1. requires a clean Git worktree;
2. archives only the current commit;
3. proves all Cargo dependencies are path-only;
4. runs workspace tests and `xtask verify --all` in an isolated source snapshot;
5. builds `aerocodex` in release mode with Cargo offline;
6. embeds the source commit and Rust target in `aerocodex version --json`;
7. runs success and fail-closed binary smoke checks;
8. writes a release manifest and checksums;
9. creates a deterministic ZIP;
10. re-extracts and verifies the archive, including execution of the packaged binary.

No Git tag, GitHub release, upload, signing, or publication occurs.

## Verify an existing archive

```bash
python scripts/verify_beta1_release.py \
  --archive dist/beta1/aerocodex-0.0.1-beta1-concept-<target>-<commit12>.zip \
  --run-binary
```

The verifier checks path safety, exact file inventory, SHA-256 values, manifest identity, source commit, target, version, release channel, formula count, validation status, self-check totals, signed conversion behavior, invalid-scale rejection, and unknown-formula rejection.

## Required archive payload

Each target archive contains one top-level directory with:

- `bin/aerocodex` or `bin/aerocodex.exe`;
- `release-manifest.json`;
- `SHA256SUMS`;
- `SOURCE_COMMIT.txt`;
- `README.md`;
- `RELEASE_NOTES.md`;
- `LICENSE`, `LICENSE-APACHE`, `LICENSE-MIT`, and `NOTICE`.

## Candidate acceptance

A candidate is testable when:

- repository CI passes;
- the platform packaging workflow passes;
- the archive verifier passes with `--run-binary`;
- the manifest reports `release_channel=beta1-concept` and `package_version=0.0.1`;
- the embedded build commit equals the packaged source commit;
- all six packaged-binary smoke checks pass;
- validation remains `research_required`;
- no operational-readiness, certification, full-inventory, external-parity, or safety claim is added.

Passing this gate authorizes private or internal Beta 1 concept testing only. Publication, signing, tagging, or broader distribution requires a separate release decision.
