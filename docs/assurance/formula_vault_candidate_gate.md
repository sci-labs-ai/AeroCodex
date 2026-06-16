# Formula-vault candidate gate

Stage 4 Chunk 7A defines the minimum gate that must exist before any formula-vault implementation-candidate slice can be considered. It is governance and metadata scaffolding only. It does not import M07 source code, does not create public Rust application programming interfaces, does not parse the external M07 archive, and does not overwrite `crates/aero-codex-astrodynamics`.

AeroCodex remains research and preliminary-design software. This gate is not certification evidence, flight-readiness evidence, mission-readiness evidence, habitat-safety approval, medical-use approval, operational approval, or regulated-use approval.

## Current source boundary

The M07 release-candidate source artifact remains the external data-governance entry `stage4.m07_rust_port_v14.2026_06_15` with SHA256 `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190`.

The registered source facts remain unchanged:

- 1,350 represented function rows;
- 188 Scilab equivalence jobs;
- release-candidate / not certified status;
- no bulk merge into public crates;
- no replacement of the existing astrodynamics crate.

## Candidate-gate rule

No future formula-vault slice may propose public AeroCodex implementation work until it supplies all of the following for the specific formula or formula family:

1. a stable slice identifier and formula identifier list;
2. a source-registry seed for the slice or formula family;
3. a validation card for the slice or formula family;
4. source artifact identifiers, normally including `stage4.m07_rust_port_v14.2026_06_15`;
5. source equation, table, page, function-row, or file-local locators that a reviewer can inspect;
6. variable, unit, coordinate-frame, time-scale, and sign-convention metadata;
7. valid-domain, singularity, invalid-region, branch-behavior, and exclusion metadata;
8. numerical tolerance policy with rationale;
9. reference-oracle, Scilab equivalence, analytical identity, or Simplified General Perturbations 4 (SGP4) gate plan where applicable;
10. explicit promotion-gate status that remains blocked until evidence exists;
11. documentation that repeats the research/preliminary-design caveat;
12. a scan result showing no raw M07 source code, generated binaries, archives, local absolute paths, evidence logs, or public application programming interfaces were promoted accidentally.

The gate is intentionally per-slice. A source-registry seed or validation card for one formula family does not cover unrelated formulas, all 1,350 represented rows, or a bulk import of the M07 workspace.

## Metadata template

Chunk 7A adds a non-operative template at `formula-vault/templates/implementation_candidate_slice.yaml`. The template is a record shape, not an approved formula candidate. Future chunks may copy it into a reviewed record only after selecting a bounded formula family and filling every required field.

Required template sections are:

- `slice` for slice identity and quarantine lifecycle state;
- `sources` for source artifact IDs and human-reviewable locators;
- `formula_contract` for variables, units, frames, time assumptions, domains, singularities, invalid regions, and branch behavior;
- `validation_records` for the required source-registry seed and validation-card IDs;
- `evidence_plan` for Rust gates, Scilab equivalence, SGP4 checks, reference-oracle plans, and tolerance rationale;
- `promotion_gate` for blocked-by-default checklist state;
- `non_claims` for explicit no-certification, no-readiness, no-operational-use, and no-bulk-import caveats.

## Promotion boundary

A future formula-vault implementation candidate remains quarantined unless all of the following are true for that candidate only:

- the source-registry seed and validation card exist and use allowed vocabulary values;
- the machine-readable contract has no placeholder source locators, units, domains, or tolerance fields;
- applicable Rust, source-equivalence, and reference-oracle gates pass;
- documentation names the remaining limitations;
- the changed code path has a bounded public surface and no unreviewed M07 implementation copy;
- the active chunk explicitly approves the bounded promotion.

Chunk 7A does not itself authorize any formula implementation, validation-status upgrade, Scilab execution, SGP4 certification, external fixture import, public astrodynamics application programming interface, or release-readiness claim.

## Chunk 7C verifier scaffold

Chunk 7C adds `cargo run -p xtask -- verify formula-vault` as a dependency-free scaffold check for candidate metadata. The verifier checks required candidate sections, existing source/validation cross-links, duplicate slice/formula identifiers, blocked promotion state, required non-claim booleans, and absence of local evidence paths. It is included in `cargo run -p xtask -- verify --all`.

The verifier is not numerical validation, source-equivalence evidence, certification evidence, readiness evidence, or implementation approval.
