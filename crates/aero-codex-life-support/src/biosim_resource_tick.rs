use aero_codex_core::{
    validation, AeroError, AeroResult, EngineeringResult, ValidityStatus, VerificationRecord,
};

/// Codex identifier for the clean-room BioSim-RS resource catalog gate.
#[must_use]
pub fn biosim_resource_catalog_codex_id() -> &'static str {
    "life_support.biosim_rs.resource_catalog_identity"
}

/// Codex identifier for the clean-room BioSim-RS tick-semantics gate.
#[must_use]
pub fn biosim_tick_validation_codex_id() -> &'static str {
    "life_support.biosim_rs.tick_validation"
}

/// Source-registry seed for the Chunk 6A clean-room resource/tick slice.
#[must_use]
pub fn biosim_resource_tick_clean_room_source_id() -> &'static str {
    "source.life_support.biosim_rs.resource_tick_clean_room.research_required"
}

fn biosim_resource_tick_sources() -> &'static [&'static str] {
    &["source.life_support.biosim_rs.resource_tick_clean_room.research_required"]
}

/// Minimal clean-room resource families for the first BioSim-RS validation slice.
///
/// These are intentionally generic resource identities, not a translation of
/// GPL BioSim Java classes or the GPL-bound BioSim-RS scaffold crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BioSimResourceKind {
    OxygenGas,
    CarbonDioxideGas,
    PotableWater,
    WasteWater,
    EdibleBiomass,
    DryWaste,
    ElectricalEnergy,
    ThermalEnergy,
}

/// Canonical identity and unit metadata for one BioSim-RS resource kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BioSimResourceIdentity {
    pub kind: BioSimResourceKind,
    pub canonical_id: &'static str,
    pub canonical_unit: &'static str,
}

/// Positive-duration simulation tick after syntactic validation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimTick {
    pub index: u64,
    pub duration_seconds: f64,
}

/// Consecutive tick transition accepted by the Chunk 6A validation gate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimTickAdvance {
    pub previous_index: u64,
    pub next_index: u64,
    pub next_duration_seconds: f64,
}

/// Conservative built-in resource catalog for future BioSim-RS slices.
#[must_use]
pub fn biosim_resource_catalog() -> &'static [BioSimResourceKind] {
    &[
        BioSimResourceKind::OxygenGas,
        BioSimResourceKind::CarbonDioxideGas,
        BioSimResourceKind::PotableWater,
        BioSimResourceKind::WasteWater,
        BioSimResourceKind::EdibleBiomass,
        BioSimResourceKind::DryWaste,
        BioSimResourceKind::ElectricalEnergy,
        BioSimResourceKind::ThermalEnergy,
    ]
}

/// Canonical metadata for a clean-room BioSim-RS resource kind.
#[must_use]
pub const fn biosim_resource_identity(kind: BioSimResourceKind) -> BioSimResourceIdentity {
    match kind {
        BioSimResourceKind::OxygenGas => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.oxygen_gas",
            canonical_unit: "kg",
        },
        BioSimResourceKind::CarbonDioxideGas => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.carbon_dioxide_gas",
            canonical_unit: "kg",
        },
        BioSimResourceKind::PotableWater => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.potable_water",
            canonical_unit: "kg",
        },
        BioSimResourceKind::WasteWater => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.waste_water",
            canonical_unit: "kg",
        },
        BioSimResourceKind::EdibleBiomass => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.edible_biomass",
            canonical_unit: "kg",
        },
        BioSimResourceKind::DryWaste => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.dry_waste",
            canonical_unit: "kg",
        },
        BioSimResourceKind::ElectricalEnergy => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.electrical_energy",
            canonical_unit: "kWh",
        },
        BioSimResourceKind::ThermalEnergy => BioSimResourceIdentity {
            kind,
            canonical_id: "biosim_rs.resource.thermal_energy",
            canonical_unit: "kWh",
        },
    }
}

/// Conservative traceability metadata for the Chunk 6A clean-room slice.
#[must_use]
pub fn biosim_resource_tick_verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        id if id == biosim_resource_catalog_codex_id() => Some(VerificationRecord::research_required(
            biosim_resource_catalog_codex_id(),
            biosim_resource_tick_sources(),
            "Clean-room resource identity validation implemented; no GPL BioSim source, scaffold code, fixtures, or habitat-operation evidence is included.",
        )),
        id if id == biosim_tick_validation_codex_id() => Some(VerificationRecord::research_required(
            biosim_tick_validation_codex_id(),
            biosim_resource_tick_sources(),
            "Clean-room tick validation implemented for positive-duration and consecutive-index checks; no transaction commit, ledger, replay, or external validation evidence is included.",
        )),
        _ => None,
    }
}

fn resource_tick_record(codex_id: &'static str) -> VerificationRecord {
    biosim_resource_tick_verification_record(codex_id).unwrap_or_else(|| {
        VerificationRecord::research_required(
            codex_id,
            biosim_resource_tick_sources(),
            "BioSim-RS Chunk 6A helper is present but has no upgraded source-validation status.",
        )
    })
}

fn out_of_domain(parameter: &'static str, value: f64, expected: &'static str) -> AeroError {
    AeroError::OutOfDomain {
        parameter,
        value,
        expected,
    }
}

/// Validates that a clean-room BioSim-RS resource catalog is nonempty and unique.
///
/// The enum carries the allowed resource identity vocabulary; this function only
/// checks catalog structure for the first implementation slice. It does not
/// create resource ledgers, execute transactions, or validate biological control
/// behavior.
pub fn validate_biosim_resource_catalog(
    catalog: &[BioSimResourceKind],
) -> AeroResult<EngineeringResult<usize>> {
    if catalog.is_empty() {
        return Err(out_of_domain(
            "resource_catalog",
            0.0,
            "at least one clean-room resource identity",
        ));
    }

    for (index, kind) in catalog.iter().enumerate() {
        if catalog[..index].contains(kind) {
            return Err(out_of_domain(
                "resource_catalog",
                index as f64,
                "unique resource identities",
            ));
        }
    }

    Ok(EngineeringResult::new(
        catalog.len(),
        biosim_resource_catalog_codex_id(),
        resource_tick_record(biosim_resource_catalog_codex_id()),
    )
    .with_assumption(
        "biosim_rs.clean_room_chunk6a",
        "resource identities are clean-room generic mass/energy buckets, not GPL source translations",
    )
    .with_assumption(
        "biosim_rs.resource_identity_only",
        "catalog validation does not imply transaction, ledger, replay, or biological model validation",
    )
    .with_validity(ValidityStatus::WithinDocumentedDomain))
}

/// Validates a positive-duration BioSim-RS simulation tick.
pub fn validate_biosim_tick(
    index: u64,
    duration_seconds: f64,
) -> AeroResult<EngineeringResult<BioSimTick>> {
    validation::ensure_positive("tick_duration_seconds", duration_seconds)?;

    let validity = if index == 0 {
        ValidityStatus::BoundaryCase
    } else {
        ValidityStatus::WithinDocumentedDomain
    };

    let mut result = EngineeringResult::new(
        BioSimTick {
            index,
            duration_seconds,
        },
        biosim_tick_validation_codex_id(),
        resource_tick_record(biosim_tick_validation_codex_id()),
    )
    .with_assumption(
        "biosim_rs.discrete_tick",
        "tick duration is finite, strictly positive seconds and index is caller-supplied",
    )
    .with_assumption(
        "biosim_rs.tick_validation_only",
        "tick validation does not commit resource deltas or prove deterministic replay",
    )
    .with_validity(validity);

    if index == 0 {
        result = result.with_warning(
            "biosim_rs.tick.initial_boundary",
            "tick index zero is treated as an initialization boundary before dynamic replay evidence exists",
        );
    }

    Ok(result)
}

/// Validates that the next BioSim-RS tick is exactly one index after the previous tick.
pub fn validate_biosim_tick_advance(
    previous: BioSimTick,
    next: BioSimTick,
) -> AeroResult<EngineeringResult<BioSimTickAdvance>> {
    validation::ensure_positive("previous_tick_duration_seconds", previous.duration_seconds)?;
    validation::ensure_positive("next_tick_duration_seconds", next.duration_seconds)?;

    let expected_next = previous.index.checked_add(1).ok_or_else(|| {
        out_of_domain(
            "previous_tick_index",
            previous.index as f64,
            "previous tick index with an available successor",
        )
    })?;

    if next.index != expected_next {
        return Err(out_of_domain(
            "next_tick_index",
            next.index as f64,
            "exactly previous_tick_index + 1",
        ));
    }

    Ok(EngineeringResult::new(
        BioSimTickAdvance {
            previous_index: previous.index,
            next_index: next.index,
            next_duration_seconds: next.duration_seconds,
        },
        biosim_tick_validation_codex_id(),
        resource_tick_record(biosim_tick_validation_codex_id()),
    )
    .with_assumption(
        "biosim_rs.tick_consecutive_index",
        "accepted transitions advance one discrete tick index at a time",
    )
    .with_assumption(
        "biosim_rs.no_transaction_commit",
        "Chunk 6A records tick ordering only; atomic transaction commit is deferred to Chunk 6B",
    )
    .with_validity(ValidityStatus::WithinDocumentedDomain))
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_core::{ValidityStatus, VerificationStatus};

    #[test]
    fn resource_identity_catalog_rejects_duplicates() {
        let duplicate = [BioSimResourceKind::OxygenGas, BioSimResourceKind::OxygenGas];

        let err = validate_biosim_resource_catalog(&duplicate).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("resource_catalog"));
    }

    #[test]
    fn resource_identity_catalog_accepts_unique_clean_room_resources() {
        let catalog = [
            BioSimResourceKind::OxygenGas,
            BioSimResourceKind::CarbonDioxideGas,
            BioSimResourceKind::PotableWater,
        ];

        let result = validate_biosim_resource_catalog(&catalog).unwrap();

        assert_eq!(result.value, 3);
        assert_eq!(result.codex_id, biosim_resource_catalog_codex_id());
        assert_eq!(
            result.verification_status(),
            VerificationStatus::ResearchRequired
        );
        assert_eq!(result.validity, ValidityStatus::WithinDocumentedDomain);
        assert!(result
            .assumptions
            .iter()
            .any(|item| item.id == "biosim_rs.clean_room_chunk6a"));
        assert_eq!(
            biosim_resource_identity(BioSimResourceKind::OxygenGas).canonical_id,
            "biosim_rs.resource.oxygen_gas"
        );
    }

    #[test]
    fn tick_validation_rejects_nonpositive_duration() {
        let err = validate_biosim_tick(0, 0.0).unwrap_err();

        assert_eq!(err.code(), "non_positive_input");
        assert_eq!(err.parameter(), Some("tick_duration_seconds"));
    }

    #[test]
    fn tick_validation_marks_zero_as_boundary_case() {
        let result = validate_biosim_tick(0, 60.0).unwrap();

        assert_eq!(result.value.index, 0);
        assert_eq!(result.value.duration_seconds, 60.0);
        assert_eq!(result.codex_id, biosim_tick_validation_codex_id());
        assert_eq!(result.validity, ValidityStatus::BoundaryCase);
        assert!(result.has_warnings());
    }

    #[test]
    fn tick_advance_requires_consecutive_ticks() {
        let previous = validate_biosim_tick(3, 60.0).unwrap().value;
        let next = validate_biosim_tick(5, 60.0).unwrap().value;

        let err = validate_biosim_tick_advance(previous, next).unwrap_err();

        assert_eq!(err.code(), "out_of_domain");
        assert_eq!(err.parameter(), Some("next_tick_index"));
    }

    #[test]
    fn tick_advance_accepts_consecutive_ticks_with_positive_duration() {
        let previous = validate_biosim_tick(3, 60.0).unwrap().value;
        let next = validate_biosim_tick(4, 30.0).unwrap().value;

        let result = validate_biosim_tick_advance(previous, next).unwrap();

        assert_eq!(result.value.previous_index, 3);
        assert_eq!(result.value.next_index, 4);
        assert_eq!(result.value.next_duration_seconds, 30.0);
        assert_eq!(result.validity, ValidityStatus::WithinDocumentedDomain);
        assert_eq!(
            result.verification_status(),
            VerificationStatus::ResearchRequired
        );
    }
}
