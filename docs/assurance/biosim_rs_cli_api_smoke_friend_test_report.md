# BioSim-RS CLI/API Smoke and Friend-Test Report Validation

Stage 4 Chunk 6E adds a minimal clean-room static smoke slice for the BioSim-RS public API and a package-example friend-test report. It builds on the Chunk 6A resource/tick primitives, the Chunk 6B atomic commit primitive, the Chunk 6C deterministic replay proof, and the Chunk 6D resource-ledger/minimal oxygen-loop proof in `crates/aero-codex-life-support/src/biosim_resource_tick.rs`.

AeroCodex remains research and preliminary-design software. This slice is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved.

## Implemented scope

Chunk 6E introduces:

- `biosim_cli_api_smoke_codex_id()` for `life_support.biosim_rs.cli_api_smoke_friend_test_report`;
- `biosim_cli_api_smoke_clean_room_source_id()` for the Chunk 6E source seed;
- `BioSimCliApiSmokeReport` for static API/example-output smoke evidence;
- `run_biosim_cli_api_smoke_report()` for deterministic built-in smoke execution over the existing clean-room helpers;
- `format_biosim_friend_test_report(...)` for friend-test report text;
- package example `crates/aero-codex-life-support/examples/biosim_friend_test_smoke.rs`.

The example command is:

```bash
cargo run -p aero-codex-life-support --example biosim_friend_test_smoke
```

It prints a deterministic report containing:

- `status: research_required`;
- the Chunk 6E codex id;
- API and command-line smoke booleans;
- built-in resource catalog count;
- bounded minimal oxygen-loop tick count and pass/fail state;
- deterministic replay before/after/delta digests;
- explicit non-claim wording.

## Clean-room and license boundary

This slice does not import or translate Java BioSim implementation code, Java comments, Java package structure, translated tests, BioSim scenario files, GPL-bound fixtures, external golden-master outputs, or the BioSim-RS bootstrap scaffold crates. The implementation is a static smoke/reporting wrapper over the existing dual `MIT OR Apache-2.0` life-support crate.

The relevant validation artifacts are:

- validation card: `validation/cards/life_support_biosim_rs_cli_api_smoke_friend_test_report.yaml`;
- source seed: `validation/source_registry/life_support_biosim_rs_cli_api_smoke_clean_room.yaml`;
- inherited Chunk 6D card: `validation/cards/life_support_biosim_rs_resource_ledger_minimal_o2_loop_conservation.yaml`;
- inherited Chunk 6D source seed: `validation/source_registry/life_support_biosim_rs_resource_ledger_clean_room.yaml`.

The validation status remains `research_required`.

## Explicit non-scope

Chunk 6E itself does not add:

- BioSim scenario execution;
- external fixture loading or golden-master parity;
- persistent resource-ledger storage;
- module scheduling or operational command surfaces;
- biological dynamics;
- habitat-control behavior;
- no crew-safety, medical, operational, certification, or regulated-use approval;
- BioSim archive import;
- GPL-compatible distribution decision beyond this clean-room smoke/reporting slice.

## Gate expectations

A complete Chunk 6E closeout should keep evidence outside the repository and include:

- focused test-first red/green evidence for the static API smoke report and friend-test formatter;
- `cargo run -p aero-codex-life-support --example biosim_friend_test_smoke` output evidence;
- `cargo fmt --all -- --check`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `cargo test --workspace --all-features`;
- `cargo test -p aero-codex-life-support biosim_resource_tick -- --nocapture`;
- `cargo run -p xtask -- verify --all`;
- `cargo run -p xtask -- verify data-registry`;
- `cargo run -p xtask -- verify status-vocabulary`;
- `cargo run -p xtask -- dependency-policy`;
- `cargo doc --workspace --all-features --no-deps`;
- Rust-only public governance checks using `cargo run -p xtask -- verify --all`;
- safety scans proving no GPL source, archives, fixtures, generated binaries, evidence logs, `target/`, or root `Cargo.lock` are staged.

The next BioSim-RS implementation-oriented chunk is Chunk 6F only if a later roadmap update authorizes it. Otherwise Stage 4 returns to formula-vault and implementation-candidate work under the per-slice source/validation gates.
