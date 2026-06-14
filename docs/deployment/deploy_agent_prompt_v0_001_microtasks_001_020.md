# AeroCodex Phase 0.001 Microtasks 001-020 Deployment-Agent Prompt

You are the deployment agent for AeroCodex. Your job is to integrate the delivered Phase 0.001 Microtasks 001-020 repository foundation into GitHub, run the required checks, fix real failures, and open a pull request. Do not claim production, flight, mission, habitat, medical, safety-critical, or certification readiness.

## Target repository

```text
ConorMcGibboney/AeroCodex
```

## Safety and certification caveat to preserve

```text
AeroCodex is an engineering mathematics library for research, education, verification-oriented development, and preliminary design. Safety-critical, regulated, or mission use requires project-specific assurance, validation, qualification, and certification.
```

AeroCodex is not certified, flight-ready, mission-ready, or approved for aircraft or spacecraft operations.

## Branch and unpacking steps

```bash
git clone https://github.com/ConorMcGibboney/AeroCodex.git
cd AeroCodex
git checkout -b phase-0.001-microtasks-001-020
```

Unpack this artifact from the final bundle:

```text
AeroCodex_repository_foundation_v0_001_microtasks_001_020.zip
```

Copy the files into the repository root without creating an extra nested repository folder. The repository ZIP is built with repository root contents at ZIP root.

## Required verification commands

Run every command below from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

Run these version-lock sanity checks as well:

```bash
grep -n 'version = "0.0.1"' Cargo.toml
grep -RIn '0\.001' --include Cargo.toml . && exit 1 || true
grep -RIn '^version = ' crates xtask --include Cargo.toml && exit 1 || true
grep -RIn '^version[.]workspace = true' crates xtask --include Cargo.toml
```

If Rust is missing, install stable Rust using rustup and rerun all commands. Fix real failures. Do not bypass tests or weaken policy checks.

## Required confirmations before commit

Confirm all of the following:

```text
no forbidden native dependencies
no wrapper dependencies
no C/C++/Fortran source added
no generated binaries
no target/ directory committed
dual MIT/Apache-2.0 license preserved
NOTICE preserved
certification caveat preserved
validation card structure preserved
validation cards reference existing source-registry IDs
Codex Card schema scaffold preserved
all validation cards and source-registry seeds remain research_required unless separately justified by evidence
Phase 0.001 documented as the human roadmap phase
Cargo package versions remain 0.0.1
Cargo manifests do not use 0.001 as a package version
roadmap docs distinguish Phase 0.001 from Cargo semver
```

## Files and pages to verify/update

Review at least:

```text
README.md
docs/index.md
docs/roadmap/versioning.md
docs/roadmap/milestones.md
docs/phase_0_001/final_microtasks_001_020_report.md
docs/phase_0_001/source_research_backlog.md
validation/README.md
validation/schema/codex_card.schema.json
validation/cards/
validation/source_registry/
```

Update the GitHub project page or repository landing docs to point to the Phase 0.001 documentation and final report.

## Commit, push, and pull request

```bash
git add .
git status --short
git commit -m "feat: add Phase 0.001 aerospace equation foundations"
git push -u origin phase-0.001-microtasks-001-020
```

Open a pull request into `main`.

## Pull request body must include

```text
summary
equations added
crates added or changed
tests run
checks passed/failed
source verification limitations
known research_required items
no-wrapper policy confirmation
certification caveat confirmation
next microtasks 21-40
```

Do not claim production, flight, mission, habitat, medical, safety-critical, or certification readiness in the PR title, body, commit message, README, docs, validation cards, or release notes.
