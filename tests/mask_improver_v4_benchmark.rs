// Benchmark tests for mask-improver v4
// Validates that v4 enhancements (Pattern Library expansion) are present

use palace_skills::*;

#[test]
fn test_mask_improver_v4_pattern_library_count() {
    println!("\n=== V4 BENCHMARK: Mask Improver - Pattern Library Count ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    // Count pattern entries in the Pattern Library
    let pattern_count = content.matches("### Pattern:").count();

    println!("Pattern Library:");
    println!("  ✓ Total patterns: {}", pattern_count);
    println!("  Expected: 11 (5 from v3A + 6 from v4)");

    assert!(
        pattern_count >= 11,
        "Should have at least 11 patterns (got {})",
        pattern_count
    );
}

#[test]
fn test_mask_improver_v4_mcp_integration_pattern() {
    println!("\n=== V4 BENCHMARK: Mask Improver - MCP Integration Pattern ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    let has_mcp_pattern = content.contains("### Pattern: MCP Tool Integration")
        || content.contains("MCP Tool Integration (v4)");
    let has_mcp_template = content.contains("## MCP Integration Notes")
        || content.contains("MCP Integration");
    let has_tool_specialist = content.contains("Tool Specialist")
        || content.contains("tool specialist");
    let mentions_playwright = content.contains("Playwright")
        || content.contains("playwright");

    println!("MCP Integration Pattern:");
    println!(
        "  ✓ MCP pattern present: {}",
        if has_mcp_pattern { "✅" } else { "❌" }
    );
    println!(
        "  ✓ MCP template provided: {}",
        if has_mcp_template { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Tool specialist mentioned: {}",
        if has_tool_specialist { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Example (Playwright): {}",
        if mentions_playwright { "✅" } else { "❌" }
    );

    let score = [
        has_mcp_pattern,
        has_mcp_template,
        has_tool_specialist,
        mentions_playwright,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/4", score);

    assert_eq!(score, 4, "Should have complete MCP integration pattern");
}

#[test]
fn test_mask_improver_v4_cicd_test_generation_pattern() {
    println!("\n=== V4 BENCHMARK: Mask Improver - CI/CD Test Generation Pattern ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    let has_cicd_pattern = content.contains("### Pattern: CI/CD Test Generation")
        || content.contains("CI/CD Test Generation (v4)");
    let has_cargo_test_template = content.contains("cargo test")
        || content.contains("#[test]");
    let has_regression_mention = content.contains("regression")
        || content.contains("Regression");
    let has_github_actions = content.contains("GitHub Actions")
        || content.contains("CI pipeline");
    let has_test_strategy = content.contains("Critical Issues → Blocking Tests")
        || content.contains("Test Generation Strategy");

    println!("CI/CD Test Generation Pattern:");
    println!(
        "  ✓ CI/CD pattern present: {}",
        if has_cicd_pattern { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Cargo test template: {}",
        if has_cargo_test_template { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Regression testing concept: {}",
        if has_regression_mention { "✅" } else { "❌" }
    );
    println!(
        "  ✓ GitHub Actions/CI pipeline: {}",
        if has_github_actions { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Test strategy documented: {}",
        if has_test_strategy { "✅" } else { "❌" }
    );

    let score = [
        has_cicd_pattern,
        has_cargo_test_template,
        has_regression_mention,
        has_github_actions,
        has_test_strategy,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(score, 5, "Should have complete CI/CD test generation pattern");
}

#[test]
fn test_mask_improver_v4_workflow_examples_pattern() {
    println!("\n=== V4 BENCHMARK: Mask Improver - End-to-End Workflow Examples Pattern ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    let has_workflow_pattern = content.contains("### Pattern: End-to-End Workflow Examples")
        || content.contains("Workflow Examples (v4)");
    let has_step_structure =
        content.contains("Step 1:") && content.contains("Step 2:");
    let has_value_delivered = content.contains("Value Delivered")
        || content.contains("value delivered");
    let has_workflow_template = content.contains("Complete Workflow")
        || content.contains("complete workflow");

    println!("Workflow Examples Pattern:");
    println!(
        "  ✓ Workflow pattern present: {}",
        if has_workflow_pattern { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Step-by-step structure: {}",
        if has_step_structure { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Value delivered section: {}",
        if has_value_delivered { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Workflow template provided: {}",
        if has_workflow_template { "✅" } else { "❌" }
    );

    let score = [
        has_workflow_pattern,
        has_step_structure,
        has_value_delivered,
        has_workflow_template,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/4", score);

    assert_eq!(score, 4, "Should have complete workflow examples pattern");
}

#[test]
fn test_mask_improver_v4_scoring_frameworks_pattern() {
    println!("\n=== V4 BENCHMARK: Mask Improver - Objective Scoring Frameworks Pattern ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    let has_scoring_pattern = content.contains("### Pattern: Objective Scoring Frameworks")
        || content.contains("Scoring Frameworks (v4)");
    let has_dimensions = content.contains("dimensions") || content.contains("Dimension");
    let has_weighted_average = content.contains("weighted") || content.contains("Weighted");
    let has_0_100_scale = content.contains("0-100") || content.contains("/100");
    let has_severity_levels = (content.contains("Critical") || content.contains("critical"))
        && (content.contains("Major") || content.contains("major"))
        && (content.contains("Minor") || content.contains("minor"));

    println!("Scoring Frameworks Pattern:");
    println!(
        "  ✓ Scoring pattern present: {}",
        if has_scoring_pattern { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Dimensions concept: {}",
        if has_dimensions { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Weighted average: {}",
        if has_weighted_average { "✅" } else { "❌" }
    );
    println!(
        "  ✓ 0-100 scale: {}",
        if has_0_100_scale { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Severity levels (Critical/Major/Minor): {}",
        if has_severity_levels { "✅" } else { "❌" }
    );

    let score = [
        has_scoring_pattern,
        has_dimensions,
        has_weighted_average,
        has_0_100_scale,
        has_severity_levels,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(
        score, 5,
        "Should have complete objective scoring frameworks pattern"
    );
}

#[test]
fn test_mask_improver_v4_tool_vs_domain_pattern() {
    println!("\n=== V4 BENCHMARK: Mask Improver - Tool vs Domain Specialist Pattern ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    let has_tool_vs_domain_pattern = content.contains("### Pattern: Tool Specialist vs Domain Specialist")
        || content.contains("Tool vs Domain Specialist (v4)");
    let has_domain_specialist_desc = content.contains("Domain Specialist:")
        || content.contains("domain specialist");
    let has_tool_specialist_desc = content.contains("Tool Specialist:")
        || content.contains("tool specialist");
    let has_decision_guide = content.contains("Decision Guide")
        || content.contains("decision guide")
        || content.contains("MCP server");
    let has_template_differences = content.contains("Template Emphasis")
        || content.contains("template emphasis");

    println!("Tool vs Domain Specialist Pattern:");
    println!(
        "  ✓ Pattern present: {}",
        if has_tool_vs_domain_pattern { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Domain specialist described: {}",
        if has_domain_specialist_desc { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Tool specialist described: {}",
        if has_tool_specialist_desc { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Decision guide provided: {}",
        if has_decision_guide { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Template differences shown: {}",
        if has_template_differences { "✅" } else { "❌" }
    );

    let score = [
        has_tool_vs_domain_pattern,
        has_domain_specialist_desc,
        has_tool_specialist_desc,
        has_decision_guide,
        has_template_differences,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(
        score, 5,
        "Should have complete tool vs domain specialist pattern"
    );
}

#[test]
fn test_mask_improver_v4_benchmark_creation_pattern() {
    println!("\n=== V4 BENCHMARK: Mask Improver - Benchmark Creation Guide Pattern ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    let has_benchmark_pattern = content.contains("### Pattern: Benchmark Creation Guide")
        || content.contains("Benchmark Creation (v4)");
    let has_structure_validation = content.contains("Structure Validation")
        || content.contains("structure validation");
    let has_domain_coverage = content.contains("Domain Coverage")
        || content.contains("domain coverage");
    let has_test_templates = content.contains("load_mask_from_file")
        || content.contains("#[test]");
    let has_4_step_workflow = content.contains("Step 1:")
        && content.contains("Step 2:")
        && content.contains("Step 3:")
        && content.contains("Step 4:");

    println!("Benchmark Creation Pattern:");
    println!(
        "  ✓ Benchmark pattern present: {}",
        if has_benchmark_pattern { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Structure validation step: {}",
        if has_structure_validation { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Domain coverage step: {}",
        if has_domain_coverage { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Test code templates: {}",
        if has_test_templates { "✅" } else { "❌" }
    );
    println!(
        "  ✓ 4-step workflow: {}",
        if has_4_step_workflow { "✅" } else { "❌" }
    );

    let score = [
        has_benchmark_pattern,
        has_structure_validation,
        has_domain_coverage,
        has_test_templates,
        has_4_step_workflow,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(
        score, 5,
        "Should have complete benchmark creation guide pattern"
    );
}

#[test]
fn test_mask_improver_v4_improvement_notes() {
    println!("\n=== V4 BENCHMARK: Mask Improver - Version 4 Documented ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask-improver mask");

    let content = &mask.content;

    let has_v4_section = content.contains("### Version 4")
        || content.contains("Version 4 (2025-11-05)");
    let has_playwright_mention = content.contains("playwright-tester")
        || content.contains("playwright");
    let has_meta_learning_claim = content.contains("meta-meta-learning")
        || content.contains("Meta-Learning from");
    let has_pattern_count_growth = content.contains("5 → 11")
        || content.contains("11 patterns");
    let has_learning_loop = content.contains("Create mask")
        && content.contains("Extract patterns")
        && content.contains("EASIER");

    println!("Version 4 Documentation:");
    println!(
        "  ✓ Version 4 section present: {}",
        if has_v4_section { "✅" } else { "❌" }
    );
    println!(
        "  ✓ playwright-tester mentioned: {}",
        if has_playwright_mention { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Meta-learning claim: {}",
        if has_meta_learning_claim { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Pattern growth documented: {}",
        if has_pattern_count_growth { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Learning loop explained: {}",
        if has_learning_loop { "✅" } else { "❌" }
    );

    let score = [
        has_v4_section,
        has_playwright_mention,
        has_meta_learning_claim,
        has_pattern_count_growth,
        has_learning_loop,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(score, 5, "Should have complete v4 improvement documentation");
}
