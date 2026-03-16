---
name: riff-jetpack-developer
description: Build and maintain Jetpack itself plus the deployment systems around it. Use when fixing Jetpack engine behavior, adding or refactoring roles, designing inventories or playbooks, or deploying software through Jetpack across Linux and macOS targets.
---

# jetpack-developer

Use this skill when the task touches any of these layers:

- Jetpack core behavior
- reusable deployment roles
- inventories, host vars, and playbooks
- deployment architecture expressed through Jetpack

## Core Stance

- Prefer reusable roles over one-off remote commands.
- Prefer modular playbooks over giant environment scripts.
- Prefer idempotent tasks with checks, assertions, or probes.
- Keep Jetpack target-agnostic when possible.
- Treat deployment automation as product surface, not disposable glue.

## Boundary Rules

Choose the right layer before editing:

- **Jetpack core** for parser behavior, task semantics, module behavior, and tool ergonomics
- **reusable roles** for shared OS, runtime, and tooling installs
- **deploy glue** for project-specific wiring, inventory, and composition

If the problem is systemic, fix Jetpack.  
If the problem is reusable, fix a role.  
If the problem is environment-specific, fix inventory or playbook wiring.

## Working Pattern

1. Inspect the current playbook, role tree, inventory, and vars.
2. Identify the correct layer.
3. Implement the smallest correct change.
4. Add assertions or verification when certainty matters.
5. Validate locally first with targeted Jetpack tests.
6. Apply remotely through Jetpack rather than by hand unless a one-off intervention is explicitly wanted.
7. Verify the remote result directly.

## Jetpack Rules

- Quote user-controlled shell fragments carefully.
- Redesign tasks instead of trying to sneak around command validation.
- Keep installs safe to reapply.
- Express PATH and shell environment fixes through managed config, not lucky interactive state.
- Encode platform assumptions in vars, roles, or inventory instead of shell fragments.

## Good Outcome

A good Jetpack change:

- fixes the real engine/role/playbook boundary cleanly
- remains safe to rerun
- improves the next deployment instead of adding more magic
