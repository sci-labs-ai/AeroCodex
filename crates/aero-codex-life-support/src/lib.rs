#![forbid(unsafe_code)]
//! Bio-regenerative life-support mass-balance primitives for Phase 0.001.
//!
//! This crate implements scalar bookkeeping helpers for closure fraction,
//! production area, buffer residence time, crew daily requirements, and simple
//! oxygen/carbon-dioxide/water balances. Inputs and outputs are unit-agnostic
//! scalar rates or daily quantities unless the function name states otherwise;
//! callers must keep units consistent.
//!
//! Phase 0.001 deliberately does not model biology, crop growth dynamics,
//! microbial kinetics, crew metabolic variability, habitat safety margins,
//! trace-contaminant control, humidity control, storage sizing, reliability,
//! controls, emergency modes, human health, certification, flight readiness, or
//! mission readiness. Traceability metadata remains conservative
//! `research_required` until exact BVAD/ECLSS source editions, equations,
//! tables, representative examples, and tolerances are reviewed.

use aero_codex_core::{
    validation, AeroError, AeroResult, EngineeringResult, ValidityStatus, VerificationRecord,
};

/// Codex identifier for closure fraction, `recycled / required`.
pub const CODEX_ID_CLOSURE_FRACTION: &str = "life_support.bioregenerative.closure_fraction";
/// Codex identifier for production area, `required_mass_per_day / productivity_per_area_per_day`.
pub const CODEX_ID_REQUIRED_PRODUCTION_AREA: &str =
    "life_support.bioregenerative.required_production_area";
/// Codex identifier for residence time, `buffer_mass / flow_rate`.
pub const CODEX_ID_BUFFER_RESIDENCE_TIME: &str =
    "life_support.bioregenerative.buffer_residence_time";
/// Codex identifier for crew requirement, `crew_count * per_crew_per_day`.
pub const CODEX_ID_CREW_DAILY_REQUIREMENT: &str =
    "life_support.bioregenerative.crew_daily_requirement";
/// Codex identifier for generic daily net balance, `production - consumption`.
pub const CODEX_ID_NET_DAILY_BALANCE: &str = "life_support.bioregenerative.net_daily_balance";
/// Codex identifier for oxygen daily balance, `production - consumption`.
pub const CODEX_ID_OXYGEN_DAILY_BALANCE: &str = "life_support.bioregenerative.oxygen_daily_balance";
/// Backward-compatible alias for oxygen daily balance metadata.
pub const CODEX_ID_OXYGEN_BALANCE: &str = CODEX_ID_OXYGEN_DAILY_BALANCE;
/// Codex identifier for carbon-dioxide daily balance, `removal - generation`.
pub const CODEX_ID_CARBON_DIOXIDE_DAILY_BALANCE: &str =
    "life_support.bioregenerative.carbon_dioxide_daily_balance";
/// Backward-compatible alias for carbon-dioxide daily balance metadata.
pub const CODEX_ID_CARBON_DIOXIDE_BALANCE: &str = CODEX_ID_CARBON_DIOXIDE_DAILY_BALANCE;
/// Codex identifier for water recovery fraction, `recovered / required`.
pub const CODEX_ID_WATER_RECOVERY_BALANCE: &str =
    "life_support.bioregenerative.water_recovery_balance";

/// Conservative NASA BVAD/ECLSS source-registry seed retained for later source review.
pub const SOURCE_ID_LIFE_SUPPORT_NASA_BVAD_ECLSS: &str =
    "source.life_support.nasa_bvad_eclss.research_required";
/// Backward-compatible alias for the NASA BVAD/ECLSS source seed.
pub const SOURCE_ID_LIFE_SUPPORT_BVAD_ECLSS: &str = SOURCE_ID_LIFE_SUPPORT_NASA_BVAD_ECLSS;
/// Conservative source-registry seed for generic bio-regenerative mass-balance review.
pub const SOURCE_ID_LIFE_SUPPORT_BIOREGENERATIVE_MASS_BALANCE: &str =
    "source.life_support.bioregenerative_mass_balance.research_required";
/// Short alias for the life-support source-registry seed.
pub const SOURCE_ID_LIFE_SUPPORT_BASICS: &str = SOURCE_ID_LIFE_SUPPORT_BIOREGENERATIVE_MASS_BALANCE;

const LIFE_SUPPORT_SOURCES: &[&str] = &[
    SOURCE_ID_LIFE_SUPPORT_BIOREGENERATIVE_MASS_BALANCE,
    SOURCE_ID_LIFE_SUPPORT_NASA_BVAD_ECLSS,
];

/// Conservative traceability metadata for Phase 0.001 life-support helpers.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_CLOSURE_FRACTION => Some(VerificationRecord::research_required(
            CODEX_ID_CLOSURE_FRACTION,
            LIFE_SUPPORT_SOURCES,
            "Closure fraction bookkeeping implemented; exact BVAD/ECLSS boundary definition, source examples, and tolerances pending.",
        )),
        CODEX_ID_REQUIRED_PRODUCTION_AREA => Some(VerificationRecord::research_required(
            CODEX_ID_REQUIRED_PRODUCTION_AREA,
            LIFE_SUPPORT_SOURCES,
            "Production-area bookkeeping implemented; crop productivity source, units, operating assumptions, and examples pending.",
        )),
        CODEX_ID_BUFFER_RESIDENCE_TIME => Some(VerificationRecord::research_required(
            CODEX_ID_BUFFER_RESIDENCE_TIME,
            LIFE_SUPPORT_SOURCES,
            "Buffer residence-time bookkeeping implemented; buffer definition, flow convention, and reference examples pending.",
        )),
        CODEX_ID_CREW_DAILY_REQUIREMENT => Some(VerificationRecord::research_required(
            CODEX_ID_CREW_DAILY_REQUIREMENT,
            LIFE_SUPPORT_SOURCES,
            "Crew daily requirement bookkeeping implemented; crew metabolic assumptions, units, and source examples pending.",
        )),
        CODEX_ID_NET_DAILY_BALANCE => Some(VerificationRecord::research_required(
            CODEX_ID_NET_DAILY_BALANCE,
            LIFE_SUPPORT_SOURCES,
            "Generic daily mass-balance bookkeeping implemented; sign convention and source examples pending.",
        )),
        CODEX_ID_OXYGEN_DAILY_BALANCE => Some(VerificationRecord::research_required(
            CODEX_ID_OXYGEN_DAILY_BALANCE,
            LIFE_SUPPORT_SOURCES,
            "Oxygen production-minus-consumption bookkeeping implemented; metabolic and bioregenerative source assumptions pending.",
        )),
        CODEX_ID_CARBON_DIOXIDE_DAILY_BALANCE => Some(VerificationRecord::research_required(
            CODEX_ID_CARBON_DIOXIDE_DAILY_BALANCE,
            LIFE_SUPPORT_SOURCES,
            "Carbon-dioxide removal-minus-generation bookkeeping implemented; stoichiometric and ECLSS boundary assumptions pending.",
        )),
        CODEX_ID_WATER_RECOVERY_BALANCE => Some(VerificationRecord::research_required(
            CODEX_ID_WATER_RECOVERY_BALANCE,
            LIFE_SUPPORT_SOURCES,
            "Water recovery closure bookkeeping implemented; water accounting boundary and BVAD/ECLSS source examples pending.",
        )),
        _ => None,
    }
}

fn numerical_failure(codex_id: &'static str, reason: &'static str) -> AeroError {
    AeroError::NumericalFailure {
        solver: codex_id,
        reason,
    }
}

fn ensure_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed life-support result was not finite",
        ))
    }
}

fn ensure_nonnegative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed life-support result was not nonnegative and finite",
        ))
    }
}

fn checked_product(
    codex_id: &'static str,
    label: &'static str,
    left: f64,
    right: f64,
) -> AeroResult<f64> {
    let value = left * right;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(numerical_failure(codex_id, label))
    }
}

fn record_for(codex_id: &'static str) -> VerificationRecord {
    verification_record(codex_id).unwrap_or_else(|| {
        VerificationRecord::research_required(
            codex_id,
            LIFE_SUPPORT_SOURCES,
            "Life-support helper is present but has no upgraded source-validation status.",
        )
    })
}

fn closure_fraction_with_codex(
    codex_id: &'static str,
    recycled_mass_rate: f64,
    total_required_mass_rate: f64,
    warning_code: &'static str,
    assumption_id: &'static str,
    assumption_description: &'static str,
) -> AeroResult<EngineeringResult<f64>> {
    validation::ensure_nonnegative("recycled_mass_rate", recycled_mass_rate)?;
    validation::ensure_positive("total_required_mass_rate", total_required_mass_rate)?;

    let value =
        ensure_nonnegative_finite_result(codex_id, recycled_mass_rate / total_required_mass_rate)?;
    let validity = if value > 1.0 {
        ValidityStatus::OutsideDocumentedDomain
    } else if value == 0.0 || (value - 1.0).abs() <= f64::EPSILON {
        ValidityStatus::BoundaryCase
    } else {
        ValidityStatus::WithinDocumentedDomain
    };

    let mut result = EngineeringResult::new(value, codex_id, record_for(codex_id))
        .with_assumption(
            "life_support.mass_balance",
            "steady daily or rate-normalized mass balance with consistent units",
        )
        .with_assumption(assumption_id, assumption_description)
        .with_validity(validity);

    if value > 1.0 {
        result = result.with_warning(
            warning_code,
            "closure fraction exceeds 1; verify accounting boundary, storage, and overproduction assumptions",
        );
    }

    Ok(result)
}

/// Closure fraction, `recycled_mass_rate / total_required_mass_rate`.
///
/// Inputs must be finite and use consistent units. `recycled_mass_rate` may be
/// zero; `total_required_mass_rate` must be strictly positive. Values above 1
/// are returned with a warning because they may represent overproduction,
/// storage drawdown, or a mismatched accounting boundary rather than a simple
/// closure fraction inside a reviewed ECLSS boundary.
pub fn closure_fraction(
    recycled_mass_rate: f64,
    total_required_mass_rate: f64,
) -> AeroResult<EngineeringResult<f64>> {
    closure_fraction_with_codex(
        CODEX_ID_CLOSURE_FRACTION,
        recycled_mass_rate,
        total_required_mass_rate,
        "closure_fraction.gt_one",
        "life_support.simple_closure",
        "closure equals recycled rate divided by required rate",
    )
}

/// Required production area, `required_mass_per_day / productivity_per_area_per_day`.
///
/// `required_mass_per_day` must be finite and nonnegative. Productivity must be
/// finite and strictly positive. The function assumes constant productivity and
/// consistent mass/time/area units; crop biology, lighting, harvest index,
/// downtime, storage, and safety margins are outside Phase 0.001.
pub fn required_production_area(
    required_mass_per_day: f64,
    productivity_per_area_per_day: f64,
) -> AeroResult<f64> {
    validation::ensure_nonnegative("required_mass_per_day", required_mass_per_day)?;
    validation::ensure_positive(
        "productivity_per_area_per_day",
        productivity_per_area_per_day,
    )?;

    ensure_nonnegative_finite_result(
        CODEX_ID_REQUIRED_PRODUCTION_AREA,
        required_mass_per_day / productivity_per_area_per_day,
    )
}

/// Buffer residence time, `buffer_mass / flow_rate`.
///
/// `buffer_mass` may be zero; `flow_rate` must be strictly positive. The result
/// has the time unit implied by the caller's buffer and flow-rate units. Mixing,
/// control dynamics, reserves, and contingency sizing are outside this helper.
pub fn buffer_residence_time(buffer_mass: f64, flow_rate: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("buffer_mass", buffer_mass)?;
    validation::ensure_positive("flow_rate", flow_rate)?;

    ensure_nonnegative_finite_result(CODEX_ID_BUFFER_RESIDENCE_TIME, buffer_mass / flow_rate)
}

/// Crew daily requirement, `crew_count * per_crew_per_day`.
///
/// `crew_count` is an unsigned count. `per_crew_per_day` must be finite and
/// nonnegative. Crew demographics, metabolic variability, activity schedules,
/// and medical limits are outside Phase 0.001.
pub fn crew_daily_requirement(crew_count: u32, per_crew_per_day: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("per_crew_per_day", per_crew_per_day)?;
    checked_product(
        CODEX_ID_CREW_DAILY_REQUIREMENT,
        "crew daily requirement multiplication was not finite",
        f64::from(crew_count),
        per_crew_per_day,
    )
}

/// Generic daily net balance, `production_per_day - consumption_per_day`.
///
/// Both inputs must be finite and nonnegative. Positive output means production
/// exceeds consumption in the caller-defined accounting boundary; negative
/// output means a net deficit. Storage dynamics and controller behavior are not
/// modeled.
pub fn net_daily_balance(production_per_day: f64, consumption_per_day: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("production_per_day", production_per_day)?;
    validation::ensure_nonnegative("consumption_per_day", consumption_per_day)?;

    ensure_finite_result(
        CODEX_ID_NET_DAILY_BALANCE,
        production_per_day - consumption_per_day,
    )
}

/// Oxygen daily balance, `oxygen_production_per_day - oxygen_consumption_per_day`.
///
/// This is a sign-convention wrapper around [`net_daily_balance`]. It does not
/// include metabolic models, photosynthesis source terms, storage constraints,
/// or medical atmosphere-control limits.
pub fn oxygen_balance(
    oxygen_production_per_day: f64,
    oxygen_consumption_per_day: f64,
) -> AeroResult<f64> {
    validation::ensure_nonnegative("oxygen_production_per_day", oxygen_production_per_day)?;
    validation::ensure_nonnegative("oxygen_consumption_per_day", oxygen_consumption_per_day)?;
    ensure_finite_result(
        CODEX_ID_OXYGEN_DAILY_BALANCE,
        oxygen_production_per_day - oxygen_consumption_per_day,
    )
}

/// Carbon-dioxide daily balance, `co2_removal_per_day - co2_generation_per_day`.
///
/// Positive output means removal exceeds generation inside the caller-defined
/// accounting boundary. This helper does not model cabin atmosphere dynamics,
/// crew metabolism, crop uptake, sorbents, or toxicity/safety limits.
pub fn carbon_dioxide_balance(
    co2_generation_per_day: f64,
    co2_removal_per_day: f64,
) -> AeroResult<f64> {
    validation::ensure_nonnegative("co2_generation_per_day", co2_generation_per_day)?;
    validation::ensure_nonnegative("co2_removal_per_day", co2_removal_per_day)?;
    ensure_finite_result(
        CODEX_ID_CARBON_DIOXIDE_DAILY_BALANCE,
        co2_removal_per_day - co2_generation_per_day,
    )
}

/// Water recovery fraction, `recovered_water_per_day / required_water_per_day`.
///
/// This uses closure-fraction semantics for water-specific bookkeeping. Values
/// above 1 are returned with a warning and `OutsideDocumentedDomain` validity metadata.
pub fn water_recovery_balance(
    recovered_water_per_day: f64,
    required_water_per_day: f64,
) -> AeroResult<EngineeringResult<f64>> {
    closure_fraction_with_codex(
        CODEX_ID_WATER_RECOVERY_BALANCE,
        recovered_water_per_day,
        required_water_per_day,
        "water_recovery_balance.gt_one",
        "life_support.water_recovery",
        "water recovery fraction equals recovered water divided by required water",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_core::{AeroError, VerificationStatus};

    #[test]
    fn closure_fraction_is_recycled_over_required() {
        let result = closure_fraction(3.0, 4.0).unwrap();
        assert!((result.value - 0.75).abs() < 1.0e-12);
        assert_eq!(result.codex_id, CODEX_ID_CLOSURE_FRACTION);
        assert_eq!(result.validity, ValidityStatus::WithinDocumentedDomain);
        assert_eq!(
            result.verification.status,
            VerificationStatus::ResearchRequired
        );
        assert!(result.verification.has_sources());
    }

    #[test]
    fn closure_fraction_allows_overproduction_with_warning() {
        let result = closure_fraction(5.0, 4.0).unwrap();
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == "closure_fraction.gt_one"));
        assert_eq!(result.validity, ValidityStatus::OutsideDocumentedDomain);
    }

    #[test]
    fn closure_fraction_zero_and_one_are_boundary_cases() {
        assert_eq!(
            closure_fraction(0.0, 4.0).unwrap().validity,
            ValidityStatus::BoundaryCase
        );
        assert_eq!(
            closure_fraction(4.0, 4.0).unwrap().validity,
            ValidityStatus::BoundaryCase
        );
    }

    #[test]
    fn closure_fraction_rejects_invalid_inputs() {
        assert!(closure_fraction(-1.0, 1.0).is_err());
        assert!(closure_fraction(1.0, 0.0).is_err());
        assert!(closure_fraction(f64::NAN, 1.0).is_err());
        assert!(closure_fraction(1.0, f64::INFINITY).is_err());
    }

    #[test]
    fn required_area_decreases_as_productivity_increases() {
        let low_productivity = required_production_area(10.0, 2.0).unwrap();
        let high_productivity = required_production_area(10.0, 5.0).unwrap();
        assert!(high_productivity < low_productivity);
        assert_eq!(required_production_area(0.0, 5.0).unwrap(), 0.0);
    }

    #[test]
    fn required_area_rejects_invalid_inputs_and_nonfinite_outputs() {
        assert!(required_production_area(-1.0, 1.0).is_err());
        assert!(required_production_area(1.0, 0.0).is_err());
        assert!(required_production_area(1.0, f64::NAN).is_err());
        assert!(matches!(
            required_production_area(f64::MAX, f64::MIN_POSITIVE),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn residence_time_is_buffer_over_flow() {
        assert!((buffer_residence_time(100.0, 4.0).unwrap() - 25.0).abs() < 1.0e-12);
        assert_eq!(buffer_residence_time(0.0, 4.0).unwrap(), 0.0);
    }

    #[test]
    fn residence_time_rejects_invalid_inputs_and_nonfinite_outputs() {
        assert!(buffer_residence_time(-1.0, 1.0).is_err());
        assert!(buffer_residence_time(1.0, 0.0).is_err());
        assert!(buffer_residence_time(f64::INFINITY, 1.0).is_err());
        assert!(matches!(
            buffer_residence_time(f64::MAX, f64::MIN_POSITIVE),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn crew_requirement_scales_linearly() {
        assert!((crew_daily_requirement(4, 2.5).unwrap() - 10.0).abs() < 1.0e-12);
        assert_eq!(crew_daily_requirement(0, 2.5).unwrap(), 0.0);
        assert_eq!(crew_daily_requirement(4, 0.0).unwrap(), 0.0);
    }

    #[test]
    fn crew_requirement_rejects_invalid_inputs_and_nonfinite_outputs() {
        assert!(crew_daily_requirement(4, -1.0).is_err());
        assert!(crew_daily_requirement(4, f64::NAN).is_err());
        assert!(matches!(
            crew_daily_requirement(u32::MAX, f64::MAX),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn net_daily_balance_sign_is_correct() {
        assert!(net_daily_balance(12.0, 10.0).unwrap() > 0.0);
        assert!(net_daily_balance(8.0, 10.0).unwrap() < 0.0);
        assert_eq!(net_daily_balance(10.0, 10.0).unwrap(), 0.0);
    }

    #[test]
    fn oxygen_and_carbon_dioxide_balance_sign_conventions() {
        assert!(oxygen_balance(12.0, 10.0).unwrap() > 0.0);
        assert!(oxygen_balance(8.0, 10.0).unwrap() < 0.0);
        assert!(carbon_dioxide_balance(12.0, 10.0).unwrap() < 0.0);
        assert!(carbon_dioxide_balance(8.0, 10.0).unwrap() > 0.0);
    }

    #[test]
    fn balances_reject_negative_and_nonfinite_inputs() {
        assert!(net_daily_balance(-1.0, 0.0).is_err());
        assert!(net_daily_balance(0.0, -1.0).is_err());
        assert!(oxygen_balance(f64::NAN, 0.0).is_err());
        assert!(carbon_dioxide_balance(0.0, f64::INFINITY).is_err());
    }

    #[test]
    fn water_recovery_uses_water_specific_codex_id() {
        let result = water_recovery_balance(7.0, 10.0).unwrap();
        assert!((result.value - 0.7).abs() < 1.0e-12);
        assert_eq!(result.codex_id, CODEX_ID_WATER_RECOVERY_BALANCE);
        assert_eq!(result.validity, ValidityStatus::WithinDocumentedDomain);
    }

    #[test]
    fn water_recovery_allows_over_recovery_with_warning() {
        let result = water_recovery_balance(11.0, 10.0).unwrap();
        assert!(result
            .warnings
            .iter()
            .any(|w| w.code == "water_recovery_balance.gt_one"));
        assert_eq!(result.validity, ValidityStatus::OutsideDocumentedDomain);
    }

    #[test]
    fn verification_records_cover_life_support_helpers() {
        let codex_ids = [
            CODEX_ID_CLOSURE_FRACTION,
            CODEX_ID_REQUIRED_PRODUCTION_AREA,
            CODEX_ID_BUFFER_RESIDENCE_TIME,
            CODEX_ID_CREW_DAILY_REQUIREMENT,
            CODEX_ID_NET_DAILY_BALANCE,
            CODEX_ID_OXYGEN_DAILY_BALANCE,
            CODEX_ID_CARBON_DIOXIDE_DAILY_BALANCE,
            CODEX_ID_WATER_RECOVERY_BALANCE,
        ];

        for codex_id in codex_ids {
            let record = verification_record(codex_id).unwrap();
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert!(record
                .sources
                .contains(&SOURCE_ID_LIFE_SUPPORT_BIOREGENERATIVE_MASS_BALANCE));
            assert!(record
                .sources
                .contains(&SOURCE_ID_LIFE_SUPPORT_NASA_BVAD_ECLSS));
        }

        assert!(verification_record("life_support.unknown").is_none());
    }
}
