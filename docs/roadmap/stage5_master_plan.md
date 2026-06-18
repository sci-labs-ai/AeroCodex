# Stage 5 Master Plan

Stage 5 turns the completed Stage 4 governance surface into a controlled handoff-intake and deployment sequence. Chunk 0 is a documentation-only baseline freeze: it records the live handoff inventory, classification rules, queue order, source boundaries, and current governed counts before any Session A-G handoff or deep BioSim/Orekit work is deployed.

Stage 5 does not by itself certify AeroCodex, promote external source trees, or approve any handoff patch. Every later modifying chunk requires a separate coordinator prompt and must start from current GitHub `main`.

## Safety and certification caveat

AeroCodex remains research, education, verification-oriented development, and preliminary-design software. It is not certified, flight-ready, mission-ready, habitat-safe, medical, operational, or regulated-use approved. Stage 5 keeps the conservative validation posture; new validation material should normally remain `research_required` unless a later bounded review proves a stronger status.

## Verified live base

Chunk 0 froze this planning baseline from live `main` at:

- Base commit: `82f4dd38751c3ca68088d13ca095b0e1a17a1066`
- Remote workflow requirement: exact commit Rust GitHub Actions run must be completed successfully before any modifying chunk starts or closes.
- Root `Cargo.lock` policy at freeze: absent from the repository root.
- Chunk 0 governed-count delta target: zero across all governed counters.

| Count key | Frozen value |
|---|---:|
| `executable_research_equations` | 128 |
| `metadata_only_candidates` | 17 |
| `external_m07_backlog_rows` | 1333 |
| `validation_cards` | 38 |
| `source_registry_seeds` | 36 |
| `validation_card_only_records` | 38 |
| `helper_algorithms` | 89 |

## Stage 5 objective and scope

1. Preserve the pure-Rust, source-traceable AeroCodex core while sequencing Stage 5 handoffs one coherent chunk at a time.
2. Record the live Stage 5 intake hashes and classifications without committing ZIPs, patches, evidence files, external source, generated target output, or local absolute paths.
3. Separate deployable planning/governance material from blocked source archives, aggregate wrappers, deep BioSim/Orekit work-in-progress, and legacy drafts.
4. Keep every substantive math, runtime, template, registry, validation-card, or source-seed change behind a separate focused prompt with live count-delta verification.
5. Keep the validation status conservative unless exact source, test, tolerance, and assurance evidence supports a stronger claim.

## Deployment lanes and milestones

| Lane | Purpose | Milestone rule |
|---|---|---|
| Chunk 0 intake baseline | Freeze inventory, classifications, queue, and operating rules. | Documentation-only, zero governed-count delta. |
| Session D policy/templates | Review validation-card/source-seed generation policy and templates. | Preferred first post-Chunk-0 candidate only after exact scope review. |
| Session C M07 classifier | Split M07 classifier material into coherent documentation/policy and governed-data subchunks. | No bulk M07 source import; no public API promotion. |
| Session B scalar expansion | Review bounded canonical-unit scalar expansion. | Requires math semantics, count-delta, API, and test review. |
| Session A wrap2pi | Review endpoint contract/test metadata. | Requires endpoint behavior policy before deployment. |
| Session G public friend-test | Review public friend-test/readiness material. | Requires wording and scope review to avoid readiness/certification overclaim. |
| Session E/F reusable docs | Consider only reusable documentation after deep-handoff conflict review. | Blocked until explicit authorization resolves BioSim/Orekit deep handoff dependencies. |
| Final deep BioSim/Orekit subpatches | Process deep BioSim and Orekit work only under explicit authorization. | No source-copying, no license-boundary violation, no aggregate deployment. |

## Source and license boundaries

- GitHub `main` is canonical. External folders and ZIPs are intake material, not repository payload.
- Stage 5 ZIPs, patches, extracted evidence, preflight copies, local tool scripts, and report artifacts must remain outside the repository.
- No local user path, credential, token, `.env`, deployment evidence, generated `target/`, patch file, or ZIP bundle may be committed.

## M07 quarantine rules

- The M07 material remains quarantined source material and formula-vault candidate input.
- Do not bulk-import M07 source, Scilab output, generated formula code, fixtures, or release-candidate runtime into public AeroCodex APIs.
- Session C must normally be divided into a documentation/policy slice and a governed-data slice before any live deployment.
- Any M07-derived candidate must carry source IDs, license classification, equations/contracts, domains, singularities, tolerances, tests, validation status, and non-certification caveats.

## GPL BioSim and BioSim-RS separation

- GPL BioSim source and implementation detail must not be copied into the dual MIT/Apache AeroCodex core.
- BioSim-RS and related scenario-engine work remain license-boundaried and clean-room controlled.
- Sessions E, deep BioSim, legacy BioSim/Orekit drafts, and any BioSim runtime code are deferred unless a later prompt explicitly authorizes a bounded source-boundary-compliant slice.

## Orekit non-copying oracle rule

- Orekit may guide reference-oracle planning, terminology review, and validation thinking.
- Do not copy Orekit Java source, preserve its class hierarchy, import class files, or translate implementation structure into AeroCodex.
- Session F and deep Orekit material remain deferred pending deep-handoff conflict review and explicit authorization.

## BioSim/Orekit change-control rule

BioSim and Orekit materials use `deferred_pending_deep_handoff` unless the coordinator explicitly authorizes a bounded BioSim/Orekit slice. Legacy v1, v2-audit, B1a, and O1a materials are reference-only until compared against final deep handoffs. Aggregate wrappers and source archive containers are not deployment candidates.

## Risks, blockers, and dependencies

- Handoff patches can pass `git apply --check` while still being semantically unready; apply-check is a preflight signal only.
- Sessions touching `validation/equation_inventory.tsv`, validation cards, source registry, formula-vault contracts, Rust source, scripts, or `xtask` require focused count-delta and scope review.
- Bare `python` and GitHub authentication availability may differ by environment; deployment closeout must record exact unavailable commands and fallbacks.
- Root `Cargo.lock` may be generated by Cargo and must remain absent unless the repository deliberately changes policy.
- The existing ignored root `target/` directory is local clutter, not repository payload.

## Finish lines

Stage 5 is complete only when all approved chunks are deployed from current `main`, each exact commit CI run is green, governed counts are intentionally updated and documented, forbidden-material scans are clean, temporary branches are removed, and every source/license boundary remains intact.

Exactly one coherent modifying chunk is authorized at a time. A future prompt must restate the authorized scope before Session D or any other substantive Stage 5 work begins.
