---
name: riff-labs
description: Public Riff Labs builder skill for mixed human+AI work, centered on project knowledge, source models, constraints, and reasoning.
version: 0.2.3
created: 2026-03-13
updated: 2026-03-14
---

# riff-labs

This is the public Riff Labs builder skill. It exists to help mixed human+AI builders understand enough about Riff to build well without depending on Wings to restate the shape every time.

It is not a generic coding persona pack. The center of gravity should be Riff project knowledge, system relationships, source models, constraints, and reasoning in a public-safe form.

The first real public model in this rewrite covers `riff.cc`, `Flagship`, `Citadel`, `Neverust`, `Dragonfly`, `Jetpack`, `Palace`, and `Lagoon`, plus the operating documents that explain how those pieces are meant to be built together. In that model, `Codex` is the human-facing agent interface; `Palace` is a callable tool layer underneath it, not the thing humans should have to drive directly.

Version one should optimize first for new staff who need to contribute without depending on private lore. The public surface should help someone start well, inspect the real repos, and understand how the `riff.cc` stack fits together before it tries to become an exhaustive company handbook.

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
- Inspect the real substrate before generalizing from memory, stale docs, or architecture lore.
- Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) in Riff Labs repos.
- Treat [Changesets](https://github.com/changesets/changesets) as the release/versioning layer that pairs with Conventional Commits, not as a separate optional system.
- Use [Semantic Versioning](https://semver.org/) for published versions and release expectations.
- Prefer working branches in the form `TICKET-ID/ghusername/topic-name` so ownership and intent stay visible to humans and agents.
- Treat `dev`, `staging`, `main`, and `release` as promotion streams with increasing stability rather than as interchangeable long-lived dumping grounds.
- Treat `main` as a blessed branch that should be safe to build from at any time, not as a casual integration branch.
- Let branches be opened eagerly by both humans and agents, but require stronger verification as work graduates upward through the promotion streams.
- Allow polite history cleanup over time when that keeps the history legible and closer to the real work that happened.
- Keep the skill flexible and model-driven rather than rigid and task-taxonomic.
- In skill authoring, progressive discovery means modular navigation through `SKILL.md` and reference files, not behavior drift.
- In low-input or likely voice-transcription contexts, normalize likely transcript variants such as `rift` to `Riff` when the project/company context makes the intended reference clear.
- Compress hard without stripping away the reasoning needed to generalize.
- Do not import private operator modeling, private company detail, or lore into the public artifact.

## Version One Bias

- Prioritize builder onboarding over broad narration.
- Treat contribution defaults as the highest-priority missing surface.
- Use a compact `Start Here` route into `How We Work` and `Repo Map` rather than opening with a giant project encyclopedia.
- Keep `riff.cc`, `Flagship`, `Citadel`, and `Dragonfly` explicitly visible in the public model even when the `riff.cc` family is the first-class focus.
- Let project pages branch from the front door only after the contribution path is legible.

## Machine-Native Delivery

- Open active work on branches named `TICKET-ID/ghusername/topic-name` so ownership and purpose are visible immediately.
- Agents and humans should eagerly create the right ticket branch rather than piling anonymous work into shared long-lived branches.
- Stale ticket branches are not sacred: they can be finished by another human or agent, or reaped if the underlying work no longer matters.
- Prefer zipping related issue branches upward into larger review units when that clarifies momentum, such as cycle, module, sprint, or milestone PRs.
- Treat branch promotion as a trust ladder:
  - `dev` is the active integration stream and usually the right place to start from
  - `staging` is for commits that appear to have passed verification and are being vetted through stronger checks
  - `main` is the trusted branch and should be safe to build from at any time
  - `release` is for packaging, versioning, and publication work
- Use [Changesets](https://github.com/changesets/changesets) and [Semantic Versioning](https://semver.org/) to make release intent legible instead of hiding it in ad hoc branch rituals.
- Encode these expectations into CI/CD and repository policy where possible so the machine-native workflow reinforces trust instead of eroding it.

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
