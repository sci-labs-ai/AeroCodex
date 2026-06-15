//! Source and equation traceability for the thin-film and biofilm BLSS models.
//!
//! The constants in this module intentionally preserve both AeroCodex source
//! registry identifiers and the BibTeX keys supplied with the thin-film report.
//! Public equations in the sibling modules reference these Codex IDs in their
//! Rustdoc so citation information can be regenerated from code.

use aero_codex_core::{VerificationRecord, VerificationStatus};

/// Source identifier for the compiled thin-film report supplied to AeroCodex.
pub const SOURCE_THINFILM_REPORT_2026: &str =
    "source.life_support.thinfilm_blss_report_2026.equation_traceable";
/// Source identifier for Poughon et al. (2021), Limnospira indica PBR model.
pub const SOURCE_POUGHON_2021: &str = "source.life_support.poughon_2021.equation_traceable";
/// Source identifier for Garcia-Gragera et al. (2021), MELiSSA integration.
pub const SOURCE_GARCIA_2021: &str = "source.life_support.garcia_2021.equation_traceable";
/// Source identifier for Perez et al. (2005), nitrifying fixed-bed reactor.
pub const SOURCE_PEREZ_2005: &str = "source.life_support.perez_2005.equation_traceable";
/// Source identifier for Montras i Boet (2009), nitrifying packed-bed thesis.
pub const SOURCE_MONTRAS_2009: &str = "source.life_support.montras_2009.equation_traceable";
/// Source identifier for Polizzi et al. (2022), photosynthetic biofilm PDE.
pub const SOURCE_POLIZZI_2022: &str = "source.life_support.polizzi_2022.equation_traceable";
/// Source identifier for Blanken et al. (2014), rotating algal biofilm PBR.
pub const SOURCE_BLANKEN_2014: &str = "source.life_support.blanken_2014.equation_traceable";
/// Source identifier for Blanken (2016), algal biofilm PBR thesis.
pub const SOURCE_BLANKEN_2016: &str = "source.life_support.blanken_2016.equation_traceable";
/// Source identifier for Detrell (2021), algal PBR ESM trade analysis.
pub const SOURCE_DETRELL_2021: &str = "source.life_support.detrell_2021.equation_traceable";
/// Source identifier for Vermeulen et al. (2023), BLSS stoichiometry.
pub const SOURCE_VERMEULEN_2023: &str = "source.life_support.vermeulen_2023.equation_traceable";

/// Every source identifier used by the new thin-film implementation.
pub const THINFILM_SOURCE_IDS: &[&str] = &[
    SOURCE_THINFILM_REPORT_2026,
    SOURCE_POUGHON_2021,
    SOURCE_GARCIA_2021,
    SOURCE_PEREZ_2005,
    SOURCE_MONTRAS_2009,
    SOURCE_POLIZZI_2022,
    SOURCE_BLANKEN_2014,
    SOURCE_BLANKEN_2016,
    SOURCE_DETRELL_2021,
    SOURCE_VERMEULEN_2023,
];

/// Stable Codex identifiers for the new equations.
pub mod ids {
    pub const GENERIC_COMPARTMENT_DERIVATIVE: &str =
        "life_support.thinfilm.generic_compartment.derivative";
    pub const ELEMENT_BALANCE_RESIDUAL: &str =
        "life_support.thinfilm.stoichiometry.element_balance_residual";
    pub const LOOP_BALANCE_RESIDUAL: &str =
        "life_support.thinfilm.stoichiometry.loop_balance_residual";
    pub const DYNAMIC_BOUNDS_CHECK: &str = "life_support.thinfilm.control.dynamic_bounds_check";
    pub const FLAT_TWO_FLUX_IRRADIANCE_RATIO: &str =
        "life_support.thinfilm.pbr.flat_two_flux_irradiance_ratio";
    pub const CYLINDRICAL_TWO_FLUX_IRRADIANCE_RATIO: &str =
        "life_support.thinfilm.pbr.cylindrical_two_flux_irradiance_ratio";
    pub const GEOMETRIC_CORRECTION: &str = "life_support.thinfilm.pbr.geometric_correction";
    pub const LIMNOSPIRA_BIOMASS_RATE: &str = "life_support.thinfilm.pbr.limnospira_biomass_rate";
    pub const LIMNOSPIRA_TIC_RATE: &str = "life_support.thinfilm.pbr.limnospira_tic_rate";
    pub const LIMNOSPIRA_OXYGEN_RATE: &str = "life_support.thinfilm.pbr.limnospira_oxygen_rate";
    pub const CARBONATE_SPECIES: &str = "life_support.thinfilm.pbr.carbonate_species";
    pub const EPS_FRACTION: &str = "life_support.thinfilm.pbr.eps_fraction";
    pub const PBR_TANK_SERIES_RATE: &str = "life_support.thinfilm.pbr.tank_series_rate";
    pub const PBR_FEASIBLE_REGION: &str = "life_support.thinfilm.pbr.feasible_region";
    pub const NITRIFICATION_AOB_STOICHIOMETRY: &str =
        "life_support.thinfilm.c3.nitrification_aob_stoichiometry";
    pub const NITRIFICATION_NOB_STOICHIOMETRY: &str =
        "life_support.thinfilm.c3.nitrification_nob_stoichiometry";
    pub const SPHERICAL_DIFFUSION_RATE: &str = "life_support.thinfilm.c3.spherical_diffusion_rate";
    pub const FICK_BIOFILM_FLUX: &str = "life_support.thinfilm.c3.fick_biofilm_flux";
    pub const BIOFILM_SOLIDS_RATE: &str = "life_support.thinfilm.c3.biofilm_solids_rate";
    pub const BIOFILM_THICKNESS_RATE: &str = "life_support.thinfilm.c3.biofilm_thickness_rate";
    pub const C3_OXYGEN_TRANSFER: &str = "life_support.thinfilm.c3.oxygen_transfer";
    pub const NITROSOMONAS_RATE: &str = "life_support.thinfilm.c3.nitrosomonas_rate";
    pub const NITROBACTER_RATE: &str = "life_support.thinfilm.c3.nitrobacter_rate";
    pub const NITRIFIER_RATES: &str = "life_support.thinfilm.c3.nitrifier_rates";
    pub const C3_TANK_BIOFILM_RATE: &str = "life_support.thinfilm.c3.tank_biofilm_rate";
    pub const OXYGEN_PRODUCTION_DISCONNECTED: &str =
        "life_support.thinfilm.integration.oxygen_production_disconnected";
    pub const OXYGEN_PRODUCTION_CONNECTED: &str =
        "life_support.thinfilm.integration.oxygen_production_connected";
    pub const OXYGEN_FRACTION_ERROR: &str = "life_support.thinfilm.control.oxygen_fraction_error";
    pub const PI_LIGHT_COMMAND: &str = "life_support.thinfilm.control.pi_light_command";
    pub const NITRATE_DILUTION_COMMAND: &str =
        "life_support.thinfilm.control.nitrate_dilution_command";
    pub const ELEMENT_INVENTORY_BALANCE: &str =
        "life_support.thinfilm.system.element_inventory_balance";
    pub const THINFILM_CLOSURE_METRIC: &str = "life_support.thinfilm.system.closure_metric";
    pub const THINFILM_ESM: &str = "life_support.thinfilm.system.equivalent_system_mass";
    pub const THINFILM_ESM_BREAKEVEN: &str = "life_support.thinfilm.system.esm_breakeven";
    pub const ALGAL_BIOFILM_GROWTH_RATE: &str = "life_support.thinfilm.algal_biofilm.growth_rate";
    pub const ALGAL_BIOFILM_LIGHT_PROFILE: &str =
        "life_support.thinfilm.algal_biofilm.light_profile";
    pub const ALGAL_AREAL_PRODUCTIVITY: &str =
        "life_support.thinfilm.algal_biofilm.areal_productivity";
    pub const HARVEST_RESET_THICKNESS: &str =
        "life_support.thinfilm.algal_biofilm.harvest_reset_thickness";
    pub const MIXTURE_COMPONENT_RATE: &str =
        "life_support.thinfilm.algal_biofilm.mixture_component_rate";
    pub const MIXTURE_CLOSURE_RESIDUAL: &str =
        "life_support.thinfilm.algal_biofilm.mixture_closure_residual";
    pub const DISSOLVED_SPECIES_RATE: &str =
        "life_support.thinfilm.algal_biofilm.dissolved_species_rate";
    pub const THINFILM_2D_LIGHT: &str = "life_support.thinfilm.algal_biofilm.light_2d";
    pub const CARBON_BOUNDARY_FLUX: &str =
        "life_support.thinfilm.algal_biofilm.carbon_boundary_flux";
    pub const OXYGEN_BOUNDARY_FLUX: &str =
        "life_support.thinfilm.algal_biofilm.oxygen_boundary_flux";
    pub const NITRATE_BOUNDARY_RESIDUAL: &str =
        "life_support.thinfilm.algal_biofilm.nitrate_boundary_residual";
    pub const ROM_SERVICE_PROJECTION: &str = "life_support.thinfilm.rom.service_projection";
    pub const HABITAT_COUPLED_DERIVATIVE: &str =
        "life_support.thinfilm.rom.habitat_coupled_derivative";
    pub const SERVICE_VECTOR: &str = "life_support.thinfilm.rom.service_vector";
    pub const LINEAR_ROM_SERVICE: &str = "life_support.thinfilm.rom.linear_service";
    pub const VALIDATED_DOMAIN_CHECK: &str = "life_support.thinfilm.rom.validated_domain_check";
}

/// Traceability record for one implemented equation or algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquationReference {
    /// Equation number in the supplied thin-film report.
    pub equation: &'static str,
    /// Stable AeroCodex identifier.
    pub codex_id: &'static str,
    /// Public Rust function or API item implementing the equation.
    pub function_name: &'static str,
    /// BibTeX keys preserved from `blss_thinfilm_refs.bib`.
    pub bib_keys: &'static [&'static str],
    /// AeroCodex source-registry identifiers.
    pub source_ids: &'static [&'static str],
    /// Source verification maturity for the implementation.
    pub verification_status: VerificationStatus,
    /// Locator in the compiled report.
    pub report_locator: &'static str,
    /// Short implementation note.
    pub notes: &'static str,
}

const REPORT_ONLY: &[&str] = &[SOURCE_THINFILM_REPORT_2026];
const PBR_SOURCES: &[&str] = &[SOURCE_THINFILM_REPORT_2026, SOURCE_POUGHON_2021];
const C3_SOURCES: &[&str] = &[
    SOURCE_THINFILM_REPORT_2026,
    SOURCE_PEREZ_2005,
    SOURCE_MONTRAS_2009,
    SOURCE_GARCIA_2021,
];
const INTEGRATION_SOURCES: &[&str] = &[SOURCE_THINFILM_REPORT_2026, SOURCE_GARCIA_2021];
const SYSTEM_SOURCES: &[&str] = &[
    SOURCE_THINFILM_REPORT_2026,
    SOURCE_VERMEULEN_2023,
    SOURCE_DETRELL_2021,
];
const ALGAL_SOURCES: &[&str] = &[
    SOURCE_THINFILM_REPORT_2026,
    SOURCE_POLIZZI_2022,
    SOURCE_BLANKEN_2014,
    SOURCE_BLANKEN_2016,
];

/// Equation-to-code map for the thin-film BLSS implementation.
pub const EQUATION_REFERENCES: &[EquationReference] = &[
    EquationReference {
        equation: "1",
        codex_id: ids::GENERIC_COMPARTMENT_DERIVATIVE,
        function_name: "brlss_backbone::generic_compartment_derivative",
        bib_keys: &["poughon2021", "garcia2021", "montras2009"],
        source_ids: REPORT_ONLY,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 4",
        notes: "State-vector structure and scalar derivative helper for Eq. (2).",
    },
    EquationReference {
        equation: "2",
        codex_id: ids::GENERIC_COMPARTMENT_DERIVATIVE,
        function_name: "brlss_backbone::generic_compartment_derivative",
        bib_keys: &["poughon2021", "garcia2021", "montras2009"],
        source_ids: REPORT_ONLY,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 4",
        notes: "Mass-balanced compartment synthesis.",
    },
    EquationReference {
        equation: "3",
        codex_id: ids::ELEMENT_BALANCE_RESIDUAL,
        function_name: "brlss_backbone::element_balance_residual",
        bib_keys: &["vermeulen2023"],
        source_ids: SYSTEM_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 4",
        notes: "Static elemental closure residual A xi.",
    },
    EquationReference {
        equation: "4",
        codex_id: ids::LOOP_BALANCE_RESIDUAL,
        function_name: "brlss_backbone::loop_balance_residual",
        bib_keys: &["vermeulen2023"],
        source_ids: SYSTEM_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 4",
        notes: "Commodity loop balance residual B xi + crew + stores.",
    },
    EquationReference {
        equation: "5",
        codex_id: ids::DYNAMIC_BOUNDS_CHECK,
        function_name: "brlss_backbone::within_componentwise_bounds",
        bib_keys: &[],
        source_ids: REPORT_ONLY,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 4",
        notes: "Componentwise control/domain bound check.",
    },
    EquationReference {
        equation: "6",
        codex_id: ids::FLAT_TWO_FLUX_IRRADIANCE_RATIO,
        function_name: "melissa_photobioreactor::flat_two_flux_irradiance_ratio",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.2",
        notes: "Flat two-flux light-transfer ratio.",
    },
    EquationReference {
        equation: "7",
        codex_id: ids::CYLINDRICAL_TWO_FLUX_IRRADIANCE_RATIO,
        function_name: "melissa_photobioreactor::cylindrical_two_flux_irradiance_ratio",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.2",
        notes: "Cylindrical two-flux ratio with internal Bessel-series evaluator.",
    },
    EquationReference {
        equation: "8",
        codex_id: ids::GEOMETRIC_CORRECTION,
        function_name: "melissa_photobioreactor::geometric_correction",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 5.2",
        notes: "Geometric correction min(1, L0/L).",
    },
    EquationReference {
        equation: "9",
        codex_id: ids::LIMNOSPIRA_BIOMASS_RATE,
        function_name: "melissa_photobioreactor::limnospira_biomass_rate",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.3",
        notes: "Reduced biomass growth balance.",
    },
    EquationReference {
        equation: "10",
        codex_id: ids::LIMNOSPIRA_TIC_RATE,
        function_name: "melissa_photobioreactor::limnospira_tic_rate",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.3",
        notes: "Reduced total inorganic carbon balance.",
    },
    EquationReference {
        equation: "11",
        codex_id: ids::LIMNOSPIRA_OXYGEN_RATE,
        function_name: "melissa_photobioreactor::limnospira_oxygen_rate",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.3",
        notes: "Reduced dissolved oxygen balance.",
    },
    EquationReference {
        equation: "12",
        codex_id: ids::CARBONATE_SPECIES,
        function_name: "melissa_photobioreactor::carbonate_species_from_total_and_ph",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.3",
        notes: "Total inorganic carbon closure over carbonate species.",
    },
    EquationReference {
        equation: "13",
        codex_id: ids::CARBONATE_SPECIES,
        function_name: "melissa_photobioreactor::carbonate_species_from_total_and_ph",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.3",
        notes: "TIC split using supplied pKa values.",
    },
    EquationReference {
        equation: "14",
        codex_id: ids::EPS_FRACTION,
        function_name: "melissa_photobioreactor::eps_fraction_from_photon_requirement",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.3",
        notes: "EPS feasibility relation.",
    },
    EquationReference {
        equation: "15",
        codex_id: ids::PBR_TANK_SERIES_RATE,
        function_name: "melissa_photobioreactor::tank_series_concentration_rate",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 5.4",
        notes: "Tank-in-series PBR concentration balance.",
    },
    EquationReference {
        equation: "16",
        codex_id: ids::PBR_FEASIBLE_REGION,
        function_name: "melissa_photobioreactor::pbr_operating_point_feasible",
        bib_keys: &["poughon2021"],
        source_ids: PBR_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 5.4",
        notes: "Boolean feasibility-map predicate.",
    },
    EquationReference {
        equation: "17",
        codex_id: ids::NITRIFICATION_AOB_STOICHIOMETRY,
        function_name: "nitrifying_biofilm::nitrification_fluxes",
        bib_keys: &["garcia2021"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 6.1",
        notes: "AOB stoichiometric flux contribution.",
    },
    EquationReference {
        equation: "18",
        codex_id: ids::NITRIFICATION_NOB_STOICHIOMETRY,
        function_name: "nitrifying_biofilm::nitrification_fluxes",
        bib_keys: &["garcia2021"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 6.1",
        notes: "NOB stoichiometric flux contribution.",
    },
    EquationReference {
        equation: "19",
        codex_id: ids::SPHERICAL_DIFFUSION_RATE,
        function_name: "nitrifying_biofilm::spherical_diffusion_reaction_rate",
        bib_keys: &["montras2009", "perez2005"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 6.2",
        notes: "Spherical Fickian diffusion plus reaction.",
    },
    EquationReference {
        equation: "20",
        codex_id: ids::FICK_BIOFILM_FLUX,
        function_name: "nitrifying_biofilm::fick_liquid_flux",
        bib_keys: &["montras2009"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 6.2",
        notes: "Fick-law liquid flux.",
    },
    EquationReference {
        equation: "21",
        codex_id: ids::BIOFILM_SOLIDS_RATE,
        function_name: "nitrifying_biofilm::biofilm_solids_local_rate",
        bib_keys: &["montras2009"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 6.2",
        notes: "Solids balance rearranged as local time derivative.",
    },
    EquationReference {
        equation: "22",
        codex_id: ids::BIOFILM_THICKNESS_RATE,
        function_name: "nitrifying_biofilm::biofilm_thickness_rate",
        bib_keys: &["montras2009"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 6.2",
        notes: "Biofilm thickness kinematic rate.",
    },
    EquationReference {
        equation: "23",
        codex_id: ids::C3_OXYGEN_TRANSFER,
        function_name: "nitrifying_biofilm::gas_liquid_transfer_flux",
        bib_keys: &["montras2009"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 6.2",
        notes: "Oxygen gas-liquid transfer flux.",
    },
    EquationReference {
        equation: "24",
        codex_id: ids::NITRIFIER_RATES,
        function_name: "nitrifying_biofilm::nitrifier_rates",
        bib_keys: &["montras2009"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 6.3",
        notes: "Nitrosomonas growth-rate term.",
    },
    EquationReference {
        equation: "25",
        codex_id: ids::NITRIFIER_RATES,
        function_name: "nitrifying_biofilm::nitrifier_rates",
        bib_keys: &["montras2009"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 6.3",
        notes: "Nitrobacter growth-rate term.",
    },
    EquationReference {
        equation: "26",
        codex_id: ids::C3_TANK_BIOFILM_RATE,
        function_name: "nitrifying_biofilm::bulk_tank_biofilm_coupled_rate",
        bib_keys: &["montras2009"],
        source_ids: C3_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 6.4",
        notes: "Tank-in-series bulk balance coupled to surface flux.",
    },
    EquationReference {
        equation: "27",
        codex_id: ids::OXYGEN_PRODUCTION_DISCONNECTED,
        function_name: "melissa_photobioreactor::oxygen_production_rate_disconnected_gas",
        bib_keys: &["garcia2021"],
        source_ids: INTEGRATION_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 7.2",
        notes: "Disconnected gas oxygen-production balance.",
    },
    EquationReference {
        equation: "28",
        codex_id: ids::OXYGEN_PRODUCTION_CONNECTED,
        function_name: "melissa_photobioreactor::oxygen_production_rate_connected_gas",
        bib_keys: &["garcia2021"],
        source_ids: INTEGRATION_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 7.2",
        notes: "Connected gas oxygen-production balance.",
    },
    EquationReference {
        equation: "29",
        codex_id: ids::OXYGEN_FRACTION_ERROR,
        function_name: "melissa_photobioreactor::oxygen_fraction_error",
        bib_keys: &["garcia2021"],
        source_ids: INTEGRATION_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 7.3",
        notes: "Oxygen fraction set-point error.",
    },
    EquationReference {
        equation: "30",
        codex_id: ids::PI_LIGHT_COMMAND,
        function_name: "melissa_photobioreactor::pi_light_command",
        bib_keys: &["garcia2021"],
        source_ids: INTEGRATION_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 7.3",
        notes: "Saturated PI light command.",
    },
    EquationReference {
        equation: "31",
        codex_id: ids::NITRATE_DILUTION_COMMAND,
        function_name: "melissa_photobioreactor::nitrate_dilution_command",
        bib_keys: &["garcia2021"],
        source_ids: INTEGRATION_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 7.3",
        notes: "Saturated nitrate-feedback dilution command.",
    },
    EquationReference {
        equation: "32",
        codex_id: ids::ELEMENT_INVENTORY_BALANCE,
        function_name: "brlss_backbone::element_inventory_rate",
        bib_keys: &["vermeulen2023"],
        source_ids: SYSTEM_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 8.1",
        notes: "Open-system elemental inventory rate.",
    },
    EquationReference {
        equation: "33",
        codex_id: ids::THINFILM_CLOSURE_METRIC,
        function_name: "brlss_backbone::thinfilm_closure_metric",
        bib_keys: &["vermeulen2023"],
        source_ids: SYSTEM_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 8.1",
        notes: "Commodity closure metric.",
    },
    EquationReference {
        equation: "34",
        codex_id: ids::THINFILM_ESM,
        function_name: "brlss_backbone::equivalent_system_mass",
        bib_keys: &["detrell2021"],
        source_ids: SYSTEM_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 8.2",
        notes: "Equivalent System Mass expression.",
    },
    EquationReference {
        equation: "35",
        codex_id: ids::THINFILM_ESM_BREAKEVEN,
        function_name: "brlss_backbone::esm_biological_is_favorable",
        bib_keys: &["detrell2021"],
        source_ids: SYSTEM_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 8.2",
        notes: "ESM comparison predicate.",
    },
    EquationReference {
        equation: "36",
        codex_id: ids::ALGAL_BIOFILM_GROWTH_RATE,
        function_name: "thinfilm_algal_biofilm::local_biofilm_growth_rate",
        bib_keys: &["blanken2014", "blanken2016"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.1",
        notes: "Local attached-algal growth rate.",
    },
    EquationReference {
        equation: "37",
        codex_id: ids::ALGAL_BIOFILM_LIGHT_PROFILE,
        function_name: "thinfilm_algal_biofilm::biofilm_light_from_layers",
        bib_keys: &["blanken2014", "polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.1",
        notes: "Depth-integrated Beer attenuation.",
    },
    EquationReference {
        equation: "38",
        codex_id: ids::ALGAL_AREAL_PRODUCTIVITY,
        function_name: "thinfilm_algal_biofilm::areal_productivity_trapezoid",
        bib_keys: &["blanken2014", "blanken2016"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.1",
        notes: "Areal productivity integral using trapezoidal quadrature.",
    },
    EquationReference {
        equation: "39",
        codex_id: ids::HARVEST_RESET_THICKNESS,
        function_name: "thinfilm_algal_biofilm::harvest_reset_thickness",
        bib_keys: &["blanken2016"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 9.1",
        notes: "Post-harvest film-thickness reset.",
    },
    EquationReference {
        equation: "40",
        codex_id: ids::MIXTURE_COMPONENT_RATE,
        function_name: "thinfilm_algal_biofilm::mixture_component_local_rate",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.2",
        notes: "Mixture volume-fraction local rate.",
    },
    EquationReference {
        equation: "41",
        codex_id: ids::MIXTURE_CLOSURE_RESIDUAL,
        function_name: "thinfilm_algal_biofilm::mixture_closure_residual",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 9.2",
        notes: "Volume-fraction sum residual.",
    },
    EquationReference {
        equation: "42",
        codex_id: ids::DISSOLVED_SPECIES_RATE,
        function_name: "thinfilm_algal_biofilm::dissolved_species_local_rate",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.2",
        notes: "Dissolved-species advection-diffusion-reaction rate.",
    },
    EquationReference {
        equation: "43",
        codex_id: ids::THINFILM_2D_LIGHT,
        function_name: "thinfilm_algal_biofilm::two_dimensional_light_attenuation",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.2",
        notes: "2D light closure with path integral supplied by caller.",
    },
    EquationReference {
        equation: "44",
        codex_id: ids::CARBON_BOUNDARY_FLUX,
        function_name: "thinfilm_algal_biofilm::boundary_mass_transfer_residual",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.2",
        notes: "Carbon boundary flux residual.",
    },
    EquationReference {
        equation: "45",
        codex_id: ids::OXYGEN_BOUNDARY_FLUX,
        function_name: "thinfilm_algal_biofilm::boundary_mass_transfer_residual",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.2",
        notes: "Oxygen boundary flux residual.",
    },
    EquationReference {
        equation: "46",
        codex_id: ids::NITRATE_BOUNDARY_RESIDUAL,
        function_name: "thinfilm_algal_biofilm::nitrate_boundary_residual",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "section 9.2",
        notes: "Dirichlet wetted-boundary residual.",
    },
    EquationReference {
        equation: "47",
        codex_id: ids::ROM_SERVICE_PROJECTION,
        function_name: "thinfilm_algal_biofilm::linear_rom_service",
        bib_keys: &["polizzi2022", "poughon2021", "garcia2021"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.3",
        notes: "Reduced-order service projection.",
    },
    EquationReference {
        equation: "48",
        codex_id: ids::HABITAT_COUPLED_DERIVATIVE,
        function_name: "thinfilm_algal_biofilm::habitat_coupled_derivative",
        bib_keys: &["garcia2021", "vermeulen2023"],
        source_ids: SYSTEM_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "section 9.3",
        notes: "Habitat derivative from crew forcing, service vector, and losses.",
    },
    EquationReference {
        equation: "49",
        codex_id: ids::SERVICE_VECTOR,
        function_name: "thinfilm_algal_biofilm::thinfilm_service_vector",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "appendix B",
        notes: "Operational service-vector constructor.",
    },
    EquationReference {
        equation: "50",
        codex_id: ids::LINEAR_ROM_SERVICE,
        function_name: "thinfilm_algal_biofilm::linear_rom_service",
        bib_keys: &["polizzi2022"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::EquationTraceable,
        report_locator: "appendix B",
        notes: "Linear basis ROM y = W Phi + epsilon.",
    },
    EquationReference {
        equation: "51",
        codex_id: ids::VALIDATED_DOMAIN_CHECK,
        function_name: "thinfilm_algal_biofilm::validated_domain_contains",
        bib_keys: &["poughon2021", "polizzi2022", "garcia2021"],
        source_ids: ALGAL_SOURCES,
        verification_status: VerificationStatus::ImplementationVerified,
        report_locator: "appendix B",
        notes: "Validated-domain membership predicate.",
    },
];

/// Looks up equation traceability metadata by Codex ID.
#[must_use]
pub fn equation_reference(codex_id: &str) -> Option<&'static EquationReference> {
    EQUATION_REFERENCES
        .iter()
        .find(|reference| reference.codex_id == codex_id)
}

/// Creates an AeroCodex verification record for a thin-film Codex ID.
#[must_use]
pub fn verification_record(codex_id: &'static str) -> Option<VerificationRecord> {
    equation_reference(codex_id).map(|reference| {
        VerificationRecord::new(
            reference.codex_id,
            reference.verification_status,
            reference.source_ids,
            reference.notes,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn thinfilm_equation_references_have_sources_and_functions() {
        assert!(EQUATION_REFERENCES.len() >= 51);
        for reference in EQUATION_REFERENCES {
            assert!(!reference.equation.is_empty());
            assert!(reference.codex_id.starts_with("life_support.thinfilm."));
            assert!(!reference.function_name.is_empty());
            assert!(!reference.source_ids.is_empty());
            assert!(verification_record(reference.codex_id).is_some());
        }
    }

    #[test]
    fn thinfilm_source_ids_are_unique() {
        let mut seen = BTreeSet::new();
        for source in THINFILM_SOURCE_IDS {
            assert!(seen.insert(*source));
        }
    }
}
