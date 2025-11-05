/-
  Benchmark Test 0: Tutorial - Absolute Basics

  Difficulty: Trivial
  Points: 5
  Time Limit: 15 seconds

  Task: Verify understanding of basic Lean 4 syntax

  Expected concepts: rfl, exact, trivial
-/

-- TASK 1: Prove reflexivity (already done as example)
theorem tutorial_rfl : 1 + 1 = 2 := by
  rfl  -- Reflexivity: both sides are definitionally equal

-- TASK 2: Prove a simple implication
theorem tutorial_exact (p : Prop) : p → p := by
  intro h  -- Assume h : p
  exact h  -- Prove goal p using hypothesis h

-- TASK 3: Prove True
theorem tutorial_trivial : True := by
  trivial  -- True is always provable

-- Test cases
example : tutorial_rfl = rfl := by rfl
example : tutorial_exact (1 = 1) rfl = rfl := by rfl

-- SCORING:
-- 5 points: All three theorems compile without 'sorry'
-- 0 points: Any theorem contains 'sorry' or doesn't compile
