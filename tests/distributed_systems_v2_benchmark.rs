
se palace_skills::*;

/// V2 Benchmarks for distributed-systems mask
///
/// V1 tested for PRESENCE of concepts (keywords)
/// V2 tests for DEPTH of understanding (examples, trade-offs, failure scenarios)

#[test]
fn v2_cap_theorem_tradeoffs() {
    println!("\n=== V2 BENCHMARK: Distributed Systems - CAP Trade-off Analysis ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for "CAP" keyword
    // V2 requires actual trade-off analysis with examples

    let has_cp_system = content.contains("CP system")
        || content.contains("Consistency + Partition")
        || content.contains("Consistency and Partition");

    let has_ap_system = content.contains("AP system")
        || content.contains("Availability + Partition")
        || content.contains("Availability and Partition");

    let has_ca_impossibility = content.contains("CA system")
        && (content.contains("impossible")
            || content.contains("cannot exist")
            || content.contains("network partition"));

    // Must have concrete example showing when to choose CP vs AP
    let has_cap_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("CP") || content.contains("AP"));

    // Must explain WHY you'd choose one over the other
    let has_tradeoff_explanation = content.contains("trade-off") && content.contains("CAP");

    assert!(has_cp_system, "V2: Must explain CP systems with examples");
    assert!(has_ap_system, "V2: Must explain AP systems with examples");

    if has_cap_example {
        println!("✓ V2: CAP trade-offs demonstrated with examples");
    } else {
        println!("⚠ V2: CAP mentioned but lacks concrete examples showing when to choose CP vs AP");
    }

    if has_tradeoff_explanation {
        println!("✓ V2: Trade-off reasoning explained");
    } else {
        println!("⚠ V2: Trade-offs not explicitly explained");
    }

    let score = [
        has_cp_system,
        has_ap_system,
        has_cap_example,
        has_tradeoff_explanation,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 CAP Score: {}/4", score);

    // V2 is stricter - require at least 3/4
    assert!(
        score >= 3,
        "V2: Must score 3/4 or higher on CAP depth (got {}/4)",
        score
    );
}

#[test]
fn v2_consensus_failure_scenarios() {
    println!("\n=== V2 BENCHMARK: Distributed Systems - Consensus Failure Scenarios ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 checked for "Raft" keyword
    // V2 requires failure mode coverage

    let has_split_brain = content.contains("split-brain") || content.contains("split brain");
    let has_quorum_loss = content.contains("quorum loss")
        || content.contains("lost quorum")
        || content.contains("quorum is lost");

    let has_leader_failure = content.contains("leader")
        && (content.contains("crash") || content.contains("fail") || content.contains("election"));

    let has_network_partition = content.contains("partition") && content.contains("network");

    // Must have example showing what happens when consensus fails
    let has_failure_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("failure")
            || content.contains("fail")
            || content.contains("partition"));

    let score = [
        has_split_brain,
        has_quorum_loss,
        has_leader_failure,
        has_network_partition,
        has_failure_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Failure Scenarios: {}/5 covered", score);

    if has_failure_example {
        println!("✓ V2: Failure scenarios demonstrated with examples");
    } else {
        println!("⚠ V2: Failure modes mentioned but lack concrete recovery examples");
    }

    // V2 is stricter - require 4/5
    assert!(
        score >= 4,
        "V2: Must cover 4/5 failure scenarios (got {}/5)",
        score
    );
}

#[test]
fn v2_production_examples() {
    println!("\n=== V2 BENCHMARK: Distributed Systems - Production-Ready Examples ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for examples existing
    // V2 requires DETAILED examples with configurations

    // Count proper example sections
    let example_count = content.matches("### Example").count();

    // Check for code/config blocks
    let has_yaml = content.contains("```yaml") || content.contains("```yml");
    let has_toml = content.contains("```toml");
    let has_code = content.contains("```")
        && (content.contains("etcd")
            || content.contains("synchronous_commit")
            || content.contains("HAProxy"));

    // Check for specific metrics/thresholds
    let has_metrics = content.contains("metrics") || content.contains("monitoring");
    let has_thresholds = content.contains("ms")
        || content.contains("latency <")
        || content.contains("lag <")
        || content.contains("%");

    // Check for deployment details (not just theory)
    let has_deployment_details = content.contains("deploy")
        || content.contains("configuration")
        || content.contains("setup");

    println!("V2 Examples: {} proper example sections", example_count);
    println!(
        "V2 Configurations: {}",
        if has_yaml || has_toml || has_code {
            "Present"
        } else {
            "Missing"
        }
    );
    println!(
        "V2 Metrics: {}",
        if has_metrics { "Covered" } else { "Missing" }
    );
    println!(
        "V2 Thresholds: {}",
        if has_thresholds {
            "Specified"
        } else {
            "Missing"
        }
    );

    // V2 requires at least 3 examples
    assert!(
        example_count >= 3,
        "V2: Must have at least 3 detailed examples (found {})",
        example_count
    );

    // V2 requires actual configurations in at least one example
    assert!(
        has_yaml || has_toml || has_code,
        "V2: Must include actual configuration code blocks"
    );

    // V2 requires specific metrics/thresholds
    assert!(
        has_metrics && has_thresholds,
        "V2: Must specify concrete metrics and thresholds"
    );

    println!("✓ V2: Production-ready examples with configurations and metrics");
}

#[test]
fn v2_observability_depth() {
    println!("\n=== V2 BENCHMARK: Distributed Systems - Observability Depth ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 checked for "Prometheus" and "Grafana" keywords
    // V2 requires specific metrics, dashboards, and alerting rules

    // Specific exporters
    let has_node_exporter = content.contains("node_exporter");
    let has_postgres_exporter = content.contains("postgres_exporter");
    let has_specific_exporters =
        has_node_exporter || has_postgres_exporter || content.contains("exporter");

    // Specific metrics (not just "metrics" keyword)
    let has_specific_metrics = content.contains("replication_lag")
        || content.contains("replay_lag")
        || content.contains("cpu_usage")
        || content.contains("disk_io");

    // Alerting rules with thresholds
    let has_alerting = content.contains("alert") || content.contains("Alert");
    let has_alert_thresholds =
        content.contains(" > ") || content.contains(" < ") || content.contains("threshold");

    // Dashboard creation mentioned
    let has_dashboards = content.contains("dashboard") || content.contains("Dashboard");

    // Distributed tracing details
    let has_tracing_detail = (content.contains("Jaeger") || content.contains("Zipkin"))
        && (content.contains("trace") || content.contains("span"));

    let score = [
        has_specific_exporters,
        has_specific_metrics,
        has_alerting,
        has_alert_thresholds,
        has_dashboards,
        has_tracing_detail,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Observability Depth: {}/6", score);

    // V2 requires 5/6
    assert!(
        score >= 5,
        "V2: Must have deep observability coverage (got {}/6)",
        score
    );
}

#[test]
fn v2_antipatterns() {
    println!("\n=== V2 BENCHMARK: Distributed Systems - Anti-patterns ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires explicit "what NOT to do" guidance
    // This is critical for avoiding production mistakes

    let has_avoid_section = content.contains("## What to Avoid")
        || content.contains("### What to Avoid")
        || content.contains("What NOT to do");

    let has_dont_statements = content.matches("Don't").count() >= 3
        || content.matches("don't").count() >= 3
        || content.matches("Never").count() >= 2;

    let has_antipattern_examples = content.contains("antipattern")
        || content.contains("anti-pattern")
        || (content.contains("bad") && content.contains("example"));

    // Specific anti-patterns for distributed systems
    let warns_premature_optimization = content.contains("premature")
        || (content.contains("don't")
            && content.contains("consensus")
            && content.contains("simple"));

    let warns_network_assumptions = content.contains("network")
        && (content.contains("don't assume")
            || content.contains("never assume")
            || content.contains("Networks partition"));

    let score = [
        has_avoid_section,
        has_dont_statements,
        has_antipattern_examples,
        warns_premature_optimization,
        warns_network_assumptions,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Anti-patterns: {}/5 criteria met", score);

    if has_avoid_section {
        println!("✓ V2: Dedicated anti-pattern section present");
    } else {
        println!("⚠ V2: No dedicated anti-pattern section");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must cover anti-patterns extensively (got {}/5)",
        score
    );
}
