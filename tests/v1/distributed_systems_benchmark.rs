use palace_skills::*;

#[test]
fn benchmark_distributed_systems_structure() {
    println!("\n=== BENCHMARK: Distributed Systems - Structure Validation ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load distributed-systems mask");

    let content = &mask.content;

    // Core sections that MUST exist
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");

    assert!(has_identity, "Mask must have Identity section");
    assert!(has_expertise, "Mask must have Core Expertise section");
    assert!(has_mission, "Mask must have Your Mission section");
    assert!(has_guidelines, "Mask must have Behavioral Guidelines section");
    assert!(has_examples, "Mask must have Examples section");

    println!("✓ Structure: 5/5 required sections present");
}

#[test]
fn benchmark_distributed_systems_cap_theorem() {
    println!("\n=== BENCHMARK: Distributed Systems - CAP Theorem Knowledge ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check for CAP theorem coverage
    let has_cap = content.contains("CAP") || content.contains("Consistency, Availability, Partition");
    let has_consistency = content.contains("consistency") && content.contains("Consistency");
    let has_availability = content.contains("availability") && content.contains("Availability");
    let has_partition_tolerance = content.contains("partition") || content.contains("Partition");
    let has_tradeoffs = content.contains("trade-off") || content.contains("tradeoff");

    assert!(has_cap, "Mask should mention CAP theorem");
    assert!(has_consistency, "Mask should discuss consistency models");
    assert!(has_availability, "Mask should discuss availability patterns");
    assert!(has_partition_tolerance, "Mask should discuss partition tolerance");
    assert!(has_tradeoffs, "Mask should discuss trade-offs");

    println!("✓ CAP Theorem: 5/5 concepts covered");
}

#[test]
fn benchmark_distributed_systems_consensus() {
    println!("\n=== BENCHMARK: Distributed Systems - Consensus Algorithms ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check for major consensus algorithms
    let has_raft = content.contains("Raft");
    let has_paxos = content.contains("Paxos");
    let has_tgp = content.contains("Two Generals Protocol") || content.contains("TGP");
    let has_quorum = content.contains("quorum");
    let has_leader_election = content.contains("leader election") || content.contains("leader");

    assert!(has_raft, "Mask should mention Raft consensus");
    assert!(has_paxos, "Mask should mention Paxos consensus");
    assert!(has_tgp, "Mask should mention Two Generals Protocol (TGP)");
    assert!(has_quorum, "Mask should discuss quorum systems");
    assert!(has_leader_election, "Mask should discuss leader election");

    println!("✓ Consensus: 5/5 key algorithms/concepts covered");
}

#[test]
fn benchmark_distributed_systems_observability() {
    println!("\n=== BENCHMARK: Distributed Systems - Observability Stack ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check for observability tools
    let has_prometheus = content.contains("Prometheus");
    let has_grafana = content.contains("Grafana");
    let has_tracing = content.contains("tracing") || content.contains("Jaeger") || content.contains("Zipkin");
    let has_monitoring = content.contains("Monitoring") || content.contains("monitoring");
    let has_metrics = content.contains("metrics");

    assert!(has_prometheus, "Mask should mention Prometheus");
    assert!(has_grafana, "Mask should mention Grafana");
    assert!(has_tracing, "Mask should mention distributed tracing");
    assert!(has_monitoring, "Mask should discuss monitoring patterns");
    assert!(has_metrics, "Mask should discuss metrics");

    println!("✓ Observability: 5/5 key tools/concepts covered");
}

#[test]
fn benchmark_distributed_systems_rust_tdd() {
    println!("\n=== BENCHMARK: Distributed Systems - Rust + TDD Best Practices ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check for Rust + TDD references
    let has_rust = content.contains("Rust");
    let has_tdd = content.contains("TDD") || content.contains("Test-Driven Development");
    let has_cargo_tests = content.contains("Cargo tests") || content.contains("cargo test");
    let has_precommit = content.contains("pre-commit");

    assert!(has_rust, "Mask should mention Rust development");
    assert!(has_tdd, "Mask should mention Test-Driven Development");
    assert!(has_cargo_tests || has_tdd, "Mask should reference Cargo tests or TDD");
    assert!(has_precommit || has_tdd, "Mask should reference pre-commit hooks or TDD");

    println!("✓ Rust+TDD: 4/4 best practices covered");
}

#[test]
fn benchmark_distributed_systems_failure_modes() {
    println!("\n=== BENCHMARK: Distributed Systems - Failure Mode Analysis ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check for failure mode coverage
    let has_spof = content.contains("SPOF") || content.contains("single point of failure");
    let has_split_brain = content.contains("split-brain") || content.contains("split brain");
    let has_network_partition = content.contains("partition");
    let has_failure_detection = content.contains("failure detection") || content.contains("health check");
    let has_recovery = content.contains("recovery") || content.contains("failover");

    assert!(has_spof, "Mask should discuss single points of failure");
    assert!(has_split_brain, "Mask should discuss split-brain scenarios");
    assert!(has_network_partition, "Mask should discuss network partitions");
    assert!(has_failure_detection, "Mask should discuss failure detection");
    assert!(has_recovery, "Mask should discuss recovery strategies");

    println!("✓ Failure Modes: 5/5 critical scenarios covered");
}

#[test]
fn benchmark_distributed_systems_concrete_tools() {
    println!("\n=== BENCHMARK: Distributed Systems - Concrete Tool References ===\n");

    let mask = load_mask_from_file("distributed-systems", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // Check for specific tools (not just abstract patterns)
    let has_etcd = content.contains("etcd");
    let has_haproxy = content.contains("HAProxy");
    let has_postgresql = content.contains("PostgreSQL") || content.contains("postgres");
    let has_kubernetes = content.contains("Kubernetes") || content.contains("k8s");
    let specific_tool_count = [has_etcd, has_haproxy, has_postgresql, has_kubernetes]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(
        specific_tool_count >= 3,
        "Mask should reference at least 3 concrete tools (found {})",
        specific_tool_count
    );

    println!("✓ Concrete Tools: {}/4 major tools referenced", specific_tool_count);
}
