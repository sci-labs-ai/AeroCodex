# Release-slice floating-point tolerance policy

The M00 conversion release slice uses three explicit comparison modes together:

- absolute tolerance handles exact-zero and values close to zero;
- relative tolerance scales with the larger magnitude of actual and expected finite values;
- ULP distance detects small representation-level drift when the values share a sign.

A result passes when it is exactly equal or satisfies at least one declared bound. Analytical-vector tolerances are recorded per row in `m00_reference_vectors.tsv`. Exact scale-one identities use exact equality. Round trips and broad-domain properties use `1e-12` absolute and relative bounds plus eight ULPs. Angle identities use `1e-15` absolute/relative bounds plus four ULPs for radians and `1e-12` plus four ULPs for degrees.

NaN is never equal to any expected result. Infinite inputs are rejected before evaluation. Finite computations that overflow to a non-finite result fail closed. Signed zero is accepted as numerically equal for scalar conversion round trips; no positive-zero canonicalization claim is made for these twelve formulas.

The policy is designed to expose cross-platform differences without claiming mathematical exactness beyond IEEE-754 binary64 behavior. It supports implementation evidence for research software only and is not an uncertainty model or a physical-validation claim.
