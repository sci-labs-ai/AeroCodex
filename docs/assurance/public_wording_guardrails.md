# Public wording guardrails

RR-002 installs the public wording and forbidden-claim checks for AeroCodex research-readiness work.

## Approved public wording

AeroCodex is intended to become professional-grade, traceable aerospace research software suitable for academic, laboratory, and agency evaluation. It is not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval.

Use the paragraph above verbatim when describing the public posture of this repository.

## Forbidden positive claim markers

The verifier treats the following as forbidden positive claim markers unless the same line is clearly a negated disclaimer, allowed phrase, or guardrail description:

- Forbidden marker: `nasa-ready` / `nasa_ready` / `NASA-ready`
- Forbidden marker: `flight-ready` / `flight_ready`
- Forbidden marker: `mission-ready` / `mission_ready`
- Forbidden marker: `certified for flight`
- Forbidden marker: `operationally approved`
- Forbidden marker: `habitat-safe` / `habitat_safe`
- Forbidden marker: `medical/life-support certified`
- Forbidden marker: `regulatory-approved`

Do not introduce public wording that states or implies NASA approval, flight readiness, mission readiness, operational approval, habitat-safety approval, medical/life-support certification, or regulated-use approval.

## Allowed non-claim phrases

Allowed phrases are explicit disclaimers or guardrail descriptions, including:

- Allowed phrase: `not certified`
- Allowed phrase: `not certified for flight, mission operations, habitat safety, medical/life-support decisions, or regulatory approval`
- Allowed phrase: `does not claim flight readiness, mission readiness, operational approval, or regulated-use approval`
- Allowed phrase: `no certification evidence, no flight-readiness claim, and no mission-readiness claim`
- Allowed phrase: `Forbidden marker: flight-ready`

These negative statements are intentional safety caveats. They must remain distinguishable from positive certification, approval, safety, medical, operational, or mission-readiness claims.

## Verification

The repository gate is:

```bash
cargo run -p xtask -- verify --all
```

The focused helper tests are:

```bash
cargo test -p xtask forbidden_readiness -- --nocapture
```
