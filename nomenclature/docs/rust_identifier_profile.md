# Rust Identifier Profile for AeroCodex

## Purpose

This profile makes valid Rust naming stricter for AeroCodex, especially where code implements math, aviation data processing, unit conversions, or source ingestion.

## Default rule

> Rust identifiers should be semantic enough that they remain understandable without the surrounding derivation.

## Allowed casing

```text
snake_case             variables, functions, modules, fields
UpperCamelCase         types, traits, enum variants
SCREAMING_SNAKE_CASE   constants and statics
```

## Single-letter identifiers

### Allowed

```rust
fn identity<T>(value: T) -> T {
    value
}

fn parse<'src>(source: &'src str) -> SourceView<'src> {
    SourceView { raw: source }
}
```

### Discouraged or forbidden in durable logic

```rust
let n = samples.len();
let x = positions;
let dt = 0.1;
let v = estimate(x, dt);
```

Use:

```rust
let sample_count = samples.len();
let position_samples = positions;
let sample_interval_seconds = 0.1;
let estimated_velocity_mps =
    estimate_velocity(position_samples, sample_interval_seconds);
```

## Const generics

Prefer semantic const generic names.

Bad:

```rust
struct Matrix<T, const N: usize, const M: usize> {
    data: [[T; M]; N],
}
```

Good:

```rust
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}
```

Domain-specific:

```rust
struct SampleWindow<const SAMPLE_COUNT: usize> {
    samples: [TrajectorySample; SAMPLE_COUNT],
}
```

## Lifetimes

Public or durable APIs should use semantic lifetimes when the role matters.

Bad:

```rust
struct SourceView<'a> {
    raw: &'a str,
}
```

Good:

```rust
struct SourceView<'src> {
    raw: &'src str,
}
```

## Units and primitive numeric types

When using primitive numeric types, encode the unit in the identifier.

Bad:

```rust
let speed: f64 = 120.0;
let altitude: f64 = 3000.0;
```

Good:

```rust
let ground_speed_knots: f64 = 120.0;
let altitude_ft_msl: f64 = 3000.0;
```

Better:

```rust
let ground_speed = GroundSpeed::from_knots(120.0);
let altitude = Altitude::<Msl, Feet>::new(3000.0);
```

## Frames

Frame-dependent vectors must encode the frame in the type or identifier.

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

## Raw identifiers

Raw identifiers such as `r#type` require a waiver unless they appear in generated external bindings.

Bad:

```rust
let r#type = source.type;
```

Good:

```rust
let record_type = source.type_field;
```

## Shadowing

Semantic shadowing is forbidden.

Bad:

```rust
let count = samples.len();
let count = graph.nodes().len();
```

Good:

```rust
let sample_count = samples.len();
let node_count = graph.nodes().len();
```

## Acronyms in Rust identifiers

Acronyms may appear in Rust identifiers only when they are common, approved, and clearer than spelling out the full term. Prefer semantic words in durable APIs.

Bad:

```rust
let rcs = compute_rcs(input);
```

Better when meaning is spacecraft subsystem:

```rust
let reaction_control_system = vehicle.reaction_control_system();
let rcs_thruster_allocation = allocate_reaction_control_thrusters(commands);
```

Better when meaning is radar signature:

```rust
let radar_cross_section_m2 = estimate_radar_cross_section(target_mesh);
```

For colliding acronyms such as `RCS`, `CDR`, or `AC`, durable Rust identifiers must either spell out the meaning or use an approved domain-specific type that resolves the token.
