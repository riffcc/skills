#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MANIFEST="$ROOT/skills-manifest.json"
MODE="copy"

if [[ "${1:-}" == "--symlink" ]]; then
  MODE="symlink"
fi

if [[ ! -f "$MANIFEST" ]]; then
  echo "Missing manifest: $MANIFEST" >&2
  exit 1
fi

repo_version="$(python3 - <<'PY' "$MANIFEST"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    data = json.load(f)
print(data["version"])
PY
)"

mapfile -t manifest_skills < <(python3 - <<'PY' "$MANIFEST"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    data = json.load(f)
for skill in data["skills"]:
    print(skill)
PY
)

version_gt() {
  local left="$1"
  local right="$2"
  if [[ "$(printf '%s\n%s\n' "$left" "$right" | sort -V | tail -n1)" == "$left" && "$left" != "$right" ]]; then
    echo "1"
  else
    echo "0"
  fi
}

install_root() {
  local target_root="$1"
  local marker="$target_root/.skills-rewrite-version"

  mkdir -p "$target_root"

  if [[ -f "$marker" ]]; then
    installed_version="$(cat "$marker")"
    if [[ "$(version_gt "$installed_version" "$repo_version")" == "1" ]]; then
      echo "warning: $target_root already has newer version $installed_version (source is $repo_version)" >&2
    fi
  fi

  for skill_name in "${manifest_skills[@]}"; do
    skill_dir="$ROOT/$skill_name"
    if [[ ! -d "$skill_dir" ]]; then
      echo "warning: skipping missing skill directory $skill_dir" >&2
      continue
    fi
    # Skills already prefixed with riff- keep their name; others get the prefix
    if [[ "$skill_name" == riff-* ]]; then
      shortname="$skill_name"
    else
      shortname="riff-$skill_name"
    fi
    dest="$target_root/$shortname"
    rm -r "$dest" 2>/dev/null || true
    if [[ "$MODE" == "symlink" ]]; then
      ln -s "$skill_dir" "$dest"
    else
      cp -a "$skill_dir" "$dest"
    fi
    echo "installed $skill_name -> $dest"
    # Clean up stale unprefixed symlink from older installs
    if [[ "$shortname" != "$skill_name" ]]; then
      stale="$target_root/$skill_name"
      if [[ -L "$stale" ]]; then
        rm "$stale"
        echo "removed stale symlink $stale"
      elif [[ -d "$stale" ]]; then
        echo "warning: stale directory $stale exists, remove manually" >&2
      fi
    fi
  done

  printf '%s\n' "$repo_version" > "$marker"
}

install_root "$HOME/.codex/skills"
install_root "$HOME/.claude/skills"

echo "skills-rewrite $repo_version installed via $MODE mode"
