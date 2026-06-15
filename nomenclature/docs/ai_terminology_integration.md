# AI Terminology Integration

## Objective

Make AeroCodex terminology easy for AI systems to use without flooding every prompt with the full registry.

The system should expose terminology through three paths:

```text
1. Exact lookup:       lookup token or canonical term.
2. Contextual pack:    retrieve only terms relevant to the current artifact.
3. Lint feedback:      flag undefined, ambiguous, deprecated, or candidate-only terms.
```

## Recommended runtime flow

```text
User/document text
   ↓
Detect tokens and source context
   ↓
Load registry/concepts.yaml, aliases.yaml, acronyms.yaml, terminology_sources.yaml
   ↓
Resolve exact token hits
   ↓
Score domain/context matches
   ↓
Build compact AI terminology pack
   ↓
AI reads pack before answering, editing, linting, or generating code
```

The AI pack is not a glossary dump. It is a small, task-specific contract.

## Tooling commands

Generate a machine-readable index:

```bash
python tooling/aerocodex_terminology.py --root . export-jsonl --output build/terminology/index.jsonl
```

Look up a token:

```bash
python tooling/aerocodex_terminology.py --root . lookup RCS
```

Generate an AI context pack from a document:

```bash
python tooling/aerocodex_terminology.py --root . pack \
  --text-file examples/specs/acronym_resolution_demo.md \
  --domain spacecraft \
  --domain systems_engineering
```

Run nomenclature lint including acronym registry checks:

```bash
python tooling/aerocodex_nom_lint.py --root .
```

Optionally scan Markdown and Rust files for unknown all-caps tokens:

```bash
python tooling/aerocodex_nom_lint.py --root . --scan-acronyms
```

## AI pack format

The default pack format is Markdown because it can be injected into a model prompt, a code-review comment, or a generated document header.

```text
# AeroCodex AI Terminology Pack

Scope hints:
- spacecraft
- systems_engineering

Detected acronyms:
- CDR = Critical Design Review [candidate; systems_engineering, program_lifecycle]
  Source: AeroCodex seed / project_seed
  AI note: CDR often means Critical Design Review in program lifecycle contexts, but the token is ambiguous.

Ambiguous acronyms:
- RCS
  - Reaction Control System [candidate; spacecraft, propulsion, attitude_control]
  - Radar Cross Section [candidate; radar, electromagnetics, signatures]
  Resolver: use local evidence; otherwise return ambiguity.
```

## Resolver behavior for AI assistants

When an AI assistant receives an AeroCodex terminology pack, it must:

- Use approved project terms when the pack gives a single approved result.
- Mention candidate status when a result is not yet approved and the answer depends on it.
- Preserve source-original terms if the source has not been mapped to canonical form.
- Return ambiguity rather than guessing when multiple plausible meanings remain.
- Prefer definitions scoped to the project, contract, standard, or document over general aerospace usage.

## Minimal API shape

AeroCodex systems can expose the registry through functions like:

```ts
type DomainHint = string;

type TerminologyLookup = {
  token: string;
  context?: string;
  domains?: DomainHint[];
};

lookupTerm(request: TerminologyLookup): TermResolution[];
buildTerminologyPack(text: string, domains?: DomainHint[]): TerminologyPack;
proposeAcronym(candidate: AcronymCandidate, evidence: Evidence[]): ReviewTicket;
checkTerminology(documentId: string): TerminologyDiagnostic[];
```

The important part is not the exact API. The important part is that AI tools retrieve small, scoped terminology packets rather than relying on memory or a giant static prompt.

## Pack injection rule

For any AI task involving aerospace documents, source ingestion, requirements, design reviews, schemas, code generation, or report writing, inject a pack when any of these are true:

```text
- The text contains two or more all-caps tokens of length 2–10.
- The document type is a specification, requirement, ICD, review package, test plan, procedure, or source import.
- The task asks for normalization, extraction, summarization, code generation, or schema design.
- The user asks what an acronym means.
```

## Suggested prompt prefix

```text
You are operating under ACX-NOM-001. Treat terminology as scoped bindings. Use the following terminology pack as the governing vocabulary for this task. If a token is ambiguous or candidate-only, say so instead of silently choosing a meaning.
```
