# AeroCodex AI Terminology Pack

Generated for: `examples/specs/acronym_resolution_demo.md`
Generated at: `2026-06-15T02:25:49Z`
Domains: `spacecraft`, `systems_engineering`, `aviation`

## Governing instruction

Use this pack as scoped terminology context under `ACX-NOM-001`. Do not assume acronym meanings that are absent or marked ambiguous. When a token has multiple plausible meanings, use explicit local evidence or return the ambiguity.

## Detected acronyms

- ADS-B = Automatic Dependent Surveillance-Broadcast [candidate; aviation, surveillance, air_traffic; score=2]
  Source: AeroCodex seed / agency_source (acx:source:faa_acronyms:v1)
  AI note: ADS-B usually means Automatic Dependent Surveillance-Broadcast in aviation surveillance contexts.
  First-use rule: expand at first durable use unless already defined by the artifact.

- EDL = Entry, Descent, and Landing [candidate; spacecraft, planetary_missions, mission_phase; score=2]
  Source: AeroCodex seed / project_seed (acx:source:internal_seed:v1)
  AI note: EDL usually means Entry, Descent, and Landing for planetary mission phases.
  First-use rule: expand at first durable use unless already defined by the artifact.

- GNC = Guidance, Navigation, and Control [candidate; spacecraft, flight_systems, autonomy; score=2]
  Source: AeroCodex seed / project_seed (acx:source:internal_seed:v1)
  AI note: GNC usually means Guidance, Navigation, and Control in aerospace flight-system contexts.
  First-use rule: expand at first durable use unless already defined by the artifact.

- PDR = Preliminary Design Review [candidate; systems_engineering, program_lifecycle, reviews; score=2]
  Source: AeroCodex seed / project_seed (acx:source:internal_seed:v1)
  AI note: PDR usually means Preliminary Design Review in aerospace systems-engineering review contexts.
  First-use rule: expand at first durable use unless already defined by the artifact.

## Ambiguous acronyms

- `AC` has 2 registered meanings:
  - Advisory Circular [candidate; aviation, regulation, certification; score=4; source=acx:source:faa_acronyms:v1]
    Signals: FAA, circular, advisory, certification, compliance
  - Alternating Current [candidate; electrical_power, avionics, test; score=-1; source=acx:source:internal_seed:v1]
    Signals: voltage, inverter, frequency, hertz, phase, power bus
  Resolver rule: choose only when local evidence is explicit; otherwise mark ambiguous.

- `CDR` has 2 registered meanings:
  - Critical Design Review [candidate; systems_engineering, program_lifecycle, reviews; score=2; source=acx:source:internal_seed:v1]
    Signals: design review, review board, design maturity, baseline, lifecycle milestone
  - Command/Data Recorder [candidate; avionics, spacecraft_data_systems, onboard_storage; score=0; source=acx:source:internal_seed:v1]
    Signals: recorder, onboard storage, telemetry, command history, data handling
  Resolver rule: choose only when local evidence is explicit; otherwise mark ambiguous.

- `RCS` has 2 registered meanings:
  - Radar Cross Section [candidate; radar, electromagnetics, signatures; score=2; source=acx:source:internal_seed:v1]
    Signals: radar, scattering, signature, square meters, monostatic, bistatic
  - Reaction Control System [candidate; spacecraft, propulsion, attitude_control; score=1; source=acx:source:internal_seed:v1]
    Signals: thruster, attitude control, propellant, impulse bit, spacecraft control
  Resolver rule: choose only when local evidence is explicit; otherwise mark ambiguous.

## Resolver reminders

- Prefer project/contract glossary meanings over general aerospace usage.
- Candidate records are useful hints, not final authority.
- Preserve source-original wording when canonical mapping is not approved.
- Do not invent acronym expansions for durable AeroCodex outputs.
