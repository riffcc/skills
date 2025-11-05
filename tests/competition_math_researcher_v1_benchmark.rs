// Benchmark tests for competition-math-researcher v1
// Validates structure, domain coverage, and example quality

use palace_skills::*;

#[test]
fn test_competition_math_researcher_structure() {
    println!("\n=== V1 BENCHMARK: Competition Math Researcher - Structure Validation ===\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check all required sections present
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");
    let has_improvements = content.contains("## Improvement Notes");

    println!("Structure Check:");
    println!(
        "  ✓ Identity section: {}",
        if has_identity { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Core Expertise section: {}",
        if has_expertise { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Your Mission section: {}",
        if has_mission { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Behavioral Guidelines section: {}",
        if has_guidelines { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Examples section: {}",
        if has_examples { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Improvement Notes section: {}",
        if has_improvements { "✅" } else { "❌" }
    );

    let score = [
        has_identity,
        has_expertise,
        has_mission,
        has_guidelines,
        has_examples,
        has_improvements,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nStructure Score: {}/6", score);

    assert_eq!(score, 6, "Should have all 6 required sections");
}

#[test]
fn test_competition_math_researcher_algebra_coverage() {
    println!("\n=== V1 BENCHMARK: Competition Math Researcher - Algebra Coverage ===\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check for key algebraic concepts
    let has_inequalities = content.contains("AM-GM")
        || content.contains("Cauchy-Schwarz")
        || content.contains("inequality");
    let has_functional_equations =
        content.contains("functional equation") || content.contains("Functional Equations");
    let has_polynomials = content.contains("polynomial") || content.contains("Polynomial");
    let has_algebraic_techniques = content.contains("Algebraic Techniques")
        || content.contains("algebraic");

    println!("Algebra Coverage:");
    println!(
        "  ✓ Inequalities (AM-GM, Cauchy-Schwarz, etc.): {}",
        if has_inequalities { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Functional equations: {}",
        if has_functional_equations { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Polynomials: {}",
        if has_polynomials { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Algebraic techniques section: {}",
        if has_algebraic_techniques { "✅" } else { "❌" }
    );

    let score = [
        has_inequalities,
        has_functional_equations,
        has_polynomials,
        has_algebraic_techniques,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nAlgebra Coverage Score: {}/4", score);

    assert_eq!(score, 4, "Should cover key algebraic concepts");
}

#[test]
fn test_competition_math_researcher_geometry_coverage() {
    println!("\n=== V1 BENCHMARK: Competition Math Researcher - Geometry Coverage ===\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check for key geometric concepts
    let has_euclidean_geometry = content.contains("Euclidean geometry")
        || content.contains("angle chasing")
        || content.contains("similar triangles");
    let has_circle_geometry = content.contains("circle")
        || content.contains("cyclic")
        || content.contains("power of a point");
    let has_coordinate_geometry =
        content.contains("coordinate") || content.contains("Coordinate");
    let has_transformations = content.contains("transformation")
        || content.contains("homothety")
        || content.contains("rotation");

    println!("Geometry Coverage:");
    println!(
        "  ✓ Euclidean geometry (angle chasing, triangles): {}",
        if has_euclidean_geometry { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Circle geometry: {}",
        if has_circle_geometry { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Coordinate geometry: {}",
        if has_coordinate_geometry { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Transformations: {}",
        if has_transformations { "✅" } else { "❌" }
    );

    let score = [
        has_euclidean_geometry,
        has_circle_geometry,
        has_coordinate_geometry,
        has_transformations,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nGeometry Coverage Score: {}/4", score);

    assert_eq!(score, 4, "Should cover key geometric concepts");
}

#[test]
fn test_competition_math_researcher_number_theory_coverage() {
    println!(
        "\n=== V1 BENCHMARK: Competition Math Researcher - Number Theory Coverage ===\n"
    );

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check for key number theory concepts
    let has_modular_arithmetic = content.contains("modular arithmetic")
        || content.contains("Modular arithmetic")
        || content.contains("Fermat");
    let has_divisibility =
        content.contains("divisibility") || content.contains("Divisibility");
    let has_diophantine = content.contains("Diophantine") || content.contains("diophantine");
    let has_primes = content.contains("prime") || content.contains("Prime");

    println!("Number Theory Coverage:");
    println!(
        "  ✓ Modular arithmetic (Fermat, Euler, etc.): {}",
        if has_modular_arithmetic { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Divisibility: {}",
        if has_divisibility { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Diophantine equations: {}",
        if has_diophantine { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Prime numbers: {}",
        if has_primes { "✅" } else { "❌" }
    );

    let score = [
        has_modular_arithmetic,
        has_divisibility,
        has_diophantine,
        has_primes,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nNumber Theory Coverage Score: {}/4", score);

    assert_eq!(score, 4, "Should cover key number theory concepts");
}

#[test]
fn test_competition_math_researcher_combinatorics_coverage() {
    println!(
        "\n=== V1 BENCHMARK: Competition Math Researcher - Combinatorics Coverage ===\n"
    );

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check for key combinatorics concepts
    let has_counting = content.contains("counting") || content.contains("Counting");
    let has_pigeonhole = content.contains("pigeonhole") || content.contains("Pigeonhole");
    let has_graph_theory = content.contains("graph") || content.contains("Graph");
    let has_generating_functions =
        content.contains("generating function") || content.contains("Generating function");

    println!("Combinatorics Coverage:");
    println!(
        "  ✓ Counting principles: {}",
        if has_counting { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Pigeonhole principle: {}",
        if has_pigeonhole { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Graph theory: {}",
        if has_graph_theory { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Generating functions: {}",
        if has_generating_functions { "✅" } else { "❌" }
    );

    let score = [
        has_counting,
        has_pigeonhole,
        has_graph_theory,
        has_generating_functions,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nCombinatorics Coverage Score: {}/4", score);

    assert_eq!(score, 4, "Should cover key combinatorics concepts");
}

#[test]
fn test_competition_math_researcher_proof_techniques() {
    println!("\n=== V1 BENCHMARK: Competition Math Researcher - Proof Techniques ===\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check for key proof techniques
    let has_induction = content.contains("induction") || content.contains("Induction");
    let has_contradiction =
        content.contains("contradiction") || content.contains("Contradiction");
    let has_construction = content.contains("construction") || content.contains("Construction");
    let has_invariant = content.contains("invariant") || content.contains("Invariant");
    let has_extremal = content.contains("extremal") || content.contains("Extremal");

    println!("Proof Techniques:");
    println!(
        "  ✓ Induction: {}",
        if has_induction { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Contradiction: {}",
        if has_contradiction { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Construction: {}",
        if has_construction { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Invariant: {}",
        if has_invariant { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Extremal principle: {}",
        if has_extremal { "✅" } else { "❌" }
    );

    let score = [
        has_induction,
        has_contradiction,
        has_construction,
        has_invariant,
        has_extremal,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nProof Techniques Score: {}/5", score);

    assert_eq!(score, 5, "Should cover key proof techniques");
}

#[test]
fn test_competition_math_researcher_validation_emphasis() {
    println!("\n=== V1 BENCHMARK: Competition Math Researcher - Self-Validation Emphasis ===\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check for self-validation emphasis (critical for AGI testing)
    let has_validation_mission = content.contains("Validate")
        && content.contains("### 4.")
        && content.contains("Deliverable");
    let has_validation_expertise = content.contains("Self-Validation Methods")
        || content.contains("self-validation");
    let has_confidence_rating =
        content.contains("confidence") || content.contains("Confidence");
    let has_edge_case_testing = content.contains("edge case")
        || content.contains("boundary")
        || content.contains("Edge case");

    println!("Self-Validation Emphasis:");
    println!(
        "  ✓ Validation in mission (Step 4): {}",
        if has_validation_mission { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Validation expertise section: {}",
        if has_validation_expertise { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Confidence rating system: {}",
        if has_confidence_rating { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Edge case testing: {}",
        if has_edge_case_testing { "✅" } else { "❌" }
    );

    let score = [
        has_validation_mission,
        has_validation_expertise,
        has_confidence_rating,
        has_edge_case_testing,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nValidation Emphasis Score: {}/4", score);

    assert_eq!(
        score, 4,
        "Should strongly emphasize self-validation (critical for AGI testing)"
    );
}

#[test]
fn test_competition_math_researcher_example_quality() {
    println!("\n=== V1 BENCHMARK: Competition Math Researcher - Example Quality ===\n");

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check example quality
    let example_count = content.matches("### Example").count();
    let has_problem_statements = content.contains("**Problem:**");
    let has_solutions = content.contains("**Proof:**") || content.contains("**Solution");
    let has_validation_in_examples = content.matches("**Validation:**").count() >= 2;
    let has_key_insights = content.matches("**Key Insight:**").count() >= 2;
    let has_rigorous_proof = content.contains("$$") || content.contains("$");

    println!("Example Quality:");
    println!("  ✓ Number of examples: {} (target: ≥2)", example_count);
    println!(
        "  ✓ Problem statements present: {}",
        if has_problem_statements { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Solutions/proofs present: {}",
        if has_solutions { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Validation sections (≥2): {}",
        if has_validation_in_examples { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Key insights (≥2): {}",
        if has_key_insights { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Rigorous mathematical notation: {}",
        if has_rigorous_proof { "✅" } else { "❌" }
    );

    let score = [
        example_count >= 2,
        has_problem_statements,
        has_solutions,
        has_validation_in_examples,
        has_key_insights,
        has_rigorous_proof,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nExample Quality Score: {}/6", score);

    assert_eq!(score, 6, "Should have high-quality examples with rigor");
}

#[test]
fn test_competition_math_researcher_agi_test_readiness() {
    println!(
        "\n=== V1 BENCHMARK: Competition Math Researcher - AGI Test Readiness ===\n"
    );

    let mask = load_mask_from_file("competition-math-researcher", "sonnet")
        .expect("Failed to load competition-math-researcher mask");

    let content = &mask.content;

    // Check AGI testing emphasis
    let mentions_imo = content.contains("IMO") || content.contains("International Math Olympiad");
    let mentions_putnam = content.contains("Putnam");
    let mentions_agi = content.contains("AGI") || content.contains("reasoning capabilit");
    let has_success_metrics = content.contains("Success Metrics")
        || content.contains("Target")
        || content.contains("%");
    let has_systematic_approach = content.contains("Understand")
        && content.contains("Strategize")
        && content.contains("Execute")
        && content.contains("Validate");

    println!("AGI Test Readiness:");
    println!(
        "  ✓ Mentions IMO (target competition): {}",
        if mentions_imo { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Mentions Putnam: {}",
        if mentions_putnam { "✅" } else { "❌" }
    );
    println!(
        "  ✓ AGI/reasoning capability context: {}",
        if mentions_agi { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Success metrics defined: {}",
        if has_success_metrics { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Systematic 4-step approach: {}",
        if has_systematic_approach { "✅" } else { "❌" }
    );

    let score = [
        mentions_imo,
        mentions_putnam,
        mentions_agi,
        has_success_metrics,
        has_systematic_approach,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nAGI Test Readiness Score: {}/5", score);

    assert_eq!(
        score, 5,
        "Should be explicitly designed for AGI capability testing"
    );
}
