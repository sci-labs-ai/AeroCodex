#![forbid(unsafe_code)]
//! User-facing AeroCodex Beta 1 concept command-line interface.
//!
//! This binary exposes a deliberately bounded, machine-readable vertical slice
//! of the already governed M00 canonical-unit conversion family. It is research
//! and preliminary-design software, not operational or certified software.

use aero_codex_astrodynamics::{
    m00_canonical_mu_from_units, m00_canonical_speed_unit_from_du_tu,
    m00_canonical_speed_unit_from_mu_du, m00_canonical_time_unit_from_mu_du,
    m00_distance_from_canonical, m00_distance_to_canonical, m00_speed_from_canonical,
    m00_speed_to_canonical, m00_time_from_canonical, m00_time_to_canonical,
};
use aero_codex_core::AeroError;
use std::{
    collections::BTreeMap,
    env,
    fmt::{self, Write as _},
    process::ExitCode,
};

fn release_channel() -> &'static str {
    "beta1-concept"
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
    "research_required"
}

fn safety_notice() -> &'static str {
    "research/preliminary-design software; not certified, flight-ready, mission-ready, operational, medical, habitat-safe, or approved for regulated use"
}

#[derive(Debug)]
struct FormulaSpec {
    id: &'static str,
    runtime_symbol: &'static str,
    output_variable: &'static str,
    inputs: &'static [&'static str],
    summary: &'static str,
}

fn formula_specs() -> &'static [FormulaSpec] {
    &[
        FormulaSpec {
            id: "formula_vault.m00.canonical.time_unit_from_mu_du",
            runtime_symbol: "m00_canonical_time_unit_from_mu_du",
            output_variable: "time_unit",
            inputs: &["mu", "distance_unit"],
            summary: "TU = sqrt(DU^3 / mu)",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.speed_unit_from_du_tu",
            runtime_symbol: "m00_canonical_speed_unit_from_du_tu",
            output_variable: "speed_unit",
            inputs: &["distance_unit", "time_unit"],
            summary: "speed_unit = DU / TU",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.speed_unit_from_mu_du",
            runtime_symbol: "m00_canonical_speed_unit_from_mu_du",
            output_variable: "speed_unit",
            inputs: &["mu", "distance_unit"],
            summary: "speed_unit = sqrt(mu / DU)",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.mu_from_units",
            runtime_symbol: "m00_canonical_mu_from_units",
            output_variable: "canonical_mu",
            inputs: &["mu", "distance_unit", "time_unit"],
            summary: "canonical_mu = mu * TU^2 / DU^3",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.distance_to_canonical",
            runtime_symbol: "m00_distance_to_canonical",
            output_variable: "canonical_distance",
            inputs: &["distance", "distance_unit"],
            summary: "canonical_distance = distance / DU",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.distance_from_canonical",
            runtime_symbol: "m00_distance_from_canonical",
            output_variable: "distance",
            inputs: &["canonical_distance", "distance_unit"],
            summary: "distance = canonical_distance * DU",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.time_to_canonical",
            runtime_symbol: "m00_time_to_canonical",
            output_variable: "canonical_time",
            inputs: &["time", "time_unit"],
            summary: "canonical_time = time / TU",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.time_from_canonical",
            runtime_symbol: "m00_time_from_canonical",
            output_variable: "time",
            inputs: &["canonical_time", "time_unit"],
            summary: "time = canonical_time * TU",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.speed_to_canonical",
            runtime_symbol: "m00_speed_to_canonical",
            output_variable: "canonical_speed",
            inputs: &["speed", "distance_unit", "time_unit"],
            summary: "canonical_speed = speed * TU / DU",
        },
        FormulaSpec {
            id: "formula_vault.m00.canonical.speed_from_canonical",
            runtime_symbol: "m00_speed_from_canonical",
            output_variable: "speed",
            inputs: &["canonical_speed", "distance_unit", "time_unit"],
            summary: "speed = canonical_speed * DU / TU",
        },
    ]
}

fn supported_formula_count() -> usize {
    formula_specs().len()
}

#[derive(Debug)]
struct EvaluationResult {
    spec: &'static FormulaSpec,
    value: f64,
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
            Self::UnknownFormula(_) => "unknown_formula",
            Self::InvalidAssignment(_) => "invalid_assignment",
            Self::DuplicateInput(_) => "duplicate_input",
            Self::InvalidNumber { .. } => "invalid_number",
            Self::MissingInput { .. } => "missing_input",
            Self::UnexpectedInput { .. } => "unexpected_input",
            Self::Equation { source, .. } => source.code(),
            Self::SelfCheckFailed { .. } => "self_check_failed",
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
            Self::Equation { .. } => 4,
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
            Self::Equation { formula_id, source } => {
                write!(formatter, "formula `{formula_id}` failed: {source}")
            }
            Self::SelfCheckFailed { failed } => {
                write!(
                    formatter,
                    "Beta 1 self-check reported {failed} failing checks"
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

fn validate_input_shape(
    spec: &'static FormulaSpec,
    inputs: &BTreeMap<String, f64>,
) -> Result<(), AppError> {
    for &required in spec.inputs {
        if !inputs.contains_key(required) {
            return Err(AppError::MissingInput {
                formula_id: spec.id,
                input: required,
            });
        }
    }
    for provided in inputs.keys() {
        if !spec.inputs.contains(&provided.as_str()) {
            return Err(AppError::UnexpectedInput {
                formula_id: spec.id,
                input: provided.clone(),
            });
        }
    }
    Ok(())
}

fn evaluate_formula(
    formula_id: &str,
    inputs: &BTreeMap<String, f64>,
) -> Result<EvaluationResult, AppError> {
    let spec =
        formula_spec(formula_id).ok_or_else(|| AppError::UnknownFormula(formula_id.to_string()))?;
    validate_input_shape(spec, inputs)?;

    let value = match spec.id {
        "formula_vault.m00.canonical.time_unit_from_mu_du" => m00_canonical_time_unit_from_mu_du(
            required_input(spec, inputs, "mu")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        "formula_vault.m00.canonical.speed_unit_from_du_tu" => m00_canonical_speed_unit_from_du_tu(
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        "formula_vault.m00.canonical.speed_unit_from_mu_du" => m00_canonical_speed_unit_from_mu_du(
            required_input(spec, inputs, "mu")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        "formula_vault.m00.canonical.mu_from_units" => m00_canonical_mu_from_units(
            required_input(spec, inputs, "mu")?,
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        "formula_vault.m00.canonical.distance_to_canonical" => m00_distance_to_canonical(
            required_input(spec, inputs, "distance")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        "formula_vault.m00.canonical.distance_from_canonical" => m00_distance_from_canonical(
            required_input(spec, inputs, "canonical_distance")?,
            required_input(spec, inputs, "distance_unit")?,
        ),
        "formula_vault.m00.canonical.time_to_canonical" => m00_time_to_canonical(
            required_input(spec, inputs, "time")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        "formula_vault.m00.canonical.time_from_canonical" => m00_time_from_canonical(
            required_input(spec, inputs, "canonical_time")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        "formula_vault.m00.canonical.speed_to_canonical" => m00_speed_to_canonical(
            required_input(spec, inputs, "speed")?,
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        "formula_vault.m00.canonical.speed_from_canonical" => m00_speed_from_canonical(
            required_input(spec, inputs, "canonical_speed")?,
            required_input(spec, inputs, "distance_unit")?,
            required_input(spec, inputs, "time_unit")?,
        ),
        _ => return Err(AppError::UnknownFormula(formula_id.to_string())),
    }
    .map_err(|source| AppError::Equation {
        formula_id: spec.id,
        source,
    })?;

    Ok(EvaluationResult { spec, value })
}

fn parse_assignments(values: &[String]) -> Result<BTreeMap<String, f64>, AppError> {
    let mut inputs = BTreeMap::new();
    for assignment in values {
        let (name, raw_value) = assignment
            .split_once('=')
            .ok_or_else(|| AppError::InvalidAssignment(assignment.clone()))?;
        if name.is_empty() || raw_value.is_empty() {
            return Err(AppError::InvalidAssignment(assignment.clone()));
        }
        let value = raw_value
            .parse::<f64>()
            .map_err(|_| AppError::InvalidNumber {
                input: name.to_string(),
                value: raw_value.to_string(),
            })?;
        if inputs.insert(name.to_string(), value).is_some() {
            return Err(AppError::DuplicateInput(name.to_string()));
        }
    }
    Ok(inputs)
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

fn json_error(error: &AppError) -> String {
    let mut output = String::from("{\"ok\":false,\"error\":{\"code\":");
    push_json_string(&mut output, error.code());
    output.push_str(",\"message\":");
    push_json_string(&mut output, &error.to_string());
    output.push_str("},\"release_channel\":");
    push_json_string(&mut output, release_channel());
    output.push_str(",\"validation_status\":");
    push_json_string(&mut output, validation_status());
    output.push_str(",\"safety_notice\":");
    push_json_string(&mut output, safety_notice());
    output.push_str("}\n");
    output
}

fn output_version(json: bool) {
    if json {
        let mut output = String::from("{\"ok\":true,\"command\":\"version\",\"package_version\":");
        push_json_string(&mut output, package_version());
        output.push_str(",\"release_channel\":");
        push_json_string(&mut output, release_channel());
        output.push_str(",\"build_commit\":");
        push_json_string(&mut output, build_commit());
        output.push_str(",\"build_target\":");
        push_json_string(&mut output, build_target());
        output.push_str(",\"build_profile\":");
        push_json_string(&mut output, build_profile());
        write!(
            output,
            ",\"supported_formula_count\":{},\"validation_status\":",
            supported_formula_count()
        )
        .expect("writing to String cannot fail");
        push_json_string(&mut output, validation_status());
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str("}\n");
        print!("{output}");
    } else {
        println!("AeroCodex {}", package_version());
        println!("release_channel={}", release_channel());
        println!("build_commit={}", build_commit());
        println!("build_target={}", build_target());
        println!("build_profile={}", build_profile());
        println!("supported_formula_count={}", supported_formula_count());
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
    }
}

fn output_formula_list(json: bool) {
    if json {
        let mut output = format!(
            "{{\"ok\":true,\"command\":\"formulas\",\"count\":{},\"validation_status\":",
            supported_formula_count()
        );
        push_json_string(&mut output, validation_status());
        output.push_str(",\"formulas\":[");
        for (index, spec) in formula_specs().iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            output.push_str("{\"formula_id\":");
            push_json_string(&mut output, spec.id);
            output.push_str(",\"runtime_symbol\":");
            push_json_string(&mut output, spec.runtime_symbol);
            output.push_str(",\"output_variable\":");
            push_json_string(&mut output, spec.output_variable);
            output.push_str(",\"inputs\":[");
            for (input_index, input) in spec.inputs.iter().enumerate() {
                if input_index > 0 {
                    output.push(',');
                }
                push_json_string(&mut output, input);
            }
            output.push_str("]}");
        }
        output.push_str("],\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str("}\n");
        print!("{output}");
    } else {
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
        for spec in formula_specs() {
            println!(
                "{}\t{}\t{}\t{}",
                spec.id,
                spec.runtime_symbol,
                spec.output_variable,
                spec.inputs.join(",")
            );
        }
    }
}

fn output_formula_description(spec: &FormulaSpec, json: bool) {
    if json {
        let mut output = String::from("{\"ok\":true,\"command\":\"describe\",\"formula_id\":");
        push_json_string(&mut output, spec.id);
        output.push_str(",\"runtime_symbol\":");
        push_json_string(&mut output, spec.runtime_symbol);
        output.push_str(",\"output_variable\":");
        push_json_string(&mut output, spec.output_variable);
        output.push_str(",\"summary\":");
        push_json_string(&mut output, spec.summary);
        output.push_str(",\"inputs\":[");
        for (index, input) in spec.inputs.iter().enumerate() {
            if index > 0 {
                output.push(',');
            }
            push_json_string(&mut output, input);
        }
        output.push_str("],\"validation_status\":");
        push_json_string(&mut output, validation_status());
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str("}\n");
        print!("{output}");
    } else {
        println!("formula_id={}", spec.id);
        println!("runtime_symbol={}", spec.runtime_symbol);
        println!("output_variable={}", spec.output_variable);
        println!("inputs={}", spec.inputs.join(","));
        println!("summary={}", spec.summary);
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
    }
}

fn output_evaluation(result: &EvaluationResult, json: bool) {
    if json {
        let mut output = String::from("{\"ok\":true,\"command\":\"run\",\"formula_id\":");
        push_json_string(&mut output, result.spec.id);
        output.push_str(",\"runtime_symbol\":");
        push_json_string(&mut output, result.spec.runtime_symbol);
        output.push_str(",\"output_variable\":");
        push_json_string(&mut output, result.spec.output_variable);
        write!(output, ",\"value\":{}", result.value).expect("writing to String cannot fail");
        output.push_str(",\"validation_status\":");
        push_json_string(&mut output, validation_status());
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str("}\n");
        print!("{output}");
    } else {
        println!("formula_id={}", result.spec.id);
        println!("runtime_symbol={}", result.spec.runtime_symbol);
        println!("{}={}", result.spec.output_variable, result.value);
        println!("validation_status={}", validation_status());
        println!("safety_notice={}", safety_notice());
    }
}

fn map_inputs(pairs: &[(&str, f64)]) -> BTreeMap<String, f64> {
    pairs
        .iter()
        .map(|(name, value)| ((*name).to_string(), *value))
        .collect()
}

fn value_check(
    name: &'static str,
    formula_id: &'static str,
    inputs: &[(&str, f64)],
    expected: f64,
) -> SelfCheckResult {
    match evaluate_formula(formula_id, &map_inputs(inputs)) {
        Ok(result) => {
            let passed = result.value.to_bits() == expected.to_bits();
            SelfCheckResult {
                name,
                formula_id,
                passed,
                detail: format!("expected={expected} observed={}", result.value),
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
    match evaluate_formula(formula_id, &map_inputs(inputs)) {
        Err(AppError::Equation { source, .. }) => SelfCheckResult {
            name,
            formula_id,
            passed: source.code() == expected_code,
            detail: format!(
                "expected_error={expected_code} observed_error={}",
                source.code()
            ),
        },
        Err(error) => SelfCheckResult {
            name,
            formula_id,
            passed: false,
            detail: format!(
                "expected_error={expected_code} observed_error={}",
                error.code()
            ),
        },
        Ok(result) => SelfCheckResult {
            name,
            formula_id,
            passed: false,
            detail: format!(
                "expected_error={expected_code} observed_value={}",
                result.value
            ),
        },
    }
}

fn unknown_formula_check() -> SelfCheckResult {
    let formula_id = "formula_vault.m00.canonical.not_present";
    let inputs = BTreeMap::new();
    match evaluate_formula(formula_id, &inputs) {
        Err(AppError::UnknownFormula(_)) => SelfCheckResult {
            name: "unknown_formula_is_rejected",
            formula_id,
            passed: true,
            detail: "expected_error=unknown_formula observed_error=unknown_formula".to_string(),
        },
        Err(error) => SelfCheckResult {
            name: "unknown_formula_is_rejected",
            formula_id,
            passed: false,
            detail: format!(
                "expected_error=unknown_formula observed_error={}",
                error.code()
            ),
        },
        Ok(result) => SelfCheckResult {
            name: "unknown_formula_is_rejected",
            formula_id,
            passed: false,
            detail: format!(
                "expected_error=unknown_formula observed_value={}",
                result.value
            ),
        },
    }
}

fn run_self_check() -> SelfCheckReport {
    let checks = vec![
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
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = checks.len() - passed;
    SelfCheckReport {
        checks,
        passed,
        failed,
    }
}

fn output_self_check(report: &SelfCheckReport, json: bool) {
    if json {
        let mut output = format!(
            "{{\"ok\":{},\"command\":\"self-check\",\"release_channel\":",
            report.failed == 0
        );
        push_json_string(&mut output, release_channel());
        write!(
            output,
            ",\"supported_formula_count\":{},\"passed\":{},\"failed\":{},\"checks\":[",
            supported_formula_count(),
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
        output.push_str("],\"validation_status\":");
        push_json_string(&mut output, validation_status());
        output.push_str(",\"safety_notice\":");
        push_json_string(&mut output, safety_notice());
        output.push_str("}\n");
        print!("{output}");
    } else {
        println!("release_channel={}", release_channel());
        println!("supported_formula_count={}", supported_formula_count());
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
        "AeroCodex Beta 1 concept CLI\n\n\
usage:\n  aerocodex version [--json]\n  aerocodex formulas [--json]\n  aerocodex describe <formula-id> [--json]\n  aerocodex run <formula-id> name=value ... [--json]\n  aerocodex self-check [--json]\n\n\
The Beta 1 concept exposes exactly ten governed M00 canonical-unit formulas.\n\
Validation status: {}.\n\
Exit codes: 0 success, 2 usage/input-shape error, 3 unknown formula, 4 equation/domain/numerical error, 5 self-check failure.\n\
Safety: {}.",
        validation_status(),
        safety_notice()
    );
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
            output_version(json);
            Ok(())
        }
        "formulas" => {
            if arguments.len() != 1 {
                return Err(AppError::Usage(
                    "formulas does not accept positional arguments".to_string(),
                ));
            }
            output_formula_list(json);
            Ok(())
        }
        "describe" => {
            if arguments.len() != 2 {
                return Err(AppError::Usage(
                    "describe requires exactly one formula id".to_string(),
                ));
            }
            let spec = formula_spec(&arguments[1])
                .ok_or_else(|| AppError::UnknownFormula(arguments[1].clone()))?;
            output_formula_description(spec, json);
            Ok(())
        }
        "run" => {
            if arguments.len() < 2 {
                return Err(AppError::Usage(
                    "run requires a formula id followed by name=value inputs".to_string(),
                ));
            }
            let inputs = parse_assignments(&arguments[2..])?;
            let result = evaluate_formula(&arguments[1], &inputs)?;
            output_evaluation(&result, json);
            Ok(())
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
    match execute(&arguments) {
        Ok(()) => ExitCode::from(0),
        Err(error) => {
            if !matches!(error, AppError::SelfCheckFailed { .. }) {
                if json_requested {
                    eprint!("{}", json_error(&error));
                } else {
                    eprintln!("aerocodex error [{}]: {error}", error.code());
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
        assert!(!build_commit().is_empty());
        assert!(!build_target().is_empty());
        assert!(matches!(build_profile(), "debug" | "release"));
    }

    #[test]
    fn registry_is_unique_and_complete() {
        assert_eq!(formula_specs().len(), supported_formula_count());
        let ids: BTreeSet<&str> = formula_specs().iter().map(|spec| spec.id).collect();
        let symbols: BTreeSet<&str> = formula_specs()
            .iter()
            .map(|spec| spec.runtime_symbol)
            .collect();
        assert_eq!(ids.len(), supported_formula_count());
        assert_eq!(symbols.len(), supported_formula_count());
        assert!(formula_specs().iter().all(|spec| !spec.inputs.is_empty()));
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
    fn self_check_passes_all_bounded_cases() {
        let report = run_self_check();
        assert_eq!(report.failed, 0);
        assert_eq!(report.passed, 14);
        assert_eq!(report.checks.len(), 14);
    }
}
