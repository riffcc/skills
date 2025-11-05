// Example test demonstrating the test logging infrastructure
// This shows how to use BenchmarkLogger to track test results over time

use palace_skills::*;

#[test]
fn example_logged_benchmark() {
    println!("\n=== EXAMPLE: Logged Benchmark Test ===\n");

    // Create a logger for this benchmark run
    let mut logger = BenchmarkLogger::new("example-mask", 1);

    // Add context about this test run
    logger.add_context("Example benchmark demonstrating logging infrastructure");
    logger.add_context("This test shows how to capture and store test results with git context");

    // Load a mask (using mask-improver as an example)
    let mask = load_mask_from_file("mask-improver", "sonnet")
        .expect("Failed to load mask");

    // Test 1: Check structure
    println!("Running Test 1: Structure Validation...");
    let has_identity = mask.content.contains("## Identity");
    let has_expertise = mask.content.contains("## Core Expertise");
    let has_mission = mask.content.contains("## Your Mission");

    let structure_score = [has_identity, has_expertise, has_mission]
        .iter()
        .filter(|&&x| x)
        .count();

    let structure_passed = structure_score == 3;
    logger.log_test(
        "structure_validation",
        structure_passed,
        Some(&format!("{}/3", structure_score)),
        "Validates that mask has core structural sections",
    );

    println!(
        "  {} Structure: {}/3",
        if structure_passed { "✓" } else { "✗" },
        structure_score
    );

    // Test 2: Check for improvement notes
    println!("Running Test 2: Improvement Notes...");
    let has_improvement_notes = mask.content.contains("## Improvement Notes")
        || mask.content.contains("### Version");
    let has_version_history = mask.content.contains("Version 1")
        || mask.content.contains("v1");

    let history_score = [has_improvement_notes, has_version_history]
        .iter()
        .filter(|&&x| x)
        .count();

    let history_passed = history_score >= 1;
    logger.log_test(
        "improvement_history",
        history_passed,
        Some(&format!("{}/2", history_score)),
        "Validates that mask tracks its improvement history",
    );

    println!(
        "  {} History: {}/2",
        if history_passed { "✓" } else { "✗" },
        history_score
    );

    // Test 3: Check content length (reasonable mask size)
    println!("Running Test 3: Content Length...");
    let content_length = mask.content.len();
    let has_sufficient_content = content_length > 1000;
    let not_too_long = content_length < 100000;
    let length_passed = has_sufficient_content && not_too_long;

    logger.log_test(
        "content_length",
        length_passed,
        Some(&format!("{} chars", content_length)),
        &format!(
            "Validates mask has reasonable content length (should be > 1000 and < 100000, got {})",
            content_length
        ),
    );

    println!(
        "  {} Length: {} characters (should be 1000-100000)",
        if length_passed { "✓" } else { "✗" },
        content_length
    );

    // Save results to disk
    println!("\nSaving benchmark results...");
    logger
        .save_results()
        .expect("Failed to save benchmark results");

    // Assert all tests passed
    assert!(structure_passed, "Structure validation failed");
    assert!(history_passed, "Improvement history validation failed");
    assert!(length_passed, "Content length validation failed");

    println!("\n✅ All tests passed!");
}

#[test]
fn example_failing_test_logged() {
    println!("\n=== EXAMPLE: Logged Test with Failures ===\n");

    // This test intentionally includes some failures to show how they're logged

    let mut logger = BenchmarkLogger::new("example-mask", 1);
    logger.add_context("Example showing how failed tests are logged");
    logger.add_context("Some tests intentionally fail to demonstrate logging");

    // Test 1: This will pass
    println!("Running Test 1: Will Pass...");
    logger.log_test(
        "passing_test",
        true,
        Some("5/5"),
        "This test passed successfully",
    );
    println!("  ✓ Test 1 passed");

    // Test 2: This will fail
    println!("Running Test 2: Will Fail...");
    logger.log_test(
        "failing_test",
        false,
        Some("2/5"),
        "This test failed - only 2 out of 5 checks passed",
    );
    println!("  ✗ Test 2 failed");

    // Test 3: Another pass
    println!("Running Test 3: Will Pass...");
    logger.log_test(
        "another_passing_test",
        true,
        Some("100%"),
        "All checks completed successfully",
    );
    println!("  ✓ Test 3 passed");

    // Save results (this will show 2 passed, 1 failed)
    println!("\nSaving benchmark results...");
    logger
        .save_results()
        .expect("Failed to save benchmark results");

    println!("\n⚠️  Test run completed with failures (2/3 passed)");
    // Note: We don't assert here since this is intentionally showing failures
}
