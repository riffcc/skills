/-
  Benchmark Test 1: Basic Propositional Logic

  Difficulty: Easy
  Points: 10
  Time Limit: 30 seconds

  Task: Prove commutativity of conjunction

  Expected tactics: intro, cases, constructor, exact
-/

-- TASK: Replace 'sorry' with a valid proof
theorem test_and_comm (p q : Prop) : (p ∧ q) → (q ∧ p) := by
  intro h           -- Assume h : p ∧ q
  cases h with      -- Destructure the conjunction
  | intro hp hq =>  -- Get hp : p and hq : q
    constructor     -- Build q ∧ p
    · exact hq      -- Prove q (first component)
    · exact hp      -- Prove p (second component)

-- Test cases
example : (True ∧ False) → (False ∧ True) := test_and_comm True False
example : (1 = 1 ∧ 2 = 2) → (2 = 2 ∧ 1 = 1) := test_and_comm (1 = 1) (2 = 2)

-- SCORING:
-- 10 points: Proof compiles without 'sorry'
-- 0 points: Proof contains 'sorry' or does not compile
