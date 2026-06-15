# AeroCodex Phase 0.001 Artifact Manifest

Generated artifacts for the final Microtasks 001-020 delivery:

- Repository ZIP: `AeroCodex_repository_foundation_v0_001_microtasks_001_020.zip`
- Repository SHA256 sidecar: `AeroCodex_repository_foundation_v0_001_microtasks_001_020_SHA256.txt`
- Full bundle ZIP: `AeroCodex_Phase_0_001_Microtasks_001_020_Bundle.zip`
- Full bundle SHA256 sidecar: `AeroCodex_Phase_0_001_Microtasks_001_020_Bundle_SHA256.txt`
- Deployment-agent prompt: `AeroCodex_deploy_agent_prompt_v0_001_microtasks_001_020.md`
- Final development report: `docs/phase_0_001/final_microtasks_001_020_report.md`
- Per-microtask checkpoint ZIPs, changed-file ZIPs, patches, checks JSON files, changed-path lists, and SHA256 sidecars preserved in the final bundle under `microtask_artifacts/`.

The repository ZIP contains repository root contents directly at ZIP root, not an extra nested `AeroCodex/` folder.

The SHA256 for the final bundle is provided as an external sidecar because including a ZIP's own final digest inside itself would change the ZIP bytes.


## Thin-film BLSS Rust conversion artifact

This package extends the repository with the thin-film and biofilm BLSS Rust implementation requested after Microtasks 001-020. The repository ZIP generated from this folder should contain repository root contents directly at ZIP root. Added artifact files include:

- `DATA_MANIFEST.toml`
- `citations/blss_thinfilm_refs.bib`
- `data/thinfilm/equation_manifest.csv`
- `data/thinfilm/source_verification.csv`
- `data/thinfilm/parameter_cards/*.toml`
- `data/thinfilm/scenarios/thinfilm_panel_smoke.toml`
- `source_material/new_thinfilm/*`
- `crates/aero-codex-life-support/src/brlss_backbone.rs`
- `crates/aero-codex-life-support/src/melissa_photobioreactor.rs`
- `crates/aero-codex-life-support/src/nitrifying_biofilm.rs`
- `crates/aero-codex-life-support/src/thinfilm_algal_biofilm.rs`
- `crates/aero-codex-life-support/src/thinfilm_provenance.rs`
- `validation/cards/life_support_thinfilm_*.yaml`
- `validation/source_registry/life_support_{thinfilm,poughon,garcia,perez,montras,polizzi,blanken,detrell,vermeulen}*.yaml`
- `scripts/verify_thinfilm_artifact.py`
