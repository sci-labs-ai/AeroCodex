# BioSim-plus clean-room runtime boundary

Stage 5 BioSim v3 B2c adds a bounded in-memory replay-integrity, ledger, and friend-test report layer on top of the deployed B2b-2 replay records. The layer consumes committed B2b-2 event records directly. It does not introduce a flat-resource adapter, external scenario parser, persistent ledger store, service, network call, environment lookup, dynamic library, generated source, fixture import, or external BioSim runtime bridge.

## Runtime boundary

The B2c runtime boundary is intentionally narrow:

- input is an already successful `BioSimScenarioReplay` returned by B2b-2;
- replay integrity is checked fail-closed before ledger rows are emitted;
- ledger rows are deterministic per compartment/resource cell;
- committed event amounts are used as the source/sink accounting amounts;
- clamped requested amounts are reported separately and never hidden inside source/sink totals;
- friend-test output is deterministic plain text and contains no filesystem paths.

The ledger self-consistency convention is:

```text
final_amount = initial_amount + source_committed_amount - sink_committed_amount
```

Clamp accounting is:

```text
clamp_amount = requested_amount - committed_amount
```

for B2b-2 consume/sink events that were explicitly clamped by the replay tolerance. Produce/source events must have zero clamp amount.

## Non-scope

B2c remains `research_required`. It is not external BioSim parity evidence, biological-dynamics validation, habitat-control behavior, medical advice, safety evidence, certification evidence, operational suitability, regulated-use approval, or a persistence/API service.

B2c does not use GPL BioSim Java source, GPL-bound BioSim-RS scaffold crates, generated outputs, fixtures, archives, class hierarchies, comments, or translated control flow.
