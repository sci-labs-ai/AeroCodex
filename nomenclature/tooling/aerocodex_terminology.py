#!/usr/bin/env python3
"""AeroCodex terminology lookup and AI-pack generator.

This tool deliberately stays small and dependency-light. It loads the YAML
registries in this starter kit and can:

- look up acronym/alias/concept records by token,
- export a JSONL index for retrieval systems, and
- build a compact Markdown terminology pack for an AI task.

It is not a replacement for a production ontology service. It is the seed of
one.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable

try:
    import yaml  # type: ignore
except Exception:  # pragma: no cover
    yaml = None


TOKEN_RE = re.compile(r"(?<![A-Za-z0-9])([A-Z][A-Z0-9&/-]{1,14})(?![A-Za-z0-9])")

DEFAULT_ALLOWED_UNKNOWN_TOKENS = {
    "ACX", "NOM", "INV", "AI", "API", "AST", "CLI", "CI", "CSV", "FFI",
    "HTML", "HTTP", "HTTPS", "ICD", "ID", "JSON", "JSONL", "PR", "SQL",
    "URL", "UTC", "YAML", "AIAA", "AIM", "CCSDS", "DOD", "ECSS", "FAA",
    "ICAO", "NASA", "GPS", "NED", "WGS84",
}


@dataclass(frozen=True)
class Registry:
    root: Path
    concepts: list[dict[str, Any]]
    aliases: list[dict[str, Any]]
    acronyms: list[dict[str, Any]]
    terminology_sources: list[dict[str, Any]]


def load_yaml(path: Path) -> Any:
    if yaml is None:
        raise RuntimeError("PyYAML is required. Install with: python -m pip install pyyaml")
    if not path.exists():
        return {"records": []}
    with path.open("r", encoding="utf-8") as f:
        return yaml.safe_load(f) or {"records": []}


def load_registry(root: Path) -> Registry:
    registry_dir = root / "registry"
    return Registry(
        root=root,
        concepts=load_yaml(registry_dir / "concepts.yaml").get("records", []) or [],
        aliases=load_yaml(registry_dir / "aliases.yaml").get("records", []) or [],
        acronyms=load_yaml(registry_dir / "acronyms.yaml").get("records", []) or [],
        terminology_sources=load_yaml(registry_dir / "terminology_sources.yaml").get("records", []) or [],
    )


def normalize_token(token: str) -> str:
    return token.strip()


def slug_text(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", value.lower()).strip("_")


def domain_score(record: dict[str, Any], domain_hints: Iterable[str]) -> int:
    hints = {d.lower() for d in domain_hints}
    domains = {str(d).lower() for d in record.get("domains", []) or []}
    return len(hints & domains) * 2


def signal_score(record: dict[str, Any], text: str) -> int:
    lowered = text.lower()
    disambiguation = record.get("disambiguation") or {}
    score = 0
    for signal in disambiguation.get("signals", []) or []:
        if str(signal).lower() in lowered:
            score += 1
    for reject in disambiguation.get("reject_if_near", []) or []:
        if str(reject).lower() in lowered:
            score -= 1
    return score


def record_score(record: dict[str, Any], text: str, domain_hints: Iterable[str]) -> int:
    return domain_score(record, domain_hints) + signal_score(record, text)


def token_occurs(token: str, text: str) -> bool:
    escaped = re.escape(token)
    pattern = re.compile(rf"(?<![A-Za-z0-9]){escaped}(?![A-Za-z0-9])")
    return bool(pattern.search(text))


def detected_known_acronyms(registry: Registry, text: str) -> dict[str, list[dict[str, Any]]]:
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for record in registry.acronyms:
        token = str(record.get("token", ""))
        if token and token_occurs(token, text):
            grouped[token].append(record)
    return dict(sorted(grouped.items()))


def lookup_records(registry: Registry, token: str) -> dict[str, Any]:
    token = normalize_token(token)
    token_lower = token.lower()
    acronyms = [r for r in registry.acronyms if str(r.get("token", "")).lower() == token_lower]
    aliases = [r for r in registry.aliases if str(r.get("alias", "")).lower() == token_lower]
    concepts = [
        r
        for r in registry.concepts
        if str(r.get("canonical", "")).lower() == token_lower
        or token in (r.get("aliases", []) or [])
        or token_lower in {str(a).lower() for a in (r.get("aliases", []) or [])}
    ]
    return {"query": token, "acronyms": acronyms, "aliases": aliases, "concepts": concepts}


def source_lookup(registry: Registry) -> dict[str, dict[str, Any]]:
    return {str(r.get("source_id")): r for r in registry.terminology_sources if r.get("source_id")}


def status_label(record: dict[str, Any]) -> str:
    status = record.get("status", "unknown")
    domains = ", ".join(record.get("domains", []) or [])
    if domains:
        return f"{status}; {domains}"
    return str(status)


def render_record_line(record: dict[str, Any], text: str, domains: list[str]) -> str:
    source = record.get("source") or {}
    score = record_score(record, text, domains)
    return (
        f"- {record.get('token')} = {record.get('expansion')} "
        f"[{status_label(record)}; score={score}]\n"
        f"  Source: {source.get('authority', 'unknown')} / {source.get('authority_rank', 'unknown')} "
        f"({source.get('source_id', 'no-source-id')})\n"
        f"  AI note: {(record.get('ai') or {}).get('summary', 'No AI note provided.')}"
    )


def render_pack(registry: Registry, text: str, domains: list[str], source_name: str | None = None) -> str:
    grouped = detected_known_acronyms(registry, text)
    generated = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
    lines: list[str] = []
    lines.append("# AeroCodex AI Terminology Pack")
    lines.append("")
    if source_name:
        lines.append(f"Generated for: `{source_name}`  ")
    lines.append(f"Generated at: `{generated}`  ")
    if domains:
        lines.append("Domains: " + ", ".join(f"`{d}`" for d in domains))
    else:
        lines.append("Domains: not specified")
    lines.append("")
    lines.append("## Governing instruction")
    lines.append("")
    lines.append(
        "Use this pack as scoped terminology context under `ACX-NOM-001`. "
        "Do not assume acronym meanings that are absent or marked ambiguous. "
        "When a token has multiple plausible meanings, use explicit local evidence or return the ambiguity."
    )
    lines.append("")

    if not grouped:
        lines.append("## Detected acronyms")
        lines.append("")
        lines.append("No registered acronym tokens were detected in the supplied text.")
        lines.append("")
    else:
        singletons = {t: rs for t, rs in grouped.items() if len(rs) == 1}
        ambiguous = {t: rs for t, rs in grouped.items() if len(rs) > 1}

        lines.append("## Detected acronyms")
        lines.append("")
        if singletons:
            for token, records in singletons.items():
                lines.append(render_record_line(records[0], text, domains))
                first_use = records[0].get("first_use") or {}
                if first_use.get("requires_definition"):
                    lines.append("  First-use rule: expand at first durable use unless already defined by the artifact.")
                lines.append("")
        else:
            lines.append("No unambiguous registered acronym tokens were detected.")
            lines.append("")

        lines.append("## Ambiguous acronyms")
        lines.append("")
        if ambiguous:
            for token, records in ambiguous.items():
                scored = sorted(records, key=lambda r: record_score(r, text, domains), reverse=True)
                lines.append(f"- `{token}` has {len(records)} registered meanings:")
                for record in scored:
                    source = record.get("source") or {}
                    signals = ", ".join((record.get("disambiguation") or {}).get("signals", [])[:6])
                    lines.append(
                        f"  - {record.get('expansion')} "
                        f"[{status_label(record)}; score={record_score(record, text, domains)}; "
                        f"source={source.get('source_id', 'no-source-id')}]"
                    )
                    if signals:
                        lines.append(f"    Signals: {signals}")
                lines.append("  Resolver rule: choose only when local evidence is explicit; otherwise mark ambiguous.")
                lines.append("")
        else:
            lines.append("No ambiguous registered acronym tokens were detected.")
            lines.append("")

    # Surface unknown uppercase tokens as discovery candidates.
    known_tokens = {str(r.get("token")) for r in registry.acronyms if r.get("token")} | DEFAULT_ALLOWED_UNKNOWN_TOKENS
    unknown_tokens = sorted({m.group(1) for m in TOKEN_RE.finditer(text)} - known_tokens)
    if unknown_tokens:
        lines.append("## Unknown acronym-like tokens")
        lines.append("")
        lines.append(
            "These tokens look acronym-like but are not present in `registry/acronyms.yaml`. "
            "They may need acronym records, alias records, waivers, or local definitions."
        )
        lines.append("")
        for token in unknown_tokens[:50]:
            lines.append(f"- `{token}`")
        if len(unknown_tokens) > 50:
            lines.append(f"- ... {len(unknown_tokens) - 50} more")
        lines.append("")

    lines.append("## Resolver reminders")
    lines.append("")
    lines.append("- Prefer project/contract glossary meanings over general aerospace usage.")
    lines.append("- Candidate records are useful hints, not final authority.")
    lines.append("- Preserve source-original wording when canonical mapping is not approved.")
    lines.append("- Do not invent acronym expansions for durable AeroCodex outputs.")
    return "\n".join(lines).rstrip() + "\n"


def iter_index_records(registry: Registry) -> Iterable[dict[str, Any]]:
    for record in registry.acronyms:
        source = record.get("source") or {}
        text = f"{record.get('token', '')} = {record.get('expansion', '')}. {(record.get('ai') or {}).get('summary', '')}".strip()
        yield {
            "record_type": "acronym",
            "id": record.get("acronym_id"),
            "key": record.get("token"),
            "canonical": record.get("canonical"),
            "text": text,
            "domains": record.get("domains", []) or [],
            "status": record.get("status"),
            "source_id": source.get("source_id"),
            "source_authority": source.get("authority"),
        }
    for record in registry.concepts:
        aliases = record.get("aliases", []) or []
        text = f"{record.get('display_label', record.get('canonical', ''))}: {record.get('definition', '')}".strip()
        yield {
            "record_type": "concept",
            "id": f"acx:concept:{record.get('canonical')}:v1",
            "key": record.get("canonical"),
            "aliases": aliases,
            "text": text,
            "domains": record.get("domains", []) or [],
            "status": record.get("status"),
        }
    for record in registry.aliases:
        text = f"{record.get('alias')} -> {record.get('canonical') or record.get('candidates')}"
        yield {
            "record_type": "alias",
            "id": f"acx:alias:{slug_text(str(record.get('alias')))}:v1",
            "key": record.get("alias"),
            "canonical": record.get("canonical"),
            "candidates": record.get("candidates"),
            "text": text,
            "status": record.get("status"),
        }


def cmd_lookup(args: argparse.Namespace) -> int:
    registry = load_registry(Path(args.root).resolve())
    result = lookup_records(registry, args.token)
    print(json.dumps(result, indent=2, ensure_ascii=False))
    return 0


def cmd_export_jsonl(args: argparse.Namespace) -> int:
    registry = load_registry(Path(args.root).resolve())
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", encoding="utf-8") as f:
        for record in iter_index_records(registry):
            f.write(json.dumps(record, ensure_ascii=False, sort_keys=True) + "\n")
    print(f"wrote {output}")
    return 0


def cmd_pack(args: argparse.Namespace) -> int:
    registry = load_registry(Path(args.root).resolve())
    if args.text_file:
        text_path = Path(args.text_file)
        text = text_path.read_text(encoding="utf-8", errors="replace")
        source_name = str(text_path)
    else:
        text = sys.stdin.read()
        source_name = "stdin"
    pack = render_pack(registry, text, args.domain or [], source_name=source_name)
    if args.output:
        output = Path(args.output)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(pack, encoding="utf-8")
        print(f"wrote {output}")
    else:
        print(pack, end="")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="AeroCodex terminology lookup and AI-pack generator")
    parser.add_argument("--root", default=".", help="repository/package root")
    subparsers = parser.add_subparsers(dest="command", required=True)

    lookup = subparsers.add_parser("lookup", help="look up an acronym, alias, or canonical term")
    lookup.add_argument("token", help="token, alias, or canonical term to look up")
    lookup.set_defaults(func=cmd_lookup)

    pack = subparsers.add_parser("pack", help="generate an AI terminology pack from text")
    pack.add_argument("--text-file", help="text or Markdown file to read; stdin is used if omitted")
    pack.add_argument("--domain", action="append", default=[], help="domain hint; may be repeated")
    pack.add_argument("--output", help="write pack to file instead of stdout")
    pack.set_defaults(func=cmd_pack)

    export = subparsers.add_parser("export-jsonl", help="export a retrieval-friendly JSONL terminology index")
    export.add_argument("--output", required=True, help="output JSONL path")
    export.set_defaults(func=cmd_export_jsonl)
    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    try:
        return int(args.func(args))
    except RuntimeError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
