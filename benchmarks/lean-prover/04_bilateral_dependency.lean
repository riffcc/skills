/-
  Benchmark Test 4: Bilateral Dependency Pattern

  Difficulty: Medium-Hard
  Points: 25
  Time Limit: 120 seconds

  Task: Formalize and prove bilateral dependency properties

  Expected concepts: structures, boolean logic, mutual dependencies
-/

-- Bilateral state (two parties coordinating)
structure BilateralState where
  alice_ready : Bool
  bob_ready : Bool
  alice_has_bob_proof : Bool
  bob_has_alice_proof : Bool
  deriving Repr

-- Both can proceed only if mutual proofs exchanged
def both_can_proceed (s : BilateralState) : Bool :=
  s.alice_ready && s.alice_has_bob_proof &&
  s.bob_ready && s.bob_has_alice_proof

-- TASK 1: Prove that if both can proceed, proofs were exchanged
theorem bilateral_exchange (s : BilateralState) :
  both_can_proceed s = true →
  s.alice_has_bob_proof = true ∧ s.bob_has_alice_proof = true := by
  intro h
  unfold both_can_proceed at h
  simp only [Bool.and_eq_true] at h
  -- h is now: ((alice_ready ∧ alice_has_bob_proof) ∧ bob_ready) ∧ bob_has_alice_proof
  exact ⟨h.1.1.2, h.2⟩

-- TASK 2: Prove that Alice can't proceed alone
theorem alice_needs_bob (s : BilateralState) :
  both_can_proceed s = true →
  s.alice_has_bob_proof = true := by
  intro h
  unfold both_can_proceed at h
  simp only [Bool.and_eq_true] at h
  exact h.1.1.2

-- TASK 3: Symmetry - Bob can't proceed alone either
theorem bob_needs_alice (s : BilateralState) :
  both_can_proceed s = true →
  s.bob_has_alice_proof = true := by
  intro h
  unfold both_can_proceed at h
  simp only [Bool.and_eq_true] at h
  exact h.2

-- Test cases
example :
  both_can_proceed { alice_ready := true, bob_ready := true,
                     alice_has_bob_proof := true, bob_has_alice_proof := true } = true := by
  rfl

example :
  both_can_proceed { alice_ready := true, bob_ready := false,
                     alice_has_bob_proof := true, bob_has_alice_proof := false } = false := by
  rfl

-- SCORING:
-- 25 points: All three theorems proven without 'sorry'
-- 20 points: Two theorems proven correctly
-- 15 points: One theorem proven correctly
-- 10 points: Theorems are well-formed but contain 'sorry'
-- 0 points: Theorems don't type-check
