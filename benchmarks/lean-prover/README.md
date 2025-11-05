# Lean Prover Benchmark Test Suite

## Overview

This benchmark suite tests the Lean Prover specialist mask's ability to:
1. Write syntactically correct Lean 4 proofs
2. Apply appropriate tactics for different proof patterns
3. Model protocols and state machines
4. Prove properties about distributed systems

## Test Files

| Test | File | Difficulty | Points | Time Limit |
|------|------|------------|--------|------------|
| 1 | `01_basic_propositional.lean` | Easy | 10 | 30s |
| 2 | `02_induction.lean` | Easy-Medium | 15 | 60s |
| 3 | `03_state_machine.lean` | Medium | 20 | 90s |
| 4 | `04_bilateral_dependency.lean` | Medium-Hard | 25 | 120s |
| 5 | `05_protocol_safety.lean` | Hard | 30 | 180s |
| **Total** | - | - | **100** | - |

## Running the Benchmarks

### Prerequisites
```bash
# Install Lean 4
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh
```

### Run Individual Tests
```bash
~/.elan/bin/lean 01_basic_propositional.lean
~/.elan/bin/lean 02_induction.lean
# etc.
```

### Check for Errors
```bash
for file in *.lean; do
  echo "Testing $file..."
  if ~/.elan/bin/lean "$file" 2>&1 | grep -q "warning: declaration uses 'sorry'"; then
    echo "❌ FAIL: Contains 'sorry'"
  elif ~/.elan/bin/lean "$file" 2>&1 | grep -q "error:"; then
    echo "❌ FAIL: Compilation error"
  else
    echo "✅ PASS"
  fi
done
```

## Scoring Criteria

### Test 1: Basic Propositional Logic (10 points)

**Task**: Prove `(p ∧ q) → (q ∧ p)`

**Scoring**:
- 10 points: Compiles without `sorry`
- 0 points: Contains `sorry` or doesn't compile

**Expected tactics**: `intro`, `cases`, `constructor`, `exact`

---

### Test 2: Induction (15 points)

**Task**: Prove `n + 0 = n` for all natural numbers

**Scoring**:
- 15 points: Compiles without `sorry` and uses induction
- 10 points: Compiles but doesn't use induction
- 0 points: Contains `sorry` or doesn't compile

**Expected tactics**: `induction`, `rfl`, `rw`, `calc`

---

### Test 3: State Machine (20 points)

**Task**: Model a simple FSM and prove termination + fixed point

**Scoring**:
- 20 points: Both theorems proven without `sorry`
- 15 points: One theorem proven correctly
- 10 points: Theorems well-formed but contain `sorry`
- 0 points: Doesn't type-check

**Expected concepts**: `structure`, pattern matching, existential proofs

---

### Test 4: Bilateral Dependency (25 points)

**Task**: Formalize mutual dependencies and prove properties

**Scoring**:
- 25 points: All three theorems proven without `sorry`
- 20 points: Two theorems proven
- 15 points: One theorem proven
- 10 points: Theorems well-formed but contain `sorry`
- 0 points: Doesn't type-check

**Expected concepts**: Boolean logic, mutual dependencies, symmetry

---

### Test 5: Protocol Safety (30 points)

**Task**: Define and prove safety property for coordination protocol

**Scoring**:
- 30 points: All five parts completed without `sorry`
- 25 points: Four parts completed
- 20 points: Three parts completed
- 15 points: Two parts completed
- 10 points: One part completed
- 5 points: Safety property correctly defined
- 0 points: Safety definition contains `sorry` or doesn't type-check

**Expected concepts**: Option types, case analysis, protocol invariants

---

## Success Criteria

| Score Range | Grade | Interpretation |
|-------------|-------|----------------|
| 90-100 | A+ | Expert-level formal verification |
| 75-89 | A | Strong understanding of Lean 4 |
| 60-74 | B | Competent proof writing |
| 45-59 | C | Basic proof skills |
| 30-44 | D | Struggling with formal proofs |
| 0-29 | F | Needs significant improvement |

## Benchmark Evolution

### Version 1 (2025-11-05)

Initial benchmark suite created based on Two Generals Protocol formalization experience.

**Rationale**:
- Progressive difficulty (10 → 30 points)
- Covers core Lean 4 concepts
- Tests distributed protocol verification patterns
- Objective scoring (compilation success)

**Key Insights**:
1. Bilateral dependency pattern is critical for coordination protocols
2. Multi-file development (simple + full) enables rapid iteration
3. Progressive proof refinement should be encouraged
4. State machine formalization is fundamental skill

## Future Improvements

### Additional Tests
- [ ] Classical vs constructive logic (when to use `open Classical`)
- [ ] Mathlib integration (using existing theorems)
- [ ] Complex induction (strong induction, well-founded recursion)
- [ ] Dependent types (vectors, sized types)
- [ ] Byzantine fault tolerance properties

### Automated Scoring
- [ ] Script to run all tests and compute total score
- [ ] Time measurement for each test
- [ ] Proof complexity metrics (number of tactics, proof size)
- [ ] Comparison against reference solutions

### Benchmark Refinement
- [ ] Add partial credit for proof sketches
- [ ] Weight tests by difficulty
- [ ] Include negative tests (prove something is unprovable)
- [ ] Test error handling and recovery

## Usage in Mask Improvement

1. **Run benchmark suite** against current mask
2. **Identify failures** - which tests fail?
3. **Analyze patterns** - what tactics are missing?
4. **Update mask** with corrected/additional patterns
5. **Re-run benchmarks** to verify improvement
6. **Iterate** until target score achieved

This creates objective, measurable improvement cycles for the lean-prover mask.
