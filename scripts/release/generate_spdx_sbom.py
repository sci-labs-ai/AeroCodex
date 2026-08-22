#!/usr/bin/env python3
import argparse
import datetime
import hashlib
import json
from pathlib import Path


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def sha1(path: Path) -> str:
    digest = hashlib.sha1()
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
parser.add_argument("--epoch", required=True, type=int)
parser.add_argument("--output", required=True)
args = parser.parse_args()

dist = Path(args.dist)
output = Path(args.output)
files = []
relationships = [
    {
        "spdxElementId": "SPDXRef-DOCUMENT",
        "relationshipType": "DESCRIBES",
        "relatedSpdxElement": "SPDXRef-Package-AeroCodex",
    }
]
file_sha1s = []
subjects = [
    path for path in sorted(dist.iterdir(), key=lambda item: item.name) if is_release_subject(path)
]
for index, path in enumerate(subjects, start=1):
    spdx_id = f"SPDXRef-File-{index}"
    file_sha1 = sha1(path)
    file_sha1s.append(file_sha1)
    files.append(
        {
            "SPDXID": spdx_id,
            "fileName": path.name,
            "checksums": [
                {"algorithm": "SHA1", "checksumValue": file_sha1},
                {"algorithm": "SHA256", "checksumValue": sha256(path)},
            ],
            "licenseConcluded": "NOASSERTION",
            "copyrightText": "NOASSERTION",
        }
    )
    relationships.append(
        {
            "spdxElementId": "SPDXRef-Package-AeroCodex",
            "relationshipType": "CONTAINS",
            "relatedSpdxElement": spdx_id,
        }
    )

created = datetime.datetime.fromtimestamp(args.epoch, datetime.timezone.utc).isoformat().replace("+00:00", "Z")
verification_code = hashlib.sha1("".join(sorted(file_sha1s)).encode("ascii")).hexdigest()
document = {
    "spdxVersion": "SPDX-2.3",
    "dataLicense": "CC0-1.0",
    "SPDXID": "SPDXRef-DOCUMENT",
    "name": "AeroCodex-0.1.0-alpha.1-release",
    "documentNamespace": f"https://github.com/sci-labs-ai/AeroCodex/releases/sbom/{args.commit}",
    "creationInfo": {"created": created, "creators": ["Tool: AeroCodex-release-sbom-v1"]},
    "packages": [
        {
            "SPDXID": "SPDXRef-Package-AeroCodex",
            "name": "AeroCodex",
            "versionInfo": "0.1.0-alpha.1",
            "downloadLocation": "NOASSERTION",
            "filesAnalyzed": True,
            "packageVerificationCode": {
                "packageVerificationCodeValue": verification_code
            },
            "licenseConcluded": "MIT OR Apache-2.0",
            "licenseDeclared": "MIT OR Apache-2.0",
            "copyrightText": "NOASSERTION",
            "externalRefs": [
                {
                    "referenceCategory": "PACKAGE-MANAGER",
                    "referenceType": "purl",
                    "referenceLocator": "pkg:github/sci-labs-ai/AeroCodex@" + args.commit,
                }
            ],
        }
    ],
    "files": files,
    "relationships": relationships,
}
output.write_text(json.dumps(document, indent=2, sort_keys=True) + "\n", encoding="utf-8")
