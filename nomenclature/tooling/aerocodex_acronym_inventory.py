#!/usr/bin/env python3
"""AeroCodex repository-wide acronym inventory and adoption guard.

This script is intentionally standard-library only so it can run in CI before
any Python package installation. It complements `aerocodex_nom_lint.py`:

- inventory mode captures acronym-like tokens already present in the repository;
- check-new mode fails when a future change introduces an unregistered token
  that is not in the adoption baseline or built-in allowlist.

It is conservative and lexical, not a semantic parser. False positives should be
resolved by adding an acronym record, a waiver, or a baseline update reviewed by
the nomenclature owner.
"""

from __future__ import annotations

import argparse
import csv
import json
import re
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Iterable

TOKEN_RE = re.compile(r"(?<![A-Za-z0-9])([A-Z][A-Z0-9&/-]{1,14})(?![A-Za-z0-9])")
REGISTERED_TOKEN_RE = re.compile(r"^\s*token:\s*['\"]?([^'\"#\n]+?)['\"]?\s*(?:#.*)?$", re.MULTILINE)
ALIAS_TOKEN_RE = re.compile(r"^\s*-\s*alias:\s*['\"]?([^'\"#\n]+?)['\"]?\s*(?:#.*)?$", re.MULTILINE)
SURFACE_FORM_RE = re.compile(r"^\s*surface_form:\s*['\"]?([^'\"#\n]+?)['\"]?\s*(?:#.*)?$", re.MULTILINE)

TEXT_SUFFIXES = {
    ".bib", ".csv", ".json", ".lock", ".md", ".rs", ".tex", ".toml",
    ".txt", ".yaml", ".yml",
}

SKIP_DIRS = {
    ".git", "target", "build", "dist", "node_modules", "__pycache__",
    # Avoid self-referential churn from generated inventory files.
    "generated",
}

SKIP_RELATIVE_PATHS = {
    # Machine-generated hashes/manifests are not durable terminology surfaces.
    "checksums/SHA256SUMS",
    "nomenclature/MANIFEST.json",
}

# Tokens that are allowed repository infrastructure terms. Domain acronyms should
# still live in registry/acronyms.yaml; this list is for ubiquitous software and
# artifact terms that do not need first-use expansion in every file.
BUILTIN_ALLOWED = {
    "ACX", "AI", "API", "AST", "CI", "CLI", "CSV", "FFI", "HTML", "HTTP",
    "HTTPS", "ID", "JSON", "JSONL", "MIT", "NOM", "PR", "README", "SHA256",
    "SQL", "TOML", "URL", "UTC", "YAML", "ZIP",
}

# Uppercase headings or English words often emitted by generated inventories,
# comments, or validation names. They are captured in the inventory but do not
# cause new-token failures unless strict mode is requested.
UPPERCASE_WORD_ALLOW = {
    "AERODYNAMIC", "AERODYNAMICS", "AIR", "ALLOWED", "ALTITUDE", "ANGLE",
    "AREA", "ASSUMPTIONS", "ASTRODYNAMICS", "ATMOSPHERE", "BALANCE", "BASIC",
    "BASICS", "BEAM", "BIOFILM", "BODY", "BOLTZMANN", "BOUNDARY", "BUCKLING",
    "BUFFER", "CANTILEVER", "CARBON", "CARD", "CELESTIAL", "CHECK", "CHOKED",
    "CIRCULAR", "CLOSURE", "CODEX", "COEFFICIENT", "CONDUCTION", "CONSTANT",
    "CONSTANTS", "CONTROL", "CONVECTIVE", "DAILY", "DATA", "DEFLECTION",
    "DELTA", "DENSITY", "DERIVATIVE", "DIOXIDE", "DOWNSTREAM", "DRAG", "DRY",
    "DYNAMIC", "DYNAMICS", "EARTH", "EQUATION", "ESCAPE", "EULER", "EXCESS",
    "EXPANSION", "FIELDS", "FILE", "FLIGHT", "FLOW", "FLUX", "FRACTION",
    "FROM", "GAMMA", "GAS", "GRAVITATIONAL", "GRAVITY", "HEAT", "HOHMANN",
    "IDEAL", "INDUCED", "INFINITY", "INPUTS", "INVERSE", "IRRADIANCE",
    "ISENTROPIC", "LEVEL", "LIFE", "LIFT", "LIGHT", "LOAD", "MACH", "MANIFEST",
    "MASS", "MAX", "MEAN", "MEYER", "MIN", "MODEL", "NORMAL", "NOTES", "OF",
    "ORBITAL", "OUTPUTS", "OXYGEN", "PARAMETER", "PARAMETERS", "PER", "PERFECT",
    "PLACEHOLDER", "POSITIVE", "POWER", "PRANDTL", "PRESSURE", "PROCEED",
    "PRODUCTION", "PROPULSION", "RADIATIVE", "RADIUS", "RATE", "RATIO",
    "REFERENCE", "REFERENCES", "REPORT", "REQUIRED", "RESEARCH", "RESIDUAL",
    "RESISTANCE", "SERVICE", "SHOCK", "SOLAR", "SOURCE", "SOURCES", "SPECIFIC",
    "SPECIES", "SPEED", "SPHERE", "STALL", "STANDARD", "STATUS", "STEFAN",
    "STRESS", "STRUCTURES", "SUPPORT", "SYSTEM", "TEMPERATURE", "THERMO",
    "THINFILM", "THRUST", "TIME", "TOTAL", "TRANSFER", "TROPOSPHERE", "TURN",
    "TWO", "UNIVERSAL", "VELOCITY", "VIS", "VIVA", "OBLIQUE", "SEA", "ALGAL", "BETA", "EPSILON", "EPS",
    "PERIOD", "INFLUENCE", "PHASE", "SEEDS", "SOUND", "IMPULSE",
    "INTEGRATION", "TSIOLKOVSKY", "BENDING", "BLANKEN", "COMMAND",
    "COUNT", "EFFECTIVE", "LIMNOSPIRA", "ROWS", "COLS", "ERROR",
    "PROPOSAL",
}

CHEMICAL_OR_UNIT_RE = re.compile(
    r"^(?:[A-Z]{1,2}\d+(?:/[A-Z]{1,2}\d+)*|[A-Z]{1,2}\d*\+?|KG|PA|M2|M3|S2|MOL|K4|J/)$"
)
ARTIFACT_ID_RE = re.compile(r"^(?:AC-P\d+|ACX-NOM-(?:\d+|[EW]\d+|WAIVER-\d+)|T\d{3,}|V\d+|E\d{3,}|MACH\d+)$")
SEPARATOR_RE = re.compile(r"[\/-]")


@dataclass
class Occurrence:
    file: str
    line: int
    context: str


@dataclass
class TokenRecord:
    token: str
    count: int
    files: int
    category: str
    registered: bool
    builtin_allowed: bool
    baseline_allowed: bool
    first_file: str
    first_line: int
    first_context: str
    example_locations: list[Occurrence]


def load_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def parse_registry_tokens(nomenclature_root: Path) -> set[str]:
    tokens: set[str] = set()
    for rel in ["registry/acronyms.yaml"]:
        path = nomenclature_root / rel
        if path.exists():
            tokens.update(value.strip() for value in REGISTERED_TOKEN_RE.findall(load_text(path)))
    # Acronym aliases are useful for variants such as GN&C.
    aliases = nomenclature_root / "registry/aliases.yaml"
    if aliases.exists():
        text = load_text(aliases)
        for value in ALIAS_TOKEN_RE.findall(text):
            cleaned = value.strip()
            if re.search(r"[A-Z]", cleaned) and cleaned.upper() == cleaned:
                tokens.add(cleaned)
    # Waivers can contain symbol-like surface forms that should not be flagged.
    waivers = nomenclature_root / "registry/waivers.yaml"
    if waivers.exists():
        tokens.update(value.strip() for value in SURFACE_FORM_RE.findall(load_text(waivers)))
    return {t for t in tokens if t}


def parse_baseline(path: Path | None) -> set[str]:
    if path is None or not path.exists():
        return set()
    data = json.loads(load_text(path))
    tokens = data.get("tokens", [])
    if isinstance(tokens, list):
        return {str(t) for t in tokens}
    if isinstance(tokens, dict):
        return {str(t) for t in tokens.keys()}
    return set()


def should_scan(path: Path, repo_root: Path, include_generated: bool) -> bool:
    try:
        rel_parts = path.relative_to(repo_root).parts
    except ValueError:
        rel_parts = path.parts
    rel_posix = "/".join(rel_parts)
    if rel_posix in SKIP_RELATIVE_PATHS:
        return False
    for part in rel_parts:
        if part in SKIP_DIRS and not (include_generated and part == "generated"):
            return False
    if not path.is_file():
        return False
    if path.suffix.lower() not in TEXT_SUFFIXES:
        return False
    if path.stat().st_size > 2_000_000:
        return False
    return True


def scan_repo(repo_root: Path, include_generated: bool = False) -> dict[str, list[Occurrence]]:
    found: dict[str, list[Occurrence]] = defaultdict(list)
    for path in sorted(repo_root.rglob("*")):
        if not should_scan(path, repo_root, include_generated=include_generated):
            continue
        rel = str(path.relative_to(repo_root))
        text = load_text(path)
        for line_number, line in enumerate(text.splitlines(), start=1):
            if line.lstrip().startswith("#") and "ACX-NOM-" in line:
                continue
            for match in TOKEN_RE.finditer(line):
                token = match.group(1).strip()
                context = line.strip().replace("\t", " ")[:240]
                found[token].append(Occurrence(rel, line_number, context))
    return dict(found)


def compound_known(token: str, registered_tokens: set[str]) -> bool:
    if "/" not in token and "-" not in token:
        return False
    parts = [part for part in SEPARATOR_RE.split(token) if part]
    if len(parts) < 2:
        return False
    for part in parts:
        if part.isdigit():
            continue
        if part in registered_tokens or part in BUILTIN_ALLOWED:
            continue
        if CHEMICAL_OR_UNIT_RE.match(part) or ARTIFACT_ID_RE.match(part):
            continue
        return False
    return True


def classify_token(token: str, registered: bool, builtin_allowed: bool, baseline_allowed: bool, registered_tokens: set[str]) -> str:
    if registered:
        return "registered"
    if builtin_allowed:
        return "software_or_repo_infrastructure"
    if compound_known(token, registered_tokens):
        return "compound_registered_or_known"
    if CHEMICAL_OR_UNIT_RE.match(token):
        return "chemical_formula_or_unit_symbol"
    if ARTIFACT_ID_RE.match(token):
        return "artifact_id_or_version_token"
    if token in UPPERCASE_WORD_ALLOW:
        return "uppercase_word_or_heading"
    if baseline_allowed:
        return "adoption_baseline_unregistered"
    return "needs_registry_or_waiver"


def build_records(
    found: dict[str, list[Occurrence]],
    registered_tokens: set[str],
    baseline_tokens: set[str],
) -> list[TokenRecord]:
    records: list[TokenRecord] = []
    for token, occurrences in found.items():
        files = {o.file for o in occurrences}
        registered = token in registered_tokens
        builtin_allowed = token in BUILTIN_ALLOWED
        baseline_allowed = token in baseline_tokens
        category = classify_token(token, registered, builtin_allowed, baseline_allowed, registered_tokens)
        first = occurrences[0]
        records.append(
            TokenRecord(
                token=token,
                count=len(occurrences),
                files=len(files),
                category=category,
                registered=registered,
                builtin_allowed=builtin_allowed,
                baseline_allowed=baseline_allowed,
                first_file=first.file,
                first_line=first.line,
                first_context=first.context,
                example_locations=occurrences[:5],
            )
        )
    return sorted(records, key=lambda r: (-r.count, r.token))


def write_csv(path: Path, records: list[TokenRecord]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=[
                "token", "count", "files", "category", "registered", "builtin_allowed",
                "baseline_allowed", "first_file", "first_line", "first_context",
            ],
        )
        writer.writeheader()
        for r in records:
            writer.writerow({
                "token": r.token,
                "count": r.count,
                "files": r.files,
                "category": r.category,
                "registered": str(r.registered).lower(),
                "builtin_allowed": str(r.builtin_allowed).lower(),
                "baseline_allowed": str(r.baseline_allowed).lower(),
                "first_file": r.first_file,
                "first_line": r.first_line,
                "first_context": r.first_context,
            })


def write_json(path: Path, records: list[TokenRecord]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    serializable = []
    for r in records:
        value = asdict(r)
        value["example_locations"] = [asdict(o) for o in r.example_locations]
        serializable.append(value)
    path.write_text(
        json.dumps({"kind": "acx.acronym_inventory", "version": "0.1", "tokens": serializable}, indent=2, sort_keys=True),
        encoding="utf-8",
    )


def write_baseline(path: Path, records: list[TokenRecord]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    baseline_tokens = sorted(r.token for r in records if not r.registered and not r.builtin_allowed)
    data = {
        "kind": "acx.acronym_adoption_baseline",
        "version": "0.1",
        "policy": "Tokens listed here were present when the nomenclature policy was adopted. New unregistered tokens must be added to registry/acronyms.yaml, waived, or deliberately baseline-updated by the nomenclature owner.",
        "tokens": baseline_tokens,
    }
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_markdown(path: Path, records: list[TokenRecord]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    counts = Counter(r.category for r in records)
    lines = [
        "# Current Repository Acronym Inventory",
        "",
        "Generated by `nomenclature/tooling/aerocodex_acronym_inventory.py`.",
        "",
        "This is an adoption snapshot for the nomenclature/acronym policy. It intentionally captures lexical candidates, including false positives, so future changes can be gated against a baseline.",
        "",
        "## Summary",
        "",
    ]
    for category, count in sorted(counts.items()):
        lines.append(f"- `{category}`: {count}")
    lines.extend(["", "## Top tokens", "", "| Token | Count | Files | Category | First location |", "|---|---:|---:|---|---|"])
    for r in records[:120]:
        loc = f"{r.first_file}:{r.first_line}".replace("|", "\\|")
        lines.append(f"| `{r.token}` | {r.count} | {r.files} | `{r.category}` | `{loc}` |")
    needs = [r for r in records if r.category == "needs_registry_or_waiver"]
    lines.extend(["", "## Tokens needing registry or waiver review", ""])
    if needs:
        for r in needs[:200]:
            lines.append(f"- `{r.token}` ({r.count} occurrences, {r.files} files), first seen at `{r.first_file}:{r.first_line}`")
    else:
        lines.append("No non-baselined tokens currently need registry or waiver review.")
    path.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")


def fail_on_new(records: list[TokenRecord], strict_registered: bool) -> list[TokenRecord]:
    failures: list[TokenRecord] = []
    for r in records:
        if r.registered or r.builtin_allowed:
            continue
        if not strict_registered and r.category in {
            "chemical_formula_or_unit_symbol", "artifact_id_or_version_token",
            "compound_registered_or_known", "uppercase_word_or_heading",
            "adoption_baseline_unregistered",
        }:
            continue
        failures.append(r)
    return failures


def main() -> int:
    parser = argparse.ArgumentParser(description="AeroCodex acronym inventory and new-token guard")
    parser.add_argument("--repo-root", default=".", help="AeroCodex repository root")
    parser.add_argument("--nomenclature-root", default="nomenclature", help="nomenclature directory root")
    parser.add_argument("--baseline", help="baseline JSON path for check-new mode")
    parser.add_argument("--write-generated", action="store_true", help="write inventory CSV/JSON/Markdown and baseline under nomenclature/generated")
    parser.add_argument("--check-new", action="store_true", help="fail if new unregistered tokens are not covered by registry, allowlist, or baseline")
    parser.add_argument("--strict-registered", action="store_true", help="in check-new mode, require every token to be registered or builtin-allowed")
    parser.add_argument("--include-generated", action="store_true", help="scan generated directories too; normally disabled to avoid self-churn")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    nomenclature_root = Path(args.nomenclature_root).resolve()
    if not repo_root.is_dir():
        print(f"ERROR: repo root does not exist: {repo_root}", file=sys.stderr)
        return 2
    if not nomenclature_root.is_dir():
        print(f"ERROR: nomenclature root does not exist: {nomenclature_root}", file=sys.stderr)
        return 2

    registered = parse_registry_tokens(nomenclature_root)
    baseline = parse_baseline(Path(args.baseline).resolve() if args.baseline else None)
    found = scan_repo(repo_root, include_generated=args.include_generated)
    records = build_records(found, registered, baseline)

    if args.write_generated:
        out_dir = nomenclature_root / "generated"
        write_csv(out_dir / "current_repo_acronym_inventory.csv", records)
        write_json(out_dir / "current_repo_acronym_inventory.json", records)
        write_markdown(out_dir / "current_repo_acronym_inventory.md", records)
        write_baseline(out_dir / "current_repo_acronym_baseline.json", records)
        print(f"wrote acronym inventory and baseline under {out_dir}")

    if args.check_new:
        failures = fail_on_new(records, strict_registered=args.strict_registered)
        if failures:
            print("ERROR: new unregistered acronym-like tokens were found.", file=sys.stderr)
            print("Add registry/acronyms.yaml records, add waivers, or update the adoption baseline with review.", file=sys.stderr)
            for r in failures[:100]:
                print(f"- {r.token}: {r.count} occurrences; first {r.first_file}:{r.first_line}; {r.first_context}", file=sys.stderr)
            if len(failures) > 100:
                print(f"... {len(failures) - 100} more", file=sys.stderr)
            return 1
        print("acronym guard passed: no new unregistered tokens outside registry/allowlist/baseline")

    if not args.write_generated and not args.check_new:
        counts = Counter(r.category for r in records)
        print(json.dumps({"tokens": len(records), "categories": dict(sorted(counts.items()))}, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
