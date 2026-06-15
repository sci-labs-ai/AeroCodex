# Math Symbol Protocol

## Purpose

This protocol prevents mathematical notation from silently changing meaning across contexts.

The dangerous case is not just `n`; it is any compact symbol whose meaning can drift:

```text
n, N, m, i, j, k, x, y, z, t, T, r, R, q, p, P, e, E, f, F
```

## Rule: local declaration required

No bare algebraic symbol may appear in a durable specification unless it is declared in the nearest symbol table or inherited from an explicitly named parent scope.

## Equation record template

```yaml
equation_id: E-example-001
scope: acx.math.example.E001
status: draft
formula_source: |
  \ExampleOutput = \ExampleInput / \SampleCount
formula_rendered: |
  y = x / n_s
symbols:
  - symbol_id: acx:math:example:E001:sample_count:v1
    surface_form: n
    display_form: n_s
    latex_macro: SampleCount
    semantic_role: sample_count
    domain: natural_number
    constraints:
      - n_s >= 1
    unit: count
    rust_identifier: sample_count
```

## Preferred notation hierarchy

Use compact symbols only when a symbol table makes them safe.

| Durability level | Recommended notation |
|---|---|
| Scratch derivation | `n`, `x`, `t` allowed if obvious locally |
| Internal spec | `n_s`, `x_i`, `Δt` with symbol table |
| Long-lived spec source | semantic LaTeX macros such as `\SampleCount` |
| Rust implementation | `sample_count`, `position_samples`, `sample_interval_seconds` |

## Subscripts and superscripts

Subscripts and superscripts must be classified.

Allowed classifications:

```text
index
frame
component
exponent
semantic_label
version
source_label
```

Examples:

```yaml
- display_form: v_N
  semantic_role: north_velocity_component
  subscript:
    class: component
    value: north

- display_form: x_i
  semantic_role: position_sample_at_index
  subscript:
    class: index
    binding: sample_index

- display_form: R_B
  semantic_role: rotation_from_body_frame
  subscript:
    class: frame
    frame: body
```

## Index variables

Every index variable must define:

```text
set/domain
lower bound
upper bound
role
collection being indexed
```

Example:

```yaml
symbol_id: acx:math:trajectory.fit.window:E014:sample_index:v1
display_form: i
semantic_role: sample_index
domain: integer
constraints:
  - 0 <= i
  - i < n_s
indexes: position_samples
rust_identifier: sample_index
```

## Custom operators

Every custom operator requires a registry entry.

```yaml
operator_id: acx:math:geodesy:compose_transform:v1
display_form: ⊕
semantic_role: transform_composition
left_operand: Transform
right_operand: Transform
result: Transform
associative: true
commutative: false
identity_element: identity_transform
rust_identifier: compose_transform
```

## Reusing a glyph

A glyph may be reused only when the scope is disjoint or shadowing is explicit.

Acceptable disjoint reuse:

```yaml
- symbol_id: acx:math:trajectory:E014:sample_count:v1
  display_form: n_s
  surface_form: n
  scope: acx.math.trajectory.E014

- symbol_id: acx:math:graph:E003:node_count:v1
  display_form: n_v
  surface_form: n
  scope: acx.math.graph.E003
```

Bad shared scope:

```text
n = sample count
n = node count
```

Good shared scope:

```text
n_s = sample count
n_v = node count
```
