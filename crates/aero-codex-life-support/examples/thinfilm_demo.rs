//! Thin-film BLSS smoke example.
//!
//! Run with:
//! `cargo run -p aero-codex-life-support --example thinfilm_demo`

use aero_codex_life_support::brlss_backbone::{equivalent_system_mass, EquivalentSystemMassInput};
use aero_codex_life_support::melissa_photobioreactor::{
    carbonate_species_from_total_and_ph, geometric_correction, limnospira_biomass_rate,
};
use aero_codex_life_support::nitrifying_biofilm::{
    nitrification_fluxes, nitrifier_rates, NitrifierKineticParameters,
};
use aero_codex_life_support::thinfilm_algal_biofilm::{
    biofilm_light_uniform, thinfilm_service_vector,
};
use aero_codex_life_support::thinfilm_equation_reference;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let light_correction = geometric_correction(0.02, 0.04)?;
    println!("geometric correction = {:.3}", light_correction.value);

    let biomass_rate = limnospira_biomass_rate(0.10, 0.02, 0.01, 4.0)?;
    println!("C4a biomass rate = {:.6}", biomass_rate.value);

    let carbonate = carbonate_species_from_total_and_ph(10.0, 7.2, 6.35, 10.33)?;
    println!(
        "carbonate species: CO2={:.4}, HCO3={:.4}, CO3={:.4}",
        carbonate.value.co2, carbonate.value.hco3, carbonate.value.co3
    );

    let nitrification = nitrification_fluxes(1.0, 1.0)?;
    println!(
        "overall nitrification O2 flux = {:.3}",
        nitrification.value.o2
    );

    let rates = nitrifier_rates(
        0.1,
        0.05,
        0.002,
        1.0,
        1.0,
        NitrifierKineticParameters::default(),
    )?;
    println!(
        "nitrifier rates: Nts={:.6}, Ntb={:.6}",
        rates.value.nitrosomonas_rate, rates.value.nitrobacter_rate
    );

    let light = biofilm_light_uniform(100.0, 0.001, 200.0, 1.0, 80.0, 0.5)?;
    println!("thin-film light at depth = {:.6}", light.value);

    let service = thinfilm_service_vector(0.002, 0.002, 0.0001, 1.0e-6, 0.05)?;
    println!("service vector = {:?}", service.value.as_array());

    let esm = equivalent_system_mass(EquivalentSystemMassInput {
        installed_mass_kg: 100.0,
        pressurized_volume_m3: 2.0,
        power_kw: 1.5,
        cooling_kw: 1.5,
        crew_time_h: 0.25,
        volume_equiv_kg_per_m3: 10.0,
        power_equiv_kg_per_kw: 20.0,
        cooling_equiv_kg_per_kw: 30.0,
        crew_time_equiv_kg_per_h: 40.0,
    })?;
    println!("illustrative ESM = {:.3} kg-equivalent", esm.value);

    let reference = thinfilm_equation_reference("life_support.thinfilm.pbr.geometric_correction")
        .expect("geometric correction traceability should exist");
    println!(
        "citation mapping: Eq. {} -> {} via {:?}",
        reference.equation, reference.function_name, reference.bib_keys
    );

    Ok(())
}
