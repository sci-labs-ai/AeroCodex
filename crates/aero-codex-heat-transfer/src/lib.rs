#![forbid(unsafe_code)]
//! Phase 0.001 heat-transfer primitives.
//!
//! This crate implements scalar preliminary-design heat-transfer helpers for
//! Stefan-Boltzmann net radiative flux, convective heat flux, one-dimensional
//! conduction resistance, and conduction heat rate from a supplied thermal
//! resistance. Dimensional inputs and outputs use SI units.
//!
//! Phase 0.001 deliberately does not model view factors, spectral radiation,
//! participating media, contact resistance, transient thermal capacitance,
//! temperature-dependent material properties, multilayer networks, ablation,
//! radiative-convective coupling, or CFD/thermal-solver workflows. Traceability
//! metadata remains conservative `research_required` until exact source
//! editions, equation identifiers, sign conventions, reference examples, and
//! tolerances are reviewed.

use aero_codex_core::{validation, AeroError, AeroResult, VerificationRecord};

/// Codex identifier for net Stefan-Boltzmann radiative heat flux.
pub const CODEX_ID_STEFAN_BOLTZMANN_RADIATIVE_FLUX: &str =
    "heat_transfer.radiation.stefan_boltzmann_flux";
/// Short alias for the Stefan-Boltzmann radiative-flux Codex ID.
pub const CODEX_ID_RADIATIVE_FLUX: &str = CODEX_ID_STEFAN_BOLTZMANN_RADIATIVE_FLUX;
/// Codex identifier for convective heat flux.
pub const CODEX_ID_CONVECTIVE_HEAT_FLUX: &str = "heat_transfer.convection.convective_heat_flux";
/// Short alias for the convective heat-flux Codex ID.
pub const CODEX_ID_CONVECTIVE_FLUX: &str = CODEX_ID_CONVECTIVE_HEAT_FLUX;
/// Codex identifier for one-dimensional conduction thermal resistance.
pub const CODEX_ID_THERMAL_RESISTANCE_CONDUCTION: &str =
    "heat_transfer.conduction.thermal_resistance";
/// Short alias for the conduction-resistance Codex ID.
pub const CODEX_ID_CONDUCTION_RESISTANCE: &str = CODEX_ID_THERMAL_RESISTANCE_CONDUCTION;
/// Codex identifier for conduction heat rate from thermal resistance.
pub const CODEX_ID_CONDUCTION_HEAT_RATE: &str = "heat_transfer.conduction.heat_rate";

/// Conservative source-registry ID for Phase 0.001 heat-transfer primitive review.
pub const SOURCE_ID_HEAT_TRANSFER_BASIC_PRIMITIVES: &str =
    "source.heat_transfer.basic_primitives.research_required";
/// Short alias for the heat-transfer source-registry ID.
pub const SOURCE_ID_HEAT_TRANSFER_BASICS: &str = SOURCE_ID_HEAT_TRANSFER_BASIC_PRIMITIVES;

const HEAT_TRANSFER_SOURCES: &[&str] = &[SOURCE_ID_HEAT_TRANSFER_BASIC_PRIMITIVES];

/// Conservative traceability metadata for Phase 0.001 heat-transfer helpers.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_STEFAN_BOLTZMANN_RADIATIVE_FLUX => Some(VerificationRecord::research_required(
            CODEX_ID_STEFAN_BOLTZMANN_RADIATIVE_FLUX,
            HEAT_TRANSFER_SOURCES,
            "Net Stefan-Boltzmann radiative-flux relation implemented; exact source, gray-body assumptions, and sign convention review pending.",
        )),
        CODEX_ID_CONVECTIVE_HEAT_FLUX => Some(VerificationRecord::research_required(
            CODEX_ID_CONVECTIVE_HEAT_FLUX,
            HEAT_TRANSFER_SOURCES,
            "Newton cooling-law heat-flux bookkeeping implemented; recovery-temperature and sign-convention review pending.",
        )),
        CODEX_ID_THERMAL_RESISTANCE_CONDUCTION => Some(VerificationRecord::research_required(
            CODEX_ID_THERMAL_RESISTANCE_CONDUCTION,
            HEAT_TRANSFER_SOURCES,
            "One-dimensional plane-wall conduction resistance implemented; geometry and material-assumption review pending.",
        )),
        CODEX_ID_CONDUCTION_HEAT_RATE => Some(VerificationRecord::research_required(
            CODEX_ID_CONDUCTION_HEAT_RATE,
            HEAT_TRANSFER_SOURCES,
            "Conduction heat rate from supplied thermal resistance implemented; network sign convention review pending.",
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
            "computed heat-transfer result was not finite",
        ))
    }
}

fn ensure_nonnegative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed heat-transfer result was not nonnegative and finite",
        ))
    }
}

fn fourth_power_temperature(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    let fourth = value.powi(4);
    if fourth.is_finite() && fourth >= 0.0 {
        Ok(fourth)
    } else {
        Err(numerical_failure(
            codex_id,
            "absolute-temperature fourth power was not nonnegative and finite",
        ))
    }
}

/// Net gray-body radiative heat flux, `q'' = epsilon*sigma*(T_hot^4 - T_cold^4)`, in W/m².
///
/// `emissivity` must be finite and in `[0, 1]`. Both temperatures are absolute
/// temperatures in kelvin and must be finite and nonnegative. The result is
/// signed: it is positive when `t_hot > t_cold`, zero when temperatures are
/// equal or emissivity is zero, and negative when the named hot-side temperature
/// is lower than the cold-side reference. View factors, spectral effects,
/// participating media, shields, and geometry-specific exchange factors are
/// outside this scalar helper.
pub fn stefan_boltzmann_radiative_flux(
    emissivity: f64,
    t_hot: f64,
    t_cold: f64,
) -> AeroResult<f64> {
    validation::ensure_range("emissivity", emissivity, 0.0, 1.0, "0 <= emissivity <= 1")?;
    validation::ensure_nonnegative("t_hot", t_hot)?;
    validation::ensure_nonnegative("t_cold", t_cold)?;

    let hot_fourth = fourth_power_temperature(CODEX_ID_RADIATIVE_FLUX, t_hot)?;
    let cold_fourth = fourth_power_temperature(CODEX_ID_RADIATIVE_FLUX, t_cold)?;
    let temperature_difference_fourth = hot_fourth - cold_fourth;
    if !temperature_difference_fourth.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_RADIATIVE_FLUX,
            "temperature fourth-power difference was not finite",
        ));
    }

    ensure_finite_result(
        CODEX_ID_RADIATIVE_FLUX,
        emissivity
            * aero_codex_constants::STEFAN_BOLTZMANN_W_PER_M2_K4
            * temperature_difference_fourth,
    )
}

/// Convective heat flux, `q'' = h*(T_recovery_or_fluid - T_wall)`, in W/m².
///
/// The heat-transfer coefficient `h` must be finite and nonnegative. The two
/// temperatures are absolute temperatures in kelvin and must be finite and
/// nonnegative. The result is signed by the supplied temperature difference;
/// positive means the recovery/fluid reference is hotter than the wall. This
/// helper does not choose a boundary-layer correlation or compute `h`.
pub fn convective_heat_flux(h: f64, t_recovery_or_fluid: f64, t_wall: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("h", h)?;
    validation::ensure_nonnegative("t_recovery_or_fluid", t_recovery_or_fluid)?;
    validation::ensure_nonnegative("t_wall", t_wall)?;

    let delta_t = t_recovery_or_fluid - t_wall;
    if !delta_t.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_CONVECTIVE_FLUX,
            "convective temperature difference was not finite",
        ));
    }

    ensure_finite_result(CODEX_ID_CONVECTIVE_FLUX, h * delta_t)
}

/// One-dimensional plane-wall conduction thermal resistance, `R = L/(k*A)`, in K/W.
///
/// `thickness` must be finite and nonnegative, while `conductivity` and `area`
/// must be finite and strictly positive. Zero thickness returns zero resistance.
/// Temperature-dependent conductivity, cylindrical/spherical coordinates,
/// contact resistance, and multidimensional effects are outside this scalar
/// helper.
pub fn thermal_resistance_conduction(
    thickness: f64,
    conductivity: f64,
    area: f64,
) -> AeroResult<f64> {
    validation::ensure_nonnegative("thickness", thickness)?;
    validation::ensure_positive("conductivity", conductivity)?;
    validation::ensure_positive("area", area)?;

    let denominator = conductivity * area;
    if !(denominator.is_finite() && denominator > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_CONDUCTION_RESISTANCE,
            "conductivity times area was not finite and positive",
        ));
    }

    ensure_nonnegative_finite_result(CODEX_ID_CONDUCTION_RESISTANCE, thickness / denominator)
}

/// Conductive heat rate through a supplied thermal resistance, `Qdot = DeltaT/R`, in W.
///
/// `delta_t` is signed and finite. `resistance` must be finite and strictly
/// positive; zero-resistance limiting cases must be handled by the caller rather
/// than divided through this helper. Thermal-network assembly, transient storage,
/// and sign-convention mapping to a geometry are outside this function.
pub fn conduction_heat_rate(delta_t: f64, resistance: f64) -> AeroResult<f64> {
    validation::ensure_finite("delta_t", delta_t)?;
    validation::ensure_positive("resistance", resistance)?;

    ensure_finite_result(CODEX_ID_CONDUCTION_HEAT_RATE, delta_t / resistance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_core::{AeroError, VerificationStatus};

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual}, expected={expected}, tolerance={tolerance}"
        );
    }

    #[test]
    fn radiative_flux_zero_when_temperatures_equal() {
        assert_close(
            stefan_boltzmann_radiative_flux(0.8, 300.0, 300.0).unwrap(),
            0.0,
            1.0e-12,
        );
    }

    #[test]
    fn radiative_flux_increases_with_hot_temperature() {
        let low = stefan_boltzmann_radiative_flux(0.8, 310.0, 300.0).unwrap();
        let high = stefan_boltzmann_radiative_flux(0.8, 400.0, 300.0).unwrap();
        assert!(high > low);
    }

    #[test]
    fn radiative_flux_matches_stefan_boltzmann_formula() {
        let flux = stefan_boltzmann_radiative_flux(0.5, 400.0, 300.0).unwrap();
        let expected = 0.5
            * aero_codex_constants::STEFAN_BOLTZMANN_W_PER_M2_K4
            * (400.0_f64.powi(4) - 300.0_f64.powi(4));
        assert_close(flux, expected, 1.0e-9);
    }

    #[test]
    fn radiative_flux_preserves_signed_temperature_difference() {
        assert!(stefan_boltzmann_radiative_flux(1.0, 250.0, 300.0).unwrap() < 0.0);
        assert_eq!(
            stefan_boltzmann_radiative_flux(0.0, 500.0, 300.0).unwrap(),
            0.0
        );
    }

    #[test]
    fn convective_flux_sign_matches_temperature_difference() {
        assert!(convective_heat_flux(10.0, 400.0, 300.0).unwrap() > 0.0);
        assert!(convective_heat_flux(10.0, 250.0, 300.0).unwrap() < 0.0);
        assert_eq!(convective_heat_flux(10.0, 300.0, 300.0).unwrap(), 0.0);
    }

    #[test]
    fn convective_flux_matches_h_delta_t() {
        assert_close(
            convective_heat_flux(25.0, 350.0, 300.0).unwrap(),
            1250.0,
            1.0e-12,
        );
        assert_eq!(convective_heat_flux(0.0, 350.0, 300.0).unwrap(), 0.0);
    }

    #[test]
    fn resistance_positive_for_positive_thickness() {
        assert_close(
            thermal_resistance_conduction(0.1, 20.0, 2.0).unwrap(),
            0.0025,
            1.0e-15,
        );
    }

    #[test]
    fn resistance_zero_for_zero_thickness() {
        assert_eq!(thermal_resistance_conduction(0.0, 20.0, 2.0).unwrap(), 0.0);
    }

    #[test]
    fn conduction_heat_rate_matches_delta_t_over_resistance() {
        assert_close(conduction_heat_rate(100.0, 2.0).unwrap(), 50.0, 1.0e-12);
        assert_close(conduction_heat_rate(-100.0, 2.0).unwrap(), -50.0, 1.0e-12);
    }

    #[test]
    fn invalid_heat_transfer_domains_are_rejected() {
        assert!(matches!(
            stefan_boltzmann_radiative_flux(-0.1, 300.0, 250.0),
            Err(AeroError::OutOfDomain {
                parameter: "emissivity",
                ..
            })
        ));
        assert!(matches!(
            stefan_boltzmann_radiative_flux(1.1, 300.0, 250.0),
            Err(AeroError::OutOfDomain {
                parameter: "emissivity",
                ..
            })
        ));
        assert!(matches!(
            stefan_boltzmann_radiative_flux(0.5, -1.0, 250.0),
            Err(AeroError::NegativeInput {
                parameter: "t_hot",
                ..
            })
        ));
        assert!(matches!(
            convective_heat_flux(-1.0, 300.0, 250.0),
            Err(AeroError::NegativeInput { parameter: "h", .. })
        ));
        assert!(matches!(
            convective_heat_flux(1.0, -300.0, 250.0),
            Err(AeroError::NegativeInput {
                parameter: "t_recovery_or_fluid",
                ..
            })
        ));
        assert!(matches!(
            thermal_resistance_conduction(-0.1, 20.0, 1.0),
            Err(AeroError::NegativeInput {
                parameter: "thickness",
                ..
            })
        ));
        assert!(matches!(
            thermal_resistance_conduction(0.1, 0.0, 1.0),
            Err(AeroError::NonPositiveInput {
                parameter: "conductivity",
                ..
            })
        ));
        assert!(matches!(
            thermal_resistance_conduction(0.1, 20.0, 0.0),
            Err(AeroError::NonPositiveInput {
                parameter: "area",
                ..
            })
        ));
        assert!(matches!(
            conduction_heat_rate(10.0, 0.0),
            Err(AeroError::NonPositiveInput {
                parameter: "resistance",
                ..
            })
        ));
    }

    #[test]
    fn nonfinite_outputs_return_numerical_failure() {
        assert!(matches!(
            stefan_boltzmann_radiative_flux(1.0, f64::MAX, 0.0),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            convective_heat_flux(f64::MAX, f64::MAX, 0.0),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            thermal_resistance_conduction(f64::MAX, f64::MIN_POSITIVE, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            conduction_heat_rate(f64::MAX, f64::MIN_POSITIVE),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn heat_transfer_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_STEFAN_BOLTZMANN_RADIATIVE_FLUX,
            CODEX_ID_CONVECTIVE_HEAT_FLUX,
            CODEX_ID_THERMAL_RESISTANCE_CONDUCTION,
            CODEX_ID_CONDUCTION_HEAT_RATE,
        ] {
            let record = verification_record(codex_id).expect("record should exist");
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert_eq!(record.codex_id, codex_id);
            assert!(record.has_sources());
            assert!(record.sources.contains(&SOURCE_ID_HEAT_TRANSFER_BASICS));
        }
        assert!(verification_record("heat_transfer.unknown").is_none());
    }
}
