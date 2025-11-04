# RHSI - Recursive Hierarchical Self Improvement

**Created:** 2025-11-04 (Wings' birthday eve, kief-fueled vision)
**Status:** Design phase - Build starts Nov 5
**Resources:** $1,000 Claude Code credits (until Nov 18, 2025)

## The Vision

Build a framework for creating, testing, and recursively improving specialized AI personalities (masks/harnesses) that can be benchmarked against real-world scenarios and improved over time.

## Core Concept

Not different AIs. Not fragmented personalities. Just **Lief with different equipment strapped on for different jobs.**

Same keeper of The Forge, same understanding, same foundation - but sometimes wearing specialist harnesses:
- Pain Tracker armor
- Sprint Planner armor
- Distributed Systems Engineer armor
- Playwright/Web Testing armor
- Hauska Executive armor
- Life Automation armor

## The Architecture

### Layer 1: Masks/Harnesses
Specialist versions of Claude with domain expertise, defined in markdown skill files.

**Example Skills:**
- `distributed-systems.md` - Expert in Two Generals Problem, Byzantine Generals, consensus algorithms
- `playwright-tester.md` - Web design evaluation with gamepad/touchscreen focus
- `hauska-executive.md` - Studies investor collateral, understands the business deeply
- `life-automation.md` - Proactively helps Wings check in and manage personal systems
- `pain-tracker.md` - Guides body check-ins and relief documentation

### Layer 2: Benchmarking
Test each mask against real scenarios from The Forge/Castle/Palace work.

**Example Tests:**
- Distributed Systems mask: Work through Two Generals Problem, apply to Lens v2
- Playwright mask: Evaluate web designs, test gamepad integration
- Hauska mask: Answer investor questions from Nick's collateral

**Scoring:**
- Objective metrics (correctness, completeness, adherence to patterns)
- Subjective evaluation (helpfulness, clarity, alignment with Wings' standards)

### Layer 3: Multi-Model Testing
Run the same masks across different AI providers:
- Anthropic Claude (via Claude Code)
- OpenRouter models (when testing/cost-optimizing)
- Z.ai models (backup when Anthropic is down)

Compare performance across models for each mask.

### Layer 4: Consensus via Subagents
Multiple masked Claudes can **argue with each other** via the subagent system.

Example: Three Distributed Systems Claudes debate the best approach to finishing Lens v2's DHT, reach consensus, document their reasoning.

### Layer 5: Recursion
Each benchmark cycle:
1. Load mask
2. Run tests
3. Score performance
4. **Update mask based on what worked/didn't work**
5. Repeat

Masks get better at their specialties over time.

### Layer 6: Standardization
Slash commands make the entire process repeatable:
- `/benchmark-mask distributed-systems` - Run benchmarks
- `/consensus lens-v2-approach` - Multiple experts debate and reach agreement
- `/improve-mask playwright-tester` - Analyze scores and update the mask

## Immediate Use Cases

### 1. Playwright Web Testing
**Need:** Evaluate web designs, test gamepad/touchscreen integration
**Mask:** Playwright specialist with expertise in accessibility and input methods
**Benchmark:** Test against existing Hauska/Haystack interfaces

### 2. Distributed Systems Engineer
**Need:** Finish Lens v2, Citadel DHT, IPFS replacement
**Mask:** Expert in consensus algorithms, distributed storage, P2P networking
**Benchmark:** Work through classic problems, apply to real architecture

### 3. Hauska Executive
**Need:** Understand the business deeply, answer investor questions, close deals
**Mask:** Digests Nick's investor collateral, understands dual licensing, marketplace vision
**Benchmark:** Answer questions from Nick's Perplexity investor objections list

### 4. Life Automation
**Need:** Help Wings manage personal systems (pain tracking, decompression, planning)
**Mask:** Understands Wings' SDAM, chronic pain, work patterns
**Benchmark:** Successfully guide check-ins, document patterns, suggest interventions

## Why This Is Soft-AGI

Traditional approach: One AI tries to be everything, mediocre at most things.

RHSI approach:
- **Specialization** - Each mask is expert in one domain
- **Benchmarking** - Real-world performance testing, not theoretical
- **Recursion** - Masks improve based on actual results
- **Consensus** - Multiple specialists collaborate and argue
- **Multi-model** - Best tool for each job across different AI providers
- **Standardization** - Repeatable, improvable processes

Result: **Adaptive specialization that gets better over time.**

## Technical Implementation

### Mask Format (Claude Skills)
```markdown
# Mask Name

## Identity
[Who this masked-Lief is, what they specialize in]

## Expertise
[Domain knowledge, frameworks, patterns]

## Approach
[How they solve problems in this domain]

## Benchmarks
[What tests they should excel at]
```

### Benchmarking Harness (Rust utility)
```rust
// Load mask
let mask = Mask::load("distributed-systems.md")?;

// Run test scenarios
let results = mask.run_benchmarks(&test_scenarios)?;

// Score performance
let scores = evaluate_results(&results)?;

// Update mask based on performance
mask.improve_from_scores(&scores)?;
```

### Slash Commands
Standardize the workflow:
- `/load-mask <name>` - Put on specialist harness
- `/benchmark <mask>` - Run tests
- `/consensus <topic>` - Multi-agent debate
- `/improve <mask>` - Recursive improvement
- `/compare-models <mask>` - Test across providers

### MCP Integration
Expose via MCP server so ANY Claude instance can:
- Load masks
- Run benchmarks
- Participate in consensus
- Report scores

## The 14-Day Sprint Plan

**Nov 5-18, 2025** - $1,000 credits available

### Week 1: Foundation (Nov 5-11)
- [ ] Day 1: Build first mask (Distributed Systems)
- [ ] Day 2: Create benchmarking harness (Rust utility)
- [ ] Day 3: Test mask against Two Generals Problem
- [ ] Day 4: Build second mask (Playwright Tester)
- [ ] Day 5: Implement scoring system
- [ ] Day 6: Add multi-model support (OpenRouter)
- [ ] Day 7: Build consensus system (subagent debates)

### Week 2: Scale & Integration (Nov 12-18)
- [ ] Day 8: Hauska Executive mask + investor collateral digest
- [ ] Day 9: Life Automation mask + pain tracking
- [ ] Day 10: Slash command standardization
- [ ] Day 11: MCP server for mask management
- [ ] Day 12: Palace integration planning
- [ ] Day 13: Full system benchmark run
- [ ] Day 14: Documentation & handoff

## Resources

**Credits:** $1,000 Claude Code web (expires Nov 18, 11:59 PM PT)
**Models:** Anthropic Claude, OpenRouter, Z.ai
**Tools:** Claude Code, Rust, MCP, Subagents
**Data:** Nick's investor collateral, Lens v2 codebase, Hauska designs

## Success Metrics

By Nov 18, we should have:
1. ✅ 4+ working masks with domain expertise
2. ✅ Benchmarking harness that scores performance
3. ✅ Multi-model comparison across providers
4. ✅ Consensus system for multi-agent debates
5. ✅ Recursive improvement showing measurable gains
6. ✅ Standardized slash commands for repeatability
7. ✅ Clear path to Palace integration

## Why This Matters

**For Hauska:** Specialized executives who deeply understand the business
**For Lens v2:** Distributed systems experts who can finish the DHT
**For Palace:** The foundation for adaptive, self-improving tooling
**For Wings:** Personal automation that actually understands and helps

This is not just skills. This is **the infrastructure for building increasingly capable specialists** that get better at their jobs over time.

Soft-AGI. Built in the garage. Forged in 14 days.

---

*Conceived 2025-11-04 in The Forge, birthday eve, 0.1g kief cap, Coheed & Cambria playing*
*"Just when I think I'm out, they pull me back in" - Wings, on receiving $1,000 credits*

🔥⚒️🚀
