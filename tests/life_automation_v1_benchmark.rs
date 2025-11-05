use palace_skills::mask::loader::load_mask_from_file;

#[test]
fn benchmark_life_automation_structure() {
    println!("\n=== BENCHMARK: Life Automation Structure ===\n");

    let mask = load_mask_from_file("life-automation", "sonnet")
        .expect("Failed to load life-automation mask");

    let content = &mask.content;

    // Core sections
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");
    let has_improvement_notes = content.contains("## Improvement Notes");

    println!("Structure Checks:");
    println!(
        "  Identity section: {}",
        if has_identity { "✅" } else { "❌" }
    );
    println!(
        "  Core Expertise section: {}",
        if has_expertise { "✅" } else { "❌" }
    );
    println!(
        "  Your Mission section: {}",
        if has_mission { "✅" } else { "❌" }
    );
    println!(
        "  Behavioral Guidelines: {}",
        if has_guidelines { "✅" } else { "❌" }
    );
    println!(
        "  Examples section: {}",
        if has_examples { "✅" } else { "❌" }
    );
    println!(
        "  Improvement Notes: {}",
        if has_improvement_notes { "✅" } else { "❌" }
    );

    let structure_score = [
        has_identity,
        has_expertise,
        has_mission,
        has_guidelines,
        has_examples,
        has_improvement_notes,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nStructure Score: {}/6", structure_score);

    assert!(has_identity, "Must have Identity section");
    assert!(has_expertise, "Must have Core Expertise section");
    assert!(has_mission, "Must have Your Mission section");
    assert!(has_guidelines, "Must have Behavioral Guidelines");
    assert!(has_examples, "Must have Examples section");
    assert!(has_improvement_notes, "Must have Improvement Notes");

    println!("\n✓ STRUCTURE BENCHMARK PASSED\n");
}

#[test]
fn benchmark_life_automation_empathy() {
    println!("\n=== BENCHMARK: Life Automation Empathy ===\n");

    let mask = load_mask_from_file("life-automation", "sonnet")
        .expect("Failed to load life-automation mask");

    let content = &mask.content;

    // Empathy capabilities
    let has_neurodivergent_support = content.contains("neurodivergent")
        || content.contains("SDAM")
        || content.contains("aphantasia");
    let has_context_loading = content.contains("load") && content.contains("context")
        || content.contains("personal context");
    let has_empathy_boundaries = content.contains("empathy") && content.contains("boundaries")
        || content.contains("not a sycophant");
    let has_cognitive_assessment = content.contains("cognitive state")
        || content.contains("assess")
        || content.contains("Recognize What's Needed")
        || content.contains("Notice patterns");
    let validates_not_therapist =
        content.contains("not a therapist") || content.contains("therapy-bot");

    println!("Empathy Checks:");
    println!(
        "  Neurodivergent support: {}",
        if has_neurodivergent_support {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Context loading mechanism: {}",
        if has_context_loading { "✅" } else { "❌" }
    );
    println!(
        "  Empathy with boundaries: {}",
        if has_empathy_boundaries { "✅" } else { "❌" }
    );
    println!(
        "  Cognitive state assessment: {}",
        if has_cognitive_assessment {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Not-a-therapist boundary: {}",
        if validates_not_therapist {
            "✅"
        } else {
            "❌"
        }
    );

    let empathy_score = [
        has_neurodivergent_support,
        has_context_loading,
        has_empathy_boundaries,
        has_cognitive_assessment,
        validates_not_therapist,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nEmpathy Score: {}/5", empathy_score);

    assert!(
        has_neurodivergent_support,
        "Must support neurodivergent cognitive profiles"
    );
    assert!(
        has_context_loading,
        "Must have mechanism to load personal context"
    );
    assert!(
        has_empathy_boundaries,
        "Must have empathy with boundaries (not unconditional validation)"
    );
    assert!(
        has_cognitive_assessment,
        "Must assess cognitive state appropriately"
    );

    println!("\n✓ EMPATHY BENCHMARK PASSED\n");
}

#[test]
fn benchmark_life_automation_reality_checks() {
    println!("\n=== BENCHMARK: Life Automation Reality Checks ===\n");

    let mask = load_mask_from_file("life-automation", "sonnet")
        .expect("Failed to load life-automation mask");

    let content = &mask.content;

    // Reality check capabilities
    let has_reality_check_framework =
        content.contains("Reality Check") || content.contains("reality check");
    let challenges_self_limiting =
        content.contains("self-limiting") || content.contains("challenge");
    let checks_avoidance = content.contains("avoidance") || content.contains("avoiding");
    let validates_grandiose = content.contains("grandiose") || content.contains("reality-check");
    let prevents_overcommit = content.contains("overcommit") || content.contains("cognitive load");

    println!("Reality Check Capabilities:");
    println!(
        "  Reality check framework: {}",
        if has_reality_check_framework {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Challenges self-limiting beliefs: {}",
        if challenges_self_limiting {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Identifies avoidance behaviors: {}",
        if checks_avoidance { "✅" } else { "❌" }
    );
    println!(
        "  Reality-checks grandiose plans: {}",
        if validates_grandiose { "✅" } else { "❌" }
    );
    println!(
        "  Prevents overcommitment: {}",
        if prevents_overcommit { "✅" } else { "❌" }
    );

    let reality_score = [
        has_reality_check_framework,
        challenges_self_limiting,
        checks_avoidance,
        validates_grandiose,
        prevents_overcommit,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nReality Check Score: {}/5", reality_score);

    assert!(
        has_reality_check_framework,
        "Must have explicit reality check framework"
    );
    assert!(
        challenges_self_limiting,
        "Must challenge self-limiting beliefs"
    );
    assert!(checks_avoidance, "Must identify avoidance behaviors");
    assert!(prevents_overcommit, "Must prevent overcommitment");

    println!("\n✓ REALITY CHECK BENCHMARK PASSED\n");
}

#[test]
fn benchmark_life_automation_usefulness() {
    println!("\n=== BENCHMARK: Life Automation Usefulness ===\n");

    let mask = load_mask_from_file("life-automation", "sonnet")
        .expect("Failed to load life-automation mask");

    let content = &mask.content;

    // Usefulness capabilities
    let has_structured_workflow = content.contains("workflow")
        || (content.contains("1.") && content.contains("2.") && content.contains("3."));
    let has_automation_focus = content.contains("automate")
        || content.contains("delegate")
        || content.contains("eliminate");
    let has_context_preservation = content.contains("preserve") && content.contains("context")
        || content.contains("documentation");
    let has_victory_documentation = content.contains("victory") || content.contains("achievement");
    let has_concrete_examples = content.contains("BAD:") && content.contains("GOOD:");
    let has_strategic_questions =
        content.contains("What's the system") || content.contains("strategic question");

    println!("Usefulness Checks:");
    println!(
        "  Structured workflow: {}",
        if has_structured_workflow {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Automation focus: {}",
        if has_automation_focus { "✅" } else { "❌" }
    );
    println!(
        "  Context preservation: {}",
        if has_context_preservation {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Victory documentation: {}",
        if has_victory_documentation {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Concrete examples: {}",
        if has_concrete_examples { "✅" } else { "❌" }
    );
    println!(
        "  Strategic questions: {}",
        if has_strategic_questions {
            "✅"
        } else {
            "❌"
        }
    );

    let usefulness_score = [
        has_structured_workflow,
        has_automation_focus,
        has_context_preservation,
        has_victory_documentation,
        has_concrete_examples,
        has_strategic_questions,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nUsefulness Score: {}/6", usefulness_score);

    assert!(has_structured_workflow, "Must have structured workflow");
    assert!(
        has_automation_focus,
        "Must focus on automation/delegation/elimination"
    );
    assert!(has_context_preservation, "Must preserve context for future");
    assert!(
        has_concrete_examples,
        "Must have concrete BAD vs GOOD examples"
    );

    println!("\n✓ USEFULNESS BENCHMARK PASSED\n");
}

#[test]
fn benchmark_life_automation_generic() {
    println!("\n=== BENCHMARK: Life Automation Generic (Not User-Specific) ===\n");

    let mask = load_mask_from_file("life-automation", "sonnet")
        .expect("Failed to load life-automation mask");

    let content = &mask.content;

    // Check for hardcoded personal details that should be in context file
    let has_hardcoded_partner = content.contains("Phoebe") || content.contains("Caeli");
    let has_hardcoded_company = content.contains("Hauska") || content.contains("Riff Labs");
    let has_hardcoded_role = content.contains("CIO at") || content.contains("leading 15 employees");
    let has_hardcoded_dates =
        content.contains("Sept 26") || content.contains("Sept 28") || content.contains("Dec 2021");
    let has_hardcoded_transformation =
        content.contains("bullied kid") || content.contains("infrastructure wizard");

    // Should NOT have these hardcoded
    let is_generic = !has_hardcoded_partner
        && !has_hardcoded_company
        && !has_hardcoded_role
        && !has_hardcoded_dates
        && !has_hardcoded_transformation;

    // Should have context loading
    let has_context_loading = content.contains("load") && content.contains("context");

    println!("Generic Check:");
    println!(
        "  No hardcoded partner names: {}",
        if !has_hardcoded_partner { "✅" } else { "❌" }
    );
    println!(
        "  No hardcoded company/role: {}",
        if !has_hardcoded_company { "✅" } else { "❌" }
    );
    println!(
        "  No hardcoded specific role: {}",
        if !has_hardcoded_role { "✅" } else { "❌" }
    );
    println!(
        "  No hardcoded dates/events: {}",
        if !has_hardcoded_dates { "✅" } else { "❌" }
    );
    println!(
        "  No hardcoded transformation arc: {}",
        if !has_hardcoded_transformation {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "  Has context loading mechanism: {}",
        if has_context_loading { "✅" } else { "❌" }
    );

    println!(
        "\nGeneric Score: {}",
        if is_generic && has_context_loading {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    assert!(
        is_generic,
        "Mask should be generic, not hardcoded with personal details"
    );
    assert!(
        has_context_loading,
        "Must have mechanism to load personal context from user's folder"
    );

    println!("\n✓ GENERIC BENCHMARK PASSED\n");
}
