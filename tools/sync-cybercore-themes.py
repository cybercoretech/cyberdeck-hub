#!/usr/bin/env python3
"""Import canonical themes missing from the locally running Cyberdeck catalog."""

from __future__ import annotations

import json
import os
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
FRAMEWORK = ROOT.parents[1] / "framework" / "cybercore" / "schema" / "themes"
CANONICAL = ROOT.parents[1] / "web" / "cybercore-tech.github.io" / "data" / "themes"
KITTY = ROOT.parents[1] / "web" / "cybercore-control-plane" / "data" / "kitty-themes.json"
CONFIG = Path(os.environ.get("XDG_CONFIG_HOME", Path.home() / ".config")) / "cybercore"
THEMES = CONFIG / "themes"
GENERATED = CONFIG / ".cybercore-generated-theme-ids.json"

MASHUPS = (
    ("cc-chrome-horizon", "CHROME HORIZON", "cyberdyne-steel", "synthwave-outrun"),
    ("cc-skynet-plasma", "SKYNET PLASMA", "cyberdyne-skynet", "neosynth-plasma"),
    ("cc-midnight-mainframe", "MIDNIGHT MAINFRAME", "synthwave-sunset", "cyberpunk-neon"),
    ("cc-judgment-grid", "JUDGMENT GRID", "cyberdyne-judgment", "neosynth-laser-grid"),
    ("cc-dracula-tokyo", "DRACULA // TOKYO NIGHT", "kitty:dracula", "kitty:tokyo-night"),
    ("cc-catppuccin-nord", "CATPPUCCIN // NORD", "kitty:catppuccin-mocha", "kitty:nord"),
    ("cc-gruvbox-monokai", "GRUVBOX // MONOKAI", "kitty:gruvbox-dark", "kitty:monokai-pro"),
    ("cc-onyx-material", "ONE DARK // MATERIAL OCEAN", "kitty:one-dark", "kitty:material-ocean"),
    ("cc-kanagawa-rose", "KANAGAWA // ROSE PINE", "kitty:kanagawa-wave", "kitty:rose-pine-moon"),
    ("cc-evernight-fox", "EVERFOREST // NIGHTFOX", "kitty:everforest-dark", "kitty:nightfox"),
    ("cc-solarized-ayu", "SOLARIZED // AYU", "kitty:solarized-dark", "kitty:ayu"),
    ("cc-oxide-vesper", "OXOCARBON // VESPER", "kitty:oxocarbon", "kitty:vesper"),
)

SURFACE_KEYS = ("bg", "white", "panel", "line", "muted")
NEON_KEYS = ("acid_green", "hot_pink", "purple", "cyan", "orange", "red")


def palette_sources() -> dict[str, dict[str, str]]:
    sources = {
        path.stem: json.loads(path.read_text(encoding="utf-8"))
        for path in CANONICAL.glob("*/*.json")
    }
    kitty = json.loads(KITTY.read_text(encoding="utf-8"))
    sources.update({f"kitty:{name}": palette for name, palette in kitty["themes"].items()})
    return sources


def theme_document(
    theme_id: str,
    name: str,
    family: str,
    palette: dict[str, str],
) -> dict[str, object]:
    return {
        "format_version": 1,
        "metadata": {
            "id": theme_id,
            "name": name,
            "family": family,
            "author": "Cybercore",
        },
        "palette": palette,
    }
def main() -> None:
    sources = palette_sources()
    canonical = {file.stem: file for file in CANONICAL.glob("*/*.json")}
    builtin_ids = {file.stem for file in FRAMEWORK.glob("*/*.json")}
    generated = set(canonical) - builtin_ids
    generated.update(item[0] for item in MASHUPS)
    previous = set(json.loads(GENERATED.read_text())) if GENERATED.is_file() else set()
    THEMES.mkdir(parents=True, exist_ok=True)

    removed = 0
    for theme_id in previous | (set(canonical) & builtin_ids):
        path = THEMES / f"{theme_id}.json"
        if path.is_file():
            path.unlink()
            removed += 1

    for theme_id in sorted(set(canonical) - builtin_ids):
        source = canonical[theme_id]
        THEMES.joinpath(f"{theme_id}.json").write_text(
            json.dumps(
                theme_document(theme_id, theme_id.replace("-", " ").title(), source.parent.name, sources[theme_id]),
                indent=2,
            )
            + "\n",
            encoding="utf-8",
        )

    for theme_id, name, base_id, accent_id in MASHUPS:
        base = sources[base_id]
        accent = sources[accent_id]
        palette = {key: base[key] for key in SURFACE_KEYS}
        palette.update({key: accent[key] for key in NEON_KEYS})
        THEMES.joinpath(f"{theme_id}.json").write_text(
            json.dumps(theme_document(theme_id, name, "mashup", palette), indent=2) + "\n",
            encoding="utf-8",
        )

    GENERATED.write_text(json.dumps(sorted(generated), indent=2) + "\n", encoding="utf-8")
    print(
        f"{removed} obsolete generated themes removed; "
        f"{len(canonical)} source themes plus {len(MASHUPS)} mashups synchronized"
    )


if __name__ == "__main__":
    main()
