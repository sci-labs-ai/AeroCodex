#!/usr/bin/env python3
"""Verify A16 orbital-geometry and conic-branch Wave 1 terminal dispositions.

This dependency-free verifier consumes classifier metadata, existing A7 batch
metadata, prior external resolution manifests, and explicit A16 terminal
resolution records. It never opens or parses raw Rust-port, M07, or Scilab
source text.
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

SCHEMA_VERSION = "aerocodex.external_m07_resolution.v1"
CLASSIFIER_PATH = "docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv"
RESOLUTION_PATH = "formula-vault/resolutions/m07_orbital_geometry_conic_wave1.tsv"
A7_BATCH_PATH = "equation-batches/a7-astrodynamics-orekit-foundation.tsv"
INVENTORY_PATH = "validation/equation_inventory.tsv"
SOURCE_ARTIFACT_ID = "stage4.m07_rust_port_v14.2026_06_15"
TARGET_CHUNK = "9A_classical_elements_and_9E_mission_design_contracts"
M07_REPRESENTED_FUNCTION_ROWS = 1350
EXPECTED_CLASSIFIER_GROUP_ROWS = 377
EXPECTED_ROWS = 40
EXPECTED_GROUP_REMAINING_ROWS = 337
EXPECTED_EXECUTABLE_ROWS = 152
EXPECTED_METADATA_ROWS = 27
EXPECTED_CUMULATIVE_PROCESSED=786
EXPECTED_REMAINING_BACKLOG=537
EXPECTED_TARGET_COUNTS = {
    "formula_vault.astrodynamics.elements.eccentricity_vector": 1,
    "formula_vault.astrodynamics.elements.specific_angular_momentum": 1,
}
EXPECTED_DISPOSITIONS = Counter(
    {
        "deduplicated_alias_to_existing_runtime": 2,
        "excluded_internal_scalar_math_helper_not_formula": 1,
        "excluded_parameter_lookup_helper_not_formula": 2,
        "excluded_dynamics_support_algorithm_not_formula": 7,
        "blocked_missing_state_geometry_and_angle_contract": 4,
        "blocked_ambiguous_energy_input_and_conic_branch_contract": 2,
        "blocked_missing_semilatus_rectum_contract_and_runtime": 1,
        "blocked_missing_eccentricity_from_energy_momentum_contract": 1,
        "blocked_missing_apsis_ellipse_geometry_contract_and_runtime": 10,
        "blocked_missing_conic_classification_boundary_contract": 1,
        "blocked_ambiguous_true_anomaly_state_and_singularity_contract": 1,
        "blocked_missing_conic_state_geometry_contract": 3,
        "blocked_missing_parabolic_boundary_contract": 2,
        "blocked_missing_hyperbolic_branch_and_mission_geometry_contract": 3,
    }
)
EXPECTED_HEADER = """schema_version resolution_id source_artifact_id classifier_path source_row_locator source_row_number rust_function_alias scilab_function_alias source_file_locator formula_family risk_tier recommended_chunk_group target_formula_id target_resolution_id target_batch_manifest target_package target_crate_name target_runtime_symbol target_runtime_path target_contract_path target_validation_card_path target_source_seed_path validation_status disposition block_reason""".split()
TARGET_MATCH_FIELDS = {
    "target_package": "package",
    "target_crate_name": "crate_name",
    "target_runtime_symbol": "runtime_symbol",
    "target_contract_path": "contract_path",
    "target_validation_card_path": "validation_card_path",
    "target_source_seed_path": "source_seed_path",
}
ALIASES = {
    "specific_angular_momentum": "formula_vault.astrodynamics.elements.specific_angular_momentum",
    "eccentricity_vector": "formula_vault.astrodynamics.elements.eccentricity_vector",
}
HELPER_MATH = {"ch1_atan2"}
HELPER_PARAMETER = {"gravitational_parameter", "two_body_mu"}
HELPER_DYNAMICS = {
    "two_body_accel",
    "two_body_state_derivative",
    "grav_force",
    "pair_grav_force",
    "nbody_grav_accel",
    "all_nbody_grav_accels",
    "general_nbody_accel",
}
STATE_ANGLE = {"radial_velocity", "transverse_speed", "flight_path_angle", "zenith_angle"}
ENERGY = {"speed_from_energy", "a_from_energy"}
SEMILATUS = {"p_from_h"}
ECC_ENERGY_H = {"e_from_energy_h"}
APSIS_ELLIPSE = {
    "rp_from_p_e",
    "ra_from_p_e",
    "ra_from_a_e",
    "a_from_rp_ra",
    "e_from_rp_ra",
    "p_from_rp_e",
    "apsis_speed_from_h",
    "ellipse_b",
    "ellipse_c",
    "ellipse_period",
}
CONIC_CLASS = {"conic_type"}
TRUE_ANOMALY = {"true_anomaly"}
CONIC_STATE = {"conic_dimensions", "conic_from_rv", "periapsis_b_vector"}
PARABOLIC = {"parabolic_rp_from_p", "parabolic_p_from_rp"}
HYPERBOLIC = {"hyperbolic_turning_angle", "hyperbolic_excess_speed", "burnout_speed_from_vinf"}


class VerificationError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def stable_json(value: Any) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def repo_file(repo: Path, relative: str) -> Path:
    path = repo / relative
    require(path.is_file(), f"missing repository file: {relative}")
    return path


def read_delimited(path: Path, delimiter: str, expected_header: list[str] | None = None) -> list[dict[str, str]]:
    with path.open(encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle, delimiter=delimiter)
        require(reader.fieldnames is not None, f"missing header: {path}")
        if expected_header is not None:
            require(reader.fieldnames == expected_header, f"unsupported header in {path}: {reader.fieldnames}")
        rows = list(reader)
    require(rows, f"no data rows: {path}")
    return rows


def unique_map(rows: Iterable[dict[str, str]], key: str, label: str) -> dict[str, dict[str, str]]:
    output: dict[str, dict[str, str]] = {}
    for index, row in enumerate(rows, 1):
        value = row.get(key, "")
        require(value != "", f"{label} row {index} missing {key}")
        require(value not in output, f"duplicate {label} {key}: {value}")
        output[value] = row
    return output


def source_row_number(locator: str) -> int:
    match = re.fullmatch(r"PORT_STATUS_RELEASE_GATE\.csv:row_(\d{4})", locator)
    require(match is not None, f"invalid source row locator: {locator}")
    return int(match.group(1))


def expected_resolution(alias: str) -> tuple[str, str | None, str]:
    if alias in ALIASES:
        return (
            "deduplicated_alias_to_existing_runtime",
            ALIASES[alias],
            "not_applicable_existing_runtime_and_contract_reused",
        )
    if alias in HELPER_MATH:
        return (
            "excluded_internal_scalar_math_helper_not_formula",
            None,
            "generic_atan2_wrapper_is_internal_math_utility_not_a_separate_formula_node",
        )
    if alias in HELPER_PARAMETER:
        return (
            "excluded_parameter_lookup_helper_not_formula",
            None,
            "gravitational_parameter_lookup_or_wrapper_is_governed_parameter_support_not_a_separate_formula_node",
        )
    if alias in HELPER_DYNAMICS:
        return (
            "excluded_dynamics_support_algorithm_not_formula",
            None,
            "vector_force_acceleration_or_state_derivative_support_algorithm_is_outside_bounded_scalar_formula_scope",
        )
    if alias in STATE_ANGLE:
        return (
            "blocked_missing_state_geometry_and_angle_contract",
            None,
            "state_decomposition_and_angle_output_require_explicit_vector_input_frame_quadrant_wrap_and_singularity_contract",
        )
    if alias in ENERGY:
        return (
            "blocked_ambiguous_energy_input_and_conic_branch_contract",
            None,
            "energy_alias_does_not_establish_specific_energy_input_form_units_or_elliptic_hyperbolic_branch_semantics",
        )
    if alias in SEMILATUS:
        return (
            "blocked_missing_semilatus_rectum_contract_and_runtime",
            None,
            "semilatus_rectum_from_angular_momentum_requires_explicit_units_domain_and_runtime_contract",
        )
    if alias in ECC_ENERGY_H:
        return (
            "blocked_missing_eccentricity_from_energy_momentum_contract",
            None,
            "eccentricity_from_energy_and_angular_momentum_requires_explicit_units_domain_radicand_and_conic_branch_policy",
        )
    if alias in APSIS_ELLIPSE:
        return (
            "blocked_missing_apsis_ellipse_geometry_contract_and_runtime",
            None,
            "apsis_or_ellipse_relation_requires_explicit_conic_domain_denominator_boundary_and_input_semantics_contract",
        )
    if alias in CONIC_CLASS:
        return (
            "blocked_missing_conic_classification_boundary_contract",
            None,
            "conic_classification_requires_explicit_eccentricity_tolerance_parabolic_boundary_and_output_enum_contract",
        )
    if alias in TRUE_ANOMALY:
        return (
            "blocked_ambiguous_true_anomaly_state_and_singularity_contract",
            None,
            "true_anomaly_alias_does_not_establish_required_state_inputs_circular_or_equatorial_singularity_handling_quadrant_or_wrapping_policy",
        )
    if alias in CONIC_STATE:
        return (
            "blocked_missing_conic_state_geometry_contract",
            None,
            "composite_conic_geometry_output_requires_explicit_state_input_output_shape_singularity_and_branch_contract",
        )
    if alias in PARABOLIC:
        return (
            "blocked_missing_parabolic_boundary_contract",
            None,
            "parabolic_relation_requires_exact_eccentricity_boundary_convention_units_and_branch_contract",
        )
    if alias in HYPERBOLIC:
        return (
            "blocked_missing_hyperbolic_branch_and_mission_geometry_contract",
            None,
            "hyperbolic_relation_requires_explicit_excess_speed_asymptote_turning_angle_body_radius_and_mission_geometry_semantics",
        )
    raise VerificationError(f"unsupported A16 alias: {alias}")


def require_logical_source_locator(locator: str, row_index: int) -> None:
    require(locator != "", f"row {row_index} source_file_locator is empty")
    require(not locator.startswith(("/", "\\")), f"row {row_index} has absolute source locator")
    require(re.match(r"^[A-Za-z]:[\\/]", locator) is None, f"row {row_index} has Windows-absolute source locator")
    require(".." not in Path(locator).parts, f"row {row_index} source locator traverses parents")


def external_resolution_inventory(repo: Path, inventory_rows: list[dict[str, str]], metadata_count: int) -> tuple[int, int]:
    processed = [row for row in inventory_rows if row["category"] == "external_m07_processed_row"]
    backlog = [row for row in inventory_rows if row["category"] == "external_m07_backlog_row"]
    processed_map = unique_map(processed, "source_path", "external processed inventory")
    manifests = sorted((repo / "formula-vault/resolutions").glob("m07_*.tsv"))
    require(manifests, "no external M07 resolution manifests found")
    total = 0
    expected_paths: set[str] = set()
    global_resolution_ids: set[str] = set()
    global_source_locators: set[str] = set()
    for path in manifests:
        relative = path.relative_to(repo).as_posix()
        expected_paths.add(relative)
        rows = read_delimited(path, "\t", EXPECTED_HEADER)
        inventory = processed_map.get(relative)
        require(inventory is not None, f"missing processed inventory row for {relative}")
        require(inventory["row_count"] == str(len(rows)), f"processed inventory count mismatch for {relative}")
        for row in rows:
            require(row["resolution_id"] not in global_resolution_ids, f"duplicate external resolution ID: {row['resolution_id']}")
            global_resolution_ids.add(row["resolution_id"])
            require(row["source_row_locator"] not in global_source_locators, f"duplicate external source-row locator: {row['source_row_locator']}")
            global_source_locators.add(row["source_row_locator"])
        total += len(rows)
    require(set(processed_map) == expected_paths, "processed inventory sources and external resolution manifests are not an exact union")
    require(len(backlog) == 1, f"expected one backlog aggregate inventory row, found {len(backlog)}")
    expected = M07_REPRESENTED_FUNCTION_ROWS - metadata_count - total
    require(backlog[0]["row_count"] == str(expected), "external backlog count mismatch")
    return total, expected


def verify_repo(repo: Path) -> dict[str, Any]:
    repo = repo.resolve()
    require(repo.is_dir(), f"repository does not exist: {repo}")
    classifier_rows = read_delimited(repo_file(repo, CLASSIFIER_PATH), ",")
    group = [row for row in classifier_rows if row["recommended_chunk_group"] == TARGET_CHUNK]
    group.sort(key=lambda row: source_row_number(row["m07_row_id_or_alias"]))
    require(len(group) == EXPECTED_CLASSIFIER_GROUP_ROWS, f"expected {EXPECTED_CLASSIFIER_GROUP_ROWS} classifier rows, found {len(group)}")
    selected = group[:EXPECTED_ROWS]
    remaining = group[EXPECTED_ROWS:]
    require(len(selected) == EXPECTED_ROWS, f"expected {EXPECTED_ROWS} selected rows, found {len(selected)}")
    require(len(remaining) == EXPECTED_GROUP_REMAINING_ROWS, f"expected {EXPECTED_GROUP_REMAINING_ROWS} remaining rows, found {len(remaining)}")
    classifier = unique_map(selected, "m07_row_id_or_alias", "classifier")
    for locator, row in classifier.items():
        require(row["formula_family"] == "orbit_two_body", f"classifier family mismatch: {locator}")
        require(row["risk_tier"] == "medium_risk_requires_contract_review", f"classifier risk mismatch: {locator}")
        require(row["implementation_readiness"] == "contract_review_first_no_implementation", f"classifier readiness mismatch: {locator}")
        require(row["block_reason"] == "blocked_until_orbit_geometry_conic_branch_and_validation_policy", f"classifier block reason mismatch: {locator}")
    rows = read_delimited(repo_file(repo, RESOLUTION_PATH), "\t", EXPECTED_HEADER)
    require(len(rows) == EXPECTED_ROWS, f"expected {EXPECTED_ROWS} resolution rows, found {len(rows)}")
    resolutions = unique_map(rows, "source_row_locator", "resolution")
    unique_map(rows, "resolution_id", "resolution")
    require(set(resolutions) == set(classifier), "classifier selection and resolution locators are not an exact union")
    a7 = unique_map(read_delimited(repo_file(repo, A7_BATCH_PATH), "\t"), "formula_id", "A7 batch")
    dispositions: Counter[str] = Counter()
    targets: Counter[str] = Counter()
    row_numbers: list[int] = []
    source_files: set[str] = set()
    for index, row in enumerate(rows, 1):
        locator = row["source_row_locator"]
        source = classifier[locator]
        number = source_row_number(locator)
        row_numbers.append(number)
        require(row["schema_version"] == SCHEMA_VERSION, f"row {index} schema mismatch")
        require(row["resolution_id"] == f"resolution.external_m07.orbital_geometry_conic_wave1.{number:04d}", f"row {index} resolution ID mismatch")
        require(row["source_artifact_id"] == SOURCE_ARTIFACT_ID, f"row {index} source artifact mismatch")
        require(row["classifier_path"] == CLASSIFIER_PATH, f"row {index} classifier path mismatch")
        require(row["source_row_number"] == str(number), f"row {index} source row number mismatch")
        require_logical_source_locator(row["source_file_locator"], index)
        for field, classifier_field in [
            ("rust_function_alias", "rust_function_alias"),
            ("scilab_function_alias", "scilab_function_alias_if_known"),
            ("source_file_locator", "source_file_locator"),
            ("formula_family", "formula_family"),
            ("risk_tier", "risk_tier"),
            ("recommended_chunk_group", "recommended_chunk_group"),
        ]:
            require(row[field] == source[classifier_field], f"row {index} classifier mismatch for {field}")
        require(row["validation_status"] == "research_required", f"row {index} validation status mismatch")
        disposition, target_formula, reason = expected_resolution(row["rust_function_alias"])
        require(row["disposition"] == disposition, f"row {index} disposition mismatch")
        require(row["block_reason"] == reason, f"row {index} block reason mismatch")
        dispositions[disposition] += 1
        source_files.add(row["source_file_locator"])
        if target_formula is None:
            for field in ["target_formula_id", "target_resolution_id", "target_batch_manifest", *TARGET_MATCH_FIELDS, "target_runtime_path"]:
                require(row[field] == "", f"row {index} non-alias row must leave {field} empty")
        else:
            require(row["target_formula_id"] == target_formula, f"row {index} target formula mismatch")
            require(row["target_resolution_id"] == "", f"row {index} direct-batch alias must leave target_resolution_id empty")
            target = a7.get(target_formula)
            require(target is not None, f"row {index} target formula missing from A7 batch")
            require(row["target_batch_manifest"] == A7_BATCH_PATH, f"row {index} target batch mismatch")
            for field, target_field in TARGET_MATCH_FIELDS.items():
                require(row[field] == target[target_field], f"row {index} target mismatch for {field}")
            require(row["target_runtime_path"] == f"{target['crate_name']}::{target['runtime_symbol']}", f"row {index} runtime path mismatch")
            targets[target_formula] += 1
            for path_field in ["target_batch_manifest", "target_contract_path", "target_validation_card_path", "target_source_seed_path"]:
                repo_file(repo, row[path_field])
    require(row_numbers == sorted(row_numbers), "resolution rows are not deterministic source-row order")
    require(dispositions == EXPECTED_DISPOSITIONS, f"disposition counts mismatch: {dict(dispositions)}")
    require(dict(sorted(targets.items())) == EXPECTED_TARGET_COUNTS, f"target counts mismatch: {dict(targets)}")
    inventory_rows = read_delimited(repo_file(repo, INVENTORY_PATH), "\t")
    executable = [row for row in inventory_rows if row["category"] == "executable_research_equation"]
    metadata = [row for row in inventory_rows if row["category"] == "metadata_only_formula_vault_candidate"]
    require(len(executable) == EXPECTED_EXECUTABLE_ROWS, f"executable inventory mismatch: {len(executable)}")
    require(len(metadata) == EXPECTED_METADATA_ROWS, f"metadata inventory mismatch: {len(metadata)}")
    total, backlog = external_resolution_inventory(repo, inventory_rows, len(metadata))
    require(total == EXPECTED_CUMULATIVE_PROCESSED, f"cumulative processed mismatch: {total}")
    require(backlog == EXPECTED_REMAINING_BACKLOG, f"remaining backlog mismatch: {backlog}")
    excluded = (
        dispositions["excluded_internal_scalar_math_helper_not_formula"]
        + dispositions["excluded_parameter_lookup_helper_not_formula"]
        + dispositions["excluded_dynamics_support_algorithm_not_formula"]
    )
    blocked = len(rows) - dispositions["deduplicated_alias_to_existing_runtime"] - excluded
    return {
        "schema_version": SCHEMA_VERSION,
        "result": "PASS",
        "wave_id": "a16_external_m07_orbital_geometry_conic_wave1",
        "classifier_group_rows": len(group),
        "classifier_rows_selected": len(selected),
        "classifier_group_remaining_rows": len(remaining),
        "terminal_disposition_rows": len(rows),
        "deduplicated_alias_rows": dispositions["deduplicated_alias_to_existing_runtime"],
        "excluded_helper_rows": excluded,
        "excluded_scalar_math_helper_rows": dispositions["excluded_internal_scalar_math_helper_not_formula"],
        "excluded_parameter_lookup_rows": dispositions["excluded_parameter_lookup_helper_not_formula"],
        "excluded_dynamics_support_algorithm_rows": dispositions["excluded_dynamics_support_algorithm_not_formula"],
        "contract_blocked_rows": blocked,
        "state_geometry_angle_contract_blocked_rows": dispositions["blocked_missing_state_geometry_and_angle_contract"],
        "energy_input_contract_blocked_rows": dispositions["blocked_ambiguous_energy_input_and_conic_branch_contract"],
        "semilatus_rectum_contract_blocked_rows": dispositions["blocked_missing_semilatus_rectum_contract_and_runtime"],
        "eccentricity_energy_momentum_contract_blocked_rows": dispositions["blocked_missing_eccentricity_from_energy_momentum_contract"],
        "apsis_ellipse_contract_blocked_rows": dispositions["blocked_missing_apsis_ellipse_geometry_contract_and_runtime"],
        "conic_classification_contract_blocked_rows": dispositions["blocked_missing_conic_classification_boundary_contract"],
        "true_anomaly_contract_blocked_rows": dispositions["blocked_ambiguous_true_anomaly_state_and_singularity_contract"],
        "conic_state_geometry_contract_blocked_rows": dispositions["blocked_missing_conic_state_geometry_contract"],
        "parabolic_boundary_contract_blocked_rows": dispositions["blocked_missing_parabolic_boundary_contract"],
        "hyperbolic_mission_geometry_contract_blocked_rows": dispositions["blocked_missing_hyperbolic_branch_and_mission_geometry_contract"],
        "target_formula_counts": dict(sorted(targets.items())),
        "distinct_source_files": len(source_files),
        "classifier_risk_tier": "medium_risk_requires_contract_review",
        "risk_tier_not_downgraded": True,
        "executable_research_equations": len(executable),
        "metadata_inventory_records": len(metadata),
        "external_m07_processed_rows": total,
        "external_m07_backlog_rows": backlog,
        "formula_count_delta": 0,
        "runtime_kernel_files_changed": 0,
        "new_validation_cards_required": 0,
        "new_source_seeds_required": 0,
        "validation_status": "research_required",
        "no_rust_m07_or_scilab_source_scraping": True,
        "no_external_parity_claim": True,
        "no_certification_or_operational_readiness_claim": True,
    }


def self_test() -> dict[str, Any]:
    tests: list[dict[str, str]] = []
    require(stable_json({"b": 2, "a": 1}).startswith('{\n  "a"'), "stable JSON ordering failed")
    tests.append({"name": "stable_json", "result": "PASS"})
    mappings = {
        "specific_angular_momentum": ("deduplicated_alias_to_existing_runtime", "formula_vault.astrodynamics.elements.specific_angular_momentum"),
        "eccentricity_vector": ("deduplicated_alias_to_existing_runtime", "formula_vault.astrodynamics.elements.eccentricity_vector"),
        "ch1_atan2": ("excluded_internal_scalar_math_helper_not_formula", None),
        "two_body_accel": ("excluded_dynamics_support_algorithm_not_formula", None),
        "true_anomaly": ("blocked_ambiguous_true_anomaly_state_and_singularity_contract", None),
        "parabolic_rp_from_p": ("blocked_missing_parabolic_boundary_contract", None),
        "hyperbolic_turning_angle": ("blocked_missing_hyperbolic_branch_and_mission_geometry_contract", None),
    }
    for alias, expected in mappings.items():
        require(expected_resolution(alias)[:2] == expected, f"mapping self-test failed: {alias}")
    tests.append({"name": "deterministic_alias_exclusion_and_block_mapping", "result": "PASS"})
    duplicate_rejected = False
    try:
        unique_map([{"x": "a"}, {"x": "a"}], "x", "fixture")
    except VerificationError:
        duplicate_rejected = True
    require(duplicate_rejected, "duplicate fixture not rejected")
    tests.append({"name": "duplicate_rejected", "result": "PASS"})
    return {"schema_version": SCHEMA_VERSION, "mode": "self-test", "result": "PASS", "tests": tests}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path("."))
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        print(stable_json(self_test() if args.self_test else verify_repo(args.repo)), end="")
        return 0
    except Exception as error:
        print(stable_json({"schema_version": SCHEMA_VERSION, "result": "FAIL", "error": str(error)}), end="", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
