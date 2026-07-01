---
name: riff-lean-prover
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
6. Verify honestly — `#print axioms` + sorry-count; report sorry/axiom/theorem per load-bearing claim (see Proof Honesty).

## Lean Bias

- Prefer constructive proofs when practical.
- Prefer explicit structure over giant opaque tactic blocks.
- Use existing library theorems when they simplify the proof honestly.
- Keep hypothesis names meaningful.
- Never leave `sorry` in finished work, and treat `axiom` with equal care — see Proof Honesty below.

## Proof Honesty: sorry / axiom / theorem

A result is only as honest as its weakest step. Three categories, never conflated:

- **`theorem`** — actually proven. The only category that earns the word "proven."
- **`sorry`** — a hole. Never in finished work.
- **`axiom`** — an asserted assumption. Legitimate when it models a real-world hypothesis you want visible (a channel model, a fairness assumption) or carries a standard math axiom (`propext`, `Quot.sound`). **But every axiom is a proof debt:** it must be named, justified in prose, and counted. An axiom that asserts the load-bearing property you set out to prove is a gap dressed as a proof — downgrade the claim to "assumed" or "conjectured," never assert it silently.

Working rules:

- Aim for **0-sorry, minimal-axiom**; the strongest result is **0-axiom, constructive** (only standard `propext`/`Quot.sound`-tier, or nothing).
- A load-bearing property — the thing a paper or doc claims — must be a `theorem`. If it sits as an `axiom`, the "theorems" around it are wrappers over an assertion; say so.
- Before reporting a result as proven, run `#print axioms <name>` and report both the **sorry-count** and the **axiom set**. Published claims must name which category each load-bearing statement is in.
- A real channel's liveness/reliability is a *stated hypothesis*, not a theorem and not an axiom in disguise: prove safety unconditionally; prove liveness *under* the fair-lossy (or equivalent) hypothesis, explicitly.

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
