/-
  Benchmark Test 3: State Machine Modeling

  Difficulty: Medium
  Points: 20
  Time Limit: 90 seconds

  Task: Model a simple state machine and prove a basic property

  Expected concepts: structures, pattern matching, existential proofs
-/

-- State machine with two states
inductive State : Type where
  | Initial : State
  | Final : State
  deriving Repr, DecidableEq

-- Transition function
def transition : State → State
  | State.Initial => State.Final
  | State.Final => State.Final

-- Apply transition n times
def applyN : Nat → State → State
  | 0, s => s
  | n+1, s => applyN n (transition s)

-- TASK 1: Prove that we reach Final state in at most 1 step
theorem reaches_final (s : State) :
  ∃ n : Nat, n ≤ 1 ∧ (applyN n s = State.Final) := by
  sorry

-- TASK 2: Prove that Final is a fixed point
theorem final_is_fixpoint :
  transition State.Final = State.Final := by
  sorry

-- Test cases
example : applyN 1 State.Initial = State.Final := by rfl
example : applyN 0 State.Final = State.Final := by rfl

-- SCORING:
-- 20 points: Both theorems proven without 'sorry'
-- 15 points: One theorem proven correctly
-- 10 points: Theorems are well-formed but contain 'sorry'
-- 0 points: Theorems don't type-check
