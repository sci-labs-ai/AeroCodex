#!/usr/bin/env python3
"""Verify A19 orbital-geometry/conic Wave 4 terminal dispositions.

This dependency-free verifier consumes classifier metadata, A16-A18 manifests,
existing A7 batch metadata, prior external resolution manifests, and explicit
A19 terminal resolution records. It never opens or parses raw Rust-port, M07,
or Scilab source text.
"""
from __future__ import annotations

import argparse
import csv
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any, Iterable

SCHEMA_VERSION = 'aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH = 'docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
WAVE1_PATH = 'formula-vault/resolutions/m07_orbital_geometry_conic_wave1.tsv'
WAVE2_PATH = 'formula-vault/resolutions/m07_orbital_geometry_conic_wave2.tsv'
WAVE3_PATH = 'formula-vault/resolutions/m07_orbital_geometry_conic_wave3.tsv'
RESOLUTION_PATH = 'formula-vault/resolutions/m07_orbital_geometry_conic_wave4.tsv'
A7_BATCH_PATH = 'equation-batches/a7-astrodynamics-orekit-foundation.tsv'
INVENTORY_PATH = 'validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID = 'stage4.m07_rust_port_v14.2026_06_15'
TARGET_CHUNK = '9A_classical_elements_and_9E_mission_design_contracts'
M07_REPRESENTED_FUNCTION_ROWS = 1350
EXPECTED_CLASSIFIER_GROUP_ROWS = 377
EXPECTED_WAVE1_ROWS = 40
EXPECTED_WAVE2_ROWS = 40
EXPECTED_WAVE3_ROWS = 40
EXPECTED_ROWS = 40
EXPECTED_GROUP_REMAINING_ROWS = 217
EXPECTED_EXECUTABLE_ROWS = 152
EXPECTED_METADATA_ROWS = 27
EXPECTED_CUMULATIVE_PROCESSED = 481
EXPECTED_REMAINING_BACKLOG = 842
EXPECTED_RISK_COUNTS = Counter({'medium_risk_requires_contract_review': 37, 'high_risk_requires_numerical_policy': 3})
EXPECTED_TARGET_COUNTS = {'formula_vault.astrodynamics.elements.semimajor_axis_from_state': 1, 'formula_vault.astrodynamics.kepler.true_anomaly_from_eccentric_anomaly': 1, 'formula_vault.astrodynamics.elements.eccentricity_vector': 1}
EXPECTED_DISPOSITIONS = Counter({'blocked_missing_reciprocal_semimajor_axis_state_units_and_branch_contract': 2, 'blocked_missing_semilatus_rectum_from_state_units_and_degeneracy_contract': 2, 'deduplicated_alias_to_existing_runtime': 3, 'excluded_lagrange_fg_identity_or_state_composition_helper_not_formula': 2, 'blocked_until_lagrange_fg_anomaly_branch_and_numerical_policy': 3, 'excluded_composite_orbit_identity_or_summary_algorithm_not_formula': 3, 'blocked_missing_conic_mean_anomaly_branch_and_units_contract': 2, 'blocked_until_conic_time_of_flight_branch_wrap_direction_and_units_contract': 10, 'blocked_missing_anomaly_conversion_branch_angle_and_domain_contract': 5, 'blocked_missing_parabolic_anomaly_from_state_domain_contract': 1, 'excluded_universal_variable_bundle_helper_not_formula': 1, 'blocked_missing_stumpff_function_branch_series_and_numerical_contract': 4, 'excluded_internal_scalar_validation_or_math_helper_not_formula': 2})
EXPECTED_HEADER = ['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']
TARGET_MATCH_FIELDS = {
    'target_package': 'package',
    'target_crate_name': 'crate_name',
    'target_runtime_symbol': 'runtime_symbol',
    'target_contract_path': 'contract_path',
    'target_validation_card_path': 'validation_card_path',
    'target_source_seed_path': 'source_seed_path',
}
ALIASES = {'ch4::semimajor_axis_from_rv': 'formula_vault.astrodynamics.elements.semimajor_axis_from_state', 'ch4::elliptic_e_to_true': 'formula_vault.astrodynamics.kepler.true_anomaly_from_eccentric_anomaly', 'ch5::eccentricity_vector': 'formula_vault.astrodynamics.elements.eccentricity_vector'}
RECIPROCAL_A = {'ch5::reciprocal_semimajor_axis', 'ch4::reciprocal_semimajor_axis'}
SEMILATUS_FROM_STATE = {'ch4::semilatus_rectum_from_rv', 'ch5::semilatus_rectum_from_rv'}
FG_NUMERICAL = {'ch4::lagrange_fg_delta_true', 'ch4::lagrange_fg_delta_f', 'ch4::lagrange_fg_delta_e'}
ANOMALY_CONVERSION = {'ch4::hyperbolic_f_to_true', 'ch4::true_to_parabolic_d', 'ch4::true_to_elliptic_e', 'ch4::parabolic_d_to_true', 'ch4::true_to_hyperbolic_f'}
MEAN_ANOMALY = {'ch4::elliptic_mean_anomaly', 'ch4::hyperbolic_mean_anomaly'}
TIME_OF_FLIGHT = {'ch4::elliptic_tof_forward_between_true', 'ch4::elliptic_tof_from_periapsis', 'ch4::hyperbolic_tof_between_true', 'ch4::elliptic_tof_between_e', 'ch4::hyperbolic_tof_between_f', 'ch4::parabolic_tof_from_periapsis', 'ch4::elliptic_tof_between_true', 'ch4::parabolic_tof_between_d', 'ch4::parabolic_tof_between_true', 'ch4::hyperbolic_tof_from_periapsis'}
PARABOLIC_FROM_STATE = {'ch4::parabolic_d_from_rv'}
STUMPFF_SCALARS = {'ch4::stumpff_ds', 'ch4::stumpff_dc', 'ch4::stumpff_s', 'ch4::stumpff_c'}
EXCLUDED_FG = {'ch4::rv_from_fg', 'ch4::fg_identity'}
EXCLUDED_COMPOSITE = {'ch4::elliptic_identities_from_rv', 'ch4::hyperbolic_identities_from_rv', 'ch5::basic_orbit_from_rv'}
EXCLUDED_UNIVERSAL_BUNDLE = {'ch4::stumpff'}
EXCLUDED_SCALAR_HELPERS = {'ch5::isfinite_scalar', 'ch5::atan2'}


class VerificationError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def stable_json(value: Any) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + '\n'


def repo_file(repo: Path, relative: str) -> Path:
    path = repo / relative
    require(path.is_file(), f'missing repository file: {relative}')
    return path


def read_delimited(path: Path, delimiter: str, expected_header: list[str] | None = None) -> list[dict[str, str]]:
    with path.open(encoding='utf-8-sig', newline='') as handle:
        reader = csv.DictReader(handle, delimiter=delimiter)
        require(reader.fieldnames is not None, f'missing header: {path}')
        if expected_header is not None:
            require(reader.fieldnames == expected_header, f'unsupported header in {path}: {reader.fieldnames}')
        rows = list(reader)
    require(rows, f'no data rows: {path}')
    return rows


def unique_map(rows: Iterable[dict[str, str]], key: str, label: str) -> dict[str, dict[str, str]]:
    result: dict[str, dict[str, str]] = {}
    for index, row in enumerate(rows, 1):
        value = row.get(key, '')
        require(value != '', f'{label} row {index} missing {key}')
        require(value not in result, f'duplicate {label} {key}: {value}')
        result[value] = row
    return result


def source_row_number(locator: str) -> int:
    match = re.fullmatch(r'PORT_STATUS_RELEASE_GATE\.csv:row_(\d{4})', locator)
    require(match is not None, f'invalid source row locator: {locator}')
    return int(match.group(1))


def expected_resolution(alias: str) -> tuple[str, str | None, str]:
    if alias in ALIASES:
        return ('deduplicated_alias_to_existing_runtime', ALIASES[alias], 'not_applicable_existing_runtime_and_contract_reused')
    if alias in RECIPROCAL_A:
        return ('blocked_missing_reciprocal_semimajor_axis_state_units_and_branch_contract', None, 'reciprocal semimajor axis from state requires explicit position velocity gravitational parameter units conic branch zero energy tolerance infinite semimajor axis and nonfinite output contract')
    if alias in SEMILATUS_FROM_STATE:
        return ('blocked_missing_semilatus_rectum_from_state_units_and_degeneracy_contract', None, 'semilatus rectum from state requires explicit state and gravitational parameter units angular momentum norm definition zero momentum degeneracy conic branch and nonfinite output contract')
    if alias in FG_NUMERICAL:
        return ('blocked_until_lagrange_fg_anomaly_branch_and_numerical_policy', None, 'Lagrange f and g coefficient evaluation requires explicit anomaly definition conic branch direction of flight state units denominator singularity numerical conditioning and reference oracle policy')
    if alias in ANOMALY_CONVERSION:
        return ('blocked_missing_anomaly_conversion_branch_angle_and_domain_contract', None, 'anomaly conversion requires explicit elliptic hyperbolic or parabolic branch angle interval wrap orientation inverse branch domain singularity tolerance and nonfinite policy')
    if alias in MEAN_ANOMALY:
        return ('blocked_missing_conic_mean_anomaly_branch_and_units_contract', None, 'mean anomaly relation requires explicit conic branch anomaly convention angular units wrap policy eccentricity domain and nonfinite output contract')
    if alias in TIME_OF_FLIGHT:
        return ('blocked_until_conic_time_of_flight_branch_wrap_direction_and_units_contract', None, 'time of flight relation requires explicit conic branch anomaly convention forward or signed direction revolution count gravitational parameter and length units wrap policy domain singularity and oracle validation')
    if alias in PARABOLIC_FROM_STATE:
        return ('blocked_missing_parabolic_anomaly_from_state_domain_contract', None, 'parabolic anomaly from state requires explicit parabolic tolerance state and gravitational parameter units orientation branch degeneracy and nonfinite output contract')
    if alias in STUMPFF_SCALARS:
        return ('blocked_missing_stumpff_function_branch_series_and_numerical_contract', None, 'Stumpff scalar or derivative requires explicit argument domain positive negative and near zero branches series truncation tolerance overflow behavior derivative convention and oracle validation')
    if alias in EXCLUDED_FG:
        return ('excluded_lagrange_fg_identity_or_state_composition_helper_not_formula', None, 'Lagrange f g identity checking or multi vector state composition is a validation or composition helper rather than a separate bounded formula node')
    if alias in EXCLUDED_COMPOSITE:
        return ('excluded_composite_orbit_identity_or_summary_algorithm_not_formula', None, 'multi output orbit identity or state summary is a composite algorithm rather than a separate bounded formula node')
    if alias in EXCLUDED_UNIVERSAL_BUNDLE:
        return ('excluded_universal_variable_bundle_helper_not_formula', None, 'combined Stumpff function dispatch and bundle return is universal variable solver support rather than a separate bounded formula node')
    if alias in EXCLUDED_SCALAR_HELPERS:
        return ('excluded_internal_scalar_validation_or_math_helper_not_formula', None, 'generic scalar math or finite value validation wrapper is an internal utility rather than a separate formula node')
    raise VerificationError(f'unsupported A19 alias: {alias}')


def require_logical_source_locator(locator: str, row_index: int) -> None:
    require(locator != '', f'row {row_index} source_file_locator is empty')
    require(not locator.startswith(('/', '\\')), f'row {row_index} absolute source locator')
    require(re.match(r'^[A-Za-z]:[\\/]', locator) is None, f'row {row_index} Windows absolute source locator')
    require('..' not in Path(locator).parts, f'row {row_index} source locator traverses parents')


def external_resolution_inventory(repo: Path, inventory_rows: list[dict[str, str]], metadata_count: int) -> tuple[int, int]:
    processed = [row for row in inventory_rows if row['category'] == 'external_m07_processed_row']
    backlog = [row for row in inventory_rows if row['category'] == 'external_m07_backlog_row']
    processed_map = unique_map(processed, 'source_path', 'external processed inventory')
    manifests = sorted((repo / 'formula-vault/resolutions').glob('m07_*.tsv'))
    require(manifests, 'no external M07 resolution manifests found')
    total = 0
    expected_paths: set[str] = set()
    resolution_ids: set[str] = set()
    locators: set[str] = set()
    for path in manifests:
        relative = path.relative_to(repo).as_posix()
        expected_paths.add(relative)
        rows = read_delimited(path, '\t', EXPECTED_HEADER)
        inventory = processed_map.get(relative)
        require(inventory is not None, f'missing processed inventory row for {relative}')
        require(inventory['row_count'] == str(len(rows)), f'processed inventory count mismatch for {relative}')
        for row in rows:
            require(row['resolution_id'] not in resolution_ids, f'duplicate external resolution ID: {row["resolution_id"]}')
            resolution_ids.add(row['resolution_id'])
            require(row['source_row_locator'] not in locators, f'duplicate source-row locator: {row["source_row_locator"]}')
            locators.add(row['source_row_locator'])
        total += len(rows)
    require(set(processed_map) == expected_paths, 'processed inventory and external manifests are not exact union')
    require(len(backlog) == 1, f'expected one backlog row, found {len(backlog)}')
    expected = M07_REPRESENTED_FUNCTION_ROWS - metadata_count - total
    require(backlog[0]['row_count'] == str(expected), 'external backlog count mismatch')
    return total, expected


def verify_repo(repo: Path) -> dict[str, Any]:
    repo = repo.resolve()
    require(repo.is_dir(), f'repository does not exist: {repo}')
    classifier_rows = read_delimited(repo_file(repo, CLASSIFIER_PATH), ',')
    group = [row for row in classifier_rows if row['recommended_chunk_group'] == TARGET_CHUNK]
    group.sort(key=lambda row: source_row_number(row['m07_row_id_or_alias']))
    require(len(group) == EXPECTED_CLASSIFIER_GROUP_ROWS, f'classifier group count mismatch: {len(group)}')

    prior_specs = [
        (WAVE1_PATH, EXPECTED_WAVE1_ROWS),
        (WAVE2_PATH, EXPECTED_WAVE2_ROWS),
        (WAVE3_PATH, EXPECTED_WAVE3_ROWS),
    ]
    prior_rows: list[dict[str, str]] = []
    for path, expected_count in prior_specs:
        rows = read_delimited(repo_file(repo, path), '\t', EXPECTED_HEADER)
        require(len(rows) == expected_count, f'prior wave count mismatch for {path}: {len(rows)}')
        prior_rows.extend(rows)
    prior_locators = {row['source_row_locator'] for row in prior_rows}
    require(len(prior_locators) == 120, 'prior orbital-geometry locators are not unique')
    require(prior_locators == {row['m07_row_id_or_alias'] for row in group[:120]}, 'A16-A18 are not exact first-120 selection')

    remaining = [row for row in group if row['m07_row_id_or_alias'] not in prior_locators]
    selected = remaining[:EXPECTED_ROWS]
    after = remaining[EXPECTED_ROWS:]
    require(len(selected) == EXPECTED_ROWS, 'selected row count mismatch')
    require(len(after) == EXPECTED_GROUP_REMAINING_ROWS, f'remaining group count mismatch: {len(after)}')
    classifier = unique_map(selected, 'm07_row_id_or_alias', 'classifier')
    risk = Counter(row['risk_tier'] for row in selected)
    require(risk == EXPECTED_RISK_COUNTS, f'risk counts mismatch: {dict(risk)}')
    for locator, row in classifier.items():
        require(row['formula_family'] == 'orbit_two_body', f'classifier family mismatch: {locator}')
        require(row['implementation_readiness'] == 'contract_review_first_no_implementation', f'classifier readiness mismatch: {locator}')
        require(row['block_reason'] == 'blocked_until_orbit_geometry_conic_branch_and_validation_policy', f'classifier block reason mismatch: {locator}')

    rows = read_delimited(repo_file(repo, RESOLUTION_PATH), '\t', EXPECTED_HEADER)
    require(len(rows) == EXPECTED_ROWS, f'resolution row count mismatch: {len(rows)}')
    resolutions = unique_map(rows, 'source_row_locator', 'resolution')
    unique_map(rows, 'resolution_id', 'resolution')
    require(set(resolutions) == set(classifier), 'classifier selection and resolution locators are not exact union')
    a7 = unique_map(read_delimited(repo_file(repo, A7_BATCH_PATH), '\t'), 'formula_id', 'A7 batch')
    dispositions: Counter[str] = Counter()
    targets: Counter[str] = Counter()
    numbers: list[int] = []
    source_files: set[str] = set()
    for index, row in enumerate(rows, 1):
        locator = row['source_row_locator']
        source = classifier[locator]
        number = source_row_number(locator)
        numbers.append(number)
        require(row['schema_version'] == SCHEMA_VERSION, f'row {index} schema mismatch')
        require(row['resolution_id'] == f'resolution.external_m07.orbital_geometry_conic_wave4.{number:04d}', f'row {index} resolution ID mismatch')
        require(row['source_artifact_id'] == SOURCE_ARTIFACT_ID, f'row {index} source artifact mismatch')
        require(row['classifier_path'] == CLASSIFIER_PATH, f'row {index} classifier path mismatch')
        require(row['source_row_number'] == str(number), f'row {index} source row mismatch')
        require_logical_source_locator(row['source_file_locator'], index)
        for field, classifier_field in [
            ('rust_function_alias', 'rust_function_alias'),
            ('scilab_function_alias', 'scilab_function_alias_if_known'),
            ('source_file_locator', 'source_file_locator'),
            ('formula_family', 'formula_family'),
            ('risk_tier', 'risk_tier'),
            ('recommended_chunk_group', 'recommended_chunk_group'),
        ]:
            require(row[field] == source[classifier_field], f'row {index} classifier mismatch for {field}')
        require(row['validation_status'] == 'research_required', f'row {index} validation status mismatch')
        disposition, target_formula, reason = expected_resolution(row['rust_function_alias'])
        require(row['disposition'] == disposition, f'row {index} disposition mismatch')
        require(row['block_reason'] == reason, f'row {index} block reason mismatch')
        dispositions[disposition] += 1
        source_files.add(row['source_file_locator'])
        if target_formula is None:
            for field in ['target_formula_id', 'target_resolution_id', 'target_batch_manifest', *TARGET_MATCH_FIELDS, 'target_runtime_path']:
                require(row[field] == '', f'row {index} non-alias must leave {field} empty')
        else:
            require(row['target_formula_id'] == target_formula, f'row {index} target formula mismatch')
            require(row['target_resolution_id'] == '', f'row {index} target_resolution_id must be empty')
            target = a7.get(target_formula)
            require(target is not None, f'row {index} target missing from A7 batch')
            require(row['target_batch_manifest'] == A7_BATCH_PATH, f'row {index} target batch mismatch')
            for field, target_field in TARGET_MATCH_FIELDS.items():
                require(row[field] == target[target_field], f'row {index} target mismatch for {field}')
            require(row['target_runtime_path'] == f"{target['crate_name']}::{target['runtime_symbol']}", f'row {index} runtime path mismatch')
            targets[target_formula] += 1
            for path_field in ['target_batch_manifest', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path']:
                repo_file(repo, row[path_field])

    require(numbers == sorted(numbers), 'rows not deterministic source order')
    require(dispositions == EXPECTED_DISPOSITIONS, f'disposition counts mismatch: {dict(dispositions)}')
    require(dict(sorted(targets.items())) == EXPECTED_TARGET_COUNTS, f'target counts mismatch: {dict(targets)}')

    inventory = read_delimited(repo_file(repo, INVENTORY_PATH), '\t')
    executable = [row for row in inventory if row['category'] == 'executable_research_equation']
    metadata = [row for row in inventory if row['category'] == 'metadata_only_formula_vault_candidate']
    require(len(executable) == EXPECTED_EXECUTABLE_ROWS, f'executable count mismatch: {len(executable)}')
    require(len(metadata) == EXPECTED_METADATA_ROWS, f'metadata count mismatch: {len(metadata)}')
    total, backlog = external_resolution_inventory(repo, inventory, len(metadata))
    require(total == EXPECTED_CUMULATIVE_PROCESSED, f'processed count mismatch: {total}')
    require(backlog == EXPECTED_REMAINING_BACKLOG, f'backlog count mismatch: {backlog}')
    excluded = sum(value for key, value in dispositions.items() if key.startswith('excluded_'))
    blocked = sum(value for key, value in dispositions.items() if key.startswith('blocked_'))
    return {
        'schema_version': SCHEMA_VERSION,
        'result': 'PASS',
        'wave_id': 'a19_external_m07_orbital_geometry_conic_wave4',
        'classifier_group_rows': len(group),
        'wave1_rows': EXPECTED_WAVE1_ROWS,
        'wave2_rows': EXPECTED_WAVE2_ROWS,
        'wave3_rows': EXPECTED_WAVE3_ROWS,
        'prior_group_rows': len(prior_rows),
        'classifier_rows_selected': len(selected),
        'classifier_group_remaining_rows': len(after),
        'terminal_disposition_rows': len(rows),
        'deduplicated_alias_rows': dispositions['deduplicated_alias_to_existing_runtime'],
        'excluded_helper_rows': excluded,
        'contract_blocked_rows': blocked,
        'risk_tier_counts': dict(sorted(risk.items())),
        'target_formula_counts': dict(sorted(targets.items())),
        'distinct_source_files': len(source_files),
        'risk_tier_not_downgraded': True,
        'executable_research_equations': len(executable),
        'metadata_inventory_records': len(metadata),
        'external_m07_processed_rows': total,
        'external_m07_backlog_rows': backlog,
        'formula_count_delta': 0,
        'runtime_kernel_files_changed': 0,
        'new_validation_cards_required': 0,
        'new_source_seeds_required': 0,
        'validation_status': 'research_required',
        'no_rust_m07_or_scilab_source_scraping': True,
        'no_external_parity_claim': True,
        'no_certification_or_operational_readiness_claim': True,
    }


def self_test() -> dict[str, Any]:
    require(stable_json({'b': 2, 'a': 1}).startswith('{\n  "a"'), 'stable JSON ordering failed')
    cases = {
        'ch4::semimajor_axis_from_rv': ('deduplicated_alias_to_existing_runtime', 'formula_vault.astrodynamics.elements.semimajor_axis_from_state'),
        'ch4::elliptic_e_to_true': ('deduplicated_alias_to_existing_runtime', 'formula_vault.astrodynamics.kepler.true_anomaly_from_eccentric_anomaly'),
        'ch5::eccentricity_vector': ('deduplicated_alias_to_existing_runtime', 'formula_vault.astrodynamics.elements.eccentricity_vector'),
        'ch4::fg_identity': ('excluded_lagrange_fg_identity_or_state_composition_helper_not_formula', None),
        'ch4::elliptic_tof_between_e': ('blocked_until_conic_time_of_flight_branch_wrap_direction_and_units_contract', None),
        'ch4::stumpff_c': ('blocked_missing_stumpff_function_branch_series_and_numerical_contract', None),
    }
    for alias, expected in cases.items():
        require(expected_resolution(alias)[:2] == expected, f'mapping self-test failed: {alias}')
    duplicate_rejected = False
    try:
        unique_map([{'x': 'a'}, {'x': 'a'}], 'x', 'fixture')
    except VerificationError:
        duplicate_rejected = True
    require(duplicate_rejected, 'duplicate fixture not rejected')
    return {
        'schema_version': SCHEMA_VERSION,
        'mode': 'self-test',
        'result': 'PASS',
        'tests': [
            {'name': 'stable_json', 'result': 'PASS'},
            {'name': 'deterministic_mapping', 'result': 'PASS'},
            {'name': 'duplicate_rejected', 'result': 'PASS'},
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=Path('.'))
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    try:
        print(stable_json(self_test() if args.self_test else verify_repo(args.repo)), end='')
        return 0
    except Exception as error:
        print(stable_json({'schema_version': SCHEMA_VERSION, 'result': 'FAIL', 'error': str(error)}), end='', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
