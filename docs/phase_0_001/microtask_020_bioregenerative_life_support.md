# Microtask 20 — Bio-Regenerative Life Support v0.001

Status: complete in this session.

## Scope

Microtask 20 reviewed and tightened the existing `aero-codex-life-support` crate for first Phase 0.001 bio-regenerative life-support mass-balance helpers.

The reviewed functions are:

```text
closure_fraction(recycled_mass_rate, total_required_mass_rate)
required_production_area(required_mass_per_day, productivity_per_area_per_day)
buffer_residence_time(buffer_mass, flow_rate)
crew_daily_requirement(crew_count, per_crew_per_day)
net_daily_balance(production_per_day, consumption_per_day)
oxygen_balance(oxygen_production_per_day, oxygen_consumption_per_day)
carbon_dioxide_balance(co2_generation_per_day, co2_removal_per_day)
water_recovery_balance(recovered_water_per_day, required_water_per_day)
verification_record(codex_id)
```

## Implementation notes

The helpers are scalar arithmetic primitives only. They do not model plant physiology, microbial processes, crop scheduling, storage dynamics, cabin atmosphere control, medical safety, crew health, redundancy, reliability, emergency reserves, environmental-control architecture, or ECLSS operations.

Reviewed domain behavior:

- `closure_fraction` requires `recycled_mass_rate >= 0` and `total_required_mass_rate > 0`.
- Closure-style values greater than one are returned with warning code `closure_fraction.gt_one` and `ValidityStatus::OutsideDocumentedDomain` for accounting-boundary review.
- `required_production_area` requires `required_mass_per_day >= 0` and `productivity_per_area_per_day > 0`.
- `buffer_residence_time` requires `buffer_mass >= 0` and `flow_rate > 0`.
- `crew_daily_requirement` uses `u32` for `crew_count`, making crew count nonnegative by type, and requires `per_crew_per_day >= 0`.
- `net_daily_balance` requires nonnegative production and consumption terms and returns signed `production - consumption`.
- `oxygen_balance` returns production minus consumption.
- `carbon_dioxide_balance` returns removal minus generation.
- `water_recovery_balance` uses closure-fraction warning semantics with a water-specific Codex ID.
- Nonfinite derived outputs are reported as `AeroError::NumericalFailure`.

## Validation metadata

Added or updated cards:

```text
validation/cards/life_support_bioregenerative_mass_balance.yaml
validation/cards/life_support_closure_fraction.yaml
validation/cards/life_support_required_production_area.yaml
validation/cards/life_support_buffer_residence_time.yaml
validation/cards/life_support_daily_mass_balance.yaml
```

Added or updated source-registry seeds:

```text
validation/source_registry/life_support_bioregenerative_mass_balance.yaml
validation/source_registry/nasa_life_support_bvad_eclss.yaml
```

All life-support validation cards and source-registry entries remain:

```text
status: research_required
```

No validation card or source-registry entry was upgraded to `equation_traceable`, `implementation_verified`, `reference_validated`, or `experiment_validated`.

## Tests added in source

The `aero-codex-life-support` test module includes scaffolding for:

- closure fraction equals recycled divided by required;
- closure fraction greater than one emits warning metadata;
- required production area decreases as productivity increases;
- zero required mass and zero buffer boundary behavior;
- buffer residence time equals buffer divided by flow;
- crew daily requirement scales linearly with crew count;
- net daily balance sign is correct;
- oxygen and carbon-dioxide wrapper sign conventions;
- water recovery closure metadata;
- invalid input rejection;
- nonfinite derived-output numerical-failure cases;
- life-support verification records remaining `research_required`.

## Checks completed in this environment

- Parsed all Cargo manifests with Python `tomllib`.
- Confirmed `aero-codex-life-support` depends only on `aero-codex-core`.
- Confirmed required Microtask 20 public function names are present.
- Confirmed optional oxygen, carbon-dioxide, and water-recovery wrappers are present.
- Confirmed life-support Codex/source/verification metadata markers are present.
- Confirmed domain-validation and warning markers are present.
- Confirmed finite-output and `NumericalFailure` guard markers are present.
- Confirmed life-support validation cards link to existing source-registry IDs.
- Confirmed all validation cards and source-registry files remain `research_required`.
- Ran static forbidden native/wrapper dependency token scan across Cargo manifests.
- Ran static forbidden readiness marker scan across validation files.
- Ran rough delimiter-balance checks on changed Rust source.
- Generated ZIP artifacts and verified SHA256 sidecars.

## Checks not run here

This environment does not include `rustc` or `cargo`, so the deployment agent must run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo run -p xtask -- verify --all
cargo run -p xtask -- verify cards
cargo run -p xtask -- verify source-registry
cargo run -p xtask -- dependency-policy
cargo doc --workspace --all-features --no-deps
```

## Source verification status

No NASA BVAD/ECLSS source edition, equation number, table, page range, reference example, tolerance, biological model, habitat-safety requirement, or crew-health requirement was verified during this microtask. No validation card or source-registry entry was upgraded from `research_required`.
