# E-trajectory-fit-014 — Average velocity over a sample window

**Scope:** `acx.math.trajectory.fit.window.E014`
**Bridge:** `acx:bridge:trajectory.fit.window:E014:v1`
**Status:** Example

## Formula

Rendered compactly:

```text
v̂ = (x_{n_s-1} - x_0) / ((n_s - 1) Δt)
```

Recommended semantic LaTeX source:

```latex
\EstimatedVelocity =
  \frac{\PositionSample_{\SampleCount - 1} - \PositionSample_0}
       {(\SampleCount - 1)\SampleInterval}
```

## Symbol table

| Display | Role | Domain | Unit | Frame | Rust identifier |
|---|---|---|---|---|---|
| `v̂` | estimated velocity | vector3 | meters per second | local tangent NED | `estimated_velocity_mps` |
| `x_i` | position sample at index `i` | vector3 | meters | local tangent NED | `position_samples[sample_index]` |
| `n_s` | sample count | natural number | count | none | `sample_count` |
| `i` | sample index | integer | count | none | `sample_index` |
| `Δt` | sample interval | positive real | seconds | none | `sample_interval_seconds` |

## Constraints

```text
n_s >= 2
0 <= i < n_s
Δt > 0
```

## Rust bridge

```yaml
bridge_id: acx:bridge:trajectory.fit.window:E014:v1
bindings:
  - math_symbol: n_s
    rust_identifier: sample_count
    rust_type: usize
  - math_symbol: Δt
    rust_identifier: sample_interval_seconds
    rust_type: f64
  - math_symbol: x_i
    rust_identifier: position_samples[sample_index]
    rust_type: PositionNedMeters
  - math_symbol: v_hat
    rust_identifier: estimated_velocity_mps
    rust_type: VelocityNedMetersPerSecond
```

## Implementation note

Rust code must not use `n`, `x`, `dt`, or `v` as durable identifiers for this formula. Use the bridge identifiers.
