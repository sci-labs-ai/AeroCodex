# Historical RR-055 reserved-output fixtures

These files preserve the deterministic path-reservation snapshots created by historical task RR-055. They are historical fixtures, not current golden contracts, generated release artifacts, formula-validation evidence, or executable CLI expectations.

No current test may consume this directory as an approval oracle. Current generated state is governed under `generated/`, and current CLI behavior is verified by executable integration tests. A future task that introduces stable golden contract tests must create a separately reviewed live contract rather than reactivating these placeholders.

The retained snapshots remain intentionally blocked and non-executable. Their `placeholder` fields and RR-055 notices describe their historical contents; the enclosing path establishes their present classification.
