# M07 formula-family validation-card strategy

This Stage 5 Session C candidate is a planning strategy for future M07 formula-family validation records. It does not create validation cards, source-registry seeds, formula-vault candidates, public Rust APIs, fixtures, generated code, or source imports.

## Boundary

The classifier material is derived from metadata for registered source artifact `stage4.m07_rust_port_v14.2026_06_15` with SHA256 `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190`. It remains quarantined planning material. No M07 Scilab source text, comments, control flow, generated Rust, Scilab output, archives, binaries, or fixtures are bundled by this strategy.

## Recommended model

Use family-level validation cards plus per-formula inventory rows only when a future bounded chunk promotes a reviewed formula-family slice to formula-vault metadata. Do not create one validation card per M07 row; the classifier has 1,333 remaining backlog rows and row-level cards would create noisy governance without source contracts, family policy, and reviewed generation behavior.

Session D already established validation-card and source-seed generation policy plus non-operative templates. Future M07 family cards should use those templates or a later separately authorized generator. Until such generator behavior is explicitly reviewed, create cards manually from the templates and keep every card at `research_required` unless a later verifier and prompt authorize another status.

## Future family-card fields

Each future family validation card should record:

1. a stable family/slice identifier;
2. `status: research_required` unless a later verifier explicitly permits a status change;
3. the registered source artifact ID and exact metadata row locators used by that slice;
4. assumptions for units, frames, time scales, branch conventions, singularities, invalid inputs, and numerical tolerances;
5. analytical, reference-oracle, or Scilab-equivalence evidence requirements;
6. negative tests and explicitly blocked domains;
7. links to any matching source-registry seed and formula-vault candidate metadata;
8. explicit non-claims for certification, flight readiness, mission readiness, operational approval, habitat safety, medical use, and regulated use;
9. a no-source-import boundary statement.

## Inventory integration

When a future formula-family slice is promoted to formula-vault metadata, update governed counts as deltas from live main rather than hard-coded absolute counts:

- `metadata_only_formula_vault_candidate`: increase only by selected formula metadata candidates;
- `external_m07_backlog_row`: decrease only by the same selected row count when rows leave the backlog;
- `validation_card_only_record`: increase only when a new family validation card is added;
- `source_registry_seeds`: increase only when a new source seed is added;
- `executable_research_equation`: unchanged unless a later implementation chunk adds public research kernels and updates inventory;
- `helper_algorithm`: unchanged unless the verifier classification changes.

## Non-claims

This strategy does not approve low-risk classifier rows for implementation. It does not certify source traceability, numerical parity, frame/time correctness, physical validity, operational readiness, flight readiness, mission readiness, habitat safety, medical use, or regulated use. Solver, rank, tolerance, frame/time, propagator-frame, J2/perturbation, restricted-three-body, external-data, and `app_resolve_coplanar` families remain policy-gated until separate bounded prompts resolve them.
