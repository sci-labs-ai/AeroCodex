#!/usr/bin/env python3
"""Static artifact checks for the AeroCodex thin-film BLSS package.

This script intentionally avoids external dependencies. It checks that source
materials, code modules, citation files, manifests, validation cards, and source
registry files required by the thin-film BLSS conversion are present and internally
consistent.
"""
from __future__ import annotations

import csv
import hashlib
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_FILES = [
    "DATA_MANIFEST.toml",
    "citations/blss_thinfilm_refs.bib",
    "data/thinfilm/equation_manifest.csv",
    "data/thinfilm/source_verification.csv",
    "crates/aero-codex-life-support/src/brlss_backbone.rs",
    "crates/aero-codex-life-support/src/melissa_photobioreactor.rs",
    "crates/aero-codex-life-support/src/nitrifying_biofilm.rs",
    "crates/aero-codex-life-support/src/thinfilm_algal_biofilm.rs",
    "crates/aero-codex-life-support/src/thinfilm_provenance.rs",
    "source_material/new_thinfilm/blss_thinfilm_report.pdf",
    "source_material/new_thinfilm/blss_thinfilm_report.tex",
    "source_material/new_thinfilm/blss_thinfilm_refs.bib",
    "source_material/new_thinfilm/blss_thinfilm_latex_source.zip",
]

REQUIRED_BIB_KEYS = {
    "poughon2021",
    "garcia2021",
    "perez2005",
    "montras2009",
    "polizzi2022",
    "detrell2021",
    "vermeulen2023",
    "blanken2014",
    "blanken2016",
    "schaap2017",
    "esaC3",
}

REQUIRED_VALIDATION_CARDS = [
    "validation/cards/life_support_thinfilm_brlss_backbone.yaml",
    "validation/cards/life_support_thinfilm_melissa_c4a_photobioreactor.yaml",
    "validation/cards/life_support_thinfilm_melissa_c3_nitrifying_biofilm.yaml",
    "validation/cards/life_support_thinfilm_algal_biofilm_pde_rom.yaml",
    "validation/cards/life_support_thinfilm_citation_manifest.yaml",
]

REQUIRED_SOURCE_REGISTRY_PREFIXES = [
    "source.life_support.thinfilm_blss_report_2026.equation_traceable",
    "source.life_support.poughon_2021.equation_traceable",
    "source.life_support.garcia_2021.equation_traceable",
    "source.life_support.perez_2005.equation_traceable",
    "source.life_support.montras_2009.equation_traceable",
    "source.life_support.polizzi_2022.equation_traceable",
    "source.life_support.blanken_2014.equation_traceable",
    "source.life_support.blanken_2016.equation_traceable",
    "source.life_support.detrell_2021.equation_traceable",
    "source.life_support.vermeulen_2023.equation_traceable",
]


def fail(message: str) -> None:
    print(f"thinfilm artifact verification failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def require_files() -> None:
    for rel in REQUIRED_FILES + REQUIRED_VALIDATION_CARDS:
        path = ROOT / rel
        if not path.is_file():
            fail(f"missing required file: {rel}")


def check_bib_keys() -> None:
    text = (ROOT / "citations/blss_thinfilm_refs.bib").read_text(encoding="utf-8")
    keys = set(re.findall(r"@\w+\{([^,]+),", text))
    missing = sorted(REQUIRED_BIB_KEYS - keys)
    if missing:
        fail(f"missing BibTeX keys: {missing}")


def check_equation_manifest() -> None:
    manifest = ROOT / "data/thinfilm/equation_manifest.csv"
    with manifest.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    if len(rows) != 51:
        fail(f"expected 51 equation manifest rows, found {len(rows)}")
    equations = {row["equation"] for row in rows}
    for eq in map(str, range(1, 52)):
        if eq not in equations:
            fail(f"equation manifest missing Eq. {eq}")
    for row in rows:
        if not row["codex_id"].startswith("life_support.thinfilm."):
            fail(f"invalid Codex ID in equation manifest: {row['codex_id']}")
        if "::" not in row["rust_function"]:
            fail(f"function missing module separator: {row['rust_function']}")


def check_provenance_matches_manifest() -> None:
    provenance = (ROOT / "crates/aero-codex-life-support/src/thinfilm_provenance.rs").read_text(
        encoding="utf-8"
    )
    refs = re.findall(
        r"EquationReference\s*\{\s*equation:\s*\"([^\"]+)\"",
        provenance,
        re.S,
    )
    if len(refs) != 51:
        fail(f"expected 51 runtime EquationReference entries, found {len(refs)}")
    for eq in map(str, range(1, 52)):
        if eq not in refs:
            fail(f"runtime provenance missing Eq. {eq}")
    if "THINFILM_SOURCE_IDS" not in provenance or "EQUATION_REFERENCES" not in provenance:
        fail("provenance table symbols missing")


def check_source_registry() -> None:
    registry_text = "\n".join(
        path.read_text(encoding="utf-8") for path in (ROOT / "validation/source_registry").glob("*.yaml")
    )
    for source_id in REQUIRED_SOURCE_REGISTRY_PREFIXES:
        if source_id not in registry_text:
            fail(f"missing source registry id: {source_id}")


def check_manifest_hashes() -> None:
    text = (ROOT / "DATA_MANIFEST.toml").read_text(encoding="utf-8")
    pairs = re.findall(r'path = "([^"]+)"\nbytes = (\d+)\nsha256 = "([0-9a-f]{64})"', text)
    if len(pairs) < 10:
        fail("DATA_MANIFEST.toml does not list enough hashed files")
    for rel, size_text, expected_hash in pairs:
        path = ROOT / rel
        if not path.is_file():
            fail(f"manifest references missing file: {rel}")
        if path.stat().st_size != int(size_text):
            fail(f"manifest byte count mismatch: {rel}")
        actual_hash = sha256(path)
        if actual_hash != expected_hash:
            fail(f"manifest sha256 mismatch: {rel}")


def main() -> None:
    require_files()
    check_bib_keys()
    check_equation_manifest()
    check_provenance_matches_manifest()
    check_source_registry()
    check_manifest_hashes()
    print("thinfilm artifact verification passed")


if __name__ == "__main__":
    main()
