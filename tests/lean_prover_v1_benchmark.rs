// Benchmark tests for lean-prover mask v1
// These validate that the mask has the necessary expertise to perform formal theorem proving in Lean 4

use palace_skills::*;

#[test]
fn test_lean_prover_structure() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Structure ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");
    let has_improvements = content.contains("## Improvement Notes");

    println!("Structure validation:");
    println!("  ✓ Identity section: {}", if has_identity { "✅" } else { "❌" });
    println!("  ✓ Core Expertise: {}", if has_expertise { "✅" } else { "❌" });
    println!("  ✓ Mission section: {}", if has_mission { "✅" } else { "❌" });
    println!("  ✓ Behavioral Guidelines: {}", if has_guidelines { "✅" } else { "❌" });
    println!("  ✓ Examples: {}", if has_examples { "✅" } else { "❌" });
    println!("  ✓ Improvement Notes: {}", if has_improvements { "✅" } else { "❌" });

    let score = [has_identity, has_expertise, has_mission, has_guidelines, has_examples, has_improvements]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/6", score);

    assert_eq!(score, 6, "All structure sections should be present");
}

#[test]
fn test_lean_prover_type_theory() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Type Theory Foundations ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for type theory concepts
    let has_dependent_types = content.contains("dependent type") || content.contains("Dependent Type");
    let has_curry_howard = content.contains("Curry-Howard") || content.contains("propositions as types");
    let has_type_universes = content.contains("type universe") || content.contains("Type");
    let has_function_types = content.contains("function type") || content.contains("→");

    println!("Type Theory Checks:");
    println!("  ✓ Dependent types: {}", if has_dependent_types { "✅" } else { "❌" });
    println!("  ✓ Curry-Howard correspondence: {}", if has_curry_howard { "✅" } else { "❌" });
    println!("  ✓ Type universes: {}", if has_type_universes { "✅" } else { "❌" });
    println!("  ✓ Function types: {}", if has_function_types { "✅" } else { "❌" });

    let score = [has_dependent_types, has_curry_howard, has_type_universes, has_function_types]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/4", score);

    assert!(
        score >= 3,
        "Should have at least 3/4 type theory concepts covered"
    );
}

#[test]
fn test_lean_prover_core_tactics() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Core Tactics ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for essential Lean 4 tactics
    let has_intro = content.contains("intro");
    let has_apply = content.contains("apply");
    let has_exact = content.contains("exact");
    let has_rw = content.contains("rw") || content.contains("rewrite");
    let has_cases = content.contains("cases");
    let has_induction = content.contains("induction");
    let has_constructor = content.contains("constructor");
    let has_calc = content.contains("calc");

    println!("Core Tactics Documentation:");
    println!("  ✓ intro: {}", if has_intro { "✅" } else { "❌" });
    println!("  ✓ apply: {}", if has_apply { "✅" } else { "❌" });
    println!("  ✓ exact: {}", if has_exact { "✅" } else { "❌" });
    println!("  ✓ rw/rewrite: {}", if has_rw { "✅" } else { "❌" });
    println!("  ✓ cases: {}", if has_cases { "✅" } else { "❌" });
    println!("  ✓ induction: {}", if has_induction { "✅" } else { "❌" });
    println!("  ✓ constructor: {}", if has_constructor { "✅" } else { "❌" });
    println!("  ✓ calc: {}", if has_calc { "✅" } else { "❌" });

    let score = [has_intro, has_apply, has_exact, has_rw, has_cases, has_induction, has_constructor, has_calc]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/8", score);

    assert!(
        score >= 6,
        "Should document at least 6/8 core tactics"
    );
}

#[test]
fn test_lean_prover_logical_connectives() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Logical Connectives ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for logical connectives and operations
    let has_conjunction = content.contains("∧") || content.contains("And");
    let has_disjunction = content.contains("∨") || content.contains("Or");
    let has_negation = content.contains("¬") || content.contains("Not");
    let has_implication = content.contains("→") || content.contains("implication");
    let has_biconditional = content.contains("↔") || content.contains("iff");

    println!("Logical Connectives:");
    println!("  ✓ Conjunction (∧): {}", if has_conjunction { "✅" } else { "❌" });
    println!("  ✓ Disjunction (∨): {}", if has_disjunction { "✅" } else { "❌" });
    println!("  ✓ Negation (¬): {}", if has_negation { "✅" } else { "❌" });
    println!("  ✓ Implication (→): {}", if has_implication { "✅" } else { "❌" });
    println!("  ✓ Biconditional (↔): {}", if has_biconditional { "✅" } else { "❌" });

    let score = [has_conjunction, has_disjunction, has_negation, has_implication, has_biconditional]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/5", score);

    assert_eq!(
        score, 5,
        "All logical connectives should be documented"
    );
}

#[test]
fn test_lean_prover_quantifiers() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Quantifiers ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for quantifier support
    let has_universal = content.contains("∀") || content.contains("Universal");
    let has_existential = content.contains("∃") || content.contains("Existential");
    let has_witness = content.contains("witness") || content.contains("use") || content.contains("exists");

    println!("Quantifier Support:");
    println!("  ✓ Universal (∀): {}", if has_universal { "✅" } else { "❌" });
    println!("  ✓ Existential (∃): {}", if has_existential { "✅" } else { "❌" });
    println!("  ✓ Witness construction: {}", if has_witness { "✅" } else { "❌" });

    let score = [has_universal, has_existential, has_witness]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/3", score);

    assert_eq!(
        score, 3,
        "All quantifier concepts should be covered"
    );
}

#[test]
fn test_lean_prover_classical_logic() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Classical Logic ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for classical logic support
    let has_lem = content.contains("excluded middle") || content.contains("em");
    let has_contradiction = content.contains("contradiction") || content.contains("byContradiction");
    let has_classical_keyword = content.contains("classical") || content.contains("Classical");
    let mentions_constructive = content.contains("constructive") || content.contains("Constructive");

    println!("Classical Logic Coverage:");
    println!("  ✓ Law of excluded middle: {}", if has_lem { "✅" } else { "❌" });
    println!("  ✓ Proof by contradiction: {}", if has_contradiction { "✅" } else { "❌" });
    println!("  ✓ Classical logic keyword: {}", if has_classical_keyword { "✅" } else { "❌" });
    println!("  ✓ Constructive vs classical: {}", if mentions_constructive { "✅" } else { "❌" });

    let score = [has_lem, has_contradiction, has_classical_keyword, mentions_constructive]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/4", score);

    assert!(
        score >= 3,
        "Should cover classical logic concepts adequately"
    );
}

#[test]
fn test_lean_prover_proof_patterns() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Proof Patterns ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for proof pattern documentation
    let has_direct = content.contains("direct proof") || content.contains("Direct proof");
    let has_cases_pattern = content.contains("proof by cases") || content.contains("case analysis");
    let has_induction_pattern = content.contains("induction") || content.contains("Induction");
    let has_have_pattern = content.contains("have") && content.contains("intermediate");
    let has_calc_pattern = content.contains("calc") && content.contains("calculational");

    println!("Proof Pattern Documentation:");
    println!("  ✓ Direct proof: {}", if has_direct { "✅" } else { "❌" });
    println!("  ✓ Proof by cases: {}", if has_cases_pattern { "✅" } else { "❌" });
    println!("  ✓ Proof by induction: {}", if has_induction_pattern { "✅" } else { "❌" });
    println!("  ✓ 'have' for lemmas: {}", if has_have_pattern { "✅" } else { "❌" });
    println!("  ✓ 'calc' for chains: {}", if has_calc_pattern { "✅" } else { "❌" });

    let score = [has_direct, has_cases_pattern, has_induction_pattern, has_have_pattern, has_calc_pattern]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/5", score);

    assert!(
        score >= 4,
        "Should document at least 4/5 proof patterns"
    );
}

#[test]
fn test_lean_prover_examples_quality() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Examples Quality ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Count examples and check quality markers
    let example_count = content.matches("### Example").count();
    let has_formal_proofs = content.contains("```lean");
    let has_explanations = content.contains("**Explanation:**");
    let has_strategies = content.contains("**Strategy:**") || content.contains("**Informal Strategy:**");
    let has_induction_example = content.contains("induction") && content.contains("```lean");
    let has_classical_example = content.contains("Classical") && content.contains("```lean");

    println!("Example Quality:");
    println!("  ✓ Number of examples: {}", example_count);
    println!("  ✓ Formal Lean proofs: {}", if has_formal_proofs { "✅" } else { "❌" });
    println!("  ✓ Explanations provided: {}", if has_explanations { "✅" } else { "❌" });
    println!("  ✓ Proof strategies: {}", if has_strategies { "✅" } else { "❌" });
    println!("  ✓ Induction example: {}", if has_induction_example { "✅" } else { "❌" });
    println!("  ✓ Classical logic example: {}", if has_classical_example { "✅" } else { "❌" });

    let quality_score = [has_formal_proofs, has_explanations, has_strategies, has_induction_example, has_classical_example]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nExample count: {}", example_count);
    println!("Quality score: {}/5", quality_score);

    assert!(
        example_count >= 3,
        "Should have at least 3 examples"
    );
    assert!(
        quality_score >= 4,
        "Examples should have high quality (4/5+)"
    );
}

#[test]
fn test_lean_prover_mathlib_integration() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Mathlib Integration ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for mathlib awareness
    let mentions_mathlib = content.contains("mathlib") || content.contains("Mathlib");
    let has_import_example = content.contains("import");
    let mentions_library_size = content.contains("210,000") || content.contains("theorems");

    println!("Mathlib Integration:");
    println!("  ✓ Mentions mathlib: {}", if mentions_mathlib { "✅" } else { "❌" });
    println!("  ✓ Shows import usage: {}", if has_import_example { "✅" } else { "❌" });
    println!("  ✓ References library scale: {}", if mentions_library_size { "✅" } else { "❌" });

    let score = [mentions_mathlib, has_import_example, mentions_library_size]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/3", score);

    assert!(
        score >= 2,
        "Should demonstrate mathlib integration awareness"
    );
}

#[test]
fn test_lean_prover_mission_structure() {
    println!("\n=== V1 BENCHMARK: Lean Prover - Mission Structure ===\n");

    let mask = load_mask_from_file("lean-prover", "sonnet")
        .expect("Failed to load lean-prover mask");

    let content = &mask.content;

    // Check for 4-step mission structure
    let has_analyze = content.contains("### 1. Analyze") || content.contains("## 1. Analyze");
    let has_design = content.contains("### 2. Design") || content.contains("## 2. Design");
    let has_construct = content.contains("### 3. Construct") || (content.contains("## 3.") && content.contains("Construct"));
    let has_verify = content.contains("### 4. Verify") || (content.contains("## 4.") && content.contains("Verify"));

    println!("Mission Structure (4-step workflow):");
    println!("  ✓ Step 1 - Analyze: {}", if has_analyze { "✅" } else { "❌" });
    println!("  ✓ Step 2 - Design: {}", if has_design { "✅" } else { "❌" });
    println!("  ✓ Step 3 - Construct: {}", if has_construct { "✅" } else { "❌" });
    println!("  ✓ Step 4 - Verify: {}", if has_verify { "✅" } else { "❌" });

    let score = [has_analyze, has_design, has_construct, has_verify]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/4", score);

    assert_eq!(
        score, 4,
        "Should have complete 4-step mission structure"
    );
}
