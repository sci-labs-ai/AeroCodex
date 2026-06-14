use core::fmt;

/// Errors returned by checked AeroCodex equations.
///
/// The variants are intentionally specific enough for downstream callers to
/// distinguish input-domain failures, branch-selection failures, numerical
/// failures, and source-traceability failures without parsing display text.
#[derive(Debug, Clone, PartialEq)]
pub enum AeroError {
    /// A parameter must be strictly positive.
    NonPositiveInput { parameter: &'static str, value: f64 },
    /// A parameter must be nonnegative.
    NegativeInput { parameter: &'static str, value: f64 },
    /// A parameter is finite but outside the documented domain.
    OutOfDomain {
        parameter: &'static str,
        value: f64,
        expected: &'static str,
    },
    /// A gas-dynamics relation requires a supersonic Mach number.
    RequiresSupersonic { mach: f64, minimum: f64 },
    /// A model has multiple branches and the API call did not select one.
    AmbiguousBranch {
        model: &'static str,
        branches: &'static [&'static str],
    },
    /// A numerical method failed to converge or could not bracket a solution.
    NumericalFailure {
        solver: &'static str,
        reason: &'static str,
    },
    /// A calculation was requested from an intentionally unverified source.
    UnverifiedSource { source_id: &'static str },
}

impl AeroError {
    /// Stable snake-case error code for logs, validation cards, and tests.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::NonPositiveInput { .. } => "non_positive_input",
            Self::NegativeInput { .. } => "negative_input",
            Self::OutOfDomain { .. } => "out_of_domain",
            Self::RequiresSupersonic { .. } => "requires_supersonic",
            Self::AmbiguousBranch { .. } => "ambiguous_branch",
            Self::NumericalFailure { .. } => "numerical_failure",
            Self::UnverifiedSource { .. } => "unverified_source",
        }
    }

    /// Input parameter associated with the error, when a single parameter is
    /// the most useful diagnostic handle.
    #[must_use]
    pub fn parameter(&self) -> Option<&'static str> {
        match self {
            Self::NonPositiveInput { parameter, .. }
            | Self::NegativeInput { parameter, .. }
            | Self::OutOfDomain { parameter, .. } => Some(parameter),
            Self::RequiresSupersonic { .. } => Some("mach"),
            Self::AmbiguousBranch { .. }
            | Self::NumericalFailure { .. }
            | Self::UnverifiedSource { .. } => None,
        }
    }

    /// Returns true when the error reports a violated input or model domain.
    #[must_use]
    pub fn is_domain_error(&self) -> bool {
        matches!(
            self,
            Self::NonPositiveInput { .. }
                | Self::NegativeInput { .. }
                | Self::OutOfDomain { .. }
                | Self::RequiresSupersonic { .. }
        )
    }
}

impl fmt::Display for AeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonPositiveInput { parameter, value } => {
                write!(f, "parameter `{parameter}` must be > 0, got {value}")
            }
            Self::NegativeInput { parameter, value } => {
                write!(f, "parameter `{parameter}` must be >= 0, got {value}")
            }
            Self::OutOfDomain {
                parameter,
                value,
                expected,
            } => {
                write!(
                    f,
                    "parameter `{parameter}`={value} is outside domain: {expected}"
                )
            }
            Self::RequiresSupersonic { mach, minimum } => {
                write!(
                    f,
                    "relation requires supersonic Mach number >= {minimum}, got {mach}"
                )
            }
            Self::AmbiguousBranch { model, branches } => {
                write!(f, "model `{model}` has ambiguous branches: {branches:?}")
            }
            Self::NumericalFailure { solver, reason } => {
                write!(f, "numerical solver `{solver}` failed: {reason}")
            }
            Self::UnverifiedSource { source_id } => {
                write!(f, "source `{source_id}` is marked unverified")
            }
        }
    }
}

impl std::error::Error for AeroError {}
