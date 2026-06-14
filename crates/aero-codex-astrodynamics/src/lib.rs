#![forbid(unsafe_code)]
//! Phase 0.001 two-body astrodynamics primitives.
//!
//! This crate implements a small set of scalar, SI-unit, two-body helpers for
//! preliminary astrodynamics calculations. Microtask 18 reviewed the core
//! circular-orbit, escape-speed, vis-viva, and specific-energy equations.
//! Microtask 19 reviews the first Hohmann-transfer and scalar
//! sphere-of-influence helpers.
//!
//! Phase 0.001 does not model perturbations, non-spherical gravity, third-body
//! effects, finite burns, maneuver losses, launch windows, atmosphere,
//! ephemerides, coordinate frames, time standards, navigation covariance, or
//! mission-design constraints. Traceability metadata remains conservative
//! `research_required` until exact source editions, equation identifiers,
//! reference examples, constants, and tolerances are reviewed.

use aero_codex_core::{validation, AeroError, AeroResult, VerificationRecord};
use std::f64::consts::PI;

/// Codex identifier for circular-orbit speed, `v = sqrt(mu/r)`.
pub const CODEX_ID_CIRCULAR_ORBIT_SPEED: &str = "astrodynamics.two_body.circular_orbit_speed";
/// Codex identifier for circular-orbit period, `T = 2*pi*sqrt(r^3/mu)`.
pub const CODEX_ID_ORBITAL_PERIOD_CIRCULAR: &str = "astrodynamics.two_body.orbital_period_circular";
/// Codex identifier for escape velocity, `v_e = sqrt(2*mu/r)`.
pub const CODEX_ID_ESCAPE_VELOCITY: &str = "astrodynamics.two_body.escape_velocity";
/// Codex identifier for vis-viva speed, `v = sqrt(mu*(2/r - 1/a))`.
pub const CODEX_ID_VIS_VIVA: &str = "astrodynamics.two_body.vis_viva";
/// Codex identifier for elliptical specific orbital energy, `epsilon = -mu/(2*a)`.
pub const CODEX_ID_SPECIFIC_ORBITAL_ENERGY: &str = "astrodynamics.two_body.specific_orbital_energy";
/// Codex identifier for the first Hohmann-transfer impulse magnitude.
pub const CODEX_ID_HOHMANN_TRANSFER_DELTA_V1: &str = "astrodynamics.transfer.hohmann.delta_v1";
/// Backward-compatible short alias for the first Hohmann impulse magnitude.
pub const CODEX_ID_HOHMANN_DELTA_V1: &str = CODEX_ID_HOHMANN_TRANSFER_DELTA_V1;
/// Codex identifier for the second Hohmann-transfer impulse magnitude.
pub const CODEX_ID_HOHMANN_TRANSFER_DELTA_V2: &str = "astrodynamics.transfer.hohmann.delta_v2";
/// Backward-compatible short alias for the second Hohmann impulse magnitude.
pub const CODEX_ID_HOHMANN_DELTA_V2: &str = CODEX_ID_HOHMANN_TRANSFER_DELTA_V2;
/// Codex identifier for Hohmann-transfer total delta-v magnitude.
pub const CODEX_ID_HOHMANN_TOTAL_DELTA_V: &str = "astrodynamics.transfer.hohmann.total_delta_v";
/// Codex identifier for Hohmann half-transfer-ellipse time of flight.
pub const CODEX_ID_HOHMANN_TRANSFER_TIME: &str = "astrodynamics.transfer.hohmann.transfer_time";
/// Codex identifier for sphere-of-influence radius.
pub const CODEX_ID_SPHERE_OF_INFLUENCE: &str = "astrodynamics.celestial.sphere_of_influence_radius";

/// Conservative source-registry ID for Phase 0.001 astrodynamics review.
pub const SOURCE_ID_ASTRODYNAMICS_NASA_JPL_PARAMETERS: &str =
    aero_codex_constants::SOURCE_ID_NASA_JPL_ASTRODYNAMICS_PARAMETERS;
/// Alias matching the source-registry filename used for Phase 0.001 astrodynamics seeds.
pub const SOURCE_ID_NASA_JPL_ASTRODYNAMICS_PARAMETERS: &str =
    SOURCE_ID_ASTRODYNAMICS_NASA_JPL_PARAMETERS;
/// Conservative source-registry ID for Phase 0.001 two-body equation-source review.
pub const SOURCE_ID_ASTRODYNAMICS_TWO_BODY_BASICS: &str =
    "source.astrodynamics.two_body_basics.research_required";
/// Short alias for the two-body source-registry seed.
pub const SOURCE_ID_ASTRODYNAMICS_BASICS: &str = SOURCE_ID_ASTRODYNAMICS_TWO_BODY_BASICS;
/// Conservative source-registry ID for Hohmann and scalar celestial helper review.
pub const SOURCE_ID_ASTRODYNAMICS_TRANSFER_CELESTIAL_BASICS: &str =
    "source.astrodynamics.transfer_celestial_basics.research_required";
/// Backward-compatible alias for the Hohmann/sphere-of-influence source seed.
pub const SOURCE_ID_ASTRODYNAMICS_HOHMANN_CELESTIAL_HELPERS: &str =
    SOURCE_ID_ASTRODYNAMICS_TRANSFER_CELESTIAL_BASICS;
/// Short alias for the Microtask 19 transfer/celestial source-registry seed.
pub const SOURCE_ID_ASTRODYNAMICS_HOHMANN_SOI_BASICS: &str =
    SOURCE_ID_ASTRODYNAMICS_TRANSFER_CELESTIAL_BASICS;
/// Short alias for the transfer/celestial source-registry seed.
pub const SOURCE_ID_ASTRODYNAMICS_TRANSFER_BASICS: &str =
    SOURCE_ID_ASTRODYNAMICS_TRANSFER_CELESTIAL_BASICS;

const ASTRODYNAMICS_TWO_BODY_SOURCES: &[&str] = &[
    SOURCE_ID_ASTRODYNAMICS_TWO_BODY_BASICS,
    SOURCE_ID_ASTRODYNAMICS_NASA_JPL_PARAMETERS,
];

const ASTRODYNAMICS_TRANSFER_CELESTIAL_SOURCES: &[&str] = &[
    SOURCE_ID_ASTRODYNAMICS_TRANSFER_CELESTIAL_BASICS,
    SOURCE_ID_ASTRODYNAMICS_TWO_BODY_BASICS,
    SOURCE_ID_ASTRODYNAMICS_NASA_JPL_PARAMETERS,
];

/// Conservative traceability metadata for Phase 0.001 astrodynamics helpers.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_CIRCULAR_ORBIT_SPEED => Some(VerificationRecord::research_required(
            CODEX_ID_CIRCULAR_ORBIT_SPEED,
            ASTRODYNAMICS_TWO_BODY_SOURCES,
            "Circular two-body orbit-speed relation implemented; exact source equation, constants, and reference examples pending.",
        )),
        CODEX_ID_ORBITAL_PERIOD_CIRCULAR => Some(VerificationRecord::research_required(
            CODEX_ID_ORBITAL_PERIOD_CIRCULAR,
            ASTRODYNAMICS_TWO_BODY_SOURCES,
            "Circular two-body period relation implemented; exact source equation, radius convention, and tolerances pending.",
        )),
        CODEX_ID_ESCAPE_VELOCITY => Some(VerificationRecord::research_required(
            CODEX_ID_ESCAPE_VELOCITY,
            ASTRODYNAMICS_TWO_BODY_SOURCES,
            "Two-body escape-speed relation implemented; exact source equation, radius convention, and reference examples pending.",
        )),
        CODEX_ID_VIS_VIVA => Some(VerificationRecord::research_required(
            CODEX_ID_VIS_VIVA,
            ASTRODYNAMICS_TWO_BODY_SOURCES,
            "Elliptic-orbit vis-viva relation implemented for positive semi-major axis; source equation and validation values pending.",
        )),
        CODEX_ID_SPECIFIC_ORBITAL_ENERGY => Some(VerificationRecord::research_required(
            CODEX_ID_SPECIFIC_ORBITAL_ENERGY,
            ASTRODYNAMICS_TWO_BODY_SOURCES,
            "Elliptic specific-orbital-energy relation implemented for positive semi-major axis; source locator and reference values pending.",
        )),
        CODEX_ID_HOHMANN_TRANSFER_DELTA_V1 => Some(VerificationRecord::research_required(
            CODEX_ID_HOHMANN_TRANSFER_DELTA_V1,
            ASTRODYNAMICS_TRANSFER_CELESTIAL_SOURCES,
            "First Hohmann-transfer impulse magnitude implemented for positive circular-orbit radii; source equation, sign convention, and reference cases pending.",
        )),
        CODEX_ID_HOHMANN_TRANSFER_DELTA_V2 => Some(VerificationRecord::research_required(
            CODEX_ID_HOHMANN_TRANSFER_DELTA_V2,
            ASTRODYNAMICS_TRANSFER_CELESTIAL_SOURCES,
            "Second Hohmann-transfer impulse magnitude implemented for positive circular-orbit radii; source equation, sign convention, and reference cases pending.",
        )),
        CODEX_ID_HOHMANN_TOTAL_DELTA_V => Some(VerificationRecord::research_required(
            CODEX_ID_HOHMANN_TOTAL_DELTA_V,
            ASTRODYNAMICS_TRANSFER_CELESTIAL_SOURCES,
            "Total two-impulse Hohmann-transfer delta-v magnitude implemented; transfer assumptions and validation values pending.",
        )),
        CODEX_ID_HOHMANN_TRANSFER_TIME => Some(VerificationRecord::research_required(
            CODEX_ID_HOHMANN_TRANSFER_TIME,
            ASTRODYNAMICS_TRANSFER_CELESTIAL_SOURCES,
            "Half-period Hohmann transfer time implemented for positive radii; exact source locator and timing convention pending.",
        )),
        CODEX_ID_SPHERE_OF_INFLUENCE => Some(VerificationRecord::research_required(
            CODEX_ID_SPHERE_OF_INFLUENCE,
            ASTRODYNAMICS_TRANSFER_CELESTIAL_SOURCES,
            "Laplace-style scalar sphere-of-influence radius helper implemented; source form, mass convention, and representative examples pending.",
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

fn ensure_nonnegative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed astrodynamics result was not nonnegative and finite",
        ))
    }
}

fn ensure_positive_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed astrodynamics result was not positive and finite",
        ))
    }
}

fn ensure_negative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value < 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(
            codex_id,
            "computed astrodynamics result was not negative and finite",
        ))
    }
}

fn checked_product(
    codex_id: &'static str,
    reason: &'static str,
    left: f64,
    right: f64,
) -> AeroResult<f64> {
    let value = left * right;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(numerical_failure(codex_id, reason))
    }
}

fn checked_sum_positive(
    codex_id: &'static str,
    reason: &'static str,
    left: f64,
    right: f64,
) -> AeroResult<f64> {
    let value = left + right;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(codex_id, reason))
    }
}

fn checked_positive_ratio(
    codex_id: &'static str,
    reason: &'static str,
    numerator: f64,
    denominator: f64,
) -> AeroResult<f64> {
    let value = numerator / denominator;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(numerical_failure(codex_id, reason))
    }
}

fn validate_mu_radius(mu: f64, radius: f64) -> AeroResult<()> {
    validation::ensure_positive("mu", mu)?;
    validation::ensure_positive("radius", radius)?;
    Ok(())
}

fn validate_mu_and_two_radii(mu: f64, r1: f64, r2: f64) -> AeroResult<()> {
    validation::ensure_positive("mu", mu)?;
    validation::ensure_positive("r1", r1)?;
    validation::ensure_positive("r2", r2)?;
    Ok(())
}

fn checked_circular_speed_for_transfer(
    codex_id: &'static str,
    mu: f64,
    radius: f64,
) -> AeroResult<f64> {
    let radicand = checked_positive_ratio(
        codex_id,
        "Hohmann circular-orbit speed radicand was not positive and finite",
        mu,
        radius,
    )?;
    ensure_positive_finite_result(codex_id, radicand.sqrt())
}

fn checked_hohmann_factor(
    codex_id: &'static str,
    numerator_radius: f64,
    radius_sum: f64,
) -> AeroResult<f64> {
    let twice_radius = checked_product(
        codex_id,
        "Hohmann transfer factor numerator was not finite",
        2.0,
        numerator_radius,
    )?;
    let radicand = checked_positive_ratio(
        codex_id,
        "Hohmann transfer factor radicand was not positive and finite",
        twice_radius,
        radius_sum,
    )?;
    ensure_positive_finite_result(codex_id, radicand.sqrt())
}

/// Circular-orbit speed, `v = sqrt(mu/r)`, in metres per second.
///
/// `mu` is the central body's gravitational parameter in `m^3/s^2`, and
/// `radius` is orbital radius from the central-body centre in metres. Both must
/// be finite and strictly positive. This helper assumes an ideal two-body,
/// point-mass central field and a circular orbit.
pub fn circular_orbit_speed(mu: f64, radius: f64) -> AeroResult<f64> {
    validate_mu_radius(mu, radius)?;
    let radicand = checked_positive_ratio(
        CODEX_ID_CIRCULAR_ORBIT_SPEED,
        "circular-orbit speed radicand was not positive and finite",
        mu,
        radius,
    )?;
    ensure_positive_finite_result(CODEX_ID_CIRCULAR_ORBIT_SPEED, radicand.sqrt())
}

/// Circular-orbit period, `T = 2*pi*sqrt(r^3/mu)`, in seconds.
///
/// `mu` and `radius` must be finite and strictly positive. The returned period
/// is the ideal two-body circular period and does not include perturbations,
/// rotating-frame effects, non-Keplerian acceleration, or time-standard issues.
pub fn orbital_period_circular(mu: f64, radius: f64) -> AeroResult<f64> {
    validate_mu_radius(mu, radius)?;
    let radius_squared = checked_product(
        CODEX_ID_ORBITAL_PERIOD_CIRCULAR,
        "circular-period radius squared was not finite",
        radius,
        radius,
    )?;
    let radius_cubed = checked_product(
        CODEX_ID_ORBITAL_PERIOD_CIRCULAR,
        "circular-period radius cubed was not finite",
        radius_squared,
        radius,
    )?;
    let time_squared = checked_positive_ratio(
        CODEX_ID_ORBITAL_PERIOD_CIRCULAR,
        "circular-period squared value was not positive and finite",
        radius_cubed,
        mu,
    )?;
    let period = checked_product(
        CODEX_ID_ORBITAL_PERIOD_CIRCULAR,
        "circular-period final multiplication was not finite",
        2.0 * PI,
        time_squared.sqrt(),
    )?;
    ensure_positive_finite_result(CODEX_ID_ORBITAL_PERIOD_CIRCULAR, period)
}

/// Two-body escape velocity, `v_e = sqrt(2*mu/r)`, in metres per second.
///
/// `mu` and `radius` must be finite and strictly positive. This scalar helper
/// assumes the ideal two-body point-mass escape-speed relation and does not
/// include atmosphere, rotating-planet energy, or finite-burn losses.
pub fn escape_velocity(mu: f64, radius: f64) -> AeroResult<f64> {
    validate_mu_radius(mu, radius)?;
    let twice_mu = checked_product(
        CODEX_ID_ESCAPE_VELOCITY,
        "escape-velocity two-times-mu product was not finite",
        2.0,
        mu,
    )?;
    let radicand = checked_positive_ratio(
        CODEX_ID_ESCAPE_VELOCITY,
        "escape-velocity radicand was not positive and finite",
        twice_mu,
        radius,
    )?;
    ensure_positive_finite_result(CODEX_ID_ESCAPE_VELOCITY, radicand.sqrt())
}

/// Vis-viva speed, `v = sqrt(mu*(2/r - 1/a))`, in metres per second.
///
/// `mu`, `radius`, and `semi_major_axis` must be finite and strictly positive.
/// Phase 0.001 treats this as an elliptic-orbit helper with positive semi-major
/// axis; hyperbolic negative-`a` conventions are deliberately out of scope. A
/// negative radicand is reported as an out-of-domain input combination.
pub fn vis_viva_speed(mu: f64, radius: f64, semi_major_axis: f64) -> AeroResult<f64> {
    validate_mu_radius(mu, radius)?;
    validation::ensure_positive("semi_major_axis", semi_major_axis)?;

    let radial_term = 2.0 / radius;
    let axis_term = 1.0 / semi_major_axis;
    if !(radial_term.is_finite() && axis_term.is_finite()) {
        return Err(numerical_failure(
            CODEX_ID_VIS_VIVA,
            "vis-viva reciprocal terms were not finite",
        ));
    }

    let energy_term = radial_term - axis_term;
    if !energy_term.is_finite() {
        return Err(numerical_failure(
            CODEX_ID_VIS_VIVA,
            "vis-viva energy term was not finite",
        ));
    }

    let radicand = checked_product(
        CODEX_ID_VIS_VIVA,
        "vis-viva radicand multiplication was not finite",
        mu,
        energy_term,
    )?;
    if radicand < 0.0 {
        return Err(AeroError::OutOfDomain {
            parameter: "radius_and_semi_major_axis",
            value: radicand,
            expected: "vis-viva radicand must be nonnegative for real elliptic speed",
        });
    }

    ensure_nonnegative_finite_result(CODEX_ID_VIS_VIVA, radicand.sqrt())
}

/// Elliptic specific orbital energy, `epsilon = -mu/(2*a)`, in `J/kg` or `m^2/s^2`.
///
/// `mu` and `semi_major_axis` must be finite and strictly positive. This helper
/// covers the negative specific energy of bound elliptic two-body orbits;
/// parabolic and hyperbolic orbit-energy conventions are out of scope for
/// Microtask 18.
pub fn specific_orbital_energy(mu: f64, semi_major_axis: f64) -> AeroResult<f64> {
    validation::ensure_positive("mu", mu)?;
    validation::ensure_positive("semi_major_axis", semi_major_axis)?;
    let denominator = checked_product(
        CODEX_ID_SPECIFIC_ORBITAL_ENERGY,
        "specific-orbital-energy denominator was not finite",
        2.0,
        semi_major_axis,
    )?;
    ensure_negative_finite_result(CODEX_ID_SPECIFIC_ORBITAL_ENERGY, -mu / denominator)
}

/// First Hohmann-transfer impulse magnitude in metres per second.
///
/// `mu`, `r1`, and `r2` must be finite and strictly positive. The helper assumes
/// two coplanar circular orbits about the same point-mass central body and an
/// impulsive two-burn transfer. It returns the magnitude of the first burn, so
/// transfer direction is not represented in the sign. When `r1 == r2`, the
/// returned magnitude is zero within floating-point roundoff.
pub fn hohmann_transfer_delta_v1(mu: f64, r1: f64, r2: f64) -> AeroResult<f64> {
    validate_mu_and_two_radii(mu, r1, r2)?;
    let radius_sum = checked_sum_positive(
        CODEX_ID_HOHMANN_TRANSFER_DELTA_V1,
        "Hohmann radius sum was not positive and finite",
        r1,
        r2,
    )?;
    let circular_speed =
        checked_circular_speed_for_transfer(CODEX_ID_HOHMANN_TRANSFER_DELTA_V1, mu, r1)?;
    let transfer_factor =
        checked_hohmann_factor(CODEX_ID_HOHMANN_TRANSFER_DELTA_V1, r2, radius_sum)?;
    let signed_delta = checked_product(
        CODEX_ID_HOHMANN_TRANSFER_DELTA_V1,
        "Hohmann delta-v1 product was not finite",
        circular_speed,
        transfer_factor - 1.0,
    )?;
    ensure_nonnegative_finite_result(CODEX_ID_HOHMANN_TRANSFER_DELTA_V1, signed_delta.abs())
}

/// Second Hohmann-transfer impulse magnitude in metres per second.
///
/// `mu`, `r1`, and `r2` must be finite and strictly positive. The helper assumes
/// two coplanar circular orbits about the same point-mass central body and an
/// impulsive two-burn transfer. It returns the magnitude of the second burn, so
/// transfer direction is not represented in the sign. When `r1 == r2`, the
/// returned magnitude is zero within floating-point roundoff.
pub fn hohmann_transfer_delta_v2(mu: f64, r1: f64, r2: f64) -> AeroResult<f64> {
    validate_mu_and_two_radii(mu, r1, r2)?;
    let radius_sum = checked_sum_positive(
        CODEX_ID_HOHMANN_TRANSFER_DELTA_V2,
        "Hohmann radius sum was not positive and finite",
        r1,
        r2,
    )?;
    let circular_speed =
        checked_circular_speed_for_transfer(CODEX_ID_HOHMANN_TRANSFER_DELTA_V2, mu, r2)?;
    let transfer_factor =
        checked_hohmann_factor(CODEX_ID_HOHMANN_TRANSFER_DELTA_V2, r1, radius_sum)?;
    let signed_delta = checked_product(
        CODEX_ID_HOHMANN_TRANSFER_DELTA_V2,
        "Hohmann delta-v2 product was not finite",
        circular_speed,
        1.0 - transfer_factor,
    )?;
    ensure_nonnegative_finite_result(CODEX_ID_HOHMANN_TRANSFER_DELTA_V2, signed_delta.abs())
}

/// Total Hohmann-transfer delta-v magnitude in metres per second.
///
/// This is the checked sum of `hohmann_transfer_delta_v1` and
/// `hohmann_transfer_delta_v2` for positive `mu`, `r1`, and `r2`. It returns zero
/// when both circular-orbit radii are equal, subject to floating-point roundoff.
pub fn hohmann_transfer_total_delta_v(mu: f64, r1: f64, r2: f64) -> AeroResult<f64> {
    let delta_v1 = hohmann_transfer_delta_v1(mu, r1, r2)?;
    let delta_v2 = hohmann_transfer_delta_v2(mu, r1, r2)?;
    let total = delta_v1 + delta_v2;
    ensure_nonnegative_finite_result(CODEX_ID_HOHMANN_TOTAL_DELTA_V, total)
}

/// Hohmann half-transfer-ellipse time of flight in seconds.
///
/// `mu`, `r1`, and `r2` must be finite and strictly positive. This returns
/// `pi*sqrt(a_t^3/mu)`, where `a_t = (r1 + r2)/2` is the transfer ellipse
/// semi-major axis. It does not model launch windows, phasing, perturbations,
/// finite burns, or maneuver losses.
pub fn hohmann_transfer_time(mu: f64, r1: f64, r2: f64) -> AeroResult<f64> {
    validate_mu_and_two_radii(mu, r1, r2)?;
    let radius_sum = checked_sum_positive(
        CODEX_ID_HOHMANN_TRANSFER_TIME,
        "Hohmann transfer radius sum was not positive and finite",
        r1,
        r2,
    )?;
    let semi_major_axis = checked_product(
        CODEX_ID_HOHMANN_TRANSFER_TIME,
        "Hohmann transfer semi-major axis was not finite",
        0.5,
        radius_sum,
    )?;
    let axis_squared = checked_product(
        CODEX_ID_HOHMANN_TRANSFER_TIME,
        "Hohmann transfer semi-major-axis squared was not finite",
        semi_major_axis,
        semi_major_axis,
    )?;
    let axis_cubed = checked_product(
        CODEX_ID_HOHMANN_TRANSFER_TIME,
        "Hohmann transfer semi-major-axis cubed was not finite",
        axis_squared,
        semi_major_axis,
    )?;
    let time_squared = checked_positive_ratio(
        CODEX_ID_HOHMANN_TRANSFER_TIME,
        "Hohmann transfer time-squared value was not positive and finite",
        axis_cubed,
        mu,
    )?;
    let time = checked_product(
        CODEX_ID_HOHMANN_TRANSFER_TIME,
        "Hohmann transfer time final multiplication was not finite",
        PI,
        time_squared.sqrt(),
    )?;
    ensure_positive_finite_result(CODEX_ID_HOHMANN_TRANSFER_TIME, time)
}

/// Scalar sphere-of-influence radius using `r_soi = d*(m_secondary/m_primary)^(2/5)`.
///
/// `primary_distance`, `secondary_mass`, and `primary_mass` must be finite and
/// strictly positive. The distance is the secondary body's distance from the
/// primary body in metres. The helper is a simple scalar preliminary-design
/// approximation and does not model ephemeris variation, barycentric frames,
/// restricted three-body dynamics, or patched-conic validation.
pub fn sphere_of_influence_radius(
    primary_distance: f64,
    secondary_mass: f64,
    primary_mass: f64,
) -> AeroResult<f64> {
    validation::ensure_positive("primary_distance", primary_distance)?;
    validation::ensure_positive("secondary_mass", secondary_mass)?;
    validation::ensure_positive("primary_mass", primary_mass)?;

    let mass_ratio = checked_positive_ratio(
        CODEX_ID_SPHERE_OF_INFLUENCE,
        "sphere-of-influence mass ratio was not positive and finite",
        secondary_mass,
        primary_mass,
    )?;
    let exponent = mass_ratio.powf(2.0 / 5.0);
    let exponent = ensure_positive_finite_result(CODEX_ID_SPHERE_OF_INFLUENCE, exponent)?;
    let radius = checked_product(
        CODEX_ID_SPHERE_OF_INFLUENCE,
        "sphere-of-influence radius product was not finite",
        primary_distance,
        exponent,
    )?;
    ensure_positive_finite_result(CODEX_ID_SPHERE_OF_INFLUENCE, radius)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_constants::{EARTH_GRAVITATIONAL_PARAMETER_M3_S2, EARTH_MEAN_RADIUS_M};
    use aero_codex_core::{AeroError, VerificationStatus};

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual}, expected={expected}, tolerance={tolerance}"
        );
    }

    #[test]
    fn circular_orbit_speed_around_earth_is_plausible() {
        let r = EARTH_MEAN_RADIUS_M + 400_000.0;
        let v = circular_orbit_speed(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r).unwrap();
        assert!((7_500.0..7_900.0).contains(&v));
    }

    #[test]
    fn circular_orbit_speed_rejects_invalid_inputs_and_overflow() {
        assert!(circular_orbit_speed(0.0, 1.0).is_err());
        assert!(circular_orbit_speed(1.0, 0.0).is_err());
        assert!(circular_orbit_speed(f64::NAN, 1.0).is_err());
        assert!(matches!(
            circular_orbit_speed(f64::MIN_POSITIVE, f64::MAX),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn escape_velocity_is_sqrt_two_circular_speed() {
        let r = EARTH_MEAN_RADIUS_M + 400_000.0;
        let vc = circular_orbit_speed(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r).unwrap();
        let ve = escape_velocity(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r).unwrap();
        assert_close(ve / vc, 2.0_f64.sqrt(), 1.0e-12);
    }

    #[test]
    fn escape_velocity_rejects_invalid_inputs_and_overflow() {
        assert!(escape_velocity(0.0, 1.0).is_err());
        assert!(escape_velocity(1.0, 0.0).is_err());
        assert!(matches!(
            escape_velocity(f64::MAX, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn vis_viva_equals_circular_when_a_equals_r() {
        let r = EARTH_MEAN_RADIUS_M + 400_000.0;
        assert_close(
            vis_viva_speed(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r, r).unwrap(),
            circular_orbit_speed(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r).unwrap(),
            1.0e-9,
        );
    }

    #[test]
    fn vis_viva_rejects_invalid_inputs_and_negative_radicand() {
        assert!(vis_viva_speed(0.0, 1.0, 1.0).is_err());
        assert!(vis_viva_speed(1.0, 0.0, 1.0).is_err());
        assert!(vis_viva_speed(1.0, 1.0, 0.0).is_err());
        assert!(matches!(
            vis_viva_speed(1.0, 3.0, 1.0),
            Err(AeroError::OutOfDomain { .. })
        ));
        assert!(matches!(
            vis_viva_speed(f64::MAX, f64::MIN_POSITIVE, 1.0),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn period_positive_and_matches_circular_formula() {
        let r = EARTH_MEAN_RADIUS_M + 400_000.0;
        let period = orbital_period_circular(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r).unwrap();
        let expected = 2.0 * PI * (r * r * r / EARTH_GRAVITATIONAL_PARAMETER_M3_S2).sqrt();
        assert!(period > 0.0);
        assert_close(period, expected, 1.0e-9);
    }

    #[test]
    fn period_rejects_invalid_inputs_and_overflow() {
        assert!(orbital_period_circular(0.0, 1.0).is_err());
        assert!(orbital_period_circular(1.0, 0.0).is_err());
        assert!(matches!(
            orbital_period_circular(1.0, f64::MAX),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn specific_orbital_energy_negative_for_elliptical_orbit() {
        let r = EARTH_MEAN_RADIUS_M + 400_000.0;
        let energy = specific_orbital_energy(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r).unwrap();
        assert!(energy < 0.0);
        assert_close(
            energy,
            -EARTH_GRAVITATIONAL_PARAMETER_M3_S2 / (2.0 * r),
            1.0e-9,
        );
    }

    #[test]
    fn specific_orbital_energy_rejects_invalid_inputs_and_overflow() {
        assert!(specific_orbital_energy(0.0, 1.0).is_err());
        assert!(specific_orbital_energy(1.0, 0.0).is_err());
        assert!(matches!(
            specific_orbital_energy(1.0, f64::MAX),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn hohmann_total_delta_v_positive_when_radii_differ() {
        let r1 = EARTH_MEAN_RADIUS_M + 400_000.0;
        let r2 = EARTH_MEAN_RADIUS_M + 800_000.0;
        let dv1 = hohmann_transfer_delta_v1(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r1, r2).unwrap();
        let dv2 = hohmann_transfer_delta_v2(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r1, r2).unwrap();
        let total =
            hohmann_transfer_total_delta_v(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r1, r2).unwrap();
        assert!(dv1 > 0.0);
        assert!(dv2 > 0.0);
        assert!(total > 0.0);
        assert_close(total, dv1 + dv2, 1.0e-12);
    }

    #[test]
    fn hohmann_delta_v_is_zero_when_radii_match() {
        let r = EARTH_MEAN_RADIUS_M + 400_000.0;
        assert!(
            hohmann_transfer_delta_v1(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r, r).unwrap() < 1.0e-9
        );
        assert!(
            hohmann_transfer_delta_v2(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r, r).unwrap() < 1.0e-9
        );
        assert!(
            hohmann_transfer_total_delta_v(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r, r).unwrap()
                < 1.0e-9
        );
    }

    #[test]
    fn hohmann_transfer_is_symmetric_in_total_delta_v() {
        let r1 = EARTH_MEAN_RADIUS_M + 400_000.0;
        let r2 = EARTH_MEAN_RADIUS_M + 800_000.0;
        let outbound =
            hohmann_transfer_total_delta_v(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r1, r2).unwrap();
        let inbound =
            hohmann_transfer_total_delta_v(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r2, r1).unwrap();
        assert_close(outbound, inbound, 1.0e-9);
    }

    #[test]
    fn hohmann_transfer_time_positive_and_matches_formula() {
        let r1 = EARTH_MEAN_RADIUS_M + 400_000.0;
        let r2 = EARTH_MEAN_RADIUS_M + 800_000.0;
        let time = hohmann_transfer_time(EARTH_GRAVITATIONAL_PARAMETER_M3_S2, r1, r2).unwrap();
        let transfer_axis = 0.5 * (r1 + r2);
        let expected = PI
            * (transfer_axis * transfer_axis * transfer_axis / EARTH_GRAVITATIONAL_PARAMETER_M3_S2)
                .sqrt();
        assert!(time > 0.0);
        assert_close(time, expected, 1.0e-9);
    }

    #[test]
    fn hohmann_transfer_rejects_invalid_inputs_and_overflow() {
        assert!(hohmann_transfer_total_delta_v(0.0, 1.0, 2.0).is_err());
        assert!(hohmann_transfer_total_delta_v(1.0, 0.0, 2.0).is_err());
        assert!(hohmann_transfer_total_delta_v(1.0, 1.0, 0.0).is_err());
        assert!(hohmann_transfer_time(1.0, f64::MAX, f64::MAX).is_err());
        assert!(matches!(
            hohmann_transfer_delta_v1(f64::MAX, 1.0, f64::MAX),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn sphere_of_influence_radius_is_positive_and_below_primary_distance_for_planet_star_case() {
        let earth_sun_distance_m: f64 = 1.496e11;
        let earth_mass_kg: f64 = 5.972e24;
        let sun_mass_kg: f64 = 1.989e30;
        let soi =
            sphere_of_influence_radius(earth_sun_distance_m, earth_mass_kg, sun_mass_kg).unwrap();
        assert!(soi > 0.0);
        assert!(soi < earth_sun_distance_m);
        assert_close(
            soi,
            earth_sun_distance_m * (earth_mass_kg / sun_mass_kg).powf(2.0 / 5.0),
            1.0e-3,
        );
    }

    #[test]
    fn sphere_of_influence_rejects_invalid_inputs_and_overflow() {
        assert!(sphere_of_influence_radius(0.0, 1.0, 1.0).is_err());
        assert!(sphere_of_influence_radius(1.0, 0.0, 1.0).is_err());
        assert!(sphere_of_influence_radius(1.0, 1.0, 0.0).is_err());
        assert!(sphere_of_influence_radius(f64::INFINITY, 1.0, 1.0).is_err());
        assert!(matches!(
            sphere_of_influence_radius(f64::MAX, f64::MAX, f64::MIN_POSITIVE),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn astrodynamics_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_CIRCULAR_ORBIT_SPEED,
            CODEX_ID_ORBITAL_PERIOD_CIRCULAR,
            CODEX_ID_ESCAPE_VELOCITY,
            CODEX_ID_VIS_VIVA,
            CODEX_ID_SPECIFIC_ORBITAL_ENERGY,
        ] {
            let record = verification_record(codex_id).expect("two-body verification record");
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert!(record.has_sources());
            assert_eq!(record.sources, ASTRODYNAMICS_TWO_BODY_SOURCES);
        }

        for codex_id in [
            CODEX_ID_HOHMANN_TRANSFER_DELTA_V1,
            CODEX_ID_HOHMANN_TRANSFER_DELTA_V2,
            CODEX_ID_HOHMANN_TOTAL_DELTA_V,
            CODEX_ID_HOHMANN_TRANSFER_TIME,
            CODEX_ID_SPHERE_OF_INFLUENCE,
        ] {
            let record =
                verification_record(codex_id).expect("transfer/celestial verification record");
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert!(record.has_sources());
            assert_eq!(record.sources, ASTRODYNAMICS_TRANSFER_CELESTIAL_SOURCES);
        }

        assert_eq!(
            CODEX_ID_HOHMANN_DELTA_V1,
            CODEX_ID_HOHMANN_TRANSFER_DELTA_V1
        );
        assert_eq!(
            CODEX_ID_HOHMANN_DELTA_V2,
            CODEX_ID_HOHMANN_TRANSFER_DELTA_V2
        );
        assert!(verification_record("astrodynamics.unknown").is_none());
    }
}
