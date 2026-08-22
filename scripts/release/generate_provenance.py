#!/usr/bin/env python3
import argparse
import hashlib
import json
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def is_release_subject(path: Path) -> bool:
    return path.is_file() and not path.name.endswith(
        (".spdx.json", ".intoto.jsonl", "-SHA256SUMS")
    )


parser = argparse.ArgumentParser()
parser.add_argument("--dist", required=True)
parser.add_argument("--commit", required=True)
parser.add_argument("--run-id", required=True)
parser.add_argument("--repository", required=True)
parser.add_argument("--output", required=True)
args = parser.parse_args()

dist = Path(args.dist)
output = Path(args.output)
subjects = [
    {"name": path.name, "digest": {"sha256": sha256(path)}}
    for path in sorted(dist.iterdir(), key=lambda item: item.name)
    if is_release_subject(path)
]
statement = {
    "_type": "https://in-toto.io/Statement/v1",
    "subject": subjects,
    "predicateType": "https://slsa.dev/provenance/v1",
    "predicate": {
        "buildDefinition": {
            "buildType": "https://github.com/"
            + args.repository
            + "/.github/workflows/release.yml@refs/tags/v0.1.0-alpha.1",
            "externalParameters": {"ref": "refs/tags/v0.1.0-alpha.1"},
            "internalParameters": {"locked": True, "releaseProfile": True},
            "resolvedDependencies": [
                {
                    "uri": "git+https://github.com/" + args.repository + "@" + args.commit,
                    "digest": {"gitCommit": args.commit},
                }
            ],
        },
        "runDetails": {
            "builder": {"id": "https://github.com/" + args.repository + "/.github/workflows/release.yml"},
            "metadata": {"invocationId": "https://github.com/" + args.repository + "/actions/runs/" + args.run_id},
        },
    },
}
output.write_text(json.dumps(statement, sort_keys=True, separators=(",", ":")) + "\n", encoding="utf-8")
