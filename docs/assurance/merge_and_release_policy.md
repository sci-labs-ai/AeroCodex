# Merge and Release Policy

This policy keeps Stage 4 work on one canonical, testable `main` while preserving conservative release and safety claims.

## Safety status

AeroCodex is research and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved. A merge to `main` is not a certification event.

## Canonical main

- GitHub `main` is the source of truth.
- Keep one canonical `main`; avoid long-running branches and competing integration heads.
- Use a short-lived branch only when needed for review or pull-request checks.
- After checks pass, the deployment agent merges or fast-forwards into `main`, pushes `main`, verifies the remote commit, and deletes the short-lived branch.
- Do not leave the user with manual merge cleanup.

## Version policy

- Cargo package versions must remain valid semantic versions.
- Human roadmap labels such as Stage 4, Chunk 0, and M07 are not Cargo package versions.
- Do not encode roadmap labels as package versions such as `0.002` or `0.003`.
- Version changes require an explicit release decision and an updated release note.

## Merge gates

Before merging a Stage 4 chunk, run the required checks listed in `removed deployment operating-rules document` or the active prompt, whichever is stricter. Record pass, fail, or unavailable status exactly. If a check is unavailable, do not call the chunk fully verified; report the tool dependency.

## Source-boundary release gates

External material may be merged only in the correct state:

- Planning/governance docs may be merged after documentation checks.
- Formula-vault material may be staged only with source IDs, license status, equation contracts, tests, tolerance policy, and validation status.
- M07 public API promotion is blocked until Rust continuous integration, Scilab equivalence, and SGP4 certification pass.
- BioSim-RS integration is blocked from the dual MIT/Apache core until its GPL-3.0-or-later boundary is resolved by a deliberate licensing path.
- Orekit may inform tests and architecture, but Java class hierarchy cloning is blocked.

## Release labels

Use conservative status language:

- `source_material`
- `quarantined_candidate`
- `implemented_research`
- `reference_checked`
- `source_equivalent`
- `release_candidate`

Do not use flight-ready, mission-ready, habitat-safe, medical, operational, regulated-use-approved, or certified language unless a separate scoped assurance package exists and explicitly authorizes that claim.

The `beta1-concept` label is permitted only as a software release-channel experiment. It does not alter the Cargo `0.0.1` version lock, validation status, external parity status, public API stability promise, or any safety/certification boundary.

## Required closeout evidence

Every Stage 4 chunk closeout must include:

- branch name if used;
- commit hash on `main`;
- push status and remote verification;
- files changed;
- checks run and exact statuses;
- source-boundary decisions;
- intentional deferrals and blockers;
- next recommended chunk.
## Beta 1 candidate artifact gate

A Beta 1 concept archive may be produced only from a clean committed source snapshot. The candidate packager must run Cargo offline, prove the current dependency graph is repository-local and path-only, embed the source commit and target in the binary, emit SHA-256 checksums and a release manifest, and execute the packaged binary smoke contract after archive extraction.

Candidate packaging does not authorize a Git tag, GitHub release, signing event, publication, operational-readiness claim, certification claim, or validation-status upgrade. Those actions require a separate explicit release decision.
