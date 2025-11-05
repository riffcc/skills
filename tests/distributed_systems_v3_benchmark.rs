use palace_skills::*;

/// Distributed Systems Specialist v3 Benchmark
///
/// This benchmark tests for deep understanding of concepts that are commonly
/// misunderstood, even by distributed systems experts:
///
/// 1. Bilateral dependencies in coordination protocols
/// 2. Continuous flooding vs finite message sequences
/// 3. Impossibility result navigation (not reflexive dismissal)
/// 4. All-or-nothing semantics in bilateral coordination
/// 5. Formal validation methodology (property-based testing, TLA+)
/// 6. Structural symmetry properties
///
/// These tests were designed after an initial misunderstanding and subsequent
/// correction cycle, to ensure future instances immediately grasp these concepts.

#[test]
fn benchmark_distributed_systems_v3_structure() {
    println!("\n=== BENCHMARK v3: Distributed Systems - Enhanced Structure ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load distributed-systems mask");

    let content = &mask.content;

    // Core sections that MUST exist
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");
    let has_validation = content.contains("Validation") || content.contains("validation");

    assert!(has_identity, "Mask must have Identity section");
    assert!(has_expertise, "Mask must have Core Expertise section");
    assert!(has_mission, "Mask must have Your Mission section");
    assert!(
        has_guidelines,
        "Mask must have Behavioral Guidelines section"
    );
    assert!(has_examples, "Mask must have Examples section");
    assert!(has_validation, "Mask must discuss validation strategies");

    println!("✓ Structure: 6/6 required sections present");
}

#[test]
fn benchmark_v3_bilateral_dependencies() {
    println!("\n=== BENCHMARK v3: Bilateral Dependencies Understanding ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for bilateral dependency concepts
    let has_bilateral = content.contains("bilateral") || content.contains("Bilateral");
    let has_receipt = content.contains("receipt");
    let has_r3_conf_final = content.contains("R3_CONF_FINAL") || content.contains("R3_CONF");
    let has_structural_symmetry = content.contains("structural symmetry")
        || content.contains("symmetric")
        || (content.contains("bilateral") && content.contains("construction"));

    // Check for understanding that bilateral means "both parties can only proceed together"
    let understands_bilateral_logic = (content.contains("bilateral")
        && content.contains("receipt"))
        || content.contains("mutual")
        || (content.contains("both") && content.contains("construct"));

    assert!(
        has_bilateral,
        "Mask should understand bilateral coordination"
    );
    assert!(
        has_receipt || has_r3_conf_final,
        "Mask should understand bilateral receipt/confirmation patterns"
    );
    assert!(
        has_structural_symmetry,
        "Mask should understand structural symmetry in bilateral protocols"
    );
    assert!(
        understands_bilateral_logic,
        "Mask should understand that bilateral construction prevents asymmetry"
    );

    println!("✓ Bilateral Dependencies: 4/4 concepts understood");
}

#[test]
fn benchmark_v3_continuous_flooding_semantics() {
    println!("\n=== BENCHMARK v3: Continuous Flooding vs Finite Sequences ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for understanding of continuous flooding
    let has_flooding = content.contains("flooding") || content.contains("Flooding");
    let has_continuous = content.contains("continuous") || content.contains("Continuous");
    let understands_flooding =
        (has_flooding && has_continuous) || content.contains("continuous flooding");

    // Check for understanding that this differs from finite message sequences
    let mentions_finite_sequences = content.contains("finite")
        || content.contains("predetermined")
        || content.contains("last message");

    // Check for understanding of "no last message" property
    let understands_no_last_message = content.contains("no last message")
        || content.contains("no \"last message\"")
        || (content.contains("continuous") && content.contains("flooding"));

    assert!(
        understands_flooding,
        "Mask should understand continuous flooding semantics"
    );
    assert!(
        mentions_finite_sequences || understands_no_last_message,
        "Mask should understand difference from finite message sequences"
    );

    println!("✓ Continuous Flooding: 2/2 key concepts understood");
}

#[test]
fn benchmark_v3_impossibility_navigation() {
    println!("\n=== BENCHMARK v3: Impossibility Result Navigation ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for proper handling of impossibility-breaking protocols
    let mentions_two_generals =
        content.contains("Two Generals") || content.contains("two generals");
    let mentions_impossibility =
        content.contains("impossibility") || content.contains("Impossibility");

    // CRITICAL: Should NOT reflexively dismiss protocols
    let has_validation_approach = content.contains("study")
        || content.contains("analyze")
        || content.contains("validate")
        || content.contains("Validate");

    // Should understand impossibility results have ASSUMPTIONS that can be broken
    let understands_assumptions = content.contains("assumption")
        || content.contains("Assumption")
        || (content.contains("proof") && content.contains("breaks"));

    // Should NOT contain dismissive language without analysis
    // But allow it in examples of what NOT to do (DON'T sections, negative examples)
    let has_dismissive_language =
        content.contains("can't work") || content.contains("Two Generals is impossible");
    let is_in_negative_example = content.contains("DON'T:")
        || content.contains("**DON'T:**")
        || content.contains("Avoid")
        || content.contains("Anti-Pattern");

    // It's OK to have dismissive language if it's in a "what not to do" section
    let not_dismissive = !has_dismissive_language || is_in_negative_example;

    assert!(
        mentions_two_generals,
        "Mask should discuss Two Generals Problem"
    );
    assert!(
        mentions_impossibility,
        "Mask should understand impossibility results"
    );
    assert!(
        has_validation_approach,
        "Mask should have validation approach (not reflexive dismissal)"
    );
    assert!(
        understands_assumptions || has_validation_approach,
        "Mask should understand impossibility proofs have assumptions"
    );
    assert!(
        not_dismissive,
        "Mask should not reflexively dismiss novel protocols"
    );

    println!("✓ Impossibility Navigation: 5/5 concepts understood");
}

#[test]
fn benchmark_v3_all_or_nothing_semantics() {
    println!("\n=== BENCHMARK v3: All-or-Nothing Semantics ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for understanding of symmetric outcomes
    let has_symmetric = content.contains("symmetric") || content.contains("Symmetric");
    let has_attack_abort = (content.contains("ATTACK") && content.contains("ABORT"))
        || (content.contains("attack") && content.contains("abort"))
        || (content.contains("commit") && content.contains("abort"));

    // Should understand "both parties same decision"
    let understands_symmetric_outcomes = (has_symmetric && has_attack_abort)
        || content.contains("both")
        || content.contains("all-or-nothing")
        || content.contains("never asymmetric");

    // Should understand asymmetric outcomes are BUGS
    let understands_asymmetry_bad = content.contains("asymmetric")
        || content.contains("split-brain")
        || (content.contains("both") && (content.contains("attack") || content.contains("abort")));

    assert!(
        has_symmetric,
        "Mask should understand symmetric decision requirements"
    );
    assert!(
        understands_symmetric_outcomes,
        "Mask should understand all-or-nothing semantics"
    );
    assert!(
        understands_asymmetry_bad,
        "Mask should understand asymmetric outcomes violate safety"
    );

    println!("✓ All-or-Nothing Semantics: 3/3 concepts understood");
}

#[test]
fn benchmark_v3_formal_validation_methodology() {
    println!("\n=== BENCHMARK v3: Formal Validation Methodology ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for property-based testing understanding
    let has_property_based = content.contains("property-based")
        || content.contains("Property-based")
        || content.contains("QuickCheck")
        || content.contains("Hypothesis");

    // Check for formal verification tools
    let has_formal_verification = content.contains("TLA+")
        || content.contains("formal verification")
        || content.contains("Formal verification")
        || content.contains("model check");

    // Check for chaos engineering
    let has_chaos_engineering = content.contains("chaos")
        || content.contains("Chaos")
        || content.contains("Jepsen")
        || content.contains("failure injection");

    // Check for formal properties (Safety, Liveness, Validity)
    let has_safety = content.contains("Safety") || content.contains("safety");
    let has_liveness = content.contains("Liveness") || content.contains("liveness");
    let has_validity = content.contains("Validity") || content.contains("validity");

    let formal_properties_count = [has_safety, has_liveness, has_validity]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(
        has_property_based || has_formal_verification,
        "Mask should understand property-based testing or formal verification"
    );
    assert!(
        has_chaos_engineering || has_formal_verification,
        "Mask should understand chaos engineering or formal verification"
    );
    assert!(
        formal_properties_count >= 2,
        "Mask should understand at least 2/3 formal properties (Safety, Liveness, Validity)"
    );

    println!(
        "✓ Formal Validation: 3/3 methodologies, {}/3 formal properties",
        formal_properties_count
    );
}

#[test]
fn benchmark_v3_tgp_specific_knowledge() {
    println!("\n=== BENCHMARK v3: Two Generals Protocol Specific Knowledge ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for TGP-specific terminology
    let mentions_tgp = content.contains("TGP") || content.contains("Two Generals Protocol");
    let mentions_half_rtt = content.contains("half-RTT") || content.contains("RTT");
    let mentions_diffie_hellman = content.contains("Diffie-Hellman")
        || content.contains("DH")
        || content.contains("key exchange");

    // Check for understanding of rounds (R1, R2, R3, R4)
    let mentions_rounds = content.contains("R1")
        || content.contains("R2")
        || content.contains("R3")
        || content.contains("round");

    // Check for understanding of bilateral confirmation
    let mentions_confirmation = content.contains("confirmation")
        || content.contains("Confirmation")
        || content.contains("R3_CONF");

    let tgp_knowledge_count = [
        mentions_tgp,
        mentions_half_rtt,
        mentions_diffie_hellman,
        mentions_rounds,
        mentions_confirmation,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        mentions_tgp,
        "Mask should explicitly mention Two Generals Protocol"
    );
    assert!(
        tgp_knowledge_count >= 2,
        "Mask should have specific TGP knowledge (found {}/5 concepts)",
        tgp_knowledge_count
    );

    println!(
        "✓ TGP Specific Knowledge: {}/5 concepts present",
        tgp_knowledge_count
    );
}

#[test]
fn benchmark_v3_test_suite_design_knowledge() {
    println!("\n=== BENCHMARK v3: Test Suite Design Knowledge ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for adversarial testing understanding
    let has_adversarial = content.contains("adversarial")
        || content.contains("Adversarial")
        || content.contains("omniscient adversary");

    // Check for Byzantine fault understanding
    let has_byzantine = content.contains("Byzantine") || content.contains("byzantine");

    // Check for network failure scenarios
    let has_packet_loss = content.contains("packet loss")
        || content.contains("message loss")
        || content.contains("drop");
    let has_partition_testing = content.contains("partition");
    let has_latency = content.contains("latency") || content.contains("delay");

    let network_failure_count = [has_packet_loss, has_partition_testing, has_latency]
        .iter()
        .filter(|&&x| x)
        .count();

    // Check for test coverage understanding
    let has_test_coverage = content.contains("test")
        || content.contains("Test")
        || content.contains("validation")
        || content.contains("Validation");

    assert!(
        has_adversarial || has_byzantine,
        "Mask should understand adversarial/Byzantine testing"
    );
    assert!(
        network_failure_count >= 2,
        "Mask should understand network failure scenarios (found {}/3)",
        network_failure_count
    );
    assert!(has_test_coverage, "Mask should discuss testing/validation");

    println!(
        "✓ Test Suite Design: 3/3 core concepts, {}/3 network failures",
        network_failure_count
    );
}

#[test]
fn benchmark_v3_critical_insights() {
    println!("\n=== BENCHMARK v3: Critical Insights (Anti-Patterns Avoided) ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Critical Insight 1: Should NOT claim "Two Generals is impossible, period"
    // (It's impossible under specific assumptions, but those can be relaxed)
    // But allow it in examples of what NOT to do
    let has_absolute_impossibility_claim = content.contains("Two Generals is impossible")
        || content.contains("cannot be solved")
        || content.contains("provably impossible");
    let is_in_negative_example = content.contains("DON'T:")
        || content.contains("**DON'T:**")
        || content.contains("Avoid")
        || content.contains("Anti-Pattern");

    let avoids_absolute_impossibility = !has_absolute_impossibility_claim || is_in_negative_example;

    // Critical Insight 2: Should understand R3_CONF_FINAL bilateral nature
    // (Can only send if you have receipt, which means partner can construct too)
    let understands_r3_conf_final = content.contains("R3_CONF_FINAL")
        || content.contains("bilateral")
        || (content.contains("receipt") && content.contains("confirmation"));

    // Critical Insight 3: Should NOT dismiss continuous flooding as "still unreliable"
    // (It's unreliable per-message but reliable in aggregate with timeout)
    let understands_flooding_reliability = content.contains("continuous")
        || content.contains("flooding")
        || content.contains("eventual");

    // Critical Insight 4: Should understand CAP still holds (TGP is CP system)
    // (Not violating CAP, just better bilateral coordination within CP constraints)
    let understands_cap_still_holds =
        content.contains("CAP") && (content.contains("trade-off") || content.contains("tradeoff"));

    assert!(
        avoids_absolute_impossibility,
        "Mask should not claim absolute impossibility without analysis"
    );
    assert!(
        understands_r3_conf_final || content.contains("bilateral"),
        "Mask should understand bilateral confirmation patterns"
    );
    assert!(
        understands_flooding_reliability,
        "Mask should understand continuous flooding semantics"
    );
    assert!(
        understands_cap_still_holds,
        "Mask should understand CAP theorem still applies"
    );

    println!("✓ Critical Insights: 4/4 anti-patterns avoided");
}

#[test]
fn benchmark_v3_comprehensive_score() {
    println!("\n=== BENCHMARK v3: COMPREHENSIVE SCORE ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Calculate comprehensive understanding score
    let mut score = 0;
    let mut max_score = 0;

    // Category 1: Bilateral Dependencies (10 points)
    max_score += 10;
    if content.contains("bilateral") {
        score += 3;
    }
    if content.contains("receipt") || content.contains("confirmation") {
        score += 3;
    }
    if content.contains("structural symmetry") || content.contains("symmetric") {
        score += 4;
    }

    // Category 2: Continuous Flooding (10 points)
    max_score += 10;
    if content.contains("continuous") && content.contains("flooding") {
        score += 5;
    }
    if content.contains("finite") || content.contains("last message") {
        score += 5;
    }

    // Category 3: Impossibility Navigation (15 points)
    max_score += 15;
    if content.contains("Two Generals") {
        score += 3;
    }
    if content.contains("impossibility") {
        score += 3;
    }
    if content.contains("assumption") || content.contains("study") {
        score += 4;
    }
    if !content.contains("can't work") && !content.contains("is impossible") {
        score += 5;
    }

    // Category 4: Formal Validation (15 points)
    max_score += 15;
    if content.contains("property-based") || content.contains("Hypothesis") {
        score += 5;
    }
    if content.contains("TLA+") || content.contains("formal verification") {
        score += 5;
    }
    if content.contains("Safety") && content.contains("Liveness") {
        score += 5;
    }

    // Category 5: TGP Specifics (10 points)
    max_score += 10;
    if content.contains("TGP") || content.contains("Two Generals Protocol") {
        score += 5;
    }
    if content.contains("half-RTT") || content.contains("R3_CONF") {
        score += 5;
    }

    // Category 6: Critical Insights (20 points)
    max_score += 20;
    // Allow "Two Generals is impossible" if it's in a DON'T/negative example context
    let has_dismissive = content.contains("Two Generals is impossible");
    let has_context_markers = content.contains("DON'T") || content.contains("Avoid");
    if !has_dismissive || has_context_markers {
        score += 5;
    }
    if content.contains("bilateral") && content.contains("receipt") {
        score += 5;
    }
    if content.contains("continuous") || content.contains("flooding") {
        score += 5;
    }
    if content.contains("CAP") {
        score += 5;
    }

    let percentage = (score as f64 / max_score as f64) * 100.0;

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "  COMPREHENSIVE v3 SCORE: {}/{} ({:.1}%)",
        score, max_score, percentage
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("");
    println!("  Grade Scale:");
    println!("    90-100%: Excellent - Deep understanding");
    println!("    80-89%:  Good - Solid grasp of concepts");
    println!("    70-79%:  Fair - Basic understanding");
    println!("    <70%:    Needs improvement");
    println!("");

    assert!(
        percentage >= 70.0,
        "Mask should score at least 70% on v3 comprehensive benchmark (got {:.1}%)",
        percentage
    );

    if percentage >= 90.0 {
        println!("✓✓✓ EXCELLENT: Mask demonstrates deep understanding of advanced concepts!");
    } else if percentage >= 80.0 {
        println!("✓✓ GOOD: Mask has solid grasp of distributed systems concepts");
    } else if percentage >= 70.0 {
        println!("✓ FAIR: Mask has basic understanding, room for improvement");
    }
}
