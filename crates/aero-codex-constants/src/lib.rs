#![forbid(unsafe_code)]
//! Phase 0.001 constants for AeroCodex.
//!
//! These constants are engineering seed values for early, pure-Rust equation
//! implementations. They are not certification data, mission data, or a
//! replacement for project-specific source review. Source registry entries are
//! deliberately conservative and remain `research_required` until exact
//! publication/table provenance, editions, uncertainties, and licensing notes
//! are reviewed.

pub const CODEX_ID: &str = "constants.phase_0_001";

/// Conservative source/verification status used for Phase 0.001 constants.
pub const RESEARCH_REQUIRED_STATUS: &str = "research_required";

/// Source-registry seed for the U.S. Standard Atmosphere 1976 research target.
pub const SOURCE_ID_US_STANDARD_ATMOSPHERE_1976: &str =
    "source.atmosphere.us_standard_atmosphere_1976.research_required";
/// Source-registry seed for physical constants such as `R` and Stefan-Boltzmann.
pub const SOURCE_ID_NIST_CODATA_PHYSICAL_CONSTANTS: &str =
    "source.constants.nist_codata_physical_constants.research_required";
/// Source-registry seed for compressible-flow reference review.
pub const SOURCE_ID_NACA_REPORT_1135: &str =
    "source.gasdynamics.naca_report_1135.research_required";
/// Source-registry seed for NASA Glenn thermodynamics / CEA review.
pub const SOURCE_ID_NASA_GLENN_CEA: &str = "source.thermo.nasa_glenn_cea.research_required";
/// Source-registry seed for NASA/JPL astrodynamics parameter review.
pub const SOURCE_ID_NASA_JPL_ASTRODYNAMICS_PARAMETERS: &str =
    "source.astrodynamics.nasa_jpl_parameters.research_required";
/// Source-registry seed for NASA BVAD/ECLSS life-support review.
pub const SOURCE_ID_NASA_BVAD_ECLSS: &str = "source.life_support.nasa_bvad_eclss.research_required";

/// Lightweight metadata for a Phase 0.001 constant seed.
///
/// This deliberately stores strings only, so the constants crate remains free of
/// serialization, registry, or native dependencies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConstantSeed {
    /// Public Rust constant symbol.
    pub symbol: &'static str,
    /// Numerical value in the listed unit.
    pub value: f64,
    /// Canonical unit used by the public constant symbol.
    pub unit: &'static str,
    /// Conservative source status, currently `research_required`.
    pub source_status: &'static str,
    /// Source-registry entry intended for later review.
    pub source_registry_hint: &'static str,
    /// Caveat or review note for the value.
    pub notes: &'static str,
}

impl ConstantSeed {
    /// Build static metadata for a constant seed.
    pub const fn new(
        symbol: &'static str,
        value: f64,
        unit: &'static str,
        source_status: &'static str,
        source_registry_hint: &'static str,
        notes: &'static str,
    ) -> Self {
        Self {
            symbol,
            value,
            unit,
            source_status,
            source_registry_hint,
            notes,
        }
    }
}

/// Standard gravity, m/s².
pub const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;
/// Universal molar gas constant, J/(mol·K).
pub const UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K: f64 = 8.314_462_618_153_24;
/// Standard sea-level pressure, Pa.
pub const STANDARD_SEA_LEVEL_PRESSURE_PA: f64 = 101_325.0;
/// Standard sea-level temperature, K.
pub const STANDARD_SEA_LEVEL_TEMPERATURE_K: f64 = 288.15;
/// Standard sea-level density, kg/m³.
pub const STANDARD_SEA_LEVEL_DENSITY_KG_M3: f64 = 1.225;
/// Specific gas constant for dry air, J/(kg·K).
pub const STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K: f64 = 287.052_87;
/// Standard heat-capacity ratio for dry air.
pub const STANDARD_GAMMA_DRY_AIR: f64 = 1.4;
/// Stefan-Boltzmann constant, W/(m²·K⁴).
pub const STEFAN_BOLTZMANN_W_PER_M2_K4: f64 = 5.670_374_419e-8;
/// Earth standard gravitational parameter, m³/s².
pub const EARTH_GRAVITATIONAL_PARAMETER_M3_S2: f64 = 3.986_004_418e14;
/// Earth mean radius, m.
pub const EARTH_MEAN_RADIUS_M: f64 = 6_371_000.0;
/// Solar gravitational parameter placeholder, m³/s².
///
/// This is retained only as a Phase 0.001 research seed. It is not used by any
/// Phase 0.001 equation implementation and must not be treated as a verified
/// value until the astrodynamics source-registry review is complete.
pub const SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER: f64 = 1.327_124_400_18e20;
/// Explicit verification flag for the solar gravitational-parameter placeholder.
pub const SOLAR_GRAVITATIONAL_PARAMETER_PLACEHOLDER_VERIFIED: bool = false;

/// Metadata table for all Phase 0.001 constant seed values.
pub const PHASE_0_001_CONSTANT_SEEDS: &[ConstantSeed] = &[
    ConstantSeed::new(
        "STANDARD_GRAVITY_M_S2",
        STANDARD_GRAVITY_M_S2,
        "m/s^2",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_US_STANDARD_ATMOSPHERE_1976,
        "Seed value only; exact source edition/table review pending.",
    ),
    ConstantSeed::new(
        "UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K",
        UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K,
        "J/(mol*K)",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_NIST_CODATA_PHYSICAL_CONSTANTS,
        "Seed value only; exact CODATA/NIST edition and uncertainty review pending.",
    ),
    ConstantSeed::new(
        "STANDARD_SEA_LEVEL_PRESSURE_PA",
        STANDARD_SEA_LEVEL_PRESSURE_PA,
        "Pa",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_US_STANDARD_ATMOSPHERE_1976,
        "Seed value only; standard-atmosphere source review pending.",
    ),
    ConstantSeed::new(
        "STANDARD_SEA_LEVEL_TEMPERATURE_K",
        STANDARD_SEA_LEVEL_TEMPERATURE_K,
        "K",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_US_STANDARD_ATMOSPHERE_1976,
        "Seed value only; standard-atmosphere source review pending.",
    ),
    ConstantSeed::new(
        "STANDARD_SEA_LEVEL_DENSITY_KG_M3",
        STANDARD_SEA_LEVEL_DENSITY_KG_M3,
        "kg/m^3",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_US_STANDARD_ATMOSPHERE_1976,
        "Seed value only; standard-atmosphere source review pending.",
    ),
    ConstantSeed::new(
        "STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K",
        STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K,
        "J/(kg*K)",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_US_STANDARD_ATMOSPHERE_1976,
        "Seed value only; dry-air convention and source review pending.",
    ),
    ConstantSeed::new(
        "STANDARD_GAMMA_DRY_AIR",
        STANDARD_GAMMA_DRY_AIR,
        "dimensionless",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_NACA_REPORT_1135,
        "Seed value only; constant-gamma dry-air assumption review pending.",
    ),
    ConstantSeed::new(
        "STEFAN_BOLTZMANN_W_PER_M2_K4",
        STEFAN_BOLTZMANN_W_PER_M2_K4,
        "W/(m^2*K^4)",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_NIST_CODATA_PHYSICAL_CONSTANTS,
        "Seed value only; exact CODATA/NIST edition and uncertainty review pending.",
    ),
    ConstantSeed::new(
        "EARTH_GRAVITATIONAL_PARAMETER_M3_S2",
        EARTH_GRAVITATIONAL_PARAMETER_M3_S2,
        "m^3/s^2",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_NASA_JPL_ASTRODYNAMICS_PARAMETERS,
        "Seed value only; exact parameter source, epoch, and convention review pending.",
    ),
    ConstantSeed::new(
        "EARTH_MEAN_RADIUS_M",
        EARTH_MEAN_RADIUS_M,
        "m",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_NASA_JPL_ASTRODYNAMICS_PARAMETERS,
        "Seed value only; radius definition and source review pending.",
    ),
    ConstantSeed::new(
        "SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER",
        SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER,
        "m^3/s^2",
        RESEARCH_REQUIRED_STATUS,
        SOURCE_ID_NASA_JPL_ASTRODYNAMICS_PARAMETERS,
        "Placeholder only; not used by Phase 0.001 equations and not source verified.",
    ),
];

/// Return constant metadata by public symbol.
pub fn constant_seed(symbol: &str) -> Option<&'static ConstantSeed> {
    PHASE_0_001_CONSTANT_SEEDS
        .iter()
        .find(|seed| seed.symbol == symbol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_are_positive() {
        let positive_constants = [
            ("STANDARD_GRAVITY_M_S2", STANDARD_GRAVITY_M_S2),
            (
                "UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K",
                UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K,
            ),
            (
                "STANDARD_SEA_LEVEL_PRESSURE_PA",
                STANDARD_SEA_LEVEL_PRESSURE_PA,
            ),
            (
                "STANDARD_SEA_LEVEL_TEMPERATURE_K",
                STANDARD_SEA_LEVEL_TEMPERATURE_K,
            ),
            (
                "STANDARD_SEA_LEVEL_DENSITY_KG_M3",
                STANDARD_SEA_LEVEL_DENSITY_KG_M3,
            ),
            (
                "STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K",
                STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K,
            ),
            ("STEFAN_BOLTZMANN_W_PER_M2_K4", STEFAN_BOLTZMANN_W_PER_M2_K4),
            (
                "EARTH_GRAVITATIONAL_PARAMETER_M3_S2",
                EARTH_GRAVITATIONAL_PARAMETER_M3_S2,
            ),
            ("EARTH_MEAN_RADIUS_M", EARTH_MEAN_RADIUS_M),
            (
                "SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER",
                SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER,
            ),
        ];

        for (symbol, value) in positive_constants {
            assert!(value > 0.0, "{symbol} should be positive");
        }
        let gamma = STANDARD_GAMMA_DRY_AIR;
        assert!(gamma > 1.0, "STANDARD_GAMMA_DRY_AIR should exceed 1.0");
    }

    #[test]
    fn sea_level_density_is_consistent_with_ideal_gas_seed_values() {
        let rho = STANDARD_SEA_LEVEL_PRESSURE_PA
            / (STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K * STANDARD_SEA_LEVEL_TEMPERATURE_K);
        assert!((rho - STANDARD_SEA_LEVEL_DENSITY_KG_M3).abs() < 1.0e-6);
    }

    #[test]
    fn constant_seed_table_covers_public_constants() {
        let symbols = [
            "STANDARD_GRAVITY_M_S2",
            "UNIVERSAL_GAS_CONSTANT_J_PER_MOL_K",
            "STANDARD_SEA_LEVEL_PRESSURE_PA",
            "STANDARD_SEA_LEVEL_TEMPERATURE_K",
            "STANDARD_SEA_LEVEL_DENSITY_KG_M3",
            "STANDARD_AIR_GAS_CONSTANT_J_PER_KG_K",
            "STANDARD_GAMMA_DRY_AIR",
            "STEFAN_BOLTZMANN_W_PER_M2_K4",
            "EARTH_GRAVITATIONAL_PARAMETER_M3_S2",
            "EARTH_MEAN_RADIUS_M",
            "SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER",
        ];
        for symbol in symbols {
            assert!(
                constant_seed(symbol).is_some(),
                "missing seed metadata for {symbol}"
            );
        }
    }

    #[test]
    fn constant_seed_table_keeps_conservative_source_status() {
        assert!(!PHASE_0_001_CONSTANT_SEEDS.is_empty());
        for seed in PHASE_0_001_CONSTANT_SEEDS {
            assert_eq!(seed.source_status, RESEARCH_REQUIRED_STATUS);
            assert!(seed
                .source_registry_hint
                .ends_with(RESEARCH_REQUIRED_STATUS));
            assert!(seed.notes.contains("pending") || seed.notes.contains("not source verified"));
        }
    }

    #[test]
    fn solar_placeholder_is_explicitly_not_verified() {
        let placeholder_verified = SOLAR_GRAVITATIONAL_PARAMETER_PLACEHOLDER_VERIFIED;
        assert!(!placeholder_verified);
        let seed = constant_seed("SOLAR_GRAVITATIONAL_PARAMETER_M3_S2_PLACEHOLDER")
            .expect("solar placeholder metadata should exist");
        assert_eq!(seed.source_status, RESEARCH_REQUIRED_STATUS);
        assert!(seed.notes.contains("not source verified"));
    }
}
