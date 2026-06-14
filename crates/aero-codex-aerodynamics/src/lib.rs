#![forbid(unsafe_code)]
//! Phase 0.001 aerodynamic force and coefficient helpers.
//!
//! These helpers implement scalar preliminary-design relations for dynamic
//! pressure, lift, drag, coefficient inversion, and induced drag. Inputs and
//! outputs use SI units where the quantity has dimensions. The functions do not
//! model compressibility corrections, viscous effects, Reynolds-number effects,
//! stall, finite-wing lift-curve slope, trim, stability derivatives, or flight
//! envelopes.
//!
//! Traceability metadata remains conservative in Phase 0.001. The validation
//! card and source-registry seed for these coefficient definitions remain
//! `research_required` until exact source editions, equation identifiers,
//! reference examples, and tolerances are reviewed.

use aero_codex_core::{validation, AeroError, AeroResult, VerificationRecord};
use std::f64::consts::PI;

/// Codex identifier for `q = 0.5*rho*V^2`.
pub const CODEX_ID_DYNAMIC_PRESSURE: &str = "aero.forces.dynamic_pressure";
/// Codex identifier for `L = q*S*CL`.
pub const CODEX_ID_LIFT: &str = "aero.forces.lift";
/// Codex identifier for `D = q*S*CD`.
pub const CODEX_ID_DRAG: &str = "aero.forces.drag";
/// Codex identifier for `CL = L/(q*S)`.
pub const CODEX_ID_LIFT_COEFFICIENT: &str = "aero.coefficients.lift_coefficient";
/// Codex identifier for `CD = D/(q*S)`.
pub const CODEX_ID_DRAG_COEFFICIENT: &str = "aero.coefficients.drag_coefficient";
/// Codex identifier for `CD_i = CL^2/(pi*AR*e)`.
pub const CODEX_ID_INDUCED_DRAG: &str = "aero.drag.induced_drag_coefficient";
/// Fully descriptive alias for the induced-drag coefficient Codex ID.
pub const CODEX_ID_INDUCED_DRAG_COEFFICIENT: &str = CODEX_ID_INDUCED_DRAG;

/// Conservative source-registry ID for basic aerodynamic coefficient definitions.
pub const SOURCE_ID_AERODYNAMIC_COEFFICIENTS: &str =
    "source.aerodynamics.basic_coefficients.research_required";
/// Backward-compatible descriptive alias for the source-registry ID.
pub const SOURCE_ID_AERODYNAMICS_BASIC_COEFFICIENTS: &str = SOURCE_ID_AERODYNAMIC_COEFFICIENTS;

const AERODYNAMIC_COEFFICIENT_SOURCES: &[&str] = &[SOURCE_ID_AERODYNAMIC_COEFFICIENTS];

/// Conservative traceability metadata for Phase 0.001 aerodynamic helpers.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_DYNAMIC_PRESSURE => Some(VerificationRecord::research_required(
            CODEX_ID_DYNAMIC_PRESSURE,
            AERODYNAMIC_COEFFICIENT_SOURCES,
            "Dynamic-pressure relation implemented; source equation and tolerance review pending.",
        )),
        CODEX_ID_LIFT => Some(VerificationRecord::research_required(
            CODEX_ID_LIFT,
            AERODYNAMIC_COEFFICIENT_SOURCES,
            "Lift force coefficient definition implemented; source convention review pending.",
        )),
        CODEX_ID_DRAG => Some(VerificationRecord::research_required(
            CODEX_ID_DRAG,
            AERODYNAMIC_COEFFICIENT_SOURCES,
            "Drag force coefficient definition implemented; source convention review pending.",
        )),
        CODEX_ID_LIFT_COEFFICIENT => Some(VerificationRecord::research_required(
            CODEX_ID_LIFT_COEFFICIENT,
            AERODYNAMIC_COEFFICIENT_SOURCES,
            "Lift coefficient inverse definition implemented; sign convention and reference examples pending.",
        )),
        CODEX_ID_DRAG_COEFFICIENT => Some(VerificationRecord::research_required(
            CODEX_ID_DRAG_COEFFICIENT,
            AERODYNAMIC_COEFFICIENT_SOURCES,
            "Drag coefficient inverse definition implemented; sign convention and reference examples pending.",
        )),
        CODEX_ID_INDUCED_DRAG => Some(VerificationRecord::research_required(
            CODEX_ID_INDUCED_DRAG,
            AERODYNAMIC_COEFFICIENT_SOURCES,
            "Classical induced-drag coefficient relation implemented; finite-wing assumptions and source validation pending.",
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
            "computed aerodynamic result was not finite",
        ))
    }
}

fn ensure_nonnegative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed aerodynamic result was not nonnegative and finite",
        ))
    }
}

fn positive_reference_product(codex_id: &'static str, q: f64, area: f64) -> AeroResult<f64> {
    validation::ensure_positive("q", q)?;
    validation::ensure_positive("area", area)?;

    let denominator = q * area;
    if denominator.is_finite() && denominator > 0.0 {
        Ok(denominator)
    } else {
        Err(numerical_failure(
            codex_id,
            "q times area is not a finite positive denominator",
        ))
    }
}

/// Dynamic pressure, `q = 0.5*rho*V^2`, in pascals.
///
/// Density must be finite and nonnegative. Velocity is treated as a scalar
/// speed magnitude and must be finite and nonnegative. This is the basic
/// incompressible-form dynamic pressure relation; compressibility corrections
/// and atmosphere-model coupling are outside this helper.
pub fn dynamic_pressure(rho: f64, velocity: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("rho", rho)?;
    validation::ensure_nonnegative("velocity", velocity)?;

    ensure_nonnegative_finite_result(CODEX_ID_DYNAMIC_PRESSURE, 0.5 * rho * velocity * velocity)
}

/// Lift force, `L = q*S*CL`, in newtons when `q` is pascals and `S` is m².
///
/// Dynamic pressure and reference area must be finite and nonnegative. The lift
/// coefficient is allowed to be signed but must be finite.
pub fn lift(q: f64, area: f64, cl: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("q", q)?;
    validation::ensure_nonnegative("area", area)?;
    validation::ensure_finite("cl", cl)?;

    ensure_finite_result(CODEX_ID_LIFT, q * area * cl)
}

/// Drag force, `D = q*S*CD`, in newtons when `q` is pascals and `S` is m².
///
/// Dynamic pressure and reference area must be finite and nonnegative. The drag
/// coefficient is required to be finite; Phase 0.001 does not enforce a
/// nonnegative sign convention because some analysis conventions use signed
/// axial-force components.
pub fn drag(q: f64, area: f64, cd: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("q", q)?;
    validation::ensure_nonnegative("area", area)?;
    validation::ensure_finite("cd", cd)?;

    ensure_finite_result(CODEX_ID_DRAG, q * area * cd)
}

/// Lift coefficient, `CL = L/(q*S)`.
///
/// Lift force may be signed but must be finite. Dynamic pressure and reference
/// area must be strictly positive so the denominator is physically meaningful.
pub fn lift_coefficient(lift: f64, q: f64, area: f64) -> AeroResult<f64> {
    validation::ensure_finite("lift", lift)?;
    let denominator = positive_reference_product(CODEX_ID_LIFT_COEFFICIENT, q, area)?;

    ensure_finite_result(CODEX_ID_LIFT_COEFFICIENT, lift / denominator)
}

/// Drag coefficient, `CD = D/(q*S)`.
///
/// Drag force may be signed but must be finite. Dynamic pressure and reference
/// area must be strictly positive so the denominator is physically meaningful.
pub fn drag_coefficient(drag: f64, q: f64, area: f64) -> AeroResult<f64> {
    validation::ensure_finite("drag", drag)?;
    let denominator = positive_reference_product(CODEX_ID_DRAG_COEFFICIENT, q, area)?;

    ensure_finite_result(CODEX_ID_DRAG_COEFFICIENT, drag / denominator)
}

/// Classical induced-drag coefficient, `CD_i = CL^2/(pi*AR*e)`.
///
/// The lift coefficient must be finite. Aspect ratio and Oswald efficiency must
/// be strictly positive. Phase 0.001 leaves detailed finite-wing applicability,
/// planform definitions, and efficiency bounds to later source review.
pub fn induced_drag_coefficient(
    cl: f64,
    aspect_ratio: f64,
    oswald_efficiency: f64,
) -> AeroResult<f64> {
    validation::ensure_finite("cl", cl)?;
    validation::ensure_positive("aspect_ratio", aspect_ratio)?;
    validation::ensure_positive("oswald_efficiency", oswald_efficiency)?;

    let numerator = cl * cl;
    if !numerator.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_INDUCED_DRAG,
            "squared lift coefficient was not finite",
        ));
    }

    let denominator = PI * aspect_ratio * oswald_efficiency;
    if !(denominator.is_finite() && denominator > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_INDUCED_DRAG,
            "pi times aspect ratio times Oswald efficiency is not a finite positive denominator",
        ));
    }

    ensure_nonnegative_finite_result(CODEX_ID_INDUCED_DRAG, numerator / denominator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_core::VerificationStatus;

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        let error = (actual - expected).abs();
        assert!(
            error <= tolerance,
            "actual={actual}, expected={expected}, error={error}, tolerance={tolerance}"
        );
    }

    fn assert_numerical_failure(result: AeroResult<f64>) {
        match result {
            Err(AeroError::NumericalFailure { .. }) => {}
            other => panic!("expected numerical failure, got {other:?}"),
        }
    }

    #[test]
    fn dynamic_pressure_formula() {
        assert_close(dynamic_pressure(1.2, 10.0).unwrap(), 60.0, 1.0e-12);
        assert_close(dynamic_pressure(0.0, 10.0).unwrap(), 0.0, 1.0e-12);
        assert_close(dynamic_pressure(1.2, 0.0).unwrap(), 0.0, 1.0e-12);
    }

    #[test]
    fn dynamic_pressure_rejects_negative_and_nonfinite_inputs() {
        assert!(dynamic_pressure(-1.0, 10.0).is_err());
        assert!(dynamic_pressure(1.0, -10.0).is_err());
        assert!(dynamic_pressure(f64::NAN, 10.0).is_err());
        assert!(dynamic_pressure(1.0, f64::INFINITY).is_err());
    }

    #[test]
    fn lift_and_coefficient_inverse_round_trip() {
        let q = 500.0;
        let area = 12.0;
        let cl = 0.8;
        let force = lift(q, area, cl).unwrap();

        assert_close(force, 4800.0, 1.0e-12);
        assert_close(lift_coefficient(force, q, area).unwrap(), cl, 1.0e-12);
    }

    #[test]
    fn drag_and_coefficient_inverse_round_trip() {
        let q = 500.0;
        let area = 12.0;
        let cd = 0.04;
        let force = drag(q, area, cd).unwrap();

        assert_close(force, 240.0, 1.0e-12);
        assert_close(drag_coefficient(force, q, area).unwrap(), cd, 1.0e-12);
    }

    #[test]
    fn signed_lift_and_drag_coefficients_are_finite_conventions() {
        assert!(lift(100.0, 2.0, -0.5).unwrap() < 0.0);
        assert!(lift_coefficient(-100.0, 100.0, 2.0).unwrap() < 0.0);
        assert!(drag(100.0, 2.0, -0.02).unwrap() < 0.0);
        assert!(drag_coefficient(-4.0, 100.0, 2.0).unwrap() < 0.0);
    }

    #[test]
    fn induced_drag_positive_for_nonzero_lift() {
        let cdi = induced_drag_coefficient(0.7, 8.0, 0.85).unwrap();
        assert!(cdi > 0.0);
        assert_close(induced_drag_coefficient(0.0, 8.0, 0.85).unwrap(), 0.0, 0.0);
    }

    #[test]
    fn invalid_basic_aerodynamic_inputs_are_rejected() {
        assert!(dynamic_pressure(-1.0, 10.0).is_err());
        assert!(dynamic_pressure(1.0, -10.0).is_err());
        assert!(lift(-1.0, 1.0, 0.5).is_err());
        assert!(lift(1.0, -1.0, 0.5).is_err());
        assert!(drag(-1.0, 1.0, 0.05).is_err());
        assert!(drag(1.0, -1.0, 0.05).is_err());
        assert!(lift_coefficient(1.0, 0.0, 1.0).is_err());
        assert!(lift_coefficient(1.0, 1.0, 0.0).is_err());
        assert!(drag_coefficient(1.0, 0.0, 1.0).is_err());
        assert!(drag_coefficient(1.0, 1.0, 0.0).is_err());
        assert!(induced_drag_coefficient(0.7, 0.0, 0.85).is_err());
        assert!(induced_drag_coefficient(0.7, 8.0, 0.0).is_err());
        assert!(induced_drag_coefficient(0.7, 8.0, -0.1).is_err());
    }

    #[test]
    fn induced_drag_rejects_invalid_inputs() {
        assert!(induced_drag_coefficient(f64::NAN, 8.0, 0.85).is_err());
        assert!(induced_drag_coefficient(0.7, -1.0, 0.85).is_err());
        assert!(induced_drag_coefficient(0.7, 8.0, f64::INFINITY).is_err());
    }

    #[test]
    fn nonfinite_derived_outputs_return_numerical_failure() {
        assert_numerical_failure(dynamic_pressure(f64::MAX, f64::MAX));
        assert_numerical_failure(lift(f64::MAX, f64::MAX, 1.0));
        assert_numerical_failure(drag(f64::MAX, f64::MAX, 1.0));
        assert_numerical_failure(lift_coefficient(f64::MAX, f64::MAX, f64::MAX));
        assert_numerical_failure(drag_coefficient(f64::MAX, f64::MAX, f64::MAX));
        assert_numerical_failure(induced_drag_coefficient(f64::MAX, 1.0, 1.0));
    }

    #[test]
    fn aerodynamics_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_DYNAMIC_PRESSURE,
            CODEX_ID_LIFT,
            CODEX_ID_DRAG,
            CODEX_ID_LIFT_COEFFICIENT,
            CODEX_ID_DRAG_COEFFICIENT,
            CODEX_ID_INDUCED_DRAG_COEFFICIENT,
        ] {
            let record = verification_record(codex_id).expect("record should exist");
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert_eq!(record.codex_id, codex_id);
            assert!(record.has_sources());
            assert!(record.sources.contains(&SOURCE_ID_AERODYNAMIC_COEFFICIENTS));
        }
        assert!(verification_record("aero.unknown").is_none());
    }
}
