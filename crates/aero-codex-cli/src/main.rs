#![forbid(unsafe_code)]
//! User-facing AeroCodex Research Software Alpha command-line interface.
//!
//! This binary exposes deliberately bounded, machine-readable vertical slices
//! of the already governed M00 canonical-unit and angle-conversion families. It is research
//! and preliminary-design software, not operational or certified software.

use aero_codex_astrodynamics::{
    m00_canonical_mu_from_units, m00_canonical_speed_unit_from_du_tu,
    m00_canonical_speed_unit_from_mu_du, m00_canonical_time_unit_from_mu_du,
    m00_degrees_to_radians, m00_distance_from_canonical, m00_distance_to_canonical,
    m00_radians_to_degrees, m00_speed_from_canonical, m00_speed_to_canonical,
    m00_time_from_canonical, m00_time_to_canonical,
};
use aero_codex_core::AeroError;
use std::{
    collections::BTreeMap,
    env,
    fmt::{self, Write as _},
    io::{self, Write as IoWrite},
    process::ExitCode,
    sync::OnceLock,
};

#[path = "../../../generated/rust/formula_registry.rs"]
#[allow(clippy::manual_contains)]
mod generated_formula_registry;

const GENERATED_FORMULA_REGISTRY_JSON: &str =
    include_str!("../../../generated/formula_registry.json");

const PROGRAM_NAME: &str = "aerocodex";
const WORKSPACE_PACKAGE_COUNT: usize = 14;
const JSON_CONTRACT_VERSION: &str = "aerocodex.cli.json.v1";
const FORCE_SELF_CHECK_FAILURE_ENV: &str = "AEROCODEX_TEST_FORCE_SELF_CHECK_FAILURE";

fn release_tier() -> &'static str {
    "research_software_alpha"
}

fn release_tier_display() -> &'static str {
    "Research Software Alpha"
}

fn package_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn build_commit() -> &'static str {
    option_env!("AEROCODEX_BUILD_COMMIT").unwrap_or("unknown")
}

fn build_target() -> &'static str {
    option_env!("AEROCODEX_BUILD_TARGET").unwrap_or("unknown")
}

fn build_profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

fn validation_status() -> &'static str {
    "implementation_verified"
}

fn safety_notice() -> &'static str {
    "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use"
}

#[derive(Debug)]
struct FormulaSpec {
    canonical_id: &'static str,
    id: &'static str,
    runtime_symbol: &'static str,
    output_variable: &'static str,
    inputs: Vec<&'static str>,
    summary: &'static str,
    dispatch: RuntimeDispatch,
}

#[derive(Debug, Clone, Copy)]
enum RuntimeDispatch {
    CanonicalTimeUnitFromMuDu,
    CanonicalSpeedUnitFromDuTu,
    CanonicalSpeedUnitFromMuDu,
    CanonicalMuFromUnits,
    DistanceToCanonical,
    DistanceFromCanonical,
    TimeToCanonical,
    TimeFromCanonical,
    SpeedToCanonical,
    SpeedFromCanonical,
    DegreesToRadians,
    RadiansToDegrees,
}

fn runtime_dispatch(runtime_symbol: &str) -> Option<RuntimeDispatch> {
    match runtime_symbol {
        "m00_canonical_time_unit_from_mu_du" => Some(RuntimeDispatch::CanonicalTimeUnitFromMuDu),
        "m00_canonical_speed_unit_from_du_tu" => Some(RuntimeDispatch::CanonicalSpeedUnitFromDuTu),
        "m00_canonical_speed_unit_from_mu_du" => Some(RuntimeDispatch::CanonicalSpeedUnitFromMuDu),
        "m00_canonical_mu_from_units" => Some(RuntimeDispatch::CanonicalMuFromUnits),
        "m00_distance_to_canonical" => Some(RuntimeDispatch::DistanceToCanonical),
        "m00_distance_from_canonical" => Some(RuntimeDispatch::DistanceFromCanonical),
        "m00_time_to_canonical" => Some(RuntimeDispatch::TimeToCanonical),
        "m00_time_from_canonical" => Some(RuntimeDispatch::TimeFromCanonical),
        "m00_speed_to_canonical" => Some(RuntimeDispatch::SpeedToCanonical),
        "m00_speed_from_canonical" => Some(RuntimeDispatch::SpeedFromCanonical),
        "m00_degrees_to_radians" => Some(RuntimeDispatch::DegreesToRadians),
        "m00_radians_to_degrees" => Some(RuntimeDispatch::RadiansToDegrees),
        _ => None,
    }
}

fn formula_specs() -> &'static [FormulaSpec] {
    static SPECS: OnceLock<Vec<FormulaSpec>> = OnceLock::new();
    SPECS
        .get_or_init(|| {
            parse_dispatch_metadata(include_str!("../dispatch_metadata.tsv"))
                .expect("checked-in CLI dispatch metadata must be valid")
        })
        .as_slice()
}

fn parse_dispatch_metadata(text: &'static str) -> Result<Vec<FormulaSpec>, String> {
    const HEADER: &str = "schema_version\tcanonical_formula_id\tdispatch_formula_id\truntime_symbol\toutput_variable\tinputs\tsummary";
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("CLI dispatch metadata has an invalid header".to_string());
    }

    let mut specs = Vec::new();
    let mut canonical_ids = std::collections::BTreeSet::new();
    let mut dispatch_ids = std::collections::BTreeSet::new();
    let mut runtime_symbols = std::collections::BTreeSet::new();
    for (index, line) in lines.enumerate() {
        let line_number = index + 2;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 7 {
            return Err(format!(
                "CLI dispatch metadata line {line_number} has {} fields, expected 7",
                fields.len()
            ));
        }
        if fields.iter().any(|field| field.trim().is_empty()) {
            return Err(format!(
                "CLI dispatch metadata line {line_number} contains an empty field"
            ));
        }
        if fields[0] != "aerocodex.cli_dispatch.v1" {
            return Err(format!(
                "CLI dispatch metadata line {line_number} has unsupported schema `{}`",
                fields[0]
            ));
        }
        if !canonical_ids.insert(fields[1]) {
            return Err(format!("duplicate canonical formula ID `{}`", fields[1]));
        }
        if !dispatch_ids.insert(fields[2]) {
            return Err(format!("duplicate dispatch formula ID `{}`", fields[2]));
        }
        if !runtime_symbols.insert(fields[3]) {
            return Err(format!("duplicate runtime symbol `{}`", fields[3]));
        }
        let dispatch = runtime_dispatch(fields[3]).ok_or_else(|| {
            format!(
                "CLI dispatch metadata line {line_number} has unbound runtime symbol `{}`",
                fields[3]
            )
        })?;
        let inputs: Vec<&str> = fields[5].split(',').collect();
        if inputs.iter().any(|input| input.trim().is_empty()) {
            return Err(format!(
                "CLI dispatch metadata line {line_number} has an empty input name"
            ));
        }
        specs.push(FormulaSpec {
            canonical_id: fields[1],
            id: fields[2],
            runtime_symbol: fields[3],
            output_variable: fields[4],
            inputs,
            summary: fields[6],
            dispatch,
        });
    }
    if specs.is_empty() {
        return Err("CLI dispatch metadata has no formula records".to_string());
    }
    Ok(specs)
}

fn dispatchable_formula_count() -> usize {
    formula_specs().len()
}

#[derive(Debug)]
struct EvaluationResult {
    spec: &'static FormulaSpec,
    value: f64,
}

#[derive(Debug)]
struct PublicFormulaRun {
    resolved: ResolvedFormula,
    result: EvaluationResult,
    input_syntax: InputSyntax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputSyntax {
    FlagStyle,
    LegacyAssignment,
}

impl InputSyntax {
    fn as_str(self) -> &'static str {
        match self {
            Self::FlagStyle => "flag_style",
            Self::LegacyAssignment => "legacy_assignment",
        }
    }
}

#[derive(Debug)]
struct ParsedFormulaInputs {
    inputs: BTreeMap<String, f64>,
    syntax: InputSyntax,
}

#[derive(Debug)]
enum AppError {
    Usage(String),
    UnknownFormula(String),
    InvalidAssignment(String),
    DuplicateInput(String),
    InvalidNumber {
        input: String,
        value: String,
    },
    MissingInput {
        formula_id: &'static str,
        input: &'static str,
    },
    UnexpectedInput {
        formula_id: &'static str,
        input: String,
    },
    ExecutionBlockedByStatus {
        formula_id: String,
        status: &'static str,
        execution_policy: &'static str,
    },
    PreliminaryFlagRequired {
        formula_id: String,
        status: &'static str,
        execution_policy: &'static str,
    },
    M07CandidateBlocked {
        formula_id: String,
        status: &'static str,
        execution_policy: &'static str,
    },
    DispatchUnavailable {
        formula_id: String,
        status: &'static str,
        execution_policy: &'static str,
    },
    Equation {
        formula_id: &'static str,
        source: AeroError,
    },
    SelfCheckFailed {
        failed: usize,
    },
}

impl AppError {
    fn code(&self) -> &'static str {
        match self {
            Self::Usage(_) => "usage_error",
            Self::UnknownFormula(_) => "formula_not_found",
            Self::InvalidAssignment(_) => "invalid_assignment",
            Self::DuplicateInput(_) => "duplicate_input",
            Self::InvalidNumber { .. } => "invalid_number",
            Self::MissingInput { .. } => "missing_input",
            Self::UnexpectedInput { .. } => "unexpected_input",
            Self::ExecutionBlockedByStatus { .. } => "execution_blocked_by_status",
            Self::PreliminaryFlagRequired { .. } => "preliminary_flag_required",
            Self::M07CandidateBlocked { .. } => "m07_candidate_blocked",
            Self::DispatchUnavailable { .. } => "formula_dispatch_unavailable",
            Self::Equation { source, .. } => source.code(),
            Self::SelfCheckFailed { .. } => "self_check_failed",
        }
    }

    fn formula_id(&self) -> Option<&str> {
        match self {
            Self::UnknownFormula(formula_id) => Some(formula_id.as_str()),
            Self::MissingInput { formula_id, .. }
            | Self::UnexpectedInput { formula_id, .. }
            | Self::Equation { formula_id, .. } => Some(formula_id),
            Self::ExecutionBlockedByStatus { formula_id, .. }
            | Self::PreliminaryFlagRequired { formula_id, .. }
            | Self::M07CandidateBlocked { formula_id, .. }
            | Self::DispatchUnavailable { formula_id, .. } => Some(formula_id.as_str()),
            Self::Usage(_)
            | Self::InvalidAssignment(_)
            | Self::DuplicateInput(_)
            | Self::InvalidNumber { .. }
            | Self::SelfCheckFailed { .. } => None,
        }
    }

    fn status(&self) -> Option<&str> {
        match self {
            Self::ExecutionBlockedByStatus { status, .. }
            | Self::PreliminaryFlagRequired { status, .. }
            | Self::M07CandidateBlocked { status, .. }
            | Self::DispatchUnavailable { status, .. } => Some(status),
            _ => None,
        }
    }

    fn execution_policy(&self) -> Option<&str> {
        match self {
            Self::ExecutionBlockedByStatus {
                execution_policy, ..
            }
            | Self::PreliminaryFlagRequired {
                execution_policy, ..
            }
            | Self::M07CandidateBlocked {
                execution_policy, ..
            }
            | Self::DispatchUnavailable {
                execution_policy, ..
            } => Some(execution_policy),
            _ => None,
        }
    }

    fn exit_code(&self) -> u8 {
        match self {
            Self::Usage(_)
            | Self::InvalidAssignment(_)
            | Self::DuplicateInput(_)
            | Self::InvalidNumber { .. }
            | Self::MissingInput { .. }
            | Self::UnexpectedInput { .. } => 2,
            Self::UnknownFormula(_) => 3,
            Self::Equation { .. }
            | Self::ExecutionBlockedByStatus { .. }
            | Self::PreliminaryFlagRequired { .. }
            | Self::M07CandidateBlocked { .. }
            | Self::DispatchUnavailable { .. } => 4,
            Self::SelfCheckFailed { .. } => 5,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) => formatter.write_str(message),
            Self::UnknownFormula(formula_id) => {
                write!(formatter, "unknown formula id `{formula_id}`")
            }
            Self::InvalidAssignment(value) => write!(
                formatter,
                "input assignment `{value}` must use the form name=value"
            ),
            Self::DuplicateInput(input) => {
                write!(formatter, "input `{input}` was provided more than once")
            }
            Self::InvalidNumber { input, value } => {
                write!(formatter, "input `{input}` has invalid f64 value `{value}`")
            }
            Self::MissingInput { formula_id, input } => {
                write!(formatter, "formula `{formula_id}` requires input `{input}`")
            }
            Self::UnexpectedInput { formula_id, input } => {
                write!(
                    formatter,
                    "formula `{formula_id}` does not accept input `{input}`"
                )
            }
            Self::ExecutionBlockedByStatus {
                formula_id,
                status,
                execution_policy,
            } => write!(
                formatter,
                "formula `{formula_id}` execution is blocked by status `{status}` with execution_policy `{execution_policy}`"
            ),
            Self::PreliminaryFlagRequired {
                formula_id,
                status,
                execution_policy,
            } => write!(
                formatter,
                "formula `{formula_id}` status `{status}` requires --preliminary before public-alpha execution; execution_policy `{execution_policy}`"
            ),
            Self::M07CandidateBlocked {
                formula_id,
                status,
                execution_policy,
            } => write!(
                formatter,
                "formula `{formula_id}` is an M07 candidate and remains blocked by public-alpha execution gates with status `{status}` and execution_policy `{execution_policy}`"
            ),
            Self::DispatchUnavailable {
                formula_id,
                status,
                execution_policy,
            } => write!(
                formatter,
                "formula `{formula_id}` has executable status `{status}` with execution_policy `{execution_policy}` but no CLI dispatch specification"
            ),
            Self::Equation { formula_id, source } => {
                write!(formatter, "formula `{formula_id}` failed: {source}")
            }
            Self::SelfCheckFailed { failed } => {
                write!(
                    formatter,
                    "AeroCodex self-check reported {failed} failing checks"
                )
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Equation { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct SelfCheckResult {
    name: &'static str,
    formula_id: &'static str,
    passed: bool,
    detail: String,
}

#[derive(Debug)]
struct SelfCheckReport {
    checks: Vec<SelfCheckResult>,
    passed: usize,
    failed: usize,
}

fn formula_spec(formula_id: &str) -> Option<&'static FormulaSpec> {
    formula_specs().iter().find(|spec| spec.id == formula_id)
}

#[derive(Debug, Clone, Copy)]
enum CommandContext {
    Namespace {
        command: &'static str,
    },
    LegacyAlias {
        command: &'static str,
        migration_command: &'static str,
    },
}

impl CommandContext {
    fn command(self) -> &'static str {
        match self {
            Self::Namespace { command } | Self::LegacyAlias { command, .. } => command,
        }
    }

    fn deprecated_alias(self) -> bool {
        matches!(self, Self::LegacyAlias { .. })
    }

    fn migration_command(self) -> Option<&'static str> {
        match self {
            Self::Namespace { .. } => None,
            Self::LegacyAlias {
                migration_command, ..
            } => Some(migration_command),
        }
    }
}

#[derive(Debug, Default)]
struct FormulaListFilters {
    family: Option<String>,
    status: Option<String>,
    executable: bool,
}

impl FormulaListFilters {
    fn matches(
        &self,
        entry: &'static generated_formula_registry::FormulaRegistryEntry,
    ) -> bool {
        if let Some(family) = self.family.as_deref() {
            let root_family = registry_family(entry);
            if entry.family != family && root_family != family {
                return false;
            }
        }
        if let Some(status) = self.status.as_deref() {
            if entry.status != status {
                return false;
            }
        }
        if self.executable
            && (!matches!(
                entry.execution_policy,
                "normal_research" | "publication_supporting"
            ) || formula_spec_for_registry_entry(entry).is_none())
        {
            return false;
        }
        true
    }
}

fn parse_formula_list_filters(arguments: &[String]) -> Result<FormulaListFilters, AppError> {
    let mut filters = FormulaListFilters::default();
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--family" => {
                index += 1;
                let Some(value) = arguments.get(index) else {
                    return Err(AppError::Usage(
                        "formula list option `--family` requires a value".to_string(),
                    ));
                };
                if value.starts_with('-') {
                    return Err(AppError::Usage(
                        "formula list option `--family` requires a family value".to_string(),
                    ));
                }
                filters.family = Some(value.clone());
            }
            "--status" => {
                index += 1;
                let Some(value) = arguments.get(index) else {
                    return Err(AppError::Usage(
                        "formula list option `--status` requires a value".to_string(),
                    ));
                };
                if value.starts_with('-') {
                    return Err(AppError::Usage(
                        "formula list option `--status` requires a status value".to_string(),
                    ));
                }
                filters.status = Some(value.clone());
            }
            "--executable" => {
                filters.executable = true;
            }
            other if other.starts_with('-') => {
                return Err(AppError::Usage(format!(
                    "unknown formula list option `{other}`"
                )));
            }
            other => {
                return Err(AppError::Usage(format!(
                    "formula list does not accept positional argument `{other}`"
                )));
            }
        }
        index += 1;
    }
    Ok(filters)
}

#[derive(Debug)]
struct ResolvedFormula {
    requested_id: String,
    registry_entry: Option<&'static generated_formula_registry::FormulaRegistryEntry>,
    spec: Option<&'static FormulaSpec>,
    alias_used: Option<String>,
}

impl ResolvedFormula {
    fn formula_id(&self) -> &'static str {
        self.registry_entry
            .map(|entry| entry.formula_id)
            .or_else(|| self.spec.map(|spec| spec.id))
            .expect("resolved formula always carries registry entry or executable spec")
    }

    fn legacy_formula_id(&self) -> Option<&'static str> {
        self.registry_entry
            .and_then(|entry| entry.legacy_formula_id)
            .or_else(|| self.spec.map(|spec| spec.id))
    }

    fn status(&self) -> &'static str {
        self.registry_entry
            .map(|entry| entry.status)
            .unwrap_or_else(validation_status)
    }

    fn execution_policy(&self) -> Option<&'static str> {
        self.registry_entry.map(|entry| entry.execution_policy)
    }

    fn quarantine_state(&self) -> Option<&'static str> {
        self.registry_entry.map(|entry| entry.quarantine_state)
    }

    fn name(&self) -> Option<&'static str> {
        self.registry_entry.map(|entry| entry.name)
    }

    fn runtime_symbol(&self) -> Option<&'static str> {
        self.spec
            .map(|spec| spec.runtime_symbol)
            .or_else(|| self.registry_entry.and_then(|entry| entry.runtime_symbol))
    }

    fn output_variable(&self) -> Option<&'static str> {
        self.spec
            .map(|spec| spec.output_variable)
            .or_else(|| self.registry_entry.and_then(|entry| entry.output_variable))
    }

    fn inputs(&self) -> &'static [&'static str] {
        self.spec
            .map(|spec| spec.inputs.as_slice())
            .or_else(|| self.registry_entry.map(|entry| entry.input_names))
            .unwrap_or(&[])
    }

    fn output_names(&self) -> &'static [&'static str] {
        self.registry_entry
            .map(|entry| entry.output_names)
            .unwrap_or(&[])
    }

    fn summary(&self) -> Option<&'static str> {
        self.spec.map(|spec| spec.summary)
    }
}

fn execution_policy_for_status(status: &str) -> &'static str {
    match status {
        "research_required" => "blocked",
        "equation_traceable" => "preliminary_flag_required",
        "implementation_verified" => "normal_research",
        "reference_validated" => "publication_supporting",
        _ => "blocked",
    }
}

fn is_m07_candidate_parts(
    formula_id: &str,
    family: Option<&str>,
    legacy_formula_id: Option<&str>,
) -> bool {
    [Some(formula_id), family, legacy_formula_id]
        .into_iter()
        .flatten()
        .any(|value| value.to_ascii_lowercase().contains("m07"))
}

fn execution_gate_error_for_parts(
    formula_id: &str,
    family: Option<&str>,
    legacy_formula_id: Option<&str>,
    status: &'static str,
    preliminary: bool,
) -> Option<AppError> {
    if is_m07_candidate_parts(formula_id, family, legacy_formula_id) {
        return Some(AppError::M07CandidateBlocked {
            formula_id: formula_id.to_string(),
            status,
            execution_policy: "blocked",
        });
    }

    match status {
        "implementation_verified" | "reference_validated" => None,
        "equation_traceable" if preliminary => None,
        "equation_traceable" => Some(AppError::PreliminaryFlagRequired {
            formula_id: formula_id.to_string(),
            status,
            execution_policy: execution_policy_for_status(status),
        }),
        _ => Some(AppError::ExecutionBlockedByStatus {
            formula_id: formula_id.to_string(),
            status,
            execution_policy: execution_policy_for_status(status),
        }),
    }
}

fn execution_gate_error(resolved: &ResolvedFormula, preliminary: bool) -> Option<AppError> {
    execution_gate_error_for_parts(
        resolved.formula_id(),
        resolved.registry_entry.map(|entry| entry.family),
        resolved.legacy_formula_id(),
        resolved.status(),
        preliminary,
    )
}

fn public_run_precondition_error_for_parts(
    formula_id: &str,
    family: Option<&str>,
    legacy_formula_id: Option<&str>,
    status: &'static str,
    preliminary: bool,
    dispatch_available: bool,
) -> Option<AppError> {
    execution_gate_error_for_parts(
        formula_id,
        family,
        legacy_formula_id,
        status,
        preliminary,
    )
    .or_else(|| {
        (!dispatch_available).then(|| AppError::DispatchUnavailable {
            formula_id: formula_id.to_string(),
            status,
            execution_policy: execution_policy_for_status(status),
        })
    })
}

fn public_run_precondition_error(
    resolved: &ResolvedFormula,
    preliminary: bool,
) -> Option<AppError> {
    public_run_precondition_error_for_parts(
        resolved.formula_id(),
        resolved.registry_entry.map(|entry| entry.family),
        resolved.legacy_formula_id(),
        resolved.status(),
        preliminary,
        resolved.spec.is_some(),
    )
}

fn formula_is_implemented(resolved: &ResolvedFormula) -> bool {
    resolved.runtime_symbol().is_some()
        && resolved
            .registry_entry
            .and_then(|entry| entry.implementation_path)
            .is_some()
}

fn formula_is_dispatchable(resolved: &ResolvedFormula) -> bool {
    resolved.spec.is_some()
}

fn formula_is_executable(resolved: &ResolvedFormula) -> bool {
    formula_is_dispatchable(resolved) && execution_gate_error(resolved, false).is_none()
}

fn formula_is_validated(resolved: &ResolvedFormula) -> bool {
    matches!(resolved.status(), "reference_validated" | "experiment_validated")
}

fn formula_is_blocked(resolved: &ResolvedFormula) -> bool {
    resolved.execution_policy() == Some("blocked")
        || is_m07_candidate_parts(
            resolved.formula_id(),
            resolved.registry_entry.map(|entry| entry.family),
            resolved.legacy_formula_id(),
        )
}

fn append_formula_state_json(output: &mut String, resolved: &ResolvedFormula) {
    write!(
        output,
        ",\"implemented\":{},\"dispatchable\":{},\"executable\":{},\"validated\":{},\"blocked\":{}",
        formula_is_implemented(resolved),
        formula_is_dispatchable(resolved),
        formula_is_executable(resolved),
        formula_is_validated(resolved),
        formula_is_blocked(resolved),
    )
    .expect("writing to String cannot fail");
}

fn registry_entry_for_spec(
    spec: &FormulaSpec,
) -> Option<&'static generated_formula_registry::FormulaRegistryEntry> {
    generated_formula_registry::find_by_formula_id(spec.canonical_id)
        .or_else(|| generated_formula_registry::find_by_alias(spec.id))
        .or_else(|| generated_formula_registry::find_by_formula_id(spec.id))
}

fn formula_spec_for_registry_entry(
    entry: &'static generated_formula_registry::FormulaRegistryEntry,
) -> Option<&'static FormulaSpec> {
    formula_specs()
        .iter()
        .find(|spec| spec.canonical_id == entry.formula_id)
        .or_else(|| entry.legacy_formula_id.and_then(formula_spec))
        .or_else(|| entry.aliases.iter().find_map(|alias| formula_spec(alias)))
        .or_else(|| formula_spec(entry.formula_id))
}

fn resolve_formula(formula_id: &str) -> Option<ResolvedFormula> {
    if let Some(entry) = generated_formula_registry::find_by_formula_id(formula_id) {
        return Some(ResolvedFormula {
            requested_id: formula_id.to_string(),
            registry_entry: Some(entry),
            spec: formula_spec_for_registry_entry(entry),
            alias_used: None,
        });
    }

    if let Some(entry) = generated_formula_registry::find_by_alias(formula_id) {
        return Some(ResolvedFormula {
            requested_id: formula_id.to_string(),
            registry_entry: Some(entry),
            spec: formula_spec_for_registry_entry(entry),
            alias_used: Some(formula_id.to_string()),
        });
    }

    formula_spec(formula_id).map(|spec| ResolvedFormula {
        requested_id: formula_id.to_string(),
        registry_entry: registry_entry_for_spec(spec),
        spec: Some(spec),
        alias_used: None,
    })
}

fn required_input(
    spec: &'static FormulaSpec,
    inputs: &BTreeMap<String, f64>,
    name: &'static str,
) -> Result<f64, AppError> {
    inputs.get(name).copied().ok_or(AppError::MissingInput {
        formula_id: spec.id,
        input: name,
    })
}

fn validate_input_names(
    formula_id: &'static str,
    required_inputs: &'static [&'static str],
    inputs: &BTreeMap<String, f64>,
) -> Result<(), AppError> {
    for &required in required_inputs {
        if !inputs.contains_key(required) {
            return Err(AppError::MissingInput {
                formula_id,
                input: required,
            });
        }
    }
    for provided in inputs.keys() {
        if !required_inputs.contains(&provided.as_str()) {
            return Err(AppError::UnexpectedInput {
                formula_id,
                input: provided.clone(),
            });
        }
    }
    Ok(())
}

fn validate_input_shape(
    spec: &'static FormulaSpec,
    inputs: &BTreeMap<String, f64>,
) -> Result<(), AppError> {
    validate_input_names(spec.id, spec.inputs.as_slice(), inputs)
}

fn evaluate_formula(
    formula_id: &str,
    inputs: &BTreeMap<String, f64>,
) -> Result<EvaluationResult, AppError> {
    let spec =
        formula_spec(formula_id).ok_or_else(|| AppError::UnknownFormula(formula_id.to_string()))?;
    validate_input_shape(spec, inputs)?;

    let value = match spec.dispatch {
        RuntimeDispatch::CanonicalTimeUnitFromMuDu => m00_canonical_time_unit_from_mu_du(
            required_input(spec, inputs, "mu")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        RuntimeDispatch::CanonicalSpeedUnitFromDuTu => m00_canonical_speed_unit_from_du_tu(
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        RuntimeDispatch::CanonicalSpeedUnitFromMuDu => m00_canonical_speed_unit_from_mu_du(
            required_input(spec, inputs, "mu")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        RuntimeDispatch::CanonicalMuFromUnits => m00_canonical_mu_from_units(
            required_input(spec, inputs, "mu")?,
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        RuntimeDispatch::DistanceToCanonical => m00_distance_to_canonical(
            required_input(spec, inputs, "distance")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        RuntimeDispatch::DistanceFromCanonical => m00_distance_from_canonical(
            required_input(spec, inputs, "canonical_distance")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        RuntimeDispatch::TimeToCanonical => m00_time_to_canonical(
            required_input(spec, inputs, "time")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        RuntimeDispatch::TimeFromCanonical => m00_time_from_canonical(
            required_input(spec, inputs, "canonical_time")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        RuntimeDispatch::SpeedToCanonical => m00_speed_to_canonical(
            required_input(spec, inputs, "speed")?,
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        RuntimeDispatch::SpeedFromCanonical => m00_speed_from_canonical(
            required_input(spec, inputs, "canonical_speed")?,
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        RuntimeDispatch::DegreesToRadians => {
            m00_degrees_to_radians(required_input(spec, inputs, "degrees")?)
        }
        RuntimeDispatch::RadiansToDegrees => {
            m00_radians_to_degrees(required_input(spec, inputs, "radians")?)
        }
    }
    .map_err(|source| AppError::Equation {
        formula_id: spec.id,
        source,
    })?;

    Ok(EvaluationResult { spec, value })
}

fn parse_scalar_input(input: &str, raw_value: &str) -> Result<f64, AppError> {
    if raw_value.starts_with('[') || raw_value.ends_with(']') || raw_value.contains(',') {
        return Err(AppError::Usage(format!(
            "vector inputs are not yet supported by CLI run; input `{input}` must be provided as a scalar f64"
        )));
    }

    raw_value
        .parse::<f64>()
        .map_err(|_| AppError::InvalidNumber {
            input: input.to_string(),
            value: raw_value.to_string(),
        })
}

fn parse_legacy_assignments(values: &[String]) -> Result<BTreeMap<String, f64>, AppError> {
    let mut inputs = BTreeMap::new();
    for assignment in values {
        let (name, raw_value) = assignment
            .split_once('=')
            .ok_or_else(|| AppError::InvalidAssignment(assignment.clone()))?;
        if name.is_empty() || raw_value.is_empty() {
            return Err(AppError::InvalidAssignment(assignment.clone()));
        }
        let value = parse_scalar_input(name, raw_value)?;
        if inputs.insert(name.to_string(), value).is_some() {
            return Err(AppError::DuplicateInput(name.to_string()));
        }
    }
    Ok(inputs)
}

fn parse_flag_style_inputs(
    formula_id: &'static str,
    required_inputs: &'static [&'static str],
    values: &[String],
) -> Result<BTreeMap<String, f64>, AppError> {
    let mut inputs = BTreeMap::new();
    let mut index = 0;
    while index < values.len() {
        let flag = &values[index];
        let Some(input_name) = flag.strip_prefix("--") else {
            return Err(AppError::InvalidAssignment(flag.clone()));
        };
        let Some(&required_input) = required_inputs
            .iter()
            .find(|&&candidate| candidate == input_name)
        else {
            return Err(AppError::UnexpectedInput {
                formula_id,
                input: input_name.to_string(),
            });
        };

        index += 1;
        let Some(raw_value) = values.get(index) else {
            return Err(AppError::MissingInput {
                formula_id,
                input: required_input,
            });
        };
        if raw_value.starts_with("--") {
            return Err(AppError::MissingInput {
                formula_id,
                input: required_input,
            });
        }

        let value = parse_scalar_input(required_input, raw_value)?;
        if inputs.insert(required_input.to_string(), value).is_some() {
            return Err(AppError::DuplicateInput(required_input.to_string()));
        }
        index += 1;
    }
    Ok(inputs)
}

fn parse_formula_inputs(
    formula_id: &'static str,
    required_inputs: &'static [&'static str],
    values: &[String],
) -> Result<ParsedFormulaInputs, AppError> {
    let has_flag_style = values.iter().any(|value| value.starts_with("--"));
    let has_legacy_assignments = values
        .iter()
        .any(|value| !value.starts_with("--") && value.contains('='));

    if has_flag_style && has_legacy_assignments {
        return Err(AppError::Usage(
            "formula run input syntax must not mix flag-style inputs with legacy name=value assignments".to_string(),
        ));
    }

    let (inputs, syntax) = if has_flag_style {
        (
            parse_flag_style_inputs(formula_id, required_inputs, values)?,
            InputSyntax::FlagStyle,
        )
    } else {
        (
            parse_legacy_assignments(values)?,
            InputSyntax::LegacyAssignment,
        )
    };
    validate_input_names(formula_id, required_inputs, &inputs)?;
    Ok(ParsedFormulaInputs { inputs, syntax })
}

fn remove_json_flag(arguments: &mut Vec<String>) -> Result<bool, AppError> {
    let count = arguments
        .iter()
        .filter(|argument| argument.as_str() == "--json")
        .count();
    if count > 1 {
        return Err(AppError::Usage(
            "`--json` may be supplied at most once".to_string(),
        ));
    }
    arguments.retain(|argument| argument != "--json");
    Ok(count == 1)
}

fn remove_preliminary_flag(arguments: &[String]) -> Result<(bool, Vec<String>), AppError> {
    let count = arguments
        .iter()
        .filter(|argument| argument.as_str() == "--preliminary")
        .count();
    if count > 1 {
        return Err(AppError::Usage(
            "`--preliminary` may be supplied at most once for formula run".to_string(),
        ));
    }

    let remaining = arguments
        .iter()
        .filter(|argument| argument.as_str() != "--preliminary")
        .cloned()
        .collect();
    Ok((count == 1, remaining))
}

fn json_command_for_arguments(arguments: &[String]) -> Option<&'static str> {
    let mut arguments = arguments
        .iter()
        .map(String::as_str)
        .filter(|argument| *argument != "--json");
    match (arguments.next()?, arguments.next()) {
        ("formula", Some("list")) => Some("formula_list"),
        ("formula", Some("describe")) => Some("formula describe"),
        ("formula", Some("status-report")) => Some("formula status-report"),
        ("formula", Some("run")) => Some("formula run"),
        ("formulas", _) => Some("formulas"),
        ("describe", _) => Some("describe"),
        ("run", _) => Some("run"),
        ("version" | "--version" | "-V", _) => Some("version"),
        ("self-check", _) => Some("self-check"),
        _ => None,
    }
}

fn push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            control if control.is_control() => {
                write!(output, "\\u{:04x}", u32::from(control))
                    .expect("writing to String cannot fail");
            }
            other => output.push(other),
        }
    }
    output.push('"');
}

fn push_optional_json_string(output: &mut String, value: Option<&str>) {
    if let Some(value) = value {
        push_json_string(output, value);
    } else {
        output.push_str("null");
    }
}

fn append_release_identity_json_fields(output: &mut String) {
    output.push_str(",\"program_name\":");
    push_json_string(output, PROGRAM_NAME);
    output.push_str(",\"semantic_version\":");
    push_json_string(output, package_version());
    output.push_str(",\"release_tier\":");
    push_json_string(output, release_tier());
    output.push_str(",\"release_tier_display\":");
    push_json_string(output, release_tier_display());
}

fn append_context_json_fields(output: &mut String, context: CommandContext) {
    if context.deprecated_alias() {
        output.push_str(",\"deprecated_alias\":true,\"migration_command\":");
        push_json_string(
            output,
            context
                .migration_command()
                .expect("legacy aliases always carry a migration command"),
        );
    }
}

fn append_string_array(output: &mut String, values: &[&str]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, value);
    }
    output.push(']');
}

fn append_default_json_warnings(output: &mut String) {
    append_string_array(
        output,
        &[
            "Research/preliminary-design JSON contract; status and registry fields are not certification or execution approval.",
        ],
    );
}

fn append_registry_warnings_or_default(output: &mut String, registry_object: Option<&str>) {
    if let Some(warnings) =
        registry_object.and_then(|object| json_field_value_slice(object, "warnings"))
    {
        output.push_str(warnings);
    } else {
        append_default_json_warnings(output);
    }
}

fn formula_registry_json_object(formula_id: &str) -> Option<&'static str> {
    let pattern = format!("\"formula_id\": \"{formula_id}\"");
    let formula_id_position = GENERATED_FORMULA_REGISTRY_JSON.find(&pattern)?;
    let object_start = GENERATED_FORMULA_REGISTRY_JSON[..formula_id_position].rfind('{')?;
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;

    for (offset, character) in GENERATED_FORMULA_REGISTRY_JSON[object_start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        match character {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let object_end = object_start + offset + character.len_utf8();
                    return Some(&GENERATED_FORMULA_REGISTRY_JSON[object_start..object_end]);
                }
            }
            _ => {}
        }
    }

    None
}

fn json_field_value_slice<'a>(object: &'a str, field: &str) -> Option<&'a str> {
    let pattern = format!("\"{field}\":");
    let position = object.find(&pattern)? + pattern.len();
    let mut value_start = position;
    while let Some(character) = object[value_start..].chars().next() {
        if character.is_whitespace() {
            value_start += character.len_utf8();
        } else {
            break;
        }
    }

    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut seen_value_start = false;

    for (offset, character) in object[value_start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
                if depth == 0 {
                    let end = value_start + offset + character.len_utf8();
                    return Some(object[value_start..end].trim());
                }
            }
            continue;
        }

        match character {
            '"' => {
                in_string = true;
                seen_value_start = true;
            }
            '[' | '{' => {
                depth += 1;
                seen_value_start = true;
            }
            ']' | '}' => {
                if depth == 0 {
                    return Some(object[value_start..value_start + offset].trim());
                }
                depth -= 1;
                if depth == 0 {
                    let end = value_start + offset + character.len_utf8();
                    return Some(object[value_start..end].trim());
                }
            }
            ',' if depth == 0 && seen_value_start => {
                return Some(object[value_start..value_start + offset].trim());
            }
            other if !other.is_whitespace() => seen_value_start = true,
            _ => {}
        }
    }

    let value = object[value_start..].trim().trim_end_matches('}').trim();
    (!value.is_empty()).then_some(value)
}

fn json_string_field_value<'a>(object: &'a str, field: &str) -> Option<&'a str> {
    let value = json_field_value_slice(object, field)?;
    value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
}

fn append_formula_json_body(output: &mut String, object: &str) {
    let body = object
        .trim()
        .strip_prefix('{')
        .and_then(|value| value.strip_suffix('}'))
        .expect("generated formula JSON object must have braces");
    let mut compact = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for character in body.chars() {
        if in_string {
            compact.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }

        match character {
            '"' => {
                compact.push(character);
                in_string = true;
            }
            character if character.is_whitespace() => {}
            character => compact.push(character),
        }
    }

    if !compact.is_empty() {
        output.push(',');
        output.push_str(&compact);
    }
}

fn registry_json_string_field(field: &str) -> &'static str {
    let pattern = format!("\"{field}\": \"");
    let start = GENERATED_FORMULA_REGISTRY_JSON
        .find(&pattern)
        .expect("checked-in generated registry JSON must expose required top-level field")
        + pattern.len();
    let end = GENERATED_FORMULA_REGISTRY_JSON[start..]
        .find('"')
        .expect("checked-in generated registry JSON field must be a string");
    &GENERATED_FORMULA_REGISTRY_JSON[start..start + end]
}

fn registry_schema_version() -> &'static str {
    registry_json_string_field("schema_version")
}

fn registry_source_hash() -> &'static str {
    registry_json_string_field("source_hash")
}

fn registry_family(entry: &generated_formula_registry::FormulaRegistryEntry) -> &'static str {
    entry
        .family
        .split_once('.')
        .map(|(root, _)| root)
        .unwrap_or(entry.family)
}

fn registry_output(
    entry: &generated_formula_registry::FormulaRegistryEntry,
) -> Option<&'static str> {
    entry
        .output_variable
        .or_else(|| entry.output_names.first().copied())
}

fn registry_runtime_label(
    entry: &generated_formula_registry::FormulaRegistryEntry,
) -> Option<&'static str> {
    entry.runtime_symbol.or(entry.implementation_path)
}

fn append_formula_list_filters_json(output: &mut String, filters: &FormulaListFilters) {
    output.push_str(",\"filters\":{\"family\":");
    push_optional_json_string(output, filters.family.as_deref());
    output.push_str(",\"status\":");
    push_optional_json_string(output, filters.status.as_deref());
    write!(output, ",\"executable\":{}", filters.executable)
        .expect("writing to String cannot fail");
    output.push('}');
}

#[derive(Debug)]
struct FormulaStatusReport {
    total_formula_count: usize,
    dispatchable_formula_count: usize,
    counts_by_status: BTreeMap<&'static str, usize>,
    counts_by_execution_policy: BTreeMap<&'static str, usize>,
    counts_by_family: BTreeMap<&'static str, usize>,
    normal_executable_count: usize,
    preliminary_only_formula_count: usize,
    blocked_formula_count: usize,
    m07_candidate_count: usize,
    promotion_candidate_count: usize,
}

impl FormulaStatusReport {
    fn from_registry() -> Self {
        let mut report = Self {
            total_formula_count: generated_formula_registry::FORMULA_REGISTRY.len(),
            dispatchable_formula_count: formula_specs().len(),
            counts_by_status: BTreeMap::new(),
            counts_by_execution_policy: BTreeMap::new(),
            counts_by_family: BTreeMap::new(),
            normal_executable_count: 0,
            preliminary_only_formula_count: 0,
            blocked_formula_count: 0,
            m07_candidate_count: 0,
            promotion_candidate_count: 0,
        };

        for entry in generated_formula_registry::FORMULA_REGISTRY {
            *report.counts_by_status.entry(entry.status).or_insert(0) += 1;
            *report
                .counts_by_execution_policy
                .entry(entry.execution_policy)
                .or_insert(0) += 1;
            *report
                .counts_by_family
                .entry(registry_family(entry))
                .or_insert(0) += 1;

            match entry.execution_policy {
                "normal_research" | "publication_supporting" => {
                    if formula_spec_for_registry_entry(entry).is_some() {
                        report.normal_executable_count += 1;
                    }
                }
                "preliminary_flag_required" => {
                    report.preliminary_only_formula_count += 1;
                }
                _ => {
                    report.blocked_formula_count += 1;
                }
            }

            if is_m07_candidate(entry) {
                report.m07_candidate_count += 1;
            }
            if is_promotion_candidate(entry) {
                report.promotion_candidate_count += 1;
            }
        }

        report
    }

    fn execution_policy_bucket_total(&self) -> usize {
        self.counts_by_execution_policy.values().sum()
    }
}

fn is_m07_candidate(entry: &generated_formula_registry::FormulaRegistryEntry) -> bool {
    entry.formula_id.starts_with("m07.")
        || entry.family == "m07"
        || entry.family.starts_with("m07.")
        || matches!(entry.legacy_formula_id, Some(id) if id.starts_with("formula_vault.m07."))
        || matches!(entry.source_formula_id, Some(id) if id.starts_with("formula_vault.m07."))
}

fn is_promotion_candidate(entry: &generated_formula_registry::FormulaRegistryEntry) -> bool {
    !is_m07_candidate(entry)
        && matches!(entry.execution_policy, "preliminary_flag_required")
        && matches!(entry.status, "equation_traceable")
}

fn append_count_map(output: &mut String, counts: &BTreeMap<&'static str, usize>) {
    output.push('{');
    for (index, (key, count)) in counts.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        push_json_string(output, key);
        write!(output, ":{count}").expect("writing to String cannot fail");
    }
    output.push('}');
}

fn append_formula_id_examples<F>(output: &mut String, mut predicate: F)
where
    F: FnMut(&generated_formula_registry::FormulaRegistryEntry) -> bool,
{
    output.push('[');
    let mut written = 0usize;
    for entry in generated_formula_registry::FORMULA_REGISTRY {
        if !predicate(entry) {
            continue;
        }
        if written > 0 {
            output.push(',');
        }
        push_json_string(output, entry.formula_id);
        written += 1;
        if written == 5 {
            break;
        }
    }
    output.push(']');
}

fn append_status_report_categories_json(output: &mut String, report: &FormulaStatusReport) {
    output.push_str(",\"categories\":{");
    write!(
        output,
        "\"blocked_formulas\":{{\"count\":{},\"examples\":",
        report.blocked_formula_count
    )
    .expect("writing to String cannot fail");
    append_formula_id_examples(output, |entry| entry.execution_policy == "blocked");
    write!(
        output,
        "}},\"normal_executable_formulas\":{{\"count\":{},\"examples\":",
        report.normal_executable_count
    )
    .expect("writing to String cannot fail");
    append_formula_id_examples(output, |entry| {
        matches!(
            entry.execution_policy,
            "normal_research" | "publication_supporting"
        )
    });
    write!(
        output,
        "}},\"preliminary_only_formulas\":{{\"count\":{},\"examples\":",
        report.preliminary_only_formula_count
    )
    .expect("writing to String cannot fail");
    append_formula_id_examples(output, |entry| {
        entry.execution_policy == "preliminary_flag_required"
    });
    write!(
        output,
        "}},\"m07_candidates\":{{\"count\":{},\"examples\":",
        report.m07_candidate_count
    )
    .expect("writing to String cannot fail");
    append_formula_id_examples(output, is_m07_candidate);
    write!(
        output,
        "}},\"promotion_candidates\":{{\"count\":{},\"definition\":",
        report.promotion_candidate_count
    )
    .expect("writing to String cannot fail");
    push_json_string(
        output,
        "non-M07 equation_traceable formulas with preliminary_flag_required execution policy",
    );
    output.push_str(",\"examples\":");
    append_formula_id_examples(output, is_promotion_candidate);
    output.push_str("}}");
}

fn output_formula_status_report(json: bool) {
    let report = FormulaStatusReport::from_registry();

    if json {
        let mut output = String::from("{\"ok\":true,\"command\":\"formula status-report\"");
        output.push_str(",\"json_contract_version\":");
        push_json_string(&mut output, JSON_CONTRACT_VERSION);
        append_release_identity_json_fields(&mut output);
        write!(
            output,
            ",\"registry_formula_count\":{},\"total_formula_count\":{},\"inventory_formula_count\":{},\"dispatchable_formula_count\":{}",
            generated_formula_registry::FORMULA_COUNT,
            report.total_formula_count,
            report.total_formula_count,
            report.dispatchable_formula_count
        )
        .expect("writing to String cannot fail");
        output.push_str(",\"counts_by_status\":");
        append_count_map(&mut output, &report.counts_by_status);
        output.push_str(",\"counts_by_execution_policy\":");
        append_count_map(&mut output, &report.counts_by_execution_policy);
        output.push_str(",\"by_family\":");
        append_count_map(&mut output, &report.counts_by_family);
        write!(
            output,
            ",\"execution_policy_bucket_total\":{},\"normal_executable_count\":{},\"executable_formula_count\":{},\"public_executable_formula_count\":{},\"preliminary_only_formula_count\":{},\"blocked_formula_count\":{},\"m07_candidate_count\":{},\"promotion_candidate_count\":{}",
            report.execution_policy_bucket_total(),
            report.normal_executable_count,
            report.normal_executable_count,
            report.normal_executable_count,
            report.preliminary_only_formula_count,
            report.blocked_formula_count,
            report.m07_candidate_count,
            report.promotion_candidate_count
        )
        .expect("writing to String cannot fail");
        output.push_str(",\"registry_schema_version\":");
        push_json_string(&mut output, registry_schema_version());
        output.push_str(",\"source_hash\":");
        push_json_string(&mut output, registry_source_hash());
        output.push_str(",\"validation_status\":");
        push_json_string(&mut output, validation_status());
        append_status_report_categories_json(&mut output, &report);
        output.push_str(",\"warnings\":");
        append_default_json_warnings(&mut output);
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str(",\"error\":null}\n");
        write_stdout(&output);
    } else {
        let mut output = String::new();
        output.push_str("Formula status report\n");
        output.push_str("command=formula status-report\n");
        writeln!(output, "program_name={PROGRAM_NAME}").expect("writing to String cannot fail");
        writeln!(output, "semantic_version={}", package_version())
            .expect("writing to String cannot fail");
        writeln!(output, "release_tier={}", release_tier()).expect("writing to String cannot fail");
        writeln!(output, "release_tier_display={}", release_tier_display())
            .expect("writing to String cannot fail");
        writeln!(
            output,
            "registry_formula_count={}",
            generated_formula_registry::FORMULA_COUNT
        )
        .expect("writing to String cannot fail");
        writeln!(output, "total_formula_count={}", report.total_formula_count)
            .expect("writing to String cannot fail");
        writeln!(
            output,
            "inventory_formula_count={}",
            report.total_formula_count
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "dispatchable_formula_count={}",
            report.dispatchable_formula_count
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "normal_executable_count={}",
            report.normal_executable_count
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "executable_formula_count={}",
            report.normal_executable_count
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "public_executable_formula_count={}",
            report.normal_executable_count
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "preliminary_only_formula_count={}",
            report.preliminary_only_formula_count
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "blocked_formula_count={}",
            report.blocked_formula_count
        )
        .expect("writing to String cannot fail");
        writeln!(output, "m07_candidate_count={}", report.m07_candidate_count)
            .expect("writing to String cannot fail");
        writeln!(
            output,
            "promotion_candidate_count={}",
            report.promotion_candidate_count
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "execution_policy_bucket_total={}",
            report.execution_policy_bucket_total()
        )
        .expect("writing to String cannot fail");
        output.push_str("counts_by_status:\n");
        for (status, count) in &report.counts_by_status {
            writeln!(output, "status.{status}={count}").expect("writing to String cannot fail");
        }
        output.push_str("counts_by_execution_policy:\n");
        for (policy, count) in &report.counts_by_execution_policy {
            writeln!(output, "execution_policy.{policy}={count}")
                .expect("writing to String cannot fail");
        }
        output.push_str("by_family:\n");
        for (family, count) in &report.counts_by_family {
            writeln!(output, "by_family.{family}={count}").expect("writing to String cannot fail");
        }
        writeln!(
            output,
            "registry_schema_version={}",
            registry_schema_version()
        )
        .expect("writing to String cannot fail");
        writeln!(output, "source_hash={}", registry_source_hash())
            .expect("writing to String cannot fail");
        writeln!(output, "validation_status={}", validation_status())
            .expect("writing to String cannot fail");
        output.push_str(
            "status_note=blocked formulas are honest inventory entries, not command errors\n",
        );
        writeln!(output, "safety_notice={}", safety_notice())
            .expect("writing to String cannot fail");
        write_stdout(&output);
    }
}

fn write_stdout(output: &str) {
    let mut stdout = io::stdout().lock();
    if let Err(error) = stdout.write_all(output.as_bytes()) {
        if error.kind() != io::ErrorKind::BrokenPipe {
            panic!("failed writing to stdout: {error}");
        }
    }
}

fn json_error(error: &AppError, command: Option<&str>) -> String {
    let mut output = String::from("{\"ok\":false,\"command\":");
    push_optional_json_string(&mut output, command);
    output.push_str(",\"json_contract_version\":");
    push_json_string(&mut output, JSON_CONTRACT_VERSION);
    output.push_str(",\"formula_id\":");
    push_optional_json_string(&mut output, error.formula_id());
    output.push_str(",\"status\":");
    push_optional_json_string(&mut output, error.status());
    output.push_str(",\"execution_policy\":");
    push_optional_json_string(&mut output, error.execution_policy());
    output.push_str(",\"error\":{\"code\":");
    push_json_string(&mut output, error.code());
    output.push_str(",\"message\":");
    push_json_string(&mut output, &error.to_string());
    output.push('}');
    append_release_identity_json_fields(&mut output);
    output.push_str(",\"validation_status\":");
    push_json_string(&mut output, validation_status());
    output.push_str(",\"safety_notice\":");
    push_json_string(&mut output, safety_notice());
    output.push_str("}\n");
    output
}

fn output_version(json: bool, standard_flag: bool) {
    let report = FormulaStatusReport::from_registry();
    if json {
        let mut output = String::from("{\"ok\":true,\"command\":\"version\",\"package_version\":");
        push_json_string(&mut output, package_version());
        output.push_str(",\"json_contract_version\":");
        push_json_string(&mut output, JSON_CONTRACT_VERSION);
        append_release_identity_json_fields(&mut output);
        output.push_str(",\"build_commit\":");
        push_json_string(&mut output, build_commit());
        output.push_str(",\"build_target\":");
        push_json_string(&mut output, build_target());
        output.push_str(",\"build_profile\":");
        push_json_string(&mut output, build_profile());
        write!(
            output,
            ",\"workspace_package_count\":{},\"supported_formula_count\":{},\"dispatchable_formula_count\":{},\"registry_formula_count\":{},\"blocked_formula_count\":{},\"public_executable_formula_count\":{},\"registry_schema_version\":",
            WORKSPACE_PACKAGE_COUNT,
            report.dispatchable_formula_count,
            report.dispatchable_formula_count,
            generated_formula_registry::FORMULA_COUNT,
            report.blocked_formula_count,
            report.normal_executable_count
        )
        .expect("writing to String cannot fail");
        push_json_string(&mut output, registry_schema_version());
        output.push_str(",\"validation_status\":");
        push_json_string(&mut output, validation_status());
        output.push_str(",\"warnings\":");
        append_default_json_warnings(&mut output);
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str(",\"error\":null}\n");
        print!("{output}");
    } else if standard_flag {
        println!("{PROGRAM_NAME} {}", package_version());
    } else {
        println!("program_name={PROGRAM_NAME}");
        println!("semantic_version={}", package_version());
        println!("release_tier={}", release_tier());
        println!("release_tier_display={}", release_tier_display());
        println!("build_commit={}", build_commit());
        println!("build_target={}", build_target());
        println!("build_profile={}", build_profile());
        println!(
            "supported_formula_count={}",
            report.dispatchable_formula_count
        );
        println!(
            "dispatchable_formula_count={}",
            report.dispatchable_formula_count
        );
        println!(
            "registry_formula_count={}",
            generated_formula_registry::FORMULA_COUNT
        );
        println!("blocked_formula_count={}", report.blocked_formula_count);
        println!(
            "public_executable_formula_count={}",
            report.normal_executable_count
        );
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
    }
}

fn output_formula_list(json: bool, context: CommandContext, filters: &FormulaListFilters) {
    let entries: Vec<&generated_formula_registry::FormulaRegistryEntry> =
        generated_formula_registry::FORMULA_REGISTRY
            .iter()
            .filter(|entry| filters.matches(entry))
            .collect();

    if json {
        let mut output = String::from("{\"ok\":true,\"command\":");
        push_json_string(&mut output, context.command());
        output.push_str(",\"json_contract_version\":");
        push_json_string(&mut output, JSON_CONTRACT_VERSION);
        append_context_json_fields(&mut output, context);
        write!(
            output,
            ",\"count\":{},\"registry_formula_count\":{},\"registry_schema_version\":",
            entries.len(),
            generated_formula_registry::FORMULA_COUNT
        )
        .expect("writing to String cannot fail");
        push_json_string(&mut output, registry_schema_version());
        output.push_str(",\"source_hash\":");
        push_json_string(&mut output, registry_source_hash());
        append_formula_list_filters_json(&mut output, filters);
        output.push_str(",\"validation_status\":");
        push_json_string(&mut output, validation_status());
        output.push_str(",\"formulas\":[");
        for (index, entry) in entries.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str("{\"formula_id\":");
            push_json_string(&mut output, entry.formula_id);
            output.push_str(",\"legacy_formula_id\":");
            push_optional_json_string(&mut output, entry.legacy_formula_id);
            output.push_str(",\"aliases\":");
            append_string_array(&mut output, entry.aliases);
            output.push_str(",\"name\":");
            push_json_string(&mut output, entry.name);
            output.push_str(",\"status\":");
            push_json_string(&mut output, entry.status);
            output.push_str(",\"execution_policy\":");
            push_json_string(&mut output, entry.execution_policy);
            output.push_str(",\"quarantine_state\":");
            push_json_string(&mut output, entry.quarantine_state);
            output.push_str(",\"family\":");
            push_json_string(&mut output, registry_family(entry));
            output.push_str(",\"registry_family\":");
            push_json_string(&mut output, entry.family);
            output.push_str(",\"batch_id\":");
            push_optional_json_string(&mut output, entry.batch_id);
            output.push_str(",\"output\":");
            push_optional_json_string(&mut output, registry_output(entry));
            output.push_str(",\"output_variable\":");
            push_optional_json_string(&mut output, entry.output_variable);
            output.push_str(",\"outputs\":");
            append_string_array(&mut output, entry.output_names);
            output.push_str(",\"runtime_symbol\":");
            push_optional_json_string(&mut output, entry.runtime_symbol);
            output.push_str(",\"implementation_path\":");
            push_optional_json_string(&mut output, entry.implementation_path);
            output.push_str(",\"runtime_label\":");
            push_optional_json_string(&mut output, registry_runtime_label(entry));
            output.push_str(",\"inputs\":");
            append_string_array(&mut output, entry.input_names);
            output.push('}');
        }
        output.push_str("],\"warnings\":");
        append_default_json_warnings(&mut output);
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str(",\"error\":null}\n");
        write_stdout(&output);
    } else {
        let mut output = String::new();
        writeln!(output, "command={}", context.command()).expect("writing to String cannot fail");
        if let Some(migration_command) = context.migration_command() {
            output.push_str("deprecated_alias=true\n");
            writeln!(output, "migration_command={migration_command}")
                .expect("writing to String cannot fail");
        }
        writeln!(output, "count={}", entries.len()).expect("writing to String cannot fail");
        writeln!(
            output,
            "registry_formula_count={}",
            generated_formula_registry::FORMULA_COUNT
        )
        .expect("writing to String cannot fail");
        writeln!(
            output,
            "registry_schema_version={}",
            registry_schema_version()
        )
        .expect("writing to String cannot fail");
        writeln!(output, "source_hash={}", registry_source_hash())
            .expect("writing to String cannot fail");
        if let Some(family) = filters.family.as_deref() {
            writeln!(output, "filter_family={family}").expect("writing to String cannot fail");
        }
        if let Some(status) = filters.status.as_deref() {
            writeln!(output, "filter_status={status}").expect("writing to String cannot fail");
        }
        if filters.executable {
            output.push_str("filter_executable=true\n");
        }
        writeln!(output, "validation_status={}", validation_status())
            .expect("writing to String cannot fail");
        writeln!(output, "safety_notice={}", safety_notice())
            .expect("writing to String cannot fail");
        output.push_str("formula_id\tstatus\texecution_policy\tfamily\toutput\truntime\n");
        for entry in entries {
            writeln!(
                output,
                "{}\t{}\t{}\t{}\t{}\t{}",
                entry.formula_id,
                entry.status,
                entry.execution_policy,
                registry_family(entry),
                registry_output(entry).unwrap_or("-"),
                registry_runtime_label(entry).unwrap_or("-")
            )
            .expect("writing to String cannot fail");
        }
        write_stdout(&output);
    }
}

fn output_formula_description(resolved: &ResolvedFormula, json: bool, context: CommandContext) {
    let registry_object = formula_registry_json_object(resolved.formula_id());

    if json {
        let mut output = String::from("{\"ok\":true,\"command\":");
        push_json_string(&mut output, context.command());
        output.push_str(",\"json_contract_version\":");
        push_json_string(&mut output, JSON_CONTRACT_VERSION);
        append_context_json_fields(&mut output, context);
        output.push_str(",\"canonical_formula_id\":");
        push_json_string(&mut output, resolved.formula_id());
        output.push_str(",\"requested_formula_id\":");
        push_json_string(&mut output, &resolved.requested_id);
        output.push_str(",\"alias_used\":");
        push_optional_json_string(&mut output, resolved.alias_used.as_deref());
        output.push_str(",\"contract_path\":");
        push_optional_json_string(
            &mut output,
            resolved
                .registry_entry
                .and_then(|entry| entry.contract_path),
        );
        output.push_str(",\"validation_card_path\":");
        push_optional_json_string(
            &mut output,
            resolved
                .registry_entry
                .and_then(|entry| entry.validation_card_path),
        );
        output.push_str(",\"source_seed_path\":");
        push_optional_json_string(
            &mut output,
            resolved
                .registry_entry
                .and_then(|entry| entry.source_seed_path),
        );

        if let Some(object) = registry_object {
            append_formula_json_body(&mut output, object);
        } else {
            output.push_str(",\"formula_id\":");
            push_json_string(&mut output, resolved.formula_id());
            output.push_str(",\"legacy_formula_id\":");
            push_optional_json_string(&mut output, resolved.legacy_formula_id());
            output.push_str(",\"name\":");
            push_optional_json_string(&mut output, resolved.name());
            output.push_str(",\"runtime_symbol\":");
            push_optional_json_string(&mut output, resolved.runtime_symbol());
            output.push_str(",\"output_variable\":");
            push_optional_json_string(&mut output, resolved.output_variable());
            output.push_str(",\"outputs\":");
            append_string_array(&mut output, resolved.output_names());
            output.push_str(",\"summary\":");
            push_optional_json_string(&mut output, resolved.summary());
            output.push_str(",\"inputs\":");
            append_string_array(&mut output, resolved.inputs());
            output.push_str(",\"status\":");
            push_json_string(&mut output, resolved.status());
            output.push_str(",\"execution_policy\":");
            push_optional_json_string(&mut output, resolved.execution_policy());
            output.push_str(",\"quarantine_state\":");
            push_optional_json_string(&mut output, resolved.quarantine_state());
            output.push_str(",\"domain_constraints\":[]");
            output.push_str(",\"implementation_path\":null");
            output.push_str(",\"units\":null");
            output.push_str(",\"warnings\":[]");
        }

        append_formula_state_json(&mut output, resolved);

        output.push_str(",\"registry_schema_version\":");
        push_json_string(&mut output, registry_schema_version());
        output.push_str(",\"source_hash\":");
        push_json_string(&mut output, registry_source_hash());
        output.push_str(",\"validation_status\":");
        push_json_string(&mut output, validation_status());
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str(",\"error\":null}\n");
        print!("{output}");
    } else {
        println!("command={}", context.command());
        if let Some(migration_command) = context.migration_command() {
            println!("deprecated_alias=true");
            println!("migration_command={migration_command}");
        }
        println!("formula_id={}", resolved.formula_id());
        println!("canonical_formula_id={}", resolved.formula_id());
        println!("requested_formula_id={}", resolved.requested_id);
        if let Some(alias_used) = resolved.alias_used.as_deref() {
            println!("alias_used={alias_used}");
        }
        println!(
            "legacy_formula_id={}",
            resolved.legacy_formula_id().unwrap_or("null")
        );
        if let Some(name) = resolved.name() {
            println!("name={name}");
        }
        if let Some(entry) = resolved.registry_entry {
            println!("family={}", entry.family);
        }
        println!("status={}", resolved.status());
        println!(
            "execution_policy={}",
            resolved.execution_policy().unwrap_or("blocked")
        );
        println!(
            "quarantine_state={}",
            resolved
                .quarantine_state()
                .unwrap_or("below_execution_threshold")
        );
        println!("implemented={}", formula_is_implemented(resolved));
        println!("dispatchable={}", formula_is_dispatchable(resolved));
        println!("executable={}", formula_is_executable(resolved));
        println!("validated={}", formula_is_validated(resolved));
        println!("blocked={}", formula_is_blocked(resolved));
        if let Some(object) = registry_object {
            println!(
                "inputs={}",
                json_field_value_slice(object, "inputs").unwrap_or("[]")
            );
            println!(
                "outputs={}",
                json_field_value_slice(object, "outputs").unwrap_or("[]")
            );
            println!(
                "units={}",
                json_field_value_slice(object, "units").unwrap_or("null")
            );
            println!(
                "domain_constraints={}",
                json_field_value_slice(object, "domain_constraints").unwrap_or("[]")
            );
            println!(
                "implementation_path={}",
                json_field_value_slice(object, "implementation_path").unwrap_or("null")
            );
            let source_trace = json_field_value_slice(object, "source_trace");
            println!(
                "contract_path={}",
                source_trace
                    .and_then(|trace| json_string_field_value(trace, "contract_path"))
                    .or_else(|| resolved
                        .registry_entry
                        .and_then(|entry| entry.contract_path))
                    .unwrap_or("null")
            );
            println!(
                "validation_card_path={}",
                source_trace
                    .and_then(|trace| json_string_field_value(trace, "validation_card_path"))
                    .or_else(|| {
                        resolved
                            .registry_entry
                            .and_then(|entry| entry.validation_card_path)
                    })
                    .unwrap_or("null")
            );
            println!(
                "source_seed_path={}",
                source_trace
                    .and_then(|trace| json_string_field_value(trace, "source_seed_path"))
                    .or_else(|| resolved
                        .registry_entry
                        .and_then(|entry| entry.source_seed_path))
                    .unwrap_or("null")
            );
            println!(
                "warnings={}",
                json_field_value_slice(object, "warnings").unwrap_or("[]")
            );
        } else {
            println!("inputs={}", resolved.inputs().join(","));
            println!("outputs={}", resolved.output_names().join(","));
            println!("units=null");
            println!("domain_constraints=[]");
            println!("implementation_path=null");
            println!(
                "contract_path={}",
                resolved
                    .registry_entry
                    .and_then(|entry| entry.contract_path)
                    .unwrap_or("null")
            );
            println!(
                "validation_card_path={}",
                resolved
                    .registry_entry
                    .and_then(|entry| entry.validation_card_path)
                    .unwrap_or("null")
            );
            println!(
                "source_seed_path={}",
                resolved
                    .registry_entry
                    .and_then(|entry| entry.source_seed_path)
                    .unwrap_or("null")
            );
            println!("warnings=[]");
        }
        if let Some(runtime_symbol) = resolved.runtime_symbol() {
            println!("runtime_symbol={runtime_symbol}");
        }
        if let Some(output_variable) = resolved.output_variable() {
            println!("output_variable={output_variable}");
        }
        if let Some(summary) = resolved.summary().or_else(|| {
            registry_object.and_then(|object| json_string_field_value(object, "summary"))
        }) {
            println!("summary={summary}");
        }
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
    }
}

fn evaluation_json(
    result: &EvaluationResult,
    resolved: &ResolvedFormula,
    context: CommandContext,
    input_syntax: InputSyntax,
) -> String {
    let registry_object = formula_registry_json_object(resolved.formula_id());
    let mut output = String::from("{\"ok\":true,\"command\":");
    push_json_string(&mut output, context.command());
    output.push_str(",\"json_contract_version\":");
    push_json_string(&mut output, JSON_CONTRACT_VERSION);
    append_context_json_fields(&mut output, context);
    output.push_str(",\"formula_id\":");
    push_json_string(&mut output, resolved.formula_id());
    output.push_str(",\"canonical_formula_id\":");
    push_json_string(&mut output, resolved.formula_id());
    output.push_str(",\"requested_formula_id\":");
    push_json_string(&mut output, &resolved.requested_id);
    output.push_str(",\"alias_used\":");
    push_optional_json_string(&mut output, resolved.alias_used.as_deref());
    output.push_str(",\"legacy_formula_id\":");
    push_optional_json_string(&mut output, resolved.legacy_formula_id());
    output.push_str(",\"runtime_symbol\":");
    push_json_string(&mut output, result.spec.runtime_symbol);
    output.push_str(",\"output_variable\":");
    push_json_string(&mut output, result.spec.output_variable);
    write!(output, ",\"value\":{}", result.value).expect("writing to String cannot fail");
    output.push_str(",\"input_syntax\":");
    push_json_string(&mut output, input_syntax.as_str());
    output.push_str(",\"output\":");
    push_json_string(&mut output, result.spec.output_variable);
    output.push_str(",\"units\":");
    output.push_str(
        registry_object
            .and_then(|object| json_field_value_slice(object, "units"))
            .unwrap_or("null"),
    );
    output.push_str(",\"status\":");
    push_json_string(&mut output, resolved.status());
    output.push_str(",\"execution_policy\":");
    push_optional_json_string(&mut output, resolved.execution_policy());
    output.push_str(",\"quarantine_state\":");
    push_optional_json_string(&mut output, resolved.quarantine_state());
    output.push_str(",\"source_trace\":");
    output.push_str(
        registry_object
            .and_then(|object| json_field_value_slice(object, "source_trace"))
            .unwrap_or("null"),
    );
    output.push_str(",\"registry_schema_version\":");
    push_json_string(&mut output, registry_schema_version());
    output.push_str(",\"source_hash\":");
    push_json_string(&mut output, registry_source_hash());
    output.push_str(",\"validation_status\":");
    push_json_string(&mut output, validation_status());
    output.push_str(",\"warnings\":");
    append_registry_warnings_or_default(&mut output, registry_object);
    output.push_str(",\"safety_notice\":");
    push_json_string(&mut output, safety_notice());
    output.push_str(",\"error\":null}\n");
    output
}

fn output_evaluation(
    result: &EvaluationResult,
    resolved: &ResolvedFormula,
    json: bool,
    context: CommandContext,
    input_syntax: InputSyntax,
) {
    if json {
        print!("{}", evaluation_json(result, resolved, context, input_syntax));
    } else {
        println!("command={}", context.command());
        if let Some(migration_command) = context.migration_command() {
            println!("deprecated_alias=true");
            println!("migration_command={migration_command}");
        }
        println!("formula_id={}", resolved.formula_id());
        println!("canonical_formula_id={}", resolved.formula_id());
        println!("requested_formula_id={}", resolved.requested_id);
        if let Some(alias_used) = resolved.alias_used.as_deref() {
            println!("alias_used={alias_used}");
        }
        if let Some(legacy_formula_id) = resolved.legacy_formula_id() {
            println!("legacy_formula_id={legacy_formula_id}");
        }
        println!("runtime_symbol={}", result.spec.runtime_symbol);
        println!("input_syntax={}", input_syntax.as_str());
        println!("{}={}", result.spec.output_variable, result.value);
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
    }
}

#[cfg(test)]
fn map_inputs(pairs: &[(&str, f64)]) -> BTreeMap<String, f64> {
    pairs
        .iter()
        .map(|(name, value)| ((*name).to_string(), *value))
        .collect()
}

fn self_check_input_arguments(inputs: &[(&str, f64)]) -> Vec<String> {
    inputs
        .iter()
        .flat_map(|(name, value)| [format!("--{name}"), value.to_string()])
        .collect()
}

fn value_check(
    name: &'static str,
    formula_id: &'static str,
    inputs: &[(&str, f64)],
    expected: f64,
) -> SelfCheckResult {
    let arguments = self_check_input_arguments(inputs);
    match evaluate_public_formula_run(formula_id, &arguments) {
        Ok(run) => {
            let envelope = evaluation_json(
                &run.result,
                &run.resolved,
                CommandContext::Namespace {
                    command: "formula run",
                },
                run.input_syntax,
            );
            let passed = run.result.value.to_bits() == expected.to_bits()
                && envelope.starts_with("{\"ok\":true")
                && envelope.contains("\"error\":null");
            SelfCheckResult {
                name,
                formula_id,
                passed,
                detail: format!(
                    "public_path=resolver+status_gate+input_parser+evaluator+success_envelope expected={expected} observed={}",
                    run.result.value
                ),
            }
        }
        Err(error) => SelfCheckResult {
            name,
            formula_id,
            passed: false,
            detail: format!("unexpected_error={} message={error}", error.code()),
        },
    }
}

fn equation_error_check(
    name: &'static str,
    formula_id: &'static str,
    inputs: &[(&str, f64)],
    expected_code: &'static str,
) -> SelfCheckResult {
    let arguments = self_check_input_arguments(inputs);
    match evaluate_public_formula_run(formula_id, &arguments) {
        Err(error @ AppError::Equation { .. }) => {
            let observed_code = error.code();
            let envelope = json_error(&error, Some("formula run"));
            SelfCheckResult {
                name,
                formula_id,
                passed: observed_code == expected_code
                    && envelope.starts_with("{\"ok\":false")
                    && envelope.contains(&format!("\"code\":\"{expected_code}\"")),
                detail: format!(
                    "public_path=resolver+status_gate+input_parser+evaluator+error_envelope expected_error={expected_code} observed_error={observed_code}"
                ),
            }
        }
        Err(error) => SelfCheckResult {
            name,
            formula_id,
            passed: false,
            detail: format!(
                "expected_error={expected_code} observed_error={}",
                error.code()
            ),
        },
        Ok(run) => SelfCheckResult {
            name,
            formula_id,
            passed: false,
            detail: format!(
                "expected_error={expected_code} observed_value={}",
                run.result.value
            ),
        },
    }
}

fn unknown_formula_check() -> SelfCheckResult {
    let formula_id = "formula_vault.m00.canonical.not_present";
    match evaluate_public_formula_run(formula_id, &[]) {
        Err(error @ AppError::UnknownFormula(_)) => {
            let envelope = json_error(&error, Some("formula run"));
            SelfCheckResult {
                name: "unknown_formula_is_rejected",
                formula_id,
                passed: envelope.starts_with("{\"ok\":false")
                    && envelope.contains("\"code\":\"formula_not_found\""),
                detail: "public_path=resolver+error_envelope expected_error=formula_not_found observed_error=formula_not_found".to_string(),
            }
        }
        Err(error) => SelfCheckResult {
            name: "unknown_formula_is_rejected",
            formula_id,
            passed: false,
            detail: format!(
                "expected_error=unknown_formula observed_error={}",
                error.code()
            ),
        },
        Ok(run) => SelfCheckResult {
            name: "unknown_formula_is_rejected",
            formula_id,
            passed: false,
            detail: format!(
                "expected_error=unknown_formula observed_value={}",
                run.result.value
            ),
        },
    }
}

fn run_self_check() -> SelfCheckReport {
    let mut checks = vec![
        value_check(
            "canonical_time_unit_identity",
            "formula_vault.m00.canonical.time_unit_from_mu_du",
            &[("mu", 1.0), ("distance_unit", 1.0)],
            1.0,
        ),
        value_check(
            "canonical_speed_unit_du_tu_identity",
            "formula_vault.m00.canonical.speed_unit_from_du_tu",
            &[("distance_unit", 1.0), ("time_unit", 1.0)],
            1.0,
        ),
        value_check(
            "canonical_speed_unit_mu_du_identity",
            "formula_vault.m00.canonical.speed_unit_from_mu_du",
            &[("mu", 1.0), ("distance_unit", 1.0)],
            1.0,
        ),
        value_check(
            "canonical_mu_identity",
            "formula_vault.m00.canonical.mu_from_units",
            &[("mu", 1.0), ("distance_unit", 1.0), ("time_unit", 1.0)],
            1.0,
        ),
        value_check(
            "signed_distance_to_canonical",
            "formula_vault.m00.canonical.distance_to_canonical",
            &[("distance", -4.0), ("distance_unit", 2.0)],
            -2.0,
        ),
        value_check(
            "signed_distance_from_canonical",
            "formula_vault.m00.canonical.distance_from_canonical",
            &[("canonical_distance", -2.0), ("distance_unit", 2.0)],
            -4.0,
        ),
        value_check(
            "signed_time_to_canonical",
            "formula_vault.m00.canonical.time_to_canonical",
            &[("time", -6.0), ("time_unit", 3.0)],
            -2.0,
        ),
        value_check(
            "signed_time_from_canonical",
            "formula_vault.m00.canonical.time_from_canonical",
            &[("canonical_time", -2.0), ("time_unit", 3.0)],
            -6.0,
        ),
        value_check(
            "signed_speed_to_canonical",
            "formula_vault.m00.canonical.speed_to_canonical",
            &[("speed", -10.0), ("distance_unit", 5.0), ("time_unit", 2.0)],
            -4.0,
        ),
        value_check(
            "signed_speed_from_canonical",
            "formula_vault.m00.canonical.speed_from_canonical",
            &[
                ("canonical_speed", -4.0),
                ("distance_unit", 5.0),
                ("time_unit", 2.0),
            ],
            -10.0,
        ),
        equation_error_check(
            "nonpositive_scale_is_rejected",
            "formula_vault.m00.canonical.distance_to_canonical",
            &[("distance", 1.0), ("distance_unit", 0.0)],
            "non_positive_input",
        ),
        equation_error_check(
            "nonfinite_quantity_is_rejected",
            "formula_vault.m00.canonical.distance_to_canonical",
            &[("distance", f64::NAN), ("distance_unit", 1.0)],
            "out_of_domain",
        ),
        equation_error_check(
            "overflow_is_rejected",
            "formula_vault.m00.canonical.distance_from_canonical",
            &[("canonical_distance", f64::MAX), ("distance_unit", 2.0)],
            "numerical_failure",
        ),
        unknown_formula_check(),
    ];
    if env::var_os(FORCE_SELF_CHECK_FAILURE_ENV).as_deref() == Some(std::ffi::OsStr::new("1")) {
        checks[0].passed = false;
        checks[0].detail =
            "forced deterministic failure for production CLI contract testing".to_string();
    }
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = checks.len() - passed;
    SelfCheckReport {
        checks,
        passed,
        failed,
    }
}

fn self_check_json(report: &SelfCheckReport) -> String {
    let mut output = format!("{{\"ok\":{},\"command\":\"self-check\"", report.failed == 0);
    output.push_str(",\"json_contract_version\":");
    push_json_string(&mut output, JSON_CONTRACT_VERSION);
    append_release_identity_json_fields(&mut output);
    write!(
        output,
        ",\"supported_formula_count\":{},\"dispatchable_formula_count\":{},\"passed\":{},\"failed\":{},\"checks\":[",
        dispatchable_formula_count(),
        dispatchable_formula_count(),
        report.passed,
        report.failed
    )
    .expect("writing to String cannot fail");
    for (index, check) in report.checks.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push_str("{\"name\":");
        push_json_string(&mut output, check.name);
        output.push_str(",\"formula_id\":");
        push_json_string(&mut output, check.formula_id);
        write!(output, ",\"passed\":{}", check.passed).expect("writing to String cannot fail");
        output.push_str(",\"detail\":");
        push_json_string(&mut output, &check.detail);
        output.push('}');
    }
    output.push_str("],\"registry_schema_version\":");
    push_json_string(&mut output, registry_schema_version());
    output.push_str(",\"validation_status\":");
    push_json_string(&mut output, validation_status());
    output.push_str(",\"warnings\":");
    append_default_json_warnings(&mut output);
    output.push_str(",\"safety_notice\":");
    push_json_string(&mut output, safety_notice());
    if report.failed == 0 {
        output.push_str(",\"error\":null}\n");
    } else {
        let error = AppError::SelfCheckFailed {
            failed: report.failed,
        };
        output.push_str(",\"error\":{\"code\":");
        push_json_string(&mut output, error.code());
        output.push_str(",\"message\":");
        push_json_string(&mut output, &error.to_string());
        output.push_str("}}\n");
    }
    output
}

fn output_self_check(report: &SelfCheckReport, json: bool) {
    if json {
        print!("{}", self_check_json(report));
    } else {
        println!("program_name={PROGRAM_NAME}");
        println!("semantic_version={}", package_version());
        println!("release_tier={}", release_tier());
        println!("release_tier_display={}", release_tier_display());
        println!(
            "supported_formula_count={}",
            dispatchable_formula_count()
        );
        println!(
            "dispatchable_formula_count={}",
            dispatchable_formula_count()
        );
        for check in &report.checks {
            let status = if check.passed { "pass" } else { "fail" };
            println!(
                "{status}\t{}\t{}\t{}",
                check.name, check.formula_id, check.detail
            );
        }
        println!("passed={}", report.passed);
        println!("failed={}", report.failed);
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
    }
}

fn print_help() {
    println!(
        "AeroCodex Research Software Alpha CLI\n\n\
usage:\n  aerocodex formula list [--family <family>] [--status <status>] [--executable] [--json]\n  aerocodex formula describe <formula-id> [--json]\n  aerocodex formula status-report [--json]\n  aerocodex formula run <formula-id> [--preliminary] [--input-name <value> ...] [--json]\n  aerocodex version [--json]\n  aerocodex self-check [--json]\n\n\
legacy aliases:\n  aerocodex formulas [--json]        -> aerocodex formula list\n  aerocodex describe <formula-id> [--json]\n                                      -> aerocodex formula describe <formula-id>\n  aerocodex run <formula-id> [--preliminary] name=value ... [--json]\n                                      -> aerocodex formula run <formula-id> [--preliminary] --input-name <value> ...\n\n\
`--json` may appear before or after the command/subcommand. Formula run accepts RR-022 flag-style scalar inputs such as `--degrees 180`; legacy name=value assignments remain compatibility syntax.\n\n\
The historical Beta 1 compatibility surface includes ten governed M00 canonical-unit kernels plus two M00 angle-conversion dispatch specs behind the status gate. The checked-in Formula Registry may also describe inventory-only formulas; registry inclusion is not formula validation, status promotion, certification, execution approval, readiness approval, or regulatory approval.\n\
Release identity: {} {} ({}).\n\
Validation status: {}.\n\
Exit codes: 0 success, 2 usage/input-shape error, 3 unknown formula, 4 equation/domain/numerical/status-gate error, 5 self-check failure.\n\
Safety: {}.",
        package_version(),
        release_tier_display(),
        release_tier(),
        validation_status(),
        safety_notice()
    );
}

fn execute_describe(formula_id: &str, json: bool, context: CommandContext) -> Result<(), AppError> {
    let resolved = resolve_formula(formula_id)
        .ok_or_else(|| AppError::UnknownFormula(formula_id.to_string()))?;
    output_formula_description(&resolved, json, context);
    Ok(())
}

fn execute_run(
    formula_id: &str,
    input_arguments: &[String],
    json: bool,
    context: CommandContext,
) -> Result<(), AppError> {
    let run = evaluate_public_formula_run(formula_id, input_arguments)?;
    output_evaluation(
        &run.result,
        &run.resolved,
        json,
        context,
        run.input_syntax,
    );
    Ok(())
}

fn evaluate_public_formula_run(
    formula_id: &str,
    input_arguments: &[String],
) -> Result<PublicFormulaRun, AppError> {
    let resolved = resolve_formula(formula_id)
        .ok_or_else(|| AppError::UnknownFormula(formula_id.to_string()))?;
    let (preliminary, filtered_input_arguments) = remove_preliminary_flag(input_arguments)?;
    let parsed_inputs = parse_formula_inputs(
        resolved.formula_id(),
        resolved.inputs(),
        &filtered_input_arguments,
    )?;
    if let Some(error) = public_run_precondition_error(&resolved, preliminary) {
        return Err(error);
    }
    let spec = resolved
        .spec
        .expect("public run precondition requires a dispatch specification");
    let result = evaluate_formula(spec.id, &parsed_inputs.inputs)?;
    Ok(PublicFormulaRun {
        resolved,
        result,
        input_syntax: parsed_inputs.syntax,
    })
}

fn execute_formula_namespace(arguments: &[String], json: bool) -> Result<(), AppError> {
    let Some(subcommand) = arguments.first().map(String::as_str) else {
        return Err(AppError::Usage(
            "formula requires a subcommand: list, describe, status-report, or run".to_string(),
        ));
    };

    match subcommand {
        "help" | "--help" | "-h" => {
            if arguments.len() != 1 {
                return Err(AppError::Usage(
                    "formula help does not accept positional arguments".to_string(),
                ));
            }
            print_help();
            Ok(())
        }
        "list" => {
            let filters = parse_formula_list_filters(&arguments[1..])?;
            output_formula_list(
                json,
                CommandContext::Namespace {
                    command: "formula_list",
                },
                &filters,
            );
            Ok(())
        }
        "status-report" => {
            if arguments.len() != 1 {
                return Err(AppError::Usage(
                    "formula status-report does not accept positional arguments".to_string(),
                ));
            }
            output_formula_status_report(json);
            Ok(())
        }
        "describe" => {
            if arguments.len() != 2 {
                return Err(AppError::Usage(
                    "formula describe requires exactly one formula id".to_string(),
                ));
            }
            execute_describe(
                &arguments[1],
                json,
                CommandContext::Namespace {
                    command: "formula describe",
                },
            )
        }
        "run" => {
            if arguments.len() < 2 {
                return Err(AppError::Usage(
                    "formula run requires a formula id followed by --input-name value or legacy name=value inputs".to_string(),
                ));
            }
            execute_run(
                &arguments[1],
                &arguments[2..],
                json,
                CommandContext::Namespace {
                    command: "formula run",
                },
            )
        }
        other => Err(AppError::Usage(format!(
            "unknown formula subcommand `{other}`"
        ))),
    }
}

fn execute(raw_arguments: &[String]) -> Result<(), AppError> {
    let mut arguments = raw_arguments.to_vec();
    let json = remove_json_flag(&mut arguments)?;
    let Some(command) = arguments.first().map(String::as_str) else {
        print_help();
        return Err(AppError::Usage("missing command".to_string()));
    };

    match command {
        "help" | "--help" | "-h" => {
            if arguments.len() != 1 {
                return Err(AppError::Usage(
                    "help does not accept positional arguments".to_string(),
                ));
            }
            print_help();
            Ok(())
        }
        "version" | "--version" | "-V" => {
            if arguments.len() != 1 {
                return Err(AppError::Usage(
                    "version does not accept positional arguments".to_string(),
                ));
            }
            output_version(json, command != "version");
            Ok(())
        }
        "formula" => execute_formula_namespace(&arguments[1..], json),
        "formulas" => {
            let filters = parse_formula_list_filters(&arguments[1..])?;
            output_formula_list(
                json,
                CommandContext::LegacyAlias {
                    command: "formulas",
                    migration_command: "aerocodex formula list",
                },
                &filters,
            );
            Ok(())
        }
        "describe" => {
            if arguments.len() != 2 {
                return Err(AppError::Usage(
                    "describe requires exactly one formula id".to_string(),
                ));
            }
            execute_describe(
                &arguments[1],
                json,
                CommandContext::LegacyAlias {
                    command: "describe",
                    migration_command: "aerocodex formula describe <formula-id>",
                },
            )
        }
        "run" => {
            if arguments.len() < 2 {
                return Err(AppError::Usage(
                    "run requires a formula id followed by --input-name value or legacy name=value inputs".to_string(),
                ));
            }
            execute_run(
                &arguments[1],
                &arguments[2..],
                json,
                CommandContext::LegacyAlias {
                    command: "run",
                    migration_command:
                        "aerocodex formula run <formula-id> [--preliminary] --input-name value ...",
                },
            )
        }
        "self-check" => {
            if arguments.len() != 1 {
                return Err(AppError::Usage(
                    "self-check does not accept positional arguments".to_string(),
                ));
            }
            let report = run_self_check();
            output_self_check(&report, json);
            if report.failed == 0 {
                Ok(())
            } else {
                Err(AppError::SelfCheckFailed {
                    failed: report.failed,
                })
            }
        }
        other => Err(AppError::Usage(format!("unknown command `{other}`"))),
    }
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let json_requested = arguments.iter().any(|argument| argument == "--json");
    let json_command = json_command_for_arguments(&arguments);
    match execute(&arguments) {
        Ok(()) => ExitCode::from(0),
        Err(error) => {
            if !matches!(error, AppError::SelfCheckFailed { .. }) {
                if json_requested {
                    eprint!("{}", json_error(&error, json_command));
                } else {
                    eprintln!("aerocodex error [{}]: {error}", error.code());
                    eprintln!("semantic_version={}", package_version());
                    eprintln!("release_tier={}", release_tier());
                    eprintln!("validation_status={}", validation_status());
                    eprintln!("safety_notice={}", safety_notice());
                }
            }
            ExitCode::from(error.exit_code())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn release_identity_uses_cargo_metadata_and_safe_defaults() {
        assert_eq!(package_version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(release_tier(), "research_software_alpha");
        assert_eq!(release_tier_display(), "Research Software Alpha");
        assert!(!build_commit().is_empty());
        assert!(!build_target().is_empty());
        assert!(matches!(build_profile(), "debug" | "release"));
    }

    #[test]
    fn registry_is_unique_and_complete() {
        assert_eq!(formula_specs().len(), dispatchable_formula_count());
        let canonical_ids: BTreeSet<&str> = formula_specs()
            .iter()
            .map(|spec| spec.canonical_id)
            .collect();
        let ids: BTreeSet<&str> = formula_specs().iter().map(|spec| spec.id).collect();
        let symbols: BTreeSet<&str> = formula_specs()
            .iter()
            .map(|spec| spec.runtime_symbol)
            .collect();
        assert_eq!(canonical_ids.len(), dispatchable_formula_count());
        assert_eq!(ids.len(), dispatchable_formula_count());
        assert_eq!(symbols.len(), dispatchable_formula_count());
        assert!(formula_specs().iter().all(|spec| !spec.inputs.is_empty()));
        assert!(formula_specs().iter().all(|spec| {
            generated_formula_registry::find_by_formula_id(spec.canonical_id)
                .is_some_and(|entry| entry.runtime_symbol == Some(spec.runtime_symbol))
        }));
    }

    #[test]
    fn dispatch_metadata_rejects_unbound_runtime_symbol() {
        let metadata = "schema_version\tcanonical_formula_id\tdispatch_formula_id\truntime_symbol\toutput_variable\tinputs\tsummary\n\
                        aerocodex.cli_dispatch.v1\tfixture.one\tdispatch.one\tunbound_symbol\toutput\tinput\tsummary\n";
        let error = parse_dispatch_metadata(metadata)
            .expect_err("an unbound runtime symbol must not become CLI dispatch metadata");
        assert!(error.contains("unbound runtime symbol"));
    }

    #[test]
    fn exact_signed_conversion_vector_is_executable() {
        let result = evaluate_formula(
            "formula_vault.m00.canonical.distance_to_canonical",
            &map_inputs(&[("distance", -42.0), ("distance_unit", 7.0)]),
        )
        .expect("bounded signed conversion should succeed");
        assert_eq!(result.value, -6.0);
        assert_eq!(result.spec.output_variable, "canonical_distance");
    }

    #[test]
    fn input_shape_is_fail_closed() {
        let missing = evaluate_formula(
            "formula_vault.m00.canonical.distance_to_canonical",
            &map_inputs(&[("distance", 1.0)]),
        )
        .expect_err("missing scale must fail");
        assert_eq!(missing.code(), "missing_input");

        let unexpected = evaluate_formula(
            "formula_vault.m00.canonical.distance_to_canonical",
            &map_inputs(&[("distance", 1.0), ("distance_unit", 1.0), ("unused", 1.0)]),
        )
        .expect_err("unexpected inputs must fail");
        assert_eq!(unexpected.code(), "unexpected_input");
    }

    #[test]
    fn rr022_flag_style_parser_accepts_scalar_inputs_and_legacy_compatibility() {
        let parsed = parse_formula_inputs(
            "m00.angle.deg_to_rad",
            &["degrees"],
            &["--degrees".to_string(), "-180".to_string()],
        )
        .expect("flag-style scalar input should parse");
        assert_eq!(parsed.syntax, InputSyntax::FlagStyle);
        assert_eq!(parsed.inputs.get("degrees"), Some(&-180.0));

        let legacy = parse_formula_inputs(
            "formula_vault.m00.canonical.distance_to_canonical",
            &["distance", "distance_unit"],
            &["distance=-42".to_string(), "distance_unit=7".to_string()],
        )
        .expect("legacy name=value compatibility should parse");
        assert_eq!(legacy.syntax, InputSyntax::LegacyAssignment);
        assert_eq!(legacy.inputs.get("distance"), Some(&-42.0));
        assert_eq!(legacy.inputs.get("distance_unit"), Some(&7.0));
    }

    #[test]
    fn rr022_flag_style_parser_fails_closed_for_mixed_or_vector_inputs() {
        let mixed = parse_formula_inputs(
            "m00.angle.deg_to_rad",
            &["degrees"],
            &[
                "--degrees".to_string(),
                "180".to_string(),
                "degrees=90".to_string(),
            ],
        )
        .expect_err("mixed flag-style and legacy syntax must fail closed");
        assert_eq!(mixed.code(), "usage_error");

        let vector = parse_formula_inputs(
            "m00.vector.norm",
            &["v"],
            &["--v".to_string(), "[1,2,3]".to_string()],
        )
        .expect_err("vector/array inputs are explicitly out of scope for RR-022");
        assert_eq!(vector.code(), "usage_error");
        assert!(vector
            .to_string()
            .contains("vector inputs are not yet supported by CLI run"));
    }

    #[test]
    fn rr023_m00_angle_dispatch_matches_contract_vectors() {
        let deg_to_rad = evaluate_formula(
            "formula_vault.m00.angle.deg2rad",
            &map_inputs(&[("degrees", 180.0)]),
        )
        .expect("RR-023 degrees-to-radians dispatch should be wired");
        assert!((deg_to_rad.value - std::f64::consts::PI).abs() <= 1.0e-15);
        assert_eq!(deg_to_rad.spec.output_variable, "angle_radians");

        let rad_to_deg = evaluate_formula(
            "formula_vault.m00.angle.rad2deg",
            &map_inputs(&[("radians", std::f64::consts::PI)]),
        )
        .expect("RR-023 radians-to-degrees dispatch should be wired");
        assert!((rad_to_deg.value - 180.0).abs() <= 1.0e-12);
        assert_eq!(rad_to_deg.spec.output_variable, "angle_degrees");
    }

    #[test]
    fn promoted_m00_angle_registry_ids_resolve_to_public_dispatch_specs() {
        let deg_to_rad = resolve_formula("m00.angle.deg_to_rad")
            .expect("readable M00 angle registry id should resolve");
        assert_eq!(deg_to_rad.formula_id(), "m00.angle.deg_to_rad");
        assert_eq!(deg_to_rad.status(), "implementation_verified");
        assert_eq!(deg_to_rad.execution_policy(), Some("normal_research"));
        assert_eq!(deg_to_rad.inputs(), &["degrees"]);
        assert_eq!(
            deg_to_rad
                .spec
                .expect("registry row should map to bounded CLI dispatch spec")
                .id,
            "formula_vault.m00.angle.deg2rad"
        );
        assert!(execution_gate_error(&deg_to_rad, false).is_none());

        let rad_to_deg = resolve_formula("formula_vault.m00.angle.rad2deg")
            .expect("legacy M00 angle formula-vault id should resolve");
        assert_eq!(rad_to_deg.formula_id(), "m00.angle.rad_to_deg");
        assert_eq!(
            rad_to_deg.alias_used.as_deref(),
            Some("formula_vault.m00.angle.rad2deg")
        );
        assert_eq!(rad_to_deg.inputs(), &["radians"]);
        assert_eq!(
            rad_to_deg
                .spec
                .expect("legacy alias should map to bounded CLI dispatch spec")
                .id,
            "formula_vault.m00.angle.rad2deg"
        );
    }

    #[test]
    fn self_check_passes_all_bounded_cases() {
        let report = run_self_check();
        assert_eq!(report.failed, 0);
        assert_eq!(report.passed, 14);
        assert_eq!(report.checks.len(), 14);
    }

    #[test]
    fn self_check_failure_json_has_error_envelope() {
        let report = SelfCheckReport {
            checks: vec![SelfCheckResult {
                name: "forced_failure_for_json_envelope_regression",
                formula_id: "formula_vault.m00.canonical.distance_to_canonical",
                passed: false,
                detail: "forced failure for RR-024 self-check envelope regression".to_string(),
            }],
            passed: 0,
            failed: 1,
        };

        let text = self_check_json(&report);
        let python = if cfg!(windows) { "python" } else { "python3" };
        let mut parser = std::process::Command::new(python)
            .arg("-c")
            .arg("import json, sys; json.load(sys.stdin)")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Python should be available for self-check JSON syntax regression checks");
        {
            use std::io::Write as _;
            parser
                .stdin
                .as_mut()
                .expect("parser stdin should be open")
                .write_all(text.as_bytes())
                .expect("self-check JSON should be writable to parser stdin");
        }
        let parsed = parser
            .wait_with_output()
            .expect("python3 JSON parser should exit");
        assert!(
            parsed.status.success(),
            "self-check failure JSON must parse; stdout={} stderr={} json={text}",
            String::from_utf8_lossy(&parsed.stdout),
            String::from_utf8_lossy(&parsed.stderr)
        );
        assert!(
            text.starts_with("{\"ok\":false"),
            "self-check failure must be an ok=false envelope: {text}"
        );
        assert!(text.contains("\"command\":\"self-check\""));
        assert!(text.contains("\"registry_schema_version\":\"aerocodex.formula_registry.v1\""));
        assert!(text.contains("\"warnings\":"));
        assert!(text.contains("\"safety_notice\":"));
        assert!(
            text.contains("\"error\":{\"code\":\"self_check_failed\",\"message\":\"AeroCodex self-check reported 1 failing checks\"}"),
            "self-check failure must populate error.code and error.message: {text}"
        );
        assert!(
            !text.contains("\"error\":null"),
            "self-check failure must not emit error=null: {text}"
        );
        assert_eq!(text.matches("\"ok\":false").count(), 1);
        assert_eq!(text.matches("\"error\":{").count(), 1);
    }

    #[test]
    fn execution_gate_status_policy_fixture_mapping_matches_rr025() {
        assert_eq!(execution_policy_for_status("research_required"), "blocked");
        assert_eq!(
            execution_policy_for_status("equation_traceable"),
            "preliminary_flag_required"
        );
        assert_eq!(
            execution_policy_for_status("implementation_verified"),
            "normal_research"
        );
        assert_eq!(
            execution_policy_for_status("reference_validated"),
            "publication_supporting"
        );

        let research = execution_gate_error_for_parts(
            "fixture.research_required",
            None,
            None,
            "research_required",
            false,
        )
        .expect("research_required must be blocked");
        assert_eq!(research.code(), "execution_blocked_by_status");

        let preliminary_required = execution_gate_error_for_parts(
            "fixture.equation_traceable",
            None,
            None,
            "equation_traceable",
            false,
        )
        .expect("equation_traceable requires --preliminary by default");
        assert_eq!(preliminary_required.code(), "preliminary_flag_required");
        assert!(execution_gate_error_for_parts(
            "fixture.equation_traceable",
            None,
            None,
            "equation_traceable",
            true,
        )
        .is_none());

        assert!(execution_gate_error_for_parts(
            "fixture.implementation_verified",
            None,
            None,
            "implementation_verified",
            false,
        )
        .is_none());
        assert!(execution_gate_error_for_parts(
            "fixture.reference_validated",
            None,
            None,
            "reference_validated",
            false,
        )
        .is_none());

        let m07 = execution_gate_error_for_parts(
            "m07.fixture.candidate",
            Some("m07.fixture"),
            Some("formula_vault.m07.fixture.candidate"),
            "implementation_verified",
            true,
        )
        .expect("M07 candidates must stay blocked even with --preliminary");
        assert_eq!(m07.code(), "m07_candidate_blocked");
    }

    #[test]
    fn self_check_public_preconditions_reject_blocked_release_formula_fixture() {
        let error = public_run_precondition_error_for_parts(
            "m00.angle.deg_to_rad",
            Some("m00"),
            Some("formula_vault.m00.angle.deg2rad"),
            "research_required",
            false,
            true,
        )
        .expect("a blocked release formula must fail the same public precondition used by self-check");
        assert_eq!(error.code(), "execution_blocked_by_status");
    }

    #[test]
    fn self_check_public_preconditions_reject_missing_release_dispatch_fixture() {
        let error = public_run_precondition_error_for_parts(
            "m00.angle.deg_to_rad",
            Some("m00"),
            Some("formula_vault.m00.angle.deg2rad"),
            "implementation_verified",
            false,
            false,
        )
        .expect("an executable release formula without dispatch must fail self-check");
        assert_eq!(error.code(), "formula_dispatch_unavailable");
    }
}
