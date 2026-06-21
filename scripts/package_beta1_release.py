#!/usr/bin/env python3
"""Build and package an AeroCodex Beta 1 concept release candidate.

The script builds only committed source, uses Cargo offline, runs the governed
workspace gates inside a temporary source snapshot, emits a deterministic ZIP,
and verifies the packaged binary before reporting success.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import tomllib
import zipfile
from io import BytesIO
from pathlib import Path, PurePosixPath
from typing import Any

RELEASE_CHANNEL = "beta1-concept"
PACKAGE_VERSION = "0.0.1"
VALIDATION_STATUS = "research_required"
SUPPORTED_FORMULA_COUNT = 10
SELF_CHECK_CASES = 14
SAFETY_NOTICE = (
    "research/preliminary-design software; not certified, flight-ready, mission-ready, "
    "operational, medical, habitat-safe, or approved for regulated use"
)
COMMIT_PATTERN = re.compile(r"^[0-9a-f]{40}$")


class PackagingError(RuntimeError):
    """Raised when packaging must fail closed."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise PackagingError(message)


def stable_json(data: Any) -> str:
    return json.dumps(data, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run(
    command: list[str],
    *,
    cwd: Path,
    env: dict[str, str] | None = None,
    expected_exit: int = 0,
    timeout: int = 900,
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        command,
        cwd=cwd,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=timeout,
        check=False,
    )
    require(
        completed.returncode == expected_exit,
        "command returned unexpected exit status: "
        f"expected={expected_exit} actual={completed.returncode} command={command!r} "
        f"stdout={completed.stdout!r} stderr={completed.stderr!r}",
    )
    return completed


def git(repo: Path, args: list[str]) -> str:
    return run(["git", *args], cwd=repo, timeout=120).stdout.strip()


def parse_host_target(rustc_verbose: str) -> str:
    for line in rustc_verbose.splitlines():
        if line.startswith("host: "):
            target = line.removeprefix("host: ").strip()
            require(target, "rustc reported an empty host target")
            return target
    raise PackagingError("rustc -vV did not report a host target")


def sanitize_component(value: str) -> str:
    sanitized = re.sub(r"[^A-Za-z0-9._-]+", "-", value).strip("-")
    require(bool(sanitized), f"invalid archive-name component: {value!r}")
    return sanitized


def verify_path_only_dependencies(source_root: Path) -> list[str]:
    manifests = sorted(source_root.rglob("Cargo.toml"))
    require(manifests, "no Cargo.toml files found")
    checked: list[str] = []
    for manifest_path in manifests:
        relative = manifest_path.relative_to(source_root).as_posix()
        data = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
        for section in ("dependencies", "dev-dependencies", "build-dependencies"):
            dependencies = data.get(section, {})
            require(isinstance(dependencies, dict), f"{relative} [{section}] is not a table")
            for name, value in dependencies.items():
                require(
                    isinstance(value, dict) and isinstance(value.get("path"), str),
                    f"{relative} dependency {name!r} is not path-only",
                )
                forbidden = {key for key in ("git", "registry", "version") if key in value}
                require(not forbidden, f"{relative} dependency {name!r} has non-local keys: {sorted(forbidden)}")
        checked.append(relative)
    return checked


def safe_extract_git_archive(data: bytes, destination: Path) -> None:
    with tarfile.open(fileobj=BytesIO(data), mode="r:") as archive:
        members = archive.getmembers()
        require(members, "git archive was empty")
        for member in members:
            path = PurePosixPath(member.name)
            require(not path.is_absolute(), f"git archive contains absolute path: {member.name!r}")
            require(".." not in path.parts, f"git archive contains traversal: {member.name!r}")
            require(not member.issym() and not member.islnk(), f"git archive link is forbidden: {member.name!r}")
        archive.extractall(destination, filter="fully_trusted")


def parse_json(text: str, label: str) -> dict[str, Any]:
    try:
        value = json.loads(text)
    except json.JSONDecodeError as error:
        raise PackagingError(f"{label} emitted invalid JSON: {error}: {text!r}") from error
    require(isinstance(value, dict), f"{label} JSON is not an object")
    return value


def smoke_binary(binary: Path, source_commit: str, target: str) -> list[dict[str, Any]]:
    if os.name != "nt":
        binary.chmod(binary.stat().st_mode | stat.S_IXUSR)

    results: list[dict[str, Any]] = []

    completed = run([str(binary), "version", "--json"], cwd=binary.parent, timeout=60)
    value = parse_json(completed.stdout, "version")
    expected = {
        "ok": True,
        "package_version": PACKAGE_VERSION,
        "release_channel": RELEASE_CHANNEL,
        "build_commit": source_commit,
        "build_target": target,
        "build_profile": "release",
        "supported_formula_count": SUPPORTED_FORMULA_COUNT,
        "validation_status": VALIDATION_STATUS,
    }
    for key, expected_value in expected.items():
        require(value.get(key) == expected_value, f"version field {key!r} mismatch")
    results.append({"name": "version_json", "exit_code": 0, "result": "PASS"})

    completed = run([str(binary), "formulas", "--json"], cwd=binary.parent, timeout=60)
    value = parse_json(completed.stdout, "formulas")
    require(value.get("ok") is True and value.get("count") == SUPPORTED_FORMULA_COUNT, "formula catalog mismatch")
    results.append({"name": "formula_catalog", "exit_code": 0, "result": "PASS"})

    completed = run(
        [
            str(binary),
            "run",
            "formula_vault.m00.canonical.distance_to_canonical",
            "distance=-42",
            "distance_unit=7",
            "--json",
        ],
        cwd=binary.parent,
        timeout=60,
    )
    value = parse_json(completed.stdout, "signed conversion")
    require(value.get("ok") is True and value.get("value") == -6, "signed conversion mismatch")
    require(value.get("output_variable") == "canonical_distance", "signed conversion output variable mismatch")
    results.append({"name": "signed_conversion", "exit_code": 0, "result": "PASS"})

    completed = run([str(binary), "self-check", "--json"], cwd=binary.parent, timeout=60)
    value = parse_json(completed.stdout, "self-check")
    require(value.get("ok") is True, "self-check did not report ok=true")
    require(value.get("passed") == SELF_CHECK_CASES and value.get("failed") == 0, "self-check count mismatch")
    results.append({"name": "self_check", "exit_code": 0, "result": "PASS"})

    completed = run(
        [
            str(binary),
            "run",
            "formula_vault.m00.canonical.distance_to_canonical",
            "distance=1",
            "distance_unit=0",
            "--json",
        ],
        cwd=binary.parent,
        expected_exit=4,
        timeout=60,
    )
    value = parse_json(completed.stderr, "invalid-scale error")
    require(value.get("error", {}).get("code") == "non_positive_input", "invalid-scale error code mismatch")
    results.append({"name": "invalid_scale", "exit_code": 4, "result": "PASS"})

    completed = run(
        [str(binary), "describe", "formula_vault.m00.canonical.unknown", "--json"],
        cwd=binary.parent,
        expected_exit=3,
        timeout=60,
    )
    value = parse_json(completed.stderr, "unknown-formula error")
    require(value.get("error", {}).get("code") == "unknown_formula", "unknown-formula error code mismatch")
    results.append({"name": "unknown_formula", "exit_code": 3, "result": "PASS"})
    return results


def payload_rows(bundle: Path, relative_paths: list[str]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for relative in sorted(relative_paths):
        path = bundle / Path(*PurePosixPath(relative).parts)
        require(path.is_file(), f"payload file is missing: {relative}")
        rows.append({"path": relative, "bytes": path.stat().st_size, "sha256": sha256_file(path)})
    return rows


def write_deterministic_zip(bundle: Path, archive: Path) -> None:
    require(not archive.exists(), f"archive already exists: {archive}")
    root_name = bundle.name
    with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_STORED) as handle:
        for path in sorted((item for item in bundle.rglob("*") if item.is_file()), key=lambda item: item.relative_to(bundle).as_posix()):
            relative = path.relative_to(bundle).as_posix()
            info = zipfile.ZipInfo(f"{root_name}/{relative}", date_time=(1980, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_STORED
            info.create_system = 3
            executable = relative.startswith("bin/") and path.name in {"aerocodex", "aerocodex.exe"}
            mode = 0o755 if executable else 0o644
            info.external_attr = (mode & 0xFFFF) << 16
            handle.writestr(info, path.read_bytes(), compress_type=zipfile.ZIP_STORED)


def copy_payload(source_root: Path, bundle: Path, binary: Path, binary_name: str, source_commit: str) -> list[str]:
    relative_paths = [
        "LICENSE",
        "LICENSE-APACHE",
        "LICENSE-MIT",
        "NOTICE",
        "README.md",
        "RELEASE_NOTES.md",
        "SOURCE_COMMIT.txt",
        f"bin/{binary_name}",
    ]
    (bundle / "bin").mkdir(parents=True)
    for source_name in ("LICENSE", "LICENSE-APACHE", "LICENSE-MIT", "NOTICE"):
        shutil.copyfile(source_root / source_name, bundle / source_name)
    shutil.copyfile(source_root / "docs/beta1/cli_quickstart.md", bundle / "README.md")
    shutil.copyfile(source_root / "docs/beta1/release_concept.md", bundle / "RELEASE_NOTES.md")
    (bundle / "SOURCE_COMMIT.txt").write_text(source_commit + "\n", encoding="utf-8", newline="\n")
    destination_binary = bundle / "bin" / binary_name
    shutil.copyfile(binary, destination_binary)
    if os.name != "nt":
        destination_binary.chmod(0o755)
    return relative_paths


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--target", help="optional explicit Rust target triple")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    repo = args.repo.resolve()
    output_dir = args.output_dir.resolve()
    report: dict[str, Any] = {
        "phase": "AeroCodex Beta 1 concept release packaging",
        "result": "FAIL",
        "repository": str(repo),
        "output_dir": str(output_dir),
    }
    try:
        require(repo.is_dir(), f"repository does not exist: {repo}")
        require((repo / ".git").exists(), f"repository has no .git directory: {repo}")
        require(not output_dir.exists(), f"output directory already exists: {output_dir}")
        require(repo not in output_dir.parents and output_dir != repo, "output directory must be outside the repository")

        source_commit = git(repo, ["rev-parse", "HEAD"])
        require(COMMIT_PATTERN.fullmatch(source_commit) is not None, f"invalid Git HEAD: {source_commit}")
        require(git(repo, ["status", "--porcelain=v1", "--untracked-files=all"]) == "", "repository is not clean")
        require(not (repo / "Cargo.lock").exists(), "root Cargo.lock is present contrary to current repository policy")

        dependency_manifests = verify_path_only_dependencies(repo)
        rustc_verbose = run(["rustc", "-vV"], cwd=repo, timeout=120).stdout
        host_target = parse_host_target(rustc_verbose)
        target = args.target or host_target
        target_component = sanitize_component(target)
        cargo_version = run(["cargo", "--version"], cwd=repo, timeout=120).stdout.strip()
        rustc_version = rustc_verbose.splitlines()[0].strip()

        output_dir.mkdir(parents=True)
        with tempfile.TemporaryDirectory(prefix="aerocodex-beta1-package-") as temporary_name:
            temporary = Path(temporary_name)
            source_root = temporary / "source"
            source_root.mkdir()
            archive_bytes = subprocess.run(
                ["git", "archive", "--format=tar", source_commit],
                cwd=repo,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=120,
                check=False,
            )
            require(archive_bytes.returncode == 0, f"git archive failed: {archive_bytes.stderr.decode(errors='replace')}")
            safe_extract_git_archive(archive_bytes.stdout, source_root)
            require(not (source_root / "Cargo.lock").exists(), "committed source unexpectedly contains Cargo.lock")
            verify_path_only_dependencies(source_root)

            target_dir = temporary / "cargo-target"
            build_env = dict(os.environ)
            build_env.update(
                {
                    "CARGO_TARGET_DIR": str(target_dir),
                    "AEROCODEX_BUILD_COMMIT": source_commit,
                    "AEROCODEX_BUILD_TARGET": target,
                }
            )
            commands: list[dict[str, Any]] = []
            command_specs = [
                ["cargo", "test", "--offline", "--workspace", "--all-targets", "--all-features"],
                ["cargo", "run", "--offline", "-p", "xtask", "--", "verify", "--all"],
                ["cargo", "build", "--offline", "--release", "-p", "aero-codex-cli"],
            ]
            if args.target:
                command_specs[-1].extend(["--target", target])
            for command in command_specs:
                completed = run(command, cwd=source_root, env=build_env, timeout=1800)
                commands.append(
                    {
                        "command": command,
                        "exit_code": completed.returncode,
                        "stdout_tail": completed.stdout[-4000:],
                        "stderr_tail": completed.stderr[-4000:],
                    }
                )

            binary_name = "aerocodex.exe" if "windows" in target else "aerocodex"
            release_dir = target_dir / (Path(target) if args.target else Path()) / "release"
            binary = release_dir / binary_name
            require(binary.is_file(), f"release binary was not produced: {binary}")
            smoke_checks = smoke_binary(binary, source_commit, target)

            archive_root_name = f"aerocodex-{PACKAGE_VERSION}-{RELEASE_CHANNEL}-{target_component}-{source_commit[:12]}"
            bundle = output_dir / archive_root_name
            require(not bundle.exists(), f"bundle path already exists: {bundle}")
            bundle.mkdir()
            relative_paths = copy_payload(source_root, bundle, binary, binary_name, source_commit)
            rows = payload_rows(bundle, relative_paths)
            binary_relative = f"bin/{binary_name}"
            binary_row = next(row for row in rows if row["path"] == binary_relative)
            manifest = {
                "schema_version": "1.0",
                "product": "AeroCodex",
                "release_channel": RELEASE_CHANNEL,
                "package_version": PACKAGE_VERSION,
                "source_commit": source_commit,
                "target": target,
                "host_target": host_target,
                "validation_status": VALIDATION_STATUS,
                "supported_formula_count": SUPPORTED_FORMULA_COUNT,
                "self_check_cases": SELF_CHECK_CASES,
                "dependency_model": "workspace_path_only",
                "dependency_manifest_count": len(dependency_manifests),
                "cargo_lock_committed": False,
                "build": {
                    "profile": "release",
                    "offline": True,
                    "cargo_version": cargo_version,
                    "rustc_version": rustc_version,
                },
                "binary": {"path": binary_relative, "sha256": binary_row["sha256"]},
                "smoke_checks": smoke_checks,
                "safety_notice": SAFETY_NOTICE,
                "operational_readiness_claim": False,
                "certification_claim": False,
                "full_equation_inventory_complete": False,
                "files": rows,
            }
            manifest_path = bundle / "release-manifest.json"
            manifest_path.write_text(stable_json(manifest), encoding="utf-8", newline="\n")
            checksum_paths = sorted([*relative_paths, "release-manifest.json"])
            checksum_text = "".join(
                f"{sha256_file(bundle / Path(*PurePosixPath(relative).parts))}  {relative}\n"
                for relative in checksum_paths
            )
            (bundle / "SHA256SUMS").write_text(checksum_text, encoding="utf-8", newline="\n")

            archive = output_dir / f"{archive_root_name}.zip"
            write_deterministic_zip(bundle, archive)
            verifier = source_root / "scripts/verify_beta1_release.py"
            require(verifier.is_file(), "release verifier is missing from committed source")
            verification = run(
                [sys.executable, str(verifier), "--archive", str(archive), "--run-binary"],
                cwd=source_root,
                timeout=300,
            )
            verification_report = parse_json(verification.stdout, "release verifier")
            require(verification_report.get("result") == "PASS", "release verifier did not report PASS")

            report.update(
                {
                    "result": "PASS",
                    "source_commit": source_commit,
                    "target": target,
                    "host_target": host_target,
                    "archive": str(archive),
                    "archive_sha256": sha256_file(archive),
                    "bundle_dir": str(bundle),
                    "release_manifest": str(manifest_path),
                    "binary_sha256": binary_row["sha256"],
                    "dependency_model": "workspace_path_only",
                    "dependency_manifests_verified": dependency_manifests,
                    "build_commands": commands,
                    "smoke_checks": smoke_checks,
                    "archive_verification": verification_report,
                    "repository_unchanged": True,
                }
            )

        require(git(repo, ["rev-parse", "HEAD"]) == source_commit, "repository HEAD changed during packaging")
        require(git(repo, ["status", "--porcelain=v1", "--untracked-files=all"]) == "", "repository changed during packaging")
        require(not (repo / "Cargo.lock").exists(), "packaging created a root Cargo.lock")
        report_path = output_dir / "package-report.json"
        report_path.write_text(stable_json(report), encoding="utf-8", newline="\n")
        print(stable_json(report), end="")
        return 0
    except (OSError, PackagingError, subprocess.SubprocessError, tarfile.TarError, tomllib.TOMLDecodeError) as error:
        report["error"] = str(error)
        if output_dir.exists():
            failure_path = output_dir / "package-failure-report.json"
            failure_path.write_text(stable_json(report), encoding="utf-8", newline="\n")
        print(stable_json(report), end="", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
