#![forbid(unsafe_code)]
//! Shared core types for AeroCodex.
//!
//! This crate intentionally contains no external dependencies. It defines the
//! common result, error, verification, warning, and unit-scalar vocabulary used
//! by the Phase 0.001 equation crates.

mod error;
mod result;
pub mod units;
pub mod validation;

pub use error::AeroError;
pub use result::{
    Assumption, EngineeringResult, ModelWarning, ParseStatusError, ValidityStatus,
    VerificationRecord, VerificationStatus,
};
pub use units::{
    Acceleration, Angle, Area, Density, Force, Gamma, HeatFlux, Length, Mach, Mass, Pressure,
    Temperature, Time, Velocity,
};

/// Standard AeroCodex result alias.
pub type AeroResult<T> = Result<T, AeroError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_all_required_error_variants() {
        let errors = [
            AeroError::NonPositiveInput {
                parameter: "radius",
                value: 0.0,
            },
            AeroError::NegativeInput {
                parameter: "pressure",
                value: -1.0,
            },
            AeroError::OutOfDomain {
                parameter: "gamma",
                value: 0.9,
                expected: "gamma > 1",
            },
            AeroError::RequiresSupersonic {
                mach: 0.8,
                minimum: 1.0,
            },
            AeroError::AmbiguousBranch {
                model: "theta_beta_mach",
                branches: &["weak", "strong"],
            },
            AeroError::NumericalFailure {
                solver: "bisection",
                reason: "failed to bracket root",
            },
            AeroError::UnverifiedSource {
                source_id: "source.pending_review",
            },
        ];

        assert_eq!(errors[0].code(), "non_positive_input");
        assert_eq!(errors[1].code(), "negative_input");
        assert_eq!(errors[2].parameter(), Some("gamma"));
        assert!(errors[3].to_string().contains("supersonic"));
        assert_eq!(errors[4].code(), "ambiguous_branch");
        assert!(!errors[5].is_domain_error());
        assert!(errors[6].to_string().contains("pending_review"));
    }

    #[test]
    fn construct_engineering_result() {
        let record = VerificationRecord::research_required(
            "demo.codex.id",
            &["demo.source"],
            "source pending review",
        );
        let result = EngineeringResult::from_verification(42.0, record)
            .with_assumption("demo.assumption", "idealized scalar demonstration")
            .with_warning("demo.warning", "not an engineering validation")
            .with_validity(ValidityStatus::WithinDocumentedDomain);

        assert_eq!(result.value, 42.0);
        assert_eq!(result.codex_id, "demo.codex.id");
        assert_eq!(result.assumptions.len(), 1);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.has_warnings());
        assert_eq!(
            result.verification_status(),
            VerificationStatus::ResearchRequired
        );
        assert_eq!(result.validity, ValidityStatus::WithinDocumentedDomain);
    }

    #[test]
    fn engineering_result_defaults_to_not_assessed() {
        let record = VerificationRecord::new(
            "demo.default_validity",
            VerificationStatus::EquationTraceable,
            &["demo.source"],
            "demo only",
        );
        let result = EngineeringResult::new(1.0, "demo.default_validity", record);
        assert_eq!(result.validity, ValidityStatus::NotAssessed);
        assert!(result.validity.requires_attention());
    }

    #[test]
    fn verification_statuses_display_as_snake_case() {
        let cases = [
            (VerificationStatus::ResearchRequired, "research_required"),
            (VerificationStatus::EquationTraceable, "equation_traceable"),
            (
                VerificationStatus::ImplementationVerified,
                "implementation_verified",
            ),
            (
                VerificationStatus::ReferenceValidated,
                "reference_validated",
            ),
            (
                VerificationStatus::ExperimentValidated,
                "experiment_validated",
            ),
        ];

        for (status, text) in cases {
            assert_eq!(status.as_str(), text);
            assert_eq!(status.to_string(), text);
            assert_eq!(text.parse::<VerificationStatus>().unwrap(), status);
        }

        assert!("unknown".parse::<VerificationStatus>().is_err());
        assert!(VerificationStatus::ReferenceValidated.has_external_validation_evidence());
        assert!(!VerificationStatus::ResearchRequired.has_external_validation_evidence());
    }

    #[test]
    fn validity_statuses_display_as_snake_case() {
        let cases = [
            (
                ValidityStatus::WithinDocumentedDomain,
                "within_documented_domain",
            ),
            (ValidityStatus::BoundaryCase, "boundary_case"),
            (
                ValidityStatus::OutsideDocumentedDomain,
                "outside_documented_domain",
            ),
            (ValidityStatus::NotAssessed, "not_assessed"),
        ];

        for (status, text) in cases {
            assert_eq!(status.as_str(), text);
            assert_eq!(status.to_string(), text);
            assert_eq!(text.parse::<ValidityStatus>().unwrap(), status);
        }

        assert!("unknown".parse::<ValidityStatus>().is_err());
        assert!(!ValidityStatus::WithinDocumentedDomain.requires_attention());
        assert!(ValidityStatus::NotAssessed.requires_attention());
    }
}
