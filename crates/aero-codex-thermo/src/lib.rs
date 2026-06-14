#![forbid(unsafe_code)]
//! Phase 0.001 calorically perfect-gas thermodynamics primitives.
//!
//! The functions in this crate are scalar, dependency-free building blocks for
//! early AeroCodex calculations. They assume a perfect gas for density and
//! speed-of-sound helpers, and a calorically perfect gas where heat-capacity
//! relations are used. Inputs and outputs use SI units unless explicitly stated
//! otherwise.
//!
//! All traceability metadata remains conservative in Phase 0.001. The source
//! registry and validation card entries for these equations remain
//! `research_required` until exact source editions, equation/page provenance,
//! reference examples, and tolerances are reviewed.

use aero_codex_constants::{
    SOURCE_ID_NASA_GLENN_CEA, SOURCE_ID_NIST_CODATA_PHYSICAL_CONSTANTS,
    UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K,
};
use aero_codex_core::{validation, AeroError, AeroResult, VerificationRecord};

/// Codex identifier for the ideal-gas density relation `rho = p/(R*T)`.
pub const CODEX_ID_IDEAL_GAS_DENSITY: &str = "thermo.perfect_gas.density";
/// Codex identifier for the perfect-gas speed-of-sound relation.
pub const CODEX_ID_SPEED_OF_SOUND: &str = "thermo.perfect_gas.speed_of_sound";
/// Codex identifier for `cp = gamma*R/(gamma - 1)`.
pub const CODEX_ID_CP_FROM_GAMMA_R: &str = "thermo.perfect_gas.cp_from_gamma_r";
/// Codex identifier for `cv = R/(gamma - 1)`.
pub const CODEX_ID_CV_FROM_GAMMA_R: &str = "thermo.perfect_gas.cv_from_gamma_r";
/// Codex identifier for `gamma = cp/cv`.
pub const CODEX_ID_GAMMA_FROM_CP_CV: &str = "thermo.perfect_gas.gamma_from_cp_cv";
/// Codex identifier for `R_specific = R_universal / molar_mass`.
pub const CODEX_ID_SPECIFIC_GAS_CONSTANT_FROM_MOLAR_MASS: &str =
    "thermo.perfect_gas.specific_gas_constant_from_molar_mass";

const THERMO_SOURCES: &[&str] = &[SOURCE_ID_NASA_GLENN_CEA];
const GAS_CONSTANT_SOURCES: &[&str] = &[
    SOURCE_ID_NASA_GLENN_CEA,
    SOURCE_ID_NIST_CODATA_PHYSICAL_CONSTANTS,
];

/// Conservative traceability metadata for Phase 0.001 thermodynamics equations.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_IDEAL_GAS_DENSITY => Some(VerificationRecord::research_required(
            CODEX_ID_IDEAL_GAS_DENSITY,
            THERMO_SOURCES,
            "Ideal-gas density relation implemented; source equation and tolerance review pending.",
        )),
        CODEX_ID_SPEED_OF_SOUND => Some(VerificationRecord::research_required(
            CODEX_ID_SPEED_OF_SOUND,
            THERMO_SOURCES,
            "Perfect-gas speed-of-sound relation implemented; source and assumption review pending.",
        )),
        CODEX_ID_CP_FROM_GAMMA_R => Some(VerificationRecord::research_required(
            CODEX_ID_CP_FROM_GAMMA_R,
            THERMO_SOURCES,
            "Constant-gamma cp relation implemented; exact source provenance pending.",
        )),
        CODEX_ID_CV_FROM_GAMMA_R => Some(VerificationRecord::research_required(
            CODEX_ID_CV_FROM_GAMMA_R,
            THERMO_SOURCES,
            "Constant-gamma cv relation implemented; exact source provenance pending.",
        )),
        CODEX_ID_GAMMA_FROM_CP_CV => Some(VerificationRecord::research_required(
            CODEX_ID_GAMMA_FROM_CP_CV,
            THERMO_SOURCES,
            "Heat-capacity ratio relation implemented; source and property-definition review pending.",
        )),
        CODEX_ID_SPECIFIC_GAS_CONSTANT_FROM_MOLAR_MASS => Some(
            VerificationRecord::research_required(
                CODEX_ID_SPECIFIC_GAS_CONSTANT_FROM_MOLAR_MASS,
                GAS_CONSTANT_SOURCES,
                "Specific gas constant from universal gas constant and molar mass; CODATA/NIST edition review pending.",
            ),
        ),
        _ => None,
    }
}

fn numerical_failure(codex_id: &'static str, reason: &'static str) -> AeroError {
    AeroError::NumericalFailure {
        solver: codex_id,
        reason,
    }
}

fn ensure_positive_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed result was not positive and finite",
        ))
    }
}

fn ensure_nonnegative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed result was not nonnegative and finite",
        ))
    }
}

/// Ideal-gas density, `rho = p / (R*T)`, in kg/m³.
///
/// Inputs use absolute pressure in pascals, a positive specific gas constant in
/// J/(kg·K), and positive absolute temperature in kelvin. Zero absolute pressure
/// is accepted and returns zero density. Temperature and gas constant must be
/// strictly positive.
pub fn ideal_gas_density(pressure: f64, gas_constant: f64, temperature: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("pressure", pressure)?;
    validation::ensure_positive("gas_constant", gas_constant)?;
    validation::ensure_positive("temperature", temperature)?;

    let denominator = gas_constant * temperature;
    if !(denominator.is_finite() && denominator > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_IDEAL_GAS_DENSITY,
            "gas constant times temperature is not a finite positive denominator",
        ));
    }

    ensure_nonnegative_finite_result(CODEX_ID_IDEAL_GAS_DENSITY, pressure / denominator)
}

/// Perfect-gas speed of sound, `a = sqrt(gamma*R*T)`, in m/s.
///
/// This relation assumes `gamma > 1`, positive specific gas constant, and
/// positive absolute temperature.
pub fn speed_of_sound(gamma: f64, gas_constant: f64, temperature: f64) -> AeroResult<f64> {
    validation::ensure_greater_than("gamma", gamma, 1.0)?;
    validation::ensure_positive("gas_constant", gas_constant)?;
    validation::ensure_positive("temperature", temperature)?;

    let radicand = gamma * gas_constant * temperature;
    if !(radicand.is_finite() && radicand > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_SPEED_OF_SOUND,
            "gamma times gas constant times temperature is not finite and positive",
        ));
    }

    ensure_positive_finite_result(CODEX_ID_SPEED_OF_SOUND, radicand.sqrt())
}

/// Constant-gamma specific heat at constant pressure, `cp = gamma*R/(gamma - 1)`.
///
/// Returns `cp` in J/(kg·K) for a positive specific gas constant in J/(kg·K).
pub fn cp_from_gamma_r(gamma: f64, gas_constant: f64) -> AeroResult<f64> {
    validation::ensure_greater_than("gamma", gamma, 1.0)?;
    validation::ensure_positive("gas_constant", gas_constant)?;

    ensure_positive_finite_result(
        CODEX_ID_CP_FROM_GAMMA_R,
        gamma * gas_constant / (gamma - 1.0),
    )
}

/// Constant-gamma specific heat at constant volume, `cv = R/(gamma - 1)`.
///
/// Returns `cv` in J/(kg·K) for a positive specific gas constant in J/(kg·K).
pub fn cv_from_gamma_r(gamma: f64, gas_constant: f64) -> AeroResult<f64> {
    validation::ensure_greater_than("gamma", gamma, 1.0)?;
    validation::ensure_positive("gas_constant", gas_constant)?;

    ensure_positive_finite_result(CODEX_ID_CV_FROM_GAMMA_R, gas_constant / (gamma - 1.0))
}

/// Heat-capacity ratio, `gamma = cp/cv`.
///
/// Both heat capacities must be positive, finite, and must satisfy `cp > cv` for
/// this Phase 0.001 calorically perfect-gas helper.
pub fn gamma_from_cp_cv(cp: f64, cv: f64) -> AeroResult<f64> {
    validation::ensure_positive("cp", cp)?;
    validation::ensure_positive("cv", cv)?;
    if cp <= cv {
        return Err(AeroError::OutOfDomain {
            parameter: "cp",
            value: cp,
            expected: "cp > cv for a calorically perfect gas",
        });
    }

    ensure_positive_finite_result(CODEX_ID_GAMMA_FROM_CP_CV, cp / cv)
}

/// Specific gas constant, `R_specific = R_universal / molar_mass`, in J/(kg·K).
///
/// The molar mass must be positive and finite in kg/mol.
pub fn specific_gas_constant_from_molar_mass(molar_mass_kg_per_mol: f64) -> AeroResult<f64> {
    validation::ensure_positive("molar_mass_kg_per_mol", molar_mass_kg_per_mol)?;

    ensure_positive_finite_result(
        CODEX_ID_SPECIFIC_GAS_CONSTANT_FROM_MOLAR_MASS,
        UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K / molar_mass_kg_per_mol,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_constants::{
        STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K, STANDARD_GAMMA_DRY_AIR,
        STANDARD_SEA_LEVEL_PRESSURE_PA, STANDARD_SEA_LEVEL_TEMPERATURE_K,
    };
    use aero_codex_core::VerificationStatus;

    fn approx(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} !~= {b}");
    }

    #[test]
    fn air_density_near_sea_level_from_p_over_rt() {
        let rho = ideal_gas_density(
            STANDARD_SEA_LEVEL_PRESSURE_PA,
            STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K,
            STANDARD_SEA_LEVEL_TEMPERATURE_K,
        )
        .unwrap();
        approx(rho, 1.225, 0.01);
    }

    #[test]
    fn zero_pressure_returns_zero_density() {
        approx(
            ideal_gas_density(0.0, STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K, 288.15).unwrap(),
            0.0,
            1.0e-12,
        );
    }

    #[test]
    fn cp_exceeds_cv_and_gamma_round_trips() {
        let cp =
            cp_from_gamma_r(STANDARD_GAMMA_DRY_AIR, STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K).unwrap();
        let cv =
            cv_from_gamma_r(STANDARD_GAMMA_DRY_AIR, STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K).unwrap();
        assert!(cp > cv);
        approx(
            gamma_from_cp_cv(cp, cv).unwrap(),
            STANDARD_GAMMA_DRY_AIR,
            1.0e-12,
        );
    }

    #[test]
    fn cp_minus_cv_equals_specific_gas_constant() {
        let cp = cp_from_gamma_r(1.33, 287.0).unwrap();
        let cv = cv_from_gamma_r(1.33, 287.0).unwrap();
        approx(cp - cv, 287.0, 1.0e-10);
    }

    #[test]
    fn speed_of_sound_positive_for_valid_inputs() {
        let a = speed_of_sound(
            STANDARD_GAMMA_DRY_AIR,
            STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K,
            STANDARD_SEA_LEVEL_TEMPERATURE_K,
        )
        .unwrap();
        approx(a, 340.294, 0.01);
        assert!(a > 0.0);
    }

    #[test]
    fn dry_air_specific_gas_constant_from_molar_mass_is_plausible() {
        let r = specific_gas_constant_from_molar_mass(0.028_964_7).unwrap();
        approx(r, STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K, 0.1);
    }

    #[test]
    fn invalid_temperature_rejected() {
        assert!(ideal_gas_density(101_325.0, 287.0, 0.0).is_err());
        assert!(ideal_gas_density(101_325.0, 287.0, -1.0).is_err());
        assert!(speed_of_sound(1.4, 287.0, 0.0).is_err());
        assert!(speed_of_sound(1.4, 287.0, f64::NAN).is_err());
    }

    #[test]
    fn invalid_pressure_and_gas_constant_rejected() {
        assert!(ideal_gas_density(-1.0, 287.0, 288.15).is_err());
        assert!(ideal_gas_density(f64::INFINITY, 287.0, 288.15).is_err());
        assert!(ideal_gas_density(101_325.0, 0.0, 288.15).is_err());
        assert!(speed_of_sound(1.4, -287.0, 288.15).is_err());
    }

    #[test]
    fn invalid_gamma_and_heat_capacities_rejected() {
        assert!(cp_from_gamma_r(1.0, 287.0).is_err());
        assert!(cv_from_gamma_r(f64::NAN, 287.0).is_err());
        assert!(gamma_from_cp_cv(700.0, 700.0).is_err());
        assert!(gamma_from_cp_cv(600.0, 700.0).is_err());
        assert!(gamma_from_cp_cv(1000.0, 0.0).is_err());
    }

    #[test]
    fn invalid_molar_mass_rejected() {
        assert!(specific_gas_constant_from_molar_mass(0.0).is_err());
        assert!(specific_gas_constant_from_molar_mass(-0.01).is_err());
        assert!(specific_gas_constant_from_molar_mass(f64::NAN).is_err());
    }

    #[test]
    fn nonfinite_outputs_return_numerical_failure() {
        assert!(matches!(
            speed_of_sound(f64::MAX, f64::MAX, f64::MAX),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            specific_gas_constant_from_molar_mass(f64::MIN_POSITIVE),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            cp_from_gamma_r(1.0 + f64::EPSILON, f64::MAX),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn thermodynamics_verification_records_remain_research_required() {
        let record = verification_record(CODEX_ID_IDEAL_GAS_DENSITY).unwrap();
        assert_eq!(record.codex_id, CODEX_ID_IDEAL_GAS_DENSITY);
        assert_eq!(record.status, VerificationStatus::ResearchRequired);
        assert_eq!(record.sources, THERMO_SOURCES);

        let gas_constant_record =
            verification_record(CODEX_ID_SPECIFIC_GAS_CONSTANT_FROM_MOLAR_MASS).unwrap();
        assert_eq!(
            gas_constant_record.status,
            VerificationStatus::ResearchRequired
        );
        assert_eq!(gas_constant_record.sources, GAS_CONSTANT_SOURCES);
        assert!(verification_record("thermo.perfect_gas.unknown").is_none());
    }
}
