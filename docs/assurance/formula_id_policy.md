# Formula ID alias and migration policy

Source of truth: AeroCodex Research Readiness Master Plan v0.7.2, task RR-018.

This policy defines stable formula identity before registry generation and CLI migration work. It is docs/policy only: it does not generate `generated/formula_registry.json`, does not parse sidecars, does not implement formula execution, does not make formulas executable, does not promote validation status, and does not rewrite equation-batch manifests.

AeroCodex remains research/preliminary-design software and is not certified as operational aerospace software. Formula identity metadata is traceability infrastructure, not certification or readiness evidence.

## Scope

This policy governs future formula registry generation, sidecar naming, CLI alias resolution, and migration records for formula IDs. It applies to readable canonical IDs, legacy `formula_vault.*` aliases, migration/deprecation records, and user-facing CLI behavior.

This policy does not authorize:

- edits to real `equation-batches/*.tsv` manifests;
- validation status changes or validation-card edits;
- M07 promotion, M07 execution, or M07 quarantine changes;
- generated registry artifacts or generated Rust modules;
- product CLI code, runtime formula code, or formula execution code.

## Canonical readable IDs

Canonical formula IDs are the preferred public identifiers in the generated Formula Registry v1 and future `aerocodex formula ...` surfaces. They should be stable, lowercase, dotted, short enough for users to type, and readable enough for notebooks, tests, scripts, and reviews.

Examples of acceptable canonical readable IDs:

| Canonical ID | Intended shape |
|---|---|
| `m00.angle.deg_to_rad` | M00 angle unit conversion leaf. |
| `m00.angle.rad_to_deg` | M00 angle unit conversion leaf. |
| `gasdyn.normal_shock.mach2` | Gas-dynamics family, normal-shock subfamily, downstream Mach leaf. |
| `aero.lift.coefficient` | Aerodynamics family, lift subfamily, coefficient leaf. |
| `astro.orbit.vis_viva` | Astrodynamics family, orbit subfamily, vis-viva leaf. |

Canonical ID rules:

1. Use a stable lowercase dotted namespace: `family.subfamily.leaf` or `family.subfamily.topic.leaf` when another level is needed.
2. Prefer family/subfamily/leaf structure where practical.
3. Keep IDs short but readable; avoid opaque hashes, row numbers, or generated-only names.
4. Do not include unnecessary prefixes such as `formula_vault.` in canonical IDs.
5. Do not embed status or validation words such as `research_required`, `verified`, `validated`, `blocked`, `candidate`, or `promoted` in canonical IDs.
6. Do not embed certification, readiness, approval, or authority claims in canonical IDs.
7. Do not rewrite IDs opportunistically just because a shorter or prettier name is noticed later.
8. Do not use canonical IDs to imply runtime availability; execution remains controlled by status gates.

## Legacy alias policy

`formula_vault.*` IDs are legacy aliases, not preferred canonical IDs. They preserve compatibility and traceability for Beta 1 concept commands, notebooks, tests, scripts, issue references, formula-vault manifests, and existing provenance records.

Legacy alias rules:

1. Legacy `formula_vault.*` IDs must remain traceable to their canonical formula entry.
2. Registry entries sourced from legacy manifests must carry `legacy_formula_id` when there is a primary legacy ID, or an explicit `aliases` list when there are multiple governed aliases.
3. `aliases` may include `legacy_formula_id`, legacy CLI spellings, or other governed compatibility spellings, but aliases must be deterministic and reviewable.
4. Legacy aliases remain accepted for at least one release cycle, or until a governed migration map says otherwise.
5. Legacy aliases must not bypass status gates, registry freshness checks, runtime-symbol checks, domain checks, test-vector checks, or M07 quarantine checks.
6. Legacy aliases must not make `research_required` formulas executable.
7. Legacy aliases must not make M07 candidates executable, authoritative, or promoted.
8. Ambiguous alias conflicts must fail closed rather than guessing a canonical formula.

For the locked M00 Slice A map, the readable canonical IDs are `m00.angle.deg_to_rad` and `m00.angle.rad_to_deg`; their existing `formula_vault.*` spellings remain legacy aliases for traceability until a governed migration window closes.

## No silent ID rewrites

No silent ID rewrites are allowed. A rename, canonicalization, deprecation, or alias migration is a governed change, not an incidental generator behavior.

Every rename or alias migration record must include:

- old ID;
- new ID;
- reason;
- reviewer;
- date or review marker;
- deprecation window;
- affected manifests, sidecars, registry entries, notebooks, tests, scripts, or user-facing docs;
- compatibility behavior during and after the deprecation window.

Migration must preserve reproducibility for notebooks, tests, and scripts. A historical run, fixture, or notebook that names an old alias must still resolve deterministically during the accepted deprecation window, and after removal it must fail with an explicit migration/error message rather than silently choosing a different formula.

## Registry behavior for RR-015 and later generation tasks

The future registry generator must normalize IDs only through this approved alias policy and governed migration data. It must not invent canonical IDs by stripping prefixes from arbitrary strings unless the mapping is already approved.

Generator requirements:

1. Treat `formula_id` as the canonical readable ID in each generated formula entry.
2. Preserve `legacy_formula_id` where available from legacy manifests or sidecars.
3. Preserve any additional governed aliases in a deterministic `aliases` list.
4. Expose alias metadata so registry consumers can explain how a lookup resolved.
5. Report duplicate canonical IDs as errors.
6. Report duplicate aliases as errors, including duplicate `formula_vault.*` aliases.
7. Fail closed on ambiguous alias conflicts or missing migration authority.
8. Keep generated registry output deterministic: stable ordering, stable alias sorting, stable source hashing, and no wall-clock timestamps.
9. Keep execution policy independent from alias choice; alias resolution identifies the formula before status gates decide whether execution is allowed.
10. Do not hand-author generated formula registry artifacts and do not create `generated/formula_registry.json` in this policy task.

## Future CLI behavior

Future CLI tasks must make canonical IDs visible while preserving governed legacy aliases.

Expected behavior:

1. `aerocodex formula describe <id>` should show the canonical `formula_id` and, when applicable, the `legacy_formula_id` and/or `aliases` that resolve to it.
2. `aerocodex formula run <id>` should warn in human output when `<id>` is a legacy alias instead of the canonical ID.
3. JSON output for describe/run should include stable fields equivalent to `canonical_formula_id` and `alias_used` so clients can distinguish the requested spelling from the canonical identity.
4. If a canonical ID is requested, `alias_used` should be null or false-equivalent by the future JSON contract.
5. If a legacy alias is requested, `canonical_formula_id` should name the resolved canonical ID and `alias_used` should preserve the exact requested alias spelling.
6. Alias resolution must happen before execution gating so status-gate errors can name the canonical formula and the alias that was used.
7. Alias resolution must not bypass execution gating, M07 quarantine, or non-claim warnings.
8. Deprecated aliases should warn in human output and use the future public error-code/migration behavior when a deprecation window closes.

## M07 posture

M07 candidate aliases do not imply authority. M07 material remains visible-but-blocked inventory unless a later governed family-promotion task explicitly promotes a formula through traceability, schema, implementation, and validation evidence.

Alias mapping cannot:

- promote M07;
- unblock M07;
- treat M07 material as authoritative;
- make M07 formulas executable;
- change M07 validation status;
- convert M07 candidate records into normal registry authority.

An M07 candidate that has an alias or future canonical readable ID must still retain blocked posture, including `quarantine_state: "m07_candidate_blocked"` and `execution_policy: "blocked"` where represented in registry-derived inventory.

## Sidecar naming and metadata alignment

Formula sidecars should name the canonical `formula_id` and the primary legacy `legacy_formula_id` when a legacy alias exists. The sidecar path should mirror the canonical readable ID, not the legacy `formula_vault.*` spelling.

Sidecars may list additional aliases as metadata, but sidecars do not override this policy, do not generate registry artifacts, do not promote validation status, and do not make formulas executable.

## Non-claims

Formula IDs, aliases, sidecars, registry inclusion, and migration records are traceability metadata. They are not validation evidence, status promotion, runtime implementation, execution authorization, or certification evidence.

AeroCodex formula identity policy is for research/preliminary-design software. It does not provide operational certification, regulated-use approval, external parity, or safety-critical authorization.
