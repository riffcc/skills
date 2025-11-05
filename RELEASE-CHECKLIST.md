# Release Checklist: competition-math-researcher v1.1

## Status: ✅ Ready for Release (Manual Steps Required)

---

## What We Built

### 1. Competition Math Researcher Mask v1.1 ✅
- **Domain specialist** for olympiad-level mathematics
- **7 core expertise areas:** Algebra, Geometry, Number Theory, Combinatorics, Proof Techniques, Heuristics, Validation
- **Computational tools:** Python for pattern discovery, Rust for performance testing
- **Systematic approach:** Understand → Strategize → Execute → Validate
- **All benchmarks passing:** 9/9 structural validation tests

### 2. Release Package ✅
- **SKILL.md** - Claude Skills format (install directly to Claude Desktop)
- **README.md** - Installation and usage instructions
- **Benchmarks** - 9 structural tests validating mask quality
- **Test Problems** - 12 real competition problems (AMC, AIME, IMO, Putnam)
- **ZIP file** - `releases/competition-math-researcher-v1.1.zip` (25KB)

### 3. Release Infrastructure ✅
- **Design document:** `docs/pal-skill-release-design.md`
  - Complete specification for `pal skill release` command
  - Version detection, packaging, GitHub integration
  - Implementation plan across 3 phases
- **Release script:** `scripts/create-github-release.sh`
  - Automated tag creation and release
  - Fallback instructions for manual completion
- **Git tag created:** `competition-math-researcher-v1.1`

---

## Manual Steps to Complete Release

The git tag has been created locally but couldn't be pushed due to branch permissions (403 error). Complete the release manually:

### Step 1: Merge Pull Request

1. Go to: https://github.com/riffcc/palace-skills/pulls
2. Find PR for branch: `claude/competition-math-researcher-011CUpe5Nhn5q8fVSu3JrTru`
3. Review changes:
   - competition-math-researcher mask v1.1
   - Benchmarks (9/9 passing)
   - Test suite (12 problems)
   - Release package (ZIP)
   - Release infrastructure (design + script)
4. **Merge** the PR to main/master

### Step 2: Push Git Tag

```bash
# After PR is merged, on main branch:
git checkout main
git pull origin main

# Push the tag
git push origin competition-math-researcher-v1.1
```

### Step 3: Create GitHub Release

#### Option A: Using gh CLI (Recommended)

```bash
gh release create competition-math-researcher-v1.1 \
  releases/competition-math-researcher-v1.1.zip \
  --title "competition-math-researcher v1.1" \
  --notes-file releases/RELEASE-NOTES-v1.1.md \
  --prerelease
```

#### Option B: Manual (if gh CLI unavailable)

1. Go to: https://github.com/riffcc/palace-skills/releases/new
2. **Choose tag:** competition-math-researcher-v1.1
3. **Title:** competition-math-researcher v1.1
4. **Description:** Copy from `releases/RELEASE-NOTES-v1.1.md`
5. **Upload file:** `releases/competition-math-researcher-v1.1.zip`
6. **Check:** "This is a pre-release" (experimental status)
7. **Click:** "Publish release"

---

## Release Highlights

### Purpose: Testing AGI-Like Reasoning

This mask tests whether AI can demonstrate **genuine mathematical reasoning** on a domain outside the creator's expertise:

- ✅ **Objectively measurable** - Proofs work or don't
- ✅ **Genuinely hard** - IMO/Putnam are extremely challenging
- ✅ **Requires insight** - Can't memorize solutions
- ✅ **Tests generalization** - Not just encoding known patterns

**Success Metrics:**
- AMC 10/12: Target 90%+ correctness
- AIME: Target 70%+ correctness
- IMO: Target 40%+ correctness
- Putnam: Target 20%+ correctness

### Key Features

**v1.1 Enhancements:**
- Computational verification (Python/Rust)
- Pattern discovery through small case testing
- Exhaustive search for finite cases
- Formal verification preparation (Lean integration)

**Philosophy:** Computation supplements proof, doesn't replace it.

---

## Files Included in Release

```
competition-math-researcher-v1.1.zip
├── SKILL.md                                    # Claude Skills format
├── README.md                                   # Installation & usage
├── competition-math-researcher/sonnet.md       # Source mask
├── competition_math_researcher_v1_benchmark.rs # Benchmarks
└── competition_problems_suite.md               # 12 test problems
```

---

## Next Steps After Release

### Immediate (Post-Release)

1. **Test installation** - Verify ZIP installs correctly in Claude Desktop
2. **Test functionality** - Try solving a few problems from test suite
3. **Announce** - Share on relevant channels

### Short-Term (This Week)

1. **Run benchmark suite** - Test mask against all 12 problems
2. **Analyze results:**
   - Correctness rate by difficulty
   - Proof quality assessment
   - Confidence calibration
3. **Document findings** - AGI-like reasoning vs expertise encoding

### Medium-Term (Next Sprint)

1. **Implement `pal skill release`** - Automate this entire process
   - Version detection from mask
   - Automatic packaging
   - GitHub API integration
   - One command: `pal skill release competition-math-researcher`
2. **Expand test suite** - Add more problems if needed
3. **Iterate on mask** - Based on benchmark results

---

## Success Criteria

**Release is successful if:**
- ✅ ZIP installs correctly in Claude Desktop
- ✅ Mask activates when asked to solve competition math
- ✅ Mask produces rigorous proofs (not just correct answers)
- ✅ Computational verification code runs correctly

**AGI test is successful if:**
- ✅ IMO problems: 40%+ correctness with rigorous proofs
- ✅ Performance degrades gracefully (not cliff-off)
- ✅ Confidence calibration is accurate
- ✅ Can self-identify when problem is beyond capability

---

## Links

- **Repository:** https://github.com/riffcc/palace-skills
- **Branch:** https://github.com/riffcc/palace-skills/tree/claude/competition-math-researcher-011CUpe5Nhn5q8fVSu3JrTru
- **Design Doc:** `docs/pal-skill-release-design.md`
- **Release Notes:** `releases/RELEASE-NOTES-v1.1.md`

---

## Automation Roadmap

### Phase 1: Core Release (Next Sprint)
- Implement `pal skill release` command
- Version detection from mask files
- ZIP packaging with SKILL.md
- Git tagging automation

### Phase 2: GitHub Integration (Sprint +1)
- GitHub API integration
- Release notes generation
- Asset upload
- Error handling

### Phase 3: Advanced Features (Sprint +2)
- Auto-detection of breaking changes
- Changelog aggregation
- Pre-release validation
- Analytics integration

**End Goal:** One command releases everything:
```bash
pal skill release competition-math-researcher
```

---

**Built in The Forge, 2025-11-05** 🔥⚒️🧮

**Status:** Ready for manual release after PR merge
