# Tooling Workflow

Use tools to reduce ambiguity, not to produce noise.

## Source Model

For the public `Riff Labs` skill, the tooling layer is mainly a source model for learning Riff quickly.

- identify the relevant project or system first
- then inspect the primary repo
- then inspect the strongest public-safe note or document
- then pull in adjacent projects only when the relationships matter

Do not let this turn into a rigid task taxonomy.

## First Notes To Read

When orienting on Riff itself rather than one specific repo, start here:

1. `Riff Labs Thesis.md`
2. `Command/Operating Philosophy.md`
3. `Command/SoE Agentic Development.md`
4. `Immortal Infrastructure.md` when the infra/commercial stack matters

Then move into the relevant project note under `Projects/`.

## Family Routing

If the work is about the public platform and why it exists, go next to:

- `Projects/riff.cc/riff.cc.md`
- `Projects/Flagship/Flagship.md`
- `Projects/Librarian.md`
- `Projects/Content Curation.md`
- `Projects/riff.cc/Defederation Model.md`

If the work is about the P2P and storage substrate, go next to:

- `Projects/Citadel/Citadel.md`
- `Projects/Citadel/Citadel Architecture.md`
- `Projects/Neverust/Neverust.md`

If the work is about commercial infrastructure and deployment, go next to:

- `Projects/Dragonfly/Dragonfly.md`
- `Projects/Jetpack.md`
- `Immortal Infrastructure.md`

If the work is about how Riff is meant to ship across many projects, go next to:

- `Command/Operating Philosophy.md`
- `Command/SoE Agentic Development.md`
- `Projects/Palace/Palace.md` for legacy/transitional capability context

## Project Note Routing

The top-level project notes are the first landing points:

- `Projects/riff.cc/riff.cc.md`
- `Projects/Flagship/Flagship.md`
- `Projects/Citadel/Citadel.md`
- `Projects/Neverust/Neverust.md`
- `Projects/Dragonfly/Dragonfly.md`
- `Projects/Jetpack.md`
- `Projects/Palace/Palace.md`
- `Projects/Lagoon.md`

After the top-level project page, use the architecture and development sub-pages named inside that page rather than wandering blindly through the vault.

## First Repo Set

The first public repo set to ground against is:

- `riffcc/flagship`
- `riffcc/dragonfly`
- `riffcc/jetpack`
- `riffcc/neverust`
- `riffcc/citadel`
- `riffcc/librarian`

These repos cover the first public pass across the product, infrastructure, and storage families. They are the fastest way to make the public skill and the public vault useful for staff onboarding without pretending the whole portfolio needs to be modeled at once.

## DeepWiki Use

Use DeepWiki when it improves orientation on one of the first-pass repos.

- `riffcc/flagship`, `riffcc/dragonfly`, `riffcc/jetpack`, `riffcc/neverust`, and `riffcc/citadel` already have usable DeepWiki structure
- `riffcc/librarian` currently needs indexing before DeepWiki can be used as a source model for it

If DeepWiki is unavailable or a repo is not indexed yet, say so plainly and fall back to the strongest local public-safe note or the real repository substrate.

## Project Switching

Project switching still matters, but only as a way to reach the right substrate fast.

- prefer real local folders over remembered names
- keep shortlists compact and legible
- continue discovery inside the chosen project instead of speculating across all of them

## Skill Authoring

- keep `SKILL.md` as the router
- move substantive knowledge into focused reference files
- avoid monolithic lore documents
- favor structure that is easy to extend without changing behavior accidentally

## Source Of Truth

For now, the skill itself is the sharpest public artifact.

The public Obsidian vault may become a parallel source later, but it should not outrun the skill.

## Repo Conventions

Across Riff Labs repos:

- use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for commit messages
- use [Changesets](https://github.com/changesets/changesets) for versioning and release-note generation where packaging or releases matter
- use [Semantic Versioning](https://semver.org/) for released versions
- open work on branches named `[TICKET-ID]/ghusername/topic-name`

Treat these as linked conventions:

- Conventional Commits make change intent easy to scan
- Changesets make release intent and version bumps explicit
- Semantic Versioning makes compatibility expectations legible

When both apply, prefer both.

## Branch And Stream Model

Use ticket branches as the default unit of active work:

- one branch per Plane issue or tightly related work item
- branch ownership is visible in the name
- agents and humans should eagerly open the appropriate ticket branch rather than doing anonymous work on shared branches

Prefer zipping related work upward into larger review units when that improves clarity:

- related issue branches can be bundled into cycle, module, sprint, or milestone PRs
- that bundling should clarify momentum, not hide the underlying work items

Promotion streams should be treated as stability levels:

- `dev`: active integration stream; generally the right place to start from, but not guaranteed stable
- `staging`: changes that appear verified and are being vetted through stronger checks
- `main`: trustworthy branch that should be safe to build from at any time
- `release`: release preparation, packaging, and version publication work

These streams are for graduating work upward, not for replacing ticket branches.

## Stale Branches And History

Stale ticket branches are recoverable, not sacred:

- if a branch stops moving and the underlying issue still matters, another human or agent may finish it
- if the issue no longer matters, the branch can be reaped
- if cleanup makes the history easier to understand, history may be rewritten politely and deliberately

The goal is visible ownership, reduced duplicate work, and a version history that reflects the actual development arc.
