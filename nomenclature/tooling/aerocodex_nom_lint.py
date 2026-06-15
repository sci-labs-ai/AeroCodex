#!/usr/bin/env python3
"""Lightweight AeroCodex nomenclature lint scaffold.

This is intentionally conservative. It checks the seed registry files for common
ambiguities and scans Rust files for obvious risky patterns. It now also
validates the acronym/source registries and can optionally scan durable files
for unknown acronym-like tokens.

It is not a full parser and should evolve into AST-aware and document-aware
checks.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import Any

try:
    import yaml  # type: ignore
except Exception:  # pragma: no cover
    yaml = None

RISKY_SINGLE_LETTER_VALUES = {
    "n", "m", "x", "y", "z", "t", "r", "q", "p", "v", "a", "b", "c", "d", "e", "f"
}

RISKY_SHORT_VALUES = {"dt", "dx", "dy", "dz", "gs", "tas", "ias"}

RAW_IDENTIFIER_RE = re.compile(r"\br#[A-Za-z_][A-Za-z0-9_]*")
LET_IDENTIFIER_RE = re.compile(r"\blet\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\b")
CONST_GENERIC_RE = re.compile(r"const\s+([A-Z][A-Za-z0-9_]*)\s*:\s*usize")
ACRONYM_TOKEN_RE = re.compile(r"(?<![A-Za-z0-9])([A-Z][A-Z0-9&/-]{1,14})(?![A-Za-z0-9])")

DEFAULT_ALLOWED_SCAN_TOKENS = {
    # Protocol/project tokens.
    "ACX", "NOM", "INV", "AeroCodex",
    # Document/software tokens.
    "AI", "API", "AST", "CLI", "CI", "CSV", "FFI", "HTML", "HTTP", "HTTPS",
    "ICD", "ID", "JSON", "JSONL", "PR", "SQL", "URL", "UTC", "YAML",
    # Common authorities/source labels used in this starter kit.
    "AIAA", "AIM", "CCSDS", "DOD", "ECSS", "FAA", "ICAO", "NASA",
    # Seed frame/time terms.
    "GPS", "NED", "WGS84",
}


def load_yaml(path: Path) -> Any:
    if yaml is None:
        raise RuntimeError("PyYAML is required. Install with: python -m pip install pyyaml")
    with path.open("r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def error(code: str, path: Path, message: str) -> str:
    return f"ERROR {code} {path}: {message}"


def warn(code: str, path: Path, message: str) -> str:
    return f"WARN  {code} {path}: {message}"


def check_unique(records: list[dict[str, Any]], key: str, path: Path, code: str) -> list[str]:
    messages: list[str] = []
    seen: dict[str, int] = {}
    for idx, record in enumerate(records):
        value = record.get(key)
        if not value:
            messages.append(error(code, path, f"record {idx} missing required key {key!r}"))
            continue
        if value in seen:
            messages.append(error(code, path, f"duplicate {key}: {value}"))
        seen[value] = idx
    return messages


def registry_records(root: Path, name: str) -> list[dict[str, Any]]:
    path = root / "registry" / name
    if not path.exists():
        return []
    data = load_yaml(path) or {}
    return data.get("records", []) or []


def check_concepts(root: Path) -> list[str]:
    path = root / "registry" / "concepts.yaml"
    if not path.exists():
        return [error("ACX-NOM-E000", path, "missing concepts registry")]
    records = registry_records(root, "concepts.yaml")
    messages = check_unique(records, "canonical", path, "ACX-NOM-E007")
    for record in records:
        canonical = record.get("canonical", "<unknown>")
        for required in ["namespace", "display_label", "definition", "status"]:
            if not record.get(required):
                messages.append(error("ACX-NOM-E007", path, f"{canonical} missing {required}"))
    return messages


def check_aliases(root: Path) -> list[str]:
    path = root / "registry" / "aliases.yaml"
    if not path.exists():
        return [error("ACX-NOM-E000", path, "missing aliases registry")]
    records = registry_records(root, "aliases.yaml")
    messages = check_unique(records, "alias", path, "ACX-NOM-E003")

    for record in records:
        alias = record.get("alias", "<unknown>")
        status = record.get("status")
        canonical = record.get("canonical")
        candidates = record.get("candidates")
        resolution_required = record.get("resolution_required")
        if status == "ambiguous":
            if not candidates or not resolution_required:
                messages.append(error("ACX-NOM-E003", path, f"ambiguous alias {alias!r} must declare candidates and resolution_required"))
        elif not canonical:
            messages.append(error("ACX-NOM-E003", path, f"alias {alias!r} must map to canonical or be marked ambiguous"))
    return messages


def check_terminology_sources(root: Path) -> list[str]:
    path = root / "registry" / "terminology_sources.yaml"
    if not path.exists():
        return [error("ACX-NOM-E000", path, "missing terminology_sources registry")]
    records = registry_records(root, "terminology_sources.yaml")
    messages = check_unique(records, "source_id", path, "ACX-NOM-E015")
    for record in records:
        source_id = record.get("source_id", "<unknown>")
        for required in ["authority", "source_type", "title", "domains", "authority_rank", "ingestion_mode", "status"]:
            if not record.get(required):
                messages.append(error("ACX-NOM-E015", path, f"{source_id} missing {required}"))
    return messages


def check_acronyms(root: Path) -> list[str]:
    path = root / "registry" / "acronyms.yaml"
    if not path.exists():
        return [error("ACX-NOM-E000", path, "missing acronyms registry")]
    records = registry_records(root, "acronyms.yaml")
    concepts = {record.get("canonical") for record in registry_records(root, "concepts.yaml") if record.get("canonical")}
    source_ids = {record.get("source_id") for record in registry_records(root, "terminology_sources.yaml") if record.get("source_id")}
    messages = check_unique(records, "acronym_id", path, "ACX-NOM-E013")

    by_token: dict[str, list[dict[str, Any]]] = {}
    for record in records:
        acronym_id = record.get("acronym_id", "<unknown>")
        token = record.get("token")
        by_token.setdefault(str(token), []).append(record)
        for required in ["token", "expansion", "namespace", "domains", "status", "source", "first_use"]:
            if not record.get(required):
                messages.append(error("ACX-NOM-E013", path, f"{acronym_id} missing {required}"))
        if record.get("namespace") != "acronym":
            messages.append(error("ACX-NOM-E013", path, f"{acronym_id} namespace must be 'acronym'"))
        canonical = record.get("canonical")
        if canonical and canonical not in concepts:
            messages.append(warn("ACX-NOM-W007", path, f"{acronym_id} canonical {canonical!r} is not present in concepts.yaml"))
        source = record.get("source") or {}
        source_id = source.get("source_id")
        if source_id not in source_ids:
            messages.append(error("ACX-NOM-E015", path, f"{acronym_id} references missing terminology source {source_id!r}"))
        first_use = record.get("first_use") or {}
        if "requires_definition" not in first_use:
            messages.append(error("ACX-NOM-E016", path, f"{acronym_id} missing first_use.requires_definition"))
        if record.get("status") in {"approved", "candidate", "external"} and not record.get("expansion"):
            messages.append(error("ACX-NOM-E013", path, f"{acronym_id} missing expansion"))

    for token, group in by_token.items():
        if len(group) <= 1:
            continue
        collision_groups = {record.get("collision_group") for record in group}
        if None in collision_groups or "" in collision_groups:
            messages.append(error("ACX-NOM-E017", path, f"token {token!r} has multiple records but at least one lacks collision_group"))
        for record in group:
            disambiguation = record.get("disambiguation") or {}
            if not disambiguation.get("signals"):
                messages.append(error("ACX-NOM-E017", path, f"{record.get('acronym_id')} collides on {token!r} but lacks disambiguation.signals"))
        approved = [record for record in group if record.get("status") == "approved"]
        if len(approved) > 1:
            ids = [str(record.get("acronym_id")) for record in approved]
            messages.append(error("ACX-NOM-E014", path, f"token {token!r} has multiple approved meanings: {ids}"))
        else:
            ids = [str(record.get("acronym_id")) for record in group]
            messages.append(warn("ACX-NOM-W009", path, f"token {token!r} has {len(group)} candidate/colliding meanings: {ids}"))
    return messages


def check_symbols(root: Path) -> list[str]:
    path = root / "registry" / "symbols.yaml"
    if not path.exists():
        return [error("ACX-NOM-E000", path, "missing symbols registry")]
    records = registry_records(root, "symbols.yaml")
    messages = check_unique(records, "symbol_id", path, "ACX-NOM-E001")

    glyphs: dict[str, list[dict[str, Any]]] = {}
    for record in records:
        symbol_id = record.get("symbol_id", "<unknown>")
        surface_form = record.get("surface_form")
        unit = record.get("unit", None)
        domain = record.get("domain")
        rust = record.get("rust") or {}

        for required in ["namespace", "scope", "surface_form", "display_form", "semantic_role", "definition", "domain", "status"]:
            if not record.get(required):
                messages.append(error("ACX-NOM-E001", path, f"{symbol_id} missing {required}"))

        if domain in {"positive_real", "real", "vector3", "matrix"} and unit is None:
            messages.append(error("ACX-NOM-E004", path, f"{symbol_id} has physical/mathematical domain {domain} but unit is null"))

        if rust and not rust.get("identifier"):
            messages.append(error("ACX-NOM-E009", path, f"{symbol_id} has rust block without identifier"))

        if surface_form:
            glyphs.setdefault(surface_form, []).append(record)

    for surface_form, group in glyphs.items():
        if len(group) > 1:
            scopes = sorted(str(g.get("scope")) for g in group)
            messages.append(warn("ACX-NOM-W004", path, f"surface form {surface_form!r} reused in {len(group)} records: {scopes}"))
    return messages


def check_bridges(root: Path) -> list[str]:
    symbols_path = root / "registry" / "symbols.yaml"
    bridges_path = root / "registry" / "bridges.yaml"
    if not bridges_path.exists():
        return [error("ACX-NOM-E000", bridges_path, "missing bridges registry")]
    symbol_ids: set[str] = set()
    if symbols_path.exists():
        symbol_ids = {record.get("symbol_id") for record in registry_records(root, "symbols.yaml") if record.get("symbol_id")}

    records = registry_records(root, "bridges.yaml")
    messages = check_unique(records, "bridge_id", bridges_path, "ACX-NOM-E009")

    for record in records:
        bridge_id = record.get("bridge_id", "<unknown>")
        bindings = record.get("bindings") or []
        if not bindings:
            messages.append(error("ACX-NOM-E009", bridges_path, f"{bridge_id} has no bindings"))
        for binding in bindings:
            symbol_id = binding.get("symbol_id")
            if symbol_id not in symbol_ids:
                messages.append(error("ACX-NOM-E009", bridges_path, f"{bridge_id} references unknown symbol_id {symbol_id}"))
            for required in ["math_symbol", "rust_identifier", "rust_type"]:
                if not binding.get(required):
                    messages.append(error("ACX-NOM-E009", bridges_path, f"{bridge_id} binding for {symbol_id} missing {required}"))
    return messages


def check_rust_file(path: Path) -> list[str]:
    messages: list[str] = []
    text = path.read_text(encoding="utf-8", errors="replace")

    for match in RAW_IDENTIFIER_RE.finditer(text):
        messages.append(error("ACX-NOM-E011", path, f"raw identifier {match.group(0)!r} requires waiver unless generated external binding"))

    for line_number, line in enumerate(text.splitlines(), start=1):
        stripped = line.strip()
        if stripped.startswith("//"):
            continue
        for match in LET_IDENTIFIER_RE.finditer(line):
            ident = match.group(1)
            if ident in RISKY_SINGLE_LETTER_VALUES or ident in RISKY_SHORT_VALUES:
                messages.append(error("ACX-NOM-E006", path, f"line {line_number}: risky Rust identifier {ident!r}"))

        for match in CONST_GENERIC_RE.finditer(line):
            ident = match.group(1)
            if ident in {"N", "M", "K"}:
                messages.append(warn("ACX-NOM-W005", path, f"line {line_number}: const generic {ident!r} should be semantic, e.g. ROWS/COLS/SAMPLE_COUNT"))
    return messages


def check_rust(root: Path, include_examples: bool = False) -> list[str]:
    messages: list[str] = []
    for path in root.rglob("*.rs"):
        # Skip target/build outputs and illustrative examples by default.
        if "target" in path.parts or "build" in path.parts:
            continue
        if not include_examples and "examples" in path.parts:
            continue
        messages.extend(check_rust_file(path))
    return messages


def allowed_acronym_tokens(root: Path) -> set[str]:
    allowed = set(DEFAULT_ALLOWED_SCAN_TOKENS)
    for record in registry_records(root, "acronyms.yaml"):
        if record.get("token"):
            allowed.add(str(record["token"]))
    for record in registry_records(root, "aliases.yaml"):
        if record.get("namespace") == "acronym" and record.get("alias"):
            allowed.add(str(record["alias"]))
    for record in registry_records(root, "waivers.yaml"):
        if record.get("surface_form"):
            allowed.add(str(record["surface_form"]))
    return allowed


def should_scan_file(path: Path, include_examples: bool) -> bool:
    if any(part in {".git", "target", "build", "schemas", "registry"} for part in path.parts):
        return False
    if not include_examples and "examples" in path.parts:
        return False
    return path.suffix in {".md", ".rs", ".txt", ".yaml", ".yml"}


def scan_acronyms(root: Path, include_examples: bool = False) -> list[str]:
    messages: list[str] = []
    allowed = allowed_acronym_tokens(root)
    for path in root.rglob("*"):
        if not path.is_file() or not should_scan_file(path, include_examples=include_examples):
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        for line_number, line in enumerate(text.splitlines(), start=1):
            # Ignore fenced-code metadata and obvious rule-code lines.
            if line.strip().startswith("ACX-NOM-"):
                continue
            for match in ACRONYM_TOKEN_RE.finditer(line):
                token = match.group(1)
                if token in allowed:
                    continue
                if token.startswith("ACX-") or token.startswith("NOM-"):
                    continue
                messages.append(warn("ACX-NOM-W008", path, f"line {line_number}: acronym-like token {token!r} is not registered"))
    return messages


def main() -> int:
    parser = argparse.ArgumentParser(description="AeroCodex nomenclature lint scaffold")
    parser.add_argument("--root", default=".", help="repository/package root")
    parser.add_argument("--no-rust-scan", action="store_true", help="skip simple Rust regex scan")
    parser.add_argument("--include-examples", action="store_true", help="include examples/ in the Rust/acronym scans")
    parser.add_argument("--scan-acronyms", action="store_true", help="scan durable text/code files for unknown acronym-like tokens")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    messages: list[str] = []

    try:
        messages.extend(check_concepts(root))
        messages.extend(check_aliases(root))
        messages.extend(check_terminology_sources(root))
        messages.extend(check_acronyms(root))
        messages.extend(check_symbols(root))
        messages.extend(check_bridges(root))
        if not args.no_rust_scan:
            messages.extend(check_rust(root, include_examples=args.include_examples))
        if args.scan_acronyms:
            messages.extend(scan_acronyms(root, include_examples=args.include_examples))
    except RuntimeError as exc:
        print(f"ERROR ACX-NOM-E000: {exc}", file=sys.stderr)
        return 2

    for message in messages:
        print(message)

    has_errors = any(message.startswith("ERROR") for message in messages)
    return 1 if has_errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
