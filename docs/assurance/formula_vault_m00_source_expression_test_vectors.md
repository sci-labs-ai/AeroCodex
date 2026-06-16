# Formula-vault M00 source-expression and test-vector contracts

Stage 4 Chunk 7F adds a metadata-only source-expression and test-vector contract for the existing M00 angle/unit candidate slice. The machine-readable contract is:

```text
formula-vault/contracts/m00_angle_unit_conversions_contract.yaml
```

This chunk does not implement formulas, does not create a public application programming interface, does not import raw M07 source files into the repository, does not import Scilab outputs or fixtures, and does not promote any validation status beyond `research_required`.

## Reviewed scope

The reviewed candidate list is still exactly the three formula-vault identifiers selected earlier:

- `formula_vault.m00.angle.deg2rad` / `app_deg2rad` / `foa_app_ast_app_deg2rad`;
- `formula_vault.m00.angle.rad2deg` / `app_rad2deg` / `foa_app_ast_app_rad2deg`;
- `formula_vault.m00.angle.wrap2pi` / `app_wrap2pi` / `foa_app_ast_app_wrap2pi`.

The M07 artifact remains the registered external source material `stage4.m07_rust_port_v14.2026_06_15` with release-candidate / not-certified status. The existing M07 facts remain unchanged: 1,350 represented function rows and 188 Scilab equivalence jobs.

## What was not copied

The contract records independently written mathematical summaries, source locators, finite-input domains, endpoint rules, tolerance policies, and expected-value metadata. It does not commit raw M07 source text, copied comments, copied control flow, generated Rust, external fixtures, Scilab output, archive contents, or local absolute paths.

## Lower-risk conversions

`deg2rad` and `rad2deg` are lower-risk because they are linear unit conversions for finite scalar inputs. Their test vectors are exact multiples of `pi` with straightforward independent analytic oracles.

Future implementation for those two functions should still be bounded to one small chunk and should add Rust tests before public use. It must not treat this contract as certification evidence.

## Endpoint-sensitive wrap behavior

`wrap2pi` is endpoint-sensitive. Chunk 7F records the reviewed interval convention as `[0, 2*pi)` and captures endpoint behavior for zero, positive and negative period multiples, `pi`, `-pi`, `3*pi`, `-3*pi`, and small positive/negative angles.

A future implementation chunk must explicitly test endpoint and floating-point behavior, including signed zero and large-magnitude finite inputs if they are admitted. `wrap2pi` should not be implemented in the same chunk as the first low-risk conversion implementation unless the future prompt explicitly authorizes it.

## Future implementation boundary

A later implementation chunk may use this contract to add tests and a small Rust implementation for `deg2rad` and `rad2deg` only. A separate later chunk should handle `wrap2pi` unless the endpoint tests and equivalence review are explicitly approved in the implementation prompt.

AeroCodex remains research and preliminary-design software. This contract is not certification evidence, flight-readiness evidence, mission-readiness evidence, habitat-safety approval, medical-use approval, operational approval, or regulated-use approval.
