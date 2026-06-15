//! Common mass-balance, stoichiometric, closure, and ESM helpers for BLSS modules.
//!
//! These routines implement the common mathematical backbone in the supplied
//! thin-film report. They are intentionally small, deterministic, and
//! dependency-free so they can be used as AeroCodex kernel primitives.

use aero_codex_core::{validation, AeroError, AeroResult, EngineeringResult, ValidityStatus};

use crate::thinfilm_provenance::{self as provenance, ids};

fn numerical_failure(codex_id: &'static str, reason: &'static str) -> AeroError {
    AeroError::NumericalFailure {
        solver: codex_id,
        reason,
    }
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
            "input vectors must have the same length",
        ))
    }
}

fn ensure_matrix_shape(
    codex_id: &'static str,
    matrix: &[Vec<f64>],
    expected_cols: usize,
) -> AeroResult<()> {
    if matrix.iter().all(|row| row.len() == expected_cols) {
        Ok(())
    } else {
        Err(numerical_failure(
            codex_id,
            "matrix rows must all have the expected number of columns",
        ))
    }
}

fn dot_checked(codex_id: &'static str, left: &[f64], right: &[f64]) -> AeroResult<f64> {
    ensure_same_len(codex_id, left, right)?;
    let value = left.iter().zip(right).try_fold(0.0, |acc, (a, b)| {
        let next = acc + a * b;
        if next.is_finite() {
            Ok(next)
        } else {
            Err(numerical_failure(
                codex_id,
                "dot product produced a nonfinite result",
            ))
        }
    })?;
    Ok(value)
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

/// Scalar compartment derivative for the common BLSS backbone.
///
/// Implements the scalar form of thin-film report Eq. (2):
/// `dx/dt = F_in - F_out + V * N^T r + T_g + d`.
///
/// Citation keys preserved: `poughon2021`, `garcia2021`, `montras2009`.
pub fn generic_compartment_derivative(
    inflow_terms: &[f64],
    outflow_terms: &[f64],
    volume: f64,
    stoichiometric_coefficients: &[f64],
    reaction_rates: &[f64],
    gas_liquid_exchange: f64,
    disturbance: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::GENERIC_COMPARTMENT_DERIVATIVE;
    ensure_finite_slice("inflow_terms", inflow_terms)?;
    ensure_finite_slice("outflow_terms", outflow_terms)?;
    validation::ensure_nonnegative("volume", volume)?;
    ensure_finite_slice("stoichiometric_coefficients", stoichiometric_coefficients)?;
    ensure_finite_slice("reaction_rates", reaction_rates)?;
    validation::ensure_finite("gas_liquid_exchange", gas_liquid_exchange)?;
    validation::ensure_finite("disturbance", disturbance)?;

    let inflow: f64 = inflow_terms.iter().sum();
    let outflow: f64 = outflow_terms.iter().sum();
    let reaction = volume * dot_checked(ID, stoichiometric_coefficients, reaction_rates)?;
    let value = inflow - outflow + reaction + gas_liquid_exchange + disturbance;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(
            ID,
            "compartment derivative was not finite",
        ))
    }
}

/// Vector form of `n_dot = S r + B u + d - loss`.
///
/// This is the dynamic stoichiometric form from the general BLSS catalog and is
/// compatible with the thin-film report's compartment derivative Eq. (2).
pub fn stoichiometric_state_derivative(
    stoichiometric: &[Vec<f64>],
    rates: &[f64],
    controlled_inputs: &[f64],
    disturbances: &[f64],
    losses: &[f64],
) -> AeroResult<EngineeringResult<Vec<f64>>> {
    const ID: &str = ids::GENERIC_COMPARTMENT_DERIVATIVE;
    ensure_matrix_shape(ID, stoichiometric, rates.len())?;
    ensure_same_len(ID, controlled_inputs, disturbances)?;
    ensure_same_len(ID, controlled_inputs, losses)?;
    if stoichiometric.len() != controlled_inputs.len() {
        return Err(numerical_failure(
            ID,
            "stoichiometric rows must match input/disturbance/loss length",
        ));
    }
    ensure_finite_slice("rates", rates)?;
    ensure_finite_slice("controlled_inputs", controlled_inputs)?;
    ensure_finite_slice("disturbances", disturbances)?;
    ensure_finite_slice("losses", losses)?;

    let values = stoichiometric
        .iter()
        .zip(controlled_inputs)
        .zip(disturbances)
        .zip(losses)
        .map(|(((row, input), disturbance), loss)| {
            dot_checked(ID, row, rates).map(|sr| sr + *input + *disturbance - *loss)
        })
        .collect::<AeroResult<Vec<_>>>()?;
    result(values, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Computes the residual of `A xi = 0` for elemental closure.
///
/// Implements thin-film report Eq. (3). A zero vector means the provided
/// throughputs close the tracked elements under the supplied matrix.
///
/// Citation key preserved: `vermeulen2023`.
pub fn element_balance_residual(
    element_by_process: &[Vec<f64>],
    throughputs: &[f64],
) -> AeroResult<EngineeringResult<Vec<f64>>> {
    const ID: &str = ids::ELEMENT_BALANCE_RESIDUAL;
    ensure_matrix_shape(ID, element_by_process, throughputs.len())?;
    ensure_finite_slice("throughputs", throughputs)?;

    let residual = element_by_process
        .iter()
        .map(|row| dot_checked(ID, row, throughputs))
        .collect::<AeroResult<Vec<_>>>()?;
    result(residual, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Computes the residual of `B xi + s_crew + s_stores = 0`.
///
/// Implements thin-film report Eq. (4). A zero vector means commodity services
/// balance crew and storage terms under the supplied sign convention.
///
/// Citation key preserved: `vermeulen2023`.
pub fn loop_balance_residual(
    commodity_by_process: &[Vec<f64>],
    throughputs: &[f64],
    crew_terms: &[f64],
    store_terms: &[f64],
) -> AeroResult<EngineeringResult<Vec<f64>>> {
    const ID: &str = ids::LOOP_BALANCE_RESIDUAL;
    ensure_matrix_shape(ID, commodity_by_process, throughputs.len())?;
    if commodity_by_process.len() != crew_terms.len() || crew_terms.len() != store_terms.len() {
        return Err(numerical_failure(
            ID,
            "commodity rows, crew terms, and store terms must have the same length",
        ));
    }
    ensure_finite_slice("throughputs", throughputs)?;
    ensure_finite_slice("crew_terms", crew_terms)?;
    ensure_finite_slice("store_terms", store_terms)?;

    let residual = commodity_by_process
        .iter()
        .zip(crew_terms)
        .zip(store_terms)
        .map(|((row, crew), store)| {
            dot_checked(ID, row, throughputs).map(|bxi| bxi + *crew + *store)
        })
        .collect::<AeroResult<Vec<_>>>()?;
    result(residual, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Checks `x_min <= x <= x_max` or `u_min <= u <= u_max` componentwise.
///
/// Implements thin-film report Eq. (5).
pub fn within_componentwise_bounds(
    values: &[f64],
    lower_bounds: &[f64],
    upper_bounds: &[f64],
) -> AeroResult<EngineeringResult<bool>> {
    const ID: &str = ids::DYNAMIC_BOUNDS_CHECK;
    ensure_same_len(ID, values, lower_bounds)?;
    ensure_same_len(ID, values, upper_bounds)?;
    ensure_finite_slice("values", values)?;
    ensure_finite_slice("lower_bounds", lower_bounds)?;
    ensure_finite_slice("upper_bounds", upper_bounds)?;

    if lower_bounds
        .iter()
        .zip(upper_bounds)
        .any(|(lo, hi)| lo > hi)
    {
        return Err(numerical_failure(
            ID,
            "every lower bound must be less than or equal to its upper bound",
        ));
    }

    let ok = values
        .iter()
        .zip(lower_bounds)
        .zip(upper_bounds)
        .all(|((value, lo), hi)| value >= lo && value <= hi);
    let validity = if ok {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::OutsideDocumentedDomain
    };
    result(ok, ID, validity)
}

/// Computes open-system elemental inventory rates `E (B u + d - loss)`.
///
/// This is the open-system counterpart to thin-film report Eq. (32) and the
/// compiled BLSS catalog's elemental conservation equations.
///
/// Citation key preserved: `vermeulen2023`.
pub fn element_inventory_rate(
    element_by_species: &[Vec<f64>],
    species_open_rates: &[f64],
) -> AeroResult<EngineeringResult<Vec<f64>>> {
    const ID: &str = ids::ELEMENT_INVENTORY_BALANCE;
    ensure_matrix_shape(ID, element_by_species, species_open_rates.len())?;
    ensure_finite_slice("species_open_rates", species_open_rates)?;

    let value = element_by_species
        .iter()
        .map(|row| dot_checked(ID, row, species_open_rates))
        .collect::<AeroResult<Vec<_>>>()?;
    result(value, ID, ValidityStatus::WithinDocumentedDomain)
}

/// Commodity closure metric `eta = 1 - (resupply + loss) / demand`.
///
/// Implements thin-film report Eq. (33). Values can be negative when resupply
/// plus loss exceeds demand. Values above one indicate a sign or accounting
/// boundary issue and are returned as outside-domain.
///
/// Citation key preserved: `vermeulen2023`.
pub fn thinfilm_closure_metric(
    resupply_rate: f64,
    loss_rate: f64,
    demand_rate: f64,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::THINFILM_CLOSURE_METRIC;
    validation::ensure_nonnegative("resupply_rate", resupply_rate)?;
    validation::ensure_nonnegative("loss_rate", loss_rate)?;
    validation::ensure_positive("demand_rate", demand_rate)?;

    let value = 1.0 - (resupply_rate + loss_rate) / demand_rate;
    if !value.is_finite() {
        return Err(numerical_failure(ID, "closure metric was not finite"));
    }
    let validity = if value <= 1.0 {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::OutsideDocumentedDomain
    };
    result(value, ID, validity)
}

/// Inputs to the generic thin-film equivalent-system-mass expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquivalentSystemMassInput {
    pub installed_mass_kg: f64,
    pub pressurized_volume_m3: f64,
    pub power_kw: f64,
    pub cooling_kw: f64,
    pub crew_time_h: f64,
    pub volume_equiv_kg_per_m3: f64,
    pub power_equiv_kg_per_kw: f64,
    pub cooling_equiv_kg_per_kw: f64,
    pub crew_time_equiv_kg_per_h: f64,
}

/// Computes `ESM = M + lambda_V V + lambda_P P + lambda_Q Q + lambda_t t_crew`.
///
/// Implements thin-film report Eq. (34). The conversion factors are mission- and
/// architecture-specific and must be supplied by the caller.
///
/// Citation key preserved: `detrell2021`.
pub fn equivalent_system_mass(
    input: EquivalentSystemMassInput,
) -> AeroResult<EngineeringResult<f64>> {
    const ID: &str = ids::THINFILM_ESM;
    validation::ensure_nonnegative("installed_mass_kg", input.installed_mass_kg)?;
    validation::ensure_nonnegative("pressurized_volume_m3", input.pressurized_volume_m3)?;
    validation::ensure_nonnegative("power_kw", input.power_kw)?;
    validation::ensure_nonnegative("cooling_kw", input.cooling_kw)?;
    validation::ensure_nonnegative("crew_time_h", input.crew_time_h)?;
    validation::ensure_nonnegative("volume_equiv_kg_per_m3", input.volume_equiv_kg_per_m3)?;
    validation::ensure_nonnegative("power_equiv_kg_per_kw", input.power_equiv_kg_per_kw)?;
    validation::ensure_nonnegative("cooling_equiv_kg_per_kw", input.cooling_equiv_kg_per_kw)?;
    validation::ensure_nonnegative("crew_time_equiv_kg_per_h", input.crew_time_equiv_kg_per_h)?;

    let value = input.installed_mass_kg
        + input.volume_equiv_kg_per_m3 * input.pressurized_volume_m3
        + input.power_equiv_kg_per_kw * input.power_kw
        + input.cooling_equiv_kg_per_kw * input.cooling_kw
        + input.crew_time_equiv_kg_per_h * input.crew_time_h;
    if value.is_finite() {
        result(value, ID, ValidityStatus::WithinDocumentedDomain)
    } else {
        Err(numerical_failure(ID, "ESM value was not finite"))
    }
}

/// Predicate for `ESM_biological(T) < ESM_physicochemical(T)`.
///
/// Implements thin-film report Eq. (35). This is a scalar comparison only; it
/// does not imply reliability, nutrition, maintainability, or mission readiness.
///
/// Citation key preserved: `detrell2021`.
pub fn esm_biological_is_favorable(
    biological_esm_kg: f64,
    physicochemical_esm_kg: f64,
) -> AeroResult<EngineeringResult<bool>> {
    const ID: &str = ids::THINFILM_ESM_BREAKEVEN;
    validation::ensure_nonnegative("biological_esm_kg", biological_esm_kg)?;
    validation::ensure_nonnegative("physicochemical_esm_kg", physicochemical_esm_kg)?;
    let favorable = biological_esm_kg < physicochemical_esm_kg;
    let validity = if favorable {
        ValidityStatus::WithinDocumentedDomain
    } else {
        ValidityStatus::BoundaryCase
    };
    result(favorable, ID, validity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aero_codex_core::VerificationStatus;

    #[test]
    fn scalar_compartment_derivative_matches_balance() {
        let result = generic_compartment_derivative(
            &[10.0, 2.0],
            &[3.0],
            5.0,
            &[2.0, -1.0],
            &[0.5, 0.2],
            -0.1,
            0.4,
        )
        .unwrap();
        assert!((result.value - 13.3).abs() < 1.0e-12);
        assert_eq!(
            result.verification.status,
            VerificationStatus::EquationTraceable
        );
    }

    #[test]
    fn loop_and_element_residuals_are_matrix_vector_products() {
        let e = element_balance_residual(&[vec![1.0, -1.0], vec![2.0, -2.0]], &[3.0, 3.0]).unwrap();
        assert_eq!(e.value, vec![0.0, 0.0]);

        let loop_res =
            loop_balance_residual(&[vec![2.0, 0.0]], &[1.5, 0.0], &[-3.0], &[0.0]).unwrap();
        assert_eq!(loop_res.value, vec![0.0]);
    }

    #[test]
    fn bounds_and_esm_helpers_behave() {
        assert!(
            within_componentwise_bounds(&[1.0, 2.0], &[0.0, 1.0], &[1.5, 3.0])
                .unwrap()
                .value
        );
        assert!(
            !within_componentwise_bounds(&[4.0], &[0.0], &[3.0])
                .unwrap()
                .value
        );

        let esm = equivalent_system_mass(EquivalentSystemMassInput {
            installed_mass_kg: 100.0,
            pressurized_volume_m3: 2.0,
            power_kw: 3.0,
            cooling_kw: 4.0,
            crew_time_h: 5.0,
            volume_equiv_kg_per_m3: 10.0,
            power_equiv_kg_per_kw: 20.0,
            cooling_equiv_kg_per_kw: 30.0,
            crew_time_equiv_kg_per_h: 40.0,
        })
        .unwrap();
        assert_eq!(esm.value, 500.0);
    }
}
