# Thin-film BLSS equation traceability

The canonical machine-readable equation map is `data/thinfilm/equation_manifest.csv`. The runtime map is `aero_codex_life_support::thinfilm_provenance::EQUATION_REFERENCES`.

## Coverage

- Eq. 1-5: Common microbial BLSS compartment, closure, loop, and bounds backbone.
- Eq. 6-16: MELiSSA C4a Limnospira photobioreactor light transfer, reduced physiology, carbonate, tank-series, and feasibility maps.
- Eq. 17-26: MELiSSA C3 nitrifying fixed-bed biofilm stoichiometry, diffusion, growth/detachment, transfer, kinetics, and bulk coupling.
- Eq. 27-31: Integrated MELiSSA gas-loop oxygen production and reduced control structure.
- Eq. 32-35: System-level stoichiometry, closure, and equivalent-system-mass trade metrics.
- Eq. 36-46: Strict attached algal thin-film growth, light attenuation, mixture/PDE, and boundary conditions.
- Eq. 47-51: Reduced-order service model, habitat coupling, service vector, and validated-domain constraint.

## Regeneration check

Run:

```bash
python3 scripts/verify_thinfilm_artifact.py
```

The script checks source files, manifests, citation files, validation cards, source-registry seeds, and equation-map coverage.
