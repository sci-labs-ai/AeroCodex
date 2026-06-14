#![forbid(unsafe_code)]
//! Compressible-flow equations for Phase 0.001.
//!
//! Microtasks 9 through 12 review the direct isentropic, normal-shock,
//! Mach-angle, Prandtl-Meyer, and branch-explicit oblique-shock perfect-gas
//! relations in this crate. All reviewed relations are scalar, calorically
//! perfect-gas helpers that return checked `AeroResult` values instead of
//! silently accepting invalid domains, nonfinite outputs, failed inverse solves,
//! or missing attached-shock branches.
//!
//! Source traceability remains conservative. The gas-dynamics source-registry
//! seed and validation cards remain `research_required` until exact report
//! editions, equation/table/page locators, reference values, and tolerances are
//! reviewed.

use aero_codex_constants::SOURCE_ID_NACA_REPORT_1135;
use aero_codex_core::{validation, AeroError, AeroResult, Angle, VerificationRecord};
use std::f64::consts::PI;

/// Codex ID for the isentropic total-to-static temperature ratio, `T0/T`.
pub const CODEX_ID_ISENTROPIC_TEMPERATURE_RATIO: &str = "gasdyn.isentropic.temperature_ratio";
/// Codex ID for the isentropic total-to-static pressure ratio, `p0/p`.
pub const CODEX_ID_ISENTROPIC_PRESSURE_RATIO: &str = "gasdyn.isentropic.pressure_ratio";
/// Codex ID for the isentropic total-to-static density ratio, `rho0/rho`.
pub const CODEX_ID_ISENTROPIC_DENSITY_RATIO: &str = "gasdyn.isentropic.density_ratio";
/// Codex ID for the isentropic area-Mach relation, `A/A*`.
pub const CODEX_ID_ISENTROPIC_AREA_MACH_RATIO: &str = "gasdyn.isentropic.area_mach_ratio";
/// Codex ID for the isentropic mass-flow parameter.
pub const CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER: &str = "gasdyn.isentropic.mass_flow_parameter";

/// Codex ID for downstream Mach number across a normal shock, `M2`.
pub const CODEX_ID_NORMAL_SHOCK_DOWNSTREAM_MACH: &str = "gasdyn.normal_shock.mach2";
/// Backward-compatible alias for downstream Mach number across a normal shock.
pub const CODEX_ID_NORMAL_SHOCK_MACH2: &str = CODEX_ID_NORMAL_SHOCK_DOWNSTREAM_MACH;
/// Codex ID for the normal-shock static-pressure ratio, `p2/p1`.
pub const CODEX_ID_NORMAL_SHOCK_PRESSURE_RATIO: &str = "gasdyn.normal_shock.pressure_ratio_p2_p1";
/// Codex ID for the normal-shock density ratio, `rho2/rho1`.
pub const CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO: &str = "gasdyn.normal_shock.density_ratio_rho2_rho1";
/// Codex ID for the normal-shock static-temperature ratio, `T2/T1`.
pub const CODEX_ID_NORMAL_SHOCK_TEMPERATURE_RATIO: &str =
    "gasdyn.normal_shock.temperature_ratio_t2_t1";
/// Codex ID for the normal-shock total-pressure ratio, `p02/p01`.
pub const CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO: &str =
    "gasdyn.normal_shock.total_pressure_ratio_p02_p01";

/// Codex ID for the Mach angle, `mu = asin(1/M)`.
pub const CODEX_ID_MACH_ANGLE: &str = "gasdyn.mach_angle.mu";
/// Codex ID for the Prandtl-Meyer function, `nu(M, gamma)`.
pub const CODEX_ID_PRANDTL_MEYER_NU: &str = "gasdyn.prandtl_meyer.nu";
/// Backward-compatible alias for the Prandtl-Meyer function Codex ID.
pub const CODEX_ID_PRANDTL_MEYER: &str = CODEX_ID_PRANDTL_MEYER_NU;
/// Codex ID for inverting the Prandtl-Meyer function to Mach number.
pub const CODEX_ID_PRANDTL_MEYER_INVERSE: &str = "gasdyn.prandtl_meyer.inverse_mach";
/// Codex ID for the theta-beta-Mach residual used by the oblique-shock solver.
pub const CODEX_ID_OBLIQUE_SHOCK_RESIDUAL: &str = "gasdyn.oblique_shock.theta_beta_mach_residual";
/// Fully descriptive alias for the theta-beta-Mach residual Codex ID.
pub const CODEX_ID_OBLIQUE_SHOCK_THETA_BETA_MACH_RESIDUAL: &str = CODEX_ID_OBLIQUE_SHOCK_RESIDUAL;
/// Short alias for the theta-beta-Mach residual Codex ID.
pub const CODEX_ID_THETA_BETA_MACH_RESIDUAL: &str = CODEX_ID_OBLIQUE_SHOCK_RESIDUAL;
/// Codex ID for solving attached oblique-shock wave angle `beta`.
pub const CODEX_ID_OBLIQUE_SHOCK_BETA: &str = "gasdyn.oblique_shock.beta";
/// Codex ID for the upstream normal Mach component, `M1n = M1 sin(beta)`.
pub const CODEX_ID_OBLIQUE_SHOCK_NORMAL_MACH: &str = "gasdyn.oblique_shock.normal_mach1";
/// Codex ID for downstream Mach number behind an oblique shock.
pub const CODEX_ID_OBLIQUE_SHOCK_DOWNSTREAM_MACH: &str = "gasdyn.oblique_shock.downstream_mach";

/// Source-registry ID used by the Phase 0.001 isentropic gas-dynamics scaffold.
pub const SOURCE_ID_GAS_DYNAMICS_ISENTROPIC: &str = SOURCE_ID_NACA_REPORT_1135;
/// Source-registry ID used by the Phase 0.001 normal-shock scaffold.
pub const SOURCE_ID_GAS_DYNAMICS_NORMAL_SHOCK: &str = SOURCE_ID_NACA_REPORT_1135;
/// Source-registry ID used by the Phase 0.001 Mach-angle and Prandtl-Meyer scaffold.
pub const SOURCE_ID_GAS_DYNAMICS_EXPANSION_FLOW: &str = SOURCE_ID_NACA_REPORT_1135;
/// Backward-compatible alias for the expansion-flow source-registry ID.
pub const SOURCE_ID_GAS_DYNAMICS_EXPANSIONS: &str = SOURCE_ID_GAS_DYNAMICS_EXPANSION_FLOW;
/// Source-registry ID used by the Phase 0.001 oblique-shock scaffold.
pub const SOURCE_ID_GAS_DYNAMICS_OBLIQUE_SHOCK: &str = SOURCE_ID_NACA_REPORT_1135;

const ISENTROPIC_SOURCES: &[&str] = &[SOURCE_ID_GAS_DYNAMICS_ISENTROPIC];
const NORMAL_SHOCK_SOURCES: &[&str] = &[SOURCE_ID_GAS_DYNAMICS_NORMAL_SHOCK];
const EXPANSION_FLOW_SOURCES: &[&str] = &[SOURCE_ID_GAS_DYNAMICS_EXPANSION_FLOW];
const OBLIQUE_SHOCK_SOURCES: &[&str] = &[SOURCE_ID_GAS_DYNAMICS_OBLIQUE_SHOCK];

/// Explicit attached oblique-shock branch selector.
///
/// The solver never guesses weak versus strong branch. Callers must provide one
/// of these variants to `oblique_shock_beta`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShockBranch {
    /// Lower wave-angle attached solution when both attached branches exist.
    Weak,
    /// Higher wave-angle attached solution when both attached branches exist.
    Strong,
}

impl ShockBranch {
    /// Canonical branch name for logs, diagnostics, and validation artifacts.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Weak => "weak",
            Self::Strong => "strong",
        }
    }
}

/// Conservative traceability metadata for Phase 0.001 gas-dynamics equations.
#[must_use]
pub fn verification_record(codex_id: &str) -> Option<VerificationRecord> {
    match codex_id {
        CODEX_ID_ISENTROPIC_TEMPERATURE_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_ISENTROPIC_TEMPERATURE_RATIO,
            ISENTROPIC_SOURCES,
            "Isentropic total-to-static temperature ratio; source equation and tolerance review pending.",
        )),
        CODEX_ID_ISENTROPIC_PRESSURE_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_ISENTROPIC_PRESSURE_RATIO,
            ISENTROPIC_SOURCES,
            "Isentropic total-to-static pressure ratio; source equation and tolerance review pending.",
        )),
        CODEX_ID_ISENTROPIC_DENSITY_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_ISENTROPIC_DENSITY_RATIO,
            ISENTROPIC_SOURCES,
            "Isentropic total-to-static density ratio; source equation and tolerance review pending.",
        )),
        CODEX_ID_ISENTROPIC_AREA_MACH_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_ISENTROPIC_AREA_MACH_RATIO,
            ISENTROPIC_SOURCES,
            "Isentropic area-Mach relation; branch and inverse validation deferred.",
        )),
        CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER => Some(VerificationRecord::research_required(
            CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER,
            ISENTROPIC_SOURCES,
            "Isentropic mass-flow parameter; normalization and source review pending.",
        )),
        CODEX_ID_NORMAL_SHOCK_MACH2 => Some(VerificationRecord::research_required(
            CODEX_ID_NORMAL_SHOCK_MACH2,
            NORMAL_SHOCK_SOURCES,
            "Normal-shock downstream Mach relation; source equation and tolerance review pending.",
        )),
        CODEX_ID_NORMAL_SHOCK_PRESSURE_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_NORMAL_SHOCK_PRESSURE_RATIO,
            NORMAL_SHOCK_SOURCES,
            "Normal-shock static-pressure ratio; source equation and tolerance review pending.",
        )),
        CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO,
            NORMAL_SHOCK_SOURCES,
            "Normal-shock density ratio; source equation and tolerance review pending.",
        )),
        CODEX_ID_NORMAL_SHOCK_TEMPERATURE_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_NORMAL_SHOCK_TEMPERATURE_RATIO,
            NORMAL_SHOCK_SOURCES,
            "Normal-shock static-temperature ratio; source equation and tolerance review pending.",
        )),
        CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO => Some(VerificationRecord::research_required(
            CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO,
            NORMAL_SHOCK_SOURCES,
            "Normal-shock total-pressure-loss ratio; source equation and tolerance review pending.",
        )),
        CODEX_ID_MACH_ANGLE => Some(VerificationRecord::research_required(
            CODEX_ID_MACH_ANGLE,
            EXPANSION_FLOW_SOURCES,
            "Mach-angle relation; source equation and tolerance review pending.",
        )),
        CODEX_ID_PRANDTL_MEYER_NU => Some(VerificationRecord::research_required(
            CODEX_ID_PRANDTL_MEYER_NU,
            EXPANSION_FLOW_SOURCES,
            "Prandtl-Meyer function; source equation and tolerance review pending.",
        )),
        CODEX_ID_PRANDTL_MEYER_INVERSE => Some(VerificationRecord::research_required(
            CODEX_ID_PRANDTL_MEYER_INVERSE,
            EXPANSION_FLOW_SOURCES,
            "Prandtl-Meyer inverse solved by bracketing and bisection; source examples and tolerance review pending.",
        )),
        CODEX_ID_OBLIQUE_SHOCK_RESIDUAL => Some(VerificationRecord::research_required(
            CODEX_ID_OBLIQUE_SHOCK_RESIDUAL,
            OBLIQUE_SHOCK_SOURCES,
            "Theta-beta-Mach residual for attached oblique shocks; source equation and tolerance review pending.",
        )),
        CODEX_ID_OBLIQUE_SHOCK_BETA => Some(VerificationRecord::research_required(
            CODEX_ID_OBLIQUE_SHOCK_BETA,
            OBLIQUE_SHOCK_SOURCES,
            "Branch-explicit attached oblique-shock beta solve; branch conventions and reference examples pending.",
        )),
        CODEX_ID_OBLIQUE_SHOCK_NORMAL_MACH => Some(VerificationRecord::research_required(
            CODEX_ID_OBLIQUE_SHOCK_NORMAL_MACH,
            OBLIQUE_SHOCK_SOURCES,
            "Upstream normal Mach component for attached oblique shocks; source review pending.",
        )),
        CODEX_ID_OBLIQUE_SHOCK_DOWNSTREAM_MACH => Some(VerificationRecord::research_required(
            CODEX_ID_OBLIQUE_SHOCK_DOWNSTREAM_MACH,
            OBLIQUE_SHOCK_SOURCES,
            "Downstream Mach relation composed from attached oblique-shock geometry and normal-shock relation; validation pending.",
        )),
        _ => None,
    }
}

fn validate_gamma(gamma: f64) -> AeroResult<()> {
    validation::ensure_greater_than("gamma", gamma, 1.0)
}

fn validate_mach_nonnegative(mach: f64) -> AeroResult<()> {
    validation::ensure_nonnegative("mach", mach)
}

fn validate_normal_shock_inputs(mach1: f64, gamma: f64) -> AeroResult<()> {
    validation::require_strictly_supersonic(mach1)?;
    validate_gamma(gamma)
}

fn ensure_positive_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason: "computed gas-dynamics result was not positive and finite",
        })
    }
}

fn ensure_nonnegative_finite_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason: "computed gas-dynamics result was not nonnegative and finite",
        })
    }
}

fn ensure_finite_result(
    codex_id: &'static str,
    value: f64,
    reason: &'static str,
) -> AeroResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason,
        })
    }
}

fn ensure_downstream_subsonic_mach_result(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 && value < 1.0 {
        Ok(value)
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason: "normal-shock downstream Mach number was not subsonic and finite",
        })
    }
}

const NORMAL_SHOCK_TOTAL_PRESSURE_ROUNDING_ALLOWANCE: f64 = 1.0e-12;

fn ensure_total_pressure_loss_ratio(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    if value.is_finite()
        && value > 0.0
        && value <= 1.0 + NORMAL_SHOCK_TOTAL_PRESSURE_ROUNDING_ALLOWANCE
    {
        Ok(value.min(1.0))
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason: "normal-shock total-pressure ratio was not in the expected finite loss range",
        })
    }
}

fn normal_shock_mach1_squared(mach1: f64, codex_id: &'static str) -> AeroResult<f64> {
    let mach1_squared = mach1 * mach1;
    if mach1_squared.is_finite() {
        Ok(mach1_squared)
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason: "upstream Mach number squared overflowed",
        })
    }
}

fn ensure_positive_finite_term(
    codex_id: &'static str,
    value: f64,
    reason: &'static str,
) -> AeroResult<f64> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason,
        })
    }
}

const EXPANSION_ANGLE_ROUNDING_ALLOWANCE_RAD: f64 = 1.0e-14;
const PRANDTL_MEYER_INVERSE_MAX_ITERATIONS: usize = 128;
const PRANDTL_MEYER_INVERSE_MAX_BRACKET_MACH: f64 = 1.0e6;

const OBLIQUE_SHOCK_BETA_EPSILON_RAD: f64 = 1.0e-10;
const OBLIQUE_SHOCK_BETA_SCAN_INTERVALS: usize = 4_096;
const OBLIQUE_SHOCK_BETA_MAX_BISECTION_ITERATIONS: usize = 128;
const OBLIQUE_SHOCK_RESIDUAL_TOLERANCE: f64 = 1.0e-13;

fn ensure_nonnegative_finite_term(
    codex_id: &'static str,
    value: f64,
    reason: &'static str,
) -> AeroResult<f64> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason,
        })
    }
}

fn ensure_nonnegative_angle_result(
    codex_id: &'static str,
    radians: f64,
    reason: &'static str,
) -> AeroResult<Angle> {
    if radians.is_finite() && radians >= -EXPANSION_ANGLE_ROUNDING_ALLOWANCE_RAD {
        Ok(Angle::from_radians(radians.max(0.0)))
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason,
        })
    }
}

fn ensure_angle_radians_in_range(
    codex_id: &'static str,
    radians: f64,
    min: f64,
    max: f64,
    reason: &'static str,
) -> AeroResult<Angle> {
    if radians.is_finite() && radians >= min && radians <= max {
        Ok(Angle::from_radians(radians))
    } else {
        Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason,
        })
    }
}

fn mach_squared_minus_one(mach: f64, codex_id: &'static str) -> AeroResult<f64> {
    let mach_squared = mach * mach;
    if !mach_squared.is_finite() {
        return Err(AeroError::NumericalFailure {
            solver: codex_id,
            reason: "Mach number squared overflowed",
        });
    }
    ensure_nonnegative_finite_term(
        codex_id,
        mach_squared - 1.0,
        "Mach-squared-minus-one term was not nonnegative and finite",
    )
}

fn prandtl_meyer_coefficient_base(gamma: f64) -> AeroResult<f64> {
    ensure_positive_finite_term(
        CODEX_ID_PRANDTL_MEYER_NU,
        (gamma + 1.0) / (gamma - 1.0),
        "Prandtl-Meyer coefficient base was not positive and finite",
    )
}

fn prandtl_meyer_max_radians(gamma: f64) -> AeroResult<f64> {
    let coefficient_base = prandtl_meyer_coefficient_base(gamma)?;
    ensure_positive_finite_result(
        CODEX_ID_PRANDTL_MEYER_INVERSE,
        0.5 * PI * (coefficient_base.sqrt() - 1.0),
    )
}

/// Isentropic total-to-static temperature ratio, `T0/T`.
///
/// Domain: `Mach >= 0` and `gamma > 1`. The relation assumes steady,
/// one-dimensional, adiabatic, inviscid, isentropic flow of a calorically
/// perfect gas with constant `gamma`.
pub fn temperature_ratio_t0_over_t(mach: f64, gamma: f64) -> AeroResult<f64> {
    validate_mach_nonnegative(mach)?;
    validate_gamma(gamma)?;
    let ratio = 1.0 + 0.5 * (gamma - 1.0) * mach * mach;
    ensure_positive_finite_result(CODEX_ID_ISENTROPIC_TEMPERATURE_RATIO, ratio)
}

/// Isentropic total-to-static pressure ratio, `p0/p`.
///
/// Domain: `Mach >= 0` and `gamma > 1`. Returns 1 at Mach 0.
pub fn pressure_ratio_p0_over_p(mach: f64, gamma: f64) -> AeroResult<f64> {
    let tr = temperature_ratio_t0_over_t(mach, gamma)?;
    ensure_positive_finite_result(
        CODEX_ID_ISENTROPIC_PRESSURE_RATIO,
        tr.powf(gamma / (gamma - 1.0)),
    )
}

/// Isentropic total-to-static density ratio, `rho0/rho`.
///
/// Domain: `Mach >= 0` and `gamma > 1`. Returns 1 at Mach 0.
pub fn density_ratio_rho0_over_rho(mach: f64, gamma: f64) -> AeroResult<f64> {
    let tr = temperature_ratio_t0_over_t(mach, gamma)?;
    ensure_positive_finite_result(
        CODEX_ID_ISENTROPIC_DENSITY_RATIO,
        tr.powf(1.0 / (gamma - 1.0)),
    )
}

/// Isentropic area-Mach ratio, `A/A*`.
///
/// Domain: `Mach > 0` and `gamma > 1`. The direct relation is branch-free for a
/// supplied Mach number and has its minimum value of 1 at Mach 1. Inverse
/// area-Mach solving is intentionally deferred to a later microtask because it
/// requires explicit subsonic/supersonic branch selection.
pub fn area_mach_ratio(mach: f64, gamma: f64) -> AeroResult<f64> {
    validation::ensure_positive("mach", mach)?;
    validate_gamma(gamma)?;
    let term = (2.0 / (gamma + 1.0)) * (1.0 + 0.5 * (gamma - 1.0) * mach * mach);
    if !(term.is_finite() && term > 0.0) {
        return Err(AeroError::NumericalFailure {
            solver: CODEX_ID_ISENTROPIC_AREA_MACH_RATIO,
            reason: "area-Mach base term was not positive and finite",
        });
    }
    ensure_positive_finite_result(
        CODEX_ID_ISENTROPIC_AREA_MACH_RATIO,
        (1.0 / mach) * term.powf((gamma + 1.0) / (2.0 * (gamma - 1.0))),
    )
}

/// Isentropic mass-flow parameter, `sqrt(gamma) M (1 + (gamma-1)M²/2)^[-(gamma+1)/(2(gamma-1))]`.
///
/// Domain: `Mach >= 0` and `gamma > 1`. This dimensionless helper returns zero
/// at Mach 0 and a positive finite value for valid positive Mach numbers.
pub fn mass_flow_parameter(mach: f64, gamma: f64) -> AeroResult<f64> {
    validate_mach_nonnegative(mach)?;
    validate_gamma(gamma)?;
    let term = 1.0 + 0.5 * (gamma - 1.0) * mach * mach;
    if !(term.is_finite() && term > 0.0) {
        return Err(AeroError::NumericalFailure {
            solver: CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER,
            reason: "mass-flow parameter base term was not positive and finite",
        });
    }
    ensure_nonnegative_finite_result(
        CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER,
        mach * gamma.sqrt() * term.powf(-(gamma + 1.0) / (2.0 * (gamma - 1.0))),
    )
}

/// Downstream Mach number across a normal shock, `M2`.
///
/// Domain: upstream `mach1 > 1` and `gamma > 1`. The relation assumes a
/// stationary, plane, one-dimensional normal shock in a calorically perfect gas
/// with constant `gamma`. For valid inputs the returned downstream Mach number
/// is positive and subsonic.
pub fn normal_shock_mach2(mach1: f64, gamma: f64) -> AeroResult<f64> {
    validate_normal_shock_inputs(mach1, gamma)?;
    let mach1_squared = normal_shock_mach1_squared(mach1, CODEX_ID_NORMAL_SHOCK_MACH2)?;
    let numerator = ensure_positive_finite_term(
        CODEX_ID_NORMAL_SHOCK_MACH2,
        1.0 + 0.5 * (gamma - 1.0) * mach1_squared,
        "normal-shock downstream-Mach numerator was not positive and finite",
    )?;
    let denominator = ensure_positive_finite_term(
        CODEX_ID_NORMAL_SHOCK_MACH2,
        gamma * mach1_squared - 0.5 * (gamma - 1.0),
        "normal-shock downstream-Mach denominator was not positive and finite",
    )?;
    ensure_downstream_subsonic_mach_result(
        CODEX_ID_NORMAL_SHOCK_MACH2,
        (numerator / denominator).sqrt(),
    )
}

/// Static-pressure ratio across a normal shock, `p2/p1`.
///
/// Domain: upstream `mach1 > 1` and `gamma > 1`. For valid inputs the ratio is
/// positive and greater than unity, subject to floating-point roundoff near the
/// sonic boundary.
pub fn normal_shock_pressure_ratio_p2_p1(mach1: f64, gamma: f64) -> AeroResult<f64> {
    validate_normal_shock_inputs(mach1, gamma)?;
    let mach1_squared = normal_shock_mach1_squared(mach1, CODEX_ID_NORMAL_SHOCK_PRESSURE_RATIO)?;
    ensure_positive_finite_result(
        CODEX_ID_NORMAL_SHOCK_PRESSURE_RATIO,
        1.0 + (2.0 * gamma / (gamma + 1.0)) * (mach1_squared - 1.0),
    )
}

/// Density ratio across a normal shock, `rho2/rho1`.
///
/// Domain: upstream `mach1 > 1` and `gamma > 1`.
pub fn normal_shock_density_ratio_rho2_rho1(mach1: f64, gamma: f64) -> AeroResult<f64> {
    validate_normal_shock_inputs(mach1, gamma)?;
    let mach1_squared = normal_shock_mach1_squared(mach1, CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO)?;
    let numerator = ensure_positive_finite_term(
        CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO,
        (gamma + 1.0) * mach1_squared,
        "normal-shock density-ratio numerator was not positive and finite",
    )?;
    let denominator = ensure_positive_finite_term(
        CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO,
        (gamma - 1.0) * mach1_squared + 2.0,
        "normal-shock density-ratio denominator was not positive and finite",
    )?;
    ensure_positive_finite_result(CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO, numerator / denominator)
}

/// Static-temperature ratio across a normal shock, `T2/T1`.
///
/// Domain: upstream `mach1 > 1` and `gamma > 1`. This helper composes the
/// checked static-pressure and density ratios.
pub fn normal_shock_temperature_ratio_t2_t1(mach1: f64, gamma: f64) -> AeroResult<f64> {
    let pressure_ratio = normal_shock_pressure_ratio_p2_p1(mach1, gamma)?;
    let density_ratio = normal_shock_density_ratio_rho2_rho1(mach1, gamma)?;
    ensure_positive_finite_result(
        CODEX_ID_NORMAL_SHOCK_TEMPERATURE_RATIO,
        pressure_ratio / density_ratio,
    )
}

/// Total-pressure ratio across a normal shock, `p02/p01`.
///
/// Domain: upstream `mach1 > 1` and `gamma > 1`. The ratio represents total
/// pressure behind the shock divided by total pressure ahead of the shock; for a
/// physical normal shock it is positive and no greater than unity. Values within
/// a tiny roundoff allowance above unity near `Mach 1` are clamped to 1.
pub fn normal_shock_total_pressure_ratio_p02_p01(mach1: f64, gamma: f64) -> AeroResult<f64> {
    validate_normal_shock_inputs(mach1, gamma)?;
    let mach1_squared =
        normal_shock_mach1_squared(mach1, CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO)?;
    let density_like_term = ensure_positive_finite_term(
        CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO,
        ((gamma + 1.0) * mach1_squared) / ((gamma - 1.0) * mach1_squared + 2.0),
        "normal-shock total-pressure density-like term was not positive and finite",
    )?;
    let pressure_like_term = ensure_positive_finite_term(
        CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO,
        (gamma + 1.0) / (2.0 * gamma * mach1_squared - (gamma - 1.0)),
        "normal-shock total-pressure pressure-like term was not positive and finite",
    )?;
    let ratio = density_like_term.powf(gamma / (gamma - 1.0))
        * pressure_like_term.powf(1.0 / (gamma - 1.0));
    ensure_total_pressure_loss_ratio(CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO, ratio)
}

/// Mach angle, `mu = asin(1/M)`.
///
/// Domain: `Mach >= 1`. The sonic boundary returns 90 degrees. For increasing
/// finite supersonic Mach number the angle decreases toward zero.
pub fn mach_angle(mach: f64) -> AeroResult<Angle> {
    validation::require_supersonic(mach)?;
    let inverse_mach = 1.0 / mach;
    if !(inverse_mach.is_finite() && (0.0..=1.0).contains(&inverse_mach)) {
        return Err(AeroError::NumericalFailure {
            solver: CODEX_ID_MACH_ANGLE,
            reason: "inverse Mach term was outside the finite asin domain",
        });
    }
    ensure_angle_radians_in_range(
        CODEX_ID_MACH_ANGLE,
        inverse_mach.asin(),
        0.0,
        0.5 * PI,
        "Mach angle was not finite or inside 0 <= mu <= pi/2",
    )
}

/// Prandtl-Meyer expansion angle, `nu(M, gamma)`.
///
/// Domain: `Mach >= 1` and `gamma > 1`. The sonic boundary returns zero. This
/// direct scalar relation assumes a calorically perfect gas with constant
/// `gamma`; source traceability and reference-table validation remain pending.
pub fn prandtl_meyer_nu(mach: f64, gamma: f64) -> AeroResult<Angle> {
    validation::require_supersonic(mach)?;
    validate_gamma(gamma)?;
    let m2_minus_1 = mach_squared_minus_one(mach, CODEX_ID_PRANDTL_MEYER_NU)?;
    let coefficient_base = prandtl_meyer_coefficient_base(gamma)?;
    let scaled_term = ensure_nonnegative_finite_term(
        CODEX_ID_PRANDTL_MEYER_NU,
        m2_minus_1 / coefficient_base,
        "Prandtl-Meyer scaled Mach term was not nonnegative and finite",
    )?;
    let root_term = m2_minus_1.sqrt();
    let nu = coefficient_base.sqrt() * scaled_term.sqrt().atan() - root_term.atan();
    ensure_nonnegative_angle_result(
        CODEX_ID_PRANDTL_MEYER_NU,
        nu,
        "Prandtl-Meyer angle was not nonnegative and finite",
    )
}

/// Inverts the Prandtl-Meyer function to a Mach number by bracketing and bisection.
///
/// Domain: `0 <= nu < nu_max(gamma)`, `gamma > 1`, and `tolerance > 0`. The
/// zero-angle boundary returns Mach 1. The upper bracket is intentionally capped
/// in Phase 0.001 so failure to bracket is reported explicitly instead of
/// silently returning an extrapolated value.
pub fn prandtl_meyer_inverse(nu: Angle, gamma: f64, tolerance: f64) -> AeroResult<f64> {
    validate_gamma(gamma)?;
    validation::ensure_positive("tolerance", tolerance)?;
    let target = nu.as_radians();
    validation::ensure_nonnegative("nu", target)?;
    let nu_max = prandtl_meyer_max_radians(gamma)?;
    if target >= nu_max {
        return Err(AeroError::OutOfDomain {
            parameter: "nu",
            value: target,
            expected: "0 <= nu < Prandtl-Meyer maximum angle",
        });
    }
    if target == 0.0 {
        return Ok(1.0);
    }

    let mut low = 1.0;
    let mut high = 2.0;
    while prandtl_meyer_nu(high, gamma)?.as_radians() < target {
        if high >= PRANDTL_MEYER_INVERSE_MAX_BRACKET_MACH {
            return Err(AeroError::NumericalFailure {
                solver: CODEX_ID_PRANDTL_MEYER_INVERSE,
                reason: "failed to bracket target Prandtl-Meyer angle below maximum Mach limit",
            });
        }
        high = (2.0 * high).min(PRANDTL_MEYER_INVERSE_MAX_BRACKET_MACH);
        if !high.is_finite() {
            return Err(AeroError::NumericalFailure {
                solver: CODEX_ID_PRANDTL_MEYER_INVERSE,
                reason: "bracketing Mach value became nonfinite",
            });
        }
    }

    for _ in 0..PRANDTL_MEYER_INVERSE_MAX_ITERATIONS {
        let mid = 0.5 * (low + high);
        if !mid.is_finite() {
            return Err(AeroError::NumericalFailure {
                solver: CODEX_ID_PRANDTL_MEYER_INVERSE,
                reason: "bisection Mach value became nonfinite",
            });
        }
        let value = prandtl_meyer_nu(mid, gamma)?.as_radians();
        if (value - target).abs() <= tolerance {
            return ensure_positive_finite_result(CODEX_ID_PRANDTL_MEYER_INVERSE, mid);
        }
        if value < target {
            low = mid;
        } else {
            high = mid;
        }
    }
    Err(AeroError::NumericalFailure {
        solver: CODEX_ID_PRANDTL_MEYER_INVERSE,
        reason: "bisection did not converge within the configured iteration limit",
    })
}

fn validate_oblique_shock_primary_inputs(mach: f64, gamma: f64) -> AeroResult<()> {
    validation::require_strictly_supersonic(mach)?;
    validate_gamma(gamma)
}

fn validate_oblique_theta(theta: Angle, require_positive: bool) -> AeroResult<f64> {
    let theta_rad = theta.as_radians();
    if require_positive {
        validation::ensure_positive("theta", theta_rad)?;
    } else {
        validation::ensure_nonnegative("theta", theta_rad)?;
    }
    if theta_rad < 0.5 * PI {
        Ok(theta_rad)
    } else {
        Err(AeroError::OutOfDomain {
            parameter: "theta",
            value: theta_rad,
            expected: "0 <= theta < pi/2",
        })
    }
}

fn validate_oblique_beta(mach: f64, beta: Angle) -> AeroResult<f64> {
    let beta_rad = beta.as_radians();
    validation::ensure_finite("beta", beta_rad)?;
    let mach_angle_rad = mach_angle(mach)?.as_radians();
    if beta_rad > mach_angle_rad && beta_rad < 0.5 * PI {
        Ok(beta_rad)
    } else {
        Err(AeroError::OutOfDomain {
            parameter: "beta",
            value: beta_rad,
            expected: "Mach angle < beta < pi/2",
        })
    }
}

fn oblique_beta_search_bounds(mach: f64) -> AeroResult<(f64, f64)> {
    let mach_angle_rad = mach_angle(mach)?.as_radians();
    let lower = mach_angle_rad + OBLIQUE_SHOCK_BETA_EPSILON_RAD;
    let upper = 0.5 * PI - OBLIQUE_SHOCK_BETA_EPSILON_RAD;
    if lower < upper && lower.is_finite() && upper.is_finite() {
        Ok((lower, upper))
    } else {
        Err(AeroError::NumericalFailure {
            solver: CODEX_ID_OBLIQUE_SHOCK_BETA,
            reason: "oblique-shock beta search interval was empty or nonfinite",
        })
    }
}

fn push_unique_root(roots: &mut Vec<f64>, root: f64) -> AeroResult<()> {
    let root = ensure_finite_result(
        CODEX_ID_OBLIQUE_SHOCK_BETA,
        root,
        "oblique-shock root was nonfinite",
    )?;
    if roots
        .iter()
        .all(|existing| (*existing - root).abs() > 1.0e-7)
    {
        roots.push(root);
    }
    Ok(())
}

fn bisect_oblique_beta_root(
    mach: f64,
    theta: Angle,
    gamma: f64,
    mut low: f64,
    mut high: f64,
    mut f_low: f64,
    f_high: f64,
) -> AeroResult<f64> {
    if f_low.abs() <= OBLIQUE_SHOCK_RESIDUAL_TOLERANCE {
        return Ok(low);
    }
    if f_high.abs() <= OBLIQUE_SHOCK_RESIDUAL_TOLERANCE {
        return Ok(high);
    }
    if f_low.signum() == f_high.signum() {
        return Err(AeroError::NumericalFailure {
            solver: CODEX_ID_OBLIQUE_SHOCK_BETA,
            reason: "oblique-shock bisection was requested without a sign-changing bracket",
        });
    }

    for _ in 0..OBLIQUE_SHOCK_BETA_MAX_BISECTION_ITERATIONS {
        let mid = 0.5 * (low + high);
        if !mid.is_finite() {
            return Err(AeroError::NumericalFailure {
                solver: CODEX_ID_OBLIQUE_SHOCK_BETA,
                reason: "oblique-shock beta bisection midpoint became nonfinite",
            });
        }
        let f_mid = theta_beta_mach_residual(mach, Angle::from_radians(mid), gamma, theta)?;
        if f_mid.abs() <= OBLIQUE_SHOCK_RESIDUAL_TOLERANCE
            || (high - low).abs() <= OBLIQUE_SHOCK_BETA_EPSILON_RAD
        {
            return Ok(mid);
        }
        if f_low.signum() == f_mid.signum() {
            low = mid;
            f_low = f_mid;
        } else {
            high = mid;
        }
    }

    let root = 0.5 * (low + high);
    ensure_finite_result(
        CODEX_ID_OBLIQUE_SHOCK_BETA,
        root,
        "oblique-shock beta bisection did not produce a finite root",
    )
}

/// Theta-beta-Mach residual for an attached oblique shock.
///
/// Domain: `mach > 1`, `gamma > 1`, `Mach angle < beta < pi/2`, and
/// `0 <= theta < pi/2`. A zero residual indicates that the supplied wave angle
/// satisfies the constant-gamma theta-beta-Mach relation for the supplied
/// deflection angle. The helper returns a checked finite scalar residual rather
/// than allowing nonfinite trigonometric values to escape.
pub fn theta_beta_mach_residual(
    mach: f64,
    beta: Angle,
    gamma: f64,
    theta: Angle,
) -> AeroResult<f64> {
    validate_oblique_shock_primary_inputs(mach, gamma)?;
    let beta_rad = validate_oblique_beta(mach, beta)?;
    let theta_rad = validate_oblique_theta(theta, false)?;

    let mach_squared = normal_shock_mach1_squared(mach, CODEX_ID_OBLIQUE_SHOCK_RESIDUAL)?;
    let sin_beta = beta_rad.sin();
    let normal_mach_squared = ensure_positive_finite_term(
        CODEX_ID_OBLIQUE_SHOCK_RESIDUAL,
        mach_squared * sin_beta * sin_beta,
        "oblique-shock normal Mach squared was not positive and finite",
    )?;
    let numerator = ensure_positive_finite_term(
        CODEX_ID_OBLIQUE_SHOCK_RESIDUAL,
        2.0 * (normal_mach_squared - 1.0),
        "oblique-shock theta-beta-Mach numerator was not positive and finite",
    )?;
    let geometry_denominator = ensure_positive_finite_term(
        CODEX_ID_OBLIQUE_SHOCK_RESIDUAL,
        beta_rad.tan() * (mach_squared * (gamma + (2.0 * beta_rad).cos()) + 2.0),
        "oblique-shock theta-beta-Mach denominator was not positive and finite",
    )?;
    let theta_tangent = ensure_nonnegative_finite_term(
        CODEX_ID_OBLIQUE_SHOCK_RESIDUAL,
        theta_rad.tan(),
        "flow-deflection tangent was not nonnegative and finite",
    )?;
    ensure_finite_result(
        CODEX_ID_OBLIQUE_SHOCK_RESIDUAL,
        numerator / geometry_denominator - theta_tangent,
        "theta-beta-Mach residual was not finite",
    )
}

/// Solves the attached oblique-shock wave angle `beta` for an explicit branch.
///
/// Domain: `mach > 1`, `gamma > 1`, and `0 < theta < pi/2`. The `branch`
/// argument is mandatory so the solver never silently guesses weak versus
/// strong branch. If no attached branch is found for the requested deflection,
/// the function returns `AeroError::NumericalFailure` rather than `NaN`.
pub fn oblique_shock_beta(
    mach: f64,
    theta: Angle,
    gamma: f64,
    branch: ShockBranch,
) -> AeroResult<Angle> {
    validate_oblique_shock_primary_inputs(mach, gamma)?;
    let _theta_rad = validate_oblique_theta(theta, true)?;
    let (beta_min, beta_max) = oblique_beta_search_bounds(mach)?;

    let mut roots = Vec::new();
    let mut prev_beta = beta_min;
    let mut prev_value =
        theta_beta_mach_residual(mach, Angle::from_radians(prev_beta), gamma, theta)?;

    for i in 1..=OBLIQUE_SHOCK_BETA_SCAN_INTERVALS {
        let fraction = i as f64 / OBLIQUE_SHOCK_BETA_SCAN_INTERVALS as f64;
        let beta = beta_min + fraction * (beta_max - beta_min);
        let value = theta_beta_mach_residual(mach, Angle::from_radians(beta), gamma, theta)?;
        if prev_value.abs() <= OBLIQUE_SHOCK_RESIDUAL_TOLERANCE {
            push_unique_root(&mut roots, prev_beta)?;
        } else if value.abs() <= OBLIQUE_SHOCK_RESIDUAL_TOLERANCE {
            push_unique_root(&mut roots, beta)?;
        } else if prev_value.signum() != value.signum() {
            let root =
                bisect_oblique_beta_root(mach, theta, gamma, prev_beta, beta, prev_value, value)?;
            push_unique_root(&mut roots, root)?;
        }
        prev_beta = beta;
        prev_value = value;
    }

    if roots.is_empty() {
        return Err(AeroError::NumericalFailure {
            solver: CODEX_ID_OBLIQUE_SHOCK_BETA,
            reason: "no attached oblique-shock solution found for requested deflection",
        });
    }
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if roots.len() < 2 && matches!(branch, ShockBranch::Strong) {
        return Err(AeroError::NumericalFailure {
            solver: CODEX_ID_OBLIQUE_SHOCK_BETA,
            reason: "strong oblique-shock branch was not separately bracketed",
        });
    }
    let beta = match branch {
        ShockBranch::Weak => roots[0],
        ShockBranch::Strong => *roots.last().expect("roots is not empty"),
    };
    ensure_angle_radians_in_range(
        CODEX_ID_OBLIQUE_SHOCK_BETA,
        beta,
        beta_min,
        beta_max,
        "solved oblique-shock beta was outside the attached-shock search interval",
    )
}

/// Upstream normal Mach component for an attached oblique shock.
///
/// Domain: `mach > 1` and `Mach angle < beta < pi/2`. The returned normal
/// component is checked to be finite and strictly supersonic.
pub fn oblique_shock_normal_mach(mach: f64, beta: Angle) -> AeroResult<f64> {
    validation::require_strictly_supersonic(mach)?;
    let beta_rad = validate_oblique_beta(mach, beta)?;
    let normal_mach =
        ensure_positive_finite_result(CODEX_ID_OBLIQUE_SHOCK_NORMAL_MACH, mach * beta_rad.sin())?;
    validation::ensure_greater_than("normal_mach1", normal_mach, 1.0)?;
    Ok(normal_mach)
}

/// Downstream Mach number behind an attached oblique shock.
///
/// Domain: `mach > 1`, `gamma > 1`, `Mach angle < beta < pi/2`,
/// `0 < theta < pi/2`, and `beta > theta`. The calculation composes the
/// checked upstream normal Mach component with the reviewed normal-shock `M2n`
/// relation and the downstream flow geometry.
pub fn oblique_shock_downstream_mach(
    mach: f64,
    beta: Angle,
    theta: Angle,
    gamma: f64,
) -> AeroResult<f64> {
    validate_oblique_shock_primary_inputs(mach, gamma)?;
    let beta_rad = validate_oblique_beta(mach, beta)?;
    let theta_rad = validate_oblique_theta(theta, true)?;
    let normal_mach_1 = oblique_shock_normal_mach(mach, beta)?;
    let normal_mach_2 = normal_shock_mach2(normal_mach_1, gamma)?;
    let flow_deflection = beta_rad - theta_rad;
    if !(flow_deflection.is_finite() && flow_deflection > 0.0) {
        return Err(AeroError::OutOfDomain {
            parameter: "beta_minus_theta",
            value: flow_deflection,
            expected: "beta must exceed theta",
        });
    }
    let sine = ensure_positive_finite_term(
        CODEX_ID_OBLIQUE_SHOCK_DOWNSTREAM_MACH,
        flow_deflection.sin(),
        "downstream flow-angle sine was not positive and finite",
    )?;
    ensure_positive_finite_result(CODEX_ID_OBLIQUE_SHOCK_DOWNSTREAM_MACH, normal_mach_2 / sine)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_core::VerificationStatus;

    fn approx(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() <= tol, "{a} !~= {b}");
    }

    #[test]
    fn isentropic_ratios_equal_one_at_zero_mach() {
        approx(temperature_ratio_t0_over_t(0.0, 1.4).unwrap(), 1.0, 1.0e-12);
        approx(pressure_ratio_p0_over_p(0.0, 1.4).unwrap(), 1.0, 1.0e-12);
        approx(density_ratio_rho0_over_rho(0.0, 1.4).unwrap(), 1.0, 1.0e-12);
        approx(mass_flow_parameter(0.0, 1.4).unwrap(), 0.0, 1.0e-12);
    }

    #[test]
    fn isentropic_ratios_increase_with_mach() {
        assert!(
            temperature_ratio_t0_over_t(2.0, 1.4).unwrap()
                > temperature_ratio_t0_over_t(1.0, 1.4).unwrap()
        );
        assert!(
            pressure_ratio_p0_over_p(2.0, 1.4).unwrap()
                > pressure_ratio_p0_over_p(1.0, 1.4).unwrap()
        );
        assert!(
            density_ratio_rho0_over_rho(2.0, 1.4).unwrap()
                > density_ratio_rho0_over_rho(1.0, 1.4).unwrap()
        );
    }

    #[test]
    fn area_mach_ratio_is_one_at_sonic_condition() {
        approx(area_mach_ratio(1.0, 1.4).unwrap(), 1.0, 1.0e-12);
        assert!(area_mach_ratio(0.5, 1.4).unwrap() > 1.0);
        assert!(area_mach_ratio(2.0, 1.4).unwrap() > 1.0);
    }

    #[test]
    fn mass_flow_parameter_is_positive_for_positive_mach() {
        assert!(mass_flow_parameter(1.0, 1.4).unwrap() > 0.0);
        assert!(mass_flow_parameter(2.0, 1.4).unwrap() > 0.0);
    }

    #[test]
    fn isentropic_invalid_inputs_are_rejected() {
        assert!(temperature_ratio_t0_over_t(-1.0, 1.4).is_err());
        assert!(temperature_ratio_t0_over_t(1.0, 1.0).is_err());
        assert!(pressure_ratio_p0_over_p(f64::NAN, 1.4).is_err());
        assert!(density_ratio_rho0_over_rho(1.0, f64::INFINITY).is_err());
        assert!(area_mach_ratio(0.0, 1.4).is_err());
        assert!(area_mach_ratio(-1.0, 1.4).is_err());
        assert!(area_mach_ratio(1.0, 1.0).is_err());
        assert!(mass_flow_parameter(-0.1, 1.4).is_err());
        assert!(mass_flow_parameter(1.0, 1.0).is_err());
    }

    #[test]
    fn isentropic_nonfinite_outputs_return_numerical_failure() {
        assert!(matches!(
            temperature_ratio_t0_over_t(f64::MAX, 1.4),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            area_mach_ratio(f64::MAX, 1.4),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn isentropic_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_ISENTROPIC_TEMPERATURE_RATIO,
            CODEX_ID_ISENTROPIC_PRESSURE_RATIO,
            CODEX_ID_ISENTROPIC_DENSITY_RATIO,
            CODEX_ID_ISENTROPIC_AREA_MACH_RATIO,
            CODEX_ID_ISENTROPIC_MASS_FLOW_PARAMETER,
        ] {
            let record = verification_record(codex_id).unwrap();
            assert_eq!(record.codex_id, codex_id);
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert_eq!(record.sources, ISENTROPIC_SOURCES);
        }
        assert!(verification_record("gasdyn.unknown").is_none());
    }

    #[test]
    fn normal_shock_relations_match_representative_values() {
        let mach1 = 2.0;
        let gamma = 1.4;
        approx(
            normal_shock_mach2(mach1, gamma).unwrap(),
            0.577_350_269_189_625_7,
            1.0e-12,
        );
        approx(
            normal_shock_pressure_ratio_p2_p1(mach1, gamma).unwrap(),
            4.5,
            1.0e-12,
        );
        approx(
            normal_shock_density_ratio_rho2_rho1(mach1, gamma).unwrap(),
            2.666_666_666_666_667,
            1.0e-12,
        );
        approx(
            normal_shock_temperature_ratio_t2_t1(mach1, gamma).unwrap(),
            1.6875,
            1.0e-12,
        );
        approx(
            normal_shock_total_pressure_ratio_p02_p01(mach1, gamma).unwrap(),
            0.720_873_861_484_745_5,
            1.0e-12,
        );
    }

    #[test]
    fn normal_shock_relations_have_expected_physical_direction() {
        let mach1 = 2.0;
        let gamma = 1.4;
        assert!(normal_shock_mach2(mach1, gamma).unwrap() < 1.0);
        assert!(normal_shock_pressure_ratio_p2_p1(mach1, gamma).unwrap() > 1.0);
        assert!(normal_shock_density_ratio_rho2_rho1(mach1, gamma).unwrap() > 1.0);
        assert!(normal_shock_temperature_ratio_t2_t1(mach1, gamma).unwrap() > 1.0);
        assert!(normal_shock_total_pressure_ratio_p02_p01(mach1, gamma).unwrap() < 1.0);
    }

    #[test]
    fn normal_shock_near_sonic_is_near_identity() {
        let mach1 = 1.000_001;
        let gamma = 1.4;
        approx(
            normal_shock_pressure_ratio_p2_p1(mach1, gamma).unwrap(),
            1.0,
            1.0e-5,
        );
        approx(
            normal_shock_density_ratio_rho2_rho1(mach1, gamma).unwrap(),
            1.0,
            1.0e-5,
        );
        approx(
            normal_shock_temperature_ratio_t2_t1(mach1, gamma).unwrap(),
            1.0,
            1.0e-5,
        );
        approx(
            normal_shock_total_pressure_ratio_p02_p01(mach1, gamma).unwrap(),
            1.0,
            1.0e-12,
        );
    }

    #[test]
    fn normal_shock_invalid_inputs_are_rejected() {
        for mach1 in [0.0, 0.8, 1.0] {
            assert!(normal_shock_mach2(mach1, 1.4).is_err());
            assert!(normal_shock_pressure_ratio_p2_p1(mach1, 1.4).is_err());
            assert!(normal_shock_density_ratio_rho2_rho1(mach1, 1.4).is_err());
            assert!(normal_shock_temperature_ratio_t2_t1(mach1, 1.4).is_err());
            assert!(normal_shock_total_pressure_ratio_p02_p01(mach1, 1.4).is_err());
        }
        assert!(normal_shock_mach2(f64::NAN, 1.4).is_err());
        assert!(normal_shock_pressure_ratio_p2_p1(2.0, 1.0).is_err());
        assert!(normal_shock_density_ratio_rho2_rho1(2.0, f64::INFINITY).is_err());
        assert!(normal_shock_temperature_ratio_t2_t1(2.0, 1.0).is_err());
        assert!(normal_shock_total_pressure_ratio_p02_p01(2.0, 1.0).is_err());
    }

    #[test]
    fn normal_shock_nonfinite_outputs_return_numerical_failure() {
        assert!(matches!(
            normal_shock_mach2(f64::MAX, 1.4),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            normal_shock_pressure_ratio_p2_p1(f64::MAX, 1.4),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            normal_shock_density_ratio_rho2_rho1(f64::MAX, 1.4),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(matches!(
            normal_shock_total_pressure_ratio_p02_p01(f64::MAX, 1.4),
            Err(AeroError::NumericalFailure { .. })
        ));
    }

    #[test]
    fn normal_shock_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_NORMAL_SHOCK_MACH2,
            CODEX_ID_NORMAL_SHOCK_PRESSURE_RATIO,
            CODEX_ID_NORMAL_SHOCK_DENSITY_RATIO,
            CODEX_ID_NORMAL_SHOCK_TEMPERATURE_RATIO,
            CODEX_ID_NORMAL_SHOCK_TOTAL_PRESSURE_RATIO,
        ] {
            let record = verification_record(codex_id).unwrap();
            assert_eq!(record.codex_id, codex_id);
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert_eq!(record.sources, NORMAL_SHOCK_SOURCES);
        }
    }

    #[test]
    fn mach_angle_has_expected_boundary_and_representative_values() {
        approx(mach_angle(1.0).unwrap().as_degrees(), 90.0, 1.0e-10);
        approx(mach_angle(2.0).unwrap().as_degrees(), 30.0, 1.0e-12);
        assert!(mach_angle(3.0).unwrap().as_degrees() < mach_angle(2.0).unwrap().as_degrees());
    }

    #[test]
    fn prandtl_meyer_direct_relation_has_expected_values() {
        approx(
            prandtl_meyer_nu(1.0, 1.4).unwrap().as_radians(),
            0.0,
            1.0e-12,
        );
        approx(
            prandtl_meyer_nu(2.0, 1.4).unwrap().as_degrees(),
            26.379_760_813_416_457,
            1.0e-12,
        );
        assert!(
            prandtl_meyer_nu(2.0, 1.4).unwrap().as_radians()
                > prandtl_meyer_nu(1.5, 1.4).unwrap().as_radians()
        );
    }

    #[test]
    fn prandtl_meyer_inverse_round_trips() {
        approx(
            prandtl_meyer_inverse(Angle::ZERO, 1.4, 1.0e-10).unwrap(),
            1.0,
            1.0e-12,
        );
        let nu = prandtl_meyer_nu(2.25, 1.4).unwrap();
        approx(
            prandtl_meyer_inverse(nu, 1.4, 1.0e-10).unwrap(),
            2.25,
            1.0e-8,
        );
    }

    #[test]
    fn mach_angle_and_prandtl_meyer_invalid_inputs_are_rejected() {
        assert!(mach_angle(0.999).is_err());
        assert!(mach_angle(f64::NAN).is_err());
        assert!(prandtl_meyer_nu(0.999, 1.4).is_err());
        assert!(prandtl_meyer_nu(2.0, 1.0).is_err());
        assert!(matches!(
            prandtl_meyer_nu(f64::MAX, 1.4),
            Err(AeroError::NumericalFailure { .. })
        ));
        assert!(prandtl_meyer_inverse(Angle::from_radians(-1.0e-6), 1.4, 1.0e-10).is_err());
        assert!(prandtl_meyer_inverse(Angle::from_degrees(10.0), 1.0, 1.0e-10).is_err());
        assert!(prandtl_meyer_inverse(Angle::from_degrees(10.0), 1.4, 0.0).is_err());
        let nu_max = prandtl_meyer_max_radians(1.4).unwrap();
        assert!(prandtl_meyer_inverse(Angle::from_radians(nu_max), 1.4, 1.0e-10).is_err());
    }

    #[test]
    fn prandtl_meyer_inverse_reports_bracket_failure_as_numerical_failure() {
        let target = prandtl_meyer_max_radians(1.4).unwrap() - 1.0e-9;
        assert!(matches!(
            prandtl_meyer_inverse(Angle::from_radians(target), 1.4, 1.0e-12),
            Err(AeroError::NumericalFailure {
                solver: CODEX_ID_PRANDTL_MEYER_INVERSE,
                ..
            })
        ));
    }

    #[test]
    fn mach_angle_and_prandtl_meyer_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_MACH_ANGLE,
            CODEX_ID_PRANDTL_MEYER_NU,
            CODEX_ID_PRANDTL_MEYER_INVERSE,
        ] {
            let record = verification_record(codex_id).unwrap();
            assert_eq!(record.codex_id, codex_id);
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert_eq!(record.sources, EXPANSION_FLOW_SOURCES);
        }
    }

    #[test]
    fn oblique_shock_solver_branches_are_explicit_and_representative() {
        let theta = Angle::from_degrees(10.0);
        let weak = oblique_shock_beta(2.0, theta, 1.4, ShockBranch::Weak).unwrap();
        let strong = oblique_shock_beta(2.0, theta, 1.4, ShockBranch::Strong).unwrap();
        assert_eq!(ShockBranch::Weak.as_str(), "weak");
        assert_eq!(ShockBranch::Strong.as_str(), "strong");
        assert!(weak.as_radians() < strong.as_radians());
        approx(weak.as_degrees(), 39.313_931_844_818_9, 1.0e-8);
        approx(strong.as_degrees(), 83.700_080_375_746_9, 1.0e-8);
        approx(
            theta_beta_mach_residual(2.0, weak, 1.4, theta).unwrap(),
            0.0,
            1.0e-10,
        );
        approx(
            theta_beta_mach_residual(2.0, strong, 1.4, theta).unwrap(),
            0.0,
            1.0e-10,
        );
    }

    #[test]
    fn oblique_shock_normal_component_and_downstream_mach_are_checked() {
        let theta = Angle::from_degrees(10.0);
        let weak = oblique_shock_beta(2.0, theta, 1.4, ShockBranch::Weak).unwrap();
        let strong = oblique_shock_beta(2.0, theta, 1.4, ShockBranch::Strong).unwrap();
        assert!(oblique_shock_normal_mach(2.0, weak).unwrap() > 1.0);
        assert!(oblique_shock_normal_mach(2.0, strong).unwrap() > 1.0);
        approx(
            oblique_shock_downstream_mach(2.0, weak, theta, 1.4).unwrap(),
            1.640_522_229_001_08,
            1.0e-8,
        );
        approx(
            oblique_shock_downstream_mach(2.0, strong, theta, 1.4).unwrap(),
            0.603_697_643_106_26,
            1.0e-8,
        );
    }

    #[test]
    fn oblique_shock_invalid_inputs_are_rejected() {
        let theta = Angle::from_degrees(10.0);
        assert!(oblique_shock_beta(0.8, theta, 1.4, ShockBranch::Weak).is_err());
        assert!(oblique_shock_beta(2.0, Angle::ZERO, 1.4, ShockBranch::Weak).is_err());
        assert!(oblique_shock_beta(2.0, theta, 1.0, ShockBranch::Weak).is_err());
        assert!(oblique_shock_beta(f64::NAN, theta, 1.4, ShockBranch::Weak).is_err());
        assert!(theta_beta_mach_residual(0.9, Angle::from_degrees(45.0), 1.4, theta).is_err());
        assert!(theta_beta_mach_residual(2.0, Angle::from_degrees(20.0), 1.4, theta).is_err());
        assert!(oblique_shock_normal_mach(0.8, Angle::from_degrees(45.0)).is_err());
        assert!(oblique_shock_normal_mach(2.0, Angle::from_degrees(20.0)).is_err());
        assert!(oblique_shock_downstream_mach(
            2.0,
            Angle::from_degrees(45.0),
            Angle::from_degrees(50.0),
            1.4
        )
        .is_err());
    }

    #[test]
    fn oblique_shock_too_large_theta_returns_failure_not_nan() {
        let result = oblique_shock_beta(2.0, Angle::from_degrees(80.0), 1.4, ShockBranch::Weak);
        assert!(matches!(
            result,
            Err(AeroError::NumericalFailure {
                solver: CODEX_ID_OBLIQUE_SHOCK_BETA,
                ..
            })
        ));
    }

    #[test]
    fn oblique_shock_verification_records_remain_research_required() {
        for codex_id in [
            CODEX_ID_OBLIQUE_SHOCK_THETA_BETA_MACH_RESIDUAL,
            CODEX_ID_OBLIQUE_SHOCK_BETA,
            CODEX_ID_OBLIQUE_SHOCK_NORMAL_MACH,
            CODEX_ID_OBLIQUE_SHOCK_DOWNSTREAM_MACH,
        ] {
            let record = verification_record(codex_id).unwrap();
            assert_eq!(record.codex_id, codex_id);
            assert_eq!(record.status, VerificationStatus::ResearchRequired);
            assert_eq!(record.sources, OBLIQUE_SHOCK_SOURCES);
        }
    }
}
