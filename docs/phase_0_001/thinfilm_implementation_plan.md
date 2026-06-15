# AeroCodex thin-film BLSS implementation plan

## Phase 1 - Kernel integration delivered here

- Add dependency-free Rust functions for the supplied equations.
- Preserve every equation-to-function-to-citation mapping.
- Add validation cards and source-registry seeds.
- Add source material, parameter seeds, scenario seed, and static artifact verification.

## Phase 2 - Solver and scenario layer

- Add optional feature-gated ODE/DAE and nonlinear algebraic solver backends outside the core kernel.
- Add a method-of-lines reactor wrapper for C3 biofilm radial shells and tank-in-series compartments.
- Add a reduced-order model calibration interface for thin-film service-vector fitting.
- Add scenario cards for a C3-to-C4a loop, thin-film panel, and integrated crew metabolic simulator.

## Phase 3 - Validation and calibration

- Add reference numerical examples from the primary papers/theses where available.
- Calibrate C4a optical/physiology parameters, C3 kinetic/diffusion parameters, and thin-film light/diffusion/harvest parameters to experimental data.
- Promote selected validation cards from `equation_traceable` to `reference_validated` only after numerical reproduction with tolerances.

## Phase 4 - AeroCodex orchestration

- Connect the kernel to any future AeroCodex scenario runner or subsystem trait.
- Expose named state schemas for O2, CO2, TIC, NH4, NO2, NO3, biomass, pH, water, temperature, power, and risk.
- Keep fast local safety control below any MPC, ROM, or learning layer.
