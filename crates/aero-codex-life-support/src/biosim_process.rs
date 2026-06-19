use std::collections::{BTreeMap, BTreeSet};

use crate::biosim_scenario::{
    biosim_scenario_resource_kind_unit, BioSimCompartmentId, BioSimProcessId, BioSimScenarioClock,
    BioSimScenarioResourceKind,
};

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
