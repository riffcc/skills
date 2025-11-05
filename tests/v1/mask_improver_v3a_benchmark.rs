use palace_skills::*;

#[test]
fn benchmark_mask_improver_v3a_features() {
    println!("\n=== BENCHMARK: Mask Improver v3A Features ===\n");

    let mask =
        load_mask_from_file("mask-improver", "sonnet").expect("Failed to load Mask Improver");

    println!("Loaded: Mask Improver v{}", mask.version);
    println!("Specialty: {}", mask.specialty);
    println!();

    let content = &mask.content;

    // v2 features (should still be present)
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_examples = content.contains("## Examples of Good Improvements");
    let has_validation = content.contains("## Validation Strategy");
    let has_prioritization = content.contains("**Prioritize Improvements**");

    // v3A NEW features
    let has_creation_workflow = content.contains("### When Creating New Masks from Scratch");
    let has_7_steps = content.contains("#### Step 1: Domain Research")
        && content.contains("#### Step 2: Identity Formation")
        && content.contains("#### Step 3: Expertise Mapping")
        && content.contains("#### Step 4: Mission Definition")
        && content.contains("#### Step 5: Behavioral Guidelines")
        && content.contains("#### Step 6: Concrete Examples")
        && content.contains("#### Step 7: Bootstrap First Version");

    let has_pattern_library = content.contains("## Pattern Library");
    let has_patterns = content.contains("### Pattern: Concrete Examples")
        && content.contains("### Pattern: Validation Strategy")
        && content.contains("### Pattern: Prioritization Framework");

    let has_benchmark_analysis = content.contains("**Failure Pattern Analysis:**")
        && content.contains("**Proposed Fix Template:**");

    let has_v3a_identity = content.contains("v3A Enhancement")
        || content.contains("Mask Creation:")
        || content.contains("Pattern Recognition:");

    let has_v3a_notes = content.contains("### Version 3A");

    println!("=== v2 Features (Should Still Be Present) ===");
    println!("  ✓ Has Identity section: {}", has_identity);
    println!("  ✓ Has Core Expertise: {}", has_expertise);
    println!("  ✓ Has Mission: {}", has_mission);
    println!("  ✓ Has Examples (v2+): {}", has_examples);
    println!("  ✓ Has Validation Strategy (v2+): {}", has_validation);
    println!("  ✓ Has Prioritization (v2+): {}", has_prioritization);
    println!();

    println!("=== v3A NEW Features ===");
    println!(
        "  ✓ Has 'When Creating New Masks' section: {}",
        has_creation_workflow
    );
    println!("  ✓ Has complete 7-step workflow: {}", has_7_steps);
    println!("  ✓ Has Pattern Library section: {}", has_pattern_library);
    println!("  ✓ Has documented patterns: {}", has_patterns);
    println!(
        "  ✓ Has Benchmark Analysis framework: {}",
        has_benchmark_analysis
    );
    println!("  ✓ Has v3A identity markers: {}", has_v3a_identity);
    println!("  ✓ Has v3A improvement notes: {}", has_v3a_notes);
    println!();

    // Assertions
    assert!(
        has_identity && has_expertise && has_mission,
        "v2 structure missing"
    );
    assert!(
        has_examples && has_validation && has_prioritization,
        "v2 features missing"
    );
    assert!(has_creation_workflow, "v3A: Creation workflow missing!");
    assert!(has_7_steps, "v3A: 7-step workflow incomplete!");
    assert!(has_pattern_library, "v3A: Pattern Library missing!");
    assert!(has_patterns, "v3A: Documented patterns missing!");
    assert!(
        has_benchmark_analysis,
        "v3A: Benchmark analysis framework missing!"
    );

    let v2_score = [
        has_identity,
        has_expertise,
        has_mission,
        has_examples,
        has_validation,
        has_prioritization,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    let v3a_score = [
        has_creation_workflow,
        has_7_steps,
        has_pattern_library,
        has_patterns,
        has_benchmark_analysis,
        has_v3a_identity,
        has_v3a_notes,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("Benchmark Results:");
    println!("  v2 Features: {}/6", v2_score);
    println!("  v3A Features: {}/7", v3a_score);
    println!();

    if v2_score == 6 && v3a_score == 7 {
        println!("✓ BENCHMARK PASSED - Mask Improver v3A complete!");
    } else {
        println!("✗ BENCHMARK FAILED - Missing features");
        panic!(
            "v3A features incomplete: v2={}/6, v3A={}/7",
            v2_score, v3a_score
        );
    }
}

#[test]
fn benchmark_distributed_systems_structure() {
    println!("\n=== BENCHMARK: Distributed Systems Mask Structure ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load distributed-systems mask");

    println!("Loaded: {} v{}", mask.specialty, mask.version);
    println!();

    let content = &mask.content;

    // Required sections
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");
    let has_notes = content.contains("## Improvement Notes");

    // Domain-specific content
    let has_consensus = content.contains("Consensus Algorithms")
        || content.contains("Paxos")
        || content.contains("Raft");
    let has_cap = content.contains("CAP Theorem");
    let has_ha_patterns = content.contains("High Availability") || content.contains("Replication");

    // Quality markers
    let has_real_systems =
        content.contains("PostgreSQL") || content.contains("etcd") || content.contains("HAProxy");
    let has_tradeoffs = content.contains("Trade-offs:") || content.contains("Trade-off");
    let has_configs = content.contains("postgresql.conf")
        || content.contains("haproxy.cfg")
        || content.contains("Configuration:");

    println!("=== Structure Validation ===");
    println!("  ✓ Has Identity section: {}", has_identity);
    println!("  ✓ Has Core Expertise: {}", has_expertise);
    println!("  ✓ Has Mission: {}", has_mission);
    println!("  ✓ Has Behavioral Guidelines: {}", has_guidelines);
    println!("  ✓ Has Examples: {}", has_examples);
    println!("  ✓ Has Improvement Notes: {}", has_notes);
    println!();

    println!("=== Domain Content ===");
    println!("  ✓ Has Consensus Algorithms: {}", has_consensus);
    println!("  ✓ Has CAP Theorem: {}", has_cap);
    println!("  ✓ Has HA Patterns: {}", has_ha_patterns);
    println!();

    println!("=== Quality Markers ===");
    println!("  ✓ Uses Real Systems: {}", has_real_systems);
    println!("  ✓ Includes Trade-offs: {}", has_tradeoffs);
    println!("  ✓ Has Configurations: {}", has_configs);
    println!();

    // Assertions
    assert!(
        has_identity && has_expertise && has_mission,
        "Basic structure missing"
    );
    assert!(
        has_guidelines && has_examples && has_notes,
        "Required sections missing"
    );
    assert!(
        has_consensus && has_cap && has_ha_patterns,
        "Core domain expertise missing"
    );
    assert!(
        has_real_systems,
        "Should reference real systems, not abstract examples"
    );
    assert!(has_tradeoffs, "Should include trade-off analysis");

    let structure_score = [
        has_identity,
        has_expertise,
        has_mission,
        has_guidelines,
        has_examples,
        has_notes,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    let domain_score = [has_consensus, has_cap, has_ha_patterns]
        .iter()
        .filter(|&&x| x)
        .count();

    let quality_score = [has_real_systems, has_tradeoffs, has_configs]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("Benchmark Results:");
    println!("  Structure: {}/6", structure_score);
    println!("  Domain Content: {}/3", domain_score);
    println!("  Quality Markers: {}/3", quality_score);
    println!();

    if structure_score == 6 && domain_score == 3 && quality_score >= 2 {
        println!("✓ BENCHMARK PASSED - distributed-systems mask well-formed!");
    } else {
        println!("✗ BENCHMARK FAILED");
        panic!(
            "Mask quality insufficient: structure={}/6, domain={}/3, quality={}/3",
            structure_score, domain_score, quality_score
        );
    }
}
