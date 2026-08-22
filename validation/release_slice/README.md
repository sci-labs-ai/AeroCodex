# M00 release-slice validation

This directory is the independent, machine-linked validation surface for the twelve formulas declared by `release/release-manifest.toml`. It is deliberately narrower than the full 152-row research registry.

`m00_reference_vectors.tsv` contains analytical reference cases and links every case to its governed formula contract, validation card, source record, and completed promotion packet. The public Rust test `release_slice_validation` consumes those vectors and also exercises round trips, unit-scale identities, signed and zero values, invalid scales, non-finite inputs, overflow rejection, deterministic broad-domain loops, and cross-platform edge values.

`tolerance_policy.md` defines the absolute, relative, and ULP-style comparison rules. `cargo run -p xtask -- verify-release-slice-validation` verifies exact twelve-formula coverage and checks the deterministic slice-only summary at `generated/release_slice_validation_summary.json`.

The twelve compact packet files inherit their machine links from the matching vector row: canonical ID, legacy/runtime identity, equation-batch contract, validation card, source seed, runtime implementation, CLI dispatch, analytical vector, negative/property test module, and tolerance policy are reviewed as one governed chain. The packets record the current and requested statuses, requested research execution policy, two explicit review roles, recommendation, and non-claims. No packet changes a registry status by itself; status changes remain a separate reviewed mutation.

This evidence supports only the bounded `implementation_verified` claim used by the Research Software Alpha. It is not physical-model validation, flight or mission readiness, operational approval, regulatory approval, habitat or life-support safety evidence, or certification.
