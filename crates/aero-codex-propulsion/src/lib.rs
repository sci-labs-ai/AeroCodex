#![forbid(unsafe_code)]
//! Phase 0.001 rocket and nozzle basics.
//!
//! This crate implements scalar preliminary-design propulsion helpers for the
//! Tsiolkovsky rocket equation, ideal thrust bookkeeping, specific impulse from
//! effective exhaust velocity, and ideal choked mass flux per throat area.
//! Dimensional inputs and outputs use SI units.
//!
//! Phase 0.001 deliberately does not model combustion chemistry, nozzle contour
//! losses, finite-rate chemistry, real-gas effects, injector/feed systems,
//! turbomachinery, trajectory integration, throttle schedules, or engine-cycle
//! simulation. Traceability metadata remains conservative `research_required`
//! until exact source editions, equation identifiers, pressure-thrust sign
//! conventions, reference examples, and tolerances are reviewed.

use aero_codex_core::{validation, AeroError, AeroResult, VerificationRecord};

/// Codex identifier for the Tsiolkovsky ideal rocket equation.
pub const CODEX_ID_TSIOLKOVSKY_DELTA_V: &str = "propulsion.rocket.tsiolkovsky_delta_v";
/// Codex identifier for mass ratio from ideal delta-v.
pub const CODEX_ID_MASS_RATIO_FROM_DELTA_V: &str = "propulsion.rocket.mass_ratio_from_delta_v";
/// Codex identifier for ideal thrust with pressure-thrust bookkeeping.
pub const CODEX_ID_IDEAL_THRUST: &str = "propulsion.nozzle.ideal_thrust";
/// Codex identifier for specific impulse from effective exhaust velocity.
pub const CODEX_ID_SPECIFIC_IMPULSE_FROM_EFFECTIVE_EXHAUST_VELOCITY: &str =
    "propulsion.rocket.specific_impulse_from_effective_exhaust_velocity";
/// Short alias for the effective-exhaust-velocity specific-impulse Codex ID.
pub const CODEX_ID_SPECIFIC_IMPULSE: &str =
    CODEX_ID_SPECIFIC_IMPULSE_FROM_EFFECTIVE_EXHAUST_VELOCITY;
/// Codex identifier for ideal choked mass flux per throat area.
pub const CODEX_ID_CHOKED_MASS_FLUX: &str = "propulsion.nozzle.choked_mass_flux_per_area";
/// Descriptive alias for the choked mass-flux Codex ID.
pub const CODEX_ID_CHOKED_MASS_FLUX_PER_AREA: &str = CODEX_ID_CHOKED_MASS_FLUX;

/// Conservative source-registry ID for Phase 0.001 rocket/nozzle review.
pub const SOURCE_ID_PROPULSION_ROCKET_NOZZLE_BASICS: &str =
    "source.propulsion.rocket_nozzle_basics.research_required";
/// Short alias for the propulsion rocket/nozzle source-registry ID.
pub const SOURCE_ID_PROPULSION_ROCKET_NOZZLE: &str = SOURCE_ID_PROPULSION_ROCKET_NOZZLE_BASICS;

const PROPULSION_SOURCES: &[&str] = &[SOURCE_ID_PROPULSION_ROCKET_NOZZLE_BASICS];

/// Conservative traceability metadata for Phase 0.001 propulsion helpers.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_TSIOLKOVSKY_DELTA_V => Some(VerificationRecord::research_required(
            CODEX_ID_TSIOLKOVSKY_DELTA_V,
            PROPULSION_SOURCES,
            "Ideal rocket-equation relation implemented; exact source and assumptions review pending.",
        )),
        CODEX_ID_MASS_RATIO_FROM_DELTA_V => Some(VerificationRecord::research_required(
            CODEX_ID_MASS_RATIO_FROM_DELTA_V,
            PROPULSION_SOURCES,
            "Ideal mass-ratio inverse relation implemented; source examples and tolerances pending.",
        )),
        CODEX_ID_IDEAL_THRUST => Some(VerificationRecord::research_required(
            CODEX_ID_IDEAL_THRUST,
            PROPULSION_SOURCES,
            "Ideal momentum plus pressure-thrust bookkeeping implemented; pressure sign convention review pending.",
        )),
        CODEX_ID_SPECIFIC_IMPULSE_FROM_EFFECTIVE_EXHAUST_VELOCITY => {
            Some(VerificationRecord::research_required(
                CODEX_ID_SPECIFIC_IMPULSE_FROM_EFFECTIVE_EXHAUST_VELOCITY,
                PROPULSION_SOURCES,
                "Specific impulse from effective exhaust velocity implemented; standard-gravity convention review pending.",
            ))
        }
        CODEX_ID_CHOKED_MASS_FLUX => Some(VerificationRecord::research_required(
            CODEX_ID_CHOKED_MASS_FLUX,
            PROPULSION_SOURCES,
            "Ideal calorically-perfect-gas choked mass-flux relation implemented; nozzle assumptions and validation pending.",
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
            "computed propulsion result was not finite",
        ))
    }
}

fn ensure_positive_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed propulsion result was not positive and finite",
        ))
    }
}

fn ensure_nonnegative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed propulsion result was not nonnegative and finite",
        ))
    }
}

fn exhaust_velocity_scale(codex_id: &'static str, isp: f64, g0: f64) -> AeroResult<f64> {
    validation::ensure_positive("isp", isp)?;
    validation::ensure_positive("g0", g0)?;

    let scale = isp * g0;
    if scale.is_finite() && scale > 0.0 {
        Ok(scale)
    } else {
        Err(numerical_failure(
            codex_id,
            "specific impulse times standard gravity was not finite and positive",
        ))
    }
}

/// Ideal rocket-equation delta-v, `DeltaV = Isp*g0*ln(m0/mf)`, in m/s.
///
/// Inputs are scalar SI values: `isp` in seconds, `g0` in m/s², and masses in
/// kilograms or any consistent positive mass unit. The function requires
/// `initial_mass > final_mass > 0`, so it returns positive delta-v only. Staging,
/// gravity losses, drag losses, steering losses, finite burn effects, and
/// trajectory integration are outside this helper.
pub fn tsiolkovsky_delta_v(
    isp: f64,
    g0: f64,
    initial_mass: f64,
    final_mass: f64,
) -> AeroResult<f64> {
    let exhaust_velocity = exhaust_velocity_scale(CODEX_ID_TSIOLKOVSKY_DELTA_V, isp, g0)?;
    validation::ensure_positive("initial_mass", initial_mass)?;
    validation::ensure_positive("final_mass", final_mass)?;

    if initial_mass <= final_mass {
        return Err(AeroError::OutOfDomain {
            parameter: "initial_mass",
            value: initial_mass,
            expected: "initial_mass > final_mass > 0 for positive delta-v",
        });
    }

    let mass_ratio = initial_mass / final_mass;
    if !(mass_ratio.is_finite() && mass_ratio > 1.0) {
        return Err(numerical_failure(
            CODEX_ID_TSIOLKOVSKY_DELTA_V,
            "initial-to-final mass ratio was not finite and greater than one",
        ));
    }

    ensure_positive_finite_result(
        CODEX_ID_TSIOLKOVSKY_DELTA_V,
        exhaust_velocity * mass_ratio.ln(),
    )
}

/// Ideal mass ratio from delta-v, `m0/mf = exp(DeltaV/(Isp*g0))`.
///
/// `delta_v` must be finite and nonnegative. A zero delta-v returns a mass ratio
/// of one. Positive mission losses and staging models are outside this inverse
/// scalar helper.
pub fn mass_ratio_from_delta_v(delta_v: f64, isp: f64, g0: f64) -> AeroResult<f64> {
    validation::ensure_nonnegative("delta_v", delta_v)?;
    let exhaust_velocity = exhaust_velocity_scale(CODEX_ID_MASS_RATIO_FROM_DELTA_V, isp, g0)?;

    let exponent = delta_v / exhaust_velocity;
    if !(exponent.is_finite() && exponent >= 0.0) {
        return Err(numerical_failure(
            CODEX_ID_MASS_RATIO_FROM_DELTA_V,
            "delta-v over effective exhaust velocity was not finite and nonnegative",
        ));
    }

    ensure_positive_finite_result(CODEX_ID_MASS_RATIO_FROM_DELTA_V, exponent.exp())
}

/// Ideal thrust bookkeeping, `F = mdot*Ve + (pe - pa)*Ae`, in newtons.
///
/// `mass_flow`, `exit_velocity`, `exit_pressure`, `ambient_pressure`, and
/// `exit_area` must be finite and nonnegative. The returned thrust may be signed
/// because the pressure-thrust term can be negative when ambient pressure exceeds
/// exit pressure; caller conventions must document how signed axial force is used.
pub fn ideal_thrust(
    mass_flow: f64,
    exit_velocity: f64,
    exit_pressure: f64,
    ambient_pressure: f64,
    exit_area: f64,
) -> AeroResult<f64> {
    validation::ensure_nonnegative("mass_flow", mass_flow)?;
    validation::ensure_nonnegative("exit_velocity", exit_velocity)?;
    validation::ensure_nonnegative("exit_pressure", exit_pressure)?;
    validation::ensure_nonnegative("ambient_pressure", ambient_pressure)?;
    validation::ensure_nonnegative("exit_area", exit_area)?;

    let momentum_thrust = mass_flow * exit_velocity;
    if !momentum_thrust.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_IDEAL_THRUST,
            "mass-flow times exit velocity was not finite",
        ));
    }

    let pressure_delta = exit_pressure - ambient_pressure;
    if !pressure_delta.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_IDEAL_THRUST,
            "exit pressure minus ambient pressure was not finite",
        ));
    }

    let pressure_thrust = pressure_delta * exit_area;
    if !pressure_thrust.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_IDEAL_THRUST,
            "pressure-difference times exit area was not finite",
        ));
    }

    ensure_finite_result(CODEX_ID_IDEAL_THRUST, momentum_thrust + pressure_thrust)
}

/// Specific impulse from effective exhaust velocity, `Isp = c/g0`, in seconds.
///
/// The effective exhaust velocity `c` must be positive and in m/s. The standard
/// gravity value `g0` is supplied by the caller so later source review can lock
/// the convention explicitly.
pub fn specific_impulse_from_effective_exhaust_velocity(c: f64, g0: f64) -> AeroResult<f64> {
    validation::ensure_positive("effective_exhaust_velocity", c)?;
    validation::ensure_positive("g0", g0)?;

    ensure_positive_finite_result(
        CODEX_ID_SPECIFIC_IMPULSE_FROM_EFFECTIVE_EXHAUST_VELOCITY,
        c / g0,
    )
}

/// Ideal choked mass flux per area for a calorically perfect gas.
///
/// Implements `mdot/A = p0*sqrt(gamma/(R*T0)) * (2/(gamma+1))^((gamma+1)/(2*(gamma-1)))`.
/// The function requires `gamma > 1`, positive gas constant, nonnegative
/// stagnation pressure, and positive stagnation temperature. It returns zero for
/// zero stagnation pressure and does not model discharge coefficient, boundary
/// layer effects, real-gas chemistry, two-phase flow, or nozzle geometry losses.
pub fn choked_mass_flux_per_area(
    gamma: f64,
    gas_constant: f64,
    stagnation_pressure: f64,
    stagnation_temperature: f64,
) -> AeroResult<f64> {
    validation::ensure_greater_than("gamma", gamma, 1.0)?;
    validation::ensure_positive("gas_constant", gas_constant)?;
    validation::ensure_nonnegative("stagnation_pressure", stagnation_pressure)?;
    validation::ensure_positive("stagnation_temperature", stagnation_temperature)?;

    if stagnation_pressure <= 0.0 {
        return Ok(0.0);
    }

    let gas_temperature_product = gas_constant * stagnation_temperature;
    if !(gas_temperature_product.is_finite() && gas_temperature_product > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_CHOKED_MASS_FLUX,
            "gas constant times stagnation temperature was not finite and positive",
        ));
    }

    let sqrt_argument = gamma / gas_temperature_product;
    if !(sqrt_argument.is_finite() && sqrt_argument > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_CHOKED_MASS_FLUX,
            "choked-flow square-root argument was not finite and positive",
        ));
    }

    let pressure_temperature = stagnation_pressure * sqrt_argument.sqrt();
    if !(pressure_temperature.is_finite() && pressure_temperature >= 0.0) {
        return Err(numerical_failure(
            CODEX_ID_CHOKED_MASS_FLUX,
            "stagnation pressure times thermal factor was not finite and nonnegative",
        ));
    }

    let choking_base = 2.0 / (gamma + 1.0);
    let choking_exponent = (gamma + 1.0) / (2.0 * (gamma - 1.0));
    if !(choking_base.is_finite() && choking_base > 0.0 && choking_exponent.is_finite()) {
        return Err(numerical_failure(
            CODEX_ID_CHOKED_MASS_FLUX,
            "choking exponent or base was not finite and valid",
        ));
    }

    let choking_factor = choking_base.powf(choking_exponent);
    if !(choking_factor.is_finite() && choking_factor > 0.0) {
        return Err(numerical_failure(
            CODEX_ID_CHOKED_MASS_FLUX,
            "choking factor was not finite and positive",
        ));
    }

    ensure_nonnegative_finite_result(
        CODEX_ID_CHOKED_MASS_FLUX,
        pressure_temperature * choking_factor,
    )
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
    fn delta_v_and_mass_ratio_are_inverse() {
        let dv = tsiolkovsky_delta_v(300.0, 9.80665, 1_000.0, 500.0).unwrap();
        assert_close(dv, 300.0 * 9.80665 * (2.0_f64).ln(), 1.0e-10);
        assert_close(
            mass_ratio_from_delta_v(dv, 300.0, 9.80665).unwrap(),
            2.0,
            1.0e-12,
        );
    }

    #[test]
    fn delta_v_rejects_invalid_mass_order_or_nonpositive_inputs() {
        assert!(tsiolkovsky_delta_v(300.0, 9.80665, 500.0, 500.0).is_err());
        assert!(tsiolkovsky_delta_v(300.0, 9.80665, 400.0, 500.0).is_err());
        assert!(tsiolkovsky_delta_v(0.0, 9.80665, 1_000.0, 500.0).is_err());
        assert!(tsiolkovsky_delta_v(300.0, -9.80665, 1_000.0, 500.0).is_err());
        assert!(tsiolkovsky_delta_v(300.0, 9.80665, f64::NAN, 500.0).is_err());
    }

    #[test]
    fn mass_ratio_zero_delta_v_boundary_and_invalid_inputs() {
        assert_close(
            mass_ratio_from_delta_v(0.0, 300.0, 9.80665).unwrap(),
            1.0,
            0.0,
        );
        assert!(mass_ratio_from_delta_v(-1.0, 300.0, 9.80665).is_err());
        assert!(mass_ratio_from_delta_v(10.0, 0.0, 9.80665).is_err());
        assert!(mass_ratio_from_delta_v(10.0, 300.0, f64::INFINITY).is_err());
    }

    #[test]
    fn thrust_includes_momentum_and_pressure_terms() {
        let thrust = ideal_thrust(10.0, 2_000.0, 120_000.0, 100_000.0, 0.5).unwrap();
        assert_close(thrust, 30_000.0, 1.0e-12);
    }

    #[test]
    fn pressure_thrust_can_be_negative_by_convention() {
        let thrust = ideal_thrust(0.0, 0.0, 80_000.0, 100_000.0, 0.5).unwrap();
        assert_close(thrust, -10_000.0, 1.0e-12);
    }

    #[test]
    fn ideal_thrust_rejects_negative_or_nonfinite_inputs() {
        assert!(ideal_thrust(-1.0, 2_000.0, 120_000.0, 100_000.0, 0.5).is_err());
        assert!(ideal_thrust(10.0, -1.0, 120_000.0, 100_000.0, 0.5).is_err());
        assert!(ideal_thrust(10.0, 2_000.0, -1.0, 100_000.0, 0.5).is_err());
        assert!(ideal_thrust(10.0, 2_000.0, 120_000.0, f64::NAN, 0.5).is_err());
        assert!(ideal_thrust(10.0, 2_000.0, 120_000.0, 100_000.0, -0.5).is_err());
    }

    #[test]
    fn specific_impulse_from_effective_exhaust_velocity_formula() {
        assert_close(
            specific_impulse_from_effective_exhaust_velocity(2_941.995, 9.80665).unwrap(),
            300.0,
            1.0e-12,
        );
        assert!(specific_impulse_from_effective_exhaust_velocity(0.0, 9.80665).is_err());
        assert!(specific_impulse_from_effective_exhaust_velocity(2_941.995, 0.0).is_err());
    }

    #[test]
    fn choked_flux_is_positive_and_zero_pressure_returns_zero() {
        assert!(choked_mass_flux_per_area(1.4, 287.0, 101_325.0, 300.0).unwrap() > 0.0);
        assert_close(
            choked_mass_flux_per_area(1.4, 287.0, 0.0, 300.0).unwrap(),
            0.0,
            0.0,
        );
    }

    #[test]
    fn choked_flux_rejects_invalid_inputs() {
        assert!(choked_mass_flux_per_area(1.0, 287.0, 101_325.0, 300.0).is_err());
        assert!(choked_mass_flux_per_area(1.4, 0.0, 101_325.0, 300.0).is_err());
        assert!(choked_mass_flux_per_area(1.4, 287.0, -1.0, 300.0).is_err());
        assert!(choked_mass_flux_per_area(1.4, 287.0, 101_325.0, 0.0).is_err());
        assert!(choked_mass_flux_per_area(f64::INFINITY, 287.0, 101_325.0, 300.0).is_err());
    }

    #[test]
    fn nonfinite_derived_outputs_return_numerical_failure() {
        assert_numerical_failure(tsiolkovsky_delta_v(1.0, 1.0, f64::MAX, 1.0e-308));
        assert_numerical_failure(mass_ratio_from_delta_v(f64::MAX, 1.0, 1.0));
        assert_numerical_failure(ideal_thrust(f64::MAX, f64::MAX, 0.0, 0.0, 0.0));
        assert_numerical_failure(specific_impulse_from_effective_exhaust_velocity(
            f64::MAX,
            1.0e-308,
        ));
        assert_numerical_failure(choked_mass_flux_per_area(1.4, 1.0, f64::MAX, 1.0));
    }

    #[test]
    fn propulsion_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_TSIOLKOVSKY_DELTA_V,
            CODEX_ID_MASS_RATIO_FROM_DELTA_V,
            CODEX_ID_IDEAL_THRUST,
            CODEX_ID_SPECIFIC_IMPULSE_FROM_EFFECTIVE_EXHAUST_VELOCITY,
            CODEX_ID_CHOKED_MASS_FLUX_PER_AREA,
        ] {
            let record = verification_record(codex_id).expect("record should exist");
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert_eq!(record.codex_id, codex_id);
            assert!(record.has_sources());
            assert!(record
                .sources
                .contains(&SOURCE_ID_PROPULSION_ROCKET_NOZZLE_BASICS));
        }
        assert!(verification_record("propulsion.unknown").is_none());
    }
}
