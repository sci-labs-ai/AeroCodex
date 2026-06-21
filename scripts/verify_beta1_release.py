#!/usr/bin/env python3
"""Verify an AeroCodex Beta 1 concept release bundle or archive.

The verifier is standard-library-only and deliberately fail-closed. It checks
bundle structure, checksums, release metadata, and optionally executes the
packaged binary's bounded smoke contract.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import stat
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path, PurePosixPath
from typing import Any

SCHEMA_VERSION = "1.0"
PRODUCT = "AeroCodex"
RELEASE_CHANNEL = "beta1-concept"
PACKAGE_VERSION = "0.0.1"
VALIDATION_STATUS = "research_required"
SUPPORTED_FORMULA_COUNT = 10
SELF_CHECK_CASES = 14
SOURCE_COMMIT_PATTERN = re.compile(r"^[0-9a-f]{40}$")
SHA256_PATTERN = re.compile(r"^[0-9a-f]{64}$")
EXPECTED_PAYLOAD_PATHS = {
    "LICENSE",
    "LICENSE-APACHE",
    "LICENSE-MIT",
    "NOTICE",
    "README.md",
    "RELEASE_NOTES.md",
    "SOURCE_COMMIT.txt",
}


class VerificationError(RuntimeError):
    """Raised when a bundle violates the release contract."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def stable_json(data: Any) -> str:
    return json.dumps(data, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def safe_relative_path(value: str) -> PurePosixPath:
    require("\\" not in value, f"bundle path uses backslashes: {value!r}")
    path = PurePosixPath(value)
    require(value == path.as_posix(), f"non-normalized bundle path: {value!r}")
    require(not path.is_absolute(), f"absolute bundle path is forbidden: {value!r}")
    require(value not in {"", "."}, "empty bundle path is forbidden")
    require(".." not in path.parts, f"parent traversal is forbidden: {value!r}")
    return path


def parse_checksum_manifest(path: Path) -> dict[str, str]:
    entries: dict[str, str] = {}
    for line_number, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.strip():
            continue
        match = re.fullmatch(r"([0-9a-f]{64})  (.+)", raw)
        require(match is not None, f"malformed checksum line {line_number}: {raw!r}")
        digest, relative = match.groups()
        safe_relative_path(relative)
        require(relative not in entries, f"duplicate checksum path: {relative}")
        entries[relative] = digest
    require(entries, "SHA256SUMS has no entries")
    require(list(entries) == sorted(entries), "SHA256SUMS paths are not sorted")
    return entries


def run_process(command: list[str], *, expected_exit: int = 0) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=60,
        check=False,
    )
    require(
        completed.returncode == expected_exit,
        "command returned unexpected exit status: "
        f"expected={expected_exit} actual={completed.returncode} command={command!r} "
        f"stdout={completed.stdout!r} stderr={completed.stderr!r}",
    )
    return completed


def parse_json_output(completed: subprocess.CompletedProcess[str], *, stream: str) -> dict[str, Any]:
    text = completed.stdout if stream == "stdout" else completed.stderr
    try:
        value = json.loads(text)
    except json.JSONDecodeError as error:
        raise VerificationError(f"invalid JSON on {stream}: {error}: {text!r}") from error
    require(isinstance(value, dict), f"JSON output on {stream} is not an object")
    return value


def verify_binary(binary: Path, manifest: dict[str, Any]) -> list[dict[str, Any]]:
    if os.name != "nt":
        binary.chmod(binary.stat().st_mode | stat.S_IXUSR)

    checks: list[dict[str, Any]] = []

    version = run_process([str(binary), "version", "--json"])
    version_json = parse_json_output(version, stream="stdout")
    require(version_json.get("ok") is True, "version JSON did not report ok=true")
    require(version_json.get("package_version") == PACKAGE_VERSION, "version package mismatch")
    require(version_json.get("release_channel") == RELEASE_CHANNEL, "version channel mismatch")
    require(
        version_json.get("supported_formula_count") == SUPPORTED_FORMULA_COUNT,
        "version supported-formula count mismatch",
    )
    require(version_json.get("validation_status") == VALIDATION_STATUS, "version validation mismatch")
    require(version_json.get("build_commit") == manifest["source_commit"], "binary build commit mismatch")
    require(version_json.get("build_target") == manifest["target"], "binary build target mismatch")
    require(version_json.get("build_profile") == "release", "binary build profile is not release")
    checks.append({"name": "version_json", "exit_code": 0, "result": "PASS"})

    formulas = run_process([str(binary), "formulas", "--json"])
    formulas_json = parse_json_output(formulas, stream="stdout")
    require(formulas_json.get("ok") is True, "formulas JSON did not report ok=true")
    require(formulas_json.get("count") == SUPPORTED_FORMULA_COUNT, "formula catalog count mismatch")
    require(formulas_json.get("validation_status") == VALIDATION_STATUS, "formula catalog validation mismatch")
    checks.append({"name": "formula_catalog", "exit_code": 0, "result": "PASS"})

    signed = run_process(
        [
            str(binary),
            "run",
            "formula_vault.m00.canonical.distance_to_canonical",
            "distance=-42",
            "distance_unit=7",
            "--json",
        ]
    )
    signed_json = parse_json_output(signed, stream="stdout")
    require(signed_json.get("ok") is True, "signed conversion JSON did not report ok=true")
    require(signed_json.get("output_variable") == "canonical_distance", "signed output variable mismatch")
    require(signed_json.get("value") == -6, "signed conversion value mismatch")
    checks.append({"name": "signed_conversion", "exit_code": 0, "result": "PASS"})

    self_check = run_process([str(binary), "self-check", "--json"])
    self_check_json = parse_json_output(self_check, stream="stdout")
    require(self_check_json.get("ok") is True, "self-check JSON did not report ok=true")
    require(self_check_json.get("passed") == SELF_CHECK_CASES, "self-check pass count mismatch")
    require(self_check_json.get("failed") == 0, "self-check reported failures")
    checks.append({"name": "self_check", "exit_code": 0, "result": "PASS"})

    invalid = run_process(
        [
            str(binary),
            "run",
            "formula_vault.m00.canonical.distance_to_canonical",
            "distance=1",
            "distance_unit=0",
            "--json",
        ],
        expected_exit=4,
    )
    invalid_json = parse_json_output(invalid, stream="stderr")
    require(invalid_json.get("ok") is False, "invalid-scale JSON did not report ok=false")
    require(invalid_json.get("error", {}).get("code") == "non_positive_input", "invalid-scale code mismatch")
    checks.append({"name": "invalid_scale", "exit_code": 4, "result": "PASS"})

    unknown = run_process(
        [str(binary), "describe", "formula_vault.m00.canonical.unknown", "--json"],
        expected_exit=3,
    )
    unknown_json = parse_json_output(unknown, stream="stderr")
    require(unknown_json.get("ok") is False, "unknown-formula JSON did not report ok=false")
    require(unknown_json.get("error", {}).get("code") == "unknown_formula", "unknown-formula code mismatch")
    checks.append({"name": "unknown_formula", "exit_code": 3, "result": "PASS"})

    return checks


def verify_bundle(bundle_dir: Path, *, run_binary: bool) -> dict[str, Any]:
    require(bundle_dir.is_dir(), f"bundle directory does not exist: {bundle_dir}")
    manifest_path = bundle_dir / "release-manifest.json"
    checksums_path = bundle_dir / "SHA256SUMS"
    require(manifest_path.is_file(), "release-manifest.json is missing")
    require(checksums_path.is_file(), "SHA256SUMS is missing")

    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as error:
        raise VerificationError(f"release-manifest.json is invalid JSON: {error}") from error
    require(isinstance(manifest, dict), "release manifest is not an object")

    required_values = {
        "schema_version": SCHEMA_VERSION,
        "product": PRODUCT,
        "release_channel": RELEASE_CHANNEL,
        "package_version": PACKAGE_VERSION,
        "validation_status": VALIDATION_STATUS,
        "supported_formula_count": SUPPORTED_FORMULA_COUNT,
        "self_check_cases": SELF_CHECK_CASES,
        "dependency_model": "workspace_path_only",
        "cargo_lock_committed": False,
    }
    for field, expected in required_values.items():
        require(manifest.get(field) == expected, f"manifest field {field!r} mismatch")
    require(SOURCE_COMMIT_PATTERN.fullmatch(str(manifest.get("source_commit", ""))) is not None, "invalid source commit")
    target = manifest.get("target")
    require(isinstance(target, str) and target.strip() == target and target, "invalid target")
    safety_notice = manifest.get("safety_notice")
    require(isinstance(safety_notice, str) and "not certified" in safety_notice, "safety notice missing conservative language")
    require(manifest.get("operational_readiness_claim") is False, "operational-readiness claim must be false")
    require(manifest.get("certification_claim") is False, "certification claim must be false")
    require(manifest.get("full_equation_inventory_complete") is False, "full inventory claim must be false")

    binary_data = manifest.get("binary")
    require(isinstance(binary_data, dict), "manifest binary section is missing")
    binary_relative = str(binary_data.get("path", ""))
    safe_relative_path(binary_relative)
    expected_binary_name = "aerocodex.exe" if "windows" in target else "aerocodex"
    require(PurePosixPath(binary_relative).name == expected_binary_name, "binary name does not match target")
    require(SHA256_PATTERN.fullmatch(str(binary_data.get("sha256", ""))) is not None, "invalid binary SHA-256")

    payload_rows = manifest.get("files")
    require(isinstance(payload_rows, list) and payload_rows, "manifest files list is missing or empty")
    payload: dict[str, dict[str, Any]] = {}
    for row in payload_rows:
        require(isinstance(row, dict), "manifest file row is not an object")
        relative = str(row.get("path", ""))
        safe_relative_path(relative)
        require(relative not in payload, f"duplicate manifest file row: {relative}")
        digest = str(row.get("sha256", ""))
        require(SHA256_PATTERN.fullmatch(digest) is not None, f"invalid manifest file SHA-256: {relative}")
        size = row.get("bytes")
        require(isinstance(size, int) and size >= 0, f"invalid manifest file size: {relative}")
        payload[relative] = row

    require(binary_relative in payload, "binary is not represented in manifest files")
    require(payload[binary_relative]["sha256"] == binary_data["sha256"], "binary SHA mismatch within manifest")
    expected_payload_paths = EXPECTED_PAYLOAD_PATHS | {binary_relative}
    require(set(payload) == expected_payload_paths, "release payload path set is not exact")

    build = manifest.get("build")
    require(isinstance(build, dict), "manifest build section is missing")
    require(build.get("profile") == "release", "manifest build profile is not release")
    require(build.get("offline") is True, "manifest offline-build marker must be true")
    require(isinstance(build.get("cargo_version"), str) and build["cargo_version"], "manifest cargo version missing")
    require(isinstance(build.get("rustc_version"), str) and build["rustc_version"], "manifest rustc version missing")
    require(isinstance(manifest.get("host_target"), str) and manifest["host_target"], "manifest host target missing")
    require(
        isinstance(manifest.get("dependency_manifest_count"), int)
        and manifest["dependency_manifest_count"] > 0,
        "manifest dependency count must be positive",
    )
    expected_smoke_checks = [
        {"name": "version_json", "exit_code": 0, "result": "PASS"},
        {"name": "formula_catalog", "exit_code": 0, "result": "PASS"},
        {"name": "signed_conversion", "exit_code": 0, "result": "PASS"},
        {"name": "self_check", "exit_code": 0, "result": "PASS"},
        {"name": "invalid_scale", "exit_code": 4, "result": "PASS"},
        {"name": "unknown_formula", "exit_code": 3, "result": "PASS"},
    ]
    require(manifest.get("smoke_checks") == expected_smoke_checks, "manifest smoke-check record mismatch")

    checksum_entries = parse_checksum_manifest(checksums_path)
    expected_checksum_paths = sorted([*payload, "release-manifest.json"])
    require(list(checksum_entries) == expected_checksum_paths, "SHA256SUMS path set does not match manifest payload")

    actual_files = sorted(
        path.relative_to(bundle_dir).as_posix()
        for path in bundle_dir.rglob("*")
        if path.is_file()
    )
    expected_files = sorted([*expected_checksum_paths, "SHA256SUMS"])
    require(actual_files == expected_files, f"bundle file inventory mismatch: actual={actual_files} expected={expected_files}")

    for relative, expected_digest in checksum_entries.items():
        path = bundle_dir / Path(*PurePosixPath(relative).parts)
        require(path.is_file(), f"checksummed file is missing: {relative}")
        actual_digest = sha256_file(path)
        require(actual_digest == expected_digest, f"checksum mismatch for {relative}")
        if relative in payload:
            require(path.stat().st_size == payload[relative]["bytes"], f"size mismatch for {relative}")
            require(actual_digest == payload[relative]["sha256"], f"manifest digest mismatch for {relative}")

    source_commit_text = (bundle_dir / "SOURCE_COMMIT.txt").read_text(encoding="utf-8")
    require(source_commit_text == manifest["source_commit"] + "\n", "SOURCE_COMMIT.txt mismatch")

    binary_path = bundle_dir / Path(*PurePosixPath(binary_relative).parts)
    require(binary_path.is_file(), "packaged binary is missing")
    binary_checks = verify_binary(binary_path, manifest) if run_binary else []
    if run_binary:
        require(binary_checks == expected_smoke_checks, "live binary smoke checks differ from manifest")

    return {
        "result": "PASS",
        "bundle_dir": str(bundle_dir.resolve()),
        "manifest": str(manifest_path.resolve()),
        "source_commit": manifest["source_commit"],
        "target": target,
        "payload_file_count": len(payload),
        "checksum_entry_count": len(checksum_entries),
        "binary_sha256": binary_data["sha256"],
        "binary_executed": run_binary,
        "binary_smoke_checks": binary_checks,
        "validation_status": VALIDATION_STATUS,
        "supported_formula_count": SUPPORTED_FORMULA_COUNT,
    }


def safe_extract_zip(archive: Path, destination: Path) -> Path:
    with zipfile.ZipFile(archive, "r") as handle:
        names = handle.namelist()
        require(names, "archive is empty")
        normalized: list[PurePosixPath] = []
        for name in names:
            require("\\" not in name, f"archive path uses backslashes: {name!r}")
            path = PurePosixPath(name)
            require(not path.is_absolute(), f"archive contains absolute path: {name!r}")
            require(".." not in path.parts, f"archive contains parent traversal: {name!r}")
            normalized.append(path)
        roots = {path.parts[0] for path in normalized if path.parts}
        require(len(roots) == 1, f"archive must contain one top-level directory: {sorted(roots)}")
        handle.extractall(destination)
        root = destination / next(iter(roots))
        require(root.is_dir(), "archive top-level directory is missing after extraction")
        return root


def verify_archive(archive: Path, *, run_binary: bool) -> dict[str, Any]:
    require(archive.is_file(), f"archive does not exist: {archive}")
    require(archive.suffix.lower() == ".zip", "only .zip archives are supported")
    with tempfile.TemporaryDirectory(prefix="aerocodex-beta1-verify-") as temporary:
        bundle = safe_extract_zip(archive, Path(temporary))
        report = verify_bundle(bundle, run_binary=run_binary)
        report["archive"] = str(archive.resolve())
        report["archive_sha256"] = sha256_file(archive)
        return report


def self_test() -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="aerocodex-beta1-verifier-self-test-") as temporary:
        root = Path(temporary) / "aerocodex-test"
        (root / "bin").mkdir(parents=True)
        binary_name = "aerocodex.exe" if os.name == "nt" else "aerocodex"
        files = {
            "LICENSE": b"test license\n",
            "LICENSE-APACHE": b"test Apache license\n",
            "LICENSE-MIT": b"test MIT license\n",
            "NOTICE": b"test notice\n",
            "README.md": b"test readme\n",
            "RELEASE_NOTES.md": b"test release notes\n",
            "SOURCE_COMMIT.txt": b"0" * 40 + b"\n",
            f"bin/{binary_name}": b"synthetic binary fixture\n",
        }
        for relative, content in files.items():
            path = root / Path(*PurePosixPath(relative).parts)
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(content)
        payload_rows = [
            {"path": relative, "bytes": len(content), "sha256": hashlib.sha256(content).hexdigest()}
            for relative, content in sorted(files.items())
        ]
        manifest = {
            "schema_version": SCHEMA_VERSION,
            "product": PRODUCT,
            "release_channel": RELEASE_CHANNEL,
            "package_version": PACKAGE_VERSION,
            "source_commit": "0" * 40,
            "target": "x86_64-pc-windows-msvc" if os.name == "nt" else "x86_64-unknown-linux-gnu",
            "host_target": "x86_64-pc-windows-msvc" if os.name == "nt" else "x86_64-unknown-linux-gnu",
            "validation_status": VALIDATION_STATUS,
            "supported_formula_count": SUPPORTED_FORMULA_COUNT,
            "self_check_cases": SELF_CHECK_CASES,
            "dependency_model": "workspace_path_only",
            "dependency_manifest_count": 1,
            "cargo_lock_committed": False,
            "build": {"profile": "release", "offline": True, "cargo_version": "cargo test", "rustc_version": "rustc test"},
            "smoke_checks": [
                {"name": "version_json", "exit_code": 0, "result": "PASS"},
                {"name": "formula_catalog", "exit_code": 0, "result": "PASS"},
                {"name": "signed_conversion", "exit_code": 0, "result": "PASS"},
                {"name": "self_check", "exit_code": 0, "result": "PASS"},
                {"name": "invalid_scale", "exit_code": 4, "result": "PASS"},
                {"name": "unknown_formula", "exit_code": 3, "result": "PASS"},
            ],
            "safety_notice": "research software; not certified",
            "operational_readiness_claim": False,
            "certification_claim": False,
            "full_equation_inventory_complete": False,
            "binary": {
                "path": f"bin/{binary_name}",
                "sha256": next(row["sha256"] for row in payload_rows if row["path"].startswith("bin/")),
            },
            "files": payload_rows,
        }
        (root / "release-manifest.json").write_text(stable_json(manifest), encoding="utf-8", newline="\n")
        checksum_paths = sorted([*files, "release-manifest.json"])
        checksum_text = "".join(f"{sha256_file(root / Path(*PurePosixPath(relative).parts))}  {relative}\n" for relative in checksum_paths)
        (root / "SHA256SUMS").write_text(checksum_text, encoding="utf-8", newline="\n")

        valid = verify_bundle(root, run_binary=False)
        tamper_path = root / "README.md"
        tamper_path.write_text("tampered\n", encoding="utf-8")
        tamper_rejected = False
        try:
            verify_bundle(root, run_binary=False)
        except VerificationError:
            tamper_rejected = True
        require(tamper_rejected, "self-test did not reject a tampered payload")
        return {
            "result": "PASS",
            "valid_fixture_result": valid["result"],
            "tampered_fixture_rejected": True,
        }


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--archive", type=Path)
    source.add_argument("--bundle-dir", type=Path)
    source.add_argument("--self-test", action="store_true")
    parser.add_argument("--run-binary", action="store_true", help="execute the packaged binary smoke contract")
    parser.add_argument("--report", type=Path, help="optional path for the JSON verification report")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    try:
        if args.self_test:
            require(not args.run_binary, "--run-binary is not valid with --self-test")
            report = self_test()
        elif args.archive is not None:
            report = verify_archive(args.archive.resolve(), run_binary=args.run_binary)
        else:
            report = verify_bundle(args.bundle_dir.resolve(), run_binary=args.run_binary)
        text = stable_json(report)
        if args.report is not None:
            report_path = args.report.resolve()
            require(not report_path.exists(), f"report path already exists: {report_path}")
            report_path.parent.mkdir(parents=True, exist_ok=True)
            report_path.write_text(text, encoding="utf-8", newline="\n")
        print(text, end="")
        return 0
    except (OSError, VerificationError, subprocess.SubprocessError, zipfile.BadZipFile) as error:
        failure = {"result": "FAIL", "error": str(error)}
        print(stable_json(failure), end="", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
