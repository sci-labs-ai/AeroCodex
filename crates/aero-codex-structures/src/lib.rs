#![forbid(unsafe_code)]
//! Phase 0.001 structural mechanics primitives.
//!
//! This crate implements scalar preliminary-design helpers for elementary
//! axial stress, elastic bending stress, cantilever tip deflection under an end
//! load, and Euler column buckling load. Dimensional inputs and outputs use SI
//! units.
//!
//! Phase 0.001 deliberately does not model combined loading interaction,
//! plasticity, stress concentration factors, shear deformation, large
//! deflection, lateral-torsional buckling, local panel buckling, crippling,
//! fatigue, fracture, material allowables, finite-element workflows, or
//! certification margins. Traceability metadata remains conservative
//! `research_required` until exact source editions, equation identifiers,
//! reference examples, sign conventions, and tolerances are reviewed.

use aero_codex_core::{validation, AeroError, AeroResult, VerificationRecord};
use std::f64::consts::PI;

/// Codex identifier for elementary axial normal stress, `sigma = F/A`.
pub const CODEX_ID_AXIAL_STRESS: &str = "structures.stress.axial";
/// Codex identifier for elementary elastic bending stress, `sigma = M*y/I`.
pub const CODEX_ID_BENDING_STRESS: &str = "structures.stress.bending";
/// Codex identifier for cantilever tip deflection under an end load.
pub const CODEX_ID_CANTILEVER_TIP_DEFLECTION_END_LOAD: &str =
    "structures.beam.cantilever_tip_deflection_end_load";
/// Backward-compatible short alias for the cantilever tip-deflection Codex ID.
pub const CODEX_ID_CANTILEVER_DEFLECTION: &str = CODEX_ID_CANTILEVER_TIP_DEFLECTION_END_LOAD;
/// Codex identifier for Euler elastic column buckling load.
pub const CODEX_ID_EULER_COLUMN_BUCKLING_LOAD: &str =
    "structures.stability.euler_column_buckling_load";
/// Backward-compatible short alias for the Euler buckling Codex ID.
pub const CODEX_ID_EULER_BUCKLING: &str = CODEX_ID_EULER_COLUMN_BUCKLING_LOAD;

/// Conservative source-registry ID for Phase 0.001 structures primitive review.
pub const SOURCE_ID_STRUCTURES_BASIC_MECHANICS: &str =
    "source.structures.basic_mechanics.research_required";
/// Descriptive alias for the elementary structures source-registry ID.
pub const SOURCE_ID_STRUCTURES_BEAM_BUCKLING_BASICS: &str = SOURCE_ID_STRUCTURES_BASIC_MECHANICS;
/// Short alias for the structures source-registry ID.
pub const SOURCE_ID_STRUCTURES_BASICS: &str = SOURCE_ID_STRUCTURES_BASIC_MECHANICS;

const STRUCTURES_SOURCES: &[&str] = &[SOURCE_ID_STRUCTURES_BASIC_MECHANICS];

/// Conservative traceability metadata for Phase 0.001 structures helpers.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_AXIAL_STRESS => Some(VerificationRecord::research_required(
            CODEX_ID_AXIAL_STRESS,
            STRUCTURES_SOURCES,
            "Elementary axial-stress relation implemented; exact source, sign convention, and reference examples pending.",
        )),
        CODEX_ID_BENDING_STRESS => Some(VerificationRecord::research_required(
            CODEX_ID_BENDING_STRESS,
            STRUCTURES_SOURCES,
            "Elementary elastic bending-stress relation implemented; section-axis and sign-convention review pending.",
        )),
        CODEX_ID_CANTILEVER_TIP_DEFLECTION_END_LOAD => Some(VerificationRecord::research_required(
            CODEX_ID_CANTILEVER_TIP_DEFLECTION_END_LOAD,
            STRUCTURES_SOURCES,
            "Euler-Bernoulli cantilever end-load tip deflection implemented; small-deflection and shear-deformation assumptions pending source review.",
        )),
        CODEX_ID_EULER_COLUMN_BUCKLING_LOAD => Some(VerificationRecord::research_required(
            CODEX_ID_EULER_COLUMN_BUCKLING_LOAD,
            STRUCTURES_SOURCES,
            "Euler elastic column critical-load relation implemented; end-condition factor and applicability review pending.",
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
            "computed structures result was not finite",
        ))
    }
}

fn ensure_positive_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed structures result was not positive and finite",
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

/// Elementary axial normal stress, `sigma = F/A`, in pascals.
///
/// `force` is signed and finite; tension/compression sign conventions are left
/// to the caller. `area` must be finite and strictly positive. Net-section,
/// bearing, stress concentration, material allowables, combined loading, and
/// margin-of-safety calculations are outside this helper.
pub fn axial_stress(force: f64, area: f64) -> AeroResult<f64> {
    validation::ensure_finite("force", force)?;
    validation::ensure_positive("area", area)?;

    ensure_finite_result(CODEX_ID_AXIAL_STRESS, force / area)
}

/// Elementary elastic bending stress, `sigma = M*y/I`, in pascals.
///
/// `moment` and `y` are signed and finite so callers can preserve their chosen
/// section-axis and fiber sign convention. `second_moment_area` must be finite
/// and strictly positive. Plastic section behavior, asymmetric sections,
/// combined axial-bending checks, shear stress, and local buckling are outside
/// this scalar helper.
pub fn bending_stress(moment: f64, y: f64, second_moment_area: f64) -> AeroResult<f64> {
    validation::ensure_finite("moment", moment)?;
    validation::ensure_finite("y", y)?;
    validation::ensure_positive("second_moment_area", second_moment_area)?;

    let numerator = checked_product(
        CODEX_ID_BENDING_STRESS,
        "bending-stress moment times distance was not finite",
        moment,
        y,
    )?;
    ensure_finite_result(CODEX_ID_BENDING_STRESS, numerator / second_moment_area)
}

/// Small-deflection Euler-Bernoulli cantilever tip deflection under an end load.
///
/// Implements `delta = F*L^3/(3*E*I)`. `force` is signed and finite so caller
/// sign conventions are preserved. `length`, `elastic_modulus`, and
/// `second_moment_area` must be finite and strictly positive. The result is in
/// metres and signed by the supplied force. Shear deformation, large deflection,
/// distributed loads, nonprismatic beams, boundary flexibility, and material
/// nonlinearity are outside this helper.
pub fn cantilever_tip_deflection_end_load(
    force: f64,
    length: f64,
    elastic_modulus: f64,
    second_moment_area: f64,
) -> AeroResult<f64> {
    validation::ensure_finite("force", force)?;
    validation::ensure_positive("length", length)?;
    validation::ensure_positive("elastic_modulus", elastic_modulus)?;
    validation::ensure_positive("second_moment_area", second_moment_area)?;

    let length_cubed = length.powi(3);
    if !(length_cubed.is_finite() && length_cubed > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_CANTILEVER_DEFLECTION,
            "cantilever length cubed was not positive and finite",
        ));
    }

    let numerator = checked_product(
        CODEX_ID_CANTILEVER_DEFLECTION,
        "cantilever force times length cubed was not finite",
        force,
        length_cubed,
    )?;
    let stiffness = checked_product(
        CODEX_ID_CANTILEVER_DEFLECTION,
        "elastic modulus times second moment of area was not finite",
        elastic_modulus,
        second_moment_area,
    )?;
    let denominator = 3.0 * stiffness;
    if !(denominator.is_finite() && denominator > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_CANTILEVER_DEFLECTION,
            "cantilever stiffness denominator was not positive and finite",
        ));
    }

    ensure_finite_result(CODEX_ID_CANTILEVER_DEFLECTION, numerator / denominator)
}

/// Euler elastic column critical buckling load, `P_cr = pi^2*E*I/(K*L)^2`, in newtons.
///
/// `effective_length_factor`, `elastic_modulus`, `second_moment_area`, and
/// `length` must be finite and strictly positive. The returned critical load is
/// positive. This scalar helper assumes an ideal, straight, prismatic,
/// concentrically loaded elastic column with the caller-supplied effective
/// length factor. Inelastic buckling, imperfections, eccentricity, local
/// buckling, lateral-torsional buckling, material allowables, knockdown factors,
/// and code-specific design margins are outside Phase 0.001.
pub fn euler_column_buckling_load(
    effective_length_factor: f64,
    elastic_modulus: f64,
    second_moment_area: f64,
    length: f64,
) -> AeroResult<f64> {
    validation::ensure_positive("effective_length_factor", effective_length_factor)?;
    validation::ensure_positive("elastic_modulus", elastic_modulus)?;
    validation::ensure_positive("second_moment_area", second_moment_area)?;
    validation::ensure_positive("length", length)?;

    let effective_length = checked_product(
        CODEX_ID_EULER_BUCKLING,
        "effective length factor times length was not finite",
        effective_length_factor,
        length,
    )?;
    if effective_length <= 0.0 {
        return Err(numerical_failure(
            CODEX_ID_EULER_BUCKLING,
            "effective length was not positive",
        ));
    }

    let elastic_rigidity = checked_product(
        CODEX_ID_EULER_BUCKLING,
        "elastic modulus times second moment of area was not finite",
        elastic_modulus,
        second_moment_area,
    )?;
    let numerator = checked_product(
        CODEX_ID_EULER_BUCKLING,
        "pi squared times elastic rigidity was not finite",
        PI * PI,
        elastic_rigidity,
    )?;
    let denominator = effective_length.powi(2);
    if !(denominator.is_finite() && denominator > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_EULER_BUCKLING,
            "effective length squared was not positive and finite",
        ));
    }

    ensure_positive_finite_result(CODEX_ID_EULER_BUCKLING, numerator / denominator)
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
    fn axial_stress_is_force_over_area() {
        assert_close(axial_stress(100.0, 2.0).unwrap(), 50.0, 1.0e-12);
        assert_close(axial_stress(-100.0, 2.0).unwrap(), -50.0, 1.0e-12);
    }

    #[test]
    fn axial_stress_rejects_invalid_area_and_nonfinite_force() {
        assert!(axial_stress(100.0, 0.0).is_err());
        assert!(axial_stress(100.0, -1.0).is_err());
        assert!(axial_stress(f64::NAN, 2.0).is_err());
    }

    #[test]
    fn bending_stress_is_moment_times_y_over_i() {
        assert_close(
            bending_stress(200.0, 0.1, 1.0e-4).unwrap(),
            200_000.0,
            1.0e-6,
        );
        assert_close(
            bending_stress(-200.0, 0.1, 1.0e-4).unwrap(),
            -200_000.0,
            1.0e-6,
        );
    }

    #[test]
    fn bending_stress_increases_with_moment() {
        let lower = bending_stress(100.0, 0.1, 1.0e-4).unwrap();
        let higher = bending_stress(200.0, 0.1, 1.0e-4).unwrap();
        assert!(higher > lower);
    }

    #[test]
    fn bending_stress_rejects_invalid_second_moment_and_overflow() {
        assert!(bending_stress(1.0, 1.0, 0.0).is_err());
        assert!(matches!(
            bending_stress(f64::MAX, 2.0, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn cantilever_deflection_matches_end_load_formula() {
        let force: f64 = 10.0;
        let length: f64 = 2.0;
        let elastic_modulus: f64 = 70.0e9;
        let second_moment_area: f64 = 1.0e-6;
        let expected = force * length.powi(3) / (3.0 * elastic_modulus * second_moment_area);
        assert_close(
            cantilever_tip_deflection_end_load(force, length, elastic_modulus, second_moment_area)
                .unwrap(),
            expected,
            1.0e-15,
        );
    }

    #[test]
    fn cantilever_deflection_increases_with_force() {
        let lower = cantilever_tip_deflection_end_load(10.0, 1.0, 70.0e9, 1.0e-6).unwrap();
        let higher = cantilever_tip_deflection_end_load(20.0, 1.0, 70.0e9, 1.0e-6).unwrap();
        assert!(higher > lower);
    }

    #[test]
    fn cantilever_deflection_preserves_signed_force() {
        let positive = cantilever_tip_deflection_end_load(10.0, 1.0, 70.0e9, 1.0e-6).unwrap();
        let negative = cantilever_tip_deflection_end_load(-10.0, 1.0, 70.0e9, 1.0e-6).unwrap();
        assert_close(positive, -negative, 1.0e-15);
    }

    #[test]
    fn cantilever_deflection_rejects_invalid_domains_and_overflow() {
        assert!(cantilever_tip_deflection_end_load(10.0, 0.0, 70.0e9, 1.0e-6).is_err());
        assert!(cantilever_tip_deflection_end_load(10.0, 1.0, 0.0, 1.0e-6).is_err());
        assert!(cantilever_tip_deflection_end_load(10.0, 1.0, 70.0e9, 0.0).is_err());
        assert!(matches!(
            cantilever_tip_deflection_end_load(f64::MAX, 2.0, 1.0, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn euler_buckling_load_matches_formula_and_decreases_with_length() {
        let short = euler_column_buckling_load(1.0, 70.0e9, 1.0e-6, 1.0).unwrap();
        let long = euler_column_buckling_load(1.0, 70.0e9, 1.0e-6, 2.0).unwrap();
        let expected = PI * PI * 70.0e9 * 1.0e-6;
        assert_close(short, expected, 1.0e-6);
        assert!(long < short);
    }

    #[test]
    fn euler_buckling_load_decreases_with_length() {
        let short = euler_column_buckling_load(1.0, 70.0e9, 1.0e-6, 1.0).unwrap();
        let long = euler_column_buckling_load(1.0, 70.0e9, 1.0e-6, 2.0).unwrap();
        assert!(long < short);
    }

    #[test]
    fn euler_buckling_rejects_invalid_domains_and_overflow() {
        assert!(euler_column_buckling_load(0.0, 70.0e9, 1.0e-6, 1.0).is_err());
        assert!(euler_column_buckling_load(1.0, 0.0, 1.0e-6, 1.0).is_err());
        assert!(euler_column_buckling_load(1.0, 70.0e9, 0.0, 1.0).is_err());
        assert!(euler_column_buckling_load(1.0, 70.0e9, 1.0e-6, 0.0).is_err());
        assert!(matches!(
            euler_column_buckling_load(1.0, f64::MAX, 2.0, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn nonfinite_derived_outputs_return_numerical_failure() {
        assert!(matches!(
            axial_stress(f64::MAX, f64::MIN_POSITIVE),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            bending_stress(f64::MAX, 2.0, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            cantilever_tip_deflection_end_load(f64::MAX, 2.0, 1.0, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            euler_column_buckling_load(1.0, f64::MAX, 2.0, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn structures_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_AXIAL_STRESS,
            CODEX_ID_BENDING_STRESS,
            CODEX_ID_CANTILEVER_DEFLECTION,
            CODEX_ID_EULER_BUCKLING,
        ] {
            let record = verification_record(codex_id).expect("structures verification record");
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert!(record.has_sources());
            assert_eq!(record.sources, STRUCTURES_SOURCES);
        }
        assert!(verification_record("structures.unknown").is_none());
    }
}
