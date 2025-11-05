# Benchmark Evolution Framework

**Problem:** Once all masks hit 100% on benchmarks, we're "teaching to the test" - no more improvement signal.

**Solution:** Freeze saturated benchmarks as v1, create stricter v2 benchmarks, repeat indefinitely.

---

## Evolution Strategy

### When to Evolve Benchmarks

**Trigger:** A mask category saturates (all masks at 100% for 2+ iterations)

**Current Status:**
- distributed-systems: v1 saturated (7/7, 100%)
- database-architecture: v1 saturated (8/8, 100%)
- storage-systems: v1 saturated (9/9, 100%)
- infrastructure-deployment: v1 saturated (9/9, 100%)
- mask-improver: v1 saturated (10/10, 100%)

**Action Required:** ALL categories ready for v2 benchmarks! 🎯

---

## Benchmark Versioning

### Version Lifecycle

```
v1: Basic Coverage
├── Tests fundamental concepts exist
├── Validates structure and required sections
└── Checks for basic domain knowledge

v2: Depth & Understanding
├── Tests practical application (examples, scenarios)
├── Validates trade-off awareness
├── Checks for anti-patterns and "what not to do"
└── Requires concrete, runnable examples

v3: Mastery & Edge Cases
├── Tests edge cases and failure scenarios
├── Validates cross-domain integration
├── Checks for performance considerations
├── Requires production-ready guidance

v4: Innovation & Meta-Learning
├── Tests novel pattern application
├── Validates ability to improve other masks
├── Checks for generalization across domains
└── Requires discovery of new patterns
```

### Freezing Benchmarks

When a benchmark version saturates:

1. **Freeze it** - Tests become immutable, version locked
2. **Archive results** - Store final scores in `benchmarks/v1/results.json`
3. **Create v2** - New test file with stricter criteria
4. **Preserve v1** - Keep running to prevent regression

**File Structure:**
```
tests/
├── v1/
│   ├── distributed_systems_benchmark.rs (frozen)
│   ├── database_architecture_benchmark.rs (frozen)
│   └── results/
│       ├── distributed_systems_v1_final.json
│       └── database_architecture_v1_final.json
├── v2/
│   ├── distributed_systems_benchmark.rs (active)
│   └── database_architecture_benchmark.rs (active)
└── v3/
    └── (future)
```

---

## V2 Benchmark Criteria

### Distributed Systems V2

**V1 Coverage (Saturated):**
- ✅ CAP theorem mentioned
- ✅ Consensus algorithms mentioned
- ✅ Observability tools mentioned

**V2 Depth:**
- ❓ CAP trade-offs explained with concrete examples (CP vs AP systems)
- ❓ Consensus failure scenarios covered (split-brain, quorum loss)
- ❓ Observability includes specific metric thresholds (replication lag < 100ms)
- ❓ Anti-patterns documented ("Don't use consensus for..." examples)
- ❓ Real-world production scenarios (3+ detailed examples with configurations)

**V2 Tests:**
```rust
#[test]
fn v2_cap_theorem_tradeoffs() {
    // Not just "mentions CAP" but shows CP vs AP decision-making
    assert!(content.contains("CP system") || content.contains("Consistency + Partition"));
    assert!(content.contains("AP system") || content.contains("Availability + Partition"));
    assert!(content.contains("Example:") && content.contains("CAP"));
}

#[test]
fn v2_consensus_failure_scenarios() {
    // Not just "mentions Raft" but covers failure modes
    assert!(content.contains("split-brain"));
    assert!(content.contains("quorum loss") || content.contains("lost quorum"));
    assert!(content.contains("Example:") && content.contains("failure"));
}

#[test]
fn v2_production_examples() {
    // Requires 3+ complete deployment examples with configs
    let example_count = content.matches("### Example").count();
    assert!(example_count >= 3, "Must have 3+ detailed examples");

    // At least one example must include actual configuration
    assert!(content.contains("```") || content.contains("```yaml") || content.contains("```toml"));
}
```

---

### Database Architecture V2

**V1 Coverage (Saturated):**
- ✅ ACID properties mentioned
- ✅ Replication patterns mentioned
- ✅ PostgreSQL mentioned

**V2 Depth:**
- ❓ ACID trade-offs with isolation level examples (Serializable vs Read Committed performance)
- ❓ Replication failure scenarios (lag detection, failover procedures)
- ❓ Query optimization with actual EXPLAIN plans
- ❓ Schema anti-patterns ("Don't normalize everything" with examples)
- ❓ Production scenarios (3+ with actual schema DDL, queries, metrics)

**V2 Tests:**
```rust
#[test]
fn v2_acid_isolation_tradeoffs() {
    // Not just "mentions isolation levels" but explains trade-offs
    assert!(content.contains("Serializable") && content.contains("performance"));
    assert!(content.contains("Read Committed") && content.contains("phantom reads"));
}

#[test]
fn v2_replication_failure_handling() {
    // Not just "mentions replication" but covers failure modes
    assert!(content.contains("replication lag"));
    assert!(content.contains("failover") || content.contains("promotion"));
    assert!(content.contains("Example:") && content.contains("replica"));
}

#[test]
fn v2_query_optimization_examples() {
    // Requires actual EXPLAIN plans, not just mentions
    assert!(content.contains("EXPLAIN") || content.contains("execution plan"));
    assert!(content.contains("Seq Scan") || content.contains("Index Scan"));
}
```

---

### Storage Systems V2

**V1 Coverage (Saturated):**
- ✅ MooseFS mentioned
- ✅ Distributed FS mentioned
- ✅ No deprecated tech (GlusterFS removed)

**V2 Depth:**
- ❓ MooseFS deployment with actual configuration (master, chunkserver, client)
- ❓ Storage performance benchmarks (fio commands, expected IOPS)
- ❓ Failure recovery procedures (disk failure, chunkserver failure, master failure)
- ❓ Capacity planning with growth projections
- ❓ Production scenarios (3+ with hardware specs, network topology, monitoring)

---

### Infrastructure Deployment V2

**V1 Coverage (Saturated):**
- ✅ Security mentioned
- ✅ IaC mentioned
- ✅ Proxmox/LXC mentioned

**V2 Depth:**
- ❓ Security with actual configurations (Vault setup, TLS certificate automation)
- ❓ Deployment rollback procedures (tested rollback examples)
- ❓ Zero-downtime deployment patterns (blue-green with actual configs)
- ❓ Disaster recovery runbooks (tested restore procedures)
- ❓ Production scenarios (3+ with complete Jetpack playbooks)

---

### Mask-Improver V2

**V1 Coverage (Saturated):**
- ✅ Analysis capabilities present
- ✅ Specificity emphasized
- ✅ Evidence-based approach

**V2 Depth:**
- ❓ Before/after diffs in output (exact text changes)
- ❓ Score prediction (predict exact benchmark score changes)
- ❓ Implementation difficulty ratings (trivial/easy/moderate/hard)
- ❓ Priority framework (impact × effort matrix)
- ❓ Cross-domain pattern library (apply database patterns to storage)

---

## Evolution Workflow

### Step 1: Detect Saturation

```bash
# Run all v1 benchmarks
cargo test --test "*_benchmark" -- --nocapture

# Check for 100% across all masks
# If all masks at 100% for 2+ improvement cycles → saturated
```

### Step 2: Freeze V1

```bash
# Move current benchmarks to v1/
mkdir -p tests/v1
mv tests/*_benchmark.rs tests/v1/

# Archive final scores
cargo test --test "v1/*" -- --nocapture > benchmarks/v1_final_results.txt
```

### Step 3: Create V2

```bash
# Create v2 benchmarks with stricter criteria
# Use this document as reference for v2 requirements
mkdir -p tests/v2
# Write new tests with deeper validation
```

### Step 4: Run Both

```bash
# Regression test: v1 should still pass
cargo test --test "v1/*" -- --nocapture

# Progress test: v2 shows new gaps
cargo test --test "v2/*" -- --nocapture
```

### Step 5: Improve Until V2 Saturates

```bash
# Use mask-improver to close v2 gaps
# Iterate until all masks at 100% on v2
# Then freeze v2, create v3, repeat
```

---

## Preventing Teaching to the Test

### Anti-Patterns to Avoid

❌ **Keyword Stuffing:** Masks shouldn't just mention terms to pass tests
```rust
// BAD v2 test:
assert!(content.contains("CP system")); // Can be gamed with keyword

// GOOD v2 test:
assert!(has_example_with_tradeoff_analysis("CP", "AP", "when to choose"));
```

❌ **Surface Coverage:** Tests shouldn't just check presence, but depth
```rust
// BAD:
assert!(content.contains("ACID"));

// GOOD:
assert!(has_example_demonstrating_isolation_level_choice());
assert!(explains_performance_tradeoff("Serializable", "Read Committed"));
```

❌ **Static Examples:** Same examples across all masks
```rust
// Tests should verify domain-specific examples, not generic ones
assert!(example_is_specific_to_domain("distributed-systems"));
```

### Good V2 Tests

✅ **Require Runnable Examples:** Code that could actually be deployed
✅ **Validate Trade-off Analysis:** Must explain why/when/how
✅ **Check for Anti-patterns:** "Don't do X because Y"
✅ **Measure Depth:** Count of detailed examples, not just keywords
✅ **Cross-reference:** Examples should integrate multiple concepts

---

## Benchmark Difficulty Curve

### V1: Can you identify this concept exists?
"Does the mask mention ACID?"

### V2: Can you explain this concept with examples?
"Does the mask show when to use Serializable vs Read Committed?"

### V3: Can you handle edge cases?
"Does the mask explain what happens when Serializable causes deadlocks?"

### V4: Can you discover new patterns?
"Does the mask identify patterns that work across database/storage domains?"

---

## Success Metrics

### V1 Success: Coverage
- All fundamental concepts present
- Structure is correct
- Basic domain knowledge validated

### V2 Success: Depth
- Trade-offs explained with examples
- Failure modes documented
- Production-ready guidance

### V3 Success: Mastery
- Edge cases handled
- Cross-domain integration
- Performance considerations

### V4 Success: Innovation
- New pattern discovery
- Meta-learning demonstrated
- Improves other masks

---

## Next Actions

### Immediate (Now)

1. ✅ Document evolution framework (this file)
2. ⏭️ Create v2 benchmarks for all saturated masks
3. ⏭️ Run v2 benchmarks to establish new baseline
4. ⏭️ Use mask-improver to close v2 gaps

### Short-term (Next Sprint)

5. ⏭️ Freeze v1 when starting v2 improvements
6. ⏭️ Create pattern library from v1 → v2 learnings
7. ⏭️ Design v3 criteria based on v2 saturation

### Long-term (Continuous)

8. ⏭️ Monitor for v2 saturation → trigger v3
9. ⏭️ Extract meta-patterns from version transitions
10. ⏭️ Use meta-patterns to accelerate v(n) → v(n+1)

---

## Philosophy

**Key Insight:** Benchmarks should evolve faster than masks can saturate them.

**Target:** Masks should always be improving toward an unreachable horizon.

**Outcome:** Continuous improvement without plateaus.

**Metric:** Time to saturation should *increase* with each version:
- V1: Days to saturate
- V2: Weeks to saturate
- V3: Months to saturate
- V4: Years to saturate (if ever)

---

## Competitive Advantage

This evolution framework creates a **moving target** that prevents:
- Stagnation (always new benchmarks to chase)
- Gaming (keyword stuffing doesn't work at v2+)
- Plateaus (v(n+1) exists before v(n) saturates)

It enables:
- Continuous improvement (always gaps to close)
- Genuine understanding (depth over breadth)
- Innovation (v4 requires novel discoveries)

**This is how we build truly intelligent specialists.** 🎯

---

**Status:** Framework documented, ready to implement v2 benchmarks.

**Next:** Create `tests/v2/` directory and write first v2 benchmark.
