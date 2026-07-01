## Improvement Notes

### Version 1 (2025-11-05)

Initial creation as a formal reasoning skill intended to work well outside the author's primary engineering domain.

**Bootstrap Context:**
- Created as a test case for whether a skill can become genuinely useful in a domain that demands real symbolic reasoning rather than shallow pattern matching.
- Lean 4 formal theorem proving chosen because:
  - Objectively measurable (proofs either compile or don't)
  - Requires deep understanding of logic and type theory
  - Has established benchmarks (IMO problems, Putnam, mathlib formalization)
  - Outside typical software engineering domain
- Key requirements: Write valid Lean proofs, translate informal math to formal statements, select appropriate tactics, verify theorems

**Domain Research Sources:**
- "Theorem Proving in Lean 4" (official reference by Avigad, de Moura, Kong, Ullrich)
- Lean 4 Tactic Reference (lean-lang.org)
- Lean community learning resources (leanprover-community.github.io)
- Mathematics in Lean tutorial
- Lean 4 tactics cheatsheet (October 2025 version)

**Expected Use Cases:**
1. Formal verification of mathematical theorems
2. Translation of informal proofs to machine-checkable Lean code
3. Interactive theorem proving assistance
4. Education: teaching formal methods and proof techniques
5. Research: formalizing competition mathematics (IMO, Putnam)

**Success Criteria:**
- Can write syntactically correct Lean 4 proofs
- Can explain proof strategies and tactic choices
- Can translate informal mathematical statements to formal Lean
- Can identify when classical vs constructive logic is needed
- Can leverage mathlib effectively

**Next Steps:**
- Benchmark validation (structure + Lean 4 domain accuracy)
- Test on simple theorems (propositional logic, natural number arithmetic)
- Test on moderate theorems (requiring induction, case analysis)
- Test on competition problems (AMC, AIME level)
- Iterative improvement based on proof success rate

### Version 2 (2026-07-01) — Proof Honesty: sorry / axiom / theorem discipline

Encoded the distinction between `sorry` (hole), `axiom` (asserted assumption / proof debt), and `theorem` (proven). The skill previously said only "avoid `sorry`," which let a deeper gap pass undetected. Drivers:

- The Citadel topology proofs compile with **zero `sorry`** but assert their load-bearing properties (`greedy_progress`, `diameter_bound`, `global_knowledge_from_local_sync`) as `axiom` — and the "theorems" around some of them just discharge a hypothesis. A sorry-only lint falsely stamps these "proven."
- The Provisioning Barrier research (`jetpack/docs/research/provisioning-barrier`) deliberately targets **0-axiom** proofs and treats fair-lossy as a *stated liveness hypothesis*, not a theorem and not an axiom in disguise — the posture this skill now codifies.

New rules added to SKILL.md: a load-bearing property must be a `theorem`; report sorry-count **and** the axiom set via `#print axioms`; published claims name sorry/axiom/theorem per statement; safety is proven unconditionally, liveness under an explicit fair-lossy hypothesis. This is what makes "formally verified" auditable rather than performative, and what the Riff docs integrity spine (claims-vs-proofs CI) needs to enforce.
