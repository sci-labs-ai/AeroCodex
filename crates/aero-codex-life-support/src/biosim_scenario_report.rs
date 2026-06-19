use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::biosim_process::{
    validate_biosim_scenario_replay_integrity, BioSimScenarioReplay,
    BioSimScenarioReplayIntegrityError, BioSimScenarioReplayIntegrityReport, BioSimTickIntentKind,
};
use crate::biosim_scenario::{
    biosim_scenario_resource_kind_canonical_id, biosim_scenario_resource_kind_unit,
    BioSimScenarioResourceKind,
};

/// Returns the B2c validation-card codex ID for replay-ledger report metadata.
#[must_use]
pub const fn biosim_plus_scenario_engine_codex_id() -> &'static str {
    "life_support.biosim_plus.clean_room_scenario_engine"
}

/// Returns the B2c source-registry seed ID for clean-room replay-ledger reporting.
#[must_use]
pub const fn biosim_plus_scenario_engine_source_id() -> &'static str {
    "source.life_support.biosim_plus.clean_room_scenario_engine.research_required"
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LedgerKey {
    compartment_id: String,
    resource_kind: BioSimScenarioResourceKind,
}

#[derive(Debug, Clone, PartialEq)]
struct LedgerAccumulator {
    unit: &'static str,
    initial_amount: Option<f64>,
    source_committed_amount: f64,
    sink_committed_amount: f64,
    clamp_amount: f64,
    final_amount: Option<f64>,
    event_count: usize,
}

impl LedgerAccumulator {
    const fn new(resource_kind: BioSimScenarioResourceKind) -> Self {
        Self {
            unit: biosim_scenario_resource_kind_unit(resource_kind),
            initial_amount: None,
            source_committed_amount: 0.0,
            sink_committed_amount: 0.0,
            clamp_amount: 0.0,
            final_amount: None,
            event_count: 0,
        }
    }
}

/// Deterministic per-compartment/resource ledger row derived from B2b-2 events.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenarioLedgerRow {
    compartment_id: String,
    resource_id: &'static str,
    unit: &'static str,
    initial_amount: f64,
    source_committed_amount: f64,
    sink_committed_amount: f64,
    clamp_amount: f64,
    final_amount: f64,
    event_count: usize,
}

impl BioSimScenarioLedgerRow {
    /// Returns the scenario compartment ID for this row.
    #[must_use]
    pub fn compartment_id(&self) -> &str {
        &self.compartment_id
    }

    /// Returns the scenario-local resource ID for this row.
    #[must_use]
    pub const fn resource_id(&self) -> &'static str {
        self.resource_id
    }

    /// Returns the canonical unit label for this row.
    #[must_use]
    pub const fn unit(&self) -> &'static str {
        self.unit
    }

    /// Returns the inferred initial amount for this row.
    #[must_use]
    pub const fn initial_amount(&self) -> f64 {
        self.initial_amount
    }

    /// Returns committed source/produce amount for this row.
    #[must_use]
    pub const fn source_committed_amount(&self) -> f64 {
        self.source_committed_amount
    }

    /// Returns committed sink/consume amount for this row.
    #[must_use]
    pub const fn sink_committed_amount(&self) -> f64 {
        self.sink_committed_amount
    }

    /// Returns explicitly clamped requested amount for this row.
    #[must_use]
    pub const fn clamp_amount(&self) -> f64 {
        self.clamp_amount
    }

    /// Returns the final amount recorded by B2b-2 replay.
    #[must_use]
    pub const fn final_amount(&self) -> f64 {
        self.final_amount
    }

    /// Returns committed event count for this row.
    #[must_use]
    pub const fn event_count(&self) -> usize {
        self.event_count
    }
}

/// Deterministic B2c replay-ledger report derived from B2b-2 records.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenarioLedgerReport {
    scenario_id: String,
    integrity: BioSimScenarioReplayIntegrityReport,
    rows: Vec<BioSimScenarioLedgerRow>,
}

impl BioSimScenarioLedgerReport {
    /// Returns the scenario ID from the replay.
    #[must_use]
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    /// Returns the fail-closed replay-integrity summary.
    #[must_use]
    pub const fn integrity(&self) -> &BioSimScenarioReplayIntegrityReport {
        &self.integrity
    }

    /// Returns ledger rows in deterministic compartment/resource order.
    #[must_use]
    pub fn rows(&self) -> &[BioSimScenarioLedgerRow] {
        &self.rows
    }

    /// Returns true when the ledger was built after all B2c self-consistency checks passed.
    #[must_use]
    pub const fn is_self_consistent(&self) -> bool {
        true
    }
}

/// B2c report/ledger failures.
#[derive(Debug, Clone, PartialEq)]
pub enum BioSimScenarioReportError {
    ReplayIntegrity(BioSimScenarioReplayIntegrityError),
    FinalCellUnitMismatch {
        compartment_id: String,
        resource_id: &'static str,
        expected_unit: &'static str,
        actual_unit: &'static str,
    },
    NonfiniteLedgerAmount {
        compartment_id: String,
        resource_id: &'static str,
        amount: f64,
    },
    LedgerResidualMismatch {
        compartment_id: String,
        resource_id: &'static str,
        expected_final_amount: f64,
        actual_final_amount: f64,
    },
}

impl From<BioSimScenarioReplayIntegrityError> for BioSimScenarioReportError {
    fn from(value: BioSimScenarioReplayIntegrityError) -> Self {
        Self::ReplayIntegrity(value)
    }
}

/// Builds a deterministic B2c ledger from B2b-2 committed replay events.
///
/// The ledger equation is:
/// `final_amount = initial_amount + source_committed_amount - sink_committed_amount`.
/// Clamp amounts are reported separately as requested-but-uncommitted event amount.
pub fn build_biosim_scenario_resource_ledger(
    replay: &BioSimScenarioReplay,
) -> Result<BioSimScenarioLedgerReport, BioSimScenarioReportError> {
    let integrity = validate_biosim_scenario_replay_integrity(replay)?;
    let mut accumulators = BTreeMap::<LedgerKey, LedgerAccumulator>::new();

    for cell in replay.final_cells() {
        let key = LedgerKey {
            compartment_id: cell.compartment_id().as_str().to_owned(),
            resource_kind: cell.resource_kind(),
        };
        let expected_unit = biosim_scenario_resource_kind_unit(cell.resource_kind());
        if cell.unit() != expected_unit {
            return Err(BioSimScenarioReportError::FinalCellUnitMismatch {
                compartment_id: key.compartment_id,
                resource_id: biosim_scenario_resource_kind_canonical_id(cell.resource_kind()),
                expected_unit,
                actual_unit: cell.unit(),
            });
        }
        validate_ledger_amount(&key, cell.amount())?;
        accumulators
            .entry(key)
            .or_insert_with(|| LedgerAccumulator::new(cell.resource_kind()))
            .final_amount = Some(cell.amount());
    }

    for event in replay.replay_events() {
        let key = LedgerKey {
            compartment_id: event.compartment_id().as_str().to_owned(),
            resource_kind: event.resource_kind(),
        };
        let entry = accumulators
            .entry(key.clone())
            .or_insert_with(|| LedgerAccumulator::new(event.resource_kind()));
        if entry.initial_amount.is_none() {
            entry.initial_amount = Some(event.before_amount());
        }
        match event.intent_kind() {
            BioSimTickIntentKind::Produce | BioSimTickIntentKind::Source => {
                entry.source_committed_amount += event.committed_amount();
            }
            BioSimTickIntentKind::Consume | BioSimTickIntentKind::Sink => {
                entry.sink_committed_amount += event.committed_amount();
            }
        }
        entry.clamp_amount += event.clamp_amount();
        entry.event_count = entry.event_count.checked_add(1).ok_or(
            BioSimScenarioReportError::NonfiniteLedgerAmount {
                compartment_id: key.compartment_id,
                resource_id: biosim_scenario_resource_kind_canonical_id(key.resource_kind),
                amount: f64::INFINITY,
            },
        )?;
    }

    let mut rows = Vec::new();
    for (key, entry) in accumulators {
        let final_amount = entry.final_amount.unwrap_or(0.0);
        let initial_amount = entry.initial_amount.unwrap_or(final_amount);
        let expected_final_amount =
            initial_amount + entry.source_committed_amount - entry.sink_committed_amount;
        if !b2c_amounts_match(expected_final_amount, final_amount) {
            return Err(BioSimScenarioReportError::LedgerResidualMismatch {
                compartment_id: key.compartment_id,
                resource_id: biosim_scenario_resource_kind_canonical_id(key.resource_kind),
                expected_final_amount,
                actual_final_amount: final_amount,
            });
        }
        for amount in [
            initial_amount,
            entry.source_committed_amount,
            entry.sink_committed_amount,
            entry.clamp_amount,
            final_amount,
        ] {
            validate_ledger_amount(&key, amount)?;
        }
        rows.push(BioSimScenarioLedgerRow {
            compartment_id: key.compartment_id,
            resource_id: biosim_scenario_resource_kind_canonical_id(key.resource_kind),
            unit: entry.unit,
            initial_amount,
            source_committed_amount: entry.source_committed_amount,
            sink_committed_amount: entry.sink_committed_amount,
            clamp_amount: entry.clamp_amount,
            final_amount,
            event_count: entry.event_count,
        });
    }

    Ok(BioSimScenarioLedgerReport {
        scenario_id: replay.scenario_id().to_owned(),
        integrity,
        rows,
    })
}

/// Formats a deterministic path-safe friend-test report for a B2c ledger report.
#[must_use]
pub fn format_biosim_scenario_friend_test_report(report: &BioSimScenarioLedgerReport) -> String {
    let mut text = String::new();
    let integrity = report.integrity();
    writeln!(
        &mut text,
        "AeroCodex BioSim-plus synthetic scenario B2c report"
    )
    .unwrap();
    writeln!(&mut text, "status: research_required").unwrap();
    writeln!(
        &mut text,
        "codex_id: {}",
        biosim_plus_scenario_engine_codex_id()
    )
    .unwrap();
    writeln!(&mut text, "scenario_id: {}", report.scenario_id()).unwrap();
    writeln!(&mut text, "replay_integrity: pass").unwrap();
    writeln!(&mut text, "ledger_self_consistency: pass").unwrap();
    writeln!(&mut text, "tick_count: {}", integrity.tick_count()).unwrap();
    writeln!(&mut text, "event_count: {}", integrity.event_count()).unwrap();
    writeln!(
        &mut text,
        "final_cell_count: {}",
        integrity.final_cell_count()
    )
    .unwrap();
    writeln!(
        &mut text,
        "total_committed_amount: {:.6}",
        integrity.total_committed_amount()
    )
    .unwrap();
    writeln!(
        &mut text,
        "total_clamp_amount: {:.6}",
        integrity.total_clamp_amount()
    )
    .unwrap();
    writeln!(
        &mut text,
        "ledger_formula: final_amount = initial_amount + source_committed_amount - sink_committed_amount"
    )
    .unwrap();
    writeln!(
        &mut text,
        "clamp_accounting: requested amounts not committed by B2b-2 replay are reported separately"
    )
    .unwrap();
    writeln!(
        &mut text,
        "rows: compartment_id resource_id unit initial source sink clamp final events"
    )
    .unwrap();
    for row in report.rows() {
        writeln!(
            &mut text,
            "row: {} {} {} {:.6} {:.6} {:.6} {:.6} {:.6} {}",
            row.compartment_id(),
            row.resource_id(),
            row.unit(),
            row.initial_amount(),
            row.source_committed_amount(),
            row.sink_committed_amount(),
            row.clamp_amount(),
            row.final_amount(),
            row.event_count()
        )
        .unwrap();
    }
    writeln!(
        &mut text,
        "non_claims: research metadata only; no external BioSim parity, biological-dynamics, safety, medical, operational, certification, or regulated-use claim"
    )
    .unwrap();
    text
}

fn validate_ledger_amount(key: &LedgerKey, amount: f64) -> Result<(), BioSimScenarioReportError> {
    if amount.is_finite() && amount >= 0.0 {
        Ok(())
    } else {
        Err(BioSimScenarioReportError::NonfiniteLedgerAmount {
            compartment_id: key.compartment_id.clone(),
            resource_id: biosim_scenario_resource_kind_canonical_id(key.resource_kind),
            amount,
        })
    }
}

fn b2c_amounts_match(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1.0e-9 * scale
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biosim_process::{run_biosim_scenario, BioSimProcess};
    use crate::biosim_scenario::{
        BioSimCompartment, BioSimInitialStore, BioSimScenario, BioSimScenarioClock,
        BioSimScenarioMetadata,
    };
    use crate::biosim_scenario::{BioSimCompartmentId, BioSimProcessId};

    fn compartment(raw: &str) -> BioSimCompartmentId {
        BioSimCompartmentId::new(raw).expect("valid compartment")
    }

    fn process_id(raw: &str) -> BioSimProcessId {
        BioSimProcessId::new(raw).expect("valid process")
    }

    fn synthetic_scenario() -> BioSimScenario {
        let crew = compartment("crew_cabin");
        let buffer = compartment("buffer_store");
        BioSimScenario::new(
            BioSimScenarioMetadata::new(
                "synthetic_b2c_example",
                "Synthetic B2c example",
                "research_required",
            )
            .expect("metadata"),
            vec![
                BioSimCompartment::new("crew_cabin", "Crew cabin").expect("crew"),
                BioSimCompartment::new("buffer_store", "Buffer store").expect("buffer"),
            ],
            vec![
                BioSimInitialStore::new(crew, BioSimScenarioResourceKind::Oxygen, 1.0),
                BioSimInitialStore::new(buffer, BioSimScenarioResourceKind::Oxygen, 5.0),
            ],
            BioSimScenarioClock::new(10.0, 2),
        )
    }

    fn synthetic_processes() -> Vec<BioSimProcess> {
        vec![BioSimProcess::transfer(
            process_id("oxygen_transfer"),
            compartment("buffer_store"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            0.1,
        )
        .expect("transfer")]
    }

    #[test]
    fn ledger_uses_committed_events_and_final_cells() {
        let scenario = synthetic_scenario();
        let replay = run_biosim_scenario(&scenario, &synthetic_processes()).expect("replay");
        let report = build_biosim_scenario_resource_ledger(&replay).expect("ledger");
        assert!(report.is_self_consistent());
        assert_eq!(report.scenario_id(), "synthetic_b2c_example");
        assert_eq!(report.rows().len(), 2);
        let crew = report
            .rows()
            .iter()
            .find(|row| row.compartment_id() == "crew_cabin")
            .expect("crew row");
        assert_eq!(crew.resource_id(), "oxygen");
        assert!((crew.final_amount() - 3.0).abs() < 1.0e-12);
        assert_eq!(crew.clamp_amount(), 0.0);
    }

    #[test]
    fn formatted_report_is_deterministic_and_path_safe() {
        let scenario = synthetic_scenario();
        let replay = run_biosim_scenario(&scenario, &synthetic_processes()).expect("replay");
        let report = build_biosim_scenario_resource_ledger(&replay).expect("ledger");
        let text = format_biosim_scenario_friend_test_report(&report);
        assert!(text.contains("replay_integrity: pass"));
        assert!(text.contains("ledger_self_consistency: pass"));
        assert!(text.contains("row: crew_cabin oxygen kg"));
        assert!(!text.contains("/mnt/"));
        assert!(!text.contains("deploy_work"));
        assert!(!text.contains("C:\\"));
    }
}
