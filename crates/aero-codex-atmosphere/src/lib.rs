#![forbid(unsafe_code)]
//! Phase 0.001 atmosphere equations.
//!
//! The troposphere functions use a simplified standard-atmosphere lapse-rate
//! model over geometric altitudes from 0 m through 11,000 m. Phase 0.001 treats
//! the supplied geometric altitude directly as the standard-atmosphere altitude
//! variable; explicit geometric/geopotential altitude conversion is intentionally
//! deferred and must be added before higher-fidelity atmosphere use.
//!
//! All source traceability in this crate remains conservative. The equations are
//! implemented and documented as research scaffolding, but the associated source
//! registry and validation cards remain `research_required` until exact source
//! editions, equation/table provenance, and tolerances are reviewed.

use aero_codex_constants::{
    SOURCE_ID_US_STANDARD_ATMOSPHERE_1976, STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K,
    STANDARD_GAMMA_DRY_AIR, STANDARD_GRAVITY_M_S2, STANDARD_SEA_LEVEL_DENSITY_KG_M3,
    STANDARD_SEA_LEVEL_PRESSURE_PA, STANDARD_SEA_LEVEL_TEMPERATURE_K,
};
use aero_codex_core::{validation, AeroError, AeroResult, VerificationRecord, VerificationStatus};

/// Codex identifier for the standard sea-level state helper.
pub const CODEX_ID_STANDARD_SEA_LEVEL: &str = "atmosphere.standard.sea_level";
/// Codex identifier for the troposphere temperature relation.
pub const CODEX_ID_TROPOSPHERE_TEMPERATURE: &str = "atmosphere.standard.troposphere.temperature";
/// Codex identifier for the troposphere pressure relation.
pub const CODEX_ID_TROPOSPHERE_PRESSURE: &str = "atmosphere.standard.troposphere.pressure";
/// Codex identifier for the troposphere density relation.
pub const CODEX_ID_TROPOSPHERE_DENSITY: &str = "atmosphere.standard.troposphere.density";
/// Codex identifier for the perfect-gas speed-of-sound relation.
pub const CODEX_ID_SPEED_OF_SOUND: &str = "atmosphere.speed_of_sound";

/// Minimum altitude accepted by the Phase 0.001 troposphere model, m.
pub const TROPOSPHERE_MIN_ALTITUDE_M: f64 = 0.0;
/// Maximum altitude accepted by the Phase 0.001 troposphere model, m.
pub const TROPOSPHERE_MAX_ALTITUDE_M: f64 = 11_000.0;
/// Constant lapse rate used by the Phase 0.001 troposphere model, K/m.
pub const TROPOSPHERE_LAPSE_RATE_K_PER_M: f64 = -0.0065;

const ATMOSPHERE_SOURCES: &[&str] = &[SOURCE_ID_US_STANDARD_ATMOSPHERE_1976];

/// Thermodynamic state returned by the atmosphere helpers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtmosphereState {
    /// Static temperature, K.
    pub temperature_k: f64,
    /// Static pressure, Pa.
    pub pressure_pa: f64,
    /// Static density, kg/m³.
    pub density_kg_m3: f64,
    /// Perfect-gas speed of sound, m/s.
    pub speed_of_sound_m_s: f64,
}

/// Conservative traceability metadata for a Phase 0.001 atmosphere equation.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_STANDARD_SEA_LEVEL => Some(VerificationRecord::new(
            CODEX_ID_STANDARD_SEA_LEVEL,
            VerificationStatus::ResearchRequired,
            ATMOSPHERE_SOURCES,
            "Sea-level seed values only; exact U.S. Standard Atmosphere source review pending.",
        )),
        CODEX_ID_TROPOSPHERE_TEMPERATURE => Some(VerificationRecord::new(
            CODEX_ID_TROPOSPHERE_TEMPERATURE,
            VerificationStatus::ResearchRequired,
            ATMOSPHERE_SOURCES,
            "Simplified constant-lapse-rate troposphere temperature relation; provenance review pending.",
        )),
        CODEX_ID_TROPOSPHERE_PRESSURE => Some(VerificationRecord::new(
            CODEX_ID_TROPOSPHERE_PRESSURE,
            VerificationStatus::ResearchRequired,
            ATMOSPHERE_SOURCES,
            "Simplified constant-lapse-rate troposphere pressure relation; provenance review pending.",
        )),
        CODEX_ID_TROPOSPHERE_DENSITY => Some(VerificationRecord::new(
            CODEX_ID_TROPOSPHERE_DENSITY,
            VerificationStatus::ResearchRequired,
            ATMOSPHERE_SOURCES,
            "Troposphere density from pressure, temperature, and dry-air gas constant; provenance review pending.",
        )),
        CODEX_ID_SPEED_OF_SOUND => Some(VerificationRecord::new(
            CODEX_ID_SPEED_OF_SOUND,
            VerificationStatus::ResearchRequired,
            ATMOSPHERE_SOURCES,
            "Perfect-gas speed of sound with constant gamma; source and assumption review pending.",
        )),
        _ => None,
    }
}

/// Return the Phase 0.001 standard sea-level seed state.
///
/// This helper returns constants and derived speed of sound. It does not imply
/// external validation beyond the `research_required` source registry seed.
#[must_use]
pub fn standard_sea_level() -> AtmosphereState {
    AtmosphereState {
        temperature_k: STANDARD_SEA_LEVEL_TEMPERATURE_K,
        pressure_pa: STANDARD_SEA_LEVEL_PRESSURE_PA,
        density_kg_m3: STANDARD_SEA_LEVEL_DENSITY_KG_M3,
        speed_of_sound_m_s: speed_of_sound(
            STANDARD_GAMMA_DRY_AIR,
            STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K,
            STANDARD_SEA_LEVEL_TEMPERATURE_K,
        )
        .expect("standard sea-level constants are positive and finite"),
    }
}

fn validate_troposphere_altitude(altitude_m: f64) -> AeroResult<()> {
    validation::ensure_finite("altitude_m", altitude_m)?;
    if altitude_m < TROPOSPHERE_MIN_ALTITUDE_M {
        return Err(AeroError::NegativeInput {
            parameter: "altitude_m",
            value: altitude_m,
        });
    }
    if altitude_m > TROPOSPHERE_MAX_ALTITUDE_M {
        return Err(AeroError::OutOfDomain {
            parameter: "altitude_m",
            value: altitude_m,
            expected: "0 m <= geometric altitude <= 11000 m for Phase 0.001 troposphere model",
        });
    }
    Ok(())
}

/// Static temperature in the simplified Phase 0.001 troposphere model, K.
pub fn troposphere_temperature(altitude_m: f64) -> AeroResult<f64> {
    validate_troposphere_altitude(altitude_m)?;
    Ok(STANDARD_SEA_LEVEL_TEMPERATURE_K + TROPOSPHERE_LAPSE_RATE_K_PER_M * altitude_m)
}

/// Static pressure in the simplified Phase 0.001 troposphere model, Pa.
pub fn troposphere_pressure(altitude_m: f64) -> AeroResult<f64> {
    validate_troposphere_altitude(altitude_m)?;
    let temperature_k = troposphere_temperature(altitude_m)?;
    let exponent = -STANDARD_GRAVITY_M_S2
        / (TROPOSPHERE_LAPSE_RATE_K_PER_M * STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K);
    Ok(STANDARD_SEA_LEVEL_PRESSURE_PA
        * (temperature_k / STANDARD_SEA_LEVEL_TEMPERATURE_K).powf(exponent))
}

/// Static density in the simplified Phase 0.001 troposphere model, kg/m³.
pub fn troposphere_density(altitude_m: f64) -> AeroResult<f64> {
    validate_troposphere_altitude(altitude_m)?;
    let pressure_pa = troposphere_pressure(altitude_m)?;
    let temperature_k = troposphere_temperature(altitude_m)?;
    Ok(pressure_pa / (STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K * temperature_k))
}

/// Complete atmosphere state in the simplified Phase 0.001 troposphere model.
pub fn troposphere_state(altitude_m: f64) -> AeroResult<AtmosphereState> {
    let temperature_k = troposphere_temperature(altitude_m)?;
    let pressure_pa = troposphere_pressure(altitude_m)?;
    let density_kg_m3 = troposphere_density(altitude_m)?;
    let speed_of_sound_m_s = speed_of_sound(
        STANDARD_GAMMA_DRY_AIR,
        STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K,
        temperature_k,
    )?;
    Ok(AtmosphereState {
        temperature_k,
        pressure_pa,
        density_kg_m3,
        speed_of_sound_m_s,
    })
}

/// Perfect-gas speed of sound, `sqrt(gamma * R * T)`, m/s.
pub fn speed_of_sound(gamma: f64, gas_constant: f64, temperature: f64) -> AeroResult<f64> {
    validation::ensure_greater_than("gamma", gamma, 1.0)?;
    validation::ensure_positive("gas_constant", gas_constant)?;
    validation::ensure_positive("temperature", temperature)?;
    Ok((gamma * gas_constant * temperature).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} !~= {b}");
    }

    #[test]
    fn sea_level_matches_constants() {
        let state = standard_sea_level();
        approx(
            state.temperature_k,
            STANDARD_SEA_LEVEL_TEMPERATURE_K,
            1.0e-12,
        );
        approx(state.pressure_pa, STANDARD_SEA_LEVEL_PRESSURE_PA, 1.0e-12);
        approx(
            state.density_kg_m3,
            STANDARD_SEA_LEVEL_DENSITY_KG_M3,
            1.0e-12,
        );
        assert!(state.speed_of_sound_m_s > 0.0);
    }

    #[test]
    fn troposphere_zero_altitude_matches_sea_level_seeds() {
        approx(
            troposphere_temperature(0.0).unwrap(),
            STANDARD_SEA_LEVEL_TEMPERATURE_K,
            1.0e-12,
        );
        approx(
            troposphere_pressure(0.0).unwrap(),
            STANDARD_SEA_LEVEL_PRESSURE_PA,
            1.0e-9,
        );
        approx(
            troposphere_density(0.0).unwrap(),
            STANDARD_SEA_LEVEL_DENSITY_KG_M3,
            1.0e-6,
        );
    }

    #[test]
    fn troposphere_properties_decrease_with_altitude() {
        assert!(troposphere_temperature(1_000.0).unwrap() < troposphere_temperature(0.0).unwrap());
        assert!(troposphere_pressure(1_000.0).unwrap() < troposphere_pressure(0.0).unwrap());
        assert!(troposphere_density(1_000.0).unwrap() < troposphere_density(0.0).unwrap());
    }

    #[test]
    fn troposphere_upper_boundary_is_accepted() {
        approx(
            troposphere_temperature(TROPOSPHERE_MAX_ALTITUDE_M).unwrap(),
            216.65,
            1.0e-12,
        );
        assert!(troposphere_pressure(TROPOSPHERE_MAX_ALTITUDE_M).unwrap() > 0.0);
        assert!(troposphere_density(TROPOSPHERE_MAX_ALTITUDE_M).unwrap() > 0.0);
    }

    #[test]
    fn troposphere_state_collects_all_properties() {
        let state = troposphere_state(2_500.0).unwrap();
        approx(
            state.temperature_k,
            troposphere_temperature(2_500.0).unwrap(),
            1.0e-12,
        );
        approx(
            state.pressure_pa,
            troposphere_pressure(2_500.0).unwrap(),
            1.0e-9,
        );
        approx(
            state.density_kg_m3,
            troposphere_density(2_500.0).unwrap(),
            1.0e-12,
        );
        assert!(state.speed_of_sound_m_s > 0.0);
    }

    #[test]
    fn speed_of_sound_positive() {
        assert!(speed_of_sound(1.4, 287.0, 288.15).unwrap() > 0.0);
    }

    #[test]
    fn invalid_altitudes_are_rejected() {
        assert!(matches!(
            troposphere_temperature(-1.0),
            Err(AeroError::NegativeInput {
                parameter: "altitude_m",
                ..
            })
        ));
        assert!(matches!(
            troposphere_temperature(20_000.0),
            Err(AeroError::OutOfDomain {
                parameter: "altitude_m",
                ..
            })
        ));
        assert!(troposphere_temperature(f64::NAN).is_err());
        assert!(troposphere_pressure(f64::INFINITY).is_err());
    }

    #[test]
    fn invalid_speed_of_sound_inputs_are_rejected() {
        assert!(speed_of_sound(1.0, 287.0, 288.15).is_err());
        assert!(speed_of_sound(1.4, 0.0, 288.15).is_err());
        assert!(speed_of_sound(1.4, 287.0, 0.0).is_err());
        assert!(speed_of_sound(f64::NAN, 287.0, 288.15).is_err());
    }

    #[test]
    fn atmosphere_verification_records_remain_research_required() {
        let record = verification_record(CODEX_ID_TROPOSPHERE_PRESSURE).unwrap();
        assert_eq!(record.codex_id, CODEX_ID_TROPOSPHERE_PRESSURE);
        assert_eq!(record.status, VerificationStatus::ResearchRequired);
        assert_eq!(record.sources, ATMOSPHERE_SOURCES);
        assert!(verification_record("atmosphere.unknown").is_none());
    }
}
