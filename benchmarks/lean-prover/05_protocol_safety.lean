/-
  Benchmark Test 5: Protocol Safety Property

  Difficulty: Hard
  Points: 30
  Time Limit: 180 seconds

  Task: Prove safety (symmetric outcomes) for a coordination protocol

  Expected concepts: option types, case analysis, protocol invariants
-/

-- Decision type
inductive Decision : Type where
  | Attack : Decision
  | Abort : Decision
  deriving Repr, DecidableEq

-- Protocol state
structure ProtocolState where
  alice_decision : Option Decision
  bob_decision : Option Decision
  deriving Repr

-- TASK 1: Define safety property (if both decided, decisions match)
def safety (s : ProtocolState) : Prop :=
  match s.alice_decision, s.bob_decision with
  | some d1, some d2 => d1 = d2
  | _, _ => True  -- Vacuously true if either is undecided

-- TASK 2: Prove that a specific state satisfies safety
theorem both_attack_is_safe :
  safety { alice_decision := some Decision.Attack,
           bob_decision := some Decision.Attack } := by
  unfold safety
  rfl

-- TASK 3: Prove that a specific state satisfies safety
theorem both_abort_is_safe :
  safety { alice_decision := some Decision.Abort,
           bob_decision := some Decision.Abort } := by
  unfold safety
  rfl

-- TASK 4: Prove undecided state is safe (vacuously true)
theorem undecided_is_safe :
  safety { alice_decision := none, bob_decision := none } := by
  unfold safety
  trivial

-- TASK 5 (Advanced): Prove asymmetric decisions violate safety
theorem asymmetric_violates_safety :
  ¬ safety { alice_decision := some Decision.Attack,
             bob_decision := some Decision.Abort } := by
  unfold safety
  intro h
  -- h : Decision.Attack = Decision.Abort
  cases h  -- Attack and Abort are different constructors, contradiction

-- Test case: Check that decisions match
def decisions_match (s : ProtocolState) : Bool :=
  match s.alice_decision, s.bob_decision with
  | some d1, some d2 => d1 == d2
  | _, _ => true  -- Undecided is safe

example : decisions_match { alice_decision := some Decision.Attack, bob_decision := some Decision.Attack } = true := by rfl

-- SCORING:
-- 30 points: All five parts completed without 'sorry'
-- 25 points: Four parts completed
-- 20 points: Three parts completed
-- 15 points: Two parts completed
-- 10 points: One part completed
-- 5 points: Safety property correctly defined
-- 0 points: Safety property definition contains 'sorry' or doesn't type-check
