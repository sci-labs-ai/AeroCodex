# AI terminology integration

AeroCodex keeps AI-facing terminology explicit, scoped, and source-aware. The public repository tracks the registry data, schemas, examples, and templates needed to build compact terminology context. It does not track deployment helper scripts or generated terminology indexes.

## Operating model

- AI context should be generated from reviewed registry records, not invented from nearby prose.
- Candidate external terms remain candidate/source-scoped until reviewed.
- Ambiguous acronyms such as `RCS`, `CDR`, and `AC` must resolve through local context or be flagged as ambiguous.
- Durable outputs must preserve source-original terms separately from canonical AeroCodex terms when the distinction matters.

## Public checks

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
```

Maintainers may run private pack-generation tooling outside the public repository and summarize results in review notes.
