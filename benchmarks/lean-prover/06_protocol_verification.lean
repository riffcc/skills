/-
  Benchmark Test 6: Protocol Verification

  Difficulty: Hard
  Points: 40
  Time Limit: 300 seconds

  Task: Formalize and prove properties of a two-party commit protocol

  Expected concepts: multi-party state, safety properties, coordination logic
-/

-- Two parties in the protocol
inductive Party : Type where
  | Alice : Party
  | Bob : Party
  deriving Repr, DecidableEq

-- Vote decisions
inductive Vote : Type where
  | Commit : Vote
  | Abort : Vote
  deriving Repr, DecidableEq

-- Party state (local knowledge)
structure PartyState where
  my_vote : Option Vote
  received_vote : Option Vote
  decision : Option Vote
  deriving Repr

-- Global protocol state
structure ProtocolState where
  alice : PartyState
  bob : PartyState
  deriving Repr

-- TASK 1: Define "both decided" predicate (5 pts)
-- Both parties have made decisions
def both_decided (s : ProtocolState) : Prop :=
  sorry

-- TASK 2: Define "safe" predicate (5 pts)
-- If both decided, their decisions must match
def safe (s : ProtocolState) : Prop :=
  sorry

-- TASK 3: Prove both Commit is safe (10 pts)
theorem both_commit_safe :
  safe { alice := { my_vote := some Vote.Commit,
                    received_vote := some Vote.Commit,
                    decision := some Vote.Commit },
         bob := { my_vote := some Vote.Commit,
                  received_vote := some Vote.Commit,
                  decision := some Vote.Commit } } := by
  sorry

-- TASK 4: Prove both Abort is safe (10 pts)
theorem both_abort_safe :
  safe { alice := { my_vote := some Vote.Abort,
                    received_vote := some Vote.Abort,
                    decision := some Vote.Abort },
         bob := { my_vote := some Vote.Abort,
                  received_vote := some Vote.Abort,
                  decision := some Vote.Abort } } := by
  sorry

-- TASK 5: Prove one party can't decide alone (10 pts)
-- If a party has decided, they must have received counterparty's vote
theorem requires_received_vote (s : PartyState) :
  s.decision.isSome → s.received_vote.isSome := by
  sorry

-- Test cases (should compile when tasks complete)
example : both_decided { alice := { my_vote := some Vote.Commit,
                                    received_vote := some Vote.Commit,
                                    decision := some Vote.Commit },
                         bob := { my_vote := some Vote.Commit,
                                  received_vote := some Vote.Commit,
                                  decision := some Vote.Commit } } := by
  sorry

-- SCORING:
-- 40 points: All 5 tasks complete without 'sorry'
-- 30 points: Tasks 1-4 complete
-- 20 points: Tasks 1-3 complete
-- 10 points: Tasks 1-2 complete
-- 0 points: Tasks don't compile or contain 'sorry'
