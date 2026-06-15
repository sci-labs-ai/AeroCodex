# Example AI Terminology Pack

Generated from `examples/specs/acronym_resolution_demo.md` using:

```bash
python tooling/aerocodex_terminology.py --root . pack \
  --text-file examples/specs/acronym_resolution_demo.md \
  --domain spacecraft \
  --domain systems_engineering \
  --domain aviation
```

Expected behavior:

- `PDR` should resolve as a systems-engineering review candidate.
- `CDR` should appear as ambiguous unless design-review context dominates.
- `GNC`, `EDL`, `ADS-B`, and FAA `AC` should be detected.
- `RCS` should appear with both Reaction Control System and Radar Cross Section candidates because both contexts appear in the document.
