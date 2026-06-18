# Stage 5 Handoff Inventory

This inventory records the Chunk 0 intake state for Stage 5 materials. It records hashes and classifications in repository documentation only. It does not commit ZIPs, patches, evidence, extracted files, external source, generated outputs, or local absolute paths.

## Classification values

- `ready_for_live_intake`: hash matches; archive is structurally safe; handoff and patch scope are coherent; patch preflight succeeds against current live `main` or an equivalent manual preflight is valid; no unresolved source-boundary issue, pending deep BioSim/Orekit dependency, known conflict, or required split remains. This means eligible for a future bounded deployment prompt only.
- `needs_review`: technically accessible but semantic, mathematical, governance, dependency, count-delta, or scope questions remain; patch should be divided before deployment; or preflight warnings/overlaps need review.
- `blocked`: authorization is absent; deep BioSim/Orekit dependency exists; source-license or quarantine boundary prevents deployment; hash or structural safety fails; or the artifact is a source archive or aggregate wrapper, not a deployable handoff.
- `superseded`: explicit evidence identifies a newer canonical replacement. No Chunk 0 item is marked superseded by assumption.

## A-G handoff inventory

| Session | Filename | SHA256 | Bytes | Declared purpose | Patch and SHA256 | Live preflight | Classification | Priority | Dependencies/conflicts | Source-boundary constraints | Advisory expected count deltas |
|---|---|---|---:|---|---|---|---|---:|---|---|---|
| A | `aerocodex_session_A_wrap2pi_handoff.zip` | `c99428fb1eeabf0b905379dc0ca42b71ae96055dd0395cc4c6f08257b12c0a1b` | 30333 | wrap2pi endpoint contract/test metadata | `patch.patch` `afc2b27e3b6759f8098c8c83f9aa2d43ae902c968c5a514a09e2bc6c1b09bfc1` (41634 bytes) | patch.patch: PASS rc=0 | needs_review | 5 | Separate future prompt required. | No external source import; preserve M07/BioSim/Orekit boundaries. | Advisory validation/source/formula-vault contract deltas possible; compute live. |
| B | `AeroCodex_Session_B_Chunk8C_handoff.zip` | `b09ed269b9377791f7e4d04ad6e1c070f8d6b0d904c0a2270154f8a9f6b31466` | 51444 | bounded canonical-unit scalar expansion | `rust_patch.patch` `ed4248267326ff02c81a4dd2728b371e0debe2d0ad63a4114f8d696294dca6d7` (69809 bytes); `patch.patch` `ed4248267326ff02c81a4dd2728b371e0debe2d0ad63a4114f8d696294dca6d7` (69809 bytes) | rust_patch.patch: PASS rc=0; patch.patch: PASS rc=0 | needs_review | 4 | Separate future prompt required. | No external source import; preserve M07/BioSim/Orekit boundaries. | Advisory executable/formula-vault/validation/source/xtask deltas possible; compute live. |
| C | `aerocodex_session_c_m07_classifier_handoff.zip` | `9d4e11d69dfa6aa4340214bde581c98aea0eb93b7f01490642c33c88319e4ac6` | 192137 | M07 classifier; divide into coherent subchunks | `patch.patch` `4f0e410bed692ef7a53e582b2cdd9182d2e6a0a4477e5eb3a0102e972acf6a32` (2037265 bytes) | patch.patch: PASS rc=0 | needs_review | 3 | Must split documentation/policy and governed-data slices. | M07 quarantine; no Scilab/source bulk import. | Advisory docs/source-intake classifier row additions; split before computing governed impact. |
| D | `aerocodex_session_d_handoff.zip` | `f8af7de84052a116286c257db4c0101815af306fc1e5d3f25324f059747fe140` | 27332 | validation-card/source-seed generation policy and templates | `patch.patch` `47c9428114cb7a7c5f16349c23c77fc3d479f70f82cbc11bfcaa064152ba0af1` (21564 bytes) | patch.patch: PASS rc=0 | ready_for_live_intake | 1 | Preferred first post-Chunk-0 candidate; still requires bounded prompt and live review. | No external source import; preserve M07/BioSim/Orekit boundaries. | Advisory nonzero template/schema/equation-inventory metadata delta possible; compute live. |
| E | `aerocodex_session_e_biosim_plus_clean_room_handoff.zip` | `6509d6fb7e887ed194fa3744df19359313eaa2746ca991f58580365e05d71253` | 46390 | BioSim-plus clean-room material deferred pending deep handoff | `patch.patch` `5466d411df988894e518f8f79bfc9ebc2da283594cfd8c18b838100bfb4a2a54` (44554 bytes) | patch.patch: PASS rc=0 | blocked | 7 | deferred_pending_deep_handoff. | GPL BioSim/BioSim-RS separation; no BioSim runtime/source import. | Not deployable now; no count delta authorized. |
| F | `AeroCodex_session_F_orekit_oracle_handoff.zip` | `e08ad597ca47db11a755d037ae0a11ae0b6a6efaae265a5d40017dc67fe97d8a` | 25815 | Orekit reference-oracle material deferred pending deep handoff | `patch.patch` `9939c88798a687e386b868b7b6ebafa29c2d1abe9f81b69b6d439253cc64569a` (16589 bytes) | patch.patch: PASS rc=0 | blocked | 8 | deferred_pending_deep_handoff. | Orekit oracle only; no Java source or class hierarchy copy. | Not deployable now; no count delta authorized. |
| G | `AeroCodex_session_G_public_friend_test_handoff.zip` | `51936d358138bc3102ed076854b528075f1173ea866c68e14062434ad2176495` | 37898 | public friend-test/readiness material | `patch.patch` `ace3b24209da3a2fd3a8205bbe45e80a9f266b0fa7550ca4410508742c371abe` (29946 bytes) | patch.patch: PASS rc=0 | needs_review | 6 | Separate future prompt required. | No external source import; preserve M07/BioSim/Orekit boundaries. | Advisory docs/scripts/validation/source/equation-inventory deltas possible; compute live. |

## A-G patch preflight details

### Session A patch preflight

- `patch.patch`: sha256 `afc2b27e3b6759f8098c8c83f9aa2d43ae902c968c5a514a09e2bc6c1b09bfc1`, 41634 bytes, git apply --check `PASS`, flags: touches_governed_equation_inventory. Affected: `docs/assurance/formula_vault_m00_wrap2pi_endpoint_policy.md`, `formula-vault/contracts/m00_wrap2pi_endpoint_contract.yaml`, `formula-vault/contracts/m00_wrap2pi_test_vectors.csv`, `validation/cards/validation_formula_vault_m00_wrap2pi_endpoint_policy.yaml`, `validation/equation_inventory.tsv`, `validation/source_registry/source_formula_vault_m00_wrap2pi_endpoint_policy.yaml`. Diffstat: .../formula_vault_m00_wrap2pi_endpoint_policy.md | 65 +++ .../contracts/m00_wrap2pi_endpoint_contract.yaml | 512 ++++++++++++++++++++ .../contracts/m00_wrap2pi_test_vectors.csv | 27 + ..._formula_vault_m00_wrap2pi_end...
### Session B patch preflight

- `rust_patch.patch`: sha256 `ed4248267326ff02c81a4dd2728b371e0debe2d0ad63a4114f8d696294dca6d7`, 69809 bytes, git apply --check `PASS`, flags: touches_governed_equation_inventory, touches_rust_source_or_examples. Affected: `crates/aero-codex-astrodynamics/src/lib.rs`, `docs/assurance/m00_canonical_unit_conversions_equation_expansion.md`, `formula-vault/candidates/m00_canonical_unit_conversions.yaml`, `formula-vault/contracts/m00_canonical_unit_conversions_contract.yaml`, `validation/cards/validation_formula_vault_m00_canonical_unit_conversions.yaml`, `validation/equation_inventory.tsv`, `validation/source_registry/source_formula_vault_m00_canonical_unit_conversions.yaml`, `xtask/src/main.rs`. Diffstat: crates/aero-codex-astrodynamics/src/lib.rs | 337 ++++++++++++++++++++ ...anonical_unit_conversions_equation_expansion.md | 59 ++++ .../candidates/m00_canonical_unit_conversions.yaml | 169 ++++++++++ .../m00_canonical_...
- `patch.patch`: sha256 `ed4248267326ff02c81a4dd2728b371e0debe2d0ad63a4114f8d696294dca6d7`, 69809 bytes, git apply --check `PASS`, flags: touches_governed_equation_inventory, touches_rust_source_or_examples. Affected: `crates/aero-codex-astrodynamics/src/lib.rs`, `docs/assurance/m00_canonical_unit_conversions_equation_expansion.md`, `formula-vault/candidates/m00_canonical_unit_conversions.yaml`, `formula-vault/contracts/m00_canonical_unit_conversions_contract.yaml`, `validation/cards/validation_formula_vault_m00_canonical_unit_conversions.yaml`, `validation/equation_inventory.tsv`, `validation/source_registry/source_formula_vault_m00_canonical_unit_conversions.yaml`, `xtask/src/main.rs`. Diffstat: crates/aero-codex-astrodynamics/src/lib.rs | 337 ++++++++++++++++++++ ...anonical_unit_conversions_equation_expansion.md | 59 ++++ .../candidates/m00_canonical_unit_conversions.yaml | 169 ++++++++++ .../m00_canonical_...
### Session C patch preflight

- `patch.patch`: sha256 `4f0e410bed692ef7a53e582b2cdd9182d2e6a0a4477e5eb3a0102e972acf6a32`, 2037265 bytes, git apply --check `PASS`, flags: none. Affected: `docs/assurance/validation_family_card_strategy.md`, `docs/roadmap/recommended_chunks_8c_to_10e.md`, `docs/source_intake/m07_formula_family_classifier/blocked_high_risk_rows.csv`, `docs/source_intake/m07_formula_family_classifier/low_risk_candidate_shortlist.csv`, `docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv`, `docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier_summary.md`. Diffstat: docs/assurance/validation_family_card_strategy.md | 51 + docs/roadmap/recommended_chunks_8c_to_10e.md | 85 + .../blocked_high_risk_rows.csv | 595 +++++++++ .../low_risk_candidate_shortlist.csv | 155 ++ .../m07_formula...
### Session D patch preflight

- `patch.patch`: sha256 `47c9428114cb7a7c5f16349c23c77fc3d479f70f82cbc11bfcaa064152ba0af1`, 21564 bytes, git apply --check `PASS`, flags: touches_governed_equation_inventory. Affected: `docs/assurance/source_seed_generation_policy.md`, `docs/assurance/validation_card_generation_policy.md`, `validation/cards/templates/template_formula_family_validation_card.yaml`, `validation/equation_inventory.tsv`, `validation/schema/codex_card.schema.json`, `validation/source_registry/templates/template_formula_family_source_seed.yaml`. Diffstat: docs/assurance/source_seed_generation_policy.md | 81 +++++++++++++++++ .../assurance/validation_card_generation_policy.md | 96 ++++++++++++++++++++ .../template_formula_family_validation_card.yaml | 46 ++++++++++ vali...
### Session E patch preflight

- `patch.patch`: sha256 `5466d411df988894e518f8f79bfc9ebc2da283594cfd8c18b838100bfb4a2a54`, 44554 bytes, git apply --check `PASS`, flags: blocked_by_prompt_deferred_pending_deep_handoff, touches_governed_equation_inventory. Affected: `biosim_plus_friend_test_report_v2.md`, `biosim_plus_minimal_habitat_scenario_contract.yaml`, `docs/architecture/biosim_plus_clean_room_scenario_schema.md`, `docs/assurance/biosim_plus_ledger_invariants.md`, `docs/assurance/biosim_plus_replay_policy.md`, `validation/cards/validation_biosim_plus_clean_room_scenario_schema.yaml`, `validation/equation_inventory.tsv`, `validation/source_registry/source_biosim_plus_clean_room_scenario_schema.yaml`. Diffstat: biosim_plus_friend_test_report_v2.md | 105 ++++++++++++ biosim_plus_minimal_habitat_scenario_contract.yaml | 175 ++++++++++++++++++++ .../biosim_plus_clean_room_scenario_schema.md | 165 +++++++++++++++++++ docs/assura...
### Session F patch preflight

- `patch.patch`: sha256 `9939c88798a687e386b868b7b6ebafa29c2d1abe9f81b69b6d439253cc64569a`, 16589 bytes, git apply --check `PASS`, flags: blocked_by_prompt_deferred_pending_deep_handoff, touches_governed_equation_inventory. Affected: `docs/assurance/orekit_reference_oracle_plan.md`, `validation/cards/validation_orekit_reference_oracle_mapping.yaml`, `validation/equation_inventory.tsv`, `validation/source_registry/source_orekit_reference_oracle_mapping.yaml`. Diffstat: docs/assurance/orekit_reference_oracle_plan.md | 63 ++++++++++++++++++++ ...validation_orekit_reference_oracle_mapping.yaml | 49 ++++++++++++++++ validation/equation_inventory.tsv | 1 .../source_orekit_reference_oracl...
### Session G patch preflight

- `patch.patch`: sha256 `ace3b24209da3a2fd3a8205bbe45e80a9f266b0fa7550ca4410508742c371abe`, 29946 bytes, git apply --check `PASS`, flags: touches_governed_equation_inventory. Affected: `docs/roadmap/public_alpha_readiness_dashboard.md`, `docs/testing/friend_test_expected_output.md`, `docs/testing/friend_test_quickstart.md`, `docs/testing/research_safety_caveats_for_testers.md`, `scripts/friend_test_local.ps1`, `scripts/friend_test_local.sh`, `validation/cards/validation_public_friend_test_package.yaml`, `validation/equation_inventory.tsv`, `validation/source_registry/source_public_friend_test_package.yaml`. Diffstat: docs/roadmap/public_alpha_readiness_dashboard.md | 67 ++++++++++++++++ docs/testing/friend_test_expected_output.md | 81 +++++++++++++++++++ docs/testing/friend_test_quickstart.md | 83 ++++++++++++++++++++ .../testing/...

## Deferred, blocked, reference-only, and aggregate materials

### ACX-NOM exact filename: deep BioSim clean-room scenario-engine handoff zip
### ACX-NOM exact filename: deep Orekit astrodynamics-foundation handoff zip
### ACX-NOM exact filename: legacy BioSim/Orekit aggregate zip
### ACX-NOM exact filename: legacy BioSim B1a draft directory
### ACX-NOM exact filename: legacy Orekit O1a draft directory
### ACX-NOM exact filename: BioSim v2 gap-audit document
### ACX-NOM exact filename: Orekit v2 gap-audit document

| Artifact | Role | SHA256 | Bytes | Classification | Reason |
|---|---|---|---:|---|---|
| `AeroCodex_stage5_deep_parallel_recovery_protocol.zip` | process material | `071fd8b34f55bee873a89f8c87dbb0dbddf469f99021dd765d3cfc0a8d31100d` | 8845 | blocked | process_material_not_deployable_repository_payload |
| deep BioSim clean-room scenario-engine handoff zip | deep BioSim handoff | `1ba752034f5a9c6d92e57fd72a5d9e5b4c6c47ad4c6e726809fd3b8d99b04bf0` | 34525 | blocked | deferred_pending_deep_handoff |
| deep Orekit astrodynamics-foundation handoff zip | deep Orekit handoff | `f1397b594d6a01599acb647acd669161f5afdbf5760a515af3f6bd6fb2d4e97a` | 38065 | blocked | deferred_pending_deep_handoff |
| `BioSim and Orekit v.zip` | legacy BioSim/Orekit aggregate | `ef4f18c1c929f9cb19e0e864c34f491a522a3cd9fe389aa658f3953ca6f19a5b` | 30975 | blocked | reference_only_pending_comparison_with_final_deep_handoffs |
| legacy BioSim B1a draft directory | legacy BioSim B1a draft directory | `` | 512 | blocked | reference_only_pending_comparison_with_final_deep_handoffs |
| legacy Orekit O1a draft directory | legacy Orekit O1a draft directory | `` | 512 | blocked | reference_only_pending_comparison_with_final_deep_handoffs |
| BioSim v2 gap-audit document | BioSim v2 audit | `fcecf05bd342d837824e296224cfef711196f04bc82e853c972113213a372901` | 42446 | blocked | reference_only_gap_audit_not_deployment_candidate |
| Orekit v2 gap-audit document | Orekit v2 audit | `eb511e29788473109f4fd864c0997ac1ffcc2c85bc54ad87fe47ca400e157aa1` | 48611 | blocked | reference_only_gap_audit_not_deployment_candidate |
| `stage 5.zip` | aggregate intake wrapper | `a9b91b9324c5bc1c7c46ca298ba408e649fc90e5133f1b04a81332307715de16` | 53719660 | blocked | aggregate_wrapper_not_deployable_patch |
| `files-aerocodex.zip` | external/source archive container | `9d81aec11e1d7065d994ff98f91b60647669610df0f8ce49f1039a9a09dd00ba` | 53311016 | blocked | source_archive_container_not_deployable_patch |

## Top-level non-ZIP handoff/audit documents

| Document | SHA256 | Bytes | Role |
|---|---|---:|---|
| `AeroCodex_parallel_browser_sessions_7_prompts.md` | `46c610e1a489adc0d0288d41695fe79289febbc257d0a36f119edca867eb4bf3` | 21908 | top-level Stage 5 audit/prompt/source-boundary reference; not repository payload |
| `AeroCodex_stage4_chunk_runner_prompt_pack.md` | `78149fabeba7ce56d257b5db3e68736a65177b2bc597dd859fb9d33f3117f7e5` | 12354 | top-level Stage 5 audit/prompt/source-boundary reference; not repository payload |
| `AeroCodex_stage5_bio_ore_parallel_rust_sessions.md` | `30bdb1813a1dcd0ad2bfc213c713faf106798834060988662d01607511036580` | 50877 | top-level Stage 5 audit/prompt/source-boundary reference; not repository payload |
| `AeroCodex_stage5_deployment_coordinator_bootstrap_prompt.md` | `7ab67514ab468144bee0c1bb43e1f634f530bbe639a527345b81a61b943b840c` | 18031 | top-level Stage 5 audit/prompt/source-boundary reference; not repository payload |
| BioSim v2 gap-audit document | `fcecf05bd342d837824e296224cfef711196f04bc82e853c972113213a372901` | 42446 | top-level Stage 5 audit/prompt/source-boundary reference; not repository payload |
| Orekit v2 gap-audit document | `eb511e29788473109f4fd864c0997ac1ffcc2c85bc54ad87fe47ca400e157aa1` | 48611 | top-level Stage 5 audit/prompt/source-boundary reference; not repository payload |

## Intake hash verification summary

| Artifact | Bytes | SHA256 | Structural status | Classification | Reason |
|---|---:|---|---|---|---|
| `AeroCodex_parallel_browser_sessions_7_prompts.md` | 21908 | `46c610e1a489adc0d0288d41695fe79289febbc257d0a36f119edca867eb4bf3` | not_zip | recorded | top-level Stage 5 material hash recorded |
| `aerocodex_session_A_wrap2pi_handoff.zip` | 30333 | `c99428fb1eeabf0b905379dc0ca42b71ae96055dd0395cc4c6f08257b12c0a1b` | PASS | needs_review | endpoint-sensitive wrap2pi policy/test metadata requires bounded semantic review before deployment |
| `AeroCodex_Session_B_Chunk8C_handoff.zip` | 51444 | `b09ed269b9377791f7e4d04ad6e1c070f8d6b0d904c0a2270154f8a9f6b31466` | PASS | needs_review | substantive canonical-unit scalar expansion needs live mathematical/count-delta review before deployment |
| `aerocodex_session_c_m07_classifier_handoff.zip` | 192137 | `9d4e11d69dfa6aa4340214bde581c98aea0eb93b7f01490642c33c88319e4ac6` | PASS | needs_review | must be divided into a documentation/policy slice and governed-data slice before deployment |
| `aerocodex_session_d_handoff.zip` | 27332 | `f8af7de84052a116286c257db4c0101815af306fc1e5d3f25324f059747fe140` | PASS | ready_for_live_intake | hash matched, archive safe, docs/policy/template scope coherent, patch preflight passed; eligible for a future bounded prompt only |
| `aerocodex_session_e_biosim_plus_clean_room_handoff.zip` | 46390 | `6509d6fb7e887ed194fa3744df19359313eaa2746ca991f58580365e05d71253` | PASS | blocked | deferred_pending_deep_handoff |
| `AeroCodex_session_F_orekit_oracle_handoff.zip` | 25815 | `e08ad597ca47db11a755d037ae0a11ae0b6a6efaae265a5d40017dc67fe97d8a` | PASS | blocked | deferred_pending_deep_handoff |
| `AeroCodex_session_G_public_friend_test_handoff.zip` | 37898 | `51936d358138bc3102ed076854b528075f1173ea866c68e14062434ad2176495` | PASS | needs_review | public friend-test/readiness material requires wording/scope review before deployment |
| `AeroCodex_stage4_chunk_runner_prompt_pack.md` | 12354 | `78149fabeba7ce56d257b5db3e68736a65177b2bc597dd859fb9d33f3117f7e5` | not_zip | recorded | top-level Stage 5 material hash recorded |
| `AeroCodex_stage5_bio_ore_parallel_rust_sessions.md` | 50877 | `30bdb1813a1dcd0ad2bfc213c713faf106798834060988662d01607511036580` | not_zip | recorded | top-level Stage 5 material hash recorded |
| `AeroCodex_stage5_deep_parallel_recovery_protocol.zip` | 8845 | `071fd8b34f55bee873a89f8c87dbb0dbddf469f99021dd765d3cfc0a8d31100d` | PASS | blocked | process_material_not_deployable_repository_payload |
| `AeroCodex_stage5_deployment_coordinator_bootstrap_prompt.md` | 18031 | `7ab67514ab468144bee0c1bb43e1f634f530bbe639a527345b81a61b943b840c` | not_zip | recorded | top-level Stage 5 material hash recorded |
| deep BioSim clean-room scenario-engine handoff zip | 34525 | `1ba752034f5a9c6d92e57fd72a5d9e5b4c6c47ad4c6e726809fd3b8d99b04bf0` | PASS | blocked | deferred_pending_deep_handoff |
| deep Orekit astrodynamics-foundation handoff zip | 38065 | `f1397b594d6a01599acb647acd669161f5afdbf5760a515af3f6bd6fb2d4e97a` | PASS | blocked | deferred_pending_deep_handoff |
| `BioSim and Orekit v.zip` | 30975 | `ef4f18c1c929f9cb19e0e864c34f491a522a3cd9fe389aa658f3953ca6f19a5b` | PASS | blocked | reference_only_pending_comparison_with_final_deep_handoffs |
| BioSim v2 gap-audit document | 42446 | `fcecf05bd342d837824e296224cfef711196f04bc82e853c972113213a372901` | not_zip | blocked | reference_only_gap_audit_not_deployment_candidate |
| `files-aerocodex.zip` | 53311016 | `9d81aec11e1d7065d994ff98f91b60647669610df0f8ce49f1039a9a09dd00ba` | PASS | blocked | source_archive_container_not_deployable_patch |
| Orekit v2 gap-audit document | 48611 | `eb511e29788473109f4fd864c0997ac1ffcc2c85bc54ad87fe47ca400e157aa1` | not_zip | blocked | reference_only_gap_audit_not_deployment_candidate |
| `stage 5.zip` | 53719660 | `a9b91b9324c5bc1c7c46ca298ba408e649fc90e5133f1b04a81332307715de16` | PASS | blocked | aggregate_wrapper_not_deployable_patch |

## Operating notes

- All known ZIP hashes and byte sizes matched the Chunk 0 expected values.
- All A-G archives passed structural path safety checks for traversal, absolute path, drive-letter, and unsafe link behavior.
- Patch preflight was performed only against fresh temporary git-archive copies of current live `main`; no A-G patch was applied to the canonical repository in Chunk 0.
- Mode warnings with `git apply --check` return code 0 are caveats, not failures and not deployment approval.
- Deep BioSim/Orekit, legacy BioSim/Orekit drafts, aggregate wrappers, source containers, and recovery protocol material remain deferred or reference-only.
