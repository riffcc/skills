#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$ROOT/skills-manifest.json"
DIST_ROOT="$ROOT/dist"
BUILD_ROOT="$ROOT/build"

version="$(python3 - <<'PY' "$MANIFEST"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    print(json.load(f)["version"])
PY
)"

mapfile -t skills < <(python3 - <<'PY' "$MANIFEST"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    data = json.load(f)
for skill in data["skills"]:
    print(skill)
PY
)

DIST_DIR="$DIST_ROOT/v$version"
rm -rf "$DIST_DIR" "$BUILD_ROOT"
mkdir -p "$DIST_DIR" "$BUILD_ROOT"

version_gt() {
  local left="$1"
  local right="$2"
  if [[ "$(printf '%s\n%s\n' "$left" "$right" | sort -V | tail -n1)" == "$left" && "$left" != "$right" ]]; then
    echo "1"
  else
    echo "0"
  fi
}

write_control() {
  local path="$1"
  local package_name="$2"
  local description="$3"
  cat > "$path" <<EOF
Package: $package_name
Version: $version
Section: utils
Priority: optional
Architecture: all
Maintainer: Riff Labs <opensource@riff.cc>
Description: $description
EOF
}

for skill in "${skills[@]}"; do
  skill_src="$ROOT/$skill"
  if [[ ! -d "$skill_src" ]]; then
    echo "Missing skill directory: $skill_src" >&2
    exit 1
  fi

  # Skills already prefixed with riff- keep their name; others get the prefix
  if [[ "$skill" == riff-* ]]; then
    shortname="$skill"
  else
    shortname="riff-$skill"
  fi

  artifact_base="rifflabs-skill-$skill"
  skill_stage="$BUILD_ROOT/$artifact_base-skill"
  mkdir -p "$skill_stage"
  cp -a "$skill_src/." "$skill_stage/"
  cat > "$skill_stage/skill.json" <<EOF
{
  "name": "$shortname",
  "version": "$version",
  "package": "$artifact_base"
}
EOF
  (
    cd "$skill_stage"
    zip -qr "$DIST_DIR/$artifact_base-$version.skill" .
  )

  pkg_root="$BUILD_ROOT/$artifact_base-deb"
  mkdir -p "$pkg_root/DEBIAN" "$pkg_root/usr/share/rifflabs-skills/skills/$shortname"
  cp -a "$skill_src/." "$pkg_root/usr/share/rifflabs-skills/skills/$shortname/"
  cat > "$pkg_root/usr/share/rifflabs-skills/skills/$shortname/skill.json" <<EOF
{
  "name": "$shortname",
  "version": "$version",
  "package": "$artifact_base"
}
EOF
  write_control "$pkg_root/DEBIAN/control" "$artifact_base" "Riff Labs public skill: $shortname"
  cat > "$pkg_root/DEBIAN/postinst" <<EOF
#!/usr/bin/env bash
set -euo pipefail
SKILL_PATH="/usr/share/rifflabs-skills/skills/$shortname"
for home in /home/*; do
  [[ -d "\$home" ]] || continue
  user="\$(basename "\$home")"
  for root in "\$home/.codex/skills" "\$home/.claude/skills"; do
    mkdir -p "\$root"
    ln -sfn "\$SKILL_PATH" "\$root/$shortname"
    chown -h "\$user:\$user" "\$root/$shortname" 2>/dev/null || true
  done
done
EOF
  cat > "$pkg_root/DEBIAN/prerm" <<EOF
#!/usr/bin/env bash
set -euo pipefail
SKILL_PATH="/usr/share/rifflabs-skills/skills/$shortname"
for home in /home/*; do
  [[ -d "\$home" ]] || continue
  for root in "\$home/.codex/skills" "\$home/.claude/skills"; do
    link="\$root/$shortname"
    if [[ -L "\$link" && "\$(readlink -f "\$link")" == "\$SKILL_PATH" ]]; then
      rm -f "\$link"
    fi
  done
done
EOF
  chmod 0755 "$pkg_root/DEBIAN/postinst" "$pkg_root/DEBIAN/prerm"
  dpkg-deb --root-owner-group --build "$pkg_root" "$DIST_DIR/${artifact_base}_${version}_all.deb" >/dev/null
done

updater_pkg="rifflabs-skills-updater"
updater_root="$BUILD_ROOT/$updater_pkg-deb"
mkdir -p "$updater_root/DEBIAN" "$updater_root/usr/local/bin" "$updater_root/etc/cron.daily" "$updater_root/var/lib/rifflabs-skills"
write_control "$updater_root/DEBIAN/control" "$updater_pkg" "Updater for Riff Labs public skills packages"
cat > "$updater_root/usr/local/bin/rifflabs-skills-auto-update" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

STATE_DIR="/var/lib/rifflabs-skills"
STATE_FILE="$STATE_DIR/current-version"
REPO="riffcc/skills"
API_URL="https://api.github.com/repos/$REPO/releases/latest"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

mkdir -p "$STATE_DIR"

version_gt() {
  local left="$1"
  local right="$2"
  if [[ "$(printf '%s\n%s\n' "$left" "$right" | sort -V | tail -n1)" == "$left" && "$left" != "$right" ]]; then
    echo "1"
  else
    echo "0"
  fi
}

current_version="0.0.0"
if [[ -f "$STATE_FILE" ]]; then
  current_version="$(cat "$STATE_FILE")"
fi

python3 - <<'PY' "$API_URL" "$TMP_DIR/release.json"
import sys, urllib.request
url, out = sys.argv[1], sys.argv[2]
with urllib.request.urlopen(url) as resp:
    open(out, "wb").write(resp.read())
PY

latest_tag="$(python3 - <<'PY' "$TMP_DIR/release.json"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    data = json.load(f)
print(data["tag_name"].lstrip("v"))
PY
)"

if [[ "$(version_gt "$latest_tag" "$current_version")" != "1" ]]; then
  exit 0
fi

python3 - <<'PY' "$TMP_DIR/release.json" "$TMP_DIR/urls.txt"
import json, sys
with open(sys.argv[1], "r", encoding="utf-8") as f:
    data = json.load(f)
with open(sys.argv[2], "w", encoding="utf-8") as out:
    for asset in data.get("assets", []):
        name = asset.get("name", "")
        if name.startswith("rifflabs-skill-") and name.endswith(".deb"):
            out.write(asset["browser_download_url"] + "\n")
PY

while IFS= read -r url; do
  [[ -n "$url" ]] || continue
  file="$TMP_DIR/$(basename "$url")"
  curl -fsSL "$url" -o "$file"
  dpkg -i "$file"
done < "$TMP_DIR/urls.txt"

printf '%s\n' "$latest_tag" > "$STATE_FILE"
EOF
chmod 0755 "$updater_root/usr/local/bin/rifflabs-skills-auto-update"
cat > "$updater_root/etc/cron.daily/rifflabs-skills-update" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
/usr/local/bin/rifflabs-skills-auto-update
EOF
chmod 0755 "$updater_root/etc/cron.daily/rifflabs-skills-update"
printf '%s\n' "$version" > "$updater_root/var/lib/rifflabs-skills/current-version"
dpkg-deb --root-owner-group --build "$updater_root" "$DIST_DIR/${updater_pkg}_${version}_all.deb" >/dev/null

(cd "$DIST_DIR" && sha256sum * > SHA256SUMS)
printf 'Built release artifacts in %s\n' "$DIST_DIR"
