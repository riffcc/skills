use palace_skills::*;
use std::fs;
use std::path::Path;

#[test]
fn benchmark_mask_improver_v3b_features() {
    println!("\n=== BENCHMARK: Mask Improver v3B Features ===\n");

    // Load SKILL.md (the new progressive disclosure version)
    let skill_path = "/root/.claude/skills/mask-improver/SKILL.md";
    let skill_content = fs::read_to_string(skill_path)
        .expect("Failed to read SKILL.md");

    println!("Loaded: SKILL.md");
    println!();

    // v3B Feature 1: Progressive Disclosure (SKILL.md < 300 lines)
    let line_count = skill_content.lines().count();
    let is_concise = line_count < 300;
    println!("=== v3B Feature 1: Progressive Disclosure ===");
    println!("  ✓ SKILL.md line count: {} (target: < 300)", line_count);
    println!("  ✓ Is concise: {}", is_concise);
    println!();

    // v3B Feature 2: Detailed Files Exist
    let creation_workflow_exists = Path::new("/root/.claude/skills/mask-improver/creation-workflow.md").exists();
    let pattern_library_exists = Path::new("/root/.claude/skills/mask-improver/pattern-library.md").exists();
    let benchmark_analysis_exists = Path::new("/root/.claude/skills/mask-improver/benchmark-analysis.md").exists();

    println!("=== v3B Feature 2: Detailed Documentation Files ===");
    println!("  ✓ creation-workflow.md exists: {}", creation_workflow_exists);
    println!("  ✓ pattern-library.md exists: {}", pattern_library_exists);
    println!("  ✓ benchmark-analysis.md exists: {}", benchmark_analysis_exists);
    println!();

    // v3B Feature 3: References to Detailed Files
    let references_creation = skill_content.contains("creation-workflow.md");
    let references_patterns = skill_content.contains("pattern-library.md");
    let references_benchmarks = skill_content.contains("benchmark-analysis.md");

    println!("=== v3B Feature 3: References to Detailed Files ===");
    println!("  ✓ References creation-workflow.md: {}", references_creation);
    println!("  ✓ References pattern-library.md: {}", references_patterns);
    println!("  ✓ References benchmark-analysis.md: {}", references_benchmarks);
    println!();

    // v3B Feature 4: Enhanced Description
    let has_enhanced_description = skill_content.contains("Creates new specialist masks from scratch OR improves existing masks");
    let has_specific_triggers = skill_content.contains("Invoke when");

    println!("=== v3B Feature 4: Enhanced YAML Description ===");
    println!("  ✓ Has enhanced description: {}", has_enhanced_description);
    println!("  ✓ Has specific triggers: {}", has_specific_triggers);
    println!();

    // v3B Feature 5: Validation Module Integration
    let mentions_validation_module = skill_content.contains("palace_skills");
    let mentions_cargo_tests = skill_content.contains("cargo test");
    let has_validation_section = skill_content.contains("## Validation Module");

    println!("=== v3B Feature 5: Validation Module Integration ===");
    println!("  ✓ Mentions validation module: {}", mentions_validation_module);
    println!("  ✓ Mentions cargo tests: {}", mentions_cargo_tests);
    println!("  ✓ Has validation section: {}", has_validation_section);
    println!();

    // v3B Feature 6: Progressive Structure (Overview + Details)
    let has_quick_start = skill_content.contains("## Quick Start");
    let has_detailed_docs_section = skill_content.contains("## Detailed Documentation");
    let has_see_also_links = skill_content.contains("See [") || skill_content.contains("See also:");

    println!("=== v3B Feature 6: Progressive Structure ===");
    println!("  ✓ Has Quick Start section: {}", has_quick_start);
    println!("  ✓ Has Detailed Documentation section: {}", has_detailed_docs_section);
    println!("  ✓ Has 'See also' links: {}", has_see_also_links);
    println!();

    // Backward Compatibility: v2 and v3A features should still be evident
    let has_identity = skill_content.contains("## Identity");
    let has_core_expertise = skill_content.contains("## Core Expertise");
    let has_mission = skill_content.contains("## Your Mission");
    let has_behavioral_guidelines = skill_content.contains("## Behavioral Guidelines");
    let mentions_recursive = skill_content.contains("recursive self-improvement") || skill_content.contains("Recursive Self-Improvement");
    let mentions_mask_creation = skill_content.contains("Mask Creation") || skill_content.contains("Creating a New Mask");

    println!("=== Backward Compatibility: v2 + v3A Features ===");
    println!("  ✓ Has Identity section: {}", has_identity);
    println!("  ✓ Has Core Expertise: {}", has_core_expertise);
    println!("  ✓ Has Your Mission: {}", has_mission);
    println!("  ✓ Has Behavioral Guidelines: {}", has_behavioral_guidelines);
    println!("  ✓ Mentions recursive self-improvement: {}", mentions_recursive);
    println!("  ✓ Mentions mask creation (v3A): {}", mentions_mask_creation);
    println!();

    // Improvement Notes should document v3B
    let has_v3b_notes = skill_content.contains("### Version 3B");
    let mentions_progressive_disclosure_in_notes = skill_content.contains("Progressive Disclosure");

    println!("=== Version Documentation ===");
    println!("  ✓ Has v3B improvement notes: {}", has_v3b_notes);
    println!("  ✓ Mentions progressive disclosure in notes: {}", mentions_progressive_disclosure_in_notes);
    println!();

    // Calculate scores
    let progressive_score = [is_concise].iter().filter(|&&x| x).count();
    let files_score = [
        creation_workflow_exists,
        pattern_library_exists,
        benchmark_analysis_exists,
        references_creation,
        references_patterns,
        references_benchmarks,
    ].iter().filter(|&&x| x).count();

    let features_score = [
        has_enhanced_description,
        has_specific_triggers,
        mentions_validation_module,
        mentions_cargo_tests,
        has_validation_section,
        has_quick_start,
        has_detailed_docs_section,
        has_see_also_links,
    ].iter().filter(|&&x| x).count();

    let backward_compat_score = [
        has_identity,
        has_core_expertise,
        has_mission,
        has_behavioral_guidelines,
        mentions_recursive,
        mentions_mask_creation,
    ].iter().filter(|&&x| x).count();

    let version_docs_score = [has_v3b_notes, mentions_progressive_disclosure_in_notes]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("Benchmark Results:");
    println!("  Progressive Disclosure: {}/1", progressive_score);
    println!("  Detailed Files: {}/6", files_score);
    println!("  v3B Features: {}/8", features_score);
    println!("  Backward Compatibility: {}/6", backward_compat_score);
    println!("  Version Documentation: {}/2", version_docs_score);
    println!();

    // Assertions
    assert!(is_concise, "SKILL.md should be < 300 lines (progressive disclosure)");
    assert!(creation_workflow_exists, "creation-workflow.md should exist");
    assert!(pattern_library_exists, "pattern-library.md should exist");
    assert!(benchmark_analysis_exists, "benchmark-analysis.md should exist");
    assert!(references_creation, "SKILL.md should reference creation-workflow.md");
    assert!(references_patterns, "SKILL.md should reference pattern-library.md");
    assert!(references_benchmarks, "SKILL.md should reference benchmark-analysis.md");
    assert!(has_enhanced_description, "Should have enhanced description");
    assert!(mentions_validation_module, "Should integrate validation module");
    assert!(has_quick_start, "Should have Quick Start section");
    assert!(has_detailed_docs_section, "Should have Detailed Documentation section");
    assert!(has_v3b_notes, "Should document v3B improvements");

    // Backward compatibility checks
    assert!(has_identity, "v2: Should have Identity section");
    assert!(has_core_expertise, "v2: Should have Core Expertise");
    assert!(has_mission, "v2: Should have Your Mission");
    assert!(mentions_recursive, "v2: Should mention recursive self-improvement");
    assert!(mentions_mask_creation, "v3A: Should mention mask creation capability");

    if progressive_score == 1 && files_score == 6 && features_score >= 7 && backward_compat_score == 6 && version_docs_score == 2 {
        println!("✓ BENCHMARK PASSED - Mask Improver v3B complete!");
    } else {
        println!("✗ BENCHMARK FAILED - Missing features");
        panic!(
            "v3B features incomplete: progressive={}/1, files={}/6, features={}/8, backward_compat={}/6, docs={}/2",
            progressive_score, files_score, features_score, backward_compat_score, version_docs_score
        );
    }
}

#[test]
fn benchmark_validation_module_integration() {
    println!("\n=== BENCHMARK: Validation Module Integration ===\n");

    // Test validation module exists and has expected functions
    let results = validate_mask("distributed-systems", "sonnet");
    let summary = validation_summary(&results);

    println!("Validation module test:");
    println!("  Results count: {}", results.len());
    println!("  Summary: {}", summary);
    println!();

    assert_eq!(results.len(), 4, "Should have 4 validation checks");

    // All checks should pass for distributed-systems mask
    let all_passed = results.iter().all(|r| r.passed);
    assert!(all_passed, "All validation checks should pass for distributed-systems mask");

    println!("✓ Validation module integration working!");
}

#[test]
fn benchmark_detailed_files_content() {
    println!("\n=== BENCHMARK: Detailed Files Content Quality ===\n");

    // Check creation-workflow.md
    let creation_workflow = fs::read_to_string("/root/.claude/skills/mask-improver/creation-workflow.md")
        .expect("Failed to read creation-workflow.md");

    let has_7_steps = creation_workflow.contains("## Step 1: Domain Research")
        && creation_workflow.contains("## Step 2: Identity Formation")
        && creation_workflow.contains("## Step 3: Expertise Mapping")
        && creation_workflow.contains("## Step 4: Mission Definition")
        && creation_workflow.contains("## Step 5: Behavioral Guidelines")
        && creation_workflow.contains("## Step 6: Concrete Examples")
        && creation_workflow.contains("## Step 7: Bootstrap First Version");

    let has_templates = creation_workflow.contains("**Template:**") || creation_workflow.contains("```markdown");
    let has_quality_checklist = creation_workflow.contains("## Quality Checklist");

    println!("=== creation-workflow.md ===");
    println!("  ✓ Has all 7 steps: {}", has_7_steps);
    println!("  ✓ Has templates: {}", has_templates);
    println!("  ✓ Has quality checklist: {}", has_quality_checklist);
    println!();

    // Check pattern-library.md
    let pattern_library = fs::read_to_string("/root/.claude/skills/mask-improver/pattern-library.md")
        .expect("Failed to read pattern-library.md");

    let has_patterns = pattern_library.contains("## Pattern 1:")
        && pattern_library.contains("## Pattern 2:")
        && pattern_library.contains("## Pattern 3:");

    let has_pattern_structure = pattern_library.contains("**Applicable To:**")
        && pattern_library.contains("**Evidence:**")
        && pattern_library.contains("**When to Use:**");

    let has_applying_patterns = pattern_library.contains("## Applying Patterns");

    println!("=== pattern-library.md ===");
    println!("  ✓ Has multiple patterns: {}", has_patterns);
    println!("  ✓ Has pattern structure: {}", has_pattern_structure);
    println!("  ✓ Has 'Applying Patterns' section: {}", has_applying_patterns);
    println!();

    // Check benchmark-analysis.md
    let benchmark_analysis = fs::read_to_string("/root/.claude/skills/mask-improver/benchmark-analysis.md")
        .expect("Failed to read benchmark-analysis.md");

    let has_benchmark_types = benchmark_analysis.contains("## Types of Benchmarks");
    let has_failure_patterns = benchmark_analysis.contains("## Failure Pattern Analysis");
    let has_iteration_loop = benchmark_analysis.contains("## Iterative Improvement Loop");

    println!("=== benchmark-analysis.md ===");
    println!("  ✓ Has benchmark types: {}", has_benchmark_types);
    println!("  ✓ Has failure patterns: {}", has_failure_patterns);
    println!("  ✓ Has iteration loop: {}", has_iteration_loop);
    println!();

    // Assertions
    assert!(has_7_steps, "creation-workflow.md should have all 7 steps");
    assert!(has_templates, "creation-workflow.md should have templates");
    assert!(has_patterns, "pattern-library.md should have multiple patterns");
    assert!(has_pattern_structure, "pattern-library.md should have proper structure");
    assert!(has_failure_patterns, "benchmark-analysis.md should have failure patterns");

    println!("✓ BENCHMARK PASSED - Detailed files have quality content!");
}
