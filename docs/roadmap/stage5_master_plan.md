# Stage 5 Master Plan

Stage 5 turns the completed Stage 4 governance surface into a controlled handoff-intake and deployment sequence. This plan records the live handoff inventory, classification rules, queue order, source boundaries, and governed-count expectations before any later Stage 5 runtime/code handoff is deployed.

Stage 5 does not by itself certify AeroCodex, promote external source trees, or approve any handoff patch. Every later modifying chunk requires a separate coordinator prompt and must start from current GitHub `main`.

## Safety and certification caveat

AeroCodex remains research, education, verification-oriented development, and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved. Stage 5 keeps the conservative validation posture; new validation material should normally remain `research_required` unless a later bounded review proves a stronger status.

## Verified live base

Chunk 0 froze the first Stage 5 planning baseline from live `main` at:

- Base commit: `82f4dd38751c3ca68088d13ca095b0e1a17a1066`
- Remote workflow requirement: exact commit Rust GitHub Actions run must be completed successfully before any modifying chunk starts or closes.
- Root `Cargo.lock` policy at freeze: absent from the repository root.
- Chunk 0 governed-count delta target: zero across all governed counters.

After the bounded Session D deployment and taxonomy-remediation closeout, this plan was re-synced at accepted live main `dc8bd005cd33b76ca32ec5b5ad5d1f40111801c4`. Current live counts are derived by the verifiers, not hardcoded deployment targets:

| Count key | Current live value at this planning integration |
|---|---:|
| `executable_research_equations` | 128 |
| `metadata_only_candidates` | 17 |
| `external_m07_backlog_rows` | 1333 |
| `validation_cards` | 39 |
| `source_registry_seeds` | 37 |
| `validation_card_only_records` | 39 |
| `helper_algorithms` | 89 |

## Stage 5 objective and scope

1. Preserve the pure-Rust, source-traceable AeroCodex core while sequencing Stage 5 handoffs one coherent chunk at a time.
2. Record the live Stage 5 intake hashes and classifications without committing ZIPs, patches, evidence files, external source, generated target output, or local absolute paths.
3. Separate deployable planning/governance material from blocked source archives, aggregate wrappers, deep BioSim/Orekit work, and legacy drafts.
4. Keep every substantive math, runtime, template, registry, validation-card, or source-seed change behind a separate focused prompt with live count-delta verification.
5. Keep the validation status conservative unless exact source, test, tolerance, and assurance evidence supports a stronger claim.

## Deployment lanes and milestones

| Lane | Purpose | Milestone rule |
|---|---|---|
| Chunk 0 intake baseline | Freeze inventory, classifications, queue, and operating rules. | Documentation-only, zero governed-count delta, completed. |
| Session D policy/templates | Deployed validation-card/source-seed generation policy and non-operative templates. | Completed by bounded Session D; one template card/source seed is counted by current recursive verifiers. |
| Session D taxonomy remediation | Align post-Session-D taxonomy and governed inventory semantics. | Completed before this planning integration; accepted commit must remain an ancestor of live main. |
| Session C split review | Split M07 classifier material into coherent documentation/policy and governed-data subchunks. | Review completed with caveats; no canonical repo modification, branch, commit, or patch application occurred during review. |
| Session C C1 docs/policy candidate | Documentation/policy adaptation candidate produced by the split review. | Next ready non-code deployment candidate; must still run in a separate bounded deployment prompt. |
| Session C C2 classifier dataset candidate | Atomic classifier dataset patch after C1. | Needs review; deterministic source-locator normalization and data-registry planning require explicit coordinator/user approval before any deployment. |
| Session B scalar expansion | Review bounded canonical-unit scalar expansion. | Requires math semantics, count-delta, API, and test review. |
| Session A wrap2pi | Review endpoint contract/test metadata. | Requires endpoint behavior policy before deployment. |
| Session G public friend-test | Review public friend-test/readiness material. | Requires wording and scope review to avoid readiness/certification overclaim. |
| BioSim/Orekit v3 serial intake | Process final deep BioSim and Orekit v3 work as serial subpatches only. | No Session E/F, old v1/v2, B1a/O1a, or monolithic material is a direct replacement for bounded v3 serial intake. |

## Session C split-review result

The Session C split-review is complete. It produced two candidate subpatches from the original monolithic M07 classifier handoff:

- C1 documentation/policy adaptation is classified `ready_for_live_intake` and is the next ready non-code deployment candidate.
- C2 classifier dataset remains `needs_review` pending explicit approval of the reviewer-identified absolute Scilab extraction locator normalization and explicit data-registry entry planning.
- The original monolithic Session C patch is not recommended for direct deployment and remains blocked as a deployment target.
- C1 and C2 both touch `docs/index.md`; C2 must remain ordered after C1.

No source-locator normalization rule is approved by this plan.

## BioSim/Orekit v3 arrival and serial deployment rule

Final deep BioSim and Orekit v3 handoff bundles have now arrived. They replace the old placeholder state where deep BioSim/Orekit material was merely `deferred_pending_deep_handoff`, but arrival is not deployment approval.

- BioSim/Orekit runtime deployment must proceed serially through the v3 subpatches recorded in `docs/roadmap/stage5_bio_ore_v3_serial_plan.md`.
- Do not deploy Session E/F, old v1/v2, B1a/O1a, or monolithic aggregate material as a shortcut around v3 serial intake.
- Orekit v3 and BioSim v3 retain source/license boundaries and `research_required` status.
- Orekit v3 has a nomenclature/acronym blocker before any O2 deployment.
- BioSim v3 B2a needs known test corrections before live deployment intake.

## Source and license boundaries

- GitHub `main` is canonical. External folders and ZIPs are intake material, not repository payload.
- Stage 5 ZIPs, patches, extracted evidence, preflight copies, local tool scripts, and report artifacts must remain outside the repository.
- No local user path, credential, token, `.env`, deployment evidence, generated `target/`, patch file, or ZIP bundle may be committed.

## M07 quarantine rules

- The M07 material remains quarantined source material and formula-vault candidate input.
- Do not bulk-import M07 source, Scilab output, generated formula code, fixtures, or release-candidate runtime into public AeroCodex APIs.
- Session C is split into C1 documentation/policy and C2 governed-data classifier rows before any live deployment.
- Any M07-derived candidate must carry source IDs, license classification, equations/contracts, domains, singularities, tolerances, tests, validation status, and non-certification caveats.

## GPL BioSim and BioSim-RS separation

- GPL BioSim source and implementation detail must not be copied into the dual MIT/Apache AeroCodex core.
- BioSim-RS and related scenario-engine work remain license-boundaried and clean-room controlled.
- BioSim v3 is final deep-session intake material, not a live-deployable runtime patch until future bounded live review closes every source-boundary and gate requirement.

## Orekit non-copying oracle rule

- Orekit may guide reference-oracle planning, terminology review, and validation thinking.
- Do not copy Orekit Java source, preserve its class hierarchy, import class files, or translate implementation structure into AeroCodex.
- Orekit v3 is final deep-session intake material, not a live-deployable runtime patch until future bounded live review closes nomenclature, numerical-policy, source-boundary, and gate requirements.

## BioSim/Orekit change-control rule

BioSim and Orekit v3 materials are now the canonical future deep-intake lanes. Legacy v1, v2-audit, B1a, and O1a materials are audit/history references only where the v3 comparison identifies a successor; they are not deployment candidates. Aggregate wrappers and source archive containers remain blocked.

## Risks, blockers, and dependencies

- Handoff patches can pass `git apply --check` while still being semantically unready; apply-check is a preflight signal only.
- Sessions touching `validation/equation_inventory.tsv`, validation cards, source registry, formula-vault contracts, Rust source, scripts, or `xtask` require focused count-delta and scope review.
- Bare `python` and GitHub authentication availability may differ by environment; deployment closeout must record exact unavailable commands and fallbacks.
- Root `Cargo.lock` may be generated by Cargo and must remain absent unless the repository deliberately changes policy.
- The existing ignored root `target/` directory is local clutter, not repository payload.

## Finish lines

Stage 5 is complete only when all approved chunks are deployed from current `main`, each exact commit CI run is green, governed counts are intentionally updated and documented, forbidden-material scans are clean, temporary branches are removed, and every source/license boundary remains intact.

Exactly one coherent modifying chunk is authorized at a time. A future prompt must restate the authorized scope before C1 or any other substantive Stage 5 work begins.
