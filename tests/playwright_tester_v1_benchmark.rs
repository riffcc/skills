// Benchmark tests for playwright-tester mask v1
// These validate that the mask has the necessary expertise to perform comprehensive website audits

use palace_skills::*;

#[test]
fn test_playwright_tester_structure() {
    println!("\n=== V1 BENCHMARK: Playwright Tester - Structure ===\n");

    let mask = load_mask_from_file("playwright-tester", "sonnet")
        .expect("Failed to load playwright-tester mask");

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
fn test_playwright_tester_mcp_integration() {
    println!("\n=== V1 BENCHMARK: Playwright Tester - MCP Integration ===\n");

    let mask = load_mask_from_file("playwright-tester", "sonnet")
        .expect("Failed to load playwright-tester mask");

    let content = &mask.content;

    // Check for MCP-specific content
    let has_mcp_mention = content.contains("Playwright MCP") || content.contains("@playwright/mcp");
    let has_mcp_commands = content.contains("playwright_navigate")
        && content.contains("playwright_click")
        && content.contains("playwright_fill");
    let has_mcp_section = content.contains("## MCP Integration");
    let mentions_mcp_server = content.contains("MCP server") || content.contains("mcpServers");

    println!("MCP Integration Checks:");
    println!("  ✓ MCP mentioned: {}", if has_mcp_mention { "✅" } else { "❌" });
    println!(
        "  ✓ MCP commands documented: {}",
        if has_mcp_commands { "✅" } else { "❌" }
    );
    println!(
        "  ✓ MCP integration section: {}",
        if has_mcp_section { "✅" } else { "❌" }
    );
    println!(
        "  ✓ MCP server referenced: {}",
        if mentions_mcp_server { "✅" } else { "❌" }
    );

    let score = [has_mcp_mention, has_mcp_commands, has_mcp_section, mentions_mcp_server]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("\nScore: {}/4", score);

    assert!(
        score >= 3,
        "Should have at least 3/4 MCP integration elements"
    );
}

#[test]
fn test_playwright_tester_multi_dimensional_testing() {
    println!("\n=== V1 BENCHMARK: Playwright Tester - Multi-Dimensional Testing ===\n");

    let mask = load_mask_from_file("playwright-tester", "sonnet")
        .expect("Failed to load playwright-tester mask");

    let content = &mask.content;

    // Check for all testing dimensions
    let has_functional = content.contains("Functional Testing") || content.contains("functional");
    let has_usability = content.contains("Usability") || content.contains("UX");
    let has_accessibility = content.contains("Accessibility") || content.contains("a11y") || content.contains("WCAG");
    let has_translation = content.contains("Translation") || content.contains("i18n") || content.contains("Internationalization");
    let has_performance = content.contains("Performance") || content.contains("Core Web Vitals");

    println!("Testing Dimensions:");
    println!(
        "  ✓ Functional testing: {}",
        if has_functional { "✅" } else { "❌" }
    );
    println!("  ✓ Usability testing: {}", if has_usability { "✅" } else { "❌" });
    println!(
        "  ✓ Accessibility testing: {}",
        if has_accessibility { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Translation/i18n testing: {}",
        if has_translation { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Performance testing: {}",
        if has_performance { "✅" } else { "❌" }
    );

    let score =
        [has_functional, has_usability, has_accessibility, has_translation, has_performance]
            .iter()
            .filter(|&&x| x)
            .count();

    println!("\nScore: {}/5", score);

    assert_eq!(score, 5, "Should cover all 5 testing dimensions");
}

#[test]
fn test_playwright_tester_scoring_framework() {
    println!("\n=== V1 BENCHMARK: Playwright Tester - Scoring Framework ===\n");

    let mask = load_mask_from_file("playwright-tester", "sonnet")
        .expect("Failed to load playwright-tester mask");

    let content = &mask.content;

    // Check for scoring-related content
    let has_overall_score = content.contains("Overall Score") || content.contains("overall score");
    let has_dimension_scores = content.contains("dimension score") || content.contains("per-dimension");
    let has_severity_levels = (content.contains("Critical") || content.contains("critical"))
        && (content.contains("Major") || content.contains("major"))
        && (content.contains("Minor") || content.contains("minor"));
    let has_prioritization = content.contains("Priorit") || content.contains("priorit");
    let has_quantitative = content.contains("0-100") || content.contains("/100");

    println!("Scoring Framework:");
    println!(
        "  ✓ Overall score concept: {}",
        if has_overall_score { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Per-dimension scoring: {}",
        if has_dimension_scores { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Severity levels (Critical/Major/Minor): {}",
        if has_severity_levels { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Prioritization strategy: {}",
        if has_prioritization { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Quantitative scores: {}",
        if has_quantitative { "✅" } else { "❌" }
    );

    let score = [
        has_overall_score,
        has_dimension_scores,
        has_severity_levels,
        has_prioritization,
        has_quantitative,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert!(score >= 4, "Should have comprehensive scoring framework");
}

#[test]
fn test_playwright_tester_cicd_integration() {
    println!("\n=== V1 BENCHMARK: Playwright Tester - CI/CD Integration ===\n");

    let mask = load_mask_from_file("playwright-tester", "sonnet")
        .expect("Failed to load playwright-tester mask");

    let content = &mask.content;

    // Check for CI/CD-related content
    let has_cicd_mention = content.contains("CI/CD") || content.contains("CI pipeline");
    let has_cargo_tests = content.contains("cargo test") || content.contains("#[test]");
    let has_github_actions = content.contains("GitHub Actions") || content.contains(".github/workflows");
    let has_regression_testing = content.contains("regression") || content.contains("prevent regress");
    let has_test_generation = content.contains("Generate") && (content.contains("test") || content.contains("Test"));

    println!("CI/CD Integration:");
    println!(
        "  ✓ CI/CD mentioned: {}",
        if has_cicd_mention { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Cargo test examples: {}",
        if has_cargo_tests { "✅" } else { "❌" }
    );
    println!(
        "  ✓ GitHub Actions integration: {}",
        if has_github_actions { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Regression testing concept: {}",
        if has_regression_testing { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Test generation capability: {}",
        if has_test_generation { "✅" } else { "❌" }
    );

    let score = [
        has_cicd_mention,
        has_cargo_tests,
        has_github_actions,
        has_regression_testing,
        has_test_generation,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("\nScore: {}/5", score);

    assert_eq!(score, 5, "Should have complete CI/CD integration guidance");
}

#[test]
fn test_playwright_tester_comprehensive_examples() {
    println!("\n=== V1 BENCHMARK: Playwright Tester - Comprehensive Examples ===\n");

    let mask = load_mask_from_file("playwright-tester", "sonnet")
        .expect("Failed to load playwright-tester mask");

    let content = &mask.content;

    // Check for example quality
    let example_count = content.matches("### Example").count();
    let has_ecommerce_example = content.contains("E-Commerce") || content.contains("e-commerce");
    let has_before_after = content.contains("Before") || content.contains("After");
    let has_specific_scores = content.contains("/100") && content.matches("/100").count() > 5;
    let has_concrete_recommendations = content.contains("Recommendation") && content.matches("CRITICAL").count() > 0;

    println!("Example Quality:");
    println!("  ✓ Number of examples: {}", example_count);
    println!(
        "  ✓ E-commerce example: {}",
        if has_ecommerce_example { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Before/after comparison: {}",
        if has_before_after { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Specific scores shown: {}",
        if has_specific_scores { "✅" } else { "❌" }
    );
    println!(
        "  ✓ Concrete recommendations: {}",
        if has_concrete_recommendations { "✅" } else { "❌" }
    );

    println!("\nScore: {}/5 dimensions + {} examples",
        [has_ecommerce_example, has_before_after, has_specific_scores, has_concrete_recommendations]
            .iter()
            .filter(|&&x| x)
            .count() + 1,
        example_count
    );

    assert!(example_count >= 2, "Should have at least 2 detailed examples");
    assert!(
        has_specific_scores,
        "Examples should include specific numerical scores"
    );
}
