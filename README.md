# skills-rewrite

Public skill library for agentic coding, reasoning, review, and operations work.

This repo is intentionally flat:

- one top-level directory per skill
- one top-level `README.md`
- optional per-skill support material such as `references/`, `examples/`, or `tests/`

The goal is to keep every skill easy to browse, copy, remix, and publish without dragging around a larger framework project.

## Included Skills

- `couch-mode`
- `database-architecture`
- `infrastructure-deployment`
- `jetpack-developer`
- `lean-prover`
- `pal-commit`
- `p2p-engineer`
- `playwright-tester`
- `riff-onboarding`
- `riff-labs`
- `riff-obsidian`
- `storage-systems`

## Repo Rules

- Keep the repo public-safe.
- Prefer useful skills over clever ones.
- Do not import private operator modeling, internal company detail, or unstable lore.
- Do not import client, customer, or partner-specific knowledge into the core public skill set.
- Do not import pseudoscience, therapy material, or personality artifacts that are only meaningful in one private context.
- Rewrite skills as they land instead of dumping raw exports when a cleaner public version is easy to produce.
- Keep support files only when they materially improve the skill.

## Layout

Each skill should look like this:

```text
skill-name/
├── SKILL.md
├── references/        # optional
├── examples/          # optional
└── tests/             # optional
```

For imported skills that originally used different entrypoint names, normalize the public entrypoint to `SKILL.md`.

## Current Direction

This first flattening pass focuses on practical engineering skills:

- low-input collaboration
- deployment and infra automation
- theorem proving
- commit hygiene
- website auditing
- storage architecture
- public Riff context

The next passes should expand carefully, skill by skill, with the same public-safety filter.

## Install And Update

The repo ships with a simple top-level updater:

```bash
./update.sh
```

By default it will:

- upsert all top-level skills into `~/.codex/skills`
- upsert all top-level skills into `~/.claude/skills`
- write a version marker into each target root
- warn if the installed version is newer than the source you are trying to install

Use `--symlink` if you want to link the skill directories from this checkout instead of copying them.

## Release Direction

The release model for this repo is:

- `changesets` for versioning and release notes
- CI/CD to cut tagged releases
- packaged skill artifacts for individual skills
- a tiny updater package for machines that should stay in sync automatically

The initial repository scaffolding for that model lives in:

- `.changeset/`
- `.github/workflows/release.yml`
- `package.json`
- `skills-manifest.json`

This repo is still in the first packaging pass. The target shape is:

- one release flow for the whole repo
- individual skill artifacts per release
- Debian packages for the updater and for installable skill bundles
- straightforward local installation into `~/.codex/skills` and `~/.claude/skills`
