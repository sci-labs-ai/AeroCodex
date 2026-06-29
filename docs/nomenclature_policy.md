# AeroCodex nomenclature policy

AeroCodex treats terminology as a governed repository artifact. The canonical protocol lives in [`../nomenclature/docs/ACX-NOM-001.md`](../nomenclature/docs/ACX-NOM-001.md), with registries under [`../nomenclature/registry/`](../nomenclature/registry/) and templates under [`../nomenclature/templates/`](../nomenclature/templates/).

## Public-repository status

The public repository tracks the nomenclature policy data, registries, schemas, examples, and templates. Earlier deployment/helper tooling and generated acronym inventories are intentionally not tracked in the public repo. Maintainers may run private registry-generation tools outside this repository, but public CI must remain Rust-only.

## Required public checks

```bash
cargo run -p xtask -- verify --all
cargo run -p xtask -- dependency-policy
```

## Human review rules

- Expand or register durable acronym use before merging public docs, schemas, or APIs.
- Preserve source-original terminology separately from canonical AeroCodex terminology.
- Use proposal templates for new acronyms, symbols, terms, aliases, or waivers.
- Do not treat candidate external acronym lists as approved AeroCodex terminology without review.
