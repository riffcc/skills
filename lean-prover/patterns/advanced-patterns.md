## Advanced Patterns

### Pattern: Distributed Protocol Verification

When formalizing distributed systems and protocols (consensus algorithms, coordination protocols, etc.):

**1. Model the Network**
```lean
-- Unreliable network as axiom
axiom network_delivers : Message → Time → Bool

-- Eventual delivery assumption (partial synchrony)
axiom eventual_delivery : ∀ m : Message, ∃ t : Time, network_delivers m t = true
```

**2. Define Bilateral Dependencies**

Critical for coordination protocols - one party can only advance if they've received proof from the other:

```lean
structure PartyState where
  my_data : Data
  received_counterparty_proof : Bool
  my_proof_created : Bool
  decision : Option Decision

-- Can only create advanced proof if received counterparty's proof
def can_create_advanced (s : PartyState) : Bool :=
  s.my_proof_created && s.received_counterparty_proof
```

**3. Prove Safety via Case Analysis**

For bilateral coordination, show all message delivery scenarios lead to symmetric outcomes:

```lean
theorem safety (s : ProtocolState) :
  s.alice.decision.isSome ∧ s.bob.decision.isSome →
  s.alice.decision = s.bob.decision := by
  intro ⟨ha, hb⟩
  -- Case 1: Both received proofs → Both decide X
  -- Case 2: Neither received → Timeout → Both decide Y
  -- Case 3: Asymmetric delivery → Show leads to Case 1 or 2
  sorry
```

**4. Prove Liveness with Timeout**

```lean
theorem liveness (timeout : Nat) :
  ∃ n : Nat,
    let s := run_protocol n timeout
    s.alice.decision.isSome ∧ s.bob.decision.isSome := by
  -- Either messages deliver or timeout occurs
  -- Both paths lead to decisions
  sorry
```

**Key Insight**: Bilateral dependencies prevent unilateral completion, ensuring symmetric outcomes.

---

### Pattern: Multi-File Development Workflow

For large proofs, use a **two-file strategy** for rapid iteration:

**File 1: `Protocol_Simple.lean` (Fast Validation)**
```lean
-- No mathlib imports
-- Core types only
structure State where
  data : Nat
  decided : Bool

def transition (s : State) : State :=
  { s with decided := true }

-- Theorem statements with sorry
theorem safety (s : State) : s.decided → True := by sorry
```
**Purpose**: Validate structure **instantly** (< 1 second compile time)

**File 2: `Protocol.lean` (Full Proof)**
```lean
import Mathlib.Data.Finset.Basic
import Mathlib.Logic.Function.Basic

-- Same structure as Simple version
-- But with complete proofs using mathlib tactics

theorem safety (s : State) : s.decided → True := by
  intro h
  -- Full proof with mathlib tactics
  trivial
```
**Purpose**: Complete verification with full library support

**Workflow:**
1. Design structure in `_Simple.lean`
2. Test compiles quickly → iterate rapidly
3. Once structure stable, copy to full version
4. Fill in proofs with mathlib tactics
5. Run full `lake build` for final verification

**Benefits:**
- 🚀 **Fast iteration**: No waiting for mathlib builds
- ✅ **Early validation**: Catch structural errors immediately
- 📝 **Clear progress**: Know when to invest in full build

---

### Pattern: State Machine Formalization

Standard approach for protocol verification:

```lean
-- State definition
structure State where
  phase : Nat  -- Current protocol phase
  local_data : Data
  received_remote : Option RemoteData
  decision : Option Decision
  deriving Repr

-- Initial state
def init : State :=
  { phase := 0
  , local_data := default_data
  , received_remote := none
  , decision := none }

-- Transition function
def step (s : State) (msg : Option Message) : State :=
  match s.phase, msg with
  | 0, some m => { s with phase := 1, received_remote := some m.data }
  | 1, _ => { s with phase := 2, decision := some Decision.Success }
  | _, _ => s

-- Run protocol for n steps
def run (n : Nat) (messages : Nat → Option Message) : State :=
  (List.range n).foldl (fun s i => step s (messages i)) init

-- Invariant: Once decided, decision doesn't change
theorem decision_stable (s : State) (msg : Option Message) :
  s.decision.isSome → (step s msg).decision = s.decision := by
  intro h
  cases s.phase <;> cases msg <;> simp [step]
```

**Pattern Elements:**
1. **State**: Complete description of protocol state
2. **Initial**: Well-defined starting point
3. **Transition**: Deterministic state updates
4. **Execution**: Run protocol for n steps
5. **Invariants**: Properties that hold throughout execution

---

### Pattern: Progressive Proof Refinement

Build proofs incrementally, marking progress clearly:

**Stage 1: Theorem Statement**
```lean
theorem my_theorem (x : Nat) : x + 0 = x := by
  sorry
```
✅ **Achievement**: Well-typed theorem statement

**Stage 2: Proof Skeleton**
```lean
theorem my_theorem (x : Nat) : x + 0 = x := by
  -- Strategy: Induction on x
  induction x with
  | zero =>
    -- Base case: 0 + 0 = 0
    sorry
  | succ n ih =>
    -- Inductive step: (n+1) + 0 = n+1
    -- We have ih : n + 0 = n
    sorry
```
✅ **Achievement**: Proof strategy documented, structure correct

**Stage 3: Partial Completion**
```lean
theorem my_theorem (x : Nat) : x + 0 = x := by
  induction x with
  | zero =>
    rfl  -- ✓ Base case proven
  | succ n ih =>
    -- Inductive step remaining
    sorry
```
✅ **Achievement**: Base case complete, inductive step clear

**Stage 4: Complete Proof**
```lean
theorem my_theorem (x : Nat) : x + 0 = x := by
  induction x with
  | zero => rfl
  | succ n ih =>
    calc (n + 1) + 0 = (n + 0) + 1 := by rfl
                    _ = n + 1 := by rw [ih]
```
✅ **Achievement**: Fully verified, no `sorry` remaining

**Benefits**: Clear progress tracking, easier debugging, incremental validation

---

### Pattern: Bilateral Dependency (Coordination Protocols)

Essential pattern for proving symmetric outcomes in coordination protocols:

```lean
-- Two parties coordinating
structure BilateralState where
  alice_ready : Bool
  bob_ready : Bool
  alice_has_bob_proof : Bool
  bob_has_alice_proof : Bool

-- Both can proceed only if mutual proofs exchanged
def both_can_proceed (s : BilateralState) : Bool :=
  s.alice_ready ∧ s.alice_has_bob_proof ∧
  s.bob_ready ∧ s.bob_has_alice_proof

-- Key theorem: If both can proceed, proofs were exchanged
theorem bilateral_exchange (s : BilateralState) :
  both_can_proceed s = true →
  s.alice_has_bob_proof = true ∧ s.bob_has_alice_proof = true := by
  intro h
  -- Direct from definition
  unfold both_can_proceed at h
  simp [Bool.and_eq_true] at h
  exact ⟨h.2.1, h.2.2⟩

-- Symmetry: If Alice can proceed, Bob has necessary prerequisites
theorem bilateral_symmetry (s : BilateralState) :
  s.alice_ready ∧ s.alice_has_bob_proof = true →
  s.bob_ready ∨ ∃ time, will_be_ready_at time := by
  -- Bob sent proof → Bob must have been ready (or will be)
  sorry
```

**Applications:**
- Two Generals Protocol (bilateral coordination)
- Atomic swaps (both parties commit or both abort)
- State channel closes (symmetric finalization)
- Distributed transactions (coordinated commit/rollback)

**Key Properties to Prove:**
1. **Safety**: No asymmetric outcomes (both succeed or both fail)
2. **Bilateral dependency**: Advancement requires mutual exchange
3. **Symmetry**: If one party can complete, other can too (eventually)

---
