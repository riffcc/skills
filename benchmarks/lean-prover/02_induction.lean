/-
  Benchmark Test 2: Induction on Natural Numbers

  Difficulty: Easy-Medium
  Points: 15
  Time Limit: 60 seconds

  Task: Prove that adding zero on the right is the identity

  Expected tactics: induction, rfl, rw, calc
-/

-- TASK: Replace 'sorry' with a valid proof using induction
theorem test_add_zero (n : Nat) : n + 0 = n := by
  induction n with
  | zero =>
    -- Base case: 0 + 0 = 0
    rfl
  | succ n ih =>
    -- Inductive step: (n+1) + 0 = n+1
    -- We have ih : n + 0 = n
    calc (n + 1) + 0 = (n + 0) + 1 := by rfl  -- Definition of + on Nat
                    _ = n + 1       := by rw [ih]

-- Test cases
example : 0 + 0 = 0 := test_add_zero 0
example : 5 + 0 = 5 := test_add_zero 5
example : 42 + 0 = 42 := test_add_zero 42

-- SCORING:
-- 15 points: Proof compiles without 'sorry' and uses induction
-- 10 points: Proof compiles but doesn't use induction (less elegant)
-- 0 points: Proof contains 'sorry' or does not compile
