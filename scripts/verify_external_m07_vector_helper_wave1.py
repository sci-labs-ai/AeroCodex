#!/usr/bin/env python3
"""Verify A12 external M07 vector-helper Wave 1 terminal dispositions.

This standard-library-only verifier consumes classifier metadata and explicit
resolution records. It never opens or parses raw M07 or Scilab source text.
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
CLASSIFIER_PATH = (
    "docs/source_intake/m07_formula_family_classifier/"
    "low_risk_candidate_shortlist.csv"
)
RESOLUTION_PATH = "formula-vault/resolutions/m07_vector_helper_wave1.tsv"
M00_RUNTIME_LINKS_PATH = "formula-vault/resolutions/m00_runtime_links.tsv"
INVENTORY_PATH = "validation/equation_inventory.tsv"
SOURCE_ARTIFACT_ID = "stage4.m07_rust_port_v14.2026_06_15"
TARGET_CHUNK = "8D_helper_deduplication_then_low_risk_vector_contracts"
M07_REPRESENTED_FUNCTION_ROWS = 1350
EXPECTED_CLASSIFIER_GROUP_ROWS = 74
EXPECTED_ROWS = 40
EXPECTED_EXECUTABLE_ROWS = 152
EXPECTED_METADATA_ROWS = 27
EXPECTED_TARGET_COUNTS = {
    "formula_vault.m00.vector.angle": 4,
    "formula_vault.m00.vector.cross": 7,
    "formula_vault.m00.vector.dot": 7,
    "formula_vault.m00.vector.norm": 7,
    "formula_vault.m00.vector.unit": 5,
}
EXPECTED_DISPOSITIONS = Counter(
    {
        "deduplicated_alias_to_existing_runtime": 30,
        "excluded_internal_shape_helper_not_formula": 8,
        "blocked_missing_determinant_semantics_contract": 1,
        "blocked_missing_coordinate_frame_contract": 1,
    }
)
EXPECTED_HEADER = """schema_version resolution_id source_artifact_id classifier_path
source_row_locator source_row_number rust_function_alias scilab_function_alias
source_file_locator formula_family risk_tier recommended_chunk_group
target_formula_id target_resolution_id target_batch_manifest target_package
target_crate_name target_runtime_symbol target_runtime_path target_contract_path
target_validation_card_path target_source_seed_path validation_status disposition
block_reason""".split()
TARGET_MATCH_FIELDS = {
    "target_resolution_id": "resolution_id",
    "target_batch_manifest": "batch_manifest",
    "target_package": "package",
    "target_crate_name": "crate_name",
    "target_runtime_symbol": "runtime_symbol",
    "target_runtime_path": "runtime_path",
    "target_contract_path": "contract_path",
    "target_validation_card_path": "validation_card_path",
    "target_source_seed_path": "source_seed_path",
}


class VerificationError(RuntimeError):
    """Fail-closed metadata verification error."""


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
    mapped: dict[str, dict[str, str]] = {}
    for index, row in enumerate(rows, 1):
        value = row.get(key, "")
        require(value != "", f"{label} row {index} missing {key}")
        require(value not in mapped, f"duplicate {label} {key}: {value}")
        mapped[value] = row
    return mapped


def source_row_number(locator: str) -> int:
    match = re.fullmatch(r"PORT_STATUS_RELEASE_GATE\.csv:row_(\d{4})", locator)
    require(match is not None, f"invalid source row locator: {locator}")
    return int(match.group(1))


def expected_resolution(alias: str) -> tuple[str, str | None, str]:
    lowered = alias.lower()
    tail = lowered.split("::")[-1]
    if lowered == "ch6::latlon_to_unit":
        return (
            "blocked_missing_coordinate_frame_contract",
            None,
            "latitude_longitude_axis_handedness_and_angle_unit_contract_required",
        )
    if lowered == "ch2_det3_cols":
        return (
            "blocked_missing_determinant_semantics_contract",
            None,
            "determinant_column_order_and_scalar_triple_equivalence_require_explicit_contract",
        )
    if tail in {"col", "columnize"} or lowered.endswith("_col") or lowered in {"ch2_eye3", "num_columnize"}:
        return (
            "excluded_internal_shape_helper_not_formula",
            None,
            "column_shape_normalization_or_identity_constructor_is_internal_utility_not_public_formula",
        )
    if tail.endswith("angle_between"):
        return ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.angle", "not_applicable_existing_runtime_and_contract_reused")
    if tail.endswith("veccross") or tail.endswith("cross"):
        return ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.cross", "not_applicable_existing_runtime_and_contract_reused")
    if tail.endswith("vecdot") or tail.endswith("dot"):
        return ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.dot", "not_applicable_existing_runtime_and_contract_reused")
    if tail.endswith("vnorm") or tail.endswith("norm"):
        return ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.norm", "not_applicable_existing_runtime_and_contract_reused")
    if tail.endswith("unit"):
        return ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.unit", "not_applicable_existing_runtime_and_contract_reused")
    raise VerificationError(f"unsupported vector-helper alias: {alias}")


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
    for path in manifests:
        relative = path.relative_to(repo).as_posix()
        expected_paths.add(relative)
        manifest_rows = read_delimited(path, "\t", EXPECTED_HEADER)
        inventory_row = processed_map.get(relative)
        require(inventory_row is not None, f"missing processed inventory row for {relative}")
        require(inventory_row["row_count"] == str(len(manifest_rows)), f"processed inventory count mismatch for {relative}")
        total += len(manifest_rows)
    require(set(processed_map) == expected_paths, "processed inventory sources and external resolution manifests are not an exact union")
    require(len(backlog) == 1, f"expected one backlog aggregate inventory row, found {len(backlog)}")
    expected_backlog = M07_REPRESENTED_FUNCTION_ROWS - metadata_count - total
    require(backlog[0]["row_count"] == str(expected_backlog), "external backlog count mismatch")
    return total, expected_backlog


def verify_repo(repo: Path) -> dict[str, Any]:
    repo = repo.resolve()
    require(repo.is_dir(), f"repository does not exist: {repo}")
    classifier_rows = read_delimited(repo_file(repo, CLASSIFIER_PATH), ",")
    group_rows = [row for row in classifier_rows if row["recommended_chunk_group"] == TARGET_CHUNK]
    group_rows.sort(key=lambda row: source_row_number(row["m07_row_id_or_alias"]))
    require(len(group_rows) == EXPECTED_CLASSIFIER_GROUP_ROWS, f"expected {EXPECTED_CLASSIFIER_GROUP_ROWS} group rows, found {len(group_rows)}")
    selected = group_rows[:EXPECTED_ROWS]
    classifier = unique_map(selected, "m07_row_id_or_alias", "classifier")
    for locator, row in classifier.items():
        require(row["formula_family"] == "low_risk_vector_math", f"classifier family mismatch: {locator}")
        require(row["risk_tier"] == "low_risk_candidate", f"classifier risk mismatch: {locator}")
        require(row["contract_review_needed"] == "yes_standard_formula_vault_contract_and_source_locator_review", f"classifier review flag mismatch: {locator}")

    resolution_rows = read_delimited(repo_file(repo, RESOLUTION_PATH), "\t", EXPECTED_HEADER)
    require(len(resolution_rows) == EXPECTED_ROWS, f"expected {EXPECTED_ROWS} resolution rows, found {len(resolution_rows)}")
    resolutions = unique_map(resolution_rows, "source_row_locator", "resolution")
    unique_map(resolution_rows, "resolution_id", "resolution")
    require(set(resolutions) == set(classifier), "classifier selection and resolution row locators are not an exact union")

    runtime_links = unique_map(read_delimited(repo_file(repo, M00_RUNTIME_LINKS_PATH), "\t"), "formula_id", "M00 runtime resolution")
    disposition_counts: Counter[str] = Counter()
    target_counts: Counter[str] = Counter()
    source_numbers: list[int] = []
    source_files: Counter[str] = Counter()
    for index, row in enumerate(resolution_rows, 1):
        locator = row["source_row_locator"]
        source = classifier[locator]
        number = source_row_number(locator)
        source_numbers.append(number)
        require(row["schema_version"] == SCHEMA_VERSION, f"row {index} schema mismatch")
        require(row["resolution_id"] == f"resolution.external_m07.vector_helper.{number:04d}", f"row {index} resolution ID mismatch")
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
        disposition_counts[disposition] += 1
        source_files[row["source_file_locator"]] += 1
        if target_formula is None:
            for field in ["target_formula_id", *TARGET_MATCH_FIELDS]:
                require(row[field] == "", f"row {index} non-alias disposition must leave {field} empty")
            continue
        require(row["target_formula_id"] == target_formula, f"row {index} target formula mismatch")
        target = runtime_links.get(target_formula)
        require(target is not None, f"row {index} target formula missing from M00 runtime resolution")
        for field, target_field in TARGET_MATCH_FIELDS.items():
            require(row[field] == target[target_field], f"row {index} target mismatch for {field}")
        target_counts[target_formula] += 1
        for path_field in ["target_batch_manifest", "target_contract_path", "target_validation_card_path", "target_source_seed_path"]:
            repo_file(repo, row[path_field])

    require(source_numbers == sorted(source_numbers), "resolution rows are not in deterministic source-row order")
    require(disposition_counts == EXPECTED_DISPOSITIONS, f"disposition counts mismatch: {dict(disposition_counts)}")
    require(dict(sorted(target_counts.items())) == EXPECTED_TARGET_COUNTS, f"target counts mismatch: {dict(target_counts)}")

    inventory = read_delimited(repo_file(repo, INVENTORY_PATH), "\t")
    executable = [row for row in inventory if row["category"] == "executable_research_equation"]
    metadata = [row for row in inventory if row["category"] == "metadata_only_formula_vault_candidate"]
    require(len(executable) == EXPECTED_EXECUTABLE_ROWS, f"executable inventory count mismatch: {len(executable)}")
    require(len(metadata) == EXPECTED_METADATA_ROWS, f"metadata inventory count mismatch: {len(metadata)}")
    total_processed, backlog = external_resolution_inventory(repo, inventory, len(metadata))

    return {
        "schema_version": SCHEMA_VERSION,
        "result": "PASS",
        "wave_id": "a12_external_m07_vector_helper_wave1",
        "classifier_group_rows": len(group_rows),
        "classifier_rows_selected": len(selected),
        "terminal_disposition_rows": len(resolution_rows),
        "deduplicated_alias_rows": disposition_counts["deduplicated_alias_to_existing_runtime"],
        "excluded_helper_rows": disposition_counts["excluded_internal_shape_helper_not_formula"],
        "contract_blocked_rows": disposition_counts["blocked_missing_determinant_semantics_contract"] + disposition_counts["blocked_missing_coordinate_frame_contract"],
        "determinant_contract_blocked_rows": disposition_counts["blocked_missing_determinant_semantics_contract"],
        "coordinate_frame_contract_blocked_rows": disposition_counts["blocked_missing_coordinate_frame_contract"],
        "target_formula_counts": dict(sorted(target_counts.items())),
        "distinct_source_files": len(source_files),
        "executable_research_equations": len(executable),
        "metadata_inventory_records": len(metadata),
        "external_m07_processed_rows": total_processed,
        "external_m07_backlog_rows": backlog,
        "formula_count_delta": 0,
        "runtime_kernel_files_changed": 0,
        "new_validation_cards_required": 0,
        "new_source_seeds_required": 0,
        "validation_status": "research_required",
        "no_rust_or_scilab_source_scraping": True,
        "no_external_parity_claim": True,
        "no_certification_or_operational_readiness_claim": True,
    }


def self_test() -> dict[str, Any]:
    tests: list[dict[str, str]] = []
    require(stable_json({"b": 2, "a": 1}).startswith('{\n  "a"'), "stable JSON ordering failed")
    tests.append({"name": "stable_json", "result": "PASS"})
    mappings = {
        "ch1_dot": ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.dot"),
        "num_veccross": ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.cross"),
        "ch4::norm": ("deduplicated_alias_to_existing_runtime", "formula_vault.m00.vector.norm"),
        "ch1_col": ("excluded_internal_shape_helper_not_formula", None),
        "ch2_det3_cols": ("blocked_missing_determinant_semantics_contract", None),
        "ch6::latlon_to_unit": ("blocked_missing_coordinate_frame_contract", None),
    }
    for alias, expected in mappings.items():
        actual = expected_resolution(alias)
        require(actual[:2] == expected, f"mapping self-test failed: {alias}")
    tests.append({"name": "deterministic_alias_mapping", "result": "PASS"})
    duplicate_rejected = False
    try:
        unique_map([{"x": "a"}, {"x": "a"}], "x", "fixture")
    except VerificationError:
        duplicate_rejected = True
    require(duplicate_rejected, "duplicate fixture was not rejected")
    tests.append({"name": "duplicate_rejected", "result": "PASS"})
    absolute_rejected = False
    try:
        require_logical_source_locator("C:\\private\\source.sci", 1)
    except VerificationError:
        absolute_rejected = True
    require(absolute_rejected, "absolute source locator fixture was not rejected")
    tests.append({"name": "absolute_source_locator_rejected", "result": "PASS"})
    return {"schema_version": SCHEMA_VERSION, "mode": "self-test", "result": "PASS", "tests": tests}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path("."), help="repository root")
    parser.add_argument("--self-test", action="store_true", help="run internal checks")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        report = self_test() if args.self_test else verify_repo(args.repo)
        print(stable_json(report), end="")
        return 0
    except Exception as error:
        print(stable_json({"schema_version": SCHEMA_VERSION, "result": "FAIL", "error": str(error)}), end="", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
