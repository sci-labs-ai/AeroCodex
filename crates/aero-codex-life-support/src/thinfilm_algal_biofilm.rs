//! Strict thin-film algal biofilm and reduced-order service-map helpers.
//!
//! This module covers attached microalgal film growth, light attenuation,
//! areal productivity, mixture/PDE local residuals, boundary conditions, and the
//! reduced-order service vector used by a mission-level BLSS controller.

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

fn ensure_finite_slice(parameter: &'static str, values: &[f64]) -> AeroResult<()> {
    for value in values {
        validation::ensure_finite(parameter, *value)?;
    }
    Ok(())
}

fn ensure_same_len(codex_id: &'static str, left: &[f64], right: &[f64]) -> AeroResult<()> {
    if left.len() == right.len() {
        Ok(())
    } else {
        Err(numerical_failure(
            codex_id,
            "input slices must have the same length",
        ))
    }
}

fn ensure_rectangular(codex_id: &'static str, matrix: &[Vec<f64>], cols: usize) -> AeroResult<()> {
    if matrix.iter().all(|row| row.len() == cols) {
        Ok(())
    } else {
        Err(numerical_failure(codex_id, "matrix must be rectangular"))
    }
}

/// One layer in a one-dimensional attenuating attached biofilm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LightAttenuatingLayer {
    pub thickness_m: f64,
    pub active_biomass: f64,
    pub eps_or_matrix: f64,
    pub kappa_active: f64,
    pub kappa_eps: f64,
}

/// Local attached-algal biomass rate `(mu - k_d) X`.
///
/// Implements thin-film report Eq. (36).
///
/// Citation keys preserved: `blanken2014`, `blanken2016`.
pub fn local_biofilm_growth_rate(
    specific_growth_rate_per_s: f64,
    decay_rate_per_s: f64,
    biomass: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::ALGAL_BIOFILM_GROWTH_RATE;
    validation::ensure_nonnegative("specific_growth_rate_per_s", specific_growth_rate_per_s)?;
    validation::ensure_nonnegative("decay_rate_per_s", decay_rate_per_s)?;
    validation::ensure_nonnegative("biomass", biomass)?;
    let value = (specific_growth_rate_per_s - decay_rate_per_s) * biomass;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "growth rate was not finite"))
    }
}

/// Light at the base of a stack of layers using integrated Beer attenuation.
///
/// Implements the one-dimensional form of thin-film report Eq. (37):
/// `I = I0 exp[-int (kappa_X X + kappa_E E) dz]`.
///
/// Citation keys preserved: `blanken2014`, `polizzi2022`.
pub fn biofilm_light_from_layers(
    incident_light: f64,
    layers: &[LightAttenuatingLayer],
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::ALGAL_BIOFILM_LIGHT_PROFILE;
    validation::ensure_nonnegative("incident_light", incident_light)?;
    let mut optical_depth = 0.0;
    for layer in layers {
        validation::ensure_nonnegative("layer.thickness_m", layer.thickness_m)?;
        validation::ensure_nonnegative("layer.active_biomass", layer.active_biomass)?;
        validation::ensure_nonnegative("layer.eps_or_matrix", layer.eps_or_matrix)?;
        validation::ensure_nonnegative("layer.kappa_active", layer.kappa_active)?;
        validation::ensure_nonnegative("layer.kappa_eps", layer.kappa_eps)?;
        optical_depth += layer.thickness_m
            * (layer.kappa_active * layer.active_biomass + layer.kappa_eps * layer.eps_or_matrix);
        if !optical_depth.is_finite() {
            return Err(numerical_failure(ID, "optical depth was not finite"));
        }
    }
    let value = incident_light * (-optical_depth).exp();
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "attenuated light was not finite"))
    }
}

/// Uniform-coefficient one-dimensional light attenuation helper.
///
/// This is a convenience wrapper around Eq. (37) when biomass and EPS are
/// approximately uniform from `z = 0` to `depth_m`.
pub fn biofilm_light_uniform(
    incident_light: f64,
    depth_m: f64,
    kappa_biomass: f64,
    biomass: f64,
    kappa_eps: f64,
    eps: f64,
) -> AeroResult<EngineeringResult<f64>> {
    let layer = LightAttenuatingLayer {
        thickness_m: depth_m,
        active_biomass: biomass,
        eps_or_matrix: eps,
        kappa_active: kappa_biomass,
        kappa_eps,
    };
    biofilm_light_from_layers(incident_light, &[layer])
}

/// Areal productivity integral using trapezoidal quadrature.
///
/// Implements thin-film report Eq. (38):
/// `P_A = int_0^h [mu(I,C,N) - k_d] X(z) dz`.
/// The `net_specific_rates` array should already contain `mu - k_d` at each
/// depth node.
///
/// Citation keys preserved: `blanken2014`, `blanken2016`.
pub fn areal_productivity_trapezoid(
    depths_m: &[f64],
    net_specific_rates_per_s: &[f64],
    biomass_profile: &[f64],
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::ALGAL_AREAL_PRODUCTIVITY;
    ensure_same_len(ID, depths_m, net_specific_rates_per_s)?;
    ensure_same_len(ID, depths_m, biomass_profile)?;
    if depths_m.len() < 2 {
        return Err(numerical_failure(
            ID,
            "at least two depth nodes are required",
        ));
    }
    ensure_finite_slice("depths_m", depths_m)?;
    ensure_finite_slice("net_specific_rates_per_s", net_specific_rates_per_s)?;
    ensure_finite_slice("biomass_profile", biomass_profile)?;

    let mut integral = 0.0;
    for window in depths_m
        .windows(2)
        .zip(net_specific_rates_per_s.windows(2))
        .zip(biomass_profile.windows(2))
    {
        let ((z, rate), biomass) = window;
        let dz = z[1] - z[0];
        if dz < 0.0 {
            return Err(AeroError::OutOfDomain {
                parameter: "depths_m",
                value: dz,
                expected: "depth nodes must be monotonically nondecreasing",
            });
        }
        let left = rate[0] * biomass[0];
        let right = rate[1] * biomass[1];
        integral += 0.5 * dz * (left + right);
        if !integral.is_finite() {
            return Err(numerical_failure(
                ID,
                "areal productivity integral was not finite",
            ));
        }
    }
    result(integral, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Post-harvest reset `h(t_n+) = h_H`.
///
/// Implements thin-film report Eq. (39).
///
/// Citation key preserved: `blanken2016`.
pub fn harvest_reset_thickness(
    post_harvest_thickness_m: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::HARVEST_RESET_THICKNESS;
    validation::ensure_nonnegative("post_harvest_thickness_m", post_harvest_thickness_m)?;
    result(
        post_harvest_thickness_m,
        ID,
        ValidityStatus::WithinDocumentedDomain,
    )
}

/// Local mixture volume-fraction rate `R_k - div(phi_k v_k)`.
///
/// Implements thin-film report Eq. (40) after rearranging for
/// `partial phi_k / partial t`.
///
/// Citation key preserved: `polizzi2022`.
pub fn mixture_component_local_rate(
    advective_divergence: f64,
    reaction_source: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::MIXTURE_COMPONENT_RATE;
    validation::ensure_finite("advective_divergence", advective_divergence)?;
    validation::ensure_finite("reaction_source", reaction_source)?;
    let value = reaction_source - advective_divergence;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "mixture component rate was not finite",
        ))
    }
}

/// Residual of the volume-fraction closure `sum(phi_k) - 1`.
///
/// Implements thin-film report Eq. (41).
///
/// Citation key preserved: `polizzi2022`.
pub fn mixture_closure_residual(volume_fractions: &[f64]) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::MIXTURE_CLOSURE_RESIDUAL;
    if volume_fractions.is_empty() {
        return Err(numerical_failure(
            ID,
            "at least one volume fraction is required",
        ));
    }
    for value in volume_fractions {
        validation::ensure_nonnegative("volume_fraction", *value)?;
    }
    let sum: f64 = volume_fractions.iter().sum();
    if !sum.is_finite() {
        return Err(numerical_failure(ID, "volume-fraction sum was not finite"));
    }
    let residual = sum - 1.0;
    let validity = if residual.abs() <= 1.0e-9 {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::OutsideDocumentedDomain
    };
    result(residual, ID, validity)
}

/// Local dissolved-species rate for Eq. (42).
///
/// Rearranged form:
/// `partial psi/partial t = -div(psi v_L) + div(D grad psi) + R_psi`.
///
/// Citation key preserved: `polizzi2022`.
pub fn dissolved_species_local_rate(
    advective_divergence: f64,
    diffusive_divergence: f64,
    reaction_source: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::DISSOLVED_SPECIES_RATE;
    validation::ensure_finite("advective_divergence", advective_divergence)?;
    validation::ensure_finite("diffusive_divergence", diffusive_divergence)?;
    validation::ensure_finite("reaction_source", reaction_source)?;
    let value = -advective_divergence + diffusive_divergence + reaction_source;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "dissolved species local rate was not finite",
        ))
    }
}

/// Two-dimensional light closure from a supplied path integral.
///
/// Implements thin-film report Eq. (43) as
/// `I(x,z,t) = I0(x,t) exp(-attenuation_path_integral)`. The caller computes
/// the line integral of `kappa_A phi_A + kappa_N phi_N + kappa_E phi_E` along
/// the local depth path.
///
/// Citation key preserved: `polizzi2022`.
pub fn two_dimensional_light_attenuation(
    incident_light: f64,
    attenuation_path_integral: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::THINFILM_2D_LIGHT;
    validation::ensure_nonnegative("incident_light", incident_light)?;
    validation::ensure_nonnegative("attenuation_path_integral", attenuation_path_integral)?;
    let value = incident_light * (-attenuation_path_integral).exp();
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "2D attenuated light was not finite"))
    }
}

/// Residual of the gas/film boundary condition `-D grad C dot n = k(C - C*)`.
///
/// Implements thin-film report Eq. (44)-(45). A zero residual means the supplied
/// gradient and mass-transfer law are mutually consistent.
///
/// Citation key preserved: `polizzi2022`.
pub fn boundary_mass_transfer_residual(
    diffusivity: f64,
    normal_gradient: f64,
    mass_transfer_coefficient: f64,
    concentration: f64,
    saturation_or_external_concentration: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::CARBON_BOUNDARY_FLUX;
    validation::ensure_nonnegative("diffusivity", diffusivity)?;
    validation::ensure_finite("normal_gradient", normal_gradient)?;
    validation::ensure_nonnegative("mass_transfer_coefficient", mass_transfer_coefficient)?;
    validation::ensure_nonnegative("concentration", concentration)?;
    validation::ensure_nonnegative(
        "saturation_or_external_concentration",
        saturation_or_external_concentration,
    )?;
    let diffusive_side = -diffusivity * normal_gradient;
    let transfer_side =
        mass_transfer_coefficient * (concentration - saturation_or_external_concentration);
    let residual = diffusive_side - transfer_side;
    if residual.is_finite() {
        result(residual, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "boundary mass-transfer residual was not finite",
        ))
    }
}

/// Residual of the wetted nitrate Dirichlet boundary `S - S_medium`.
///
/// Implements thin-film report Eq. (46).
///
/// Citation key preserved: `polizzi2022`.
pub fn nitrate_boundary_residual(
    nitrate_concentration: f64,
    medium_nitrate_concentration: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::NITRATE_BOUNDARY_RESIDUAL;
    validation::ensure_nonnegative("nitrate_concentration", nitrate_concentration)?;
    validation::ensure_nonnegative("medium_nitrate_concentration", medium_nitrate_concentration)?;
    let residual = nitrate_concentration - medium_nitrate_concentration;
    let validity = if residual.abs() <= 1.0e-12 {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::BoundaryCase
    };
    result(residual, ID, validity)
}

/// Service vector required by a mission-level thin-film BLSS controller.
///
/// The signs follow thin-film report Eq. (49): oxygen production is positive;
/// carbon dioxide and nitrate uptake are represented as positive service rates;
/// harvest is a positive dry-mass service; risk is a dimensionless state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThinFilmServiceVector {
    pub oxygen_production_mol_per_s: f64,
    pub carbon_dioxide_uptake_mol_per_s: f64,
    pub nitrate_uptake_mol_per_s: f64,
    pub biomass_harvest_kg_per_s: f64,
    pub risk: f64,
}

impl ThinFilmServiceVector {
    /// Returns the service vector as `[O2, CO2 uptake, NO3 uptake, harvest, risk]`.
    #[must_use]
    pub const fn as_array(self) -> [f64; 5] {
        [
            self.oxygen_production_mol_per_s,
            self.carbon_dioxide_uptake_mol_per_s,
            self.nitrate_uptake_mol_per_s,
            self.biomass_harvest_kg_per_s,
            self.risk,
        ]
    }
}

/// Constructs the service vector from thin-film report Eq. (49).
///
/// Citation key preserved: `polizzi2022`.
pub fn thinfilm_service_vector(
    oxygen_production_mol_per_s: f64,
    carbon_dioxide_uptake_mol_per_s: f64,
    nitrate_uptake_mol_per_s: f64,
    biomass_harvest_kg_per_s: f64,
    risk: f64,
) -> AeroResult<EngineeringResult<ThinFilmServiceVector>> {
    const ID: &str = ids::SERVICE_VECTOR;
    validation::ensure_nonnegative("oxygen_production_mol_per_s", oxygen_production_mol_per_s)?;
    validation::ensure_nonnegative(
        "carbon_dioxide_uptake_mol_per_s",
        carbon_dioxide_uptake_mol_per_s,
    )?;
    validation::ensure_nonnegative("nitrate_uptake_mol_per_s", nitrate_uptake_mol_per_s)?;
    validation::ensure_nonnegative("biomass_harvest_kg_per_s", biomass_harvest_kg_per_s)?;
    validation::ensure_nonnegative("risk", risk)?;
    let vector = ThinFilmServiceVector {
        oxygen_production_mol_per_s,
        carbon_dioxide_uptake_mol_per_s,
        nitrate_uptake_mol_per_s,
        biomass_harvest_kg_per_s,
        risk,
    };
    result(vector, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Reduced-order service projection `y = W Phi + epsilon`.
///
/// Implements thin-film report Eq. (47) and Eq. (50). Rows of `weights` are
/// service outputs; columns multiply `features`.
///
/// Citation keys preserved: `polizzi2022`, `poughon2021`, `garcia2021`.
pub fn linear_rom_service(
    weights: &[Vec<f64>],
    features: &[f64],
    model_error: &[f64],
) -> AeroResult<EngineeringResult<Vec<f64>>> {
    const ID: &str = ids::LINEAR_ROM_SERVICE;
    ensure_rectangular(ID, weights, features.len())?;
    if weights.len() != model_error.len() {
        return Err(numerical_failure(
            ID,
            "model_error length must equal number of weight rows",
        ));
    }
    ensure_finite_slice("features", features)?;
    ensure_finite_slice("model_error", model_error)?;
    for row in weights {
        ensure_finite_slice("weights", row)?;
    }
    let mut output = Vec::with_capacity(weights.len());
    for (row, error) in weights.iter().zip(model_error) {
        let projection = row.iter().zip(features).try_fold(0.0, |acc, (w, phi)| {
            let next = acc + w * phi;
            if next.is_finite() {
                Ok(next)
            } else {
                Err(numerical_failure(ID, "ROM projection was not finite"))
            }
        })?;
        let value = projection + *error;
        if !value.is_finite() {
            return Err(numerical_failure(ID, "ROM output was not finite"));
        }
        output.push(value);
    }
    result(output, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Habitat derivative `dx/dt = f_crew + B R_ROM - losses`.
///
/// Implements thin-film report Eq. (48). `service_coupling` maps service-vector
/// outputs into habitat state rates.
///
/// Citation keys preserved: `garcia2021`, `vermeulen2023`.
pub fn habitat_coupled_derivative(
    crew_forcing: &[f64],
    service_coupling: &[Vec<f64>],
    service_vector: &[f64],
    losses: &[f64],
) -> AeroResult<EngineeringResult<Vec<f64>>> {
    const ID: &str = ids::HABITAT_COUPLED_DERIVATIVE;
    if service_coupling.len() != crew_forcing.len() || crew_forcing.len() != losses.len() {
        return Err(numerical_failure(
            ID,
            "state vectors and service-coupling rows must have matching lengths",
        ));
    }
    ensure_rectangular(ID, service_coupling, service_vector.len())?;
    ensure_finite_slice("crew_forcing", crew_forcing)?;
    ensure_finite_slice("service_vector", service_vector)?;
    ensure_finite_slice("losses", losses)?;
    for row in service_coupling {
        ensure_finite_slice("service_coupling", row)?;
    }
    let mut derivative = Vec::with_capacity(crew_forcing.len());
    for ((crew, row), loss) in crew_forcing.iter().zip(service_coupling).zip(losses) {
        let service_term = row
            .iter()
            .zip(service_vector)
            .try_fold(0.0, |acc, (b, value)| {
                let next = acc + b * value;
                if next.is_finite() {
                    Ok(next)
                } else {
                    Err(numerical_failure(ID, "service coupling was not finite"))
                }
            })?;
        derivative.push(*crew + service_term - *loss);
    }
    result(derivative, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Scalar range used by a validated-domain check.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeInclusiveF64 {
    pub min: f64,
    pub max: f64,
}

impl RangeInclusiveF64 {
    #[must_use]
    pub fn contains(self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

/// Validated operating envelope for a reduced-order thin-film service map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValidatedThinFilmDomain {
    pub light_flux: RangeInclusiveF64,
    pub gas_flow: RangeInclusiveF64,
    pub liquid_flow: RangeInclusiveF64,
    pub film_thickness: RangeInclusiveF64,
    pub harvest_interval: RangeInclusiveF64,
    pub ph: RangeInclusiveF64,
    pub temperature_k: RangeInclusiveF64,
}

/// Operating point for [`ValidatedThinFilmDomain`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThinFilmOperatingPoint {
    pub light_flux: f64,
    pub gas_flow: f64,
    pub liquid_flow: f64,
    pub film_thickness: f64,
    pub harvest_interval: f64,
    pub ph: f64,
    pub temperature_k: f64,
}

/// Checks whether `(q0, Fg, Q, h, t_harvest, pH, T)` is inside the validated domain.
///
/// Implements thin-film report Eq. (51).
///
/// Citation keys preserved: `poughon2021`, `polizzi2022`, `garcia2021`.
pub fn validated_domain_contains(
    domain: ValidatedThinFilmDomain,
    point: ThinFilmOperatingPoint,
) -> AeroResult<EngineeringResult<bool>> {
    const ID: &str = ids::VALIDATED_DOMAIN_CHECK;
    let ranges = [
        domain.light_flux,
        domain.gas_flow,
        domain.liquid_flow,
        domain.film_thickness,
        domain.harvest_interval,
        domain.ph,
        domain.temperature_k,
    ];
    for range in &ranges {
        validation::ensure_finite("domain.min", range.min)?;
        validation::ensure_finite("domain.max", range.max)?;
        if range.min > range.max {
            return Err(numerical_failure(ID, "domain range min must be <= max"));
        }
    }
    let values = [
        point.light_flux,
        point.gas_flow,
        point.liquid_flow,
        point.film_thickness,
        point.harvest_interval,
        point.ph,
        point.temperature_k,
    ];
    ensure_finite_slice("operating_point", &values)?;
    let inside = ranges
        .iter()
        .zip(values)
        .all(|(range, value)| range.contains(value));
    let validity = if inside {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::OutsideDocumentedDomain
    };
    result(inside, ID, validity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_growth_and_light_match_formulae() {
        assert!((local_biofilm_growth_rate(0.2, 0.05, 10.0).unwrap().value - 1.5).abs() < 1.0e-12);
        let light = biofilm_light_uniform(100.0, 1.0, 0.1, 2.0, 0.2, 3.0).unwrap();
        assert!((light.value - 100.0_f64 * (-0.8_f64).exp()).abs() < 1.0e-12);
    }

    #[test]
    fn areal_productivity_uses_trapezoid_rule() {
        let p = areal_productivity_trapezoid(&[0.0, 1.0], &[2.0, 4.0], &[3.0, 5.0]).unwrap();
        assert_eq!(p.value, 13.0);
    }

    #[test]
    fn mixture_and_boundary_helpers_behave() {
        assert!(
            mixture_closure_residual(&[0.2, 0.3, 0.5])
                .unwrap()
                .value
                .abs()
                < 1.0e-12
        );
        assert_eq!(mixture_component_local_rate(2.0, 5.0).unwrap().value, 3.0);
        assert_eq!(
            dissolved_species_local_rate(2.0, 5.0, 7.0).unwrap().value,
            10.0
        );
        assert_eq!(nitrate_boundary_residual(4.0, 4.0).unwrap().value, 0.0);
    }

    #[test]
    fn rom_and_domain_helpers_behave() {
        let y = linear_rom_service(
            &[vec![1.0, 2.0], vec![0.0, 3.0]],
            &[10.0, 2.0],
            &[1.0, -1.0],
        )
        .unwrap();
        assert_eq!(y.value, vec![15.0, 5.0]);

        let derivative = habitat_coupled_derivative(
            &[1.0, 2.0],
            &[vec![1.0, 0.0], vec![0.0, 1.0]],
            &[3.0, 4.0],
            &[0.5, 1.5],
        )
        .unwrap();
        assert_eq!(derivative.value, vec![3.5, 4.5]);

        let domain = ValidatedThinFilmDomain {
            light_flux: RangeInclusiveF64 {
                min: 0.0,
                max: 100.0,
            },
            gas_flow: RangeInclusiveF64 {
                min: 0.0,
                max: 10.0,
            },
            liquid_flow: RangeInclusiveF64 {
                min: 0.0,
                max: 10.0,
            },
            film_thickness: RangeInclusiveF64 {
                min: 0.0,
                max: 0.01,
            },
            harvest_interval: RangeInclusiveF64 {
                min: 1.0,
                max: 10.0,
            },
            ph: RangeInclusiveF64 { min: 6.0, max: 9.0 },
            temperature_k: RangeInclusiveF64 {
                min: 290.0,
                max: 310.0,
            },
        };
        let point = ThinFilmOperatingPoint {
            light_flux: 50.0,
            gas_flow: 1.0,
            liquid_flow: 1.0,
            film_thickness: 0.005,
            harvest_interval: 7.0,
            ph: 7.5,
            temperature_k: 298.0,
        };
        assert!(validated_domain_contains(domain, point).unwrap().value);
    }
}
