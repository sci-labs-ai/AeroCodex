# Stage 5 Master Plan

Stage 5 turns the completed Stage 4 governance surface into a controlled handoff-intake and deployment sequence. This plan records the live status after the bounded BioSim v3 B2b-1 process-types, validated-constructor, and deterministic intent-planner deployment. It does not certify AeroCodex, promote external source trees, or approve the next implementation chunk.

AeroCodex remains research, education, verification-oriented development, and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved. Stage 5 keeps the conservative validation posture; validation material remains `research_required` unless a later bounded review proves a stronger status.

## Verified live state

- Current live main at B2b-1 deployment start: `13bcc241da6791189109d698690cb5c7cabdec66`.
- Root `Cargo.lock` policy: absent from the repository root.
- Exact-current-main CI requirement: Rust GitHub Actions must be completed successfully for the exact head before a modifying chunk starts or closes.

| Count key | Current live value | Meaning |
|---|---:|---|
| Executable research equations | 151 | Public Rust research/preliminary-design equation kernels inventoried by `validation/equation_inventory.tsv`. |
| Metadata-only formula-vault candidates | 27 | Formula-vault candidate metadata records; not implementations by themselves. |
| External M07 backlog rows | 1,323 | Registered external M07 represented rows not yet selected as formula-vault candidates. C2 classification does not remove rows from this backlog. |
| Validation cards | 45 | Conservative validation/governance records. They are not certification evidence. |
| Source-registry seeds | 43 | Source/governance traceability seeds. |
| Validation-card-only records | 45 | Metadata records, not formula implementations. |
| Helper algorithms | 204 | Support routines not counted as executable research equations. |

The B2b-1 deployment intentionally updates governed equation-inventory totals by +28 helper algorithms; executable research equations, metadata-only candidates, external M07 backlog rows, validation cards, source-registry seeds, and validation-card-only records remain unchanged from B2a.

## Completed Stage 5 lanes

| Lane | Current status | Deployment evidence | Remaining scope |
|---|---|---|---|
| Chunk 0 intake baseline | completed | `04058a81182efd1ebadfc366ced9ebebc5b03af6` | None. |
| Session D policy/templates | completed | `86b8db10aa3976406b0cd46ce1cf15b60bd9822f` | None. |
| Session D taxonomy remediation | completed | `dc8bd005cd33b76ca32ec5b5ad5d1f40111801c4` | None. |
| Session C split review | completed review | recorded by the queue/inventory split evidence | Monolithic Session C remains blocked as a direct-deploy target. |
| Session C1 docs/policy adaptation | deployed/completed | `3925dd6bb0639180a2311d10cd8060e4700d61ed` | None. |
| Session C2 classifier dataset | deployed/completed; governance coverage closed by this reconciliation | dataset `617996193aba0322d8591bb8e1b7755bbe4e1baf`; checksum/manifest refresh `989ba7b33b4c6ee83c213387e5bbbb34bd65348b` | Research/planning metadata only; no source implementation approval and no M07 backlog reduction. |
| Session B canonical-unit scalar expansion | deployed/completed | `fe45e11a6b457e0c2cc146e25f270d04e7141ce4` | None for Session B. |
| Orekit v3 O2a time/frame/state foundation | deployed/completed | `2f1e64ea7638b2f54071eca488c26252256235ca` | O2b, O2c, and O2d now deployed separately as bounded research/preliminary-only slices. |
| Orekit v3 O2b classical-elements/Kepler foundation | deployed/completed | `9ce001940bb3e423bf97e499a079e27eb0502f5a` | O2c and O2d now deployed separately; no operational Orekit parity claim. |
| Orekit v3 O2c oracle-record/tolerance-comparison helpers | deployed/completed | `c493efc5079892c97d7ceee8c4a9b74955d1ddab` | Local deterministic record/tolerance comparison only; no Orekit execution, external fixture import, evidence verification, two-line-element parsing, SGP4, TEME transform, propagation, or parity claim. |
| Orekit v3 O2d two-line-element contract/source-policy metadata | deployed/completed | `9b2a8e24e2ee55c9840371868ca0ab8343cdeb07` | Contract-only source policy and fail-closed prerequisite evaluation; no parser, checksum algorithm, epoch/orbital-field decoder, SGP4, TEME transform, propagation, operational tracking, or parity claim. |
| BioSim v3 corrected B2a scenario-domain and structural validation | deployed/completed | `13bcc241da6791189109d698690cb5c7cabdec66` | Clean-room synthetic scenario resource kinds, metadata/compartment/store/clock records, and deterministic structural validation only; no process flows, replay, adapter, ledger, report, example, complete engine, controller, biological-fidelity, habitat-safety, medical, operational, parity, or certification claim. |
| BioSim v3 B2b-1 process-types/validated-constructor/intent-planner foundation | deployed/completed | this bounded B2b-1 deployment commit | Clean-room synthetic process identifiers, source/sink/transfer/transform records, validated constructors, deterministic one-tick intent planning, and planner guards only; no scenario-state mutation, replay, adapter, ledger, report, example, digest, complete engine, controller, biological-fidelity, habitat-safety, medical, operational, parity, or certification claim. |
| Adapted Session E BioSim-plus docs/contracts | deployed/completed | `9dcc303336d12e401c4a866b3bc2410c937014dd` | Does not complete deep BioSim v3 B2b/B2c; B2a now provides only the bounded scenario-domain/structural-validation foundation. |
| Session G public friend-test package | deployed/completed | `286dab75fef46a9d729fbff3650636162dc4c8e4` | Public counts must track current main. |
| Session A wrap2pi endpoint contract/test metadata | deployed/completed | `28e3a7697c9d17559d22414abbdca9284646d629`; label fix `e20754cb3d2856a1b28c6808c96d7ed5d1871bdf` | Executable/public `wrap2pi` remains blocked. |
| Professional hardening | completed | flight dynamics `2412dfb25f1cb369d4bcb60c76b32c3cd8b2bf0f`; aerodynamics/local gates `59bbac1081457b1772019fc6851d7a2e07484141` | No new runtime scope. |
| Adapted Session F reference-oracle metadata | deployed/completed | `68dc10fc9215df2be9bc64e0f2a94121250c361a` | Planning metadata only; O2b, O2c, and O2d are deployed separately as bounded research/preliminary-only slices. |

## Remaining lanes

1. BioSim B2b-2 bounded replay/event model is the next recommended bounded implementation candidate. It still requires a separate prompt and must not start from this plan.
2. Orekit O2c is deployed as local deterministic record/tolerance-comparison infrastructure only; it does not execute Orekit or verify evidence.
3. Orekit O2d is deployed as contract/source-policy metadata only; it does not parse two-line-element records, implement checksums, run SGP4, perform TEME transforms, propagate orbits, or claim parity.
4. BioSim B2b-3 and B2c remain blocked by predecessors; B2b-2 remains not started by B2b-1.
5. Runtime/public `wrap2pi` remains blocked pending a separate endpoint-behavior runtime decision.
6. `app_resolve_coplanar` remains blocked pending rank, tolerance, and solver policy.

## C2 data-governance closure rule

C2 is deployed as planning metadata. The classifier contains 1,333 rows tied to registered artifact `stage4.m07_rust_port_v14.2026_06_15` and M07 outer SHA256 `15b1ca3a39267187167c43ea1228f28fd4736c4456f65d072dc42a32a7b19190`. C2 leaves `external_m07_backlog_rows` unchanged at 1,323 because classification is not formula-vault selection or source promotion.

The reconciled registry coverage is explicit, not aggregate-only: the core classifier CSV, low-risk derivative CSV, and blocked/high-risk derivative CSV each have a dedicated DATA_REGISTRY entry with exact SHA256. DATA_MANIFEST.toml remains thin-film-specific and is not expanded to carry C2 files.

## Source and license boundaries

- GitHub `main` is canonical. External folders and ZIPs are intake material, not repository payload.
- No local user path, credential, token, environment file, deployment evidence, generated `target/`, patch file, or ZIP bundle may be committed.
- No M07 source, Scilab source/output, generated source output, GPL BioSim/BioSim-RS source, Orekit Java source, archives, fixtures, or binaries may be imported by Stage 5 deployment chunks.
- Orekit may guide reference-oracle planning only; do not copy Java source or clone its class hierarchy.
- BioSim remains clean-room/source-boundary controlled; no habitat, medical, operational, or regulated-use claim is authorized.

## Finish-line criteria

Stage 5 is complete only when all approved chunks are deployed from current `main`, each exact commit CI run is green, governed counts are intentionally updated and documented, forbidden-material scans are clean, temporary branches are removed, and every source/license boundary remains intact.

Exactly one coherent modifying chunk is authorized at a time. The next recommended bounded chunk is BioSim v3 B2b-2 compartment-replay/compact-digest/atomic-event-model re-cut review and deployment, but this plan does not authorize or start it.
