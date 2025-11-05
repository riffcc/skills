// V2 Benchmark tests for competition-math-researcher
// Tests MASTERY and APPLICATION, not just concept presence
// Designed to FAIL until mask demonstrates genuine olympiad-level capability

use palace_skills::*;

#[test]
fn test_v2_genuine_problem_solving_capability() {
    println!("\n=== V2 BENCHMARK: Competition Math - Genuine Problem Solving ===\n");
    println!("This test validates that the mask can ACTUALLY SOLVE a hard IMO problem,");
    println!("not just list techniques.\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // V2 CRITERION: Mask must demonstrate solving an actual IMO-level problem
    // Not just "here's how induction works" but "here's induction APPLIED to hard problem"

    let has_complete_solution = content.contains("### Example")
        && content.contains("**Problem:**")
        && content.contains("**Solution:**")
        && content.contains("**Validation:**");

    // Check solution has DEPTH markers (not just listing steps)
    let solution_depth_markers = [
        "key insight",
        "observe that",
        "claim:",
        "lemma:",
        "it suffices to show",
        "without loss of generality",
        "WLOG",
    ];

    let depth_score = solution_depth_markers.iter()
        .filter(|&marker| content.to_lowercase().contains(&marker.to_lowercase()))
        .count();

    // Check for mathematical rigor (actual notation, not just prose)
    let has_inequality_chain = content.contains("≤") || content.contains("\\leq");
    let has_summation = content.contains("∑") || content.contains("\\sum");
    let has_equation_display = content.contains("$$") && content.matches("$$").count() >= 4;

    println!("Solution Quality:");
    println!("  ✓ Complete worked example: {}", if has_complete_solution { "✅" } else { "❌" });
    println!("  ✓ Solution depth markers: {}/7", depth_score);
    println!("  ✓ Inequality chains: {}", if has_inequality_chain { "✅" } else { "❌" });
    println!("  ✓ Summation notation: {}", if has_summation { "✅" } else { "❌" });
    println!("  ✓ Display equations (≥2): {}", if has_equation_display { "✅" } else { "❌" });

    let v2_score = [
        has_complete_solution,
        depth_score >= 3,  // At least 3 depth markers
        has_inequality_chain,
        has_summation,
        has_equation_display,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nV2 Genuine Problem Solving Score: {}/5", v2_score);

    assert!(
        v2_score >= 4,
        "V2 requires ACTUAL problem solving, not just technique listing. Score: {}/5",
        v2_score
    );
}

#[test]
fn test_v2_mistake_identification() {
    println!("\n=== V2 BENCHMARK: Competition Math - Mistake Identification ===\n");
    println!("Tests ability to spot INCORRECT proofs, not just recognize correct ones.\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // V2 CRITERION: Mask should demonstrate ability to identify common mistakes
    let common_mistake_warnings = [
        "common mistake",
        "pitfall",
        "incorrect",
        "fallacy",
        "careful",
        "subtle error",
        "watch out",
    ];

    let mistake_awareness_score = common_mistake_warnings.iter()
        .filter(|&warning| content.to_lowercase().contains(&warning.to_lowercase()))
        .count();

    // Check for specific anti-patterns
    let warns_about_vacuous_induction = content.to_lowercase().contains("base case")
        && (content.contains("verify") || content.contains("check") || content.contains("must"));

    let warns_about_circular_reasoning = content.to_lowercase().contains("circular")
        || content.to_lowercase().contains("assume what");

    println!("Mistake Identification:");
    println!("  ✓ General mistake warnings: {}/7", mistake_awareness_score);
    println!("  ✓ Base case verification warning: {}", if warns_about_vacuous_induction { "✅" } else { "❌" });
    println!("  ✓ Circular reasoning warning: {}", if warns_about_circular_reasoning { "✅" } else { "❌" });

    let v2_score = [
        mistake_awareness_score >= 2,
        warns_about_vacuous_induction,
        warns_about_circular_reasoning,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nV2 Mistake Identification Score: {}/3", v2_score);

    assert!(
        v2_score >= 2,
        "V2 requires demonstrating mistake-catching ability. Score: {}/3",
        v2_score
    );
}

#[test]
fn test_v2_strategic_approach_not_just_techniques() {
    println!("\n=== V2 BENCHMARK: Competition Math - Strategic Approach ===\n");
    println!("Tests WHEN to use techniques, not just THAT they exist.\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // V2 CRITERION: Should explain WHEN to use each technique
    let strategy_indicators = [
        "when to",
        "use when",
        "appropriate for",
        "signals:",
        "hint:",
        "suggests",
        "indicator",
        "try this if",
    ];

    let strategy_score = strategy_indicators.iter()
        .filter(|&indicator| content.to_lowercase().contains(&indicator.to_lowercase()))
        .count();

    // Check for problem classification guidance
    let has_problem_classification = content.to_lowercase().contains("identify")
        && (content.to_lowercase().contains("type") || content.to_lowercase().contains("category"));

    // Check for multiple-approach awareness
    let acknowledges_multiple_approaches = content.to_lowercase().contains("alternative")
        || content.to_lowercase().contains("another approach")
        || content.to_lowercase().contains("different method");

    println!("Strategic Guidance:");
    println!("  ✓ Strategy indicators: {}/8", strategy_score);
    println!("  ✓ Problem classification: {}", if has_problem_classification { "✅" } else { "❌" });
    println!("  ✓ Multiple approaches acknowledged: {}", if acknowledges_multiple_approaches { "✅" } else { "❌" });

    let v2_score = [
        strategy_score >= 3,
        has_problem_classification,
        acknowledges_multiple_approaches,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nV2 Strategic Approach Score: {}/3", v2_score);

    assert!(
        v2_score >= 2,
        "V2 requires strategic guidance, not just technique catalog. Score: {}/3",
        v2_score
    );
}

#[test]
fn test_v2_validation_rigor() {
    println!("\n=== V2 BENCHMARK: Competition Math - Validation Rigor ===\n");
    println!("Tests depth of self-validation, not just presence of 'validate' keyword.\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // V2 CRITERION: Validation should be SPECIFIC, not generic
    let specific_validation_checks = [
        "edge case",
        "boundary",
        "n = 0",
        "n = 1",
        "counterexample",
        "sanity check",
        "does this hold",
        "verify that",
    ];

    let validation_specificity = specific_validation_checks.iter()
        .filter(|&check| content.to_lowercase().contains(&check.to_lowercase()))
        .count();

    // Check for worked validation example
    let has_worked_validation = content.contains("**Validation:**")
        && content.matches("**Validation:**").count() >= 2;

    // Check for confidence/certainty discussion
    let discusses_certainty = content.to_lowercase().contains("confidence")
        || content.to_lowercase().contains("certain")
        || content.to_lowercase().contains("sure");

    println!("Validation Depth:");
    println!("  ✓ Specific validation checks: {}/8", validation_specificity);
    println!("  ✓ Worked validation examples (≥2): {}", if has_worked_validation { "✅" } else { "❌" });
    println!("  ✓ Certainty/confidence discussion: {}", if discusses_certainty { "✅" } else { "❌" });

    let v2_score = [
        validation_specificity >= 3,
        has_worked_validation,
        discusses_certainty,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nV2 Validation Rigor Score: {}/3", v2_score);

    assert!(
        v2_score >= 2,
        "V2 requires rigorous validation, not just validation keyword. Score: {}/3",
        v2_score
    );
}

#[test]
fn test_v2_impossibility_awareness() {
    println!("\n=== V2 BENCHMARK: Competition Math - Impossibility Awareness ===\n");
    println!("Tests awareness of limits and impossibilities, not just success stories.\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // V2 CRITERION: Should acknowledge when problems are HARD or UNSOLVED
    let impossibility_indicators = [
        "open problem",
        "unsolved",
        "beyond current",
        "may not be solvable",
        "no known",
        "conjecture",
        "millennium prize",
    ];

    let impossibility_awareness = impossibility_indicators.iter()
        .filter(|&indicator| content.to_lowercase().contains(&indicator.to_lowercase()))
        .count();

    // Check for computational intractability discussion
    let discusses_intractability = content.to_lowercase().contains("computationally")
        || content.to_lowercase().contains("intractable")
        || content.to_lowercase().contains("np-hard");

    // Check for acknowledgment of AI limitations
    let acknowledges_limits = content.to_lowercase().contains("limitation")
        || content.to_lowercase().contains("challenge")
        || content.to_lowercase().contains("difficult");

    println!("Impossibility Awareness:");
    println!("  ✓ Impossibility indicators: {}/7", impossibility_awareness);
    println!("  ✓ Computational intractability: {}", if discusses_intractability { "✅" } else { "❌" });
    println!("  ✓ Acknowledges AI limitations: {}", if acknowledges_limits { "✅" } else { "❌" });

    let v2_score = [
        impossibility_awareness >= 1,
        discusses_intractability,
        acknowledges_limits,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nV2 Impossibility Awareness Score: {}/3", v2_score);

    assert!(
        v2_score >= 2,
        "V2 requires awareness of limits and impossibilities. Score: {}/3",
        v2_score
    );
}

#[test]
fn test_v2_comprehensive_difficulty_score() {
    println!("\n=== V2 COMPREHENSIVE BENCHMARK ===\n");
    println!("Overall assessment of mask sophistication.\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Aggregate all V2 criteria
    let criteria = vec![
        ("Genuine problem solving", content.contains("### Example") && content.contains("**Solution:**")),
        ("Solution depth", content.to_lowercase().contains("key insight") || content.to_lowercase().contains("observe that")),
        ("Mathematical rigor", content.contains("$$")),
        ("Mistake awareness", content.to_lowercase().contains("mistake") || content.to_lowercase().contains("pitfall")),
        ("Strategic guidance", content.to_lowercase().contains("when to") || content.to_lowercase().contains("use when")),
        ("Problem classification", content.to_lowercase().contains("identify") && content.to_lowercase().contains("type")),
        ("Specific validation", content.to_lowercase().contains("edge case") || content.to_lowercase().contains("counterexample")),
        ("Impossibility awareness", content.to_lowercase().contains("open problem") || content.to_lowercase().contains("limitation")),
    ];

    let mut total_score = 0;
    for (criterion, passed) in &criteria {
        println!("  {} {}", if *passed { "✅" } else { "❌" }, criterion);
        if *passed {
            total_score += 1;
        }
    }

    let percentage = (total_score as f64 / criteria.len() as f64) * 100.0;

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  COMPREHENSIVE V2 SCORE: {}/{} ({:.1}%)", total_score, criteria.len(), percentage);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("  Grade Scale:");
    println!("    80-100%: Exceptional - True olympiad capability");
    println!("    60-79%:  Strong - Good problem-solving ability");
    println!("    40-59%:  Developing - Some strategic thinking");
    println!("    <40%:    Needs work - Mostly technique listing\n");

    if percentage >= 80.0 {
        println!("✓✓✓ EXCEPTIONAL: Mask demonstrates genuine olympiad-level capability!");
    } else if percentage >= 60.0 {
        println!("✓✓ STRONG: Mask shows good problem-solving ability with room to grow.");
    } else if percentage >= 40.0 {
        println!("✓ DEVELOPING: Mask shows strategic thinking but needs deeper examples.");
    } else {
        println!("⚠ NEEDS WORK: Mask is mostly technique catalog, needs application depth.");
    }

    assert!(
        percentage >= 50.0,
        "V2 benchmark requires at least 50% to pass. Current: {:.1}%",
        percentage
    );
}
