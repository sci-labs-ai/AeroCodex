use core::{fmt, str::FromStr};

/// Error returned when parsing an AeroCodex status from snake-case text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseStatusError {
    kind: &'static str,
}

impl ParseStatusError {
    #[must_use]
    pub const fn new(kind: &'static str) -> Self {
        Self { kind }
    }

    #[must_use]
    pub const fn kind(&self) -> &'static str {
        self.kind
    }
}

impl fmt::Display for ParseStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unrecognized {} status", self.kind)
    }
}

impl std::error::Error for ParseStatusError {}

/// Verification maturity for a model, equation, implementation, or dataset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    ResearchRequired,
    EquationTraceable,
    ImplementationVerified,
    ReferenceValidated,
    ExperimentValidated,
}

impl VerificationStatus {
    /// Canonical snake-case representation used in validation metadata.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResearchRequired => "research_required",
            Self::EquationTraceable => "equation_traceable",
            Self::ImplementationVerified => "implementation_verified",
            Self::ReferenceValidated => "reference_validated",
            Self::ExperimentValidated => "experiment_validated",
        }
    }

    /// Returns true only for statuses backed by reference data or experiments.
    #[must_use]
    pub const fn has_external_validation_evidence(self) -> bool {
        match self {
            Self::ReferenceValidated | Self::ExperimentValidated => true,
            Self::ResearchRequired | Self::EquationTraceable | Self::ImplementationVerified => {
                false
            }
        }
    }
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for VerificationStatus {
    type Err = ParseStatusError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "research_required" => Ok(Self::ResearchRequired),
            "equation_traceable" => Ok(Self::EquationTraceable),
            "implementation_verified" => Ok(Self::ImplementationVerified),
            "reference_validated" => Ok(Self::ReferenceValidated),
            "experiment_validated" => Ok(Self::ExperimentValidated),
            _ => Err(ParseStatusError::new("verification")),
        }
    }
}

/// Whether a call result is inside the documented model domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidityStatus {
    WithinDocumentedDomain,
    BoundaryCase,
    OutsideDocumentedDomain,
    NotAssessed,
}

impl ValidityStatus {
    /// Canonical snake-case representation used in validation metadata.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinDocumentedDomain => "within_documented_domain",
            Self::BoundaryCase => "boundary_case",
            Self::OutsideDocumentedDomain => "outside_documented_domain",
            Self::NotAssessed => "not_assessed",
        }
    }

    /// Returns true when this status indicates that the call result should not
    /// be treated as silently inside the documented domain.
    #[must_use]
    pub const fn requires_attention(self) -> bool {
        match self {
            Self::BoundaryCase | Self::OutsideDocumentedDomain | Self::NotAssessed => true,
            Self::WithinDocumentedDomain => false,
        }
    }
}

impl fmt::Display for ValidityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ValidityStatus {
    type Err = ParseStatusError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "within_documented_domain" => Ok(Self::WithinDocumentedDomain),
            "boundary_case" => Ok(Self::BoundaryCase),
            "outside_documented_domain" => Ok(Self::OutsideDocumentedDomain),
            "not_assessed" => Ok(Self::NotAssessed),
            _ => Err(ParseStatusError::new("validity")),
        }
    }
}

/// A named assumption attached to an engineering result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assumption {
    pub id: &'static str,
    pub description: &'static str,
}

impl Assumption {
    #[must_use]
    pub const fn new(id: &'static str, description: &'static str) -> Self {
        Self { id, description }
    }
}

/// A non-fatal model warning attached to an engineering result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelWarning {
    pub code: &'static str,
    pub message: &'static str,
}

impl ModelWarning {
    #[must_use]
    pub const fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }
}

/// Traceability and verification metadata for an equation or result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationRecord {
    pub codex_id: &'static str,
    pub status: VerificationStatus,
    pub sources: &'static [&'static str],
    pub notes: &'static str,
}

impl VerificationRecord {
    #[must_use]
    pub const fn new(
        codex_id: &'static str,
        status: VerificationStatus,
        sources: &'static [&'static str],
        notes: &'static str,
    ) -> Self {
        Self {
            codex_id,
            status,
            sources,
            notes,
        }
    }

    #[must_use]
    pub const fn research_required(
        codex_id: &'static str,
        sources: &'static [&'static str],
        notes: &'static str,
    ) -> Self {
        Self::new(
            codex_id,
            VerificationStatus::ResearchRequired,
            sources,
            notes,
        )
    }

    #[must_use]
    pub const fn has_sources(&self) -> bool {
        !self.sources.is_empty()
    }
}

/// A value plus the first layer of evidence and domain metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct EngineeringResult<T> {
    pub value: T,
    pub codex_id: &'static str,
    pub assumptions: Vec<Assumption>,
    pub warnings: Vec<ModelWarning>,
    pub validity: ValidityStatus,
    pub verification: VerificationRecord,
}

impl<T> EngineeringResult<T> {
    /// Creates an engineering result.
    ///
    /// New results default to [`ValidityStatus::NotAssessed`] so callers do not
    /// accidentally imply domain validity before explicitly recording it.
    #[must_use]
    pub fn new(value: T, codex_id: &'static str, verification: VerificationRecord) -> Self {
        debug_assert_eq!(
            codex_id, verification.codex_id,
            "EngineeringResult codex_id should match its VerificationRecord"
        );
        Self {
            value,
            codex_id,
            assumptions: Vec::new(),
            warnings: Vec::new(),
            validity: ValidityStatus::NotAssessed,
            verification,
        }
    }

    /// Creates an engineering result using the Codex ID embedded in the record.
    #[must_use]
    pub fn from_verification(value: T, verification: VerificationRecord) -> Self {
        Self::new(value, verification.codex_id, verification)
    }

    #[must_use]
    pub fn with_assumption(mut self, id: &'static str, description: &'static str) -> Self {
        self.assumptions.push(Assumption::new(id, description));
        self
    }

    #[must_use]
    pub fn with_assumption_record(mut self, assumption: Assumption) -> Self {
        self.assumptions.push(assumption);
        self
    }

    #[must_use]
    pub fn with_warning(mut self, code: &'static str, message: &'static str) -> Self {
        self.warnings.push(ModelWarning::new(code, message));
        self
    }

    #[must_use]
    pub fn with_warning_record(mut self, warning: ModelWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    #[must_use]
    pub fn with_validity(mut self, validity: ValidityStatus) -> Self {
        self.validity = validity;
        self
    }

    #[must_use]
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    #[must_use]
    pub fn verification_status(&self) -> VerificationStatus {
        self.verification.status
    }
}
