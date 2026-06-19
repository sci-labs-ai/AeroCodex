/// Scenario-local clean-room resource kinds for synthetic resource accounting.
///
/// These are synthetic planning categories, not biological, ecological,
/// nutritional, chemical, medical, habitat-safety, or operational fidelity
/// claims. The labels and units are canonical record metadata only: B2a performs
/// no conversion, rate equation, process model, conservation proof, control
/// output, or physical simulation.
///
/// This enum is intentionally separate from
/// `biosim_resource_tick::BioSimResourceKind`. The existing resource-tick module
/// tracks flat resource quantities. B2a adds only a compartment-aware scenario
/// domain layer. No adapter exists here, no compartment state is aggregated, and
/// scenario Water, Biomass, or Food are not mapped into the flat resource kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BioSimScenarioResourceKind {
    Oxygen,
    CarbonDioxide,
    Water,
    Biomass,
    Food,
    ElectricalEnergy,
}

/// Stable validated identifier for a synthetic scenario compartment.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BioSimCompartmentId(String);

impl BioSimCompartmentId {
    /// Parses a compartment identifier using the B2a scenario ID policy.
    ///
    /// Accepted identifiers are ASCII lower-snake identifiers: first character
    /// `a..z`, then `a..z`, `0..9`, or `_`, with at most 64 bytes.
    pub fn new(value: impl Into<String>) -> Result<Self, BioSimScenarioValidationError> {
        let value = value.into();
        validate_biosim_scenario_identifier("compartment_id", &value)?;
        Ok(Self(value))
    }

    /// Returns the validated compartment identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable validated identifier for a synthetic process definition.
///
/// B2b-1 introduces process identifiers for deterministic intent planning only.
/// These identifiers do not authorize replay, state mutation, adapter mapping,
/// external BioSim compatibility, or operational control behavior.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BioSimProcessId(String);

impl BioSimProcessId {
    /// Parses a process identifier using the same lower-snake policy as compartments.
    ///
    /// Accepted identifiers are ASCII lower-snake identifiers: first character
    /// `a..z`, then `a..z`, `0..9`, or `_`, with at most 64 bytes.
    pub fn new(value: impl Into<String>) -> Result<Self, BioSimScenarioValidationError> {
        let value = value.into();
        validate_biosim_scenario_identifier("process_id", &value)?;
        Ok(Self(value))
    }

    /// Returns the validated process identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Metadata carried by a clean-room synthetic scenario.
///
/// Public fields are untrusted assembly data for B2a review and tests.
/// Constructor success is not scenario validity; callers must run
/// `validate_biosim_scenario` before downstream use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BioSimScenarioMetadata {
    pub scenario_id: String,
    pub label: String,
    pub validation_status: String,
}

impl BioSimScenarioMetadata {
    /// Constructs scenario metadata with syntactic ID and nonempty-label checks.
    ///
    /// This constructor intentionally does not enforce `research_required`; the
    /// full status policy is part of `validate_biosim_scenario(...)` in B2a
    /// B2a validation so direct struct construction and fixture-like metadata receive
    /// the same validation path.
    pub fn new(
        scenario_id: impl Into<String>,
        label: impl Into<String>,
        validation_status: impl Into<String>,
    ) -> Result<Self, BioSimScenarioValidationError> {
        let scenario_id = scenario_id.into();
        validate_biosim_scenario_identifier("scenario_id", &scenario_id)?;

        let label = normalized_nonempty_label("scenario_label", label.into())?;
        let validation_status =
            normalized_nonempty_label("validation_status", validation_status.into())?;

        Ok(Self {
            scenario_id,
            label,
            validation_status,
        })
    }
}

/// One named compartment in a synthetic resource-accounting scenario.
///
/// Public fields are untrusted assembly data. Validation is mandatory before
/// downstream use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BioSimCompartment {
    pub id: BioSimCompartmentId,
    pub label: String,
}

impl BioSimCompartment {
    /// Constructs a compartment from a validated ID and a nonempty display label.
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
    ) -> Result<Self, BioSimScenarioValidationError> {
        Ok(Self {
            id: BioSimCompartmentId::new(id)?,
            label: normalized_nonempty_label("compartment_label", label.into())?,
        })
    }
}

/// Initial resource amount in one compartment/resource cell.
///
/// Public fields are untrusted assembly data. `amount` is accepted only when it
/// is finite and nonnegative under `validate_biosim_scenario`; zero and negative
/// zero are record-level zero amounts, not biological or physical claims.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimInitialStore {
    pub compartment_id: BioSimCompartmentId,
    pub resource_kind: BioSimScenarioResourceKind,
    pub amount: f64,
}

impl BioSimInitialStore {
    /// Constructs an initial store cell.
    ///
    /// Numeric validity is intentionally checked by the full scenario validator
    /// in B2a validation so invalid direct struct construction and constructor-created
    /// values share one validation path.
    #[must_use]
    pub const fn new(
        compartment_id: BioSimCompartmentId,
        resource_kind: BioSimScenarioResourceKind,
        amount: f64,
    ) -> Self {
        Self {
            compartment_id,
            resource_kind,
            amount,
        }
    }
}

/// Discrete scenario clock metadata.
///
/// This is metadata only. A large `tick_count` does not allocate, replay, or
/// authorize B2b execution; elapsed-duration overflow/nonfinite cases are
/// rejected by validation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimScenarioClock {
    pub tick_duration_seconds: f64,
    pub tick_count: u64,
}

impl BioSimScenarioClock {
    /// Constructs a scenario clock without performing full numerical validation.
    ///
    /// B2a validation adds full clock validation, including positive finite duration,
    /// positive tick count, and finite elapsed-time checks.
    #[must_use]
    pub const fn new(tick_duration_seconds: f64, tick_count: u64) -> Self {
        Self {
            tick_duration_seconds,
            tick_count,
        }
    }
}

/// Clean-room synthetic scenario domain object for B2a.
///
/// This type intentionally contains no process definitions, replay state,
/// ledger rows, formatter output, file loading, or external fixture references.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenario {
    pub metadata: BioSimScenarioMetadata,
    pub compartments: Vec<BioSimCompartment>,
    pub initial_stores: Vec<BioSimInitialStore>,
    pub clock: BioSimScenarioClock,
}

impl BioSimScenario {
    /// Constructs a scenario domain object.
    ///
    /// Full structural validation is added in B2a validation. This constructor keeps
    /// the domain type easy to assemble for tests and future validators.
    #[must_use]
    pub fn new(
        metadata: BioSimScenarioMetadata,
        compartments: Vec<BioSimCompartment>,
        initial_stores: Vec<BioSimInitialStore>,
        clock: BioSimScenarioClock,
    ) -> Self {
        Self {
            metadata,
            compartments,
            initial_stores,
            clock,
        }
    }
}

/// Validation report used by the scenario domain and B2a structural validator.
///
/// Reports are plain data, not an authority token. Only a report produced by
/// `validate_biosim_scenario` for the exact scenario under review establishes
/// the bounded local structural-validation result.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenarioValidationReport {
    pub errors: Vec<BioSimScenarioValidationError>,
    pub warnings: Vec<String>,
}

impl BioSimScenarioValidationReport {
    /// Returns true when the report contains no validation errors.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Validates the B2a clean-room synthetic scenario structure.
///
/// B2a validation is intentionally limited to metadata, compartment,
/// initial-store, and clock structure. Process-shape validation, replay,
/// ledger/report formatting, and example behavior are deferred to B2b/B2c.
///
/// The function collects deterministic structural errors instead of failing
/// at the first error so reviewer tests can inspect all applicable failures.
#[must_use]
pub fn validate_biosim_scenario(scenario: &BioSimScenario) -> BioSimScenarioValidationReport {
    let mut errors = Vec::new();

    if let Err(error) =
        validate_biosim_scenario_identifier("scenario_id", &scenario.metadata.scenario_id)
    {
        errors.push(error);
    }

    if let Err(error) = validate_biosim_scenario_text("scenario_label", &scenario.metadata.label) {
        errors.push(error);
    }

    match validate_biosim_scenario_text("validation_status", &scenario.metadata.validation_status) {
        Ok(()) if scenario.metadata.validation_status != "research_required" => {
            errors.push(BioSimScenarioValidationError::NonResearchRequiredStatus {
                status: scenario.metadata.validation_status.clone(),
            });
        }
        Ok(()) => {}
        Err(error) => errors.push(error),
    }

    let mut compartment_ids = std::collections::BTreeSet::new();
    if scenario.compartments.is_empty() {
        errors.push(BioSimScenarioValidationError::NoCompartments);
    }

    for compartment in &scenario.compartments {
        let compartment_id = compartment.id.as_str();

        if let Err(error) = validate_biosim_scenario_identifier("compartment_id", compartment_id) {
            errors.push(error);
        }

        if let Err(error) = validate_biosim_scenario_text("compartment_label", &compartment.label) {
            errors.push(error);
        }

        if !compartment_ids.insert(compartment_id.to_owned()) {
            errors.push(BioSimScenarioValidationError::DuplicateCompartmentId {
                compartment_id: compartment_id.to_owned(),
            });
        }
    }

    if !scenario.clock.tick_duration_seconds.is_finite()
        || scenario.clock.tick_duration_seconds <= 0.0
    {
        errors.push(BioSimScenarioValidationError::InvalidTickDurationSeconds {
            value: scenario.clock.tick_duration_seconds,
        });
    }

    if scenario.clock.tick_count == 0 {
        errors.push(BioSimScenarioValidationError::InvalidTickCount {
            value: scenario.clock.tick_count,
        });
    }

    if scenario.clock.tick_duration_seconds.is_finite()
        && scenario.clock.tick_duration_seconds > 0.0
        && scenario.clock.tick_count > 0
    {
        let elapsed_seconds =
            scenario.clock.tick_duration_seconds * scenario.clock.tick_count as f64;
        if !elapsed_seconds.is_finite() {
            errors.push(BioSimScenarioValidationError::ElapsedClockNotFinite {
                tick_duration_seconds: scenario.clock.tick_duration_seconds,
                tick_count: scenario.clock.tick_count,
            });
        }
    }

    let mut initial_store_cells = std::collections::BTreeSet::new();
    for store in &scenario.initial_stores {
        let compartment_id = store.compartment_id.as_str();

        if let Err(error) =
            validate_biosim_scenario_identifier("initial_store.compartment_id", compartment_id)
        {
            errors.push(error);
        }

        if !compartment_ids.contains(compartment_id) {
            errors.push(
                BioSimScenarioValidationError::UnknownInitialStoreCompartment {
                    compartment_id: compartment_id.to_owned(),
                },
            );
        }

        let cell_key = (compartment_id.to_owned(), store.resource_kind);
        if !initial_store_cells.insert(cell_key) {
            errors.push(BioSimScenarioValidationError::DuplicateInitialStore {
                compartment_id: compartment_id.to_owned(),
                resource_kind: store.resource_kind,
            });
        }

        if !store.amount.is_finite() {
            errors.push(BioSimScenarioValidationError::NonfiniteInitialStore {
                compartment_id: compartment_id.to_owned(),
                resource_kind: store.resource_kind,
                amount: store.amount,
            });
        } else if store.amount < 0.0 {
            errors.push(BioSimScenarioValidationError::NegativeInitialStore {
                compartment_id: compartment_id.to_owned(),
                resource_kind: store.resource_kind,
                amount: store.amount,
            });
        }
    }

    BioSimScenarioValidationReport {
        errors,
        warnings: Vec::new(),
    }
}

/// Validation errors for the B2a domain and the B2a validation structural validator.
#[derive(Debug, Clone, PartialEq)]
pub enum BioSimScenarioValidationError {
    InvalidIdentifier {
        field: &'static str,
        value: String,
        expected: &'static str,
    },
    EmptyLabel {
        field: &'static str,
    },
    InvalidText {
        field: &'static str,
        value: String,
        expected: &'static str,
    },
    NoCompartments,
    DuplicateCompartmentId {
        compartment_id: String,
    },
    DuplicateInitialStore {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
    },
    UnknownInitialStoreCompartment {
        compartment_id: String,
    },
    NonResearchRequiredStatus {
        status: String,
    },
    InvalidTickDurationSeconds {
        value: f64,
    },
    InvalidTickCount {
        value: u64,
    },
    ElapsedClockNotFinite {
        tick_duration_seconds: f64,
        tick_count: u64,
    },
    NegativeInitialStore {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        amount: f64,
    },
    NonfiniteInitialStore {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        amount: f64,
    },
}

/// Returns the scenario-layer canonical ID for a resource kind.
///
/// These IDs are scenario-local synthetic accounting labels, not existing
/// `biosim_resource_tick` catalog IDs, not external fixture names, and not
/// implementation approval. Generic Water is not assumed potable or waste water,
/// Biomass is not assumed edible, and Food remains separate from Biomass.
#[must_use]
pub const fn biosim_scenario_resource_kind_canonical_id(
    kind: BioSimScenarioResourceKind,
) -> &'static str {
    match kind {
        BioSimScenarioResourceKind::Oxygen => "oxygen",
        BioSimScenarioResourceKind::CarbonDioxide => "carbon_dioxide",
        BioSimScenarioResourceKind::Water => "water",
        BioSimScenarioResourceKind::Biomass => "biomass",
        BioSimScenarioResourceKind::Food => "food",
        BioSimScenarioResourceKind::ElectricalEnergy => "electrical_energy",
    }
}

/// Returns the canonical record unit for a scenario-layer resource kind.
///
/// The units are metadata labels only. B2a performs no conversion and adds no
/// rate equation, process model, conservation proof, or physical simulation.
#[must_use]
pub const fn biosim_scenario_resource_kind_unit(kind: BioSimScenarioResourceKind) -> &'static str {
    match kind {
        BioSimScenarioResourceKind::Oxygen
        | BioSimScenarioResourceKind::CarbonDioxide
        | BioSimScenarioResourceKind::Water
        | BioSimScenarioResourceKind::Biomass
        | BioSimScenarioResourceKind::Food => "kg",
        BioSimScenarioResourceKind::ElectricalEnergy => "kWh",
    }
}

pub(crate) fn validate_biosim_scenario_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), BioSimScenarioValidationError> {
    let expected_identifier_format =
        "ASCII lower-snake identifier: first char a-z, then a-z, 0-9, or _, max 64 bytes";

    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return Err(BioSimScenarioValidationError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            expected: expected_identifier_format,
        });
    };

    if value.len() > 64 || !first.is_ascii_lowercase() {
        return Err(BioSimScenarioValidationError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            expected: expected_identifier_format,
        });
    }

    if chars.any(|ch| !(ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')) {
        return Err(BioSimScenarioValidationError::InvalidIdentifier {
            field,
            value: value.to_owned(),
            expected: expected_identifier_format,
        });
    }

    Ok(())
}

fn validate_biosim_scenario_text(
    field: &'static str,
    value: &str,
) -> Result<(), BioSimScenarioValidationError> {
    let expected_text_format = "nonempty display text without ASCII control characters";

    if value.trim().is_empty() {
        return Err(BioSimScenarioValidationError::EmptyLabel { field });
    }

    if value.chars().any(|ch| ch.is_ascii_control()) {
        return Err(BioSimScenarioValidationError::InvalidText {
            field,
            value: value.to_owned(),
            expected: expected_text_format,
        });
    }

    Ok(())
}

fn normalized_nonempty_label(
    field: &'static str,
    value: String,
) -> Result<String, BioSimScenarioValidationError> {
    let normalized = value.trim().to_owned();
    validate_biosim_scenario_text(field, &normalized)?;
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn all_resource_ids_are_stable() {
        let expected = [
            (BioSimScenarioResourceKind::Oxygen, "oxygen"),
            (BioSimScenarioResourceKind::CarbonDioxide, "carbon_dioxide"),
            (BioSimScenarioResourceKind::Water, "water"),
            (BioSimScenarioResourceKind::Biomass, "biomass"),
            (BioSimScenarioResourceKind::Food, "food"),
            (
                BioSimScenarioResourceKind::ElectricalEnergy,
                "electrical_energy",
            ),
        ];

        for (kind, canonical_id) in expected {
            assert_eq!(
                biosim_scenario_resource_kind_canonical_id(kind),
                canonical_id
            );
        }
    }

    #[test]
    fn all_resource_units_are_stable() {
        let expected = [
            (BioSimScenarioResourceKind::Oxygen, "kg"),
            (BioSimScenarioResourceKind::CarbonDioxide, "kg"),
            (BioSimScenarioResourceKind::Water, "kg"),
            (BioSimScenarioResourceKind::Biomass, "kg"),
            (BioSimScenarioResourceKind::Food, "kg"),
            (BioSimScenarioResourceKind::ElectricalEnergy, "kWh"),
        ];

        for (kind, unit) in expected {
            assert_eq!(biosim_scenario_resource_kind_unit(kind), unit);
        }
    }

    #[test]
    fn valid_id_parsing_accepts_lower_snake_identifiers() {
        let compartment_id = BioSimCompartmentId::new("crew_cabin_1").expect("valid ID");

        assert_eq!(compartment_id.as_str(), "crew_cabin_1");
    }

    #[test]
    fn invalid_id_parsing_rejects_unsafe_or_noncanonical_forms() {
        let rejected = [
            "",
            "CrewCabin",
            "crew-cabin",
            "1crew_cabin",
            "crew cabin",
            "crew_cabin!",
            "crew_cabin_é",
        ];

        for raw in rejected {
            assert!(matches!(
                BioSimCompartmentId::new(raw),
                Err(BioSimScenarioValidationError::InvalidIdentifier { .. })
            ));
        }
    }

    #[test]
    fn process_id_validation_matches_compartment_identifier_policy() {
        let one = BioSimProcessId::new("p").expect("one-character process ID");
        assert_eq!(one.as_str(), "p");

        let max_len = "a".repeat(64);
        let max_id = BioSimProcessId::new(max_len.clone()).expect("64-byte process ID");
        assert_eq!(max_id.as_str(), max_len);

        for raw in [
            "",
            "ProcessOne",
            "process-one",
            "process one",
            "process!one",
            "1process",
            "_process",
            "process_é",
            &"a".repeat(65),
        ] {
            assert!(matches!(
                BioSimProcessId::new(raw),
                Err(BioSimScenarioValidationError::InvalidIdentifier { .. })
            ));
        }

        assert_eq!(
            BioSimProcessId::new("process__one_")
                .expect("matching repeated/trailing underscore policy")
                .as_str(),
            "process__one_"
        );
    }

    #[test]
    fn empty_labels_are_rejected_by_domain_constructors() {
        assert!(matches!(
            BioSimScenarioMetadata::new("synthetic_case", " ", "research_required"),
            Err(BioSimScenarioValidationError::EmptyLabel {
                field: "scenario_label"
            })
        ));
        assert!(matches!(
            BioSimCompartment::new("crew_cabin", "\t"),
            Err(BioSimScenarioValidationError::EmptyLabel {
                field: "compartment_label"
            })
        ));
    }

    #[test]
    fn valid_minimal_scenario_construction_is_deterministic() {
        let metadata = BioSimScenarioMetadata::new(
            "synthetic_resource_case",
            "Synthetic resource case",
            "research_required",
        )
        .expect("valid metadata");
        let compartment =
            BioSimCompartment::new("crew_cabin", "Crew cabin").expect("valid compartment");
        let store = BioSimInitialStore::new(
            compartment.id.clone(),
            BioSimScenarioResourceKind::Oxygen,
            1.0,
        );
        let clock = BioSimScenarioClock::new(60.0, 1);

        let scenario = BioSimScenario::new(metadata, vec![compartment], vec![store], clock);

        assert_eq!(scenario.metadata.scenario_id, "synthetic_resource_case");
        assert_eq!(scenario.compartments.len(), 1);
        assert_eq!(scenario.initial_stores.len(), 1);
        assert_eq!(scenario.clock.tick_duration_seconds, 60.0);
        assert_eq!(scenario.clock.tick_count, 1);

        let mut ordered = BTreeMap::new();
        for store in &scenario.initial_stores {
            ordered.insert(
                (
                    store.compartment_id.as_str().to_owned(),
                    store.resource_kind,
                ),
                store.amount,
            );
        }
        assert_eq!(ordered.len(), 1);
        assert_eq!(
            ordered.get(&("crew_cabin".to_owned(), BioSimScenarioResourceKind::Oxygen)),
            Some(&1.0)
        );
    }

    fn valid_b2a_scenario() -> BioSimScenario {
        let metadata = BioSimScenarioMetadata::new(
            "synthetic_resource_case",
            "Synthetic resource case",
            "research_required",
        )
        .expect("valid metadata");
        let compartment =
            BioSimCompartment::new("crew_cabin", "Crew cabin").expect("valid compartment");
        let store = BioSimInitialStore::new(
            compartment.id.clone(),
            BioSimScenarioResourceKind::Oxygen,
            1.0,
        );

        BioSimScenario::new(
            metadata,
            vec![compartment],
            vec![store],
            BioSimScenarioClock::new(60.0, 2),
        )
    }

    fn assert_report_has_error<F>(report: &BioSimScenarioValidationReport, predicate: F)
    where
        F: Fn(&BioSimScenarioValidationError) -> bool,
    {
        assert!(
            report.errors.iter().any(predicate),
            "expected error was not present; report={report:?}"
        );
    }

    #[test]
    fn valid_minimal_scenario_validates() {
        let report = validate_biosim_scenario(&valid_b2a_scenario());

        assert!(report.is_ok(), "unexpected validation errors: {report:?}");
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn zero_compartments_are_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.compartments.clear();

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(error, BioSimScenarioValidationError::NoCompartments)
        });
    }

    #[test]
    fn duplicate_compartment_ids_are_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.compartments.push(
            BioSimCompartment::new("crew_cabin", "Duplicate crew cabin")
                .expect("valid duplicate compartment syntax"),
        );

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::DuplicateCompartmentId {
                    compartment_id,
                } if compartment_id == "crew_cabin"
            )
        });
    }

    #[test]
    fn duplicate_initial_stores_are_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario
            .initial_stores
            .push(scenario.initial_stores[0].clone());

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::DuplicateInitialStore { .. }
            )
        });
    }

    #[test]
    fn unknown_initial_store_compartment_is_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.initial_stores[0].compartment_id =
            BioSimCompartmentId::new("ghost_store").expect("valid ID syntax");

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::UnknownInitialStoreCompartment {
                    compartment_id,
                } if compartment_id == "ghost_store"
            )
        });
    }

    #[test]
    fn invalid_compartment_ids_are_rejected_by_validation() {
        let mut scenario = valid_b2a_scenario();
        scenario.compartments[0].id = BioSimCompartmentId("Crew-Cabin".to_owned());

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidIdentifier {
                    field: "compartment_id",
                    value,
                    ..
                } if value == "Crew-Cabin"
            )
        });
    }

    #[test]
    fn invalid_metadata_and_initial_store_ids_are_rejected_by_validation() {
        let mut scenario = valid_b2a_scenario();
        scenario.metadata.scenario_id = "Synthetic-Case".to_owned();
        scenario.initial_stores[0].compartment_id = BioSimCompartmentId("Ghost-Store".to_owned());

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidIdentifier {
                    field: "scenario_id",
                    value,
                    ..
                } if value == "Synthetic-Case"
            )
        });
        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidIdentifier {
                    field: "initial_store.compartment_id",
                    value,
                    ..
                } if value == "Ghost-Store"
            )
        });
    }

    #[test]
    fn empty_metadata_labels_are_rejected_by_validation() {
        let mut scenario = valid_b2a_scenario();
        scenario.metadata.label = " \n ".to_owned();
        scenario.metadata.validation_status = " \t ".to_owned();

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::EmptyLabel {
                    field: "scenario_label",
                }
            )
        });
        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::EmptyLabel {
                    field: "validation_status",
                }
            )
        });
    }

    #[test]
    fn empty_compartment_labels_are_rejected_by_validation() {
        let mut scenario = valid_b2a_scenario();
        scenario.compartments[0].label = " 	 ".to_owned();

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::EmptyLabel {
                    field: "compartment_label",
                }
            )
        });
    }

    #[test]
    fn non_research_required_status_is_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.metadata.validation_status = "validated".to_owned();

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::NonResearchRequiredStatus { status }
                    if status == "validated"
            )
        });
    }

    #[test]
    fn zero_tick_duration_is_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.clock = BioSimScenarioClock::new(0.0, 1);

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidTickDurationSeconds { value }
                    if *value == 0.0
            )
        });
    }

    #[test]
    fn negative_tick_duration_is_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.clock = BioSimScenarioClock::new(-1.0, 1);

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidTickDurationSeconds { value }
                    if *value == -1.0
            )
        });
    }

    #[test]
    fn nan_and_infinite_tick_durations_are_rejected() {
        for tick_duration_seconds in [f64::NAN, f64::INFINITY, -f64::INFINITY] {
            let mut scenario = valid_b2a_scenario();
            scenario.clock = BioSimScenarioClock::new(tick_duration_seconds, 1);

            let report = validate_biosim_scenario(&scenario);

            assert_report_has_error(&report, |error| {
                matches!(
                    error,
                    BioSimScenarioValidationError::InvalidTickDurationSeconds { .. }
                )
            });
        }
    }

    #[test]
    fn zero_tick_count_is_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.clock = BioSimScenarioClock::new(60.0, 0);

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidTickCount { value }
                    if *value == 0
            )
        });
    }

    #[test]
    fn elapsed_clock_overflow_or_nonfinite_case_is_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.clock = BioSimScenarioClock::new(f64::MAX, 2);

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::ElapsedClockNotFinite {
                    tick_duration_seconds,
                    tick_count,
                } if *tick_duration_seconds == f64::MAX && *tick_count == 2
            )
        });
    }

    #[test]
    fn negative_initial_store_is_rejected() {
        let mut scenario = valid_b2a_scenario();
        scenario.initial_stores[0].amount = -0.25;

        let report = validate_biosim_scenario(&scenario);

        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::NegativeInitialStore { amount, .. }
                    if *amount == -0.25
            )
        });
    }

    #[test]
    fn nan_and_infinite_initial_stores_are_rejected() {
        for amount in [f64::NAN, f64::INFINITY, -f64::INFINITY] {
            let mut scenario = valid_b2a_scenario();
            scenario.initial_stores[0].amount = amount;

            let report = validate_biosim_scenario(&scenario);

            assert_report_has_error(&report, |error| {
                matches!(
                    error,
                    BioSimScenarioValidationError::NonfiniteInitialStore { .. }
                )
            });
        }
    }

    #[test]
    fn compartment_id_boundaries_and_underscore_policy_are_deterministic() {
        let sixty_four = format!("a{}", "b".repeat(63));
        assert_eq!(sixty_four.len(), 64);
        let accepted = BioSimCompartmentId::new(sixty_four.clone()).expect("64-byte ID accepted");
        assert_eq!(accepted.as_str(), sixty_four);

        let sixty_five = format!("a{}", "b".repeat(64));
        assert_eq!(sixty_five.len(), 65);
        assert!(matches!(
            BioSimCompartmentId::new(sixty_five),
            Err(BioSimScenarioValidationError::InvalidIdentifier { .. })
        ));

        assert_eq!(BioSimCompartmentId::new("a").unwrap().as_str(), "a");
        assert!(matches!(
            BioSimCompartmentId::new("_crew"),
            Err(BioSimScenarioValidationError::InvalidIdentifier { .. })
        ));
        assert_eq!(
            BioSimCompartmentId::new("crew_")
                .expect("trailing underscore is documented as accepted")
                .as_str(),
            "crew_"
        );
        assert_eq!(
            BioSimCompartmentId::new("crew__cabin")
                .expect("repeated underscores are documented as accepted")
                .as_str(),
            "crew__cabin"
        );
        assert!(matches!(
            BioSimCompartmentId::new("crew_é"),
            Err(BioSimScenarioValidationError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn constructors_trim_display_labels_and_status_text() {
        let metadata = BioSimScenarioMetadata::new(
            "synthetic_case",
            "  Synthetic label  ",
            "  research_required  ",
        )
        .expect("trimmed metadata accepted");
        assert_eq!(metadata.label, "Synthetic label");
        assert_eq!(metadata.validation_status, "research_required");

        let compartment = BioSimCompartment::new("crew_cabin", "  Crew cabin  ")
            .expect("trimmed compartment label accepted");
        assert_eq!(compartment.label, "Crew cabin");
    }

    #[test]
    fn control_characters_in_labels_or_status_are_rejected() {
        assert!(matches!(
            BioSimScenarioMetadata::new("synthetic_case", "Crew\nCabin", "research_required"),
            Err(BioSimScenarioValidationError::InvalidText {
                field: "scenario_label",
                ..
            })
        ));
        assert!(matches!(
            BioSimScenarioMetadata::new("synthetic_case", "Synthetic", "research\trequired"),
            Err(BioSimScenarioValidationError::InvalidText {
                field: "validation_status",
                ..
            })
        ));
        assert!(matches!(
            BioSimCompartment::new("crew_cabin", "Crew\rCabin"),
            Err(BioSimScenarioValidationError::InvalidText {
                field: "compartment_label",
                ..
            })
        ));

        let mut scenario = valid_b2a_scenario();
        scenario.metadata.label = "Crew\nCabin".to_owned();
        scenario.metadata.validation_status = "research\trequired".to_owned();
        scenario.compartments[0].label = "Crew\rCabin".to_owned();
        let report = validate_biosim_scenario(&scenario);
        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidText {
                    field: "scenario_label",
                    ..
                }
            )
        });
        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidText {
                    field: "validation_status",
                    ..
                }
            )
        });
        assert_report_has_error(&report, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidText {
                    field: "compartment_label",
                    ..
                }
            )
        });
    }

    #[test]
    fn zero_and_negative_zero_initial_stores_are_record_level_zero_amounts() {
        for amount in [0.0, -0.0] {
            let mut scenario = valid_b2a_scenario();
            scenario.initial_stores[0].amount = amount;
            let report = validate_biosim_scenario(&scenario);
            assert!(report.is_ok(), "zero-like amount rejected: {report:?}");
        }
    }

    #[test]
    fn validation_error_order_is_repeatable_and_collects_multiple_failures() {
        let mut scenario = valid_b2a_scenario();
        scenario.metadata.scenario_id = "Bad-Scenario".to_owned();
        scenario.metadata.label = "".to_owned();
        scenario.metadata.validation_status = "validated".to_owned();
        scenario.compartments.push(
            BioSimCompartment::new("crew_cabin", "Duplicate crew cabin")
                .expect("valid duplicate compartment syntax"),
        );
        scenario
            .initial_stores
            .push(scenario.initial_stores[0].clone());
        scenario.initial_stores[0].amount = -1.0;
        scenario.clock = BioSimScenarioClock::new(0.0, 0);

        let first = validate_biosim_scenario(&scenario);
        let second = validate_biosim_scenario(&scenario);
        assert_eq!(first.errors, second.errors);
        assert!(
            first.errors.len() >= 7,
            "expected multiple failures: {first:?}"
        );
        assert!(matches!(
            first.errors[0],
            BioSimScenarioValidationError::InvalidIdentifier {
                field: "scenario_id",
                ..
            }
        ));
        assert!(matches!(
            first.errors[1],
            BioSimScenarioValidationError::EmptyLabel {
                field: "scenario_label"
            }
        ));
        assert!(matches!(
            first.errors[2],
            BioSimScenarioValidationError::NonResearchRequiredStatus { .. }
        ));
        assert_report_has_error(&first, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::DuplicateCompartmentId { .. }
            )
        });
        assert_report_has_error(&first, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::DuplicateInitialStore { .. }
            )
        });
        assert_report_has_error(&first, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidTickDurationSeconds { .. }
            )
        });
        assert_report_has_error(&first, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::InvalidTickCount { .. }
            )
        });
        assert_report_has_error(&first, |error| {
            matches!(
                error,
                BioSimScenarioValidationError::NegativeInitialStore { .. }
            )
        });
    }
}
