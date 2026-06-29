# Formula-vault runtime resolution

A10 closes the ambiguity in the 27 existing formula-vault intake records by linking each formula ID to one already governed Rust runtime. It does not add, copy, or regenerate equation kernels.

## Machine-readable record

`formula-vault/resolutions/m00_runtime_links.tsv` contains one row per formula ID with:

- candidate family and candidate metadata path;
- governed equation-batch manifest and batch ID;
- package, crate name, runtime symbol, and runtime path;
- normalized output variable;
- contract, validation-card, and source-seed paths;
- `research_required` status;
- terminal disposition `linked_to_existing_runtime`.

The 27 rows comprise 3 angle/unit, 14 vector-algebra, and 10 canonical-unit formulas. All are already represented in `equation-batches/m00-angle-vector.tsv` or `equation-batches/m00-canonical-units.tsv`.

## Verification

```text
cargo run -p xtask -- verify formula-vault
cargo run -p xtask -- verify --all
```

The verifier fails closed on duplicate or missing formula IDs, unresolved candidates, mismatched runtime or governance fields, missing repository paths, changed inventory dispositions, or a changed external backlog aggregate. It does not scrape Rust source text.

## Boundaries

- Candidate YAML files remain metadata/provenance records, not implementation source.
- Existing Rust equation kernels remain the sole executable implementation.
- Validation remains `research_required`.
- No M07 or Scilab source parity, certification, flight readiness, mission readiness, operational approval, or regulated-use approval is claimed.
- A11-A33 assign terminal dispositions to 855 external rows; 468 external M07 rows remain unprocessed and incomplete. A12-A13 complete the 74-row vector-helper group with 56 aliases, 13 internal shape-helper exclusions, and 5 contract-blocked rows. A14-A15 complete the 49-row classical two-body algebra group with 22 aliases and 27 contract blocks. A16-A25 complete all 377 orbital-geometry/conic rows with 12 aliases, 90 helper exclusions, and 275 contract or policy blocks. A26-A27 complete the coordinate-transform/frame-graph/time-scale policy backlog with 85 contract or policy blocks, retaining 58 medium-risk and 27 frame/time-policy blocked labels. A28-A30 complete the governed solver / least-squares / numerical propagation policy backlog with 123 contract or policy blocks, retaining 123 blocked-until-solver-policy labels and leaving 0 rows in that candidate pool. A31-A33 complete the relative-motion and finite-burn scalar policy backlog with 109 contract or policy blocks while retaining 29 frame/time-policy blocked labels, 70 high-risk numerical-policy labels, and 10 medium-risk contract-review labels; these waves do not downgrade classifier risk tiers.

- `formula-vault/resolutions/m07_attitude_frame_policy_wave1.tsv`: A34 attitude / inertia / quaternion policy Wave 1, 40 research-required blocked rows, no runtime mappings.

- `formula-vault/resolutions/m07_attitude_frame_policy_wave2.tsv`: A35 attitude / inertia / quaternion policy Wave 2, 19 research-required blocked rows, no runtime mappings.

- `formula-vault/resolutions/m07_attitude_dynamics_control_policy_wave1.tsv`: A36 attitude dynamics/control policy Wave 1, 38 research-required blocked rows, no runtime mappings.

- `formula-vault/resolutions/m07_j2_perturbation_policy_wave1.tsv`: A37 J2 perturbation / numerical propagation policy Wave 1, 40 research-required blocked rows, no runtime mappings.

- `formula-vault/resolutions/m07_j2_perturbation_policy_wave2.tsv`: A38 J2 perturbation / numerical propagation policy Wave 2, 40 research-required blocked rows, no runtime mappings.
- `formula-vault/resolutions/m07_j2_perturbation_policy_wave3.tsv`: A39 J2 perturbation / numerical propagation policy Wave 3, 48 research-required blocked rows, no runtime mappings.

- `formula-vault/resolutions/m07_sgp4_teme_policy_wave1.tsv`: A40 SGP4 / TEME frame-time policy Wave 1, 45 research-required terminal rows, no runtime mappings.
- `formula-vault/resolutions/m07_cr3bp_external_data_policy_wave1.tsv`: A41 CR3BP / external-data / input-output policy Wave 1, 45 research-required terminal rows, no runtime mappings.
A42 classifier-refresh/manual source-review dispositions remain metadata-only and do not add runtime links, formula IDs, or public formula candidates.

A43 scalar/unit helper dispositions remain metadata-only and do not add runtime links, formula IDs, or public formula candidates.

A44 residual scalar/unit/helper dispositions remain metadata-only and do not add runtime links, formula IDs, or public formula candidates.

### A45 final residual backlog closure Wave 1

A45 adds a metadata-only terminal-disposition manifest for the final 18 external M07 backlog rows. It adds no formula runtime and makes no external parity claim.
