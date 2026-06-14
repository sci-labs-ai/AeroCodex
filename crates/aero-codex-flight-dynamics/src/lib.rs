#![forbid(unsafe_code)]
//! Phase 0.001 flight-dynamics and scalar performance primitives.
//!
//! This crate implements preliminary-design helpers for steady level
//! coordinated turns, wing stall speed, and specific excess power. Dimensional
//! inputs and outputs use SI units unless the parameter is explicitly
//! dimensionless.
//!
//! Phase 0.001 deliberately does not model trim, stability derivatives,
//! flight-control laws, thrust lapse, propulsion installation effects, climb
//! schedules, atmospheric variation, compressibility, maneuver envelopes,
//! weight change, trajectory integration, aircraft certification rules, or
//! vehicle-level safety margins. Traceability metadata remains conservative
//! `research_required` until exact source editions, equation identifiers,
//! reference examples, sign conventions, and tolerances are reviewed.

use aero_codex_core::{validation, AeroError, AeroResult, Angle, VerificationRecord};
use std::f64::consts::FRAC_PI_2;

/// Codex identifier for level coordinated-turn load factor, `n = 1/cos(phi)`.
pub const CODEX_ID_LEVEL_TURN_LOAD_FACTOR: &str = "flight_dynamics.turn.load_factor_level_turn";
/// Codex identifier for level coordinated-turn yaw/heading rate, `omega = g*tan(phi)/V`.
pub const CODEX_ID_TURN_RATE: &str = "flight_dynamics.turn.turn_rate";
/// Codex identifier for level coordinated-turn radius magnitude, `R = V^2/(g*|tan(phi)|)`.
pub const CODEX_ID_TURN_RADIUS: &str = "flight_dynamics.turn.turn_radius";
/// Codex identifier for clean scalar stall-speed estimate, `V_s = sqrt(2W/(rho*S*C_Lmax))`.
pub const CODEX_ID_STALL_SPEED: &str = "flight_dynamics.performance.stall_speed";
/// Codex identifier for specific excess power, `P_s = (T-D)*V/W`.
pub const CODEX_ID_SPECIFIC_EXCESS_POWER: &str =
    "flight_dynamics.performance.specific_excess_power";

/// Conservative source-registry ID for Phase 0.001 flight-dynamics performance review.
pub const SOURCE_ID_FLIGHT_DYNAMICS_BASIC_PERFORMANCE: &str =
    "source.flight_dynamics.basic_performance.research_required";
/// Descriptive alias for the level-turn/performance source-registry ID.
pub const SOURCE_ID_FLIGHT_DYNAMICS_LEVEL_TURN_PERFORMANCE: &str =
    SOURCE_ID_FLIGHT_DYNAMICS_BASIC_PERFORMANCE;
/// Short alias for the flight-dynamics source-registry ID.
pub const SOURCE_ID_FLIGHT_DYNAMICS_BASICS: &str = SOURCE_ID_FLIGHT_DYNAMICS_BASIC_PERFORMANCE;

const FLIGHT_DYNAMICS_SOURCES: &[&str] = &[SOURCE_ID_FLIGHT_DYNAMICS_BASIC_PERFORMANCE];
const LEVEL_TURN_COS_EPSILON: f64 = 1.0e-12;
const LEVEL_TURN_TAN_EPSILON: f64 = 1.0e-12;

/// Conservative traceability metadata for Phase 0.001 flight-dynamics helpers.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_LEVEL_TURN_LOAD_FACTOR => Some(VerificationRecord::research_required(
            CODEX_ID_LEVEL_TURN_LOAD_FACTOR,
            FLIGHT_DYNAMICS_SOURCES,
            "Steady level coordinated-turn load-factor relation implemented; exact source, sign convention, and reference examples pending.",
        )),
        CODEX_ID_TURN_RATE => Some(VerificationRecord::research_required(
            CODEX_ID_TURN_RATE,
            FLIGHT_DYNAMICS_SOURCES,
            "Steady level coordinated-turn rate relation implemented; bank-sign convention and source tolerances pending.",
        )),
        CODEX_ID_TURN_RADIUS => Some(VerificationRecord::research_required(
            CODEX_ID_TURN_RADIUS,
            FLIGHT_DYNAMICS_SOURCES,
            "Steady level coordinated-turn radius magnitude relation implemented; zero-bank handling and reference examples pending.",
        )),
        CODEX_ID_STALL_SPEED => Some(VerificationRecord::research_required(
            CODEX_ID_STALL_SPEED,
            FLIGHT_DYNAMICS_SOURCES,
            "Scalar lift-equals-weight stall-speed relation implemented; configuration, reference-area, and CLmax conventions pending source review.",
        )),
        CODEX_ID_SPECIFIC_EXCESS_POWER => Some(VerificationRecord::research_required(
            CODEX_ID_SPECIFIC_EXCESS_POWER,
            FLIGHT_DYNAMICS_SOURCES,
            "Specific-excess-power bookkeeping relation implemented; thrust/drag station conventions and representative examples pending.",
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
            "computed flight-dynamics result was not finite",
        ))
    }
}

fn ensure_positive_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed flight-dynamics result was not positive and finite",
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

fn validate_bank_angle(bank_angle: Angle) -> AeroResult<()> {
    let radians = bank_angle.as_radians();
    validation::ensure_finite("bank_angle", radians)?;

    if radians.abs() >= FRAC_PI_2 {
        return Err(AeroError::OutOfDomain {
            parameter: "bank_angle",
            value: bank_angle.as_degrees(),
            expected: "finite |bank_angle| < 90 degrees for level coordinated turn",
        });
    }

    let cos_phi = bank_angle.cos();
    if !(cos_phi.is_finite() && cos_phi > LEVEL_TURN_COS_EPSILON) {
        return Err(AeroError::OutOfDomain {
            parameter: "bank_angle",
            value: bank_angle.as_degrees(),
            expected: "bank-angle cosine sufficiently above zero for level coordinated turn",
        });
    }

    Ok(())
}

fn validate_nonzero_bank_angle_for_radius(bank_angle: Angle) -> AeroResult<f64> {
    validate_bank_angle(bank_angle)?;
    let tan_phi = bank_angle.tan();
    if !(tan_phi.is_finite() && tan_phi.abs() > LEVEL_TURN_TAN_EPSILON) {
        return Err(AeroError::OutOfDomain {
            parameter: "bank_angle",
            value: bank_angle.as_degrees(),
            expected: "finite nonzero bank angle for finite level-turn radius",
        });
    }
    Ok(tan_phi.abs())
}

/// Level coordinated-turn load factor, `n = 1/cos(phi)`.
///
/// `bank_angle` must be finite and strictly inside `(-90 deg, +90 deg)` so the
/// level-turn relation has a positive finite vertical lift component. The
/// returned value is a nonnegative load-factor magnitude. Load-limit,
/// structural-margin, and stall-margin checks are outside this helper.
pub fn load_factor_level_turn(bank_angle: Angle) -> AeroResult<f64> {
    validate_bank_angle(bank_angle)?;
    ensure_positive_finite_result(CODEX_ID_LEVEL_TURN_LOAD_FACTOR, 1.0 / bank_angle.cos())
}

/// Level coordinated-turn rate, `omega = g*tan(phi)/V`, in radians per second.
///
/// `g` and `velocity` must be finite and strictly positive. `bank_angle` must
/// be finite and strictly inside `(-90 deg, +90 deg)`. The sign of the turn
/// rate follows the sign of `bank_angle`; zero bank returns zero turn rate.
/// Wind, sideslip, trim, and nonlevel flight effects are outside this helper.
pub fn turn_rate(g: f64, velocity: f64, bank_angle: Angle) -> AeroResult<f64> {
    validation::ensure_positive("g", g)?;
    validation::ensure_positive("velocity", velocity)?;
    validate_bank_angle(bank_angle)?;

    let numerator = checked_product(
        CODEX_ID_TURN_RATE,
        "turn-rate g times tangent of bank angle was not finite",
        g,
        bank_angle.tan(),
    )?;
    ensure_finite_result(CODEX_ID_TURN_RATE, numerator / velocity)
}

/// Level coordinated-turn radius magnitude, `R = V^2/(g*|tan(phi)|)`, in metres.
///
/// `velocity` and `g` must be finite and strictly positive. `bank_angle` must
/// be finite, nonzero, and strictly inside `(-90 deg, +90 deg)`. The returned
/// value is a positive radius magnitude, not a signed curvature. Wind,
/// transient roll-in/roll-out, altitude changes, and navigation-frame effects
/// are outside this helper.
pub fn turn_radius(velocity: f64, g: f64, bank_angle: Angle) -> AeroResult<f64> {
    validation::ensure_positive("velocity", velocity)?;
    validation::ensure_positive("g", g)?;
    let tan_phi_abs = validate_nonzero_bank_angle_for_radius(bank_angle)?;

    let velocity_squared = checked_product(
        CODEX_ID_TURN_RADIUS,
        "turn-radius velocity squared was not finite",
        velocity,
        velocity,
    )?;
    let denominator = checked_product(
        CODEX_ID_TURN_RADIUS,
        "turn-radius g times tangent of bank angle was not finite",
        g,
        tan_phi_abs,
    )?;
    if denominator <= 0.0 {
        return Err(numerical_failure(
            CODEX_ID_TURN_RADIUS,
            "turn-radius denominator was not positive",
        ));
    }

    ensure_positive_finite_result(CODEX_ID_TURN_RADIUS, velocity_squared / denominator)
}

/// Scalar stall-speed estimate, `V_s = sqrt(2W/(rho*S*C_Lmax))`, in metres per second.
///
/// `weight`, `density`, `wing_area`, and `cl_max` must be finite and strictly
/// positive. This helper assumes lift equals weight at maximum lift coefficient
/// for a caller-selected configuration. Dynamic stall, compressibility,
/// Reynolds-number effects, load-factor stall, buffet, propulsion effects, and
/// certification stall definitions are outside Phase 0.001.
pub fn stall_speed(weight: f64, density: f64, wing_area: f64, cl_max: f64) -> AeroResult<f64> {
    validation::ensure_positive("weight", weight)?;
    validation::ensure_positive("density", density)?;
    validation::ensure_positive("wing_area", wing_area)?;
    validation::ensure_positive("cl_max", cl_max)?;

    let numerator = checked_product(
        CODEX_ID_STALL_SPEED,
        "stall-speed two times weight was not finite",
        2.0,
        weight,
    )?;
    let rho_area = checked_product(
        CODEX_ID_STALL_SPEED,
        "stall-speed density times wing area was not finite",
        density,
        wing_area,
    )?;
    let denominator = checked_product(
        CODEX_ID_STALL_SPEED,
        "stall-speed density-area product times CLmax was not finite",
        rho_area,
        cl_max,
    )?;
    if denominator <= 0.0 {
        return Err(numerical_failure(
            CODEX_ID_STALL_SPEED,
            "stall-speed denominator was not positive",
        ));
    }

    let radicand = numerator / denominator;
    if !(radicand.is_finite() && radicand > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_STALL_SPEED,
            "stall-speed radicand was not positive and finite",
        ));
    }
    ensure_positive_finite_result(CODEX_ID_STALL_SPEED, radicand.sqrt())
}

/// Specific excess power, `P_s = (T-D)*V/W`, in metres per second.
///
/// `thrust` and `drag` are finite signed scalars so callers can preserve their
/// bookkeeping convention. `velocity` must be finite and nonnegative; `weight`
/// must be finite and strictly positive. Zero velocity returns zero specific
/// excess power. Engine lapse, installed thrust, climb schedule, acceleration,
/// atmospheric variation, and energy-state trajectory integration are outside
/// this scalar helper.
pub fn specific_excess_power(
    thrust: f64,
    drag: f64,
    velocity: f64,
    weight: f64,
) -> AeroResult<f64> {
    validation::ensure_finite("thrust", thrust)?;
    validation::ensure_finite("drag", drag)?;
    validation::ensure_nonnegative("velocity", velocity)?;
    validation::ensure_positive("weight", weight)?;

    let excess_force = thrust - drag;
    if !excess_force.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_SPECIFIC_EXCESS_POWER,
            "specific-excess-power thrust minus drag was not finite",
        ));
    }
    let numerator = checked_product(
        CODEX_ID_SPECIFIC_EXCESS_POWER,
        "specific-excess-power excess force times velocity was not finite",
        excess_force,
        velocity,
    )?;
    ensure_finite_result(CODEX_ID_SPECIFIC_EXCESS_POWER, numerator / weight)
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
    fn load_factor_matches_level_turn_formula() {
        assert_close(
            load_factor_level_turn(Angle::from_degrees(0.0)).unwrap(),
            1.0,
            1.0e-12,
        );
        assert_close(
            load_factor_level_turn(Angle::from_degrees(60.0)).unwrap(),
            2.0,
            1.0e-12,
        );
        assert_close(
            load_factor_level_turn(Angle::from_degrees(-60.0)).unwrap(),
            2.0,
            1.0e-12,
        );
    }

    #[test]
    fn load_factor_rejects_invalid_bank_angles() {
        assert!(load_factor_level_turn(Angle::from_degrees(90.0)).is_err());
        assert!(load_factor_level_turn(Angle::from_degrees(91.0)).is_err());
        assert!(load_factor_level_turn(Angle::from_degrees(360.0)).is_err());
        assert!(load_factor_level_turn(Angle::from_radians(f64::NAN)).is_err());
    }

    #[test]
    fn turn_rate_matches_formula_and_preserves_bank_sign() {
        let g = 9.81;
        let velocity = 100.0;
        let positive = turn_rate(g, velocity, Angle::from_degrees(30.0)).unwrap();
        let negative = turn_rate(g, velocity, Angle::from_degrees(-30.0)).unwrap();
        let expected = g * Angle::from_degrees(30.0).tan() / velocity;
        assert_close(positive, expected, 1.0e-12);
        assert_close(negative, -expected, 1.0e-12);
        assert_close(
            turn_rate(g, velocity, Angle::from_degrees(0.0)).unwrap(),
            0.0,
            1.0e-12,
        );
    }

    #[test]
    fn turn_rate_rejects_invalid_inputs_and_overflow() {
        assert!(turn_rate(0.0, 100.0, Angle::from_degrees(30.0)).is_err());
        assert!(turn_rate(9.81, 0.0, Angle::from_degrees(30.0)).is_err());
        assert!(turn_rate(9.81, 100.0, Angle::from_degrees(90.0)).is_err());
        assert!(matches!(
            turn_rate(f64::MAX, f64::MIN_POSITIVE, Angle::from_degrees(45.0)),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn turn_radius_matches_formula_and_uses_magnitude() {
        let velocity = 100.0;
        let g = 9.81;
        let expected = velocity * velocity / (g * Angle::from_degrees(30.0).tan());
        assert_close(
            turn_radius(velocity, g, Angle::from_degrees(30.0)).unwrap(),
            expected,
            1.0e-9,
        );
        assert_close(
            turn_radius(velocity, g, Angle::from_degrees(-30.0)).unwrap(),
            expected,
            1.0e-9,
        );
    }

    #[test]
    fn turn_radius_decreases_with_bank_angle() {
        let shallow = turn_radius(100.0, 9.81, Angle::from_degrees(20.0)).unwrap();
        let steep = turn_radius(100.0, 9.81, Angle::from_degrees(45.0)).unwrap();

        assert!(steep < shallow);
    }

    #[test]
    fn turn_radius_rejects_zero_bank_invalid_inputs_and_overflow() {
        assert!(turn_radius(100.0, 9.81, Angle::from_degrees(0.0)).is_err());
        assert!(turn_radius(0.0, 9.81, Angle::from_degrees(30.0)).is_err());
        assert!(turn_radius(100.0, 0.0, Angle::from_degrees(30.0)).is_err());
        assert!(turn_radius(100.0, 9.81, Angle::from_degrees(90.0)).is_err());
        assert!(matches!(
            turn_radius(f64::MAX, 9.81, Angle::from_degrees(45.0)),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn stall_speed_matches_lift_equals_weight_formula() {
        let weight: f64 = 10_000.0;
        let density: f64 = 1.225;
        let wing_area: f64 = 16.0;
        let cl_max: f64 = 1.5;
        let expected = (2.0 * weight / (density * wing_area * cl_max)).sqrt();
        assert_close(
            stall_speed(weight, density, wing_area, cl_max).unwrap(),
            expected,
            1.0e-12,
        );
    }

    #[test]
    fn stall_speed_increases_with_weight() {
        let light = stall_speed(10_000.0, 1.225, 16.0, 1.5).unwrap();
        let heavy = stall_speed(20_000.0, 1.225, 16.0, 1.5).unwrap();
        assert!(heavy > light);
    }

    #[test]
    fn stall_speed_rejects_invalid_domains_and_nonfinite_outputs() {
        assert!(stall_speed(0.0, 1.225, 16.0, 1.5).is_err());
        assert!(stall_speed(10_000.0, 0.0, 16.0, 1.5).is_err());
        assert!(stall_speed(10_000.0, 1.225, 0.0, 1.5).is_err());
        assert!(stall_speed(10_000.0, 1.225, 16.0, 0.0).is_err());
        assert!(matches!(
            stall_speed(
                f64::MAX,
                f64::MIN_POSITIVE,
                f64::MIN_POSITIVE,
                f64::MIN_POSITIVE
            ),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn specific_excess_power_tracks_excess_force_and_velocity() {
        assert_close(
            specific_excess_power(2_000.0, 1_000.0, 100.0, 10_000.0).unwrap(),
            10.0,
            1.0e-12,
        );
        assert!(specific_excess_power(1_000.0, 2_000.0, 100.0, 10_000.0).unwrap() < 0.0);
        assert_close(
            specific_excess_power(2_000.0, 1_000.0, 0.0, 10_000.0).unwrap(),
            0.0,
            1.0e-12,
        );
    }

    #[test]
    fn specific_excess_power_rejects_invalid_inputs_and_overflow() {
        assert!(specific_excess_power(f64::NAN, 1_000.0, 100.0, 10_000.0).is_err());
        assert!(specific_excess_power(2_000.0, f64::NAN, 100.0, 10_000.0).is_err());
        assert!(specific_excess_power(2_000.0, 1_000.0, -1.0, 10_000.0).is_err());
        assert!(specific_excess_power(2_000.0, 1_000.0, 100.0, 0.0).is_err());
        assert!(matches!(
            specific_excess_power(f64::MAX, -f64::MAX, 1.0, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn flight_dynamics_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_LEVEL_TURN_LOAD_FACTOR,
            CODEX_ID_TURN_RATE,
            CODEX_ID_TURN_RADIUS,
            CODEX_ID_STALL_SPEED,
            CODEX_ID_SPECIFIC_EXCESS_POWER,
        ] {
            let record =
                verification_record(codex_id).expect("flight-dynamics verification record");
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert!(record.has_sources());
            assert_eq!(record.sources, FLIGHT_DYNAMICS_SOURCES);
        }
        assert!(verification_record("flight_dynamics.unknown").is_none());
    }
}
