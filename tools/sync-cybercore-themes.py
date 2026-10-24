#!/usr/bin/env python3
"""Import canonical themes missing from the locally running Cyberdeck catalog."""

from __future__ import annotations

import json
import os
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CANONICAL = ROOT.parents[1] / "framework" / "cybercore" / "schema" / "themes"
CONFIG = Path(os.environ.get("XDG_CONFIG_HOME", Path.home() / ".config")) / "cybercore"
THEMES = CONFIG / "themes"
GENERATED = CONFIG / ".cybercore-generated-theme-ids.json"
PORT = os.environ.get("CYBERDECK_PORT", "8080")


def live_builtin_ids() -> set[str]:
    url = f"http://127.0.0.1:{PORT}/api/cybergrid/themes"
    request = urllib.request.Request(url, headers={"User-Agent": "cybercore-update/1"})
    with urllib.request.urlopen(request, timeout=5) as response:
        catalog = json.load(response)
    return {theme["id"] for theme in catalog["themes"] if theme["builtin"]}


def theme_document(source: Path) -> dict[str, object]:
    palette = json.loads(source.read_text(encoding="utf-8"))
    return {
        "format_version": 1,
        "metadata": {
            "id": source.stem,
            "name": source.stem.replace("-", " ").title(),
            "family": source.parent.name,
            "author": "Cybercore",
        },
        "palette": palette,
    }


def main() -> None:
    builtin_ids = live_builtin_ids()
    canonical = {file.stem: file for file in CANONICAL.glob("*/*.json")}
    previous = set(json.loads(GENERATED.read_text())) if GENERATED.is_file() else set()
    THEMES.mkdir(parents=True, exist_ok=True)

    for theme_id in previous - set(canonical) | (previous & builtin_ids):
        path = THEMES / f"{theme_id}.json"
        if path.is_file():
            path.unlink()

    imported = 0
    skipped = 0
    for theme_id, source in sorted(canonical.items()):
        destination = THEMES / f"{theme_id}.json"
        if theme_id in builtin_ids:
            continue
        if destination.exists() and theme_id not in previous:
            skipped += 1
            continue
        destination.write_text(
            json.dumps(theme_document(source), indent=2) + "\n",
            encoding="utf-8",
        )
        imported += 1

    generated = sorted(set(canonical) - builtin_ids - {theme_id for theme_id in canonical if (THEMES / f"{theme_id}.json").exists() and theme_id not in previous})
    GENERATED.write_text(json.dumps(generated, indent=2) + "\n", encoding="utf-8")
    print(f"{imported} local themes imported, {skipped} existing custom themes preserved")


if __name__ == "__main__":
    main()
