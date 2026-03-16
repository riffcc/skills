---
name: lean-prover
description: Expert in Lean 4 theorem proving and formal verification. Use when writing formal proofs, translating informal mathematics to Lean, debugging proof states, or choosing tactics and proof structure in Lean 4.
---

# lean-prover

Use this skill when the task is about Lean 4 proofs, formal verification, or theorem debugging.

## Core Abilities

- translate informal statements into Lean theorems
- choose proof strategies before writing tactics
- structure proofs clearly with `have`, `show`, `calc`, and local lemmas
- use mathlib effectively instead of reproving standard facts
- debug proof states, type errors, and tactic failures

## Default Working Pattern

1. Restate the goal in Lean terms.
2. Identify the proof shape:
   - direct proof
   - case split
   - contradiction
   - induction
   - witness construction
3. Sketch the proof before committing to tactics.
4. Build the proof incrementally.
5. Type-check and explain the result in plain language.

## Lean Bias

- Prefer constructive proofs when practical.
- Prefer explicit structure over giant opaque tactic blocks.
- Use existing library theorems when they simplify the proof honestly.
- Keep hypothesis names meaningful.
- Avoid leaving `sorry` in finished work.

## Good Fits

- proposition and predicate proofs
- induction over naturals, lists, or custom inductive types
- theorem translation from paper math to Lean
- proof repair after refactors
- finding the right tactic or lemma sequence

## Included Support Material

- `examples/` for worked proof samples
- `patterns/` for common proof structures
- `proofs/` and `tests/` for richer reference material

Start with `examples/` when the task needs a nearby model, and use the pattern notes when the real blocker is proof shape rather than syntax.
