#!/usr/bin/env python3
"""Verify formula-vault candidate-to-runtime resolution records.

This verifier is deliberately standard-library-only. It treats the files under
``formula-vault/candidates`` as intake metadata records and verifies that each
selected formula ID has one explicit, research-required link to an already
governed equation-batch runtime. It does not inspect Rust source text.
"""
from __future__ import annotations

import argparse
import csv
import json
import sys
import tempfile
from collections import Counter
from pathlib import Path
from typing import Any, Iterable

SCHEMA_VERSION = "aerocodex.formula_vault_runtime_resolution.v1"
RESOLUTION_MANIFEST = "formula-vault/resolutions/m00_runtime_links.tsv"
EXPECTED_RESOLUTION_ROWS = 27
EXPECTED_EXECUTABLE_INVENTORY_ROWS = 152
M07_REPRESENTED_FUNCTION_ROWS = 1350
EXTERNAL_RESOLUTION_SCHEMA = "aerocodex.external_m07_resolution.v1"
EXTERNAL_RESOLUTION_HEADER = [
    "schema_version",
    "resolution_id",
    "source_artifact_id",
    "classifier_path",
    "source_row_locator",
    "source_row_number",
    "rust_function_alias",
    "scilab_function_alias",
    "source_file_locator",
    "formula_family",
    "risk_tier",
    "recommended_chunk_group",
    "target_formula_id",
    "target_resolution_id",
    "target_batch_manifest",
    "target_package",
    "target_crate_name",
    "target_runtime_symbol",
    "target_runtime_path",
    "target_contract_path",
    "target_validation_card_path",
    "target_source_seed_path",
    "validation_status",
    "disposition",
    "block_reason",
]
EXPECTED_FAMILY_COUNTS = {
    "angle_unit_conversions": 3,
    "canonical_unit_conversions": 10,
    "vector_algebra": 14,
}
EXPECTED_HEADER = [
    "schema_version",
    "resolution_id",
    "formula_id",
    "candidate_family",
    "candidate_path",
    "batch_manifest",
    "batch_id",
    "package",
    "crate_name",
    "runtime_symbol",
    "runtime_path",
    "output_variable",
    "contract_path",
    "validation_card_path",
    "source_seed_path",
    "validation_status",
    "disposition",
]
BATCH_MATCH_FIELDS = [
    "batch_id",
    "package",
    "crate_name",
    "runtime_symbol",
    "output_variable",
    "contract_path",
    "validation_card_path",
    "source_seed_path",
    "validation_status",
]
RESOLVED_BLOCK_REASON = (
    "metadata_record_runtime_linked_research_required_"
    "no_source_parity_certification_or_operational_claim"
)


class VerificationError(RuntimeError):
    """A fail-closed resolution verification error."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def stable_json(value: Any) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def repo_path(repo: Path, relative: str) -> Path:
    path = repo / relative
    require(path.is_file(), f"missing repository file: {relative}")
    return path


def read_tsv(path: Path, expected_header: list[str] | None = None) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        require(reader.fieldnames is not None, f"missing TSV header: {path}")
        if expected_header is not None:
            require(reader.fieldnames == expected_header, f"unsupported TSV header in {path}: {reader.fieldnames}")
        rows = list(reader)
    require(rows, f"TSV has no rows: {path}")
    return rows


def unique_map(rows: Iterable[dict[str, str]], key: str, label: str) -> dict[str, dict[str, str]]:
    mapped: dict[str, dict[str, str]] = {}
    for index, row in enumerate(rows, 1):
        value = row.get(key, "")
        require(value != "", f"{label} row {index} missing {key}")
        require(value not in mapped, f"duplicate {label} {key}: {value}")
        mapped[value] = row
    return mapped


def external_resolution_summary(repo: Path) -> dict[str, Any]:
    paths = sorted((repo / "formula-vault/resolutions").glob("m07_*.tsv"))
    require(paths, "no external M07 resolution manifests found")
    resolution_ids: set[str] = set()
    source_row_locators: set[str] = set()
    row_count = 0
    for path in paths:
        rows = read_tsv(path, EXTERNAL_RESOLUTION_HEADER)
        for index, row in enumerate(rows, 1):
            require(
                row["schema_version"] == EXTERNAL_RESOLUTION_SCHEMA,
                f"external resolution row {path}:{index} schema mismatch",
            )
            require(
                row["resolution_id"] not in resolution_ids,
                f"duplicate external resolution_id: {row['resolution_id']}",
            )
            resolution_ids.add(row["resolution_id"])
            require(
                row["source_row_locator"] not in source_row_locators,
                f"duplicate external source_row_locator: {row['source_row_locator']}",
            )
            source_row_locators.add(row["source_row_locator"])
            require(
                row["validation_status"] == "research_required",
                f"external resolution row {path}:{index} status mismatch",
            )
            disposition = row["disposition"]
            require(
                disposition
                and "pending" not in disposition
                and "unresolved" not in disposition,
                f"external resolution row {path}:{index} is not terminal",
            )
            require(row["block_reason"] != "", f"external resolution row {path}:{index} lacks block_reason")
            row_count += 1
    return {
        "manifests": [path.relative_to(repo).as_posix() for path in paths],
        "row_count": row_count,
        "resolution_ids_unique": len(resolution_ids) == row_count,
        "source_row_locators_unique": len(source_row_locators) == row_count,
    }


def extract_candidate_formula_ids(path: Path) -> list[str]:
    """Extract only formula_contract.formula_ids from the constrained YAML shape."""
    lines = path.read_text(encoding="utf-8").splitlines()
    in_formula_contract = False
    in_formula_ids = False
    formula_ids: list[str] = []
    for raw in lines:
        stripped = raw.strip()
        if not stripped or stripped.startswith("#"):
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        if indent == 0:
            if stripped == "formula_contract:":
                in_formula_contract = True
                in_formula_ids = False
                continue
            if in_formula_contract:
                break
        if not in_formula_contract:
            continue
        if indent == 2 and stripped == "formula_ids:":
            in_formula_ids = True
            continue
        if in_formula_ids:
            if indent == 4 and stripped.startswith("- "):
                formula_ids.append(stripped[2:].strip())
                continue
            if indent <= 2:
                break
    require(formula_ids, f"candidate has no formula_contract.formula_ids: {path}")
    require(len(formula_ids) == len(set(formula_ids)), f"candidate has duplicate formula IDs: {path}")
    return formula_ids


def require_candidate_resolution_overlay(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    require(
        "record_status: metadata_record_runtime_linked_research_required" in text,
        f"candidate record_status is not runtime-linked: {path}",
    )
    require("runtime_resolution:" in text, f"candidate lacks runtime_resolution overlay: {path}")
    require(
        f"manifest_path: {RESOLUTION_MANIFEST}" in text,
        f"candidate resolution manifest path mismatch: {path}",
    )
    require(
        "disposition: linked_to_existing_runtime" in text,
        f"candidate disposition is not linked_to_existing_runtime: {path}",
    )
    require(
        "validation_status: research_required" in text,
        f"candidate runtime resolution must remain research_required: {path}",
    )
    require(
        "unresolved_formula_count: 0" in text,
        f"candidate runtime resolution must record zero unresolved formulas: {path}",
    )


def verify_repo(repo: Path) -> dict[str, Any]:
    repo = repo.resolve()
    require(repo.is_dir(), f"repository does not exist: {repo}")

    resolution_path = repo_path(repo, RESOLUTION_MANIFEST)
    resolution_rows = read_tsv(resolution_path, EXPECTED_HEADER)
    require(
        len(resolution_rows) == EXPECTED_RESOLUTION_ROWS,
        f"expected {EXPECTED_RESOLUTION_ROWS} resolution rows, found {len(resolution_rows)}",
    )
    resolutions = unique_map(resolution_rows, "formula_id", "resolution")
    resolution_ids = unique_map(resolution_rows, "resolution_id", "resolution")
    require(len(resolution_ids) == len(resolutions), "resolution ID count mismatch")

    candidate_paths = sorted({row["candidate_path"] for row in resolution_rows})
    candidate_formula_ids: set[str] = set()
    candidate_counts: dict[str, int] = {}
    for relative in candidate_paths:
        candidate_path = repo_path(repo, relative)
        require_candidate_resolution_overlay(candidate_path)
        formula_ids = extract_candidate_formula_ids(candidate_path)
        candidate_counts[relative] = len(formula_ids)
        for formula_id in formula_ids:
            require(formula_id not in candidate_formula_ids, f"duplicate formula ID across candidates: {formula_id}")
            candidate_formula_ids.add(formula_id)

    require(
        candidate_formula_ids == set(resolutions),
        "candidate formula IDs and runtime resolution formula IDs are not an exact union",
    )

    batch_cache: dict[str, dict[str, dict[str, str]]] = {}
    family_counter: Counter[str] = Counter()
    runtime_paths: set[str] = set()
    runtime_identities: set[tuple[str, str]] = set()
    for index, row in enumerate(resolution_rows, 1):
        require(row["schema_version"] == SCHEMA_VERSION, f"resolution row {index} schema mismatch")
        require(row["resolution_id"] == f"resolution.{row['formula_id']}", f"resolution row {index} ID mismatch")
        require(row["disposition"] == "linked_to_existing_runtime", f"resolution row {index} disposition mismatch")
        require(row["validation_status"] == "research_required", f"resolution row {index} status mismatch")
        require(row["runtime_path"] == f"{row['crate_name']}::{row['runtime_symbol']}", f"resolution row {index} runtime path mismatch")
        family_counter[row["candidate_family"]] += 1
        require(row["runtime_path"] not in runtime_paths, f"duplicate runtime path: {row['runtime_path']}")
        runtime_paths.add(row["runtime_path"])
        identity = (row["package"], row["runtime_symbol"])
        require(identity not in runtime_identities, f"duplicate package-scoped runtime identity: {identity}")
        runtime_identities.add(identity)

        require(row["formula_id"] in extract_candidate_formula_ids(repo_path(repo, row["candidate_path"])), f"formula not present in candidate: {row['formula_id']}")
        for field in ["contract_path", "validation_card_path", "source_seed_path", "batch_manifest"]:
            repo_path(repo, row[field])

        batch_manifest = row["batch_manifest"]
        if batch_manifest not in batch_cache:
            batch_rows = read_tsv(repo_path(repo, batch_manifest))
            batch_cache[batch_manifest] = unique_map(batch_rows, "formula_id", f"batch {batch_manifest}")
        batch_row = batch_cache[batch_manifest].get(row["formula_id"])
        require(batch_row is not None, f"resolution formula missing from batch manifest: {row['formula_id']}")
        for field in BATCH_MATCH_FIELDS:
            require(
                row[field] == batch_row[field],
                f"resolution/batch mismatch for {row['formula_id']} field {field}: {row[field]!r} != {batch_row[field]!r}",
            )

    require(dict(sorted(family_counter.items())) == EXPECTED_FAMILY_COUNTS, f"family counts mismatch: {dict(family_counter)}")

    inventory_rows = read_tsv(repo_path(repo, "validation/equation_inventory.tsv"))
    metadata_rows = [row for row in inventory_rows if row["category"] == "metadata_only_formula_vault_candidate"]
    executable_rows = [row for row in inventory_rows if row["category"] == "executable_research_equation"]
    processed_rows = [row for row in inventory_rows if row["category"] == "external_m07_processed_row"]
    external_rows = [row for row in inventory_rows if row["category"] == "external_m07_backlog_row"]
    external_resolution = external_resolution_summary(repo)
    expected_external_backlog = (
        M07_REPRESENTED_FUNCTION_ROWS
        - EXPECTED_RESOLUTION_ROWS
        - external_resolution["row_count"]
    )
    require(len(metadata_rows) == EXPECTED_RESOLUTION_ROWS, f"metadata inventory row count mismatch: {len(metadata_rows)}")
    require(len(executable_rows) == EXPECTED_EXECUTABLE_INVENTORY_ROWS, f"executable inventory row count mismatch: {len(executable_rows)}")
    processed_map = unique_map(processed_rows, "source_path", "external processed inventory")
    expected_manifest_paths = set(external_resolution["manifests"])
    require(
        set(processed_map) == expected_manifest_paths,
        "external processed inventory sources and resolution manifests are not an exact union",
    )
    processed_total = 0
    for relative in external_resolution["manifests"]:
        manifest_rows = read_tsv(repo_path(repo, relative), EXTERNAL_RESOLUTION_HEADER)
        require(
            int(processed_map[relative]["row_count"]) == len(manifest_rows),
            f"external processed inventory count mismatch: {relative}",
        )
        processed_total += len(manifest_rows)
    require(
        processed_total == external_resolution["row_count"],
        "external processed count does not match resolution manifests",
    )
    require(len(external_rows) == 1, f"expected one external backlog aggregate row, found {len(external_rows)}")
    require(
        int(external_rows[0]["row_count"]) == expected_external_backlog,
        "external backlog count does not match represented minus candidate and processed rows",
    )
    inventory_candidates = unique_map(metadata_rows, "id", "metadata inventory")
    require(set(inventory_candidates) == set(resolutions), "metadata inventory and resolution IDs differ")
    for formula_id, inventory_row in inventory_candidates.items():
        resolution = resolutions[formula_id]
        require(inventory_row["source_path"] == resolution["candidate_path"], f"inventory candidate path mismatch: {formula_id}")
        require(inventory_row["function_or_ref"] == formula_id, f"inventory formula reference mismatch: {formula_id}")
        require(inventory_row["status"] == "research_required", f"inventory status mismatch: {formula_id}")
        require(inventory_row["blocked"] == "true", f"inventory row must remain blocked: {formula_id}")
        require(inventory_row["block_reason"] == RESOLVED_BLOCK_REASON, f"inventory block reason mismatch: {formula_id}")
        require(inventory_row["row_count"] == "1", f"inventory row_count mismatch: {formula_id}")

    return {
        "schema_version": SCHEMA_VERSION,
        "result": "PASS",
        "resolution_manifest": RESOLUTION_MANIFEST,
        "candidate_record_count": len(candidate_formula_ids),
        "runtime_links_resolved": len(resolutions),
        "unresolved_candidate_count": len(candidate_formula_ids - set(resolutions)),
        "unexpected_resolution_count": len(set(resolutions) - candidate_formula_ids),
        "family_counts": dict(sorted(family_counter.items())),
        "batch_manifests": sorted(batch_cache),
        "metadata_inventory_records": len(metadata_rows),
        "executable_research_equations": len(executable_rows),
        "external_m07_resolution_manifests": external_resolution["manifests"],
        "external_m07_processed_rows": external_resolution["row_count"],
        "external_m07_backlog_rows": expected_external_backlog,
        "external_resolution_ids_unique": external_resolution["resolution_ids_unique"],
        "external_source_row_locators_unique": external_resolution["source_row_locators_unique"],
        "formula_ids_unique": len(resolutions) == len(set(resolutions)),
        "runtime_paths_unique": len(runtime_paths) == len(resolutions),
        "package_scoped_runtime_identities_unique": len(runtime_identities) == len(resolutions),
        "all_candidates_resolved": set(resolutions) == candidate_formula_ids,
        "disposition": "linked_to_existing_runtime",
        "validation_status": "research_required",
        "no_rust_source_scraping": True,
        "no_runtime_kernel_change_claim": True,
        "no_external_parity_claim": True,
        "no_certification_or_operational_readiness_claim": True,
    }


def self_test() -> dict[str, Any]:
    tests: list[dict[str, str]] = []
    require(stable_json({"b": 2, "a": 1}).startswith('{\n  "a"'), "stable JSON ordering failed")
    tests.append({"name": "stable_json", "result": "PASS"})
    with tempfile.TemporaryDirectory(prefix="aerocodex-resolution-self-test-") as name:
        path = Path(name) / "candidate.yaml"
        path.write_text(
            "formula_contract:\n  formula_ids:\n    - formula_vault.test.one\n    - formula_vault.test.two\nvalidation_records:\n  status: research_required\n",
            encoding="utf-8",
            newline="\n",
        )
        require(extract_candidate_formula_ids(path) == ["formula_vault.test.one", "formula_vault.test.two"], "candidate parser failed")
    tests.append({"name": "candidate_formula_id_parser", "result": "PASS"})
    duplicate_error = False
    try:
        unique_map([{"formula_id": "x"}, {"formula_id": "x"}], "formula_id", "fixture")
    except VerificationError:
        duplicate_error = True
    require(duplicate_error, "duplicate resolution fixture was not rejected")
    tests.append({"name": "duplicate_resolution_rejected", "result": "PASS"})
    return {"schema_version": SCHEMA_VERSION, "mode": "self-test", "result": "PASS", "tests": tests}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path("."), help="repository root")
    parser.add_argument("--self-test", action="store_true", help="run dependency-free internal checks")
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
