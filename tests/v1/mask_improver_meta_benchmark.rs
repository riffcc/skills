use palace_skills::*;

/// Benchmarks for the Mask Improver itself
/// These are "meta-benchmarks" - testing the improver's ability to improve masks

#[test]
fn benchmark_mask_improver_structure() {
    println!("\n=== BENCHMARK: Mask Improver - Structure Validation ===\n");

    let mask =
        load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask-improver mask");

    let content = &mask.content;

    // Core sections
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_output_format = content.contains("## Output Format");

    assert!(has_identity, "Mask must have Identity section");
    assert!(has_expertise, "Mask must have Core Expertise section");
    assert!(has_mission, "Mask must have Your Mission section");
    assert!(
        has_guidelines,
        "Mask must have Behavioral Guidelines section"
    );
    assert!(
        has_output_format,
        "Mask improver should have Output Format section"
    );

    println!("✓ Structure: 5/5 required sections present");
}

#[test]
fn benchmark_mask_improver_analysis_capability() {
    println!("\n=== BENCHMARK: Mask Improver - Analysis Capabilities ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for analysis capabilities
    let has_gap_identification = content.contains("gap") || content.contains("Gap");
    let has_performance_analysis = content.contains("performance") || content.contains("benchmark");
    let has_pattern_recognition = content.contains("pattern") || content.contains("Pattern");
    let has_strengths_analysis = content.contains("strength") || content.contains("Strength");

    assert!(has_gap_identification, "Should identify gaps");
    assert!(
        has_performance_analysis,
        "Should analyze performance/benchmarks"
    );
    assert!(has_pattern_recognition, "Should recognize patterns");

    let score = [
        has_gap_identification,
        has_performance_analysis,
        has_pattern_recognition,
        has_strengths_analysis,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("✓ Analysis: {}/4 capabilities covered", score);
}

#[test]
fn benchmark_mask_improver_specificity() {
    println!("\n=== BENCHMARK: Mask Improver - Specificity Requirements ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for emphasis on specificity
    let emphasizes_specific = content.contains("Be Specific") || content.contains("specific");
    let has_concrete = content.contains("concrete") || content.contains("Concrete");
    let has_actionable = content.contains("actionable");
    let avoids_vague = content.contains("vague") || content.contains("not \"improve\"");

    assert!(emphasizes_specific, "Should emphasize specificity");
    assert!(
        has_concrete || has_actionable,
        "Should require concrete/actionable improvements"
    );

    println!("✓ Specificity: Emphasized in guidelines");
}

#[test]
fn benchmark_mask_improver_evidence_based() {
    println!("\n=== BENCHMARK: Mask Improver - Evidence-Based Analysis ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for evidence-based approach
    let has_evidence = content.contains("evidence") || content.contains("Evidence");
    let has_benchmark_data = content.contains("benchmark") || content.contains("Benchmark");
    let has_data_driven = content.contains("data") || content.contains("performance");
    let avoids_guessing = content.contains("don't guess") || content.contains("not guess");

    assert!(has_benchmark_data, "Should reference benchmark data");

    let score = [
        has_evidence,
        has_benchmark_data,
        has_data_driven,
        avoids_guessing,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("✓ Evidence-Based: {}/4 aspects covered", score);
}

#[test]
fn benchmark_mask_improver_incremental() {
    println!("\n=== BENCHMARK: Mask Improver - Incremental Improvements ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for incremental approach
    let has_incremental = content.contains("Incremental") || content.contains("incremental");
    let has_small_changes = content.contains("small") || content.contains("2-4");
    let avoids_rewrites = content.contains("not wholesale")
        || content.contains("not massive")
        || content.contains("don't rewrite");

    assert!(
        has_incremental || has_small_changes,
        "Should emphasize incremental improvements"
    );

    println!("✓ Incremental: Emphasized in guidelines");
}

#[test]
fn benchmark_mask_improver_prioritization() {
    println!("\n=== BENCHMARK: Mask Improver - Prioritization Framework ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for prioritization capability (V3 improvement)
    let has_prioritization = content.contains("priorit") || content.contains("Priorit");
    let has_impact = content.contains("impact") || content.contains("Impact");
    let has_effort = content.contains("effort") || content.contains("Effort");
    let has_difficulty = content.contains("difficulty") || content.contains("Difficulty");

    // This may fail on V2, pass on V3
    let score = [has_prioritization, has_impact, has_effort, has_difficulty]
        .iter()
        .filter(|&&x| x)
        .count();

    if score >= 2 {
        println!("✓ Prioritization: {}/4 concepts covered", score);
    } else {
        println!(
            "⚠ Prioritization: {}/4 concepts covered (could be improved)",
            score
        );
    }
}

#[test]
fn benchmark_mask_improver_implementation_details() {
    println!("\n=== BENCHMARK: Mask Improver - Implementation Detail Requirements ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for emphasis on implementation details (V3 improvement)
    let has_exact_text = content.contains("exact") || content.contains("Exact");
    let has_before_after =
        content.contains("before/after") || content.contains("Before:") || content.contains("diff");
    let has_implementation =
        content.contains("Implementation") || content.contains("implementation");
    let has_rationale = content.contains("Rationale") || content.contains("rationale");

    let score = [
        has_exact_text,
        has_before_after,
        has_implementation,
        has_rationale,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    if score >= 3 {
        println!("✓ Implementation Details: {}/4 requirements covered", score);
    } else {
        println!(
            "⚠ Implementation Details: {}/4 requirements covered (could be improved)",
            score
        );
    }
}

#[test]
fn benchmark_mask_improver_validation() {
    println!("\n=== BENCHMARK: Mask Improver - Validation Framework ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for validation framework
    let has_validation = content.contains("Validat") || content.contains("validat");
    let has_expected_impact =
        content.contains("Expected Impact") || content.contains("expected impact");
    let has_benchmark_prediction =
        content.contains("predict") || content.contains("should improve");
    let has_side_effects = content.contains("side effect") || content.contains("maintain");

    let score = [
        has_validation,
        has_expected_impact,
        has_benchmark_prediction,
        has_side_effects,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        score >= 2,
        "Should have validation framework (found {}/4)",
        score
    );

    println!("✓ Validation: {}/4 validation aspects covered", score);
}

#[test]
fn benchmark_mask_improver_meta_learning() {
    println!("\n=== BENCHMARK: Mask Improver - Meta-Learning Capability ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for meta-learning (ability to improve itself)
    let has_meta = content.contains("meta") || content.contains("Meta");
    let has_self_improvement = content.contains("For Next Time")
        || content.contains("improve ME")
        || content.contains("improve itself");
    let has_pattern_learning =
        content.contains("what types of improvements work") || content.contains("what makes masks");

    let score = [has_meta, has_self_improvement, has_pattern_learning]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(score >= 1, "Should have some meta-learning capability");

    println!("✓ Meta-Learning: {}/3 meta-learning aspects covered", score);
}

#[test]
fn benchmark_mask_improver_output_structure() {
    println!("\n=== BENCHMARK: Mask Improver - Structured Output Format ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for structured output requirements
    let has_output_format = content.contains("Output Format");
    let has_current_analysis =
        content.contains("Current Analysis") || content.contains("Analyze Current");
    let has_proposed_improvements =
        content.contains("Proposed Improvement") || content.contains("propose");
    let has_expected_impact = content.contains("Expected Impact");

    let score = [
        has_output_format,
        has_current_analysis,
        has_proposed_improvements,
        has_expected_impact,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        score >= 3,
        "Should define structured output (found {}/4)",
        score
    );

    println!(
        "✓ Output Structure: {}/4 output requirements defined",
        score
    );
}
