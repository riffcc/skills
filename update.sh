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

  while IFS= read -r skill_dir; do
    skill_name="$(basename "$skill_dir")"
    dest="$target_root/$skill_name"
    rm -rf "$dest"
    if [[ "$MODE" == "symlink" ]]; then
      ln -s "$skill_dir" "$dest"
    else
      cp -a "$skill_dir" "$dest"
    fi
    echo "installed $skill_name -> $dest"
  done < <(find "$ROOT" -mindepth 1 -maxdepth 1 -type d ! -name ".git" ! -name ".github" ! -name ".changeset")

  printf '%s\n' "$repo_version" > "$marker"
}

install_root "$HOME/.codex/skills"
install_root "$HOME/.claude/skills"

echo "skills-rewrite $repo_version installed via $MODE mode"
