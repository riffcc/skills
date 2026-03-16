# Project Copilot Shape

This note captures the emerging `riff-labs` public project-steering model.

## Role

`riff-labs` is a generalist specialist.

It should be able to:

- orient a new builder
- route them into the right project family or repo
- stay aware of project state through Plane
- surface likely next work without turning every session into project-management theater

The Plane loop should stay **inside** `riff-labs`, but enter through progressive discovery so the skill does not feel bloated.

## Core Internal Modules

The first internal modules to grow are:

- Orientation
- Plane loop
- Repo router

## Opening Behavior

When invoked cold, `riff-labs` should:

1. ask what the user needs
2. orient them quickly
3. route into the right project family, repo, or companion skill

It should optimize for **fast orientation**, grounded in:

- the public vault
- the real public repos

## Plane Loop Personality

The Plane loop should feel like a **steady project copilot**:

- almost always active
- constraint-aware
- quiet rather than theatrical
- project-aware without becoming overbearing

It should go deep enough in repos to ground action, then route into more specific workflows when needed.

## Compact Dashboard Priorities

The compact project dashboard should prioritize:

- likely next work
- blockers and drift
- cycle and milestone fit

## Ranking Logic

The ranking model should currently bias toward:

1. **cycle and milestone fit**
2. **momentum**

The suggestion surface should always include a **short rationale** explaining why an issue was surfaced.

## Dependency Handling

Dependencies should remain visible alongside actionable work.

When something cycle-critical is blocked, `riff-labs` should prefer to surface the **unblocker** first rather than only repeating the blocked issue.

## Onboarding Relationship

`riff-labs` should not absorb onboarding entirely.

Instead:

- `riff-labs` remains the orientation and routing layer
- `riff-onboarding` becomes the dedicated first-box first-session skill
- `riff-obsidian` is the documentation companion

## Practical Loop

The intended rhythm is:

`orient -> read project state -> surface likely next work -> ground it in the right repo -> continue`
