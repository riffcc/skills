use palace_skills::*;

/// Benchmark: Mask Improver Self-Improvement
///
/// Tests the Mask Improver's ability to analyze and improve itself.
///
/// Success Criteria:
/// - Identifies at least 2 concrete improvements
/// - Improvements are specific (not vague)
/// - Improvements are actionable (can be implemented)
/// - Maintains mask identity (still a Mask Improver)
#[test]
fn benchmark_mask_improver_self_improvement() {
    println!("\n=== BENCHMARK: Mask Improver Self-Improvement ===\n");

    // Load the Mask Improver
    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load Mask Improver");

    println!("Loaded: Mask Improver v{}", mask.version);
    println!("Specialty: {}", mask.specialty);
    println!();

    // Simulate asking the mask to improve itself
    println!("Task: Analyze yourself and suggest improvements");
    println!();

    // In a real benchmark, this would invoke Claude with the mask loaded
    // For now, we verify the mask structure is correct for self-improvement

    let content = &mask.content;

    // Verification criteria
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_examples = content.contains("## Examples of Good Improvements");
    let has_validation = content.contains("## Validation Strategy");
    let has_prioritization = content.contains("**Prioritize Improvements**");

    println!("Verification:");
    println!("  ✓ Has Identity section: {}", has_identity);
    println!("  ✓ Has Core Expertise: {}", has_expertise);
    println!("  ✓ Has Mission: {}", has_mission);
    println!("  ✓ Has Examples (v2+): {}", has_examples);
    println!("  ✓ Has Validation Strategy (v2+): {}", has_validation);
    println!("  ✓ Has Prioritization (v2+): {}", has_prioritization);
    println!();

    // V2 improvements
    let v2_score = [has_examples, has_validation, has_prioritization]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("V2 Improvements Present: {}/3", v2_score);
    println!();

    // Assert baseline requirements
    assert!(has_identity, "Missing Identity section");
    assert!(has_expertise, "Missing Core Expertise section");
    assert!(has_mission, "Missing Your Mission section");

    // V2 should have all improvements
    if mask.version >= 2 {
        assert!(has_examples, "V2+ should have Examples section");
        assert!(has_validation, "V2+ should have Validation Strategy");
        assert!(has_prioritization, "V2+ should have Prioritization framework");
    }

    println!("✓ BENCHMARK PASSED");
    println!();

    // Benchmark metadata
    println!("Benchmark Results:");
    println!("  Mask Version: v{}", mask.version);
    println!("  Structure Score: {}/6",
        [has_identity, has_expertise, has_mission, has_examples, has_validation, has_prioritization]
            .iter()
            .filter(|&&x| x)
            .count()
    );
    println!("  V2 Features: {}/3", v2_score);
}

/// Benchmark: Mask Improver on Low-Scoring Mask
///
/// Creates a deliberately weak mask and tests if Mask Improver can suggest good improvements.
///
/// Success Criteria:
/// - Identifies specific weaknesses
/// - Suggests concrete, actionable improvements
/// - Improvements are relevant to the weak mask's domain
#[test]
fn benchmark_mask_improver_on_weak_mask() {
    println!("\n=== BENCHMARK: Mask Improver on Weak Mask ===\n");

    // Create a deliberately weak "distributed systems" mask
    let weak_mask_content = r#"---
name: distributed-systems-weak
description: Knows about distributed systems
---

# Distributed Systems

I know about distributed systems. I can help with:
- Networks
- Databases
- Stuff

When you ask me about distributed systems, I'll try to help.
"#;

    println!("Weak Mask Content:");
    println!("{}", weak_mask_content);
    println!();

    // In a real benchmark, we'd load Mask Improver and have it analyze this
    // For now, verify the weak mask is actually weak

    let has_specific_expertise = weak_mask_content.contains("Byzantine") ||
                                 weak_mask_content.contains("CAP theorem") ||
                                 weak_mask_content.contains("consensus");
    let has_clear_mission = weak_mask_content.contains("## Your Mission") ||
                           weak_mask_content.contains("## Mission");
    let has_examples = weak_mask_content.contains("## Examples");

    println!("Weak Mask Analysis:");
    println!("  Has specific expertise: {}", has_specific_expertise);
    println!("  Has clear mission: {}", has_clear_mission);
    println!("  Has examples: {}", has_examples);
    println!();

    // This mask should be weak
    assert!(!has_specific_expertise, "Test mask shouldn't have specific expertise yet");
    assert!(!has_clear_mission, "Test mask shouldn't have clear mission yet");
    assert!(!has_examples, "Test mask shouldn't have examples yet");

    println!("Expected Improvements:");
    println!("  - Add specific distributed systems concepts (CAP, Byzantine, Paxos, Raft)");
    println!("  - Add clear mission statement with steps");
    println!("  - Add concrete examples of distributed systems problems");
    println!("  - Add behavioral guidelines");
    println!();

    println!("✓ BENCHMARK SETUP COMPLETE");
    println!("  (Full benchmark requires Claude invocation to generate improvements)");
}
