//! MELiSSA C3 nitrifying fixed-bed biofilm helpers.
//!
//! This module implements the thin-film report's fixed-bed/biofilm equations:
//! nitrification stoichiometry, Fickian diffusion, biofilm thickness dynamics,
//! gas-liquid transfer, calibrated nitrifier-rate forms, and a bulk tank coupled
//! to a local biofilm surface flux.

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

fn ensure_positive_params(
    _codex_id: &'static str,
    values: &[(&'static str, f64)],
) -> AeroResult<()> {
    for (name, value) in values {
        validation::ensure_positive(name, *value)?;
    }
    Ok(())
}

/// Stoichiometric flux vector for the two nitrification steps.
///
/// All fields are signed rates in the caller's amount/time basis. Positive
/// values are production; negative values are consumption.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NitrificationFluxes {
    pub nh4: f64,
    pub no2: f64,
    pub no3: f64,
    pub o2: f64,
    pub h_plus: f64,
    pub h2o: f64,
}

/// Computes the signed stoichiometric fluxes from ammonium and nitrite oxidation extents.
///
/// Implements thin-film report Eq. (17)-(18):
/// `NH4+ + 1.5 O2 -> NO2- + 2H+ + H2O` and
/// `NO2- + 0.5 O2 -> NO3-`.
///
/// Citation key preserved: `garcia2021`.
pub fn nitrification_fluxes(
    ammonium_oxidation_rate: f64,
    nitrite_oxidation_rate: f64,
) -> AeroResult<EngineeringResult<NitrificationFluxes>> {
    const ID: &str = ids::NITRIFICATION_AOB_STOICHIOMETRY;
    validation::ensure_nonnegative("ammonium_oxidation_rate", ammonium_oxidation_rate)?;
    validation::ensure_nonnegative("nitrite_oxidation_rate", nitrite_oxidation_rate)?;
    let fluxes = NitrificationFluxes {
        nh4: -ammonium_oxidation_rate,
        no2: ammonium_oxidation_rate - nitrite_oxidation_rate,
        no3: nitrite_oxidation_rate,
        o2: -1.5 * ammonium_oxidation_rate - 0.5 * nitrite_oxidation_rate,
        h_plus: 2.0 * ammonium_oxidation_rate,
        h2o: ammonium_oxidation_rate,
    };
    result(fluxes, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Constant-diffusivity spherical diffusion plus reaction rate.
///
/// Implements thin-film report Eq. (19) in expanded form for `r > 0`:
/// `D(d2C/dr2 + 2/r dC/dr) + reaction_sum`. Near the bead/film center,
/// callers should use a symmetry-compatible discretization or pass a finite
/// center approximation.
///
/// Citation keys preserved: `montras2009`, `perez2005`.
pub fn spherical_diffusion_reaction_rate(
    radius_m: f64,
    diffusivity_m2_per_s: f64,
    concentration_gradient_per_m: f64,
    concentration_second_derivative_per_m2: f64,
    reaction_sum: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::SPHERICAL_DIFFUSION_RATE;
    validation::ensure_positive("radius_m", radius_m)?;
    validation::ensure_nonnegative("diffusivity_m2_per_s", diffusivity_m2_per_s)?;
    validation::ensure_finite("concentration_gradient_per_m", concentration_gradient_per_m)?;
    validation::ensure_finite(
        "concentration_second_derivative_per_m2",
        concentration_second_derivative_per_m2,
    )?;
    validation::ensure_finite("reaction_sum", reaction_sum)?;
    let value = diffusivity_m2_per_s
        * (concentration_second_derivative_per_m2 + 2.0 * concentration_gradient_per_m / radius_m)
        + reaction_sum;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "spherical diffusion rate was not finite",
        ))
    }
}

/// Uniform-grid method-of-lines helper for Eq. (19).
///
/// The first node uses a spherical symmetry center approximation and the last
/// node uses a zero-gradient outer boundary unless the surface exchange is
/// handled separately by Eq. (26). `reactions[i]` must already be the local
/// stoichiometric reaction sum for species `i`.
pub fn spherical_diffusion_rhs_uniform(
    concentrations: &[f64],
    radial_step_m: f64,
    diffusivity_m2_per_s: f64,
    reactions: &[f64],
) -> AeroResult<EngineeringResult<Vec<f64>>> {
    const ID: &str = ids::SPHERICAL_DIFFUSION_RATE;
    if concentrations.len() != reactions.len() || concentrations.is_empty() {
        return Err(numerical_failure(
            ID,
            "concentrations and reactions must be nonempty arrays of equal length",
        ));
    }
    validation::ensure_positive("radial_step_m", radial_step_m)?;
    validation::ensure_nonnegative("diffusivity_m2_per_s", diffusivity_m2_per_s)?;
    for value in concentrations.iter().chain(reactions) {
        validation::ensure_finite("concentration_or_reaction", *value)?;
    }
    let dr2 = radial_step_m * radial_step_m;
    let mut rhs = Vec::with_capacity(concentrations.len());
    for (index, concentration) in concentrations.iter().enumerate() {
        let reaction = reactions[index];
        let diffusion = if concentrations.len() == 1 {
            0.0
        } else if index == 0 {
            6.0 * diffusivity_m2_per_s * (concentrations[1] - concentration) / dr2
        } else if index + 1 == concentrations.len() {
            let prev = concentrations[index - 1];
            2.0 * diffusivity_m2_per_s * (prev - concentration) / dr2
        } else {
            let r = index as f64 * radial_step_m;
            let grad =
                (concentrations[index + 1] - concentrations[index - 1]) / (2.0 * radial_step_m);
            let second =
                (concentrations[index + 1] - 2.0 * concentration + concentrations[index - 1]) / dr2;
            diffusivity_m2_per_s * (second + 2.0 * grad / r)
        };
        let value = diffusion + reaction;
        if !value.is_finite() {
            return Err(numerical_failure(ID, "finite-volume RHS was not finite"));
        }
        rhs.push(value);
    }
    result(rhs, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Liquid diffusive flux `J_L = -D dC/dr`.
///
/// Implements thin-film report Eq. (20).
///
/// Citation key preserved: `montras2009`.
pub fn fick_liquid_flux(
    diffusivity_m2_per_s: f64,
    concentration_gradient_per_m: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::FICK_BIOFILM_FLUX;
    validation::ensure_nonnegative("diffusivity_m2_per_s", diffusivity_m2_per_s)?;
    validation::ensure_finite("concentration_gradient_per_m", concentration_gradient_per_m)?;
    result(
        -diffusivity_m2_per_s * concentration_gradient_per_m,
        ID,
        ValidityStatus::WithinDocumentedDomain,
    )
}

/// Local solids time derivative rearranged from Eq. (21).
///
/// Given the spherical divergence term `1/r^2 d(r^2 u_F X_k)/dr`, returns
/// `R_X,k - divergence`.
///
/// Citation key preserved: `montras2009`.
pub fn biofilm_solids_local_rate(
    spherical_solids_flux_divergence: f64,
    reaction_or_growth_source: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::BIOFILM_SOLIDS_RATE;
    validation::ensure_finite(
        "spherical_solids_flux_divergence",
        spherical_solids_flux_divergence,
    )?;
    validation::ensure_finite("reaction_or_growth_source", reaction_or_growth_source)?;
    let value = reaction_or_growth_source - spherical_solids_flux_divergence;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "solids local rate was not finite"))
    }
}

/// Biofilm thickness rate `dL_F/dt = u_F - u_det`.
///
/// Implements thin-film report Eq. (22).
///
/// Citation key preserved: `montras2009`.
pub fn biofilm_thickness_rate(
    growth_velocity_m_per_s: f64,
    detachment_velocity_m_per_s: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::BIOFILM_THICKNESS_RATE;
    validation::ensure_nonnegative("growth_velocity_m_per_s", growth_velocity_m_per_s)?;
    validation::ensure_nonnegative("detachment_velocity_m_per_s", detachment_velocity_m_per_s)?;
    result(
        growth_velocity_m_per_s - detachment_velocity_m_per_s,
        ID,
        ValidityStatus::WithinDocumentedDomain,
    )
}

/// Gas-liquid transfer flux `J_G = kLa(C* - C)`.
///
/// Implements thin-film report Eq. (23), and is also compatible with the BLSS
/// catalog gas-liquid mass-transfer sign convention.
///
/// Citation key preserved: `montras2009`.
pub fn gas_liquid_transfer_flux(
    kla_per_s: f64,
    saturation_concentration: f64,
    concentration: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::C3_OXYGEN_TRANSFER;
    validation::ensure_nonnegative("kla_per_s", kla_per_s)?;
    validation::ensure_nonnegative("saturation_concentration", saturation_concentration)?;
    validation::ensure_nonnegative("concentration", concentration)?;
    result(
        kla_per_s * (saturation_concentration - concentration),
        ID,
        ValidityStatus::WithinDocumentedDomain,
    )
}

/// Kinetic parameters for thin-film report Eq. (24)-(25).
///
/// The defaults preserve the representative 28 C, pH 8 values stated in the
/// report from Montras i Boet (2009). Concentration units must match the caller's
/// model basis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NitrifierKineticParameters {
    pub k_nh4: f64,
    pub k_no2: f64,
    pub k_o2_nts: f64,
    pub k_o2_ntb: f64,
    pub ki_fa_nts: f64,
    pub ki_fna_ntb: f64,
    pub mu_nts_per_d: f64,
    pub mu_ntb_per_d: f64,
    pub i_fa_ntb: f64,
}

impl Default for NitrifierKineticParameters {
    fn default() -> Self {
        Self {
            k_nh4: 0.0175,
            k_no2: 5.04e-3,
            k_o2_nts: 1.616e-4,
            k_o2_ntb: 5.44e-4,
            ki_fa_nts: 0.116,
            ki_fna_ntb: 1.56e-4,
            mu_nts_per_d: 1.368,
            mu_ntb_per_d: 0.864,
            i_fa_ntb: 1.0,
        }
    }
}

/// Pair of nitrifier rates from Eq. (24)-(25).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NitrifierRates {
    pub nitrosomonas_rate: f64,
    pub nitrobacter_rate: f64,
}

/// Computes the Nitrosomonas and Nitrobacter rates from thin-film report Eq. (24)-(25).
///
/// Citation keys preserved: `montras2009`, `perez2005`.
pub fn nitrifier_rates(
    c_nh4: f64,
    c_no2: f64,
    c_o2: f64,
    x_nts: f64,
    x_ntb: f64,
    params: NitrifierKineticParameters,
) -> AeroResult<EngineeringResult<NitrifierRates>> {
    const ID: &str = ids::NITRIFIER_RATES;
    validation::ensure_nonnegative("c_nh4", c_nh4)?;
    validation::ensure_nonnegative("c_no2", c_no2)?;
    validation::ensure_nonnegative("c_o2", c_o2)?;
    validation::ensure_nonnegative("x_nts", x_nts)?;
    validation::ensure_nonnegative("x_ntb", x_ntb)?;
    ensure_positive_params(
        ID,
        &[
            ("k_nh4", params.k_nh4),
            ("k_no2", params.k_no2),
            ("k_o2_nts", params.k_o2_nts),
            ("k_o2_ntb", params.k_o2_ntb),
            ("ki_fa_nts", params.ki_fa_nts),
            ("ki_fna_ntb", params.ki_fna_ntb),
            ("mu_nts_per_d", params.mu_nts_per_d),
            ("mu_ntb_per_d", params.mu_ntb_per_d),
            ("i_fa_ntb", params.i_fa_ntb),
        ],
    )?;

    let nh4_term = c_nh4 / (params.k_nh4 + c_nh4 + c_nh4 * c_nh4 / params.ki_fa_nts);
    let no2_term = c_no2 / (params.k_no2 + c_no2 + c_no2 * c_no2 / params.ki_fna_ntb);
    let o2_nts_term = c_o2 / (params.k_o2_nts + c_o2);
    let o2_ntb_term = c_o2 / (params.k_o2_ntb + c_o2);
    let rates = NitrifierRates {
        nitrosomonas_rate: params.mu_nts_per_d * x_nts * nh4_term * o2_nts_term,
        nitrobacter_rate: params.mu_ntb_per_d * x_ntb * no2_term * o2_ntb_term * params.i_fa_ntb,
    };
    if rates.nitrosomonas_rate.is_finite() && rates.nitrobacter_rate.is_finite() {
        result(rates, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "nitrifier rate was not finite"))
    }
}

/// Bulk-liquid tank rate coupled to a biofilm surface flux.
///
/// Implements thin-film report Eq. (26):
/// `Q/V(C_i,n-1 - C_i,n) - a_n J_L|surface + kLa(C* - C_bulk)`.
/// `surface_liquid_flux` must use the sign convention from Eq. (20).
///
/// Citation key preserved: `montras2009`.
pub fn bulk_tank_biofilm_coupled_rate(
    flow_over_volume_per_s: f64,
    upstream_bulk_concentration: f64,
    bulk_concentration: f64,
    support_area_per_volume: f64,
    surface_liquid_flux: f64,
    kla_per_s: f64,
    saturation_concentration: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::C3_TANK_BIOFILM_RATE;
    validation::ensure_nonnegative("flow_over_volume_per_s", flow_over_volume_per_s)?;
    validation::ensure_nonnegative("upstream_bulk_concentration", upstream_bulk_concentration)?;
    validation::ensure_nonnegative("bulk_concentration", bulk_concentration)?;
    validation::ensure_nonnegative("support_area_per_volume", support_area_per_volume)?;
    validation::ensure_finite("surface_liquid_flux", surface_liquid_flux)?;
    validation::ensure_nonnegative("kla_per_s", kla_per_s)?;
    validation::ensure_nonnegative("saturation_concentration", saturation_concentration)?;
    let value = flow_over_volume_per_s * (upstream_bulk_concentration - bulk_concentration)
        - support_area_per_volume * surface_liquid_flux
        + kla_per_s * (saturation_concentration - bulk_concentration);
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "bulk tank biofilm rate was not finite",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nitrification_fluxes_preserve_stoichiometry() {
        let flux = nitrification_fluxes(1.0, 1.0).unwrap().value;
        assert_eq!(flux.nh4, -1.0);
        assert_eq!(flux.no2, 0.0);
        assert_eq!(flux.no3, 1.0);
        assert_eq!(flux.o2, -2.0);
        assert_eq!(flux.h_plus, 2.0);
    }

    #[test]
    fn nitrifier_rates_match_pseudocode_structure() {
        let params = NitrifierKineticParameters::default();
        let rates = nitrifier_rates(0.1, 0.05, 0.002, 1.0, 1.0, params)
            .unwrap()
            .value;
        assert!(rates.nitrosomonas_rate > 0.0);
        assert!(rates.nitrobacter_rate > 0.0);
    }

    #[test]
    fn flux_thickness_and_bulk_helpers_match_formulae() {
        assert_eq!(fick_liquid_flux(2.0, 3.0).unwrap().value, -6.0);
        assert_eq!(biofilm_thickness_rate(5.0, 2.0).unwrap().value, 3.0);
        assert_eq!(
            gas_liquid_transfer_flux(2.0, 10.0, 4.0).unwrap().value,
            12.0
        );
        let bulk = bulk_tank_biofilm_coupled_rate(1.0, 10.0, 4.0, 2.0, 0.5, 3.0, 6.0).unwrap();
        assert_eq!(bulk.value, 11.0);
    }

    #[test]
    fn uniform_diffusion_rhs_has_expected_size() {
        let rhs = spherical_diffusion_rhs_uniform(&[1.0, 0.8, 0.7], 0.1, 1.0e-9, &[0.0, 0.0, 0.0])
            .unwrap();
        assert_eq!(rhs.value.len(), 3);
    }
}
