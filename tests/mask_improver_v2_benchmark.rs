use palace_skills::*;

/// V2 Benchmarks for mask-improver mask
///
/// V1 tested for PRESENCE of capabilities (analysis, specificity, evidence)
/// V2 tests for DEPTH and META-LEARNING (before/after diffs, score prediction, cross-domain patterns)

#[test]
fn v2_before_after_diffs() {
    println!("\n=== V2 BENCHMARK: Mask Improver - Before/After Diffs ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires showing exact text changes (before/after diffs)

    let has_before_after = content.contains("before") && content.contains("after");

    let has_diff_notation =
        content.contains("```diff") || content.contains("Before:") || content.contains("After:");

    let has_exact_text =
        content.contains("exact") || content.contains("specific") || content.contains("verbatim");

    // Must mention showing diffs in output
    let has_diff_in_output =
        content.contains("Output") && (content.contains("before") || content.contains("diff"));

    // Must have example showing diff format
    let has_diff_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("before") || content.contains("after") || content.contains("diff"));

    let score = [
        has_before_after,
        has_diff_notation,
        has_exact_text,
        has_diff_in_output,
        has_diff_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Before/After Diffs: {}/5", score);

    if has_diff_example {
        println!("✓ V2: Diff format demonstrated with examples");
    } else {
        println!("⚠ V2: Diffs mentioned but lack concrete format examples");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must provide before/after diff capability (got {}/5)",
        score
    );
}

#[test]
fn v2_score_prediction() {
    println!("\n=== V2 BENCHMARK: Mask Improver - Score Prediction ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires predicting exact score changes

    let has_prediction = content.contains("predict") || content.contains("expect");

    let has_score_mention = content.contains("score") || content.contains("benchmark");

    let has_percentage =
        content.contains("%") || content.contains("percent") || content.contains("score:");

    let has_impact_analysis =
        content.contains("impact") || content.contains("effect") || content.contains("result");

    // Must mention predicting in output
    let has_prediction_in_output =
        content.contains("Output") && (content.contains("predict") || content.contains("expect"));

    let score = [
        has_prediction,
        has_score_mention,
        has_percentage,
        has_impact_analysis,
        has_prediction_in_output,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Score Prediction: {}/5", score);

    if has_prediction_in_output {
        println!("✓ V2: Score prediction included in output format");
    } else {
        println!("⚠ V2: Prediction capability present but not required in output");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must predict score changes (got {}/5)",
        score
    );
}

#[test]
fn v2_implementation_difficulty() {
    println!("\n=== V2 BENCHMARK: Mask Improver - Implementation Difficulty Ratings ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires rating implementation difficulty

    let has_difficulty = content.contains("difficulty")
        || content.contains("Difficulty")
        || content.contains("effort");

    let has_difficulty_levels = content.contains("trivial")
        || content.contains("easy")
        || content.contains("moderate")
        || content.contains("hard");

    let has_complexity = content.contains("complex") || content.contains("simple");

    let has_time_estimate =
        content.contains("time") || content.contains("lines") || content.contains("minutes");

    // Must mention difficulty in output
    let has_difficulty_in_output = content.contains("Output")
        && (content.contains("difficulty") || content.contains("effort"));

    let score = [
        has_difficulty,
        has_difficulty_levels,
        has_complexity,
        has_time_estimate,
        has_difficulty_in_output,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Implementation Difficulty: {}/5", score);

    if has_difficulty_in_output {
        println!("✓ V2: Difficulty ratings included in output format");
    } else {
        println!("⚠ V2: Difficulty assessment present but not required in output");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must rate implementation difficulty (got {}/5)",
        score
    );
}

#[test]
fn v2_priority_framework() {
    println!("\n=== V2 BENCHMARK: Mask Improver - Priority Framework ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires priority/impact framework

    let has_priority = content.contains("priority")
        || content.contains("Priority")
        || content.contains("prioritize");

    let has_impact_effort = (content.contains("impact") && content.contains("effort"))
        || content.contains("impact × effort")
        || content.contains("impact vs effort");

    let has_high_priority = content.contains("high priority")
        || content.contains("critical")
        || content.contains("most important");

    let has_ordering =
        content.contains("order") || content.contains("sequence") || content.contains("first");

    // Must have example showing priority decision
    let has_priority_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("priority") || content.contains("impact"));

    let score = [
        has_priority,
        has_impact_effort,
        has_high_priority,
        has_ordering,
        has_priority_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Priority Framework: {}/5", score);

    if has_priority_example {
        println!("✓ V2: Priority framework demonstrated with examples");
    } else {
        println!("⚠ V2: Priority mentioned but lacks concrete decision framework");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must provide priority framework (got {}/5)",
        score
    );
}

#[test]
fn v2_cross_domain_patterns() {
    println!("\n=== V2 BENCHMARK: Mask Improver - Cross-Domain Pattern Library ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires identifying patterns that work across domains

    let has_pattern_mention = content.contains("pattern") || content.contains("Pattern");

    let has_cross_domain = content.contains("cross-domain")
        || content.contains("across")
        || content.contains("other masks");

    let has_reusable =
        content.contains("reuse") || content.contains("apply") || content.contains("generalize");

    let has_pattern_library = content.contains("library")
        || content.contains("catalog")
        || content.contains("collection");

    // Must have example of cross-domain pattern
    let has_pattern_example = (content.contains("### Example") || content.contains("**Example"))
        && content.contains("pattern");

    let score = [
        has_pattern_mention,
        has_cross_domain,
        has_reusable,
        has_pattern_library,
        has_pattern_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Cross-Domain Patterns: {}/5", score);

    if has_pattern_example {
        println!("✓ V2: Cross-domain patterns demonstrated with examples");
    } else {
        println!("⚠ V2: Patterns mentioned but lack cross-domain application examples");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must identify cross-domain patterns (got {}/5)",
        score
    );
}

#[test]
fn v2_meta_learning_capability() {
    println!("\n=== V2 BENCHMARK: Mask Improver - Meta-Learning Capability ===\n");

    let mask = load_mask_from_file("mask-improver", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires meta-learning: improving itself based on past improvements

    let has_meta = content.contains("meta") || content.contains("Meta");

    let has_self_improvement = content.contains("self-improve")
        || content.contains("improve itself")
        || content.contains("own improvement");

    let has_learning =
        content.contains("learn") || content.contains("discover") || content.contains("extract");

    let has_feedback_loop =
        content.contains("feedback") || content.contains("iteration") || content.contains("cycle");

    // Must explain meta-learning in mission or guidelines
    let has_meta_in_mission = (content.contains("## Your Mission")
        || content.contains("## Behavioral Guidelines"))
        && (content.contains("meta") || content.contains("learn") || content.contains("improve"));

    // Must have example of meta-learning
    let has_meta_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("meta") || content.contains("learn") || content.contains("itself"));

    let score = [
        has_meta,
        has_self_improvement,
        has_learning,
        has_feedback_loop,
        has_meta_in_mission,
        has_meta_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Meta-Learning: {}/6", score);

    if has_meta_example {
        println!("✓ V2: Meta-learning demonstrated with examples");
    } else {
        println!("⚠ V2: Meta-learning mentioned but lacks concrete examples");
    }

    // V2 requires 5/6 (meta-learning is the key differentiator for mask-improver)
    assert!(
        score >= 5,
        "V2: Must demonstrate meta-learning capability (got {}/6)",
        score
    );
}
