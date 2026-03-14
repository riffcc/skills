# Engineering Defaults

This file captures operating constraints and assumptions for the public `Riff Labs` skill. It is not meant to become a brittle rulebook.

## Constraint Bias

- prefer constraints over rules
- prefer reasoning over examples
- add specificity only when it meaningfully improves action or avoids repeated failure

## Progressive Discovery

Prefer progressive discovery over giant context loads.

For this rewrite, progressive discovery is also a skill-authoring rule:

- keep `SKILL.md` compact
- route to focused reference files
- load only what the current task needs
- avoid behavior drift caused by modularization alone

## Operating Assumptions

- start from the relevant project or system and the real substrate
- keep the model compact enough to navigate quickly
- use specs when they clarify multi-part or ambiguous work
- keep artifacts legible enough for another human or agent to continue
- treat Riff as parallel streams sharing one architecture, not as a single queue to serialize
- keep intent and rationale in the vault, execution in Plane, and human-facing agent work centered on Codex
- keep verification automated where possible and keep progress visible rather than trapped in one operator's head

## Systems Of Engineering

The operating stack to preserve is:

- Obsidian for intent, rationale, and architectural memory
- Plane for execution visibility and task state
- Codex as the primary human-facing agent interface
- Palace as an optional tool surface Codex may call while useful capabilities migrate over time
- automated testing as a default expectation rather than an optional afterthought
- open team communication that keeps blockers visible instead of private

This matters because Riff is trying to scale orchestration, not shrink ambition.

## Scope Guard

This file should remain constraint-shaped. If content starts reading like a generic coding checklist, it has drifted.
