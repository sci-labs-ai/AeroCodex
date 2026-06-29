## Summary

Describe the change and list the affected crates, docs, validation cards, data files, or source-registry entries.

## Required CI-equivalent checks

- [ ] `git diff --check`
- [ ] `sha256sum -c checksums/SHA256SUMS`
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace --all-targets --all-features`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-targets --all-features`
- [ ] `cargo run -p xtask -- verify --all`
- [ ] `cargo run -p xtask -- dependency-policy`
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps`

## Nomenclature / acronym policy

- [ ] New durable acronyms, initialisms, shorthand tokens, and source-authority labels are added to `nomenclature/registry/acronyms.yaml`, intentionally waived in `nomenclature/registry/waivers.yaml`, or explicitly covered by the current adoption baseline.
- [ ] Any new acronym record has a source in `nomenclature/registry/terminology_sources.yaml`.
- [ ] Ambiguous acronyms include collision/disambiguation metadata.
- [ ] Durable docs expand acronyms at first use unless the artifact already defines them or a waiver applies.

## Safety / readiness caveat

- [ ] This change does not imply certification, flight readiness, mission readiness, operational approval, medical safety, or habitat-safety approval.
