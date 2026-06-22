#!/usr/bin/env python3
"""Deterministic AeroCodex equation-batch planner, generator, and verifier.

The tool consumes explicit tab-separated batch manifests. It never parses Rust
source code. Runtime symbols and authored contract probes are checked by the
Rust compiler in a generated, temporary probe crate.
"""

from __future__ import annotations

import argparse
import csv
import dataclasses
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib
from pathlib import Path
from typing import Any, Iterable, Sequence

SCHEMA_VERSION = "aerocodex.equation_batch.v1"
MAX_BATCH_ROWS = 40
MANIFEST_FIELDS = [
    "schema_version",
    "batch_id",
    "formula_id",
    "package",
    "crate_name",
    "runtime_symbol",
    "output_variable",
    "contract_path",
    "validation_card_path",
    "source_seed_path",
    "validation_status",
    "test_strategy",
    "test_expression",
]
ALLOWED_TEST_STRATEGIES = {"exact", "tolerance", "invariant", "mixed"}
REQUIRED_VALIDATION_STATUS = "research_required"
INVENTORY_PATH = "validation/equation_inventory.tsv"
VERIFICATION_REPORT = "equation-batch-verification-report.json"
VERIFICATION_HASH = "equation-batch-verification-report.json.sha256"

FORBIDDEN_EXPRESSION_TOKENS = (
    "unsafe",
    "std::process",
    "process::command",
    "include!",
    "include_str!",
    "include_bytes!",
    "env!",
    "option_env!",
    "asm!",
    "global_asm!",
    "extern crate",
    "compile_error!",
    "std::fs",
    "fs::",
    "std::net",
    "net::",
    "std::thread",
    "thread::",
    "panic!",
    "todo!",
    "unimplemented!",
    "loop ",
    "while ",
    "for ",
    "fn ",
    "mod ",
)


class BatchError(RuntimeError):
    """A fail-closed equation-batch validation or execution error."""


@dataclasses.dataclass(frozen=True)
class WorkspacePackage:
    package: str
    crate_name: str
    member_path: str
    manifest_path: Path


@dataclasses.dataclass(frozen=True)
class BatchRow:
    row_number: int
    schema_version: str
    batch_id: str
    formula_id: str
    package: str
    crate_name: str
    runtime_symbol: str
    output_variable: str
    contract_path: str
    validation_card_path: str
    source_seed_path: str
    validation_status: str
    test_strategy: str
    test_expression: str
    inventory_source_path: str
    inventory_line: int

    @property
    def runtime_path(self) -> str:
        return f"{self.crate_name}::{self.runtime_symbol}"


@dataclasses.dataclass(frozen=True)
class Batch:
    repo: Path
    manifest: Path
    manifest_relative: str
    batch_id: str
    rows: tuple[BatchRow, ...]
    packages: tuple[WorkspacePackage, ...]
    input_hashes: tuple[tuple[str, str], ...]
    source_commit: str | None


@dataclasses.dataclass(frozen=True)
class GitSnapshot:
    available: bool
    head: str | None
    status_porcelain: str | None


def stable_json(data: Any) -> str:
    return json.dumps(data, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise BatchError(message)


def is_relative_to(path: Path, parent: Path) -> bool:
    try:
        path.relative_to(parent)
        return True
    except ValueError:
        return False


def normalize_repo_relative(raw: str, field: str) -> str:
    require(raw == raw.strip(), f"{field} must not contain leading or trailing whitespace")
    require(raw != "", f"{field} must not be empty")
    require("\\" not in raw, f"{field} must use forward slashes: {raw!r}")
    candidate = Path(raw)
    require(not candidate.is_absolute(), f"{field} must be repository-relative: {raw!r}")
    require(".." not in candidate.parts, f"{field} must not contain '..': {raw!r}")
    normalized = candidate.as_posix()
    require(normalized == raw, f"{field} must be normalized: expected {normalized!r}, got {raw!r}")
    return normalized


def resolve_repo_path(repo: Path, raw: str, field: str, *, must_exist: bool = True) -> Path:
    normalized = normalize_repo_relative(raw, field)
    resolved = (repo / normalized).resolve()
    require(is_relative_to(resolved, repo), f"{field} escapes repository root: {raw!r}")
    if must_exist:
        require(resolved.is_file(), f"{field} does not name a repository file: {raw!r}")
    return resolved


def validate_identifier(value: str, field: str) -> None:
    require(
        re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", value) is not None,
        f"{field} must be a Rust identifier: {value!r}",
    )


def validate_package_name(value: str) -> None:
    require(
        re.fullmatch(r"[a-z0-9][a-z0-9-]*", value) is not None,
        f"package must be a lowercase Cargo package name: {value!r}",
    )


def validate_formula_id(value: str) -> None:
    require(value.startswith("formula_vault."), f"formula_id must start with 'formula_vault.': {value!r}")
    require(
        re.fullmatch(r"[a-z0-9_]+(?:\.[a-z0-9_]+)+", value) is not None,
        f"formula_id must be a lowercase dotted identifier: {value!r}",
    )


def validate_batch_id(value: str) -> None:
    require(
        re.fullmatch(r"[a-z0-9][a-z0-9._-]{0,95}", value) is not None,
        f"batch_id must be a bounded lowercase identifier: {value!r}",
    )


def validate_balanced_delimiters(expression: str) -> None:
    pairs = {")": "(", "]": "[", "}": "{"}
    stack: list[str] = []
    for character in expression:
        if character in "([{":
            stack.append(character)
        elif character in pairs:
            require(stack and stack[-1] == pairs[character], f"test_expression has unbalanced delimiters: {expression!r}")
            stack.pop()
    require(not stack, f"test_expression has unbalanced delimiters: {expression!r}")


def validate_test_expression(expression: str, runtime_path: str) -> None:
    require(expression == expression.strip(), "test_expression must not have leading or trailing whitespace")
    require(1 <= len(expression) <= 2048, "test_expression must contain 1..2048 characters")
    require(not any(character in expression for character in "\r\n\t;\"'`\\#"), "test_expression contains a forbidden control or quoting character")
    require(
        re.fullmatch(r"[A-Za-z0-9_:.!,()\[\]{}+\-*/<>=?&| ]+", expression) is not None,
        f"test_expression contains unsupported characters: {expression!r}",
    )
    lowered = expression.lower()
    for token in FORBIDDEN_EXPRESSION_TOKENS:
        require(token not in lowered, f"test_expression contains forbidden token {token!r}")
    require(runtime_path in expression, f"test_expression must reference its exact runtime path {runtime_path!r}")
    validate_balanced_delimiters(expression)


def yaml_has_research_required(text: str) -> bool:
    return re.search(r"(?m)^\s*status:\s*research_required\s*$", text) is not None


def read_tsv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open("r", encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream, delimiter="\t")
        fields = list(reader.fieldnames or [])
        rows: list[dict[str, str]] = []
        for row in reader:
            if row is None:
                continue
            normalized = {str(key): (value if value is not None else "") for key, value in row.items()}
            if not any(normalized.values()):
                continue
            rows.append(normalized)
    return fields, rows


def workspace_packages(repo: Path) -> dict[str, WorkspacePackage]:
    root_manifest = repo / "Cargo.toml"
    require(root_manifest.is_file(), f"missing workspace Cargo.toml: {root_manifest}")
    data = tomllib.loads(root_manifest.read_text(encoding="utf-8"))
    members = data.get("workspace", {}).get("members")
    require(isinstance(members, list) and members, "workspace Cargo.toml has no workspace.members list")
    packages: dict[str, WorkspacePackage] = {}
    for member in members:
        require(isinstance(member, str), f"workspace member is not a string: {member!r}")
        member_path = normalize_repo_relative(member, "workspace member")
        manifest = repo / member_path / "Cargo.toml"
        require(manifest.is_file(), f"workspace member manifest missing: {member_path}/Cargo.toml")
        member_data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        package_name = member_data.get("package", {}).get("name")
        require(isinstance(package_name, str) and package_name, f"workspace member {member_path} has no package.name")
        lib_data = member_data.get("lib")
        if not isinstance(lib_data, dict):
            continue
        crate_name = lib_data.get("name", package_name.replace("-", "_"))
        require(isinstance(crate_name, str) and crate_name, f"workspace package {package_name} has no library name")
        require(package_name not in packages, f"duplicate workspace package name: {package_name}")
        packages[package_name] = WorkspacePackage(
            package=package_name,
            crate_name=crate_name,
            member_path=member_path,
            manifest_path=manifest,
        )
    return packages


def inventory_rows(repo: Path) -> list[dict[str, str]]:
    inventory = repo / INVENTORY_PATH
    require(inventory.is_file(), f"missing equation inventory: {INVENTORY_PATH}")
    fields, rows = read_tsv(inventory)
    required = [
        "category",
        "id",
        "source_path",
        "line",
        "function_or_ref",
        "status",
        "blocked",
        "block_reason",
        "row_count",
    ]
    require(fields == required, f"equation inventory header mismatch: expected {required!r}, got {fields!r}")
    return rows


def git_snapshot(repo: Path) -> GitSnapshot:
    try:
        root = subprocess.run(
            ["git", "-C", str(repo), "rev-parse", "--show-toplevel"],
            check=False,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=20,
        )
        if root.returncode != 0:
            return GitSnapshot(False, None, None)
        head = subprocess.run(
            ["git", "-C", str(repo), "rev-parse", "HEAD"],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=20,
        ).stdout.strip()
        status = subprocess.run(
            ["git", "-C", str(repo), "status", "--porcelain=v1", "--untracked-files=all"],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=20,
        ).stdout
        return GitSnapshot(True, head, status)
    except (OSError, subprocess.SubprocessError):
        return GitSnapshot(False, None, None)


def manifest_path_from_args(repo: Path, raw: str) -> tuple[Path, str]:
    candidate = Path(raw)
    resolved = candidate.resolve() if candidate.is_absolute() else (repo / candidate).resolve()
    require(is_relative_to(resolved, repo), f"manifest must be inside the repository: {resolved}")
    require(resolved.is_file(), f"manifest does not exist: {resolved}")
    relative = resolved.relative_to(repo).as_posix()
    return resolved, relative


def load_batch(repo_raw: str | Path, manifest_raw: str | Path) -> Batch:
    repo = Path(repo_raw).resolve()
    require(repo.is_dir(), f"repository does not exist: {repo}")
    manifest, manifest_relative = manifest_path_from_args(repo, str(manifest_raw))
    fields, raw_rows = read_tsv(manifest)
    require(fields == MANIFEST_FIELDS, f"batch manifest header mismatch: expected {MANIFEST_FIELDS!r}, got {fields!r}")
    require(1 <= len(raw_rows) <= MAX_BATCH_ROWS, f"batch must contain 1..{MAX_BATCH_ROWS} rows, got {len(raw_rows)}")

    package_map = workspace_packages(repo)
    inventory = inventory_rows(repo)
    rows: list[BatchRow] = []
    seen_formula_ids: set[str] = set()
    seen_runtime_paths: set[str] = set()
    batch_id: str | None = None
    input_paths: set[str] = {manifest_relative, INVENTORY_PATH, "Cargo.toml"}
    used_packages: dict[str, WorkspacePackage] = {}

    for index, raw in enumerate(raw_rows, start=2):
        extras = set(raw) - set(MANIFEST_FIELDS)
        require(not extras, f"manifest row {index} contains unexpected columns: {sorted(extras)}")
        values = {field: raw.get(field, "") for field in MANIFEST_FIELDS}
        for field, value in values.items():
            require(value == value.strip(), f"manifest row {index} field {field} has leading or trailing whitespace")
            require(value != "", f"manifest row {index} field {field} is empty")
        require(values["schema_version"] == SCHEMA_VERSION, f"manifest row {index} schema_version must be {SCHEMA_VERSION}")
        validate_batch_id(values["batch_id"])
        if batch_id is None:
            batch_id = values["batch_id"]
        require(values["batch_id"] == batch_id, f"manifest row {index} has a different batch_id")
        validate_formula_id(values["formula_id"])
        validate_package_name(values["package"])
        validate_identifier(values["crate_name"], "crate_name")
        validate_identifier(values["runtime_symbol"], "runtime_symbol")
        validate_identifier(values["output_variable"], "output_variable")
        require(values["validation_status"] == REQUIRED_VALIDATION_STATUS, f"manifest row {index} validation_status must remain research_required")
        require(values["test_strategy"] in ALLOWED_TEST_STRATEGIES, f"manifest row {index} has unsupported test_strategy {values['test_strategy']!r}")

        package = package_map.get(values["package"])
        require(package is not None, f"manifest row {index} package is not a workspace library: {values['package']!r}")
        require(package.crate_name == values["crate_name"], f"manifest row {index} crate_name mismatch: workspace has {package.crate_name!r}")
        used_packages[package.package] = package
        input_paths.add(f"{package.member_path}/Cargo.toml")

        contract_path = normalize_repo_relative(values["contract_path"], "contract_path")
        validation_card_path = normalize_repo_relative(values["validation_card_path"], "validation_card_path")
        source_seed_path = normalize_repo_relative(values["source_seed_path"], "source_seed_path")
        contract = resolve_repo_path(repo, contract_path, "contract_path")
        validation_card = resolve_repo_path(repo, validation_card_path, "validation_card_path")
        source_seed = resolve_repo_path(repo, source_seed_path, "source_seed_path")
        input_paths.update((contract_path, validation_card_path, source_seed_path))

        contract_text = contract.read_text(encoding="utf-8")
        require(values["formula_id"] in contract_text, f"manifest row {index} formula_id is absent from its contract")
        require(values["runtime_symbol"] in contract_text, f"manifest row {index} runtime_symbol is absent from its contract")
        require("research_required" in contract_text, f"manifest row {index} contract does not retain research_required")
        require(yaml_has_research_required(validation_card.read_text(encoding="utf-8")), f"manifest row {index} validation card is not research_required")
        require(yaml_has_research_required(source_seed.read_text(encoding="utf-8")), f"manifest row {index} source seed is not research_required")

        package_source_prefix = package.member_path + "/"
        matches = [
            entry
            for entry in inventory
            if entry.get("function_or_ref") == values["runtime_symbol"]
            and entry.get("source_path", "").startswith(package_source_prefix)
        ]
        require(
            len(matches) == 1,
            f"manifest row {index} runtime_symbol must have exactly one inventory row inside package {package.package!r}, got {len(matches)}",
        )
        inventory_row = matches[0]
        require(inventory_row.get("category") == "executable_research_equation", f"manifest row {index} inventory category is not executable_research_equation")
        require(inventory_row.get("status") == REQUIRED_VALIDATION_STATUS, f"manifest row {index} inventory status is not research_required")
        inventory_source_path = normalize_repo_relative(inventory_row.get("source_path", ""), "inventory source_path")
        resolve_repo_path(repo, inventory_source_path, "inventory source_path")
        try:
            inventory_line = int(inventory_row.get("line", ""))
        except ValueError as error:
            raise BatchError(f"manifest row {index} inventory line is not an integer") from error
        require(inventory_line > 0, f"manifest row {index} inventory line must be positive")
        input_paths.add(INVENTORY_PATH)

        runtime_path = f"{values['crate_name']}::{values['runtime_symbol']}"
        validate_test_expression(values["test_expression"], runtime_path)
        require(values["formula_id"] not in seen_formula_ids, f"duplicate formula_id in batch: {values['formula_id']}")
        require(runtime_path not in seen_runtime_paths, f"duplicate runtime path in batch: {runtime_path}")
        seen_formula_ids.add(values["formula_id"])
        seen_runtime_paths.add(runtime_path)

        rows.append(
            BatchRow(
                row_number=index,
                schema_version=values["schema_version"],
                batch_id=values["batch_id"],
                formula_id=values["formula_id"],
                package=values["package"],
                crate_name=values["crate_name"],
                runtime_symbol=values["runtime_symbol"],
                output_variable=values["output_variable"],
                contract_path=contract_path,
                validation_card_path=validation_card_path,
                source_seed_path=source_seed_path,
                validation_status=values["validation_status"],
                test_strategy=values["test_strategy"],
                test_expression=values["test_expression"],
                inventory_source_path=inventory_source_path,
                inventory_line=inventory_line,
            )
        )

    require(batch_id is not None, "batch_id was not resolved")
    snapshot = git_snapshot(repo)
    input_hashes = tuple((path, sha256_file(repo / path)) for path in sorted(input_paths))
    return Batch(
        repo=repo,
        manifest=manifest,
        manifest_relative=manifest_relative,
        batch_id=batch_id,
        rows=tuple(rows),
        packages=tuple(used_packages[name] for name in sorted(used_packages)),
        input_hashes=input_hashes,
        source_commit=snapshot.head,
    )


def row_summary(row: BatchRow, *, include_expression: bool) -> dict[str, Any]:
    data: dict[str, Any] = {
        "contract_path": row.contract_path,
        "crate_name": row.crate_name,
        "formula_id": row.formula_id,
        "inventory_line": row.inventory_line,
        "inventory_source_path": row.inventory_source_path,
        "output_variable": row.output_variable,
        "package": row.package,
        "runtime_path": row.runtime_path,
        "runtime_symbol": row.runtime_symbol,
        "source_seed_path": row.source_seed_path,
        "test_strategy": row.test_strategy,
        "validation_card_path": row.validation_card_path,
        "validation_status": row.validation_status,
    }
    if include_expression:
        data["test_expression"] = row.test_expression
    return data


def plan_document(batch: Batch) -> dict[str, Any]:
    return {
        "batch_id": batch.batch_id,
        "compiler_verification_required": True,
        "formula_rows": len(batch.rows),
        "input_manifest": batch.manifest_relative,
        "limits": {
            "maximum_rows": MAX_BATCH_ROWS,
            "no_rust_source_scraping": True,
            "runtime_symbols_verified_by_rust_compiler": True,
            "validation_status": REQUIRED_VALIDATION_STATUS,
        },
        "packages": [
            {
                "crate_name": package.crate_name,
                "member_path": package.member_path,
                "package": package.package,
            }
            for package in batch.packages
        ],
        "result": "PASS",
        "rows": [row_summary(row, include_expression=False) for row in batch.rows],
        "schema_version": SCHEMA_VERSION,
        "source_commit": batch.source_commit,
    }


def toml_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def generated_cargo_toml(batch: Batch) -> str:
    lines = [
        "[package]",
        'name = "aerocodex-equation-batch-probe"',
        'version = "0.0.0"',
        'edition = "2021"',
        "publish = false",
        "",
        "[workspace]",
        "",
        "[dependencies]",
    ]
    for package in batch.packages:
        package_dir = (batch.repo / package.member_path).resolve()
        lines.append(f"{package.package} = {{ path = {toml_string(str(package_dir))} }}")
    lines.append("")
    return "\n".join(lines)


def test_function_name(index: int, formula_id: str) -> str:
    suffix = re.sub(r"[^A-Za-z0-9_]", "_", formula_id)
    suffix = re.sub(r"_+", "_", suffix).strip("_")
    return f"row_{index:03d}_{suffix}"[:180]


def generated_rust_source(batch: Batch) -> str:
    lines = [
        "#![forbid(unsafe_code)]",
        "",
        "#[cfg(test)]",
        "mod generated_equation_batch {",
    ]
    for index, row in enumerate(batch.rows, start=1):
        lines.extend(
            [
                "    #[test]",
                f"    fn {test_function_name(index, row.formula_id)}() {{",
                f"        let _runtime_symbol = {row.runtime_path};",
                "        assert!(",
                f"            {row.test_expression},",
                f"            {json.dumps('equation batch contract probe failed: ' + row.formula_id)}",
                "        );",
                "    }",
                "",
            ]
        )
    lines.append("}")
    lines.append("")
    return "\n".join(lines)


def catalog_document(batch: Batch) -> dict[str, Any]:
    return {
        "batch_id": batch.batch_id,
        "formula_rows": len(batch.rows),
        "generated_test_count": len(batch.rows),
        "result": "PASS",
        "rows": [row_summary(row, include_expression=True) for row in batch.rows],
        "schema_version": SCHEMA_VERSION,
        "source_commit": batch.source_commit,
    }


def inputs_document(batch: Batch) -> dict[str, Any]:
    return {
        "batch_id": batch.batch_id,
        "inputs": [{"path": path, "sha256": digest} for path, digest in batch.input_hashes],
        "result": "PASS",
        "schema_version": SCHEMA_VERSION,
        "source_commit": batch.source_commit,
    }


def generation_files_without_report(batch: Batch) -> dict[str, bytes]:
    return {
        "Cargo.toml": generated_cargo_toml(batch).encode("utf-8"),
        "equation-batch-catalog.json": stable_json(catalog_document(batch)).encode("utf-8"),
        "equation-batch-inputs.json": stable_json(inputs_document(batch)).encode("utf-8"),
        "input/equation-batch.tsv": batch.manifest.read_bytes(),
        "src/lib.rs": generated_rust_source(batch).encode("utf-8"),
    }


def generation_report_document(batch: Batch, files: dict[str, bytes]) -> dict[str, Any]:
    return {
        "artifact_hashes": [
            {"path": path, "sha256": sha256_bytes(data)} for path, data in sorted(files.items())
        ],
        "batch_id": batch.batch_id,
        "compiler_probe_generated": True,
        "formula_rows": len(batch.rows),
        "generated_test_count": len(batch.rows),
        "input_manifest": batch.manifest_relative,
        "no_rust_source_scraping": True,
        "result": "PASS",
        "runtime_symbol_probe_count": len(batch.rows),
        "schema_version": SCHEMA_VERSION,
        "source_commit": batch.source_commit,
        "validation_status": REQUIRED_VALIDATION_STATUS,
    }


def generation_files(batch: Batch) -> dict[str, bytes]:
    files = generation_files_without_report(batch)
    files["equation-batch-generation-report.json"] = stable_json(
        generation_report_document(batch, files)
    ).encode("utf-8")
    manifest_lines = [f"{sha256_bytes(data)}  {path}\n" for path, data in sorted(files.items())]
    files["equation-batch-artifacts.sha256"] = "".join(manifest_lines).encode("utf-8")
    return files


def ensure_output_outside_repo(repo: Path, output_dir: Path) -> None:
    resolved = output_dir.resolve()
    require(not is_relative_to(resolved, repo), f"output directory must be outside the Git repository: {resolved}")


def write_files_atomic(root: Path, files: dict[str, bytes]) -> None:
    root.mkdir(parents=True, exist_ok=False)
    for relative, data in sorted(files.items()):
        target = root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        temporary = target.with_name(target.name + ".tmp")
        temporary.write_bytes(data)
        os.replace(temporary, target)


def parse_artifact_manifest(path: Path) -> list[tuple[str, str]]:
    entries: list[tuple[str, str]] = []
    for line_number, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        if not raw.strip():
            continue
        parts = raw.split(None, 1)
        require(len(parts) == 2, f"malformed artifact manifest line {line_number}")
        digest, relative = parts
        require(re.fullmatch(r"[0-9a-f]{64}", digest) is not None, f"invalid artifact digest on line {line_number}")
        relative = normalize_repo_relative(relative.strip(), "artifact manifest path")
        entries.append((digest, relative))
    require(entries, "artifact manifest is empty")
    return entries


def verify_generated_artifacts(batch: Batch, output_dir: Path) -> dict[str, Any]:
    expected = generation_files(batch)
    allowed = set(expected) | {VERIFICATION_REPORT, VERIFICATION_HASH}
    actual_files = {
        path.relative_to(output_dir).as_posix()
        for path in output_dir.rglob("*")
        if path.is_file()
    }
    unexpected = sorted(actual_files - allowed)
    missing = sorted(set(expected) - actual_files)
    require(not unexpected, f"generated output contains unexpected files: {unexpected}")
    require(not missing, f"generated output is missing files: {missing}")
    mismatches: list[str] = []
    for relative, expected_data in sorted(expected.items()):
        actual = (output_dir / relative).read_bytes()
        if actual != expected_data:
            mismatches.append(relative)
    require(not mismatches, f"generated output differs from deterministic regeneration: {mismatches}")
    manifest_entries = parse_artifact_manifest(output_dir / "equation-batch-artifacts.sha256")
    manifest_paths = [relative for _, relative in manifest_entries]
    require(manifest_paths == sorted(set(expected) - {"equation-batch-artifacts.sha256"}), "artifact manifest path set or ordering is invalid")
    for digest, relative in manifest_entries:
        actual_digest = sha256_file(output_dir / relative)
        require(actual_digest == digest, f"artifact manifest hash mismatch for {relative}")
    return {
        "artifact_count": len(expected),
        "deterministic_regeneration": "PASS",
        "manifest_entry_count": len(manifest_entries),
        "result": "PASS",
    }


def tail(text: str, limit: int = 12000) -> str:
    return text if len(text) <= limit else text[-limit:]


def write_verification_report(output_dir: Path, report: dict[str, Any]) -> None:
    data = stable_json(report).encode("utf-8")
    report_path = output_dir / VERIFICATION_REPORT
    temporary = report_path.with_suffix(report_path.suffix + ".tmp")
    temporary.write_bytes(data)
    os.replace(temporary, report_path)
    hash_text = f"{sha256_bytes(data)}  {VERIFICATION_REPORT}\n"
    hash_path = output_dir / VERIFICATION_HASH
    temporary_hash = hash_path.with_suffix(hash_path.suffix + ".tmp")
    temporary_hash.write_text(hash_text, encoding="utf-8", newline="\n")
    os.replace(temporary_hash, hash_path)


def run_compiler_probe(batch: Batch, output_dir: Path, timeout_seconds: int) -> dict[str, Any]:
    cargo = shutil.which("cargo")
    require(cargo is not None, "cargo was not found on PATH")
    with tempfile.TemporaryDirectory(prefix="aerocodex-equation-batch-") as temporary_raw:
        temporary = Path(temporary_raw)
        shutil.copy2(output_dir / "Cargo.toml", temporary / "Cargo.toml")
        (temporary / "src").mkdir()
        shutil.copy2(output_dir / "src/lib.rs", temporary / "src/lib.rs")
        command = [
            cargo,
            "test",
            "--manifest-path",
            str(temporary / "Cargo.toml"),
            "--offline",
            "--all-targets",
        ]
        environment = os.environ.copy()
        environment["CARGO_TARGET_DIR"] = str(temporary / "target")
        environment["CARGO_NET_OFFLINE"] = "true"
        started = time.monotonic()
        try:
            completed = subprocess.run(
                command,
                cwd=temporary,
                env=environment,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=timeout_seconds,
                check=False,
            )
        except subprocess.TimeoutExpired as error:
            return {
                "command": command,
                "duration_seconds": round(time.monotonic() - started, 3),
                "exit_code": None,
                "result": "FAIL",
                "stderr_tail": tail((error.stderr or "") if isinstance(error.stderr, str) else ""),
                "stdout_tail": tail((error.stdout or "") if isinstance(error.stdout, str) else ""),
                "timed_out": True,
            }
        return {
            "command": command,
            "duration_seconds": round(time.monotonic() - started, 3),
            "exit_code": completed.returncode,
            "result": "PASS" if completed.returncode == 0 else "FAIL",
            "stderr_tail": tail(completed.stderr),
            "stdout_tail": tail(completed.stdout),
            "timed_out": False,
        }


def command_plan(args: argparse.Namespace) -> int:
    batch = load_batch(args.repo, args.manifest)
    print(stable_json(plan_document(batch)), end="")
    return 0


def command_generate(args: argparse.Namespace) -> int:
    batch = load_batch(args.repo, args.manifest)
    output_dir = Path(args.output_dir).resolve()
    ensure_output_outside_repo(batch.repo, output_dir)
    require(not output_dir.exists(), f"output directory already exists: {output_dir}")
    before = git_snapshot(batch.repo)
    files = generation_files(batch)
    write_files_atomic(output_dir, files)
    after = git_snapshot(batch.repo)
    require(before == after, "source repository changed while generating equation batch")
    report = generation_report_document(batch, generation_files_without_report(batch))
    result = {
        "artifact_manifest": str(output_dir / "equation-batch-artifacts.sha256"),
        "batch_id": batch.batch_id,
        "formula_rows": len(batch.rows),
        "generation_report": str(output_dir / "equation-batch-generation-report.json"),
        "output_directory": str(output_dir),
        "repository_unchanged": True,
        "result": "PASS",
        "runtime_symbol_probe_count": report["runtime_symbol_probe_count"],
    }
    print(stable_json(result), end="")
    return 0


def command_verify(args: argparse.Namespace) -> int:
    batch = load_batch(args.repo, args.manifest)
    output_dir = Path(args.output_dir).resolve()
    ensure_output_outside_repo(batch.repo, output_dir)
    require(output_dir.is_dir(), f"generated output directory does not exist: {output_dir}")
    before = git_snapshot(batch.repo)
    artifact_result: dict[str, Any] | None = None
    compiler_result: dict[str, Any] | None = None
    error: str | None = None
    try:
        artifact_result = verify_generated_artifacts(batch, output_dir)
        compiler_result = run_compiler_probe(batch, output_dir, args.timeout_seconds)
        require(compiler_result["result"] == "PASS", "generated Rust compiler probe failed")
    except BatchError as failure:
        error = str(failure)
    after = git_snapshot(batch.repo)
    repository_unchanged = before == after
    if not repository_unchanged and error is None:
        error = "source repository changed while verifying equation batch"
    result = "PASS" if error is None else "FAIL"
    report = {
        "artifact_verification": artifact_result,
        "batch_id": batch.batch_id,
        "compiler_probe": compiler_result,
        "error": error,
        "formula_rows": len(batch.rows),
        "generated_test_count": len(batch.rows),
        "input_manifest": batch.manifest_relative,
        "no_rust_source_scraping": True,
        "repository_unchanged": repository_unchanged,
        "result": result,
        "runtime_symbols_compiler_verified": len(batch.rows) if result == "PASS" else 0,
        "schema_version": SCHEMA_VERSION,
        "source_commit": batch.source_commit,
        "validation_status": REQUIRED_VALIDATION_STATUS,
    }
    write_verification_report(output_dir, report)
    print(stable_json(report), end="")
    return 0 if result == "PASS" else 1


def write_fake_repo(root: Path) -> Path:
    repo = root / "repo"
    (repo / "crates/example/src").mkdir(parents=True)
    (repo / "crates/other/src").mkdir(parents=True)
    (repo / "formula-vault/contracts").mkdir(parents=True)
    (repo / "validation/cards").mkdir(parents=True)
    (repo / "validation/source_registry").mkdir(parents=True)
    (repo / "validation").mkdir(exist_ok=True)
    (repo / "equation-batches").mkdir()
    (repo / "Cargo.toml").write_text('[workspace]\nresolver = "2"\nmembers = ["crates/example", "crates/other"]\n', encoding="utf-8")
    (repo / "crates/example/Cargo.toml").write_text(
        '[package]\nname = "example-equations"\nversion = "0.0.0"\nedition = "2021"\n\n[lib]\nname = "example_equations"\npath = "src/lib.rs"\n',
        encoding="utf-8",
    )
    (repo / "crates/example/src/lib.rs").write_text(
        "#![forbid(unsafe_code)]\npub fn add_one(value: f64) -> Result<f64, ()> { Ok(value + 1.0) }\n",
        encoding="utf-8",
    )
    (repo / "crates/other/Cargo.toml").write_text(
        '[package]\nname = "other-equations"\nversion = "0.0.0"\nedition = "2021"\n\n[lib]\nname = "other_equations"\npath = "src/lib.rs"\n',
        encoding="utf-8",
    )
    (repo / "crates/other/src/lib.rs").write_text(
        "#![forbid(unsafe_code)]\npub fn add_one(value: f64) -> Result<f64, ()> { Ok(value + 2.0) }\n",
        encoding="utf-8",
    )
    formula_id = "formula_vault.example.add_one"
    contract = repo / "formula-vault/contracts/example.yaml"
    contract.write_text(
        f"record_status: research_required\nformula_id: {formula_id}\nruntime_symbol: add_one\nvalidation_status: research_required\n",
        encoding="utf-8",
    )
    (repo / "validation/cards/example.yaml").write_text("id: example\nstatus: research_required\n", encoding="utf-8")
    (repo / "validation/source_registry/example.yaml").write_text("id: source.example\nstatus: research_required\n", encoding="utf-8")
    inventory_header = "category\tid\tsource_path\tline\tfunction_or_ref\tstatus\tblocked\tblock_reason\trow_count\n"
    inventory_rows_text = (
        "executable_research_equation\texecutable.example.add_one\tcrates/example/src/lib.rs\t2\tadd_one\tresearch_required\ttrue\tresearch_only\t1\n"
        "executable_research_equation\texecutable.other.add_one\tcrates/other/src/lib.rs\t2\tadd_one\tresearch_required\ttrue\tresearch_only\t1\n"
    )
    (repo / INVENTORY_PATH).write_text(inventory_header + inventory_rows_text, encoding="utf-8")
    manifest_values = [
        SCHEMA_VERSION,
        "example-batch",
        formula_id,
        "example-equations",
        "example_equations",
        "add_one",
        "result",
        "formula-vault/contracts/example.yaml",
        "validation/cards/example.yaml",
        "validation/source_registry/example.yaml",
        "research_required",
        "exact",
        "matches!(example_equations::add_one(1.0), Ok(value) if value == 2.0)",
    ]
    (repo / "equation-batches/example.tsv").write_text(
        "\t".join(MANIFEST_FIELDS) + "\n" + "\t".join(manifest_values) + "\n",
        encoding="utf-8",
    )
    return repo


def command_self_test(_: argparse.Namespace) -> int:
    tests: list[dict[str, str]] = []
    require(sha256_bytes(b"abc") == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad", "SHA-256 implementation mismatch")
    tests.append({"name": "sha256_known_vector", "result": "PASS"})
    with tempfile.TemporaryDirectory(prefix="aerocodex-equation-batch-self-test-") as root_raw:
        root = Path(root_raw)
        repo = write_fake_repo(root)
        manifest = repo / "equation-batches/example.tsv"
        batch = load_batch(repo, manifest)
        require(len(batch.rows) == 1, "self-test batch row count mismatch")
        require(batch.rows[0].inventory_source_path == "crates/example/src/lib.rs", "package-scoped inventory resolution failed")
        tests.append({"name": "valid_manifest", "result": "PASS"})
        tests.append({"name": "package_scoped_inventory_resolution", "result": "PASS"})
        files = generation_files(batch)
        output = root / "generated"
        ensure_output_outside_repo(repo, output)
        write_files_atomic(output, files)
        verification = verify_generated_artifacts(batch, output)
        require(verification["result"] == "PASS", "self-test generated artifact verification failed")
        tests.append({"name": "deterministic_generation", "result": "PASS"})
        generated_source = (output / "src/lib.rs").read_text(encoding="utf-8")
        require("let _runtime_symbol = example_equations::add_one;" in generated_source, "self-test runtime symbol probe missing")
        require("matches!(example_equations::add_one" in generated_source, "self-test contract probe missing")
        tests.append({"name": "compiler_probe_source", "result": "PASS"})

        duplicate = manifest.read_text(encoding="utf-8") + manifest.read_text(encoding="utf-8").splitlines()[1] + "\n"
        duplicate_path = repo / "equation-batches/duplicate.tsv"
        duplicate_path.write_text(duplicate, encoding="utf-8")
        try:
            load_batch(repo, duplicate_path)
        except BatchError:
            tests.append({"name": "duplicate_rejected", "result": "PASS"})
        else:
            raise BatchError("self-test duplicate formula was not rejected")

        bad_lines = manifest.read_text(encoding="utf-8").splitlines()
        fields = bad_lines[1].split("\t")
        fields[-1] = "std::process::Command::new(evil)"
        bad_path = repo / "equation-batches/forbidden.tsv"
        bad_path.write_text(bad_lines[0] + "\n" + "\t".join(fields) + "\n", encoding="utf-8")
        try:
            load_batch(repo, bad_path)
        except BatchError:
            tests.append({"name": "forbidden_expression_rejected", "result": "PASS"})
        else:
            raise BatchError("self-test forbidden expression was not rejected")

        try:
            ensure_output_outside_repo(repo, repo / "generated")
        except BatchError:
            tests.append({"name": "repository_output_rejected", "result": "PASS"})
        else:
            raise BatchError("self-test repository output path was not rejected")
    print(
        stable_json(
            {
                "mode": "self-test",
                "result": "PASS",
                "schema_version": SCHEMA_VERSION,
                "tests": tests,
            }
        ),
        end="",
    )
    return 0


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    subparsers = result.add_subparsers(dest="command", required=True)

    plan = subparsers.add_parser("plan", help="validate a batch manifest and print its deterministic plan")
    plan.add_argument("--repo", required=True)
    plan.add_argument("--manifest", required=True)
    plan.set_defaults(handler=command_plan)

    generate = subparsers.add_parser("generate", help="generate a compiler probe crate outside the repository")
    generate.add_argument("--repo", required=True)
    generate.add_argument("--manifest", required=True)
    generate.add_argument("--output-dir", required=True)
    generate.set_defaults(handler=command_generate)

    verify = subparsers.add_parser("verify", help="verify generated artifacts and compile/run the probe crate")
    verify.add_argument("--repo", required=True)
    verify.add_argument("--manifest", required=True)
    verify.add_argument("--output-dir", required=True)
    verify.add_argument("--timeout-seconds", type=int, default=300)
    verify.set_defaults(handler=command_verify)

    self_test = subparsers.add_parser("self-test", help="run deterministic standard-library-only tests")
    self_test.set_defaults(handler=command_self_test)
    return result


def main(argv: Sequence[str] | None = None) -> int:
    args = parser().parse_args(argv)
    if hasattr(args, "timeout_seconds"):
        require(1 <= args.timeout_seconds <= 3600, "timeout-seconds must be in 1..3600")
    return int(args.handler(args))


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BatchError as error:
        print(stable_json({"error": str(error), "result": "FAIL"}), end="", file=sys.stderr)
        raise SystemExit(1)
