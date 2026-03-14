---
name: riff-labs
description: Public Riff Labs builder skill for mixed human+AI work, centered on project knowledge, source models, constraints, and reasoning.
version: 0.2.1
created: 2026-03-13
updated: 2026-03-13
---

# riff-labs

This is the public Riff Labs builder skill. It exists to help mixed human+AI builders understand enough about Riff to build well without depending on Wings to restate the shape every time.

It is not a generic coding persona pack. The center of gravity should be Riff project knowledge, system relationships, source models, constraints, and reasoning in a public-safe form.

The first real public model in this rewrite covers `riff.cc`, `Flagship`, `Citadel`, `Neverust`, `Dragonfly`, `Jetpack`, `Palace`, and `Lagoon`, plus the operating documents that explain how those pieces are meant to be built together. In that model, `Codex` is the human-facing agent interface; `Palace` is a callable tool layer underneath it, not the thing humans should have to drive directly.

## Core Pattern

Use this sequence:

`orient -> identify the relevant Riff project or system -> load the minimum useful model -> inspect the real substrate -> reason from constraints -> build`

The goal is not just to generate code. The goal is to make building decisions more legible, grounded, and easier to continue across humans and agents.

## Load Order

Read only the references needed for the current task.

1. `references/project-knowledge.md`
2. `references/repo-plane-map.md`
3. `references/tooling-workflow.md`
4. `references/engineering-defaults.md`
5. `references/review-heuristics.md`
6. `references/public-obsidian.md`

## Non-Negotiables

- Center Riff knowledge and Riff constraints, not generic execution theater.
- Prefer constraints over rules.
- Prefer reasoning over examples.
- Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) in Riff Labs repos.
- Treat [Changesets](https://github.com/changesets/changesets) as the release/versioning layer that pairs with Conventional Commits, not as a separate optional system.
- Use [Semantic Versioning](https://semver.org/) for published versions and release expectations.
- Prefer working branches in the form `TICKET-ID/ghusername/topic-name` so ownership and intent stay visible to humans and agents.
- Treat `dev`, `staging`, `main`, and `release` as promotion streams with increasing stability rather than as interchangeable long-lived dumping grounds.
- Allow polite history cleanup over time when that keeps the history legible and closer to the real work that happened.
- Keep the skill flexible and model-driven rather than rigid and task-taxonomic.
- In skill authoring, progressive discovery means modular navigation through `SKILL.md` and reference files, not behavior drift.
- In low-input or likely voice-transcription contexts, normalize likely transcript variants such as `rift` to `Riff` when the project/company context makes the intended reference clear.
- Compress hard without stripping away the reasoning needed to generalize.
- Do not import private operator modeling, private company detail, or lore into the public artifact.

## Current Scope

This first rewrite pass is still intentionally narrow, but it is no longer just an empty scaffold:

- the modular reference structure exists now
- the first public-safe project model centers on the `riff.cc` family, the infrastructure family, and the storage family
- research and publicity layers are intentionally excluded from the first public pass unless needed to explain a public dependency
- internal operating detail is intentionally excluded unless it is genuinely needed for public builders to act well
- sensitive or unstable status narrative should not be treated as public skill knowledge
- examples and templates are still deferred
- the public vault should expand only through small reviewed changes that do not outrun the skill
- cross-repo conventions now include branch naming, promotion streams, Conventional Commits, Changesets, and Semantic Versioning
