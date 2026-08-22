# AeroCodex documentation

<!-- aerocodex-current-identity:start -->
Release version: `0.1.0-alpha.1`
Release tier: `research_software_alpha` (`Research Software Alpha`)
Workspace packages: `14`
Registry formulas: `152`
Blocked formulas: `140`
Publicly executable formulas: `12`
<!-- aerocodex-current-identity:end -->

AeroCodex is Research Software Alpha. Start with the path that matches your goal.

## Use the CLI

- [Installation and first run](../README.md)
- [CLI formula quickstart](research_alpha/cli_formula_quickstart.md)
- [Stable JSON contract](research_alpha/json_contract.md)
- [Degrees-to-radians worked example](examples/m00-angle-conversion.md)
- [Friend-test quickstart](testing/friend_test_quickstart.md)

The executable scope is the exact twelve-formula set in `release/release-manifest.toml`. The other 140 registry rows remain blocked.

## Understand release evidence

- [Current release status and Formal-plan ledger](release/v0.1.0-alpha.1-status.md)
- [Machine-readable release manifest](../release/release-manifest.toml)
- [Artifact names and layouts](release/artifact_layout.md)
- [Release build and package policy](development/release_builds.md)
- [CI and local gates](development/ci_gates.md)
- [Public wording and non-claims](assurance/public_wording_guardrails.md)
- [Research-readiness count source](roadmap/research_readiness_counts.md)

## Contribute or review

- [Contribution paths](../CONTRIBUTING.md)
- [Security reporting](../SECURITY.md)
- [Community code of conduct](../CODE_OF_CONDUCT.md)
- [Formula promotion packet template](assurance/promotion_packet_template.md)
- [Formula Registry contract](architecture/formula_registry_contract.md)
- [Formula CLI contract](architecture/formula_cli_contract.md)
- [Math correctness policy](assurance/math_correctness_policy.md)
- [Source licensing checklist](assurance/source_licensing_checklist.md)

## Cite the software and formulas

- [Citation guide](citation.md)
- [CITATION.cff](../CITATION.cff)
- [Source registry](../validation/source_registry/)
- [Validation cards](../validation/cards/)

## API documentation

Tagged and main-branch API/user documentation is published by the governed documentation workflow at [sci-labs-ai.github.io/AeroCodex](https://sci-labs-ai.github.io/AeroCodex/). The API build treats rustdoc warnings as errors.

## Historical records

Phase microtasks, Beta 1 concept files, retired stage plans, and M07 source-accounting waves are retained behind the [historical documentation archive index](archive/README.md). They are point-in-time engineering evidence, not current installation or release instructions.

## Safety boundary

AeroCodex is not certified or authorized for operational, flight, mission, habitat, medical, or regulated use. `implementation_verified` is bounded software evidence and is not scientific reference validation. Follow each formula's domain, units, assumptions, source record, validation record, and caveats.
