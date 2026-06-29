# ACX-NOM-001 — AeroCodex Nomenclature, Symbol, and Identifier Protocol

**Protocol ID:** `ACX-NOM-001`
**Version:** `0.2.0-draft`
**Status:** Draft for adoption
**Scope:** Rust code, schemas, mathematical specifications, derivations, source ingestion, generated docs, reports, acronym registries, external terminology mappings, and AI terminology packs.

## 1. Governing principle

AeroCodex treats nomenclature as a **scoped binding system**, not a flat glossary or acronym list.

A spelling like `n`, `N`, `T`, `x`, `id`, `station`, `leg`, `tail_number`, `RCS`, `CDR`, or `AC` is not a meaning. It is only a **surface form**. Meaning exists only after the surface form resolves to a declared symbol, term, type, unit, frame, time scale, or source mapping.

> Every meaningful occurrence must resolve to exactly one declared binding in its namespace and scope. Zero matches is an error. More than one match is an ambiguity error.

## 2. Definitions

### 2.1 Surface form

A **surface form** is the literal text, token, or glyph seen by a human or parser.

Examples:

```text
n
N
T
x_i
xᵢ
Δt
tail_number
AircraftId
'a
```

A surface form is not inherently meaningful.

### 2.2 Binding

A **binding** is the approved meaning assigned to a surface form inside a namespace and scope.

Example:

```yaml
symbol_id: acx:math:trajectory.fit.window:E014:sample_count:v1
surface_form: n
display_form: n_s
role: sample_count
domain: natural_number
unit: count
scope: acx.math.trajectory.fit.window.E014
rust_identifier: sample_count
status: approved
```

### 2.3 Scope

A **scope** is the region where a binding may be referenced.

AeroCodex scopes are explicit:

```text
global
domain
module
artifact
section
equation
function
block
source-document
source-field
```

Example:

```text
acx.math.trajectory.fit.window.E014
```

### 2.4 Namespace

A **namespace** is the category of meaning in which a name is resolved.

Required namespaces:

```text
domain.term
source.term
schema.field
schema.enum
rs.type
rs.value
rs.macro
rs.lifetime
rs.label
rs.field::<Type>
math.symbol
math.operator
math.function
unit
coordinate_frame
time_scale
alias
acronym
source.acronym
terminology.source
deprecated
```

### 2.5 Symbol identity

A symbol’s identity is not its glyph. It is the tuple:

```text
(namespace, scope, semantic_role, domain/type, unit/frame/time_scale, provenance)
```

Therefore these are different symbols even if they share the same surface form:

```text
n = sample count
n = node count
n = normal vector
n = polynomial order
n = index upper bound
```

## 3. Non-negotiable invariants

### INV-001 — No context-free symbols

No symbol may be used in a specification, derivation, schema, code comment, or implementation note unless it is declared in the nearest symbol table or inherited from an explicitly named parent scope.

Bad:

```text
n is the number of samples.
```

Good:

```text
In equation E014, let n_s ∈ ℕ be the number of samples in the fitting window.
```

### INV-002 — One occurrence resolves to one binding

Resolution result:

```text
1 binding   valid
0 bindings  unbound-symbol error
2+ bindings ambiguity error
```

### INV-003 — Same glyph does not imply same meaning

The glyph `n` in one equation and the glyph `n` in another equation are unrelated unless they explicitly reference the same `symbol_id`.

### INV-004 — Different glyphs may still represent the same concept

Aliases are allowed, but must map to a canonical binding.

Example:

```text
tail number
N-number
registration mark
aircraft registration
```

These may map to one canonical AeroCodex concept, but only through an explicit alias record.

### INV-005 — Units, dimensions, frames, and time scales are semantic identity

A physical quantity is not fully defined unless its unit and relevant frame are known.

Examples:

```text
altitude_m_ellipsoid
altitude_ft_msl
ground_speed_mps
indicated_airspeed_kt
heading_true_deg
heading_magnetic_deg
timestamp_utc
timestamp_gps
```

`altitude` alone is not acceptable in durable schemas or core code.

### INV-006 — Rust code must use semantic identifiers

Mathematical notation may be compact. Rust implementation code must be semantic.

Bad:

```rust
let n = samples.len();
```

Good:

```rust
let sample_count = samples.len();
```

Better when nonzero is required:

```rust
let sample_count = NonZeroUsize::new(samples.len())
    .ok_or(WindowError::EmptySampleSet)?;
```

### INV-007 — Source terms are preserved, not trusted

A source document may use ambiguous or inconsistent language. AeroCodex must preserve source-original wording for traceability, but internal schemas must map that wording to canonical terms.

```yaml
source_term: Tail No.
canonical: aircraft_registration
source: customer_import_2026_06
confidence: approved
```

### INV-008 — Meaning changes are versioned, not silently edited

Changing a symbol’s definition, unit, frame, domain, or Rust bridge is a breaking semantic change.

Do not mutate the old symbol. Supersede it:

```yaml
symbol_id: acx:math:trajectory.fit:E014:sample_count:v1
status: superseded
superseded_by: acx:math:trajectory.fit:E014:observation_count:v2
```


### INV-009 — Acronyms are scoped bindings

No acronym, abbreviation, or initialism may be normalized by token alone.

Examples:

```text
RCS = Reaction Control System      in spacecraft propulsion / attitude-control context
RCS = Radar Cross Section          in radar / electromagnetic-signature context
CDR = Critical Design Review       in program lifecycle context
AC  = Advisory Circular            in FAA/regulatory context
AC  = Alternating Current          in electrical-power context
```

Acronym resolution must consider token, source authority, domain, document type, first-use expansion, and nearby context.

### INV-010 — AI receives scoped terminology, not a full glossary dump

AI systems must be given a compact terminology pack for the current task instead of the entire registry.

The pack must include only relevant terms, known collisions, source authority, approval status, and ambiguity instructions. If no single binding is supported, the AI must return an ambiguity rather than inventing or guessing an expansion.

## 4. Rust-specific protocol

AeroCodex overlays stricter rules on top of valid Rust.

### 4.1 Rust namespace mapping

| AeroCodex namespace | Rust category | Examples |
|---|---|---|
| `rs.type` | Type namespace | `AircraftId`, `Trajectory`, `SampleWindow<T>` |
| `rs.value` | Value namespace | `sample_count`, `parse_route`, `MAX_SEGMENTS` |
| `rs.macro` | Macro namespace | `symbol_bridge!`, `assert_units!` |
| `rs.lifetime` | Lifetime namespace | `'src`, `'de`, `'a` |
| `rs.label` | Label namespace | `'retry_loop` |
| `rs.field::<Type>` | Field scoped to owning type | `TrajectoryPoint.timestamp_utc` |

### 4.2 Rust casing

| Rust item | Required convention |
|---|---|
| Modules | `snake_case` |
| Functions | `snake_case` |
| Methods | `snake_case` |
| Local variables | `snake_case` |
| Fields | `snake_case` |
| Types | `UpperCamelCase` |
| Traits | `UpperCamelCase` |
| Enum variants | `UpperCamelCase` |
| Constants | `SCREAMING_SNAKE_CASE` |
| Statics | `SCREAMING_SNAKE_CASE` |
| Type parameters | concise `UpperCamelCase`, usually `T` only in small generic contexts |
| Lifetimes | lowercase, preferably semantic in durable APIs |

### 4.3 Rust identifiers

Durable Rust identifiers must be ASCII `snake_case`, `UpperCamelCase`, or `SCREAMING_SNAKE_CASE` unless a documented exception exists.

Allowed in docs/math:

```text
Δt
θ
φ
xᵢ
```

Not allowed as ordinary Rust identifiers in core code:

```rust
let Δt = 0.1;
let θ = heading;
```

Use:

```rust
let delta_time_seconds = 0.1;
let heading_radians = heading;
```

### 4.4 Keywords and raw identifiers

No canonical AeroCodex term, schema field, Rust item, or generated identifier may use a Rust strict or reserved keyword.

Raw identifiers are allowed only for FFI, generated bindings, or compatibility with an external source.

Allowed only with waiver:

```rust
let r#type = source_record.type_field;
```

Preferred:

```rust
let record_type = source_record.type_field;
```

### 4.5 Single-letter Rust identifiers

Single-letter identifiers are unsafe by default.

Allowed without waiver:

```rust
T
E
'a
```

Only in conventional generic/lifetime contexts.

Local loop exception:

```rust
for i in 0..3 {
    // Very small, purely local loop.
}
```

Not allowed in durable logic:

```rust
for i in 0..n {
    process(x[i]);
}
```

Use:

```rust
for sample_index in 0..sample_count {
    process(position_samples[sample_index]);
}
```

### 4.6 Const generics

Bad unless explicitly bridged to math notation:

```rust
struct Matrix<T, const N: usize, const M: usize> {
    data: [[T; M]; N],
}
```

Preferred:

```rust
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}
```

Domain example:

```rust
struct SampleWindow<const SAMPLE_COUNT: usize> {
    samples: [TrajectorySample; SAMPLE_COUNT],
}
```

### 4.7 Lifetimes

Bad in durable APIs:

```rust
struct SourceView<'a> {
    raw: &'a str,
}
```

Better:

```rust
struct SourceView<'src> {
    raw: &'src str,
}
```

### 4.8 Shadowing

No semantic identifier may be shadowed with a different semantic role.

Allowed:

```rust
let raw_timestamp = source.timestamp;
let timestamp_utc = parse_utc_timestamp(raw_timestamp)?;
```

Not allowed:

```rust
let n = samples.len();
let n = nodes.len();
```

## 5. Algebraic-symbol protocol

### 5.1 Mathematical notation is local by default

No algebraic symbol is globally meaningful unless explicitly registered as globally reserved.

Globally reserved examples:

```text
ℕ natural numbers
ℤ integers
ℝ real numbers
∅ empty set
π pi
∑ summation operator
∏ product operator
```

Not globally reserved:

```text
n N m i j k x y z t r q p T E F
```

These must be declared locally.

### 5.2 Equation symbol table required

Every durable equation must have a symbol table.

Template:

```yaml
equation_id: E-trajectory-fit-014
scope: acx.math.trajectory.fit.window.E014
formula: "v_hat = (x[n_s - 1] - x[0]) / ((n_s - 1) * delta_t)"
symbols:
  - display_form: v_hat
    role: estimated_velocity
    domain: vector
    unit: meters_per_second
    frame: local_tangent_ned
    rust_identifier: estimated_velocity_mps
```

### 5.3 Prefer semantic subscripts over bare symbols

Bad:

```text
n = number of samples
m = number of nodes
```

Better:

```text
n_s = sample count
n_v = node count
```

Best in durable specs:

```text
n_samples = sample count
n_nodes = node count
```

### 5.4 LaTeX macro rule

In long-lived specifications, raw single-letter variables should be avoided in source. Use semantic macros.

Bad:

```latex
v = \frac{x_{n-1} - x_0}{(n-1)\Delta t}
```

Good:

```latex
\EstimatedVelocity =
  \frac{\PositionSample_{\SampleCount - 1} - \PositionSample_0}
       {(\SampleCount - 1)\SampleInterval}
```

### 5.5 Index variables

Index variables must be declared.

Allowed:

```text
Let i ∈ {0, …, n_s - 1} index samples.
```

Not allowed:

```text
for all i
```

unless `i` was already declared in the same equation or inherited scope.

### 5.6 Subscripts and superscripts

Subscripts and superscripts are semantic, not decorative.

Examples:

```text
x_i  value of x at index i
x_t  value of x at time t
v_N  north component of velocity
v_n  nth velocity sample
R_B  rotation matrix for body frame
R^2  square of R
```

Every subscript and superscript must be declared as an index, frame, component, exponent, label, version, or semantic qualifier.

### 5.7 Operators and functions

Custom operators and function symbols must be declared.

```yaml
operator: ⊕
symbol_id: acx:math:geodesy:compose_transform:v1
role: transform_composition
associative: true
commutative: false
domain: Transform × Transform -> Transform
```

## 6. Rust–math bridge protocol

Every mathematical symbol used in implementation must have a bridge to Rust.

Example:

```yaml
bridge_id: acx:bridge:trajectory.fit.window:E014:v1
equation_id: E-trajectory-fit-014
bindings:
  - math_symbol: n_s
    symbol_id: acx:math:trajectory.fit.window:E014:sample_count:v1
    rust_identifier: sample_count
    rust_type: usize
    unit: count
```

Code comments must reference bridge IDs:

```rust
// Implements E-trajectory-fit-014.
// Bridge: acx:bridge:trajectory.fit.window:E014:v1
let sample_count = position_samples.len();
```

## 7. Canonical term protocol

Every durable domain term must have a canonical record.

```yaml
canonical: aircraft_registration
display_label: Aircraft Registration
namespace: domain.term
definition: Civil registration identifier assigned to an aircraft.
status: approved
aliases:
  - tail_number
  - n_number
  - registration_mark
rust:
  type_name: AircraftRegistration
  field_name: aircraft_registration
schema:
  field_name: aircraft_registration
```

### 7.1 Alias direction

Aliases map toward canonical terms. Canonical terms do not map sideways.

Allowed:

```text
tail_number -> aircraft_registration
n_number -> aircraft_registration
```

Not allowed:

```text
aircraft_registration -> tail_number
```

### 7.2 Ambiguous aliases

An alias may not map automatically to multiple canonical terms.

```yaml
alias: station
status: ambiguous
candidates:
  - airport_station
  - maintenance_station
  - fuselage_station
  - reporting_station
resolution_required: true
```


## 8. Acronym and external terminology protocol

Aerospace acronyms are overloaded and authority-dependent. AeroCodex stores acronym meanings in `registry/acronyms.yaml` and external source authorities in `registry/terminology_sources.yaml`.

### 8.1 One record per meaning

The acronym registry uses one record per meaning, not one record per token.

```yaml
acronym_id: acx:acr:space:rcs:reaction_control_system:v1
token: RCS
expansion: Reaction Control System
namespace: acronym
canonical: reaction_control_system
domains:
  - spacecraft
  - propulsion
  - attitude_control
status: candidate
collision_group: acx:collision:acr:rcs:v1
source:
  authority: AeroCodex seed
  source_id: acx:source:internal_seed:v1
  authority_rank: project_seed
  evidence: Verify against vehicle subsystem glossary before approving.
first_use:
  requires_definition: true
disambiguation:
  signals:
    - thruster
    - attitude control
    - propellant
  reject_if_near:
    - radar
    - cross section
```

### 8.2 Candidate records are not authority

Starter-kit records may use `status: candidate` to make discovery and AI surfacing useful. Candidate records must not be treated as approved project terminology until reviewed.

### 8.3 Source precedence

Resolve acronym and terminology meanings in this order:

```text
1. Contract, project, or program glossary governing the artifact.
2. Applicable standard or regulation named by the artifact.
3. Agency or standards-body glossary in the same domain.
4. Controlled vocabulary or thesaurus.
5. Community/reference source.
6. AeroCodex candidate seed.
7. AI-proposed candidate.
```

### 8.4 First-use expansion

Acronyms should be expanded at first durable use unless a document-level terminology pack or governing glossary explicitly defines them.

Good:

```text
The Preliminary Design Review (PDR) package shall include the interface-control baseline.
```

Bad:

```text
The PDR package shall include the ICD baseline.
```

unless both acronyms are already defined in the artifact or its terminology pack.

### 8.5 Ambiguity behavior

When an acronym token has multiple plausible meanings:

```text
1. Preserve the token.
2. Return all plausible candidate meanings.
3. Do not normalize to a canonical term.
4. Emit ACX-NOM-E014 in durable contexts.
5. Require local context, first-use expansion, or a document-level terminology pack.
```

### 8.6 AI terminology packs

AI tools should use a maintainer-provided terminology-pack service outside the public repository, or an equivalent reviewed process, to receive a small scoped pack containing detected acronyms, canonical terms, alias mappings, collision warnings, and source status.

Example pack fragment:

```text
- RCS has multiple candidate meanings:
  - Reaction Control System [spacecraft, propulsion]
  - Radar Cross Section [radar, electromagnetics]
Resolver rule: choose only when local context is explicit; otherwise mark ambiguous.
```

## 9. Units, frames, and physical quantities

### 9.1 Physical quantity schema

```yaml
quantity_id: acx:quantity:velocity.ground_speed:v1
canonical: ground_speed
dimension: length / time
canonical_unit: meters_per_second
allowed_display_units:
  - knots
  - kilometers_per_hour
  - miles_per_hour
frame_required: true
time_scale_required: false
rust_type: GroundSpeed
```

### 9.2 Unit-bearing identifiers

When Rust types encode units, identifiers may omit unit suffixes:

```rust
let ground_speed: GroundSpeed = GroundSpeed::from_knots(120.0);
```

When using primitive types, unit suffixes are required:

```rust
let ground_speed_knots: f64 = 120.0;
let ground_speed_mps: f64 = knots_to_mps(ground_speed_knots);
```

### 9.3 Frame-bearing identifiers

Bad:

```rust
let velocity = compute_velocity(samples);
```

Good:

```rust
let velocity_ned_mps = compute_velocity_ned_mps(samples);
```

Better:

```rust
let velocity: Velocity<NedFrame, MetersPerSecond> = compute_velocity(samples);
```

### 9.4 Time-bearing identifiers

Bad:

```rust
let timestamp = source.time;
```

Good:

```rust
let timestamp_utc = source.time;
let timestamp_gps = convert_utc_to_gps(timestamp_utc)?;
```

## 10. Source ingestion protocol

Ingestion must store the source-original term exactly.

```yaml
source_field: Tail No.
source_value: N123AB
canonical_field: aircraft_registration
normalized_value: N123AB
mapping_status: approved
```

Never infer canonical meaning from spelling alone.

Bad ingestion rule:

```text
If field contains "tail", map to aircraft_registration.
```

Good ingestion rule:

```yaml
rule_id: acx:ingest:registration:customer_a:v1
source_field_exact: Tail No.
source_document_type: customer_a_aircraft_export
canonical_field: aircraft_registration
approved_by: nomenclature_owner
```

## 11. Symbol registry format

Machine-readable registry files should live under:

```text
registry/concepts.yaml
registry/aliases.yaml
registry/acronyms.yaml
registry/terminology_sources.yaml
registry/symbols.yaml
registry/units.yaml
registry/frames.yaml
registry/bridges.yaml
registry/waivers.yaml
```

## 12. Name-resolution algorithm

```text
resolve(occurrence):
  1. Determine occurrence namespace.
  2. Determine lexical/document scope.
  3. Load local symbol table.
  4. Load explicitly inherited parent scopes.
  5. Filter by namespace.
  6. Filter by surface form, alias, acronym token, or source term.
  7. Filter by applicable source authority, document type, and source context.
  8. Filter by domain hints and nearby disambiguation signals.
  9. If zero candidates: ERROR UnboundOccurrence.
 10. If more than one candidate: ERROR AmbiguousOccurrence.
 11. If candidate is deprecated: WARN or ERROR depending on context.
 12. If candidate is only candidate/external: WARN unless explicitly accepted by the artifact.
 13. If unit/frame/time constraints mismatch: ERROR SemanticMismatch.
 14. Return resolved binding.
```

Nearest scope does not automatically win if it changes meaning. Shadowing requires an explicit `shadows:` declaration.

## 13. Review and approval workflow

### 13.1 New term proposal

Use `templates/NOM_PROPOSAL.md`.

### 13.2 New algebraic symbol proposal

Use `templates/SYMBOL_PROPOSAL.md`.

### 13.3 Approval levels

| Change type | Required review |
|---|---|
| New alias | Nomenclature reviewer |
| New acronym meaning | Nomenclature reviewer + domain reviewer |
| New external terminology source | Nomenclature owner |
| New canonical term | Nomenclature owner |
| New physical quantity | Nomenclature owner + domain reviewer |
| New algebraic symbol | Spec owner |
| New Rust bridge | Spec owner + Rust reviewer |
| Unit/frame change | Domain reviewer + breaking-change review |
| Meaning change | New version required |
| Deprecated term removal | Migration review |

## 14. CI and lint rules

Error-level checks fail CI:

```text
ACX-NOM-E001  Unbound symbol
ACX-NOM-E002  Ambiguous symbol
ACX-NOM-E003  Alias maps to multiple canonical terms without context
ACX-NOM-E004  Unit missing from physical quantity
ACX-NOM-E005  Frame missing from frame-dependent quantity
ACX-NOM-E006  Rust identifier uses forbidden bare symbol
ACX-NOM-E007  Rust identifier uses deprecated canonical term
ACX-NOM-E008  Equation lacks symbol table
ACX-NOM-E009  Math symbol lacks Rust bridge where implemented
ACX-NOM-E010  Semantic meaning changed without new version
ACX-NOM-E011  Raw Rust identifier used without waiver
ACX-NOM-E012  Source field mapped by heuristic without approved rule
ACX-NOM-E013  Unregistered acronym in durable context
ACX-NOM-E014  Ambiguous acronym without contextual resolution
ACX-NOM-E015  Acronym references missing terminology source
ACX-NOM-E016  Acronym missing first-use policy
ACX-NOM-E017  Acronym collision lacks disambiguation signals
```

Warning-level checks:

```text
ACX-NOM-W001  Single-letter local loop variable
ACX-NOM-W002  Identifier lacks unit suffix while using primitive numeric type
ACX-NOM-W003  Deprecated alias appears in documentation
ACX-NOM-W004  Same glyph reused in disjoint scope
ACX-NOM-W005  Generic parameter could be more semantic
ACX-NOM-W006  Shadowing detected with same semantic role
ACX-NOM-W007  Candidate acronym used in durable context
ACX-NOM-W008  Acronym should be expanded at first use
ACX-NOM-W009  AI pack includes unresolved candidate
```

## 15. Waiver protocol

```yaml
waiver_id: ACX-NOM-WAIVER-0007
rule: ACX-NOM-E006
surface_form: i
context: src/math/small_matrix.rs
reason: Conventional 3x3 matrix implementation; loop scope <= 5 lines.
approved_by: rust_reviewer
expires: 2026-12-31
```

Permanent waiver example:

```yaml
waiver_id: ACX-NOM-WAIVER-0011
rule: ACX-NOM-E011
surface_form: r#type
context: generated/third_party_binding.rs
reason: External schema field is named "type"; generated binding preserves source compatibility.
approved_by: nomenclature_owner
expires: never
```

## 16. Immediate adoption rule

Effective immediately:

> In math, declare every symbol. In Rust, spell out every meaning. In schemas, use only canonical terms. In ingestion, preserve source wording but map it explicitly. For AI, inject scoped terminology packs and surface ambiguity.
