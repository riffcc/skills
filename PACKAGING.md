# Packaging Plan

This repo is moving toward a simple, boring release model:

- commits written with `Conventional Commits`
- versioned with `changesets`
- released with `Semantic Versioning`
- released by CI
- installable locally with `./update.sh`
- distributable as:
  - repo-wide releases
  - individual `.skill` artifacts
  - Debian packages

## Intended Artifact Types

### 1. Repo Release

Each repo release should:

- have a repo version
- include generated release notes from `changesets`
- attach build artifacts for the skills and updater packages

### 2. `.skill` Artifact

Each skill should be publishable as a standalone artifact.

Target shape:

- one artifact per skill per release
- includes:
  - `SKILL.md`
  - `references/` if present
  - `examples/` if present
  - `tests/` only when intentionally useful to downstream users
  - a tiny manifest with skill name and version

The exact file format can stay simple.
The important part is that it be:

- easy to publish
- easy to inspect
- easy to install

### 3. Updater `.deb`

The updater package should install:

- a small updater script
- a manifest location
- a cron or systemd timer that keeps the installed skill tree current

This package should update:

- `~/.codex/skills`
- `~/.claude/skills`

It should warn on downgrade and prefer safe upserts over destructive replacement.

### 4. Individual Skill `.deb`

Each public skill should also be installable as its own Debian package.

Package naming convention:

- `rifflabs-skill-<skill-name>`

Examples:

- `rifflabs-skill-riff-labs`
- `rifflabs-skill-couch-mode`
- `rifflabs-skill-lean-prover`

The package should:

- install into a shared local store
- then symlink into:
  - `~/.codex/skills/<shortname>`
  - `~/.claude/skills/<shortname>`

This keeps skill installation modular and makes selective installs easy.

### Naming Convention

Skills use a two-tier naming scheme:

- **Package name** (`.deb`): `rifflabs-skill-<bare-name>` — the full qualified package identifier
- **Shortname** (installed directory): `riff-<bare-name>` — what users see in `~/.claude/skills/`

Skills whose bare name already starts with `riff-` (e.g. `riff-labs`, `riff-obsidian`, `riff-onboarding`)
keep their name as-is for the shortname. All others get the `riff-` prefix added.

Examples:

| Repo directory | Package name | Installed as (shortname) |
|---|---|---|
| `couch-mode` | `rifflabs-skill-couch-mode` | `riff-couch-mode` |
| `lean-prover` | `rifflabs-skill-lean-prover` | `riff-lean-prover` |
| `riff-labs` | `rifflabs-skill-riff-labs` | `riff-labs` |

This makes installed skills easy to identify and uninstall — everything in `~/.claude/skills/` starting
with `riff-` came from this repo.

## CI/CD Responsibilities

CI/CD should eventually do all of the following:

1. validate the repo shape
2. validate Conventional Commit usage
3. validate skill manifests
4. run `changesets`
5. version the repo
6. build `.skill` artifacts
7. build updater `.deb`
8. build per-skill `.deb` packages
9. publish release assets

## First Practical Milestones

1. keep the repo content public-safe and flat
2. keep `update.sh` simple and reliable
3. define the `.skill` artifact format
4. define the updater package layout
5. automate artifact publishing in CI
