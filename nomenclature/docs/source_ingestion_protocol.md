# Source Ingestion Protocol

## Purpose

Source documents can be inconsistent, ambiguous, abbreviated, or domain-specific. AeroCodex must preserve source wording for auditability but never allow source wording to become canonical by accident.

## Required ingestion record

Every source-to-canonical mapping must preserve:

```yaml
source_document_id: customer_a_aircraft_export_2026_06
source_document_type: customer_a_aircraft_export
source_field: Tail No.
source_value: N123AB
canonical_field: aircraft_registration
normalized_value: N123AB
mapping_rule_id: acx:ingest:registration:customer_a:v1
mapping_status: approved
```

## Never infer by spelling alone

Bad:

```text
Any field containing "tail" maps to aircraft_registration.
```

Good:

```yaml
rule_id: acx:ingest:registration:customer_a:v1
source_document_type: customer_a_aircraft_export
source_field_exact: Tail No.
canonical_field: aircraft_registration
approved_by: nomenclature_owner
```

## Context-specific source terms

The same source term may mean different things in different contexts.

```yaml
source_term: station
source_context: maintenance_manual
canonical: fuselage_station
```

```yaml
source_term: station
source_context: airport_weather_feed
canonical: reporting_station
```

Without context, `station` is ambiguous and must not auto-map.

## Ingestion mapping statuses

```text
draft        proposed but not approved
approved     reviewed and usable
ambiguous    multiple possible canonical meanings
rejected     do not use
superseded   replaced by a newer mapping
```

## Required rejection behavior

When an ambiguous source term appears:

1. Preserve the source field and value.
2. Do not write a canonical value.
3. Emit a mapping error with candidate canonical terms.
4. Require explicit source-context disambiguation.

## Example error

```yaml
error: ACX-NOM-E003
message: Alias maps to multiple canonical terms without context.
source_field: Station
candidates:
  - airport_station
  - maintenance_station
  - fuselage_station
  - reporting_station
resolution_required: true
```

## Acronym ingestion extension

When a source document contains acronyms, preserve the source token and map it through `registry/acronyms.yaml` only when the source context is explicit enough.

```yaml
source_document_id: faa_related_test_plan_2026_06
source_token: ADS-B
source_context: flight_test_chase_aircraft
candidate_acronym_id: acx:acr:aviation:ads_b:v1
mapping_status: candidate
resolution_required: false
```

For colliding tokens, do not normalize by token alone:

```yaml
source_token: RCS
source_context: mixed_spacecraft_and_radar_review
mapping_status: ambiguous
candidate_acronym_ids:
  - acx:acr:space:rcs:reaction_control_system:v1
  - acx:acr:radar:rcs:radar_cross_section:v1
resolution_required: true
```

Imported external acronym records must reference `registry/terminology_sources.yaml` and should enter as `candidate` or `external` until reviewed.
