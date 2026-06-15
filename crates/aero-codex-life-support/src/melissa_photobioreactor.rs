//! MELiSSA C4a Limnospira indica photobioreactor and integration helpers.
//!
//! This module implements the control-oriented thin-film report equations for
//! light transfer, reduced C4a dynamics, carbonate speciation, gas-balance oxygen
//! production, and simple local controllers. Citation keys are retained in
//! Rustdoc and in `thinfilm_provenance::EQUATION_REFERENCES`.

use aero_codex_core::{validation, AeroError, AeroResult, EngineeringResult, ValidityStatus};

use crate::thinfilm_provenance::{self as provenance, ids};

fn numerical_failure(codex_id: &'static str, reason: &'static str) -> AeroError {
    AeroError::NumericalFailure {
        solver: codex_id,
        reason,
    }
}

fn result<T>(
    value: T,
    codex_id: &'static str,
    validity: ValidityStatus,
) -> AeroResult<EngineeringResult<T>> {
    let record = provenance::verification_record(codex_id).ok_or(AeroError::UnverifiedSource {
        source_id: provenance::SOURCE_THINFILM_REPORT_2026,
    })?;
    Ok(EngineeringResult::new(value, codex_id, record).with_validity(validity))
}

fn ensure_fraction(parameter: &'static str, value: f64) -> AeroResult<()> {
    validation::ensure_range(parameter, value, 0.0, 1.0, "fraction in [0, 1]")
}

fn ensure_min_le_max(codex_id: &'static str, min: f64, max: f64) -> AeroResult<()> {
    validation::ensure_finite("min", min)?;
    validation::ensure_finite("max", max)?;
    if min <= max {
        Ok(())
    } else {
        Err(numerical_failure(
            codex_id,
            "lower saturation bound must be <= upper saturation bound",
        ))
    }
}

fn checked_exp(codex_id: &'static str, value: f64) -> AeroResult<f64> {
    let out = value.exp();
    if out.is_finite() {
        Ok(out)
    } else {
        Err(numerical_failure(codex_id, "exponential overflow"))
    }
}

fn modified_bessel_i0(value: f64) -> AeroResult<f64> {
    validation::ensure_finite("bessel_argument", value)?;
    let y = 0.25 * value * value;
    let mut term = 1.0;
    let mut sum = 1.0;
    for k in 1..=80 {
        let kf = k as f64;
        term *= y / (kf * kf);
        sum += term;
        if term.abs() <= 1.0e-15 * sum.abs().max(1.0) {
            return Ok(sum);
        }
        if !sum.is_finite() {
            return Err(numerical_failure(
                ids::CYLINDRICAL_TWO_FLUX_IRRADIANCE_RATIO,
                "I0 Bessel series overflow",
            ));
        }
    }
    Ok(sum)
}

fn modified_bessel_i1(value: f64) -> AeroResult<f64> {
    validation::ensure_finite("bessel_argument", value)?;
    if value == 0.0 {
        return Ok(0.0);
    }
    let y = 0.25 * value * value;
    let mut term = 0.5 * value;
    let mut sum = term;
    for k in 1..=80 {
        let kf = k as f64;
        term *= y / (kf * (kf + 1.0));
        sum += term;
        if term.abs() <= 1.0e-15 * sum.abs().max(1.0) {
            return Ok(sum);
        }
        if !sum.is_finite() {
            return Err(numerical_failure(
                ids::CYLINDRICAL_TWO_FLUX_IRRADIANCE_RATIO,
                "I1 Bessel series overflow",
            ));
        }
    }
    Ok(sum)
}

/// Flat-culture two-flux irradiance ratio `G_z / q_0`.
///
/// Implements thin-film report Eq. (6). Inputs use the report notation: `n` is
/// the phase-function exponent, `alpha` and `delta` are the combined
/// absorption/scattering parameters, `optical_path_length_m = L`, and
/// `depth_m = z`.
///
/// Citation key preserved: `poughon2021`.
pub fn flat_two_flux_irradiance_ratio(
    phase_exponent_n: f64,
    alpha: f64,
    delta_per_m: f64,
    optical_path_length_m: f64,
    depth_m: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::FLAT_TWO_FLUX_IRRADIANCE_RATIO;
    validation::ensure_greater_than("phase_exponent_n", phase_exponent_n, -1.0)?;
    validation::ensure_finite("alpha", alpha)?;
    validation::ensure_nonnegative("delta_per_m", delta_per_m)?;
    validation::ensure_positive("optical_path_length_m", optical_path_length_m)?;
    validation::ensure_range(
        "depth_m",
        depth_m,
        0.0,
        optical_path_length_m,
        "0 <= depth_m <= optical_path_length_m",
    )?;

    let l_minus_z = optical_path_length_m - depth_m;
    let exp_forward = checked_exp(ID, delta_per_m * l_minus_z)?;
    let exp_backward = checked_exp(ID, -delta_per_m * l_minus_z)?;
    let exp_l = checked_exp(ID, delta_per_m * optical_path_length_m)?;
    let exp_neg_l = checked_exp(ID, -delta_per_m * optical_path_length_m)?;
    let numerator = (1.0 + alpha) * exp_forward - (1.0 - alpha) * exp_backward;
    let denominator = (1.0 + alpha).powi(2) * exp_l - (1.0 - alpha).powi(2) * exp_neg_l;
    if denominator.abs() <= f64::EPSILON {
        return Err(numerical_failure(
            ID,
            "two-flux denominator is numerically zero",
        ));
    }
    let value = 2.0 * (phase_exponent_n + 2.0) / (phase_exponent_n + 1.0) * numerator / denominator;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "irradiance ratio was not finite"))
    }
}

/// Cylindrical-culture two-flux irradiance ratio `G_r / q_0`.
///
/// Implements thin-film report Eq. (7). Modified Bessel functions `I0` and `I1`
/// are evaluated with pure-Rust power series to keep the AeroCodex kernel free
/// of native or external numerical dependencies.
///
/// Citation key preserved: `poughon2021`.
pub fn cylindrical_two_flux_irradiance_ratio(
    phase_exponent_n: f64,
    alpha: f64,
    delta_per_m: f64,
    radius_m: f64,
    optical_path_length_m: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::CYLINDRICAL_TWO_FLUX_IRRADIANCE_RATIO;
    validation::ensure_greater_than("phase_exponent_n", phase_exponent_n, -1.0)?;
    validation::ensure_finite("alpha", alpha)?;
    validation::ensure_nonnegative("delta_per_m", delta_per_m)?;
    validation::ensure_nonnegative("radius_m", radius_m)?;
    validation::ensure_positive("optical_path_length_m", optical_path_length_m)?;
    if radius_m > optical_path_length_m {
        return Err(AeroError::OutOfDomain {
            parameter: "radius_m",
            value: radius_m,
            expected: "0 <= radius_m <= optical_path_length_m",
        });
    }

    let numerator_bessel = modified_bessel_i0(delta_per_m * radius_m)?;
    let denominator_bessel = modified_bessel_i0(delta_per_m * optical_path_length_m)?
        + alpha * modified_bessel_i1(delta_per_m * optical_path_length_m)?;
    if denominator_bessel.abs() <= f64::EPSILON {
        return Err(numerical_failure(
            ID,
            "Bessel denominator is numerically zero",
        ));
    }
    let value = 2.0 * (phase_exponent_n + 2.0) / (phase_exponent_n + 1.0) * numerator_bessel
        / denominator_bessel;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "cylindrical irradiance ratio was not finite",
        ))
    }
}

/// Geometric correction `c = min(1, L_0/L)`.
///
/// Implements thin-film report Eq. (8).
///
/// Citation key preserved: `poughon2021`.
pub fn geometric_correction(
    calibrated_path_length_m: f64,
    optical_path_length_m: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::GEOMETRIC_CORRECTION;
    validation::ensure_nonnegative("calibrated_path_length_m", calibrated_path_length_m)?;
    validation::ensure_positive("optical_path_length_m", optical_path_length_m)?;
    let value = (calibrated_path_length_m / optical_path_length_m).min(1.0);
    result(value, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Reduced Limnospira biomass rate `(mu - D - k_d) X`.
///
/// Implements thin-film report Eq. (9).
///
/// Citation key preserved: `poughon2021`.
pub fn limnospira_biomass_rate(
    specific_growth_rate_per_s: f64,
    dilution_rate_per_s: f64,
    decay_rate_per_s: f64,
    biomass_concentration: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::LIMNOSPIRA_BIOMASS_RATE;
    validation::ensure_nonnegative("specific_growth_rate_per_s", specific_growth_rate_per_s)?;
    validation::ensure_nonnegative("dilution_rate_per_s", dilution_rate_per_s)?;
    validation::ensure_nonnegative("decay_rate_per_s", decay_rate_per_s)?;
    validation::ensure_nonnegative("biomass_concentration", biomass_concentration)?;
    let value = (specific_growth_rate_per_s - dilution_rate_per_s - decay_rate_per_s)
        * biomass_concentration;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "biomass rate was not finite"))
    }
}

/// Inputs for the reduced total inorganic carbon rate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LimnospiraTicRateInput {
    pub dilution_rate_per_s: f64,
    pub total_inorganic_carbon_in: f64,
    pub total_inorganic_carbon: f64,
    pub specific_carbon_uptake_per_biomass_s: f64,
    pub biomass_concentration: f64,
    pub kla_co2_per_s: f64,
    pub saturation_co2: f64,
    pub dissolved_co2: f64,
}

/// Reduced total inorganic carbon rate for the C4a photobioreactor.
///
/// Implements thin-film report Eq. (10):
/// `D(C_T,in - C_T) - q_C X + kLa_CO2(C*_CO2 - C_CO2)`.
///
/// Citation key preserved: `poughon2021`.
pub fn limnospira_tic_rate(input: LimnospiraTicRateInput) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::LIMNOSPIRA_TIC_RATE;
    validation::ensure_nonnegative("dilution_rate_per_s", input.dilution_rate_per_s)?;
    validation::ensure_nonnegative("total_inorganic_carbon_in", input.total_inorganic_carbon_in)?;
    validation::ensure_nonnegative("total_inorganic_carbon", input.total_inorganic_carbon)?;
    validation::ensure_nonnegative(
        "specific_carbon_uptake_per_biomass_s",
        input.specific_carbon_uptake_per_biomass_s,
    )?;
    validation::ensure_nonnegative("biomass_concentration", input.biomass_concentration)?;
    validation::ensure_nonnegative("kla_co2_per_s", input.kla_co2_per_s)?;
    validation::ensure_nonnegative("saturation_co2", input.saturation_co2)?;
    validation::ensure_nonnegative("dissolved_co2", input.dissolved_co2)?;

    let value = input.dilution_rate_per_s
        * (input.total_inorganic_carbon_in - input.total_inorganic_carbon)
        - input.specific_carbon_uptake_per_biomass_s * input.biomass_concentration
        + input.kla_co2_per_s * (input.saturation_co2 - input.dissolved_co2);
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "TIC rate was not finite"))
    }
}

/// Reduced dissolved oxygen rate for the C4a photobioreactor.
///
/// Implements thin-film report Eq. (11):
/// `q_O2 X - D C_O2 + kLa_O2(C*_O2 - C_O2)`.
///
/// Citation key preserved: `poughon2021`.
pub fn limnospira_oxygen_rate(
    specific_oxygen_production_per_biomass_s: f64,
    biomass_concentration: f64,
    dilution_rate_per_s: f64,
    dissolved_oxygen: f64,
    kla_o2_per_s: f64,
    saturation_o2: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::LIMNOSPIRA_OXYGEN_RATE;
    validation::ensure_nonnegative(
        "specific_oxygen_production_per_biomass_s",
        specific_oxygen_production_per_biomass_s,
    )?;
    validation::ensure_nonnegative("biomass_concentration", biomass_concentration)?;
    validation::ensure_nonnegative("dilution_rate_per_s", dilution_rate_per_s)?;
    validation::ensure_nonnegative("dissolved_oxygen", dissolved_oxygen)?;
    validation::ensure_nonnegative("kla_o2_per_s", kla_o2_per_s)?;
    validation::ensure_nonnegative("saturation_o2", saturation_o2)?;

    let value = specific_oxygen_production_per_biomass_s * biomass_concentration
        - dilution_rate_per_s * dissolved_oxygen
        + kla_o2_per_s * (saturation_o2 - dissolved_oxygen);
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "oxygen rate was not finite"))
    }
}

/// Carbonate speciation result for thin-film report Eq. (12)-(13).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarbonateSpecies {
    pub alpha_co2: f64,
    pub alpha_hco3: f64,
    pub alpha_co3: f64,
    pub co2: f64,
    pub hco3: f64,
    pub co3: f64,
}

/// Splits total inorganic carbon into CO2, HCO3-, and CO3-- by pH.
///
/// Implements thin-film report Eq. (12)-(13). The caller supplies `p_ka1` and
/// `p_ka2` for the selected temperature, salinity, and activity convention.
///
/// Citation key preserved: `poughon2021`.
pub fn carbonate_species_from_total_and_ph(
    total_inorganic_carbon: f64,
    ph: f64,
    p_ka1: f64,
    p_ka2: f64,
) -> AeroResult<EngineeringResult<CarbonateSpecies>> {
    const ID: &str = ids::CARBONATE_SPECIES;
    validation::ensure_nonnegative("total_inorganic_carbon", total_inorganic_carbon)?;
    validation::ensure_range(
        "ph",
        ph,
        0.0,
        14.5,
        "aqueous pH in a documented model range",
    )?;
    validation::ensure_range("p_ka1", p_ka1, 0.0, 14.5, "pKa in a documented model range")?;
    validation::ensure_range("p_ka2", p_ka2, 0.0, 14.5, "pKa in a documented model range")?;

    let h = 10.0_f64.powf(-ph);
    let ka1 = 10.0_f64.powf(-p_ka1);
    let ka2 = 10.0_f64.powf(-p_ka2);
    let denominator = h * h + ka1 * h + ka1 * ka2;
    if denominator <= 0.0 || !denominator.is_finite() {
        return Err(numerical_failure(ID, "carbonate denominator invalid"));
    }
    let alpha_co2 = h * h / denominator;
    let alpha_hco3 = ka1 * h / denominator;
    let alpha_co3 = ka1 * ka2 / denominator;
    let species = CarbonateSpecies {
        alpha_co2,
        alpha_hco3,
        alpha_co3,
        co2: alpha_co2 * total_inorganic_carbon,
        hco3: alpha_hco3 * total_inorganic_carbon,
        co3: alpha_co3 * total_inorganic_carbon,
    };
    result(species, ID, ValidityStatus::WithinDocumentedDomain)
}

/// EPS fraction relation `x_EPS = 1.33(P/2e- - 1.23)`.
///
/// Implements thin-film report Eq. (14). Negative values are returned with
/// outside-domain validity because a physical fraction should be nonnegative.
///
/// Citation key preserved: `poughon2021`.
pub fn eps_fraction_from_photon_requirement(
    photon_requirement_per_two_electrons: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::EPS_FRACTION;
    validation::ensure_finite(
        "photon_requirement_per_two_electrons",
        photon_requirement_per_two_electrons,
    )?;
    let value = 1.33 * (photon_requirement_per_two_electrons - 1.23);
    let validity = if value >= 0.0 {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::OutsideDocumentedDomain
    };
    let mut out = result(value, ID, validity)?;
    if value < 0.0 {
        out = out.with_warning(
            "eps_fraction.negative",
            "EPS relation produced a negative fraction; verify operating region and model basis",
        );
    }
    Ok(out)
}

/// Tank-in-series PBR concentration rate for species `i`.
///
/// Implements thin-film report Eq. (15):
/// `Q/V_n(C_{i,n-1} - C_{i,n}) + sum_j nu_ij r_j + kLa_i(C*_i - C_i)`.
///
/// Citation key preserved: `poughon2021`.
pub fn tank_series_concentration_rate(
    flow_over_volume_per_s: f64,
    upstream_concentration: f64,
    current_concentration: f64,
    stoichiometric_coefficients: &[f64],
    reaction_rates: &[f64],
    kla_per_s: f64,
    saturation_concentration: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::PBR_TANK_SERIES_RATE;
    validation::ensure_nonnegative("flow_over_volume_per_s", flow_over_volume_per_s)?;
    validation::ensure_nonnegative("upstream_concentration", upstream_concentration)?;
    validation::ensure_nonnegative("current_concentration", current_concentration)?;
    validation::ensure_nonnegative("kla_per_s", kla_per_s)?;
    validation::ensure_nonnegative("saturation_concentration", saturation_concentration)?;
    if stoichiometric_coefficients.len() != reaction_rates.len() {
        return Err(numerical_failure(
            ID,
            "stoichiometric and reaction-rate arrays must have the same length",
        ));
    }
    for value in stoichiometric_coefficients.iter().chain(reaction_rates) {
        validation::ensure_finite("stoichiometric_or_rate", *value)?;
    }
    let reaction_sum = stoichiometric_coefficients
        .iter()
        .zip(reaction_rates)
        .try_fold(0.0, |acc, (nu, rate)| {
            let next = acc + nu * rate;
            if next.is_finite() {
                Ok(next)
            } else {
                Err(numerical_failure(ID, "reaction sum was not finite"))
            }
        })?;
    let value = flow_over_volume_per_s * (upstream_concentration - current_concentration)
        + reaction_sum
        + kla_per_s * (saturation_concentration - current_concentration);
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "tank-series concentration rate was not finite",
        ))
    }
}

/// Operating-point inputs for the PBR feasibility region.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PbrOperatingPoint {
    pub total_inorganic_carbon: f64,
    pub total_inorganic_carbon_min: f64,
    pub ph: f64,
    pub ph_max: f64,
    pub photon_requirement_per_two_electrons: f64,
    pub photon_requirement_max: f64,
    pub oxygen_production_rate: f64,
    pub required_oxygen_production_rate: f64,
}

/// Evaluates the reduced feasibility-map predicate from thin-film report Eq. (16).
///
/// Citation key preserved: `poughon2021`.
pub fn pbr_operating_point_feasible(
    input: PbrOperatingPoint,
) -> AeroResult<EngineeringResult<bool>> {
    const ID: &str = ids::PBR_FEASIBLE_REGION;
    validation::ensure_nonnegative("total_inorganic_carbon", input.total_inorganic_carbon)?;
    validation::ensure_nonnegative(
        "total_inorganic_carbon_min",
        input.total_inorganic_carbon_min,
    )?;
    validation::ensure_range("ph", input.ph, 0.0, 14.5, "aqueous pH range")?;
    validation::ensure_range("ph_max", input.ph_max, 0.0, 14.5, "aqueous pH range")?;
    validation::ensure_finite(
        "photon_requirement_per_two_electrons",
        input.photon_requirement_per_two_electrons,
    )?;
    validation::ensure_finite("photon_requirement_max", input.photon_requirement_max)?;
    validation::ensure_finite("oxygen_production_rate", input.oxygen_production_rate)?;
    validation::ensure_finite(
        "required_oxygen_production_rate",
        input.required_oxygen_production_rate,
    )?;
    let feasible = input.total_inorganic_carbon > input.total_inorganic_carbon_min
        && input.ph < input.ph_max
        && input.photon_requirement_per_two_electrons < input.photon_requirement_max
        && input.oxygen_production_rate >= input.required_oxygen_production_rate;
    let validity = if feasible {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::OutsideDocumentedDomain
    };
    result(feasible, ID, validity)
}

/// Disconnected gas-loop oxygen production balance.
///
/// Implements thin-film report Eq. (27):
/// `r_O2 = ((F_in - F_CO2) 0.2089 - F_out y_O2,out) / (V_m MW_O2 V_R)`.
/// Keep all units consistent with the report-derived denominator convention.
///
/// Citation key preserved: `garcia2021`.
pub fn oxygen_production_rate_disconnected_gas(
    inlet_flow: f64,
    co2_feed_flow: f64,
    outlet_flow: f64,
    outlet_o2_fraction: f64,
    molar_volume: f64,
    oxygen_molecular_weight: f64,
    reactor_volume: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::OXYGEN_PRODUCTION_DISCONNECTED;
    validation::ensure_nonnegative("inlet_flow", inlet_flow)?;
    validation::ensure_nonnegative("co2_feed_flow", co2_feed_flow)?;
    validation::ensure_nonnegative("outlet_flow", outlet_flow)?;
    ensure_fraction("outlet_o2_fraction", outlet_o2_fraction)?;
    validation::ensure_positive("molar_volume", molar_volume)?;
    validation::ensure_positive("oxygen_molecular_weight", oxygen_molecular_weight)?;
    validation::ensure_positive("reactor_volume", reactor_volume)?;
    let denominator = molar_volume * oxygen_molecular_weight * reactor_volume;
    let value =
        ((inlet_flow - co2_feed_flow) * 0.2089 - outlet_flow * outlet_o2_fraction) / denominator;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "oxygen production rate was not finite",
        ))
    }
}

/// Connected gas-loop oxygen production balance.
///
/// Implements thin-film report Eq. (28):
/// `r_O2 = (F_in y_O2,in - F_out y_O2,out) / (V_m MW_O2 V_R)`.
///
/// Citation key preserved: `garcia2021`.
pub fn oxygen_production_rate_connected_gas(
    inlet_flow: f64,
    inlet_o2_fraction: f64,
    outlet_flow: f64,
    outlet_o2_fraction: f64,
    molar_volume: f64,
    oxygen_molecular_weight: f64,
    reactor_volume: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::OXYGEN_PRODUCTION_CONNECTED;
    validation::ensure_nonnegative("inlet_flow", inlet_flow)?;
    validation::ensure_nonnegative("outlet_flow", outlet_flow)?;
    ensure_fraction("inlet_o2_fraction", inlet_o2_fraction)?;
    ensure_fraction("outlet_o2_fraction", outlet_o2_fraction)?;
    validation::ensure_positive("molar_volume", molar_volume)?;
    validation::ensure_positive("oxygen_molecular_weight", oxygen_molecular_weight)?;
    validation::ensure_positive("reactor_volume", reactor_volume)?;
    let denominator = molar_volume * oxygen_molecular_weight * reactor_volume;
    let value = (inlet_flow * inlet_o2_fraction - outlet_flow * outlet_o2_fraction) / denominator;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "connected oxygen production rate was not finite",
        ))
    }
}

/// Oxygen mole-fraction control error `e = y_sp - y_animal`.
///
/// Implements thin-film report Eq. (29).
///
/// Citation key preserved: `garcia2021`.
pub fn oxygen_fraction_error(
    setpoint_fraction: f64,
    measured_fraction: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::OXYGEN_FRACTION_ERROR;
    ensure_fraction("setpoint_fraction", setpoint_fraction)?;
    ensure_fraction("measured_fraction", measured_fraction)?;
    result(
        setpoint_fraction - measured_fraction,
        ID,
        ValidityStatus::WithinDocumentedDomain,
    )
}

/// Saturated PI light command for the C4a-animal gas-loop abstraction.
///
/// Implements thin-film report Eq. (30). `integral_error` is the already
/// integrated error term in the caller's time basis.
///
/// Citation key preserved: `garcia2021`.
pub fn pi_light_command(
    base_light_flux: f64,
    proportional_gain: f64,
    integral_gain: f64,
    error: f64,
    integral_error: f64,
    min_light_flux: f64,
    max_light_flux: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::PI_LIGHT_COMMAND;
    validation::ensure_finite("base_light_flux", base_light_flux)?;
    validation::ensure_finite("proportional_gain", proportional_gain)?;
    validation::ensure_finite("integral_gain", integral_gain)?;
    validation::ensure_finite("error", error)?;
    validation::ensure_finite("integral_error", integral_error)?;
    ensure_min_le_max(ID, min_light_flux, max_light_flux)?;
    let unsaturated = base_light_flux + proportional_gain * error + integral_gain * integral_error;
    if !unsaturated.is_finite() {
        return Err(numerical_failure(ID, "PI command was not finite"));
    }
    let value = unsaturated.clamp(min_light_flux, max_light_flux);
    let validity = if (value - unsaturated).abs() <= f64::EPSILON {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::BoundaryCase
    };
    result(value, ID, validity)
}

/// Saturated nitrate-feedback dilution command.
///
/// Implements thin-film report Eq. (31):
/// `D = sat(D0 + K_N (C_NO3,sp - C_NO3))`.
///
/// Citation key preserved: `garcia2021`.
pub fn nitrate_dilution_command(
    base_dilution_rate: f64,
    nitrate_gain: f64,
    nitrate_setpoint: f64,
    measured_nitrate: f64,
    min_dilution_rate: f64,
    max_dilution_rate: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::NITRATE_DILUTION_COMMAND;
    validation::ensure_finite("base_dilution_rate", base_dilution_rate)?;
    validation::ensure_finite("nitrate_gain", nitrate_gain)?;
    validation::ensure_finite("nitrate_setpoint", nitrate_setpoint)?;
    validation::ensure_finite("measured_nitrate", measured_nitrate)?;
    ensure_min_le_max(ID, min_dilution_rate, max_dilution_rate)?;
    let unsaturated = base_dilution_rate + nitrate_gain * (nitrate_setpoint - measured_nitrate);
    if !unsaturated.is_finite() {
        return Err(numerical_failure(
            ID,
            "nitrate dilution command was not finite",
        ));
    }
    let value = unsaturated.clamp(min_dilution_rate, max_dilution_rate);
    let validity = if (value - unsaturated).abs() <= f64::EPSILON {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::BoundaryCase
    };
    result(value, ID, validity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometric_correction_clamps_to_one() {
        assert_eq!(geometric_correction(2.0, 1.0).unwrap().value, 1.0);
        assert_eq!(geometric_correction(0.5, 1.0).unwrap().value, 0.5);
    }

    #[test]
    fn carbonate_species_sum_to_total() {
        let species = carbonate_species_from_total_and_ph(10.0, 7.0, 6.35, 10.33)
            .unwrap()
            .value;
        let total = species.co2 + species.hco3 + species.co3;
        assert!((total - 10.0).abs() < 1.0e-10);
        assert!((species.alpha_co2 + species.alpha_hco3 + species.alpha_co3 - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn pbr_rates_and_control_helpers_match_formulae() {
        let biomass = limnospira_biomass_rate(0.10, 0.02, 0.01, 4.0).unwrap();
        assert!((biomass.value - 0.28).abs() < 1.0e-12);

        let command = pi_light_command(100.0, 10.0, 5.0, 0.2, 2.0, 0.0, 120.0).unwrap();
        assert_eq!(command.value, 112.0);

        let saturated = nitrate_dilution_command(1.0, 2.0, 10.0, 0.0, 0.0, 5.0).unwrap();
        assert_eq!(saturated.value, 5.0);
        assert_eq!(saturated.validity, ValidityStatus::BoundaryCase);
    }

    #[test]
    fn cylindrical_ratio_is_finite_for_small_arguments() {
        let ratio = cylindrical_two_flux_irradiance_ratio(1.0, 0.2, 0.5, 0.1, 0.2).unwrap();
        assert!(ratio.value.is_finite());
    }
}
