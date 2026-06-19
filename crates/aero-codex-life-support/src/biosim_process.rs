use std::collections::{BTreeMap, BTreeSet};

use crate::biosim_scenario::{
    biosim_scenario_resource_kind_canonical_id, biosim_scenario_resource_kind_unit,
    validate_biosim_scenario, BioSimCompartmentId, BioSimProcessId, BioSimScenario,
    BioSimScenarioClock, BioSimScenarioResourceKind, BioSimScenarioValidationError,
};
#[cfg(test)]
use crate::biosim_scenario::{BioSimCompartment, BioSimInitialStore, BioSimScenarioMetadata};

const fn max_biosim_plan_compartments() -> usize {
    1024
}

const fn max_biosim_plan_processes() -> usize {
    1024
}

const fn max_biosim_flows_per_process() -> usize {
    64
}

const fn max_biosim_intents_per_plan() -> usize {
    65_536
}

/// Synthetic process shape for one-tick clean-room intent planning.
///
/// These process kinds are neutral accounting abstractions. They are not a
/// biological dynamics model, not a BioSim Java compatibility layer, not habitat
/// control logic, and not an operational life-support controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BioSimProcessKind {
    /// Add a resource to one compartment with explicit source-accounting intent.
    Source,
    /// Remove a resource from one compartment with explicit sink-accounting intent.
    Sink,
    /// Move one resource between two compartments with matched in/out rates.
    Transfer,
    /// Consume one or more inputs and produce one or more outputs in one compartment.
    Transform,
}

/// Direction of a flow relative to a synthetic process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BioSimFlowDirection {
    Input,
    Output,
}

/// One positive finite per-second resource flow attached to a synthetic process.
///
/// Rates are in the canonical unit returned by
/// `biosim_scenario_resource_kind_unit(resource_kind)` per second. Units are
/// metadata labels only; B2b-1 performs no conversion and makes no conservation,
/// biological-fidelity, or operational claim.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimFlow {
    compartment_id: BioSimCompartmentId,
    resource_kind: BioSimScenarioResourceKind,
    direction: BioSimFlowDirection,
    rate_per_second: f64,
}

impl BioSimFlow {
    /// Builds an input flow that withdraws from the named compartment.
    pub fn input(
        compartment_id: BioSimCompartmentId,
        resource_kind: BioSimScenarioResourceKind,
        rate_per_second: f64,
    ) -> Result<Self, BioSimProcessValidationError> {
        Self::new(
            compartment_id,
            resource_kind,
            BioSimFlowDirection::Input,
            rate_per_second,
        )
    }

    /// Builds an output flow that adds to the named compartment.
    pub fn output(
        compartment_id: BioSimCompartmentId,
        resource_kind: BioSimScenarioResourceKind,
        rate_per_second: f64,
    ) -> Result<Self, BioSimProcessValidationError> {
        Self::new(
            compartment_id,
            resource_kind,
            BioSimFlowDirection::Output,
            rate_per_second,
        )
    }

    fn new(
        compartment_id: BioSimCompartmentId,
        resource_kind: BioSimScenarioResourceKind,
        direction: BioSimFlowDirection,
        rate_per_second: f64,
    ) -> Result<Self, BioSimProcessValidationError> {
        ensure_positive_finite_rate(rate_per_second)?;
        Ok(Self {
            compartment_id,
            resource_kind,
            direction,
            rate_per_second,
        })
    }

    /// Returns the validated compartment ID named by this flow.
    #[must_use]
    pub fn compartment_id(&self) -> &BioSimCompartmentId {
        &self.compartment_id
    }

    /// Returns the scenario-local resource kind named by this flow.
    #[must_use]
    pub const fn resource_kind(&self) -> BioSimScenarioResourceKind {
        self.resource_kind
    }

    /// Returns whether the flow is an input or output relative to the process.
    #[must_use]
    pub const fn direction(&self) -> BioSimFlowDirection {
        self.direction
    }

    /// Returns the positive finite per-second rate in the resource canonical unit.
    #[must_use]
    pub const fn rate_per_second(&self) -> f64 {
        self.rate_per_second
    }
}

/// One validated synthetic process definition.
///
/// Public constructors enforce process-kind shape rules. The planner revalidates
/// before producing intents so internally constructed invalid processes cannot
/// bypass B2b-1 shape validation.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimProcess {
    id: BioSimProcessId,
    kind: BioSimProcessKind,
    flows: Vec<BioSimFlow>,
}

impl BioSimProcess {
    /// Builds an exogenous source process: exactly one output flow.
    pub fn source(
        id: BioSimProcessId,
        compartment_id: BioSimCompartmentId,
        resource_kind: BioSimScenarioResourceKind,
        rate_per_second: f64,
    ) -> Result<Self, BioSimProcessValidationError> {
        Self::new_validated(
            id,
            BioSimProcessKind::Source,
            vec![BioSimFlow::output(
                compartment_id,
                resource_kind,
                rate_per_second,
            )?],
        )
    }

    /// Builds an exogenous sink process: exactly one input flow.
    pub fn sink(
        id: BioSimProcessId,
        compartment_id: BioSimCompartmentId,
        resource_kind: BioSimScenarioResourceKind,
        rate_per_second: f64,
    ) -> Result<Self, BioSimProcessValidationError> {
        Self::new_validated(
            id,
            BioSimProcessKind::Sink,
            vec![BioSimFlow::input(
                compartment_id,
                resource_kind,
                rate_per_second,
            )?],
        )
    }

    /// Builds an internal transfer with one input and one matched output flow.
    pub fn transfer(
        id: BioSimProcessId,
        source_compartment_id: BioSimCompartmentId,
        sink_compartment_id: BioSimCompartmentId,
        resource_kind: BioSimScenarioResourceKind,
        rate_per_second: f64,
    ) -> Result<Self, BioSimProcessValidationError> {
        Self::new_validated(
            id,
            BioSimProcessKind::Transfer,
            vec![
                BioSimFlow::input(source_compartment_id, resource_kind, rate_per_second)?,
                BioSimFlow::output(sink_compartment_id, resource_kind, rate_per_second)?,
            ],
        )
    }

    /// Builds a same-compartment transform from input and output flow arguments.
    ///
    /// Inputs must be input flows and outputs must be output flows. This is a
    /// synthetic planning abstraction, not a stoichiometric, biological,
    /// chemical, conservation, or efficiency model.
    pub fn transform(
        id: BioSimProcessId,
        inputs: Vec<BioSimFlow>,
        outputs: Vec<BioSimFlow>,
    ) -> Result<Self, BioSimProcessValidationError> {
        for flow in &inputs {
            if flow.direction != BioSimFlowDirection::Input {
                return Err(
                    BioSimProcessValidationError::TransformArgumentDirectionMismatch {
                        process_id: id.as_str().to_owned(),
                        expected: BioSimFlowDirection::Input,
                        actual: flow.direction,
                    },
                );
            }
        }
        for flow in &outputs {
            if flow.direction != BioSimFlowDirection::Output {
                return Err(
                    BioSimProcessValidationError::TransformArgumentDirectionMismatch {
                        process_id: id.as_str().to_owned(),
                        expected: BioSimFlowDirection::Output,
                        actual: flow.direction,
                    },
                );
            }
        }
        let total = inputs
            .len()
            .checked_add(outputs.len())
            .ok_or(BioSimProcessValidationError::IntentCountOverflow)?;
        let mut flows = Vec::new();
        flows.try_reserve_exact(total).map_err(|_| {
            BioSimProcessValidationError::IntentAllocationFailed {
                requested: total,
                max: max_biosim_intents_per_plan(),
            }
        })?;
        flows.extend(inputs);
        flows.extend(outputs);
        Self::new_validated(id, BioSimProcessKind::Transform, flows)
    }

    fn new_validated(
        id: BioSimProcessId,
        kind: BioSimProcessKind,
        flows: Vec<BioSimFlow>,
    ) -> Result<Self, BioSimProcessValidationError> {
        let mut process = Self { id, kind, flows };
        validate_process_shape(&process)?;
        canonicalize_process_flows(&mut process);
        Ok(process)
    }

    /// Returns this process's stable validated ID.
    #[must_use]
    pub fn id(&self) -> &BioSimProcessId {
        &self.id
    }

    /// Returns this process's shape kind.
    #[must_use]
    pub const fn kind(&self) -> BioSimProcessKind {
        self.kind
    }

    /// Returns validated flow definitions in canonical deterministic order.
    #[must_use]
    pub fn flows(&self) -> &[BioSimFlow] {
        &self.flows
    }
}

/// Planned one-tick intent semantics for B2b-1.
///
/// These are not replay commits, not ledger rows, and not adapter output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BioSimTickIntentKind {
    Consume,
    Produce,
    Source,
    Sink,
}

/// One deterministic per-tick intent produced by B2b-1 process planning.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimTickIntent {
    tick_index: u64,
    sequence_index: usize,
    process_id: BioSimProcessId,
    kind: BioSimTickIntentKind,
    compartment_id: BioSimCompartmentId,
    resource_kind: BioSimScenarioResourceKind,
    unit: &'static str,
    amount: f64,
}

impl BioSimTickIntent {
    /// Returns the caller-supplied tick index.
    #[must_use]
    pub const fn tick_index(&self) -> u64 {
        self.tick_index
    }

    /// Returns the zero-based deterministic sequence index inside the plan.
    #[must_use]
    pub const fn sequence_index(&self) -> usize {
        self.sequence_index
    }

    /// Returns the process ID that produced this intent.
    #[must_use]
    pub fn process_id(&self) -> &BioSimProcessId {
        &self.process_id
    }

    /// Returns the one-tick intent kind.
    #[must_use]
    pub const fn kind(&self) -> BioSimTickIntentKind {
        self.kind
    }

    /// Returns the compartment named by this intent.
    #[must_use]
    pub fn compartment_id(&self) -> &BioSimCompartmentId {
        &self.compartment_id
    }

    /// Returns the scenario-local resource kind named by this intent.
    #[must_use]
    pub const fn resource_kind(&self) -> BioSimScenarioResourceKind {
        self.resource_kind
    }

    /// Returns the B2a canonical record unit label for the resource kind.
    #[must_use]
    pub const fn unit(&self) -> &'static str {
        self.unit
    }

    /// Returns the positive finite amount planned for one tick.
    #[must_use]
    pub const fn amount(&self) -> f64 {
        self.amount
    }
}

/// Deterministic plan of process intents for one tick.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimTickIntentPlan {
    tick_index: u64,
    tick_duration_seconds: f64,
    process_count: usize,
    intents: Vec<BioSimTickIntent>,
}

impl BioSimTickIntentPlan {
    /// Returns the caller-supplied tick index.
    #[must_use]
    pub const fn tick_index(&self) -> u64 {
        self.tick_index
    }

    /// Returns the positive finite tick duration used for amount planning.
    #[must_use]
    pub const fn tick_duration_seconds(&self) -> f64 {
        self.tick_duration_seconds
    }

    /// Returns the number of process definitions considered by the planner.
    #[must_use]
    pub const fn process_count(&self) -> usize {
        self.process_count
    }

    /// Returns planned intents in deterministic sequence order.
    #[must_use]
    pub fn intents(&self) -> &[BioSimTickIntent] {
        &self.intents
    }
}

/// Deterministic validation errors for B2b-1 process construction and planning.
#[derive(Debug, Clone, PartialEq)]
pub enum BioSimProcessValidationError {
    InvalidFlowRate {
        value: f64,
        expected: &'static str,
    },
    ProcessWithNoFlows {
        process_id: String,
    },
    SourceFlowCount {
        process_id: String,
        actual: usize,
    },
    SourceDirectionMismatch {
        process_id: String,
        actual: BioSimFlowDirection,
    },
    SinkFlowCount {
        process_id: String,
        actual: usize,
    },
    SinkDirectionMismatch {
        process_id: String,
        actual: BioSimFlowDirection,
    },
    TransferFlowCount {
        process_id: String,
        actual: usize,
    },
    TransferDirectionMismatch {
        process_id: String,
    },
    TransferSameCompartment {
        process_id: String,
        compartment_id: String,
    },
    TransferResourceMismatch {
        process_id: String,
    },
    TransferRateMismatch {
        process_id: String,
    },
    TransformArgumentDirectionMismatch {
        process_id: String,
        expected: BioSimFlowDirection,
        actual: BioSimFlowDirection,
    },
    TransformMissingInput {
        process_id: String,
    },
    TransformMissingOutput {
        process_id: String,
    },
    TransformCrossCompartment {
        process_id: String,
    },
    DuplicateFlowCell {
        process_id: String,
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        direction: BioSimFlowDirection,
    },
    TransformSameCellResource {
        process_id: String,
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
    },
    InvalidTickDurationSeconds {
        value: f64,
    },
    InvalidTickCount {
        value: u64,
    },
    TickIndexOutOfRange {
        tick_index: u64,
        tick_count: u64,
    },
    TooManyCompartments {
        actual: usize,
        max: usize,
    },
    DuplicateKnownCompartmentId {
        compartment_id: String,
    },
    TooManyProcesses {
        actual: usize,
        max: usize,
    },
    DuplicateProcessId {
        process_id: String,
    },
    TooManyFlows {
        process_id: String,
        actual: usize,
        max: usize,
    },
    IntentCountOverflow,
    TooManyIntents {
        actual: usize,
        max: usize,
    },
    IntentAllocationFailed {
        requested: usize,
        max: usize,
    },
    UnknownFlowCompartment {
        process_id: String,
        compartment_id: String,
    },
    TickAmountNotFinite {
        process_id: String,
        flow_index: usize,
        rate_per_second: f64,
        tick_duration_seconds: f64,
    },
    TickAmountNotPositive {
        process_id: String,
        flow_index: usize,
        amount: f64,
    },
}

/// Plans deterministic one-tick process intents.
///
/// This B2b-1 planner validates process shapes and references, then computes
/// `rate_per_second * tick_duration_seconds` for each flow. It does not validate
/// scenario metadata, initial stores, full scenario status, replay state,
/// resource availability, conservation, ledger residuals, or physical/biological
/// correctness. B2b-2 must validate the complete B2a scenario before invoking
/// this planner.
pub fn plan_biosim_tick_intents(
    processes: &[BioSimProcess],
    known_compartment_ids: &[BioSimCompartmentId],
    clock: BioSimScenarioClock,
    tick_index: u64,
) -> Result<BioSimTickIntentPlan, BioSimProcessValidationError> {
    validate_clock_for_intent_planning(clock, tick_index)?;
    validate_known_compartment_limit(known_compartment_ids.len())?;
    let known_compartments = collect_known_compartments(known_compartment_ids)?;
    validate_process_limit(processes.len())?;

    let ordered_processes = canonical_process_order(processes)?;
    for process in &ordered_processes {
        validate_process_shape(process)?;
    }
    for process in &ordered_processes {
        validate_flow_compartments(process, &known_compartments)?;
    }

    let total_intents =
        checked_total_intent_count(ordered_processes.iter().map(|process| process.flows.len()))?;
    let mut intents = Vec::new();
    reserve_intent_capacity(&mut intents, total_intents)?;

    for process in &ordered_processes {
        for (flow_index, flow) in process.flows.iter().enumerate() {
            let amount = flow.rate_per_second * clock.tick_duration_seconds;
            if !amount.is_finite() {
                return Err(BioSimProcessValidationError::TickAmountNotFinite {
                    process_id: process.id.as_str().to_owned(),
                    flow_index,
                    rate_per_second: flow.rate_per_second,
                    tick_duration_seconds: clock.tick_duration_seconds,
                });
            }
            if amount <= 0.0 {
                return Err(BioSimProcessValidationError::TickAmountNotPositive {
                    process_id: process.id.as_str().to_owned(),
                    flow_index,
                    amount,
                });
            }
            let sequence_index = intents.len();
            intents.push(intent_from_flow(
                process,
                flow,
                tick_index,
                sequence_index,
                amount,
            ));
        }
    }

    Ok(BioSimTickIntentPlan {
        tick_index,
        tick_duration_seconds: clock.tick_duration_seconds,
        process_count: ordered_processes.len(),
        intents,
    })
}

fn validate_clock_for_intent_planning(
    clock: BioSimScenarioClock,
    tick_index: u64,
) -> Result<(), BioSimProcessValidationError> {
    if !clock.tick_duration_seconds.is_finite() || clock.tick_duration_seconds <= 0.0 {
        return Err(BioSimProcessValidationError::InvalidTickDurationSeconds {
            value: clock.tick_duration_seconds,
        });
    }
    if clock.tick_count == 0 {
        return Err(BioSimProcessValidationError::InvalidTickCount {
            value: clock.tick_count,
        });
    }
    if tick_index >= clock.tick_count {
        return Err(BioSimProcessValidationError::TickIndexOutOfRange {
            tick_index,
            tick_count: clock.tick_count,
        });
    }
    Ok(())
}

fn validate_known_compartment_limit(actual: usize) -> Result<(), BioSimProcessValidationError> {
    if actual > max_biosim_plan_compartments() {
        Err(BioSimProcessValidationError::TooManyCompartments {
            actual,
            max: max_biosim_plan_compartments(),
        })
    } else {
        Ok(())
    }
}

fn validate_process_limit(actual: usize) -> Result<(), BioSimProcessValidationError> {
    if actual > max_biosim_plan_processes() {
        Err(BioSimProcessValidationError::TooManyProcesses {
            actual,
            max: max_biosim_plan_processes(),
        })
    } else {
        Ok(())
    }
}

fn validate_process_shape(process: &BioSimProcess) -> Result<(), BioSimProcessValidationError> {
    if process.flows.is_empty() {
        return Err(BioSimProcessValidationError::ProcessWithNoFlows {
            process_id: process.id.as_str().to_owned(),
        });
    }
    if process.flows.len() > max_biosim_flows_per_process() {
        return Err(BioSimProcessValidationError::TooManyFlows {
            process_id: process.id.as_str().to_owned(),
            actual: process.flows.len(),
            max: max_biosim_flows_per_process(),
        });
    }
    for flow in &process.flows {
        ensure_positive_finite_rate(flow.rate_per_second)?;
    }
    ensure_no_duplicate_flow_cells(process)?;
    match process.kind {
        BioSimProcessKind::Source => validate_source_shape(process),
        BioSimProcessKind::Sink => validate_sink_shape(process),
        BioSimProcessKind::Transfer => validate_transfer_shape(process),
        BioSimProcessKind::Transform => validate_transform_shape(process),
    }
}

fn ensure_positive_finite_rate(rate_per_second: f64) -> Result<(), BioSimProcessValidationError> {
    if rate_per_second.is_finite() && rate_per_second > 0.0 {
        Ok(())
    } else {
        Err(BioSimProcessValidationError::InvalidFlowRate {
            value: rate_per_second,
            expected: "positive finite canonical unit per second",
        })
    }
}

fn validate_source_shape(process: &BioSimProcess) -> Result<(), BioSimProcessValidationError> {
    if process.flows.len() != 1 {
        return Err(BioSimProcessValidationError::SourceFlowCount {
            process_id: process.id.as_str().to_owned(),
            actual: process.flows.len(),
        });
    }
    let direction = process.flows[0].direction;
    if direction != BioSimFlowDirection::Output {
        return Err(BioSimProcessValidationError::SourceDirectionMismatch {
            process_id: process.id.as_str().to_owned(),
            actual: direction,
        });
    }
    Ok(())
}

fn validate_sink_shape(process: &BioSimProcess) -> Result<(), BioSimProcessValidationError> {
    if process.flows.len() != 1 {
        return Err(BioSimProcessValidationError::SinkFlowCount {
            process_id: process.id.as_str().to_owned(),
            actual: process.flows.len(),
        });
    }
    let direction = process.flows[0].direction;
    if direction != BioSimFlowDirection::Input {
        return Err(BioSimProcessValidationError::SinkDirectionMismatch {
            process_id: process.id.as_str().to_owned(),
            actual: direction,
        });
    }
    Ok(())
}

fn validate_transfer_shape(process: &BioSimProcess) -> Result<(), BioSimProcessValidationError> {
    if process.flows.len() != 2 {
        return Err(BioSimProcessValidationError::TransferFlowCount {
            process_id: process.id.as_str().to_owned(),
            actual: process.flows.len(),
        });
    }
    let mut input = None;
    let mut output = None;
    for flow in &process.flows {
        match flow.direction {
            BioSimFlowDirection::Input => input = Some(flow),
            BioSimFlowDirection::Output => output = Some(flow),
        }
    }
    let (input, output) = match (input, output) {
        (Some(input), Some(output)) => (input, output),
        _ => {
            return Err(BioSimProcessValidationError::TransferDirectionMismatch {
                process_id: process.id.as_str().to_owned(),
            })
        }
    };
    if input.compartment_id == output.compartment_id {
        return Err(BioSimProcessValidationError::TransferSameCompartment {
            process_id: process.id.as_str().to_owned(),
            compartment_id: input.compartment_id.as_str().to_owned(),
        });
    }
    if input.resource_kind != output.resource_kind {
        return Err(BioSimProcessValidationError::TransferResourceMismatch {
            process_id: process.id.as_str().to_owned(),
        });
    }
    if input.rate_per_second.to_bits() != output.rate_per_second.to_bits() {
        return Err(BioSimProcessValidationError::TransferRateMismatch {
            process_id: process.id.as_str().to_owned(),
        });
    }
    Ok(())
}

fn validate_transform_shape(process: &BioSimProcess) -> Result<(), BioSimProcessValidationError> {
    let mut input_count = 0usize;
    let mut output_count = 0usize;
    let mut compartment_id: Option<&BioSimCompartmentId> = None;
    let mut by_resource: BTreeMap<
        (&str, BioSimScenarioResourceKind),
        BTreeSet<BioSimFlowDirection>,
    > = BTreeMap::new();

    for flow in &process.flows {
        match flow.direction {
            BioSimFlowDirection::Input => input_count += 1,
            BioSimFlowDirection::Output => output_count += 1,
        }
        if let Some(expected_compartment) = compartment_id {
            if expected_compartment != &flow.compartment_id {
                return Err(BioSimProcessValidationError::TransformCrossCompartment {
                    process_id: process.id.as_str().to_owned(),
                });
            }
        } else {
            compartment_id = Some(&flow.compartment_id);
        }
        let directions = by_resource
            .entry((flow.compartment_id.as_str(), flow.resource_kind))
            .or_default();
        directions.insert(flow.direction);
    }

    if input_count == 0 {
        return Err(BioSimProcessValidationError::TransformMissingInput {
            process_id: process.id.as_str().to_owned(),
        });
    }
    if output_count == 0 {
        return Err(BioSimProcessValidationError::TransformMissingOutput {
            process_id: process.id.as_str().to_owned(),
        });
    }
    for ((compartment_id, resource_kind), directions) in by_resource {
        if directions.contains(&BioSimFlowDirection::Input)
            && directions.contains(&BioSimFlowDirection::Output)
        {
            return Err(BioSimProcessValidationError::TransformSameCellResource {
                process_id: process.id.as_str().to_owned(),
                compartment_id: compartment_id.to_owned(),
                resource_kind,
            });
        }
    }
    Ok(())
}

fn ensure_no_duplicate_flow_cells(
    process: &BioSimProcess,
) -> Result<(), BioSimProcessValidationError> {
    let mut seen = BTreeSet::new();
    for flow in &process.flows {
        let key = (
            flow.compartment_id.as_str().to_owned(),
            flow.resource_kind,
            flow.direction,
        );
        if !seen.insert(key) {
            return Err(BioSimProcessValidationError::DuplicateFlowCell {
                process_id: process.id.as_str().to_owned(),
                compartment_id: flow.compartment_id.as_str().to_owned(),
                resource_kind: flow.resource_kind,
                direction: flow.direction,
            });
        }
    }
    Ok(())
}

fn canonicalize_process_flows(process: &mut BioSimProcess) {
    process.flows.sort_by_key(canonical_flow_sort_key);
}

fn canonical_flow_sort_key(flow: &BioSimFlow) -> (u8, String, BioSimScenarioResourceKind, u64) {
    (
        direction_rank(flow.direction),
        flow.compartment_id.as_str().to_owned(),
        flow.resource_kind,
        flow.rate_per_second.to_bits(),
    )
}

const fn direction_rank(direction: BioSimFlowDirection) -> u8 {
    match direction {
        BioSimFlowDirection::Input => 0,
        BioSimFlowDirection::Output => 1,
    }
}

fn collect_known_compartments(
    known_compartment_ids: &[BioSimCompartmentId],
) -> Result<BTreeSet<String>, BioSimProcessValidationError> {
    let mut known_compartments = BTreeSet::new();
    for compartment_id in known_compartment_ids {
        if !known_compartments.insert(compartment_id.as_str().to_owned()) {
            return Err(BioSimProcessValidationError::DuplicateKnownCompartmentId {
                compartment_id: compartment_id.as_str().to_owned(),
            });
        }
    }
    Ok(known_compartments)
}

fn canonical_process_order(
    processes: &[BioSimProcess],
) -> Result<Vec<&BioSimProcess>, BioSimProcessValidationError> {
    let mut by_id: BTreeMap<&str, &BioSimProcess> = BTreeMap::new();
    for process in processes {
        if by_id.insert(process.id.as_str(), process).is_some() {
            return Err(BioSimProcessValidationError::DuplicateProcessId {
                process_id: process.id.as_str().to_owned(),
            });
        }
    }
    Ok(by_id.into_values().collect())
}

fn validate_flow_compartments(
    process: &BioSimProcess,
    known_compartments: &BTreeSet<String>,
) -> Result<(), BioSimProcessValidationError> {
    for flow in &process.flows {
        if !known_compartments.contains(flow.compartment_id.as_str()) {
            return Err(BioSimProcessValidationError::UnknownFlowCompartment {
                process_id: process.id.as_str().to_owned(),
                compartment_id: flow.compartment_id.as_str().to_owned(),
            });
        }
    }
    Ok(())
}

fn checked_total_intent_count<I>(flow_counts: I) -> Result<usize, BioSimProcessValidationError>
where
    I: IntoIterator<Item = usize>,
{
    let mut total = 0usize;
    for count in flow_counts {
        total = total
            .checked_add(count)
            .ok_or(BioSimProcessValidationError::IntentCountOverflow)?;
        if total > max_biosim_intents_per_plan() {
            return Err(BioSimProcessValidationError::TooManyIntents {
                actual: total,
                max: max_biosim_intents_per_plan(),
            });
        }
    }
    Ok(total)
}

fn reserve_intent_capacity(
    intents: &mut Vec<BioSimTickIntent>,
    requested: usize,
) -> Result<(), BioSimProcessValidationError> {
    if requested > max_biosim_intents_per_plan() {
        return Err(BioSimProcessValidationError::TooManyIntents {
            actual: requested,
            max: max_biosim_intents_per_plan(),
        });
    }
    intents.try_reserve_exact(requested).map_err(|_| {
        BioSimProcessValidationError::IntentAllocationFailed {
            requested,
            max: max_biosim_intents_per_plan(),
        }
    })
}

fn intent_from_flow(
    process: &BioSimProcess,
    flow: &BioSimFlow,
    tick_index: u64,
    sequence_index: usize,
    amount: f64,
) -> BioSimTickIntent {
    BioSimTickIntent {
        tick_index,
        sequence_index,
        process_id: process.id.clone(),
        kind: intent_kind_for_flow(process.kind, flow.direction),
        compartment_id: flow.compartment_id.clone(),
        resource_kind: flow.resource_kind,
        unit: biosim_scenario_resource_kind_unit(flow.resource_kind),
        amount,
    }
}

const fn intent_kind_for_flow(
    process_kind: BioSimProcessKind,
    direction: BioSimFlowDirection,
) -> BioSimTickIntentKind {
    match process_kind {
        BioSimProcessKind::Source => BioSimTickIntentKind::Source,
        BioSimProcessKind::Sink => BioSimTickIntentKind::Sink,
        BioSimProcessKind::Transfer | BioSimProcessKind::Transform => match direction {
            BioSimFlowDirection::Input => BioSimTickIntentKind::Consume,
            BioSimFlowDirection::Output => BioSimTickIntentKind::Produce,
        },
    }
}

const fn max_biosim_replay_ticks() -> u64 {
    10_000
}

const fn biosim_scenario_resource_kind_count() -> usize {
    6
}

const fn max_biosim_replay_cells() -> usize {
    max_biosim_plan_compartments() * biosim_scenario_resource_kind_count()
}

const fn max_biosim_replay_events() -> usize {
    1_000_000
}

const fn biosim_scenario_cell_digest_algorithm() -> &'static str {
    "fnv1a64:biosim_scenario_cells:v1"
}

/// Replay controls for bounded B2b-2 compartment-aware scenario replay.
///
/// The options are immutable after validation and contain no adapter field. They
/// bound in-memory replay size and tiny absolute underflow tolerance only. This
/// is research-planning metadata/replay support, not a biological-fidelity,
/// habitat-safety, medical, control, operational, certification, or parity claim.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BioSimScenarioReplayOptions {
    max_tick_count: u64,
    underflow_tolerance_abs: f64,
}

impl Default for BioSimScenarioReplayOptions {
    fn default() -> Self {
        Self {
            max_tick_count: max_biosim_replay_ticks(),
            underflow_tolerance_abs: 0.0,
        }
    }
}

impl BioSimScenarioReplayOptions {
    /// Validates caller-selected replay limits.
    pub fn new(
        max_tick_count: u64,
        underflow_tolerance_abs: f64,
    ) -> Result<Self, BioSimScenarioReplayError> {
        validate_replay_tick_limit(max_tick_count)?;
        validate_underflow_tolerance(underflow_tolerance_abs)?;
        Ok(Self {
            max_tick_count,
            underflow_tolerance_abs,
        })
    }

    /// Returns the nonzero caller-selected tick cap.
    #[must_use]
    pub const fn max_tick_count(&self) -> u64 {
        self.max_tick_count
    }

    /// Returns the finite nonnegative absolute underflow tolerance.
    #[must_use]
    pub const fn underflow_tolerance_abs(&self) -> f64 {
        self.underflow_tolerance_abs
    }
}

/// One immutable replay output cell identified by compartment and scenario resource.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimResourceCell {
    compartment_id: BioSimCompartmentId,
    resource_kind: BioSimScenarioResourceKind,
    unit: &'static str,
    amount: f64,
}

impl BioSimResourceCell {
    /// Returns the replay cell compartment ID.
    #[must_use]
    pub fn compartment_id(&self) -> &BioSimCompartmentId {
        &self.compartment_id
    }

    /// Returns the replay cell scenario-local resource kind.
    #[must_use]
    pub const fn resource_kind(&self) -> BioSimScenarioResourceKind {
        self.resource_kind
    }

    /// Returns the canonical record unit label for this cell.
    #[must_use]
    pub const fn unit(&self) -> &'static str {
        self.unit
    }

    /// Returns the finite nonnegative replay amount.
    #[must_use]
    pub const fn amount(&self) -> f64 {
        self.amount
    }
}

/// One immutable committed event generated by atomic B2b-2 replay.
///
/// This is a replay event for later B2c validation. It is not a ledger,
/// summarizer, report, external evidence record, conservation proof, or assurance
/// result.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenarioReplayEvent {
    tick_index: u64,
    sequence_index: usize,
    process_id: BioSimProcessId,
    intent_kind: BioSimTickIntentKind,
    compartment_id: BioSimCompartmentId,
    resource_kind: BioSimScenarioResourceKind,
    unit: &'static str,
    requested_amount: f64,
    committed_amount: f64,
    clamp_amount: f64,
    before_amount: f64,
    after_amount: f64,
}

impl BioSimScenarioReplayEvent {
    /// Returns the committed tick index.
    #[must_use]
    pub const fn tick_index(&self) -> u64 {
        self.tick_index
    }

    /// Returns the deterministic sequence index within the tick plan.
    #[must_use]
    pub const fn sequence_index(&self) -> usize {
        self.sequence_index
    }

    /// Returns the process that produced the planned intent.
    #[must_use]
    pub fn process_id(&self) -> &BioSimProcessId {
        &self.process_id
    }

    /// Returns the committed intent kind.
    #[must_use]
    pub const fn intent_kind(&self) -> BioSimTickIntentKind {
        self.intent_kind
    }

    /// Returns the event compartment.
    #[must_use]
    pub fn compartment_id(&self) -> &BioSimCompartmentId {
        &self.compartment_id
    }

    /// Returns the scenario-local resource kind.
    #[must_use]
    pub const fn resource_kind(&self) -> BioSimScenarioResourceKind {
        self.resource_kind
    }

    /// Returns the canonical record unit label.
    #[must_use]
    pub const fn unit(&self) -> &'static str {
        self.unit
    }

    /// Returns the planned requested amount.
    #[must_use]
    pub const fn requested_amount(&self) -> f64 {
        self.requested_amount
    }

    /// Returns the amount actually committed into the state transition.
    #[must_use]
    pub const fn committed_amount(&self) -> f64 {
        self.committed_amount
    }

    /// Returns the requested amount not committed because tolerance clamped it.
    #[must_use]
    pub const fn clamp_amount(&self) -> f64 {
        self.clamp_amount
    }

    /// Returns the amount before this event.
    #[must_use]
    pub const fn before_amount(&self) -> f64 {
        self.before_amount
    }

    /// Returns the amount after this event.
    #[must_use]
    pub const fn after_amount(&self) -> f64 {
        self.after_amount
    }
}

/// Deterministic immutable summary for one fully committed scenario tick.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenarioTickSummary {
    tick_index: u64,
    intent_count: usize,
    event_count: usize,
    before_cell_state_digest: String,
    after_cell_state_digest: String,
}

impl BioSimScenarioTickSummary {
    /// Returns the committed tick index.
    #[must_use]
    pub const fn tick_index(&self) -> u64 {
        self.tick_index
    }

    /// Returns the planned intent count for this tick.
    #[must_use]
    pub const fn intent_count(&self) -> usize {
        self.intent_count
    }

    /// Returns the committed event count for this tick.
    #[must_use]
    pub const fn event_count(&self) -> usize {
        self.event_count
    }

    /// Returns the compact noncryptographic digest before this tick.
    #[must_use]
    pub fn before_cell_state_digest(&self) -> &str {
        &self.before_cell_state_digest
    }

    /// Returns the compact noncryptographic digest after this tick.
    #[must_use]
    pub fn after_cell_state_digest(&self) -> &str {
        &self.after_cell_state_digest
    }
}

/// Complete successful B2b-2 bounded replay output.
///
/// Failed replay returns `BioSimScenarioReplayError` and exposes no partial
/// replay object. Digest values are compact noncryptographic Fowler-Noll-Vo 1a comparison
/// strings, not security hashes, persistence keys, external evidence, or
/// compatibility guarantees beyond the documented domain/version.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenarioReplay {
    scenario_id: String,
    tick_count: u64,
    process_count: usize,
    initial_cell_state_digest: String,
    final_cell_state_digest: String,
    final_cells: Vec<BioSimResourceCell>,
    tick_summaries: Vec<BioSimScenarioTickSummary>,
    replay_events: Vec<BioSimScenarioReplayEvent>,
}

impl BioSimScenarioReplay {
    /// Returns the scenario identifier replayed by this successful result.
    #[must_use]
    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    /// Returns the committed tick count.
    #[must_use]
    pub const fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Returns the number of process definitions supplied to replay.
    #[must_use]
    pub const fn process_count(&self) -> usize {
        self.process_count
    }

    /// Returns the initial compact noncryptographic state digest.
    #[must_use]
    pub fn initial_cell_state_digest(&self) -> &str {
        &self.initial_cell_state_digest
    }

    /// Returns the final compact noncryptographic state digest.
    #[must_use]
    pub fn final_cell_state_digest(&self) -> &str {
        &self.final_cell_state_digest
    }

    /// Returns final cells in deterministic compartment/resource order.
    #[must_use]
    pub fn final_cells(&self) -> &[BioSimResourceCell] {
        &self.final_cells
    }

    /// Returns tick summaries in committed tick order.
    #[must_use]
    pub fn tick_summaries(&self) -> &[BioSimScenarioTickSummary] {
        &self.tick_summaries
    }

    /// Returns committed replay events in deterministic tick/sequence order.
    #[must_use]
    pub fn replay_events(&self) -> &[BioSimScenarioReplayEvent] {
        &self.replay_events
    }
}

/// B2b-2 deterministic replay failures.
#[derive(Debug, Clone, PartialEq)]
pub enum BioSimScenarioReplayError {
    ScenarioValidationFailed {
        errors: Vec<BioSimScenarioValidationError>,
    },
    ProcessPlanningFailed {
        tick_index: u64,
        source: BioSimProcessValidationError,
    },
    InvalidReplayMaxTickCount {
        value: u64,
        max: u64,
    },
    InvalidUnderflowToleranceAbs {
        value: f64,
    },
    ReplaySizeLimitExceeded {
        tick_count: u64,
        max_tick_count: u64,
    },
    ReplayCountConversionFailed {
        value: u64,
        target: &'static str,
    },
    ReplayCountOverflow {
        operation: &'static str,
    },
    ReplayAllocationFailed {
        collection: &'static str,
        requested: usize,
        max: usize,
    },
    ReplayCellLimitExceeded {
        actual: usize,
        max: usize,
    },
    DuplicateInitialCell {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
    },
    NonfiniteCellAmount {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        amount: f64,
    },
    NegativeCellAmount {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        amount: f64,
    },
    CellUnderflow {
        tick_index: u64,
        sequence_index: usize,
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        available_amount: f64,
        requested_amount: f64,
        deficit: f64,
        underflow_tolerance_abs: f64,
    },
    CellAmountNotFiniteAfterCommit {
        tick_index: u64,
        sequence_index: usize,
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        attempted_amount: f64,
    },
    CellAmountNegativeAfterCommit {
        tick_index: u64,
        sequence_index: usize,
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        attempted_amount: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ScenarioCellKey {
    compartment_id: String,
    resource_kind: BioSimScenarioResourceKind,
}

/// Runs deterministic B2b-2 compartment replay with default options.
pub fn run_biosim_scenario(
    scenario: &BioSimScenario,
    processes: &[BioSimProcess],
) -> Result<BioSimScenarioReplay, BioSimScenarioReplayError> {
    run_biosim_scenario_with_options(scenario, processes, BioSimScenarioReplayOptions::default())
}

/// Runs deterministic B2b-2 compartment replay with validated explicit options.
///
/// Replay is pure in-memory state-in/state-out work. It does not read files,
/// write files, use wall-clock time, use random values, persist a ledger, call an
/// adapter, or aggregate scenario resources into flat resource kinds. Each tick
/// is committed atomically by first applying all intents to a tentative state and
/// only replacing the live state after every intent, finite-state check, digest,
/// summary construction, and output reservation succeeds.
pub fn run_biosim_scenario_with_options(
    scenario: &BioSimScenario,
    processes: &[BioSimProcess],
    options: BioSimScenarioReplayOptions,
) -> Result<BioSimScenarioReplay, BioSimScenarioReplayError> {
    validate_replay_options(options)?;
    let report = validate_biosim_scenario(scenario);
    if !report.is_ok() {
        return Err(BioSimScenarioReplayError::ScenarioValidationFailed {
            errors: report.errors,
        });
    }
    if scenario.clock.tick_count > options.max_tick_count {
        return Err(BioSimScenarioReplayError::ReplaySizeLimitExceeded {
            tick_count: scenario.clock.tick_count,
            max_tick_count: options.max_tick_count,
        });
    }
    let tick_capacity = usize::try_from(scenario.clock.tick_count).map_err(|_| {
        BioSimScenarioReplayError::ReplayCountConversionFailed {
            value: scenario.clock.tick_count,
            target: "usize tick capacity",
        }
    })?;
    let mut tick_summaries = Vec::new();
    reserve_replay_vec(&mut tick_summaries, tick_capacity, "tick_summaries")?;
    let mut replay_events = Vec::new();
    let known_compartment_ids = scenario
        .compartments
        .iter()
        .map(|compartment| compartment.id.clone())
        .collect::<Vec<_>>();
    let mut state = initial_replay_state(scenario)?;
    let initial_cell_state_digest = scenario_cell_state_digest(0, &state)?;

    for tick_index in 0..scenario.clock.tick_count {
        let plan = plan_biosim_tick_intents(
            processes,
            &known_compartment_ids,
            scenario.clock,
            tick_index,
        )
        .map_err(|source| BioSimScenarioReplayError::ProcessPlanningFailed {
            tick_index,
            source,
        })?;
        let before_state = state.clone();
        let before_cell_state_digest = scenario_cell_state_digest(tick_index, &before_state)?;
        let mut tentative_state = before_state.clone();
        let mut tick_events = Vec::new();
        reserve_replay_vec(&mut tick_events, plan.intents().len(), "tick_events")?;
        checked_total_replay_events(replay_events.len(), plan.intents().len())?;
        reserve_replay_vec(&mut replay_events, plan.intents().len(), "replay_events")?;
        for intent in plan.intents() {
            let event = apply_intent_to_tentative_state(
                intent,
                &mut tentative_state,
                options.underflow_tolerance_abs,
            )?;
            tick_events.push(event);
        }
        validate_replay_state(&tentative_state)?;
        let after_cell_state_digest = scenario_cell_state_digest(tick_index + 1, &tentative_state)?;
        let summary = BioSimScenarioTickSummary {
            tick_index,
            intent_count: plan.intents().len(),
            event_count: tick_events.len(),
            before_cell_state_digest,
            after_cell_state_digest,
        };
        state = tentative_state;
        replay_events.extend(tick_events);
        tick_summaries.push(summary);
    }
    let final_cell_state_digest = scenario_cell_state_digest(scenario.clock.tick_count, &state)?;
    let final_cells = replay_state_to_cells(&state)?;
    Ok(BioSimScenarioReplay {
        scenario_id: scenario.metadata.scenario_id.clone(),
        tick_count: scenario.clock.tick_count,
        process_count: processes.len(),
        initial_cell_state_digest,
        final_cell_state_digest,
        final_cells,
        tick_summaries,
        replay_events,
    })
}

fn validate_replay_options(
    options: BioSimScenarioReplayOptions,
) -> Result<(), BioSimScenarioReplayError> {
    validate_replay_tick_limit(options.max_tick_count)?;
    validate_underflow_tolerance(options.underflow_tolerance_abs)
}

fn validate_replay_tick_limit(max_tick_count: u64) -> Result<(), BioSimScenarioReplayError> {
    if max_tick_count == 0 || max_tick_count > max_biosim_replay_ticks() {
        Err(BioSimScenarioReplayError::InvalidReplayMaxTickCount {
            value: max_tick_count,
            max: max_biosim_replay_ticks(),
        })
    } else {
        Ok(())
    }
}

fn validate_underflow_tolerance(value: f64) -> Result<(), BioSimScenarioReplayError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(BioSimScenarioReplayError::InvalidUnderflowToleranceAbs { value })
    }
}

fn reserve_replay_vec<T>(
    vec: &mut Vec<T>,
    requested: usize,
    collection: &'static str,
) -> Result<(), BioSimScenarioReplayError> {
    vec.try_reserve_exact(requested).map_err(|_| {
        BioSimScenarioReplayError::ReplayAllocationFailed {
            collection,
            requested,
            max: max_biosim_replay_events(),
        }
    })
}

fn checked_total_replay_events(
    current: usize,
    additional: usize,
) -> Result<usize, BioSimScenarioReplayError> {
    let total =
        current
            .checked_add(additional)
            .ok_or(BioSimScenarioReplayError::ReplayCountOverflow {
                operation: "total committed replay events",
            })?;
    if total > max_biosim_replay_events() {
        Err(BioSimScenarioReplayError::ReplayAllocationFailed {
            collection: "replay_events",
            requested: total,
            max: max_biosim_replay_events(),
        })
    } else {
        Ok(total)
    }
}

#[cfg(test)]
fn checked_replay_cell_capacity_for_test(
    compartments: usize,
    resource_kinds: usize,
) -> Result<usize, BioSimScenarioReplayError> {
    let total = compartments.checked_mul(resource_kinds).ok_or(
        BioSimScenarioReplayError::ReplayCountOverflow {
            operation: "compartment resource cell capacity",
        },
    )?;
    if total > max_biosim_replay_cells() {
        Err(BioSimScenarioReplayError::ReplayCellLimitExceeded {
            actual: total,
            max: max_biosim_replay_cells(),
        })
    } else {
        Ok(total)
    }
}

#[cfg(test)]
fn try_reserve_replay_capacity_for_test(requested: usize) -> Result<(), BioSimScenarioReplayError> {
    let mut values = Vec::<BioSimScenarioReplayEvent>::new();
    values.try_reserve_exact(requested).map_err(|_| {
        BioSimScenarioReplayError::ReplayAllocationFailed {
            collection: "replay_events",
            requested,
            max: max_biosim_replay_events(),
        }
    })
}

fn initial_replay_state(
    scenario: &BioSimScenario,
) -> Result<BTreeMap<ScenarioCellKey, f64>, BioSimScenarioReplayError> {
    let mut state = BTreeMap::new();
    for store in &scenario.initial_stores {
        let key = ScenarioCellKey {
            compartment_id: store.compartment_id.as_str().to_owned(),
            resource_kind: store.resource_kind,
        };
        let amount = normalize_replay_zero(store.amount);
        if state.insert(key.clone(), amount).is_some() {
            return Err(BioSimScenarioReplayError::DuplicateInitialCell {
                compartment_id: key.compartment_id,
                resource_kind: key.resource_kind,
            });
        }
        enforce_replay_cell_limit(state.len())?;
    }
    validate_replay_state(&state)?;
    Ok(state)
}

fn apply_intent_to_tentative_state(
    intent: &BioSimTickIntent,
    state: &mut BTreeMap<ScenarioCellKey, f64>,
    underflow_tolerance_abs: f64,
) -> Result<BioSimScenarioReplayEvent, BioSimScenarioReplayError> {
    let key = ScenarioCellKey {
        compartment_id: intent.compartment_id().as_str().to_owned(),
        resource_kind: intent.resource_kind(),
    };
    let had_cell = state.contains_key(&key);
    let before_amount = normalize_replay_zero(*state.get(&key).unwrap_or(&0.0));
    let requested_amount = intent.amount();
    if !requested_amount.is_finite() || requested_amount <= 0.0 {
        return Err(BioSimScenarioReplayError::CellAmountNotFiniteAfterCommit {
            tick_index: intent.tick_index(),
            sequence_index: intent.sequence_index(),
            compartment_id: key.compartment_id,
            resource_kind: key.resource_kind,
            attempted_amount: requested_amount,
        });
    }
    let (committed_amount, clamp_amount, after_amount) = match intent.kind() {
        BioSimTickIntentKind::Produce | BioSimTickIntentKind::Source => {
            let after = before_amount + requested_amount;
            (requested_amount, 0.0, normalize_replay_zero(after))
        }
        BioSimTickIntentKind::Consume | BioSimTickIntentKind::Sink => {
            if requested_amount <= before_amount {
                (
                    requested_amount,
                    0.0,
                    normalize_replay_zero(before_amount - requested_amount),
                )
            } else {
                let deficit = requested_amount - before_amount;
                if !deficit.is_finite() || deficit > underflow_tolerance_abs {
                    return Err(BioSimScenarioReplayError::CellUnderflow {
                        tick_index: intent.tick_index(),
                        sequence_index: intent.sequence_index(),
                        compartment_id: key.compartment_id,
                        resource_kind: key.resource_kind,
                        available_amount: before_amount,
                        requested_amount,
                        deficit,
                        underflow_tolerance_abs,
                    });
                }
                (before_amount, normalize_replay_zero(deficit), 0.0)
            }
        }
    };
    validate_event_amounts(
        intent,
        &key,
        requested_amount,
        committed_amount,
        clamp_amount,
        before_amount,
        after_amount,
    )?;
    if had_cell
        || matches!(
            intent.kind(),
            BioSimTickIntentKind::Produce | BioSimTickIntentKind::Source
        )
    {
        if !had_cell {
            enforce_replay_cell_limit(state.len().checked_add(1).ok_or(
                BioSimScenarioReplayError::ReplayCountOverflow {
                    operation: "inserted replay cell count",
                },
            )?)?;
        }
        state.insert(key.clone(), after_amount);
    }
    Ok(BioSimScenarioReplayEvent {
        tick_index: intent.tick_index(),
        sequence_index: intent.sequence_index(),
        process_id: intent.process_id().clone(),
        intent_kind: intent.kind(),
        compartment_id: intent.compartment_id().clone(),
        resource_kind: intent.resource_kind(),
        unit: intent.unit(),
        requested_amount,
        committed_amount,
        clamp_amount,
        before_amount,
        after_amount,
    })
}

fn validate_event_amounts(
    intent: &BioSimTickIntent,
    key: &ScenarioCellKey,
    requested_amount: f64,
    committed_amount: f64,
    clamp_amount: f64,
    before_amount: f64,
    after_amount: f64,
) -> Result<(), BioSimScenarioReplayError> {
    for amount in [
        requested_amount,
        committed_amount,
        clamp_amount,
        before_amount,
        after_amount,
    ] {
        if !amount.is_finite() {
            return Err(BioSimScenarioReplayError::CellAmountNotFiniteAfterCommit {
                tick_index: intent.tick_index(),
                sequence_index: intent.sequence_index(),
                compartment_id: key.compartment_id.clone(),
                resource_kind: key.resource_kind,
                attempted_amount: amount,
            });
        }
        if amount < 0.0 {
            return Err(BioSimScenarioReplayError::CellAmountNegativeAfterCommit {
                tick_index: intent.tick_index(),
                sequence_index: intent.sequence_index(),
                compartment_id: key.compartment_id.clone(),
                resource_kind: key.resource_kind,
                attempted_amount: amount,
            });
        }
    }
    Ok(())
}

fn enforce_replay_cell_limit(actual: usize) -> Result<(), BioSimScenarioReplayError> {
    if actual > max_biosim_replay_cells() {
        Err(BioSimScenarioReplayError::ReplayCellLimitExceeded {
            actual,
            max: max_biosim_replay_cells(),
        })
    } else {
        Ok(())
    }
}

fn validate_replay_state(
    state: &BTreeMap<ScenarioCellKey, f64>,
) -> Result<(), BioSimScenarioReplayError> {
    enforce_replay_cell_limit(state.len())?;
    for (key, amount) in state {
        if !amount.is_finite() {
            return Err(BioSimScenarioReplayError::NonfiniteCellAmount {
                compartment_id: key.compartment_id.clone(),
                resource_kind: key.resource_kind,
                amount: *amount,
            });
        }
        if *amount < 0.0 {
            return Err(BioSimScenarioReplayError::NegativeCellAmount {
                compartment_id: key.compartment_id.clone(),
                resource_kind: key.resource_kind,
                amount: *amount,
            });
        }
    }
    Ok(())
}

fn replay_state_to_cells(
    state: &BTreeMap<ScenarioCellKey, f64>,
) -> Result<Vec<BioSimResourceCell>, BioSimScenarioReplayError> {
    validate_replay_state(state)?;
    let mut cells = Vec::new();
    reserve_replay_vec(&mut cells, state.len(), "final_cells")?;
    for (key, amount) in state {
        cells.push(BioSimResourceCell {
            compartment_id: BioSimCompartmentId::new(key.compartment_id.clone()).map_err(|_| {
                BioSimScenarioReplayError::ReplayCountOverflow {
                    operation: "reconstruct validated compartment ID",
                }
            })?,
            resource_kind: key.resource_kind,
            unit: biosim_scenario_resource_kind_unit(key.resource_kind),
            amount: normalize_replay_zero(*amount),
        });
    }
    Ok(cells)
}

fn normalize_replay_zero(value: f64) -> f64 {
    if value == 0.0 {
        0.0
    } else {
        value
    }
}

fn canonical_f64_bits_for_replay(value: f64) -> Result<u64, BioSimScenarioReplayError> {
    if !value.is_finite() {
        Err(BioSimScenarioReplayError::NonfiniteCellAmount {
            compartment_id: "digest".to_owned(),
            resource_kind: BioSimScenarioResourceKind::Oxygen,
            amount: value,
        })
    } else {
        Ok(normalize_replay_zero(value).to_bits())
    }
}

fn scenario_cell_state_digest(
    tick_index: u64,
    state: &BTreeMap<ScenarioCellKey, f64>,
) -> Result<String, BioSimScenarioReplayError> {
    validate_replay_state(state)?;
    let mut hash = fnv1a64_offset_basis();
    fnv1a64_string(&mut hash, biosim_scenario_cell_digest_algorithm());
    fnv1a64_u64(&mut hash, tick_index);
    let cell_count = u64::try_from(state.len()).map_err(|_| {
        BioSimScenarioReplayError::ReplayCountConversionFailed {
            value: u64::MAX,
            target: "u64 cell count",
        }
    })?;
    fnv1a64_u64(&mut hash, cell_count);
    for (key, amount) in state {
        fnv1a64_string(&mut hash, &key.compartment_id);
        fnv1a64_string(
            &mut hash,
            biosim_scenario_resource_kind_canonical_id(key.resource_kind),
        );
        fnv1a64_string(
            &mut hash,
            biosim_scenario_resource_kind_unit(key.resource_kind),
        );
        fnv1a64_u64(&mut hash, canonical_f64_bits_for_replay(*amount)?);
    }
    Ok(format!("{hash:016x}"))
}

fn fnv1a64_offset_basis() -> u64 {
    0xcbf2_9ce4_8422_2325
}

fn fnv1a64_prime() -> u64 {
    0x0000_0100_0000_01b3
}

fn fnv1a64_update(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(fnv1a64_prime());
    }
}

fn fnv1a64_u64(hash: &mut u64, value: u64) {
    fnv1a64_update(hash, &value.to_le_bytes());
}

fn fnv1a64_string(hash: &mut u64, value: &str) {
    let length = u64::try_from(value.len()).unwrap_or(u64::MAX);
    fnv1a64_u64(hash, length);
    fnv1a64_update(hash, value.as_bytes());
}

#[cfg(test)]
fn invalid_process_for_test(
    id: BioSimProcessId,
    kind: BioSimProcessKind,
    flows: Vec<BioSimFlow>,
) -> BioSimProcess {
    BioSimProcess { id, kind, flows }
}

#[cfg(test)]
fn try_reserve_capacity_overflow_for_test() -> Result<(), BioSimProcessValidationError> {
    let mut intents = Vec::<BioSimTickIntent>::new();
    intents.try_reserve_exact(usize::MAX).map_err(|_| {
        BioSimProcessValidationError::IntentAllocationFailed {
            requested: usize::MAX,
            max: max_biosim_intents_per_plan(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compartment(raw: &str) -> BioSimCompartmentId {
        BioSimCompartmentId::new(raw).expect("valid compartment")
    }

    fn process_id(raw: &str) -> BioSimProcessId {
        BioSimProcessId::new(raw).expect("valid process")
    }

    const fn clock() -> BioSimScenarioClock {
        BioSimScenarioClock::new(10.0, 4)
    }

    fn known_compartments() -> Vec<BioSimCompartmentId> {
        vec![
            compartment("crew_cabin"),
            compartment("plant_bed"),
            compartment("water_tank"),
        ]
    }

    fn source_process() -> BioSimProcess {
        BioSimProcess::source(
            process_id("source_oxygen"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            0.5,
        )
        .expect("valid source")
    }

    fn sink_process() -> BioSimProcess {
        BioSimProcess::sink(
            process_id("sink_carbon_dioxide"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::CarbonDioxide,
            0.25,
        )
        .expect("valid sink")
    }

    fn transfer_process() -> BioSimProcess {
        BioSimProcess::transfer(
            process_id("transfer_water"),
            compartment("water_tank"),
            compartment("plant_bed"),
            BioSimScenarioResourceKind::Water,
            0.75,
        )
        .expect("valid transfer")
    }

    fn transform_process_with_order(first_food: bool) -> BioSimProcess {
        let mut inputs = vec![
            BioSimFlow::input(
                compartment("plant_bed"),
                BioSimScenarioResourceKind::CarbonDioxide,
                0.1,
            )
            .expect("valid input"),
            BioSimFlow::input(
                compartment("plant_bed"),
                BioSimScenarioResourceKind::Water,
                0.2,
            )
            .expect("valid input"),
        ];
        let mut outputs = vec![
            BioSimFlow::output(
                compartment("plant_bed"),
                BioSimScenarioResourceKind::Oxygen,
                0.3,
            )
            .expect("valid output"),
            BioSimFlow::output(
                compartment("plant_bed"),
                BioSimScenarioResourceKind::Food,
                0.4,
            )
            .expect("valid output"),
        ];
        if first_food {
            inputs.reverse();
            outputs.reverse();
        }
        BioSimProcess::transform(process_id("transform_growth"), inputs, outputs)
            .expect("valid transform")
    }

    fn assert_error(
        result: Result<BioSimTickIntentPlan, BioSimProcessValidationError>,
        predicate: impl FnOnce(&BioSimProcessValidationError) -> bool,
    ) {
        let error = result.expect_err("expected validation error");
        assert!(predicate(&error), "unexpected error: {error:?}");
    }

    fn scenario_metadata() -> BioSimScenarioMetadata {
        BioSimScenarioMetadata::new(
            "synthetic_replay_case",
            "Synthetic replay case",
            "research_required",
        )
        .expect("valid metadata")
    }

    fn replay_compartments() -> Vec<BioSimCompartment> {
        vec![
            BioSimCompartment::new("crew_cabin", "Crew cabin").expect("valid compartment"),
            BioSimCompartment::new("buffer_store", "Buffer store").expect("valid compartment"),
            BioSimCompartment::new("plant_chamber", "Plant chamber").expect("valid compartment"),
        ]
    }

    const fn replay_clock(tick_count: u64) -> BioSimScenarioClock {
        BioSimScenarioClock::new(10.0, tick_count)
    }

    fn basic_replay_scenario(tick_count: u64) -> BioSimScenario {
        let compartments = replay_compartments();
        let crew = compartments[0].id.clone();
        let buffer = compartments[1].id.clone();
        BioSimScenario::new(
            scenario_metadata(),
            compartments,
            vec![
                BioSimInitialStore::new(crew, BioSimScenarioResourceKind::Oxygen, 1.0),
                BioSimInitialStore::new(buffer, BioSimScenarioResourceKind::Oxygen, 5.0),
            ],
            replay_clock(tick_count),
        )
    }

    fn supported_transfer_process() -> BioSimProcess {
        BioSimProcess::transfer(
            process_id("oxygen_transfer"),
            compartment("buffer_store"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            0.1,
        )
        .expect("valid transfer")
    }

    fn oxygen_sink(rate_per_second: f64) -> BioSimProcess {
        BioSimProcess::sink(
            process_id("oxygen_sink"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            rate_per_second,
        )
        .expect("valid sink")
    }

    fn oxygen_source_to_plant(rate_per_second: f64) -> BioSimProcess {
        BioSimProcess::source(
            process_id("plant_o2_source"),
            compartment("plant_chamber"),
            BioSimScenarioResourceKind::Oxygen,
            rate_per_second,
        )
        .expect("valid source")
    }

    fn final_cell_amount(
        replay: &BioSimScenarioReplay,
        compartment_id: &str,
        resource_kind: BioSimScenarioResourceKind,
    ) -> f64 {
        replay
            .final_cells()
            .iter()
            .find(|cell| {
                cell.compartment_id().as_str() == compartment_id
                    && cell.resource_kind() == resource_kind
            })
            .map(BioSimResourceCell::amount)
            .expect("cell present")
    }

    fn digest_is_compact(value: &str) -> bool {
        value.len() == 16
            && value
                .chars()
                .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase())
    }

    #[test]
    fn replay_options_validate_bounds() {
        let default = BioSimScenarioReplayOptions::default();
        assert_eq!(default.max_tick_count(), max_biosim_replay_ticks());
        assert_eq!(default.underflow_tolerance_abs(), 0.0);
        assert!(BioSimScenarioReplayOptions::new(1, 0.0).is_ok());
        assert!(BioSimScenarioReplayOptions::new(max_biosim_replay_ticks(), 0.0).is_ok());
        assert!(matches!(
            BioSimScenarioReplayOptions::new(0, 0.0),
            Err(BioSimScenarioReplayError::InvalidReplayMaxTickCount { value: 0, .. })
        ));
        assert!(matches!(
            BioSimScenarioReplayOptions::new(max_biosim_replay_ticks() + 1, 0.0),
            Err(BioSimScenarioReplayError::InvalidReplayMaxTickCount { .. })
        ));
        assert!(matches!(
            BioSimScenarioReplayOptions::new(1, -1.0),
            Err(BioSimScenarioReplayError::InvalidUnderflowToleranceAbs { .. })
        ));
        assert!(matches!(
            BioSimScenarioReplayOptions::new(1, f64::NAN),
            Err(BioSimScenarioReplayError::InvalidUnderflowToleranceAbs { .. })
        ));
        assert!(matches!(
            BioSimScenarioReplayOptions::new(1, f64::INFINITY),
            Err(BioSimScenarioReplayError::InvalidUnderflowToleranceAbs { .. })
        ));
    }

    #[test]
    fn invalid_scenario_and_planning_errors_are_rejected_before_replay_success() {
        let mut invalid = basic_replay_scenario(1);
        invalid.metadata.validation_status = "not_research_required".to_owned();
        assert!(matches!(
            run_biosim_scenario(&invalid, &[]),
            Err(BioSimScenarioReplayError::ScenarioValidationFailed { .. })
        ));

        let scenario = basic_replay_scenario(1);
        let bad_process = invalid_process_for_test(
            process_id("bad_process"),
            BioSimProcessKind::Source,
            vec![BioSimFlow::input(
                compartment("crew_cabin"),
                BioSimScenarioResourceKind::Oxygen,
                1.0,
            )
            .expect("valid flow")],
        );
        assert!(matches!(
            run_biosim_scenario(&scenario, &[bad_process]),
            Err(BioSimScenarioReplayError::ProcessPlanningFailed { tick_index: 0, .. })
        ));
    }

    #[test]
    fn replay_rejects_tick_count_above_caller_and_hard_limits() {
        let scenario = basic_replay_scenario(5);
        let options = BioSimScenarioReplayOptions::new(4, 0.0).expect("valid lower limit");
        assert!(matches!(
            run_biosim_scenario_with_options(&scenario, &[], options),
            Err(BioSimScenarioReplayError::ReplaySizeLimitExceeded {
                tick_count: 5,
                max_tick_count: 4
            })
        ));
        let hard_options = BioSimScenarioReplayOptions::new(max_biosim_replay_ticks(), 0.0)
            .expect("hard max valid");
        let too_large = BioSimScenario::new(
            scenario_metadata(),
            replay_compartments(),
            Vec::new(),
            replay_clock(max_biosim_replay_ticks() + 1),
        );
        assert!(matches!(
            run_biosim_scenario_with_options(&too_large, &[], hard_options),
            Err(BioSimScenarioReplayError::ReplaySizeLimitExceeded { .. })
        ));
    }

    #[test]
    fn valid_one_tick_and_multi_tick_replay_are_deterministic() {
        let one_tick = basic_replay_scenario(1);
        let processes = [supported_transfer_process()];
        let replay = run_biosim_scenario(&one_tick, &processes).expect("one tick replay");
        assert_eq!(replay.tick_count(), 1);
        assert_eq!(replay.tick_summaries().len(), 1);
        assert_eq!(replay.replay_events().len(), 2);
        assert!(digest_is_compact(replay.initial_cell_state_digest()));
        assert!(digest_is_compact(replay.final_cell_state_digest()));

        let multi_tick = basic_replay_scenario(3);
        let first = run_biosim_scenario(&multi_tick, &processes).expect("first replay");
        let second = run_biosim_scenario(&multi_tick, &processes).expect("second replay");
        assert_eq!(first, second);
        assert_eq!(first.tick_summaries().len(), 3);
        assert_eq!(first.replay_events().len(), 6);
    }

    #[test]
    fn process_and_initial_store_reordering_do_not_change_replay() {
        let scenario = basic_replay_scenario(2);
        let transfer = supported_transfer_process();
        let source = oxygen_source_to_plant(0.05);
        let forward = run_biosim_scenario(&scenario, &[transfer.clone(), source.clone()])
            .expect("forward replay");
        let reversed =
            run_biosim_scenario(&scenario, &[source, transfer]).expect("reversed replay");
        assert_eq!(forward, reversed);

        let mut reordered = scenario.clone();
        reordered.initial_stores.reverse();
        let initial_forward = run_biosim_scenario(&scenario, &[supported_transfer_process()])
            .expect("original stores");
        let initial_reordered = run_biosim_scenario(&reordered, &[supported_transfer_process()])
            .expect("reordered stores");
        assert_eq!(initial_forward, initial_reordered);
    }

    #[test]
    fn produced_absent_cell_is_inserted_and_final_cells_are_ordered() {
        let scenario = basic_replay_scenario(1);
        let first =
            run_biosim_scenario(&scenario, &[oxygen_source_to_plant(0.25)]).expect("first replay");
        let second =
            run_biosim_scenario(&scenario, &[oxygen_source_to_plant(0.25)]).expect("second replay");
        assert_eq!(
            final_cell_amount(&first, "plant_chamber", BioSimScenarioResourceKind::Oxygen),
            2.5
        );
        assert_eq!(
            first.final_cell_state_digest(),
            second.final_cell_state_digest()
        );
        let ids = first
            .final_cells()
            .iter()
            .map(|cell| {
                (
                    cell.compartment_id().as_str().to_owned(),
                    cell.resource_kind(),
                )
            })
            .collect::<Vec<_>>();
        let mut sorted = ids.clone();
        sorted.sort();
        assert_eq!(ids, sorted);
    }

    #[test]
    fn event_sequence_and_digest_chain_are_contiguous() {
        let scenario = basic_replay_scenario(2);
        let replay =
            run_biosim_scenario(&scenario, &[supported_transfer_process()]).expect("replay");
        for summary in replay.tick_summaries() {
            assert_eq!(summary.intent_count(), 2);
            assert_eq!(summary.event_count(), 2);
            assert!(digest_is_compact(summary.before_cell_state_digest()));
            assert!(digest_is_compact(summary.after_cell_state_digest()));
        }
        assert_eq!(
            replay.tick_summaries()[0].after_cell_state_digest(),
            replay.tick_summaries()[1].before_cell_state_digest()
        );
        for chunk in replay.replay_events().chunks(2) {
            assert_eq!(chunk[0].sequence_index(), 0);
            assert_eq!(chunk[1].sequence_index(), 1);
        }
    }

    #[test]
    fn underflow_errors_return_no_partial_replay_object() {
        let scenario = basic_replay_scenario(2);
        assert!(matches!(
            run_biosim_scenario(&scenario, &[oxygen_sink(2.0)]),
            Err(BioSimScenarioReplayError::CellUnderflow { tick_index: 0, .. })
        ));
        let mut later = basic_replay_scenario(3);
        later.initial_stores = vec![BioSimInitialStore::new(
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            1.5,
        )];
        assert!(matches!(
            run_biosim_scenario(&later, &[oxygen_sink(0.1)]),
            Err(BioSimScenarioReplayError::CellUnderflow { tick_index: 1, .. })
        ));
    }

    #[test]
    fn requested_committed_and_clamp_event_semantics_are_explicit() {
        let mut exact = basic_replay_scenario(1);
        exact.initial_stores = vec![BioSimInitialStore::new(
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            1.0,
        )];
        let replay = run_biosim_scenario(&exact, &[oxygen_sink(0.1)]).expect("exact consume");
        let event = &replay.replay_events()[0];
        assert_eq!(event.requested_amount(), 1.0);
        assert_eq!(event.committed_amount(), 1.0);
        assert_eq!(event.clamp_amount(), 0.0);
        assert_eq!(event.before_amount(), 1.0);
        assert_eq!(event.after_amount(), 0.0);

        let options = BioSimScenarioReplayOptions::new(1, 1.0e-9).expect("valid tolerance");
        let clamped =
            run_biosim_scenario_with_options(&exact, &[oxygen_sink(0.10000000001)], options)
                .expect("clamped consume");
        let clamp_event = &clamped.replay_events()[0];
        assert_eq!(clamp_event.before_amount(), 1.0);
        assert_eq!(clamp_event.committed_amount(), 1.0);
        assert!(clamp_event.clamp_amount() > 0.0);
        assert_eq!(clamp_event.after_amount(), 0.0);

        let source =
            run_biosim_scenario(&exact, &[oxygen_source_to_plant(0.25)]).expect("source event");
        let source_event = &source.replay_events()[0];
        assert_eq!(source_event.requested_amount(), 2.5);
        assert_eq!(source_event.committed_amount(), 2.5);
        assert_eq!(source_event.clamp_amount(), 0.0);
    }

    #[test]
    fn large_finite_underflow_uses_subtraction_safe_deficit() {
        let mut scenario = basic_replay_scenario(1);
        scenario.initial_stores = vec![BioSimInitialStore::new(
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            f64::MAX / 4.0,
        )];
        let process = oxygen_sink(f64::MAX / 30.0);
        let error = run_biosim_scenario(&scenario, &[process]).expect_err("underflow expected");
        assert!(matches!(
            error,
            BioSimScenarioReplayError::CellUnderflow { deficit, .. } if deficit.is_finite()
        ));
    }

    #[test]
    fn finite_state_checks_reject_nonfinite_and_negative_amounts() {
        let mut scenario = basic_replay_scenario(1);
        scenario.initial_stores = vec![BioSimInitialStore::new(
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            f64::MAX,
        )];
        let process = BioSimProcess::source(
            process_id("huge_source"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            f64::MAX,
        )
        .expect("finite positive rate");
        assert!(matches!(
            run_biosim_scenario(&scenario, &[process]),
            Err(BioSimScenarioReplayError::ProcessPlanningFailed { .. })
                | Err(BioSimScenarioReplayError::CellAmountNotFiniteAfterCommit { .. })
        ));
        let bad_state = BTreeMap::from([(
            ScenarioCellKey {
                compartment_id: "crew_cabin".to_owned(),
                resource_kind: BioSimScenarioResourceKind::Oxygen,
            },
            -1.0,
        )]);
        assert!(matches!(
            validate_replay_state(&bad_state),
            Err(BioSimScenarioReplayError::NegativeCellAmount { .. })
        ));
    }

    #[test]
    fn signed_zero_is_normalized_in_cells_and_digest() {
        let mut negative_zero = BTreeMap::new();
        negative_zero.insert(
            ScenarioCellKey {
                compartment_id: "crew_cabin".to_owned(),
                resource_kind: BioSimScenarioResourceKind::Oxygen,
            },
            -0.0,
        );
        let mut positive_zero = BTreeMap::new();
        positive_zero.insert(
            ScenarioCellKey {
                compartment_id: "crew_cabin".to_owned(),
                resource_kind: BioSimScenarioResourceKind::Oxygen,
            },
            0.0,
        );
        assert_eq!(
            scenario_cell_state_digest(0, &negative_zero).expect("digest"),
            scenario_cell_state_digest(0, &positive_zero).expect("digest")
        );
    }

    #[test]
    fn replay_limit_helpers_are_nonallocating_for_boundaries() {
        assert_eq!(
            checked_replay_cell_capacity_for_test(
                max_biosim_plan_compartments(),
                biosim_scenario_resource_kind_count()
            ),
            Ok(max_biosim_replay_cells())
        );
        assert!(matches!(
            checked_replay_cell_capacity_for_test(
                max_biosim_plan_compartments() + 1,
                biosim_scenario_resource_kind_count()
            ),
            Err(BioSimScenarioReplayError::ReplayCellLimitExceeded { .. })
        ));
        assert_eq!(
            checked_total_replay_events(max_biosim_replay_events() - 1, 1),
            Ok(max_biosim_replay_events())
        );
        assert!(matches!(
            checked_total_replay_events(max_biosim_replay_events(), 1),
            Err(BioSimScenarioReplayError::ReplayAllocationFailed { .. })
        ));
        assert!(matches!(
            checked_total_replay_events(usize::MAX, 1),
            Err(BioSimScenarioReplayError::ReplayCountOverflow { .. })
        ));
        assert!(matches!(
            try_reserve_replay_capacity_for_test(usize::MAX),
            Err(BioSimScenarioReplayError::ReplayAllocationFailed { .. })
        ));
    }

    #[test]
    fn digest_is_compact_versioned_and_sensitive_to_inputs() {
        let mut state = BTreeMap::new();
        state.insert(
            ScenarioCellKey {
                compartment_id: "crew_cabin".to_owned(),
                resource_kind: BioSimScenarioResourceKind::Oxygen,
            },
            1.0,
        );
        let base = scenario_cell_state_digest(0, &state).expect("base digest");
        assert!(digest_is_compact(&base));
        assert_eq!(
            biosim_scenario_cell_digest_algorithm(),
            "fnv1a64:biosim_scenario_cells:v1"
        );
        assert_ne!(
            base,
            scenario_cell_state_digest(1, &state).expect("tick sensitive")
        );
        let mut other_compartment = BTreeMap::new();
        other_compartment.insert(
            ScenarioCellKey {
                compartment_id: "buffer_store".to_owned(),
                resource_kind: BioSimScenarioResourceKind::Oxygen,
            },
            1.0,
        );
        assert_ne!(
            base,
            scenario_cell_state_digest(0, &other_compartment).expect("compartment sensitive")
        );
        let mut other_resource = BTreeMap::new();
        other_resource.insert(
            ScenarioCellKey {
                compartment_id: "crew_cabin".to_owned(),
                resource_kind: BioSimScenarioResourceKind::Water,
            },
            1.0,
        );
        assert_ne!(
            base,
            scenario_cell_state_digest(0, &other_resource).expect("resource sensitive")
        );
        state.insert(
            ScenarioCellKey {
                compartment_id: "crew_cabin".to_owned(),
                resource_kind: BioSimScenarioResourceKind::CarbonDioxide,
            },
            1.0,
        );
        assert_ne!(
            base,
            scenario_cell_state_digest(0, &state).expect("cell-count sensitive")
        );
    }

    #[test]
    fn replay_public_records_are_not_constructed_with_default() {
        fn assert_no_default<T>() {}
        assert_no_default::<BioSimResourceCell>();
        assert_no_default::<BioSimScenarioReplayEvent>();
        assert_no_default::<BioSimScenarioTickSummary>();
        assert_no_default::<BioSimScenarioReplay>();
    }
    #[test]
    fn process_id_accepts_stable_lower_snake_forms() {
        assert_eq!(process_id("p").as_str(), "p");
        let max_len = "a".repeat(64);
        assert_eq!(
            BioSimProcessId::new(max_len.clone())
                .expect("64-byte process ID")
                .as_str(),
            max_len
        );
        assert_eq!(process_id("process__one_").as_str(), "process__one_");
    }

    #[test]
    fn process_id_rejects_noncanonical_forms() {
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
            assert!(BioSimProcessId::new(raw).is_err(), "accepted {raw:?}");
        }
    }

    #[test]
    fn positive_finite_input_and_output_flows_are_constructed() {
        let input = BioSimFlow::input(
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            1.0,
        )
        .expect("valid input flow");
        let output = BioSimFlow::output(
            compartment("plant_bed"),
            BioSimScenarioResourceKind::Oxygen,
            2.0,
        )
        .expect("valid output flow");
        assert_eq!(input.direction(), BioSimFlowDirection::Input);
        assert_eq!(output.direction(), BioSimFlowDirection::Output);
        assert_eq!(input.rate_per_second(), 1.0);
    }

    #[test]
    fn nonpositive_or_nonfinite_rates_are_rejected_without_fake_process_id() {
        for bad_rate in [
            0.0,
            -1.0,
            f64::from_bits(0x7ff8_0000_0000_0000),
            f64::from_bits(0x7ff0_0000_0000_0000),
            f64::from_bits(0xfff0_0000_0000_0000),
        ] {
            let error = BioSimFlow::input(
                compartment("crew_cabin"),
                BioSimScenarioResourceKind::Oxygen,
                bad_rate,
            )
            .expect_err("invalid flow rate");
            assert!(matches!(
                error,
                BioSimProcessValidationError::InvalidFlowRate { .. }
            ));
            let debug = format!("{error:?}");
            assert!(!debug.contains("flow"));
        }
    }

    #[test]
    fn valid_source_sink_transfer_and_transform_constructors_work() {
        assert_eq!(source_process().kind(), BioSimProcessKind::Source);
        assert_eq!(sink_process().kind(), BioSimProcessKind::Sink);
        assert_eq!(transfer_process().kind(), BioSimProcessKind::Transfer);
        assert_eq!(
            transform_process_with_order(false).kind(),
            BioSimProcessKind::Transform
        );
    }

    #[test]
    fn source_and_sink_shape_mismatches_are_rejected_by_planner_revalidation() {
        let invalid_source = invalid_process_for_test(
            process_id("bad_source"),
            BioSimProcessKind::Source,
            vec![BioSimFlow::input(
                compartment("crew_cabin"),
                BioSimScenarioResourceKind::Oxygen,
                1.0,
            )
            .expect("valid flow")],
        );
        assert_error(
            plan_biosim_tick_intents(&[invalid_source], &known_compartments(), clock(), 0),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::SourceDirectionMismatch { .. }
                )
            },
        );

        let invalid_sink = invalid_process_for_test(
            process_id("bad_sink"),
            BioSimProcessKind::Sink,
            vec![BioSimFlow::output(
                compartment("crew_cabin"),
                BioSimScenarioResourceKind::Oxygen,
                1.0,
            )
            .expect("valid flow")],
        );
        assert_error(
            plan_biosim_tick_intents(&[invalid_sink], &known_compartments(), clock(), 0),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::SinkDirectionMismatch { .. }
                )
            },
        );
    }

    #[test]
    fn transfer_shape_rules_reject_wrong_count_compartment_resource_and_rate() {
        let wrong_count = invalid_process_for_test(
            process_id("bad_transfer_count"),
            BioSimProcessKind::Transfer,
            vec![BioSimFlow::input(
                compartment("crew_cabin"),
                BioSimScenarioResourceKind::Water,
                1.0,
            )
            .expect("valid flow")],
        );
        assert_error(
            plan_biosim_tick_intents(&[wrong_count], &known_compartments(), clock(), 0),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::TransferFlowCount { .. }
                )
            },
        );
        assert!(matches!(
            BioSimProcess::transfer(
                process_id("same_transfer"),
                compartment("crew_cabin"),
                compartment("crew_cabin"),
                BioSimScenarioResourceKind::Water,
                1.0,
            ),
            Err(BioSimProcessValidationError::TransferSameCompartment { .. })
        ));
        let resource_mismatch = invalid_process_for_test(
            process_id("bad_transfer_resource"),
            BioSimProcessKind::Transfer,
            vec![
                BioSimFlow::input(
                    compartment("water_tank"),
                    BioSimScenarioResourceKind::Water,
                    1.0,
                )
                .expect("valid flow"),
                BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Food,
                    1.0,
                )
                .expect("valid flow"),
            ],
        );
        assert_error(
            plan_biosim_tick_intents(&[resource_mismatch], &known_compartments(), clock(), 0),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::TransferResourceMismatch { .. }
                )
            },
        );
        let rate_mismatch = invalid_process_for_test(
            process_id("bad_transfer_rate"),
            BioSimProcessKind::Transfer,
            vec![
                BioSimFlow::input(
                    compartment("water_tank"),
                    BioSimScenarioResourceKind::Water,
                    1.0,
                )
                .expect("valid flow"),
                BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Water,
                    2.0,
                )
                .expect("valid flow"),
            ],
        );
        assert_error(
            plan_biosim_tick_intents(&[rate_mismatch], &known_compartments(), clock(), 0),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::TransferRateMismatch { .. }
                )
            },
        );
    }

    #[test]
    fn transform_rules_reject_missing_or_mismatched_flow_shapes() {
        assert!(matches!(
            BioSimProcess::transform(
                process_id("missing_input"),
                Vec::new(),
                vec![BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Oxygen,
                    1.0,
                )
                .expect("valid flow")],
            ),
            Err(BioSimProcessValidationError::TransformMissingInput { .. })
        ));
        assert!(matches!(
            BioSimProcess::transform(
                process_id("missing_output"),
                vec![BioSimFlow::input(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Water,
                    1.0,
                )
                .expect("valid flow")],
                Vec::new(),
            ),
            Err(BioSimProcessValidationError::TransformMissingOutput { .. })
        ));
        assert!(matches!(
            BioSimProcess::transform(
                process_id("bad_input_direction"),
                vec![BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Water,
                    1.0,
                )
                .expect("valid flow")],
                vec![BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Oxygen,
                    1.0,
                )
                .expect("valid flow")],
            ),
            Err(BioSimProcessValidationError::TransformArgumentDirectionMismatch { .. })
        ));
        assert!(matches!(
            BioSimProcess::transform(
                process_id("cross_compartment"),
                vec![BioSimFlow::input(
                    compartment("crew_cabin"),
                    BioSimScenarioResourceKind::Water,
                    1.0,
                )
                .expect("valid flow")],
                vec![BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Oxygen,
                    1.0,
                )
                .expect("valid flow")],
            ),
            Err(BioSimProcessValidationError::TransformCrossCompartment { .. })
        ));
    }

    #[test]
    fn duplicate_flow_cell_and_same_cell_resource_transform_are_rejected() {
        assert!(matches!(
            BioSimProcess::transform(
                process_id("duplicate_cell"),
                vec![
                    BioSimFlow::input(
                        compartment("plant_bed"),
                        BioSimScenarioResourceKind::Water,
                        1.0,
                    )
                    .expect("valid flow"),
                    BioSimFlow::input(
                        compartment("plant_bed"),
                        BioSimScenarioResourceKind::Water,
                        2.0,
                    )
                    .expect("valid flow"),
                ],
                vec![BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Oxygen,
                    1.0,
                )
                .expect("valid flow")],
            ),
            Err(BioSimProcessValidationError::DuplicateFlowCell { .. })
        ));
        assert!(matches!(
            BioSimProcess::transform(
                process_id("same_cell_resource"),
                vec![BioSimFlow::input(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Water,
                    1.0,
                )
                .expect("valid flow")],
                vec![BioSimFlow::output(
                    compartment("plant_bed"),
                    BioSimScenarioResourceKind::Water,
                    1.0,
                )
                .expect("valid flow")],
            ),
            Err(BioSimProcessValidationError::TransformSameCellResource { .. })
        ));
    }

    #[test]
    fn planner_outputs_expected_intent_kinds_and_amounts() {
        let processes = vec![
            sink_process(),
            source_process(),
            transfer_process(),
            transform_process_with_order(false),
        ];
        let plan = plan_biosim_tick_intents(&processes, &known_compartments(), clock(), 2)
            .expect("valid plan");
        assert_eq!(plan.tick_index(), 2);
        assert_eq!(plan.tick_duration_seconds(), 10.0);
        assert_eq!(plan.process_count(), 4);
        assert_eq!(plan.intents().len(), 8);
        assert!(plan.intents().iter().all(|intent| intent.tick_index() == 2));
        assert!(plan
            .intents()
            .iter()
            .enumerate()
            .all(|(index, intent)| intent.sequence_index() == index));
        assert!(plan
            .intents()
            .iter()
            .all(|intent| intent.amount().is_finite() && intent.amount() > 0.0));
        assert!(plan
            .intents()
            .iter()
            .any(|intent| intent.kind() == BioSimTickIntentKind::Source));
        assert!(plan
            .intents()
            .iter()
            .any(|intent| intent.kind() == BioSimTickIntentKind::Sink));
        assert!(plan
            .intents()
            .iter()
            .any(|intent| intent.kind() == BioSimTickIntentKind::Consume));
        assert!(plan
            .intents()
            .iter()
            .any(|intent| intent.kind() == BioSimTickIntentKind::Produce));
        assert!(plan
            .intents()
            .iter()
            .any(|intent| intent.resource_kind() == BioSimScenarioResourceKind::Food));
        assert!(plan.intents().iter().all(|intent| intent.unit() == "kg"));
    }

    #[test]
    fn planner_rejects_duplicate_processes_unknown_compartments_and_empty_internal_flows() {
        assert_error(
            plan_biosim_tick_intents(
                &[source_process(), source_process()],
                &known_compartments(),
                clock(),
                0,
            ),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::DuplicateProcessId { .. }
                )
            },
        );
        let unknown = BioSimProcess::source(
            process_id("unknown_compartment"),
            compartment("unknown_compartment"),
            BioSimScenarioResourceKind::Oxygen,
            1.0,
        )
        .expect("constructor only checks process shape");
        assert_error(
            plan_biosim_tick_intents(&[unknown], &known_compartments(), clock(), 0),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::UnknownFlowCompartment { .. }
                )
            },
        );
        let no_flows = invalid_process_for_test(
            process_id("no_flows"),
            BioSimProcessKind::Source,
            Vec::new(),
        );
        assert_error(
            plan_biosim_tick_intents(&[no_flows], &known_compartments(), clock(), 0),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::ProcessWithNoFlows { .. }
                )
            },
        );
    }

    #[test]
    fn planner_rejects_invalid_clock_and_tick_indices_before_other_errors() {
        let processes = [source_process(), source_process()];
        for bad_duration in [
            0.0,
            -1.0,
            f64::from_bits(0x7ff8_0000_0000_0000),
            f64::from_bits(0x7ff0_0000_0000_0000),
        ] {
            assert_error(
                plan_biosim_tick_intents(
                    &processes,
                    &known_compartments(),
                    BioSimScenarioClock::new(bad_duration, 1),
                    0,
                ),
                |error| {
                    matches!(
                        error,
                        BioSimProcessValidationError::InvalidTickDurationSeconds { .. }
                    )
                },
            );
        }
        assert_error(
            plan_biosim_tick_intents(
                &processes,
                &known_compartments(),
                BioSimScenarioClock::new(1.0, 0),
                0,
            ),
            |error| matches!(error, BioSimProcessValidationError::InvalidTickCount { .. }),
        );
        assert_error(
            plan_biosim_tick_intents(
                &processes,
                &known_compartments(),
                BioSimScenarioClock::new(1.0, 1),
                1,
            ),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::TickIndexOutOfRange { .. }
                )
            },
        );
    }

    #[test]
    fn tick_amount_overflow_and_underflow_are_rejected() {
        let huge_rate = f64::from_bits(0x7fefffffffffffff);
        let overflow = BioSimProcess::source(
            process_id("overflow_amount"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            huge_rate,
        )
        .expect("finite source rate");
        assert_error(
            plan_biosim_tick_intents(
                &[overflow],
                &known_compartments(),
                BioSimScenarioClock::new(2.0, 1),
                0,
            ),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::TickAmountNotFinite { .. }
                )
            },
        );

        let tiny = BioSimProcess::source(
            process_id("tiny_amount"),
            compartment("crew_cabin"),
            BioSimScenarioResourceKind::Oxygen,
            f64::from_bits(1),
        )
        .expect("positive finite source rate");
        assert_error(
            plan_biosim_tick_intents(
                &[tiny],
                &known_compartments(),
                BioSimScenarioClock::new(f64::from_bits(1), 1),
                0,
            ),
            |error| {
                matches!(
                    error,
                    BioSimProcessValidationError::TickAmountNotPositive { .. }
                )
            },
        );
    }

    #[test]
    fn b2c_replay_integrity_accepts_authoritative_b2b2_replay() {
        let scenario = basic_replay_scenario(3);
        let replay = run_biosim_scenario(&scenario, &[supported_transfer_process()])
            .expect("authoritative replay");
        let report = validate_biosim_scenario_replay_integrity(&replay).expect("integrity report");
        assert_eq!(report.tick_count(), 3);
        assert_eq!(report.tick_summary_count(), 3);
        assert_eq!(report.event_count(), 6);
        assert_eq!(report.total_clamp_amount(), 0.0);
        assert!(report.total_committed_amount() > 0.0);
    }

    #[test]
    fn b2c_replay_integrity_fails_closed_on_bad_tick_count() {
        let scenario = basic_replay_scenario(2);
        let mut replay = run_biosim_scenario(&scenario, &[supported_transfer_process()])
            .expect("authoritative replay");
        replay.tick_count = 3;
        assert!(matches!(
            validate_biosim_scenario_replay_integrity(&replay),
            Err(BioSimScenarioReplayIntegrityError::TickSummaryCountMismatch { .. })
        ));
    }

    #[test]
    fn b2c_replay_integrity_fails_closed_on_event_arithmetic_drift() {
        let scenario = basic_replay_scenario(1);
        let mut replay = run_biosim_scenario(&scenario, &[supported_transfer_process()])
            .expect("authoritative replay");
        replay.replay_events[0].after_amount += 1.0;
        assert!(matches!(
            validate_biosim_scenario_replay_integrity(&replay),
            Err(BioSimScenarioReplayIntegrityError::EventArithmeticMismatch { .. })
        ));
    }

    #[test]
    fn planning_order_is_independent_of_process_compartment_and_transform_input_order() {
        let ordered = vec![
            source_process(),
            sink_process(),
            transfer_process(),
            transform_process_with_order(false),
        ];
        let mut reordered = ordered.clone();
        reordered.reverse();
        let known = known_compartments();
        let mut known_reordered = known.clone();
        known_reordered.reverse();
        let plan_a = plan_biosim_tick_intents(&ordered, &known, clock(), 1).expect("plan a");
        let plan_b =
            plan_biosim_tick_intents(&reordered, &known_reordered, clock(), 1).expect("plan b");
        assert_eq!(plan_a, plan_b);

        let with_transform_a = vec![transform_process_with_order(false)];
        let with_transform_b = vec![transform_process_with_order(true)];
        let flow_order_a = plan_biosim_tick_intents(&with_transform_a, &known, clock(), 1)
            .expect("plan transform a");
        let flow_order_b = plan_biosim_tick_intents(&with_transform_b, &known, clock(), 1)
            .expect("plan transform b");
        assert_eq!(flow_order_a, flow_order_b);
    }

    #[test]
    fn planning_is_repeatable_and_does_not_mutate_inputs_on_success_or_failure() {
        let processes = vec![source_process(), transfer_process()];
        let original = processes.clone();
        let plan_a = plan_biosim_tick_intents(&processes, &known_compartments(), clock(), 0)
            .expect("first plan");
        let plan_b = plan_biosim_tick_intents(&processes, &known_compartments(), clock(), 0)
            .expect("second plan");
        assert_eq!(plan_a, plan_b);
        assert_eq!(processes, original);

        let mut invalid = processes.clone();
        invalid.push(
            BioSimProcess::source(
                process_id("unknown_compartment"),
                compartment("unknown_compartment"),
                BioSimScenarioResourceKind::Oxygen,
                1.0,
            )
            .expect("shape-valid source"),
        );
        let invalid_original = invalid.clone();
        assert!(plan_biosim_tick_intents(&invalid, &known_compartments(), clock(), 0).is_err());
        assert_eq!(invalid, invalid_original);
    }

    #[test]
    fn empty_process_list_is_an_empty_plan_after_clock_and_compartment_validation() {
        let plan = plan_biosim_tick_intents(&[], &known_compartments(), clock(), 0)
            .expect("empty process plan");
        assert_eq!(plan.process_count(), 0);
        assert!(plan.intents().is_empty());
    }

    #[test]
    fn duplicate_known_compartment_ids_are_rejected() {
        let known = vec![compartment("crew_cabin"), compartment("crew_cabin")];
        assert_error(plan_biosim_tick_intents(&[], &known, clock(), 0), |error| {
            matches!(
                error,
                BioSimProcessValidationError::DuplicateKnownCompartmentId { .. }
            )
        });
    }

    #[test]
    fn resource_limits_are_checked_without_large_allocations() {
        assert!(matches!(
            validate_known_compartment_limit(max_biosim_plan_compartments()),
            Ok(())
        ));
        assert!(matches!(
            validate_known_compartment_limit(max_biosim_plan_compartments() + 1),
            Err(BioSimProcessValidationError::TooManyCompartments { .. })
        ));
        assert!(matches!(
            validate_process_limit(max_biosim_plan_processes()),
            Ok(())
        ));
        assert!(matches!(
            validate_process_limit(max_biosim_plan_processes() + 1),
            Err(BioSimProcessValidationError::TooManyProcesses { .. })
        ));
        let max_flows = (0..max_biosim_flows_per_process())
            .map(|index| {
                BioSimFlow::input(
                    compartment("crew_cabin"),
                    match index % 2 {
                        0 => BioSimScenarioResourceKind::Oxygen,
                        _ => BioSimScenarioResourceKind::CarbonDioxide,
                    },
                    (index + 1) as f64,
                )
                .expect("valid flow")
            })
            .collect::<Vec<_>>();
        let too_many = invalid_process_for_test(
            process_id("too_many_flows"),
            BioSimProcessKind::Transform,
            {
                let mut flows = max_flows.clone();
                flows.push(
                    BioSimFlow::output(
                        compartment("crew_cabin"),
                        BioSimScenarioResourceKind::Water,
                        1.0,
                    )
                    .expect("valid flow"),
                );
                flows
            },
        );
        assert!(matches!(
            validate_process_shape(&too_many),
            Err(BioSimProcessValidationError::TooManyFlows { .. })
        ));
        assert_eq!(
            checked_total_intent_count([max_biosim_intents_per_plan()]),
            Ok(max_biosim_intents_per_plan())
        );
        assert!(matches!(
            checked_total_intent_count([max_biosim_intents_per_plan(), 1]),
            Err(BioSimProcessValidationError::TooManyIntents { .. })
        ));
        assert!(matches!(
            checked_total_intent_count([usize::MAX, 1]),
            Err(BioSimProcessValidationError::TooManyIntents { .. })
        ));
        assert!(matches!(
            try_reserve_capacity_overflow_for_test(),
            Err(BioSimProcessValidationError::IntentAllocationFailed { .. })
        ));
    }
}

/// B2c fail-closed integrity summary for a successful B2b-2 replay.
///
/// The report is derived only from committed replay records. It is not a
/// persistent ledger, external parity proof, biological model, command surface,
/// source import, or regulated-use claim.
#[derive(Debug, Clone, PartialEq)]
pub struct BioSimScenarioReplayIntegrityReport {
    tick_count: u64,
    tick_summary_count: usize,
    event_count: usize,
    final_cell_count: usize,
    total_committed_amount: f64,
    total_clamp_amount: f64,
}

impl BioSimScenarioReplayIntegrityReport {
    /// Returns the committed replay tick count.
    #[must_use]
    pub const fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Returns the number of tick summaries inspected.
    #[must_use]
    pub const fn tick_summary_count(&self) -> usize {
        self.tick_summary_count
    }

    /// Returns the number of committed replay events inspected.
    #[must_use]
    pub const fn event_count(&self) -> usize {
        self.event_count
    }

    /// Returns the number of final state cells inspected.
    #[must_use]
    pub const fn final_cell_count(&self) -> usize {
        self.final_cell_count
    }

    /// Returns the unsigned sum of committed replay-event amounts.
    #[must_use]
    pub const fn total_committed_amount(&self) -> f64 {
        self.total_committed_amount
    }

    /// Returns the unsigned sum of explicitly clamped requested amounts.
    #[must_use]
    pub const fn total_clamp_amount(&self) -> f64 {
        self.total_clamp_amount
    }
}

/// B2c fail-closed replay-integrity validation errors.
#[derive(Debug, Clone, PartialEq)]
pub enum BioSimScenarioReplayIntegrityError {
    TickSummaryCountMismatch {
        tick_count: u64,
        summary_count: usize,
    },
    TickSummaryIndexMismatch {
        expected_tick_index: u64,
        actual_tick_index: u64,
    },
    InitialDigestMismatch,
    DigestChainMismatch {
        tick_index: u64,
    },
    FinalDigestMismatch,
    EventTickOutOfRange {
        tick_index: u64,
        tick_count: u64,
    },
    EventSequenceMismatch {
        tick_index: u64,
        expected_sequence_index: usize,
        actual_sequence_index: usize,
    },
    TickEventCountMismatch {
        tick_index: u64,
        expected_event_count: usize,
        actual_event_count: usize,
    },
    EventAmountInvalid {
        tick_index: u64,
        sequence_index: usize,
        field: &'static str,
        value: f64,
    },
    EventUnitMismatch {
        tick_index: u64,
        sequence_index: usize,
        expected_unit: &'static str,
        actual_unit: &'static str,
    },
    EventArithmeticMismatch {
        tick_index: u64,
        sequence_index: usize,
    },
    EventCellContinuityMismatch {
        tick_index: u64,
        sequence_index: usize,
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        expected_before_amount: f64,
        actual_before_amount: f64,
    },
    DuplicateFinalCell {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
    },
    FinalCellAmountInvalid {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        amount: f64,
    },
    FinalCellUnitMismatch {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        expected_unit: &'static str,
        actual_unit: &'static str,
    },
    MissingFinalCellForEvent {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
    },
    FinalCellMismatch {
        compartment_id: String,
        resource_kind: BioSimScenarioResourceKind,
        expected_amount: f64,
        actual_amount: f64,
    },
}

/// Validates that a B2b-2 replay is internally self-consistent before B2c reporting.
///
/// The check consumes immutable replay records, never rebuilds state from external
/// input, and fails closed on digest-chain, event-order, event-arithmetic,
/// clamp-accounting, unit, continuity, or final-state inconsistencies.
pub fn validate_biosim_scenario_replay_integrity(
    replay: &BioSimScenarioReplay,
) -> Result<BioSimScenarioReplayIntegrityReport, BioSimScenarioReplayIntegrityError> {
    let tick_count = replay.tick_count();
    let tick_summaries = replay.tick_summaries();
    if u64::try_from(tick_summaries.len()).unwrap_or(u64::MAX) != tick_count {
        return Err(
            BioSimScenarioReplayIntegrityError::TickSummaryCountMismatch {
                tick_count,
                summary_count: tick_summaries.len(),
            },
        );
    }

    for (index, summary) in tick_summaries.iter().enumerate() {
        let expected_tick_index = u64::try_from(index).unwrap_or(u64::MAX);
        if summary.tick_index() != expected_tick_index {
            return Err(
                BioSimScenarioReplayIntegrityError::TickSummaryIndexMismatch {
                    expected_tick_index,
                    actual_tick_index: summary.tick_index(),
                },
            );
        }
        if index == 0 && summary.before_cell_state_digest() != replay.initial_cell_state_digest() {
            return Err(BioSimScenarioReplayIntegrityError::InitialDigestMismatch);
        }
        if index > 0 {
            let previous = &tick_summaries[index - 1];
            if summary.before_cell_state_digest() != previous.after_cell_state_digest() {
                return Err(BioSimScenarioReplayIntegrityError::DigestChainMismatch {
                    tick_index: summary.tick_index(),
                });
            }
        }
        if index + 1 == tick_summaries.len()
            && summary.after_cell_state_digest() != replay.final_cell_state_digest()
        {
            return Err(BioSimScenarioReplayIntegrityError::FinalDigestMismatch);
        }
    }

    let mut event_counts = vec![0usize; tick_summaries.len()];
    let mut next_sequence_indexes = vec![0usize; tick_summaries.len()];
    let mut last_amounts = BTreeMap::<ScenarioCellKey, f64>::new();
    let mut total_committed_amount = 0.0;
    let mut total_clamp_amount = 0.0;

    for event in replay.replay_events() {
        if event.tick_index() >= tick_count {
            return Err(BioSimScenarioReplayIntegrityError::EventTickOutOfRange {
                tick_index: event.tick_index(),
                tick_count,
            });
        }
        let tick_index = usize::try_from(event.tick_index()).map_err(|_| {
            BioSimScenarioReplayIntegrityError::EventTickOutOfRange {
                tick_index: event.tick_index(),
                tick_count,
            }
        })?;
        let expected_sequence_index = next_sequence_indexes[tick_index];
        if event.sequence_index() != expected_sequence_index {
            return Err(BioSimScenarioReplayIntegrityError::EventSequenceMismatch {
                tick_index: event.tick_index(),
                expected_sequence_index,
                actual_sequence_index: event.sequence_index(),
            });
        }
        next_sequence_indexes[tick_index] = expected_sequence_index.checked_add(1).ok_or(
            BioSimScenarioReplayIntegrityError::EventSequenceMismatch {
                tick_index: event.tick_index(),
                expected_sequence_index,
                actual_sequence_index: event.sequence_index(),
            },
        )?;
        event_counts[tick_index] = event_counts[tick_index].checked_add(1).ok_or(
            BioSimScenarioReplayIntegrityError::TickEventCountMismatch {
                tick_index: event.tick_index(),
                expected_event_count: tick_summaries[tick_index].event_count(),
                actual_event_count: event_counts[tick_index],
            },
        )?;

        validate_integrity_event_amount(
            event.tick_index(),
            event.sequence_index(),
            "requested_amount",
            event.requested_amount(),
            true,
        )?;
        validate_integrity_event_amount(
            event.tick_index(),
            event.sequence_index(),
            "committed_amount",
            event.committed_amount(),
            false,
        )?;
        validate_integrity_event_amount(
            event.tick_index(),
            event.sequence_index(),
            "clamp_amount",
            event.clamp_amount(),
            false,
        )?;
        validate_integrity_event_amount(
            event.tick_index(),
            event.sequence_index(),
            "before_amount",
            event.before_amount(),
            false,
        )?;
        validate_integrity_event_amount(
            event.tick_index(),
            event.sequence_index(),
            "after_amount",
            event.after_amount(),
            false,
        )?;

        let expected_unit = biosim_scenario_resource_kind_unit(event.resource_kind());
        if event.unit() != expected_unit {
            return Err(BioSimScenarioReplayIntegrityError::EventUnitMismatch {
                tick_index: event.tick_index(),
                sequence_index: event.sequence_index(),
                expected_unit,
                actual_unit: event.unit(),
            });
        }

        let key = ScenarioCellKey {
            compartment_id: event.compartment_id().as_str().to_owned(),
            resource_kind: event.resource_kind(),
        };
        if let Some(expected_before_amount) = last_amounts.get(&key) {
            if !biosim_integrity_amounts_match(*expected_before_amount, event.before_amount()) {
                return Err(
                    BioSimScenarioReplayIntegrityError::EventCellContinuityMismatch {
                        tick_index: event.tick_index(),
                        sequence_index: event.sequence_index(),
                        compartment_id: key.compartment_id.clone(),
                        resource_kind: key.resource_kind,
                        expected_before_amount: *expected_before_amount,
                        actual_before_amount: event.before_amount(),
                    },
                );
            }
        }

        let arithmetic_ok = match event.intent_kind() {
            BioSimTickIntentKind::Produce | BioSimTickIntentKind::Source => {
                biosim_integrity_amounts_match(event.committed_amount(), event.requested_amount())
                    && biosim_integrity_amounts_match(event.clamp_amount(), 0.0)
                    && biosim_integrity_amounts_match(
                        event.after_amount(),
                        event.before_amount() + event.committed_amount(),
                    )
            }
            BioSimTickIntentKind::Consume | BioSimTickIntentKind::Sink => {
                biosim_integrity_amounts_match(
                    event.requested_amount(),
                    event.committed_amount() + event.clamp_amount(),
                ) && biosim_integrity_amounts_match(
                    event.after_amount(),
                    event.before_amount() - event.committed_amount(),
                )
            }
        };
        if !arithmetic_ok {
            return Err(
                BioSimScenarioReplayIntegrityError::EventArithmeticMismatch {
                    tick_index: event.tick_index(),
                    sequence_index: event.sequence_index(),
                },
            );
        }

        total_committed_amount += event.committed_amount();
        total_clamp_amount += event.clamp_amount();
        last_amounts.insert(key, event.after_amount());
    }

    for (index, summary) in tick_summaries.iter().enumerate() {
        let actual_event_count = event_counts[index];
        if summary.event_count() != actual_event_count {
            return Err(BioSimScenarioReplayIntegrityError::TickEventCountMismatch {
                tick_index: summary.tick_index(),
                expected_event_count: summary.event_count(),
                actual_event_count,
            });
        }
    }

    let mut final_amounts = BTreeMap::<ScenarioCellKey, f64>::new();
    for cell in replay.final_cells() {
        if !cell.amount().is_finite() || cell.amount() < 0.0 {
            return Err(BioSimScenarioReplayIntegrityError::FinalCellAmountInvalid {
                compartment_id: cell.compartment_id().as_str().to_owned(),
                resource_kind: cell.resource_kind(),
                amount: cell.amount(),
            });
        }
        let expected_unit = biosim_scenario_resource_kind_unit(cell.resource_kind());
        if cell.unit() != expected_unit {
            return Err(BioSimScenarioReplayIntegrityError::FinalCellUnitMismatch {
                compartment_id: cell.compartment_id().as_str().to_owned(),
                resource_kind: cell.resource_kind(),
                expected_unit,
                actual_unit: cell.unit(),
            });
        }
        let key = ScenarioCellKey {
            compartment_id: cell.compartment_id().as_str().to_owned(),
            resource_kind: cell.resource_kind(),
        };
        if final_amounts.insert(key.clone(), cell.amount()).is_some() {
            return Err(BioSimScenarioReplayIntegrityError::DuplicateFinalCell {
                compartment_id: key.compartment_id,
                resource_kind: key.resource_kind,
            });
        }
    }

    for (key, expected_amount) in last_amounts {
        let Some(actual_amount) = final_amounts.get(&key) else {
            return Err(
                BioSimScenarioReplayIntegrityError::MissingFinalCellForEvent {
                    compartment_id: key.compartment_id,
                    resource_kind: key.resource_kind,
                },
            );
        };
        if !biosim_integrity_amounts_match(expected_amount, *actual_amount) {
            return Err(BioSimScenarioReplayIntegrityError::FinalCellMismatch {
                compartment_id: key.compartment_id,
                resource_kind: key.resource_kind,
                expected_amount,
                actual_amount: *actual_amount,
            });
        }
    }

    Ok(BioSimScenarioReplayIntegrityReport {
        tick_count,
        tick_summary_count: tick_summaries.len(),
        event_count: replay.replay_events().len(),
        final_cell_count: replay.final_cells().len(),
        total_committed_amount,
        total_clamp_amount,
    })
}

fn validate_integrity_event_amount(
    tick_index: u64,
    sequence_index: usize,
    field: &'static str,
    value: f64,
    require_positive: bool,
) -> Result<(), BioSimScenarioReplayIntegrityError> {
    if !value.is_finite() || value < 0.0 || (require_positive && value <= 0.0) {
        Err(BioSimScenarioReplayIntegrityError::EventAmountInvalid {
            tick_index,
            sequence_index,
            field,
            value,
        })
    } else {
        Ok(())
    }
}

fn biosim_integrity_amounts_match(left: f64, right: f64) -> bool {
    let scale = left.abs().max(right.abs()).max(1.0);
    (left - right).abs() <= 1.0e-9 * scale
}
