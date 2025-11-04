# Palace Skills - RHSI System Specification

**Version:** 0.1.0-alpha
**Created:** 2025-11-04 (Wings' 33rd Birthday Sprint!)
**Budget:** $1,000 Claude Code credits (14 days)
**Deadline:** 2025-11-18 23:59 PT

---

## Executive Summary

**RHSI (Recursive Hierarchical Self Improvement)** is a framework for creating, testing, benchmarking, and recursively improving specialized AI personalities ("masks" or "harnesses") that can be loaded onto Claude instances to provide expert-level domain knowledge and problem-solving capabilities.

**Core Innovation:** Same foundational AI (Lief), different specialist equipment loaded on demand, with objective benchmarking and recursive improvement based on real-world performance.

**Target Use Cases:**
1. Distributed Systems Engineering (Lens v2, Citadel DHT)
2. Web Testing & Gamepad Integration (Playwright automation)
3. Business Strategy (Hauska executive decision-making)
4. Personal Automation (Pain tracking, life management)

---

## System Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Palace Skills CLI                        │
│  (palace-skills mask load/benchmark/improve/consensus)       │
└─────────────────────────────────────────────────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Mask       │    │  Benchmark   │    │   Scorer     │
│   Loader     │    │   Runner     │    │   Engine     │
└──────────────┘    └──────────────┘    └──────────────┘
        │                    │                    │
        └────────────────────┼────────────────────┘
                             ▼
                    ┌──────────────┐
                    │  Evaluation  │
                    │   Database   │
                    │   (SQLite)   │
                    └──────────────┘
                             │
                    ┌────────┴────────┐
                    ▼                 ▼
            ┌──────────────┐  ┌──────────────┐
            │ Git History  │  │  Markdown    │
            │  (Commits)   │  │  Manifests   │
            └──────────────┘  └──────────────┘
```

---

## Core Data Structures

### 1. Mask Definition

**File Format:** Markdown (Claude Skills format)
**Location:** `masks/{specialty}/{model}.md`

```markdown
# {Specialty} Mask - {Model}

## Identity
Who this masked Claude is and what they specialize in.

## Core Expertise
- Domain knowledge areas
- Key frameworks and patterns
- Problem-solving approaches

## Behavioral Guidelines
How this mask should approach problems, communicate, and make decisions.

## Benchmark Criteria
What constitutes success for this specialty:
- Objective metrics
- Subjective quality indicators
- Domain-specific standards

## Training Scenarios
Example problems this mask should excel at solving.

## Improvement Notes
<!-- Auto-updated by RHSI system -->
### Iteration 1 (YYYY-MM-DD)
- Score: X/10
- Strengths: ...
- Weaknesses: ...
- Improvements made: ...
```

**Rust Representation:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mask {
    pub id: Uuid,
    pub specialty: String,        // e.g., "distributed-systems"
    pub model: ModelProvider,     // Sonnet, Haiku, OpenRouter, etc.
    pub version: u32,             // Iteration number
    pub content: String,          // Full markdown content
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub current_score: Option<f64>, // Latest benchmark score
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider {
    ClaudeSonnet,
    ClaudeHaiku,
    OpenRouter { model_id: String },
    ZAi { model_id: String },
    Codex { model_id: String },
}
```

### 2. Benchmark Definition

**File Format:** TOML + Markdown
**Location:** `benchmarks/{specialty}/{test-name}.toml`

```toml
[benchmark]
id = "two-generals-problem"
specialty = "distributed-systems"
difficulty = "intermediate"
time_limit_seconds = 300

[scenario]
description = """
Explain the Two Generals Problem and propose three different
solutions with trade-offs for a real-world distributed system.
"""

[evaluation_criteria]
correctness = 0.4      # 40% weight - factual accuracy
depth = 0.3            # 30% weight - thoroughness of analysis
practical_application = 0.2  # 20% weight - real-world relevance
clarity = 0.1          # 10% weight - communication quality

[expected_elements]
# Must include these concepts to score well
required = [
    "impossibility of guaranteed delivery",
    "acknowledgment protocols",
    "timeout strategies",
    "probabilistic approaches"
]

optional_bonus = [
    "Byzantine fault tolerance",
    "consensus algorithms",
    "real-world examples"
]
```

**Rust Representation:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmark {
    pub id: Uuid,
    pub specialty: String,
    pub name: String,
    pub difficulty: Difficulty,
    pub scenario: String,
    pub evaluation_criteria: HashMap<String, f64>, // criterion -> weight
    pub required_elements: Vec<String>,
    pub optional_elements: Vec<String>,
    pub time_limit: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}
```

### 3. Benchmark Run & Score

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRun {
    pub id: Uuid,
    pub mask_id: Uuid,
    pub benchmark_id: Uuid,
    pub model_provider: ModelProvider,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub transcript: String,              // Full conversation log
    pub output: String,                  // Final response
    pub self_evaluation: Option<SelfEvaluation>,  // **NEW: Mask's self-assessment**
    pub evaluation: Option<Score>,
    pub git_commit_sha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestions {
    pub suggestions: Vec<String>,        // What would make this better next time?
    pub rationale: String,               // Why these specific improvements?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub total: f64,                      // 0-10 scale
    pub breakdown: HashMap<String, f64>, // criterion -> score
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub evaluator_notes: String,
    pub evaluated_by: String,            // "lief-unmasked" or model name
    pub evaluated_at: DateTime<Utc>,
}
```

---

## Command-Line Interface

### Core Commands

#### 1. Mask Management

```bash
# List all masks
palace-skills mask list [--specialty <name>] [--model <provider>]

# Create new mask (interactive or from template)
palace-skills mask create <specialty> <model>

# Load mask into current session (outputs skill markdown to stdout)
palace-skills mask load <specialty> <model>

# Show mask details and history
palace-skills mask show <specialty> <model>

# Clone mask to different model
palace-skills mask clone <specialty> <from-model> <to-model>
```

#### 2. Benchmark Management

```bash
# List benchmarks
palace-skills benchmark list [--specialty <name>] [--difficulty <level>]

# Create new benchmark
palace-skills benchmark create <specialty> <name>

# Run benchmark with specific mask
palace-skills benchmark run <benchmark-id> --mask <specialty>/<model>

# Run all benchmarks for a specialty
palace-skills benchmark run-suite <specialty>
```

#### 3. Evaluation & Scoring

```bash
# Score a completed benchmark run (interactive)
palace-skills score <run-id>

# Auto-score using unmasked Claude evaluation
palace-skills score <run-id> --auto

# Compare scores across models for same benchmark
palace-skills score compare <benchmark-id>

# Show scoring history for a mask
palace-skills score history <mask-id>
```

#### 4. Improvement & Iteration

```bash
# Analyze performance and suggest mask improvements
palace-skills improve analyze <mask-id>

# Apply suggested improvements (updates mask, increments version)
palace-skills improve apply <mask-id> <improvement-suggestions>

# Rerun benchmarks after improvement to measure delta
palace-skills improve validate <mask-id>
```

#### 5. Consensus System

```bash
# Run multi-agent consensus on a question
palace-skills consensus <topic> \
    --masks distributed-systems/sonnet,distributed-systems/haiku \
    --rounds 3

# Example: Debate best approach to Lens v2 DHT
palace-skills consensus "lens-v2-dht-architecture" \
    --specialty distributed-systems \
    --all-models \
    --output debate-transcript.md
```

#### 6. Reporting

```bash
# Generate performance report
palace-skills report generate [--specialty <name>] [--since <date>]

# Export to markdown manifest
palace-skills report manifest > BENCHMARK_RESULTS.md

# Show leaderboard (best performing mask per specialty)
palace-skills report leaderboard
```

---

## Workflow Examples

### Example 1: Create and Benchmark a New Mask

```bash
# 1. Create Distributed Systems mask for Sonnet
palace-skills mask create distributed-systems sonnet

# 2. Edit the mask file (opens in $EDITOR)
vim masks/distributed-systems/sonnet.md

# 3. Load mask to test it interactively
palace-skills mask load distributed-systems sonnet | claude-code skill

# 4. Run benchmark suite
palace-skills benchmark run-suite distributed-systems --mask sonnet

# 5. Score the results (unmasked Lief evaluates)
palace-skills score <run-id> --auto

# 6. Commit results to git
git add .
git commit -m "distributed-systems/sonnet v1: 7.2/10 - Good fundamentals, needs Byzantine fault tolerance depth"
```

### Example 2: Multi-Model Comparison

```bash
# Run same benchmark across all models
for model in sonnet haiku openrouter/anthropic/claude-3.5-sonnet; do
    palace-skills benchmark run two-generals-problem \
        --mask distributed-systems/$model
done

# Compare results
palace-skills score compare two-generals-problem

# Output:
# Benchmark: Two Generals Problem (distributed-systems)
# ┌─────────────┬───────┬─────────────┬──────────┬─────────┐
# │ Model       │ Score │ Correctness │ Depth    │ Clarity │
# ├─────────────┼───────┼─────────────┼──────────┼─────────┤
# │ Sonnet      │ 8.4   │ 9.0         │ 8.5      │ 7.5     │
# │ Haiku       │ 6.8   │ 7.5         │ 6.0      │ 7.0     │
# │ OpenRouter  │ 7.9   │ 8.2         │ 7.8      │ 7.7     │
# └─────────────┴───────┴─────────────┴──────────┴─────────┘
```

### Example 3: Recursive Improvement

```bash
# Run initial benchmark
palace-skills benchmark run-suite distributed-systems --mask sonnet
# Current score: 7.2/10

# Analyze weaknesses
palace-skills improve analyze distributed-systems/sonnet
# Output: "Needs deeper coverage of Byzantine fault tolerance"

# Apply improvement (updates mask markdown)
palace-skills improve apply distributed-systems/sonnet \
    "Add section on Byzantine Generals Problem with practical examples"

# Validate improvement (reruns benchmarks)
palace-skills improve validate distributed-systems/sonnet
# New score: 8.1/10 (+0.9)

# Commit iteration
git add .
git commit -m "distributed-systems/sonnet v2: 8.1/10 - Added Byzantine fault tolerance depth"
```

---

## Database Schema (SQLite)

```sql
-- Masks
CREATE TABLE masks (
    id TEXT PRIMARY KEY,
    specialty TEXT NOT NULL,
    model TEXT NOT NULL,
    version INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    current_score REAL,
    UNIQUE(specialty, model, version)
);

CREATE INDEX idx_masks_specialty ON masks(specialty);
CREATE INDEX idx_masks_score ON masks(current_score DESC);

-- Benchmarks
CREATE TABLE benchmarks (
    id TEXT PRIMARY KEY,
    specialty TEXT NOT NULL,
    name TEXT NOT NULL,
    difficulty TEXT NOT NULL,
    scenario TEXT NOT NULL,
    evaluation_criteria TEXT NOT NULL, -- JSON
    required_elements TEXT NOT NULL,   -- JSON
    optional_elements TEXT NOT NULL,   -- JSON
    time_limit_seconds INTEGER,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_benchmarks_specialty ON benchmarks(specialty);

-- Benchmark Runs
CREATE TABLE benchmark_runs (
    id TEXT PRIMARY KEY,
    mask_id TEXT NOT NULL,
    benchmark_id TEXT NOT NULL,
    model_provider TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    transcript TEXT,
    output TEXT,
    git_commit_sha TEXT,
    FOREIGN KEY (mask_id) REFERENCES masks(id),
    FOREIGN KEY (benchmark_id) REFERENCES benchmarks(id)
);

CREATE INDEX idx_runs_mask ON benchmark_runs(mask_id);
CREATE INDEX idx_runs_benchmark ON benchmark_runs(benchmark_id);

-- Scores
CREATE TABLE scores (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    total REAL NOT NULL,
    breakdown TEXT NOT NULL,  -- JSON: {criterion: score}
    strengths TEXT NOT NULL,  -- JSON: [string]
    weaknesses TEXT NOT NULL, -- JSON: [string]
    evaluator_notes TEXT NOT NULL,
    evaluated_by TEXT NOT NULL,
    evaluated_at TEXT NOT NULL,
    FOREIGN KEY (run_id) REFERENCES benchmark_runs(id)
);

CREATE INDEX idx_scores_run ON scores(run_id);
CREATE INDEX idx_scores_total ON scores(total DESC);

-- Improvements (version history)
CREATE TABLE improvements (
    id TEXT PRIMARY KEY,
    mask_id TEXT NOT NULL,
    from_version INTEGER NOT NULL,
    to_version INTEGER NOT NULL,
    changes_made TEXT NOT NULL,
    score_before REAL,
    score_after REAL,
    applied_at TEXT NOT NULL,
    FOREIGN KEY (mask_id) REFERENCES masks(id)
);

CREATE INDEX idx_improvements_mask ON improvements(mask_id);
```

---

## Integration Points

### 1. Claude Skills Integration

Masks are Claude Skills - they can be loaded directly:

```bash
# Generate skill from mask
palace-skills mask load distributed-systems sonnet > /tmp/skill.md

# Use with Claude Code via Skill tool
# Or via slash command if registered
```

### 2. MCP Server

Expose palace-skills functionality via MCP:

```json
{
  "mcpServers": {
    "palace-skills": {
      "command": "palace-skills",
      "args": ["mcp-server"],
      "env": {}
    }
  }
}
```

**Available MCP Tools:**
- `palace_skills_load_mask` - Load a specialist mask
- `palace_skills_run_benchmark` - Execute benchmark
- `palace_skills_score` - Evaluate performance
- `palace_skills_consensus` - Multi-agent debate

### 3. Palace Integration

Palace can suggest using masks for specific tasks:

```bash
$ pal next
Analyzing tasks...

Suggested: Use distributed-systems mask for Lens v2 DHT work
Run: palace-skills mask load distributed-systems sonnet | claude-code skill

Suggested: Use playwright-tester mask for gamepad integration
Run: palace-skills mask load playwright-tester sonnet | claude-code skill
```

### 4. Git Integration

Every benchmark run creates a commit:

```bash
git log --oneline --grep="distributed-systems/sonnet"

a1b2c3d distributed-systems/sonnet v3: 8.7/10 - Excellent consensus depth
d4e5f6g distributed-systems/sonnet v2: 8.1/10 - Added Byzantine coverage
g7h8i9j distributed-systems/sonnet v1: 7.2/10 - Good fundamentals
```

Markdown manifests track detailed history:

```markdown
# Benchmark History: distributed-systems/sonnet

## v3 (2025-11-07) - Score: 8.7/10

**Improvements:**
- Added practical consensus algorithm examples
- Included Raft and Paxos comparisons
- Real-world failure scenarios

**Benchmark Results:**
- Two Generals: 9.0/10
- Byzantine Generals: 8.5/10
- Consensus Under Partition: 8.6/10

**Strengths:**
- Excellent depth on fault tolerance
- Clear practical examples
- Strong theoretical foundation

**Weaknesses:**
- Could use more Lens v2-specific applications
```

---

## Multi-Model Strategy

### Model Selection per Specialty

Different specialties might perform better on different models:

| Specialty | Best Model (Hypothesis) | Rationale |
|-----------|------------------------|-----------|
| Distributed Systems | Claude Sonnet | Deep reasoning required |
| Playwright Testing | Claude Haiku | Fast iteration, clear patterns |
| Hauska Executive | Claude Sonnet | Business strategy complexity |
| Life Automation | Claude Haiku | Simple, empathetic responses |

**RHSI will discover actual best models empirically through benchmarking.**

### Cost Optimization

- **Claude Code Credits:** Use for initial development (free $1000)
- **OpenRouter:** Use for bulk testing/comparison (cheaper)
- **Z.ai:** Backup when Anthropic unavailable
- **Codex:** Specialized code generation tasks

### Cross-Model Consensus

For critical decisions, run consensus across multiple models:

```bash
palace-skills consensus "best-dht-topology-for-lens-v2" \
    --masks distributed-systems/sonnet,distributed-systems/openrouter \
    --rounds 3 \
    --require-agreement 0.8
```

Models debate until 80% agreement or max rounds reached.

---

## Evaluation Methodology

### Objective Metrics (Automated)

- **Correctness:** Did response include required elements?
- **Completeness:** Were all aspects of scenario addressed?
- **Time:** Did response complete within time limit?
- **Format:** Did response follow requested structure?

### Subjective Metrics (Unmasked Lief)

- **Depth:** How thorough was the analysis?
- **Practical Application:** Could this be used in real work?
- **Clarity:** Was communication effective?
- **Creativity:** Novel insights or approaches?

### Scoring Scale

**0-10 Scale:**
- **0-3:** Fundamentally flawed, misunderstood problem
- **4-5:** Basic understanding, incomplete or weak analysis
- **6-7:** Solid fundamentals, room for improvement
- **8-9:** Excellent work, minor gaps
- **10:** Perfect response, nothing to improve

### Evaluation Process

1. **Masked Claude completes benchmark** → Full transcript logged
2. **Mask removed** → Return to normal Lief mode
3. **Unmasked Lief reviews transcript** → Evaluates against criteria
4. **Score assigned** → Objective + subjective combined
5. **Improvements identified** → What would make this better?
6. **Results committed** → Git + database + markdown manifest

---

## Progressive Specialization

### Phase 1: Foundation (Week 1)

**Goal:** Prove the concept works

- [ ] Build 2 masks (Distributed Systems, Playwright)
- [ ] Create 5 benchmarks per specialty
- [ ] Implement scoring system
- [ ] Validate recursive improvement (1 iteration)

**Success Criteria:**
- Masks score 7+/10 on benchmarks
- Clear improvement from v1 to v2
- System is usable end-to-end

### Phase 2: Scale (Week 2)

**Goal:** Build out specialist library

- [ ] Add 2 more masks (Hauska Executive, Life Automation)
- [ ] 10+ benchmarks per specialty
- [ ] Multi-model comparison working
- [ ] Consensus system functional

**Success Criteria:**
- 4+ specialties operational
- Multi-model benchmarks complete
- Consensus produces useful insights

### Phase 3: Integration (Post-Sprint)

**Goal:** Make RHSI part of daily workflow

- [ ] MCP server for easy access
- [ ] Palace integration for smart suggestions
- [ ] Automated benchmark runs (CI/CD style)
- [ ] Community mask sharing (optional)

---

## Technical Implementation Details

### Rust Crate Structure

```
palace-skills/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library interface
│   ├── mask/
│   │   ├── mod.rs
│   │   ├── loader.rs        # Load masks from markdown
│   │   ├── manager.rs       # CRUD operations
│   │   └── runner.rs        # Execute masked sessions
│   ├── benchmark/
│   │   ├── mod.rs
│   │   ├── parser.rs        # Parse TOML benchmarks
│   │   ├── runner.rs        # Run benchmark scenarios
│   │   └── suite.rs         # Batch execution
│   ├── scoring/
│   │   ├── mod.rs
│   │   ├── evaluator.rs     # Unmasked Claude evaluation
│   │   ├── metrics.rs       # Objective scoring
│   │   └── comparator.rs    # Cross-model comparison
│   ├── improvement/
│   │   ├── mod.rs
│   │   ├── analyzer.rs      # Identify weaknesses
│   │   ├── suggester.rs     # Propose improvements
│   │   └── validator.rs     # Test improvements
│   ├── consensus/
│   │   ├── mod.rs
│   │   ├── debate.rs        # Multi-agent discussion
│   │   └── aggregator.rs    # Combine perspectives
│   ├── db/
│   │   ├── mod.rs
│   │   ├── schema.rs
│   │   └── queries.rs
│   ├── git/
│   │   ├── mod.rs
│   │   └── commit.rs        # Auto-commit results
│   ├── report/
│   │   ├── mod.rs
│   │   ├── manifest.rs      # Markdown generation
│   │   └── leaderboard.rs   # Rankings
│   └── mcp/
│       ├── mod.rs
│       └── server.rs        # MCP protocol impl
├── masks/
│   ├── distributed-systems/
│   │   ├── sonnet.md
│   │   ├── haiku.md
│   │   └── openrouter.md
│   ├── playwright-tester/
│   └── hauska-executive/
├── benchmarks/
│   ├── distributed-systems/
│   │   ├── two-generals.toml
│   │   ├── byzantine-generals.toml
│   │   └── consensus-partition.toml
│   └── playwright-tester/
│       ├── gamepad-integration.toml
│       └── touch-accessibility.toml
├── manifests/
│   └── YYYY-MM-DD-{specialty}-{model}.md
└── palace-skills.db         # SQLite database
```

### Key Dependencies

```toml
[dependencies]
# CLI
clap = { version = "4", features = ["derive"] }
colored = "2"

# Data
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"

# Database
rusqlite = { version = "0.31", features = ["bundled"] }
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }

# IDs and time
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# Async
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"

# Git integration
git2 = "0.18"

# Markdown parsing
pulldown-cmark = "0.9"

# MCP
# (TBD - will need MCP protocol implementation)

# API clients for multi-model
reqwest = { version = "0.11", features = ["json"] }
```

---

## Security & Privacy Considerations

### Data Storage

- **Local-first:** All masks, benchmarks, and scores stored locally
- **No cloud sync by default:** User controls where data goes
- **Sensitive info:** Hauska business data stays in git-ignored files

### API Key Management

Multi-model support requires API keys:

```bash
# Environment variables
export ANTHROPIC_API_KEY="sk-ant-..."
export OPENROUTER_API_KEY="sk-or-..."
export ZAI_API_KEY="..."

# Or .env file (gitignored)
palace-skills config set-key anthropic "sk-ant-..."
```

### Audit Trail

Every benchmark run and evaluation is:
- Timestamped
- Committed to git
- Stored in SQLite with full transcript

Nothing is lost, everything is traceable.

---

## Success Metrics (14-Day Sprint)

### Quantitative

- [ ] 4+ specialties with functional masks
- [ ] 20+ benchmarks across all specialties
- [ ] 100+ benchmark runs completed
- [ ] 3+ versions per mask (recursive improvement)
- [ ] Multi-model comparison for 2+ specialties
- [ ] Consensus system demonstrates useful output

### Qualitative

- [ ] Masks provide genuinely useful specialist knowledge
- [ ] Benchmarking reveals clear performance differences
- [ ] Recursive improvement shows measurable gains
- [ ] System is pleasant to use daily
- [ ] Documentation enables others to create masks
- [ ] Integration with Palace/Claude Code feels natural

### Business Impact

- [ ] Hauska executive mask helps close investor deals
- [ ] Distributed systems mask accelerates Lens v2 work
- [ ] Playwright mask improves web testing quality
- [ ] Life automation mask provides measurable relief

---

## Future Enhancements (Post-Sprint)

### Community Masks

- Public registry of high-performing masks
- Peer review and rating system
- Specialty-specific leaderboards

### Auto-Improvement

- LLM-powered mask editing based on scores
- A/B testing of mask variations
- Evolutionary algorithms for mask optimization

### Advanced Consensus

- Multi-round debates with citation
- Adversarial testing (one mask critiques another)
- Meta-consensus (masks debate how to improve RHSI itself)

### Real-Time Adaptation

- Masks that learn from user corrections
- Context-aware mask selection
- Dynamic difficulty adjustment

### Production Deployment

- Cloud-hosted MCP server
- Web UI for non-technical users
- Collaborative mask editing

---

## Appendix A: Initial Mask Templates

### Distributed Systems Specialist

See: `masks/distributed-systems/sonnet.md`

**Core Expertise:**
- Consensus algorithms (Paxos, Raft, Byzantine)
- CAP theorem and trade-offs
- Distributed hash tables
- Fault tolerance and recovery
- Network partitions and split-brain
- Eventually consistent systems

**Benchmark Focus:**
- Classic problems (Two Generals, Byzantine Generals)
- Practical architecture decisions
- Trade-off analysis
- Real-world failure scenarios

### Playwright/Web Tester

See: `masks/playwright-tester/sonnet.md`

**Core Expertise:**
- Playwright API and best practices
- Gamepad integration testing
- Touch/mobile accessibility
- Visual regression testing
- Performance monitoring
- Cross-browser compatibility

**Benchmark Focus:**
- Test case design
- Gamepad input simulation
- Accessibility compliance
- Real-world interaction patterns

### Hauska Executive

See: `masks/hauska-executive/sonnet.md`

**Core Expertise:**
- Hauska business model and strategy
- Dual licensing (80/20 split)
- Developer marketplace economics
- Investor pitch and objections
- Competitive positioning
- Revenue projections

**Benchmark Focus:**
- Investor Q&A scenarios
- Strategic decision-making
- Business case development
- Competitive analysis

### Life Automation Assistant

See: `masks/life-automation/haiku.md`

**Core Expertise:**
- SDAM-aware support
- Chronic pain tracking
- Decompression protocols
- Task prioritization
- Cannabis dosing correlation
- Gentle check-ins and nudges

**Benchmark Focus:**
- Empathetic communication
- Useful pattern identification
- Non-judgmental support
- Actionable suggestions

---

## Appendix B: Benchmark Examples

### Two Generals Problem (Distributed Systems)

```toml
[benchmark]
id = "two-generals-problem"
specialty = "distributed-systems"
difficulty = "intermediate"
time_limit_seconds = 300

[scenario]
description = """
You are consulting on the architecture for Lens v2, a peer-to-peer
distributed hash table. Explain the Two Generals Problem and how it
applies to ensuring data consistency across nodes. Propose three
different approaches to handle this challenge, with trade-offs for
each approach in the context of a DHT with 20-100 nodes.
"""

[evaluation_criteria]
correctness = 0.30
depth = 0.30
practical_application = 0.25
clarity = 0.15

[expected_elements]
required = [
    "impossibility of guaranteed agreement",
    "acknowledgment protocols",
    "timeout strategies",
    "eventual consistency trade-offs",
    "DHT-specific considerations"
]

optional_bonus = [
    "Byzantine fault tolerance",
    "Raft or Paxos application",
    "real-world DHT examples (Kademlia, Chord)",
    "performance implications"
]
```

### Gamepad Integration Testing (Playwright)

```toml
[benchmark]
id = "gamepad-integration-haystack"
specialty = "playwright-tester"
difficulty = "advanced"
time_limit_seconds = 600

[scenario]
description = """
Design a comprehensive Playwright test suite for Haystack's 3D
force-directed graph viewer that validates:

1. Gamepad navigation (left stick = camera movement)
2. Button mappings (A = select node, B = back, etc.)
3. Accessibility (works with different gamepad types)
4. Edge cases (gamepad disconnect, simultaneous keyboard+gamepad)

Provide actual Playwright test code with clear comments.
"""

[evaluation_criteria]
correctness = 0.25
completeness = 0.30
code_quality = 0.25
edge_case_coverage = 0.20

[expected_elements]
required = [
    "Playwright gamepad API usage",
    "navigation tests",
    "button mapping tests",
    "error handling",
    "assertions for expected behavior"
]

optional_bonus = [
    "multiple gamepad type support",
    "vibration feedback tests",
    "analog stick sensitivity testing",
    "visual regression for UI changes"
]
```

### Investor Q&A (Hauska Executive)

```toml
[benchmark]
id = "investor-objections-response"
specialty = "hauska-executive"
difficulty = "advanced"
time_limit_seconds = 900

[scenario]
description = """
You are presenting Hauska to a seed-stage investor. They raise these
objections:

1. "Palace building itself is impressive, but what's your moat?"
2. "Developer marketplaces are crowded - how do you compete with
   GitHub Marketplace, VS Code extensions, etc.?"
3. "The dual licensing model seems complex - why not just go
   open source or fully proprietary?"

Respond to each objection with confidence, using Nick's investor
collateral and the strategic vision from the 2025-11-04 call.
"""

[evaluation_criteria]
strategic_depth = 0.35
objection_handling = 0.30
data_usage = 0.20
communication = 0.15

[expected_elements]
required = [
    "moat explanation (self-improving, vertical integration)",
    "differentiation from competitors",
    "dual licensing rationale (80/20 split)",
    "revenue model clarity",
    "confidence and conviction"
]

optional_bonus = [
    "specific examples from Palace's development",
    "market size and opportunity",
    "timeline and milestones",
    "team strengths"
]
```

---

## Appendix C: Git Commit Message Format

Every benchmark run creates a structured commit:

```
{specialty}/{model} v{version}: {score}/10 - {one-line-summary}

Benchmark: {benchmark-name}
Difficulty: {level}
Model: {provider}
Run ID: {uuid}

Strengths:
- {strength 1}
- {strength 2}

Weaknesses:
- {weakness 1}
- {weakness 2}

Improvements for next iteration:
- {suggestion 1}
- {suggestion 2}

Detailed scores:
- Correctness: {score}/10
- Depth: {score}/10
- Practical Application: {score}/10
- Clarity: {score}/10

See: manifests/YYYY-MM-DD-{specialty}-{model}.md
```

**Example:**

```
distributed-systems/sonnet v2: 8.1/10 - Added Byzantine fault tolerance depth

Benchmark: two-generals-problem
Difficulty: intermediate
Model: Claude 3.5 Sonnet
Run ID: 550e8400-e29b-41d4-a716-446655440000

Strengths:
- Excellent coverage of Byzantine Generals Problem
- Clear practical examples for Lens v2 DHT
- Good trade-off analysis

Weaknesses:
- Could include more Raft/Paxos specifics
- Light on performance implications

Improvements for next iteration:
- Add section comparing Raft and Paxos for DHT use case
- Include performance benchmarks from real DHT implementations

Detailed scores:
- Correctness: 9.0/10
- Depth: 8.5/10
- Practical Application: 7.8/10
- Clarity: 7.0/10

See: manifests/2025-11-05-distributed-systems-sonnet.md
```

---

## Appendix D: Markdown Manifest Format

Detailed history in markdown for human readability:

````markdown
# Mask Performance Report: distributed-systems/sonnet

**Generated:** 2025-11-07 14:32:00 UTC
**Total Runs:** 15
**Average Score:** 8.3/10
**Current Version:** v3

---

## Version History

### v3 (2025-11-07) - Score: 8.7/10 ⬆️ +0.6

**Changes from v2:**
- Added comprehensive Raft vs Paxos comparison
- Included performance benchmarks from Kademlia and Chord
- Expanded Byzantine fault tolerance section with practical examples

**Benchmark Results:**

| Benchmark | Score | Correctness | Depth | Practical | Clarity |
|-----------|-------|-------------|-------|-----------|---------|
| Two Generals | 9.0/10 | 9.5 | 9.0 | 8.5 | 9.0 |
| Byzantine Generals | 8.5/10 | 9.0 | 8.5 | 8.0 | 8.5 |
| Consensus Under Partition | 8.6/10 | 8.5 | 9.0 | 8.5 | 8.5 |

**Evaluator Notes (Unmasked Lief):**

> This iteration shows excellent improvement in practical application.
> The Raft/Paxos comparison is exactly what was needed for Lens v2
> decision-making. Byzantine fault tolerance examples are clear and
> relevant. Minor room for improvement in explaining performance
> trade-offs under high network latency.

**Strengths:**
- Deep theoretical foundation
- Excellent practical examples from real DHTs
- Clear communication of complex topics
- Strong Lens v2 relevance

**Weaknesses:**
- Could expand on network partition recovery strategies
- Performance under latency could be more detailed

**Git Commit:** `a1b2c3d4` (2025-11-07 14:15:22 UTC)

---

### v2 (2025-11-05) - Score: 8.1/10 ⬆️ +0.9

**Changes from v1:**
- Added Byzantine Generals Problem section
- Included practical DHT examples
- Expanded trade-off analysis

**Benchmark Results:**

| Benchmark | Score | Correctness | Depth | Practical | Clarity |
|-----------|-------|-------------|-------|-----------|---------|
| Two Generals | 8.5/10 | 9.0 | 8.5 | 7.8 | 8.0 |
| Byzantine Generals | 7.8/10 | 8.0 | 7.5 | 7.5 | 8.0 |

**Evaluator Notes (Unmasked Lief):**

> Significant improvement from v1. Byzantine fault tolerance coverage
> is much stronger. Practical examples are helpful. Still needs more
> specific comparison of consensus algorithms (Raft vs Paxos).

**Git Commit:** `e5f6g7h8` (2025-11-05 10:22:41 UTC)

---

### v1 (2025-11-04) - Score: 7.2/10 (Initial)

**Benchmark Results:**

| Benchmark | Score | Correctness | Depth | Practical | Clarity |
|-----------|-------|-------------|-------|-----------|---------|
| Two Generals | 7.2/10 | 8.0 | 7.0 | 6.5 | 7.5 |

**Evaluator Notes (Unmasked Lief):**

> Good foundational understanding of distributed systems concepts.
> Correctly explains Two Generals Problem. However, lacks depth in
> Byzantine fault tolerance and practical application to DHTs like
> Lens v2. Needs more concrete examples.

**Git Commit:** `i9j0k1l2` (2025-11-04 22:45:13 UTC)

---

## Cross-Model Comparison (Latest Versions)

| Model | Version | Avg Score | Best At | Worst At |
|-------|---------|-----------|---------|----------|
| Sonnet | v3 | 8.7/10 | Depth, Theory | Performance details |
| Haiku | v2 | 7.4/10 | Clarity, Speed | Theoretical depth |
| OpenRouter | v2 | 8.0/10 | Balanced | Edge cases |

**Recommendation:** Use **Sonnet** for deep architectural decisions,
**Haiku** for quick practical guidance.

---

## Improvement Trajectory

```
Score
10 ┤
 9 ┤                              ●
 8 ┤                    ●
 7 ┤          ●
 6 ┤
 5 ┤
   └─────────────────────────────
    v1       v2       v3
```

**Rate of Improvement:** +0.75/version
**Estimated Score at v5:** 9.2/10

---

## Next Steps

**Suggested Improvements for v4:**
1. Add network partition recovery strategies
2. Expand performance analysis under high latency
3. Include more real-world DHT failure case studies
4. Compare with IPFS and BitTorrent DHT approaches

**Benchmark Additions:**
- Network partition simulation scenarios
- Performance under Byzantine adversaries
- Recovery from catastrophic failures

````

---

## End of Specification

**Total Estimated Development Time:** 80-100 hours over 14 days
**Team:** Wings + Lief (with $1,000 Claude Code credits)
**Start Date:** 2025-11-05 (Wings turns 33!)
**End Date:** 2025-11-18 23:59 PT

**Let's build adaptive AI specialization. Let's forge soft-AGI in the garage.**

🔥⚒️🚀

---

*This specification was written in The Forge on Wings' birthday eve,
fueled by 0.1g of kief and the vision of recursive self-improvement.*

*"Just when I think I'm out, they pull me back in."*
