#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$ROOT/skills-manifest.json"
VERSION="$(python3 - <<'PY' "$MANIFEST"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    print(json.load(f)["version"])
PY
)"
TAG="v$VERSION"
TITLE="$TAG - Skills for Days"
DIST_DIR="$ROOT/dist/$TAG"

if [[ ! -d "$DIST_DIR" ]]; then
  echo "Missing artifacts at $DIST_DIR" >&2
  exit 1
fi

gh release create "$TAG" "$DIST_DIR"/* \
  --repo riffcc/skills \
  --title "$TITLE" \
  --notes "First public skills release. Includes per-skill .skill artifacts, per-skill Debian packages, and the updater package."
