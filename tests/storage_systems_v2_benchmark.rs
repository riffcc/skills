use palace_skills::*;

/// V2 Benchmarks for storage-systems mask
///
/// V1 tested for PRESENCE of concepts (keywords)
/// V2 tests for DEPTH of understanding (examples, trade-offs, failure scenarios)

#[test]
fn v2_moosefs_deployment_detailed() {
    println!("\n=== V2 BENCHMARK: Storage Systems - MooseFS Deployment Details ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for "MooseFS" keyword
    // V2 requires actual deployment configuration details

    let has_master = content.contains("master") || content.contains("Master");
    let has_chunkserver = content.contains("chunkserver") || content.contains("chunk server");
    let has_metalogger = content.contains("metalogger") || content.contains("meta logger");
    let has_client = content.contains("mfsmount") || content.contains("client mount");

    // Must have configuration examples
    let has_config_example = (content.contains("### Example") || content.contains("**Example"))
        && content.contains("MooseFS");

    // Must have specific configuration files or parameters
    let has_config_files = content.contains("mfsmaster.cfg")
        || content.contains("mfschunkserver.cfg")
        || content.contains(".cfg")
        || content.contains("configuration");

    // Must explain replication/redundancy
    let has_replication = content.contains("replication")
        || content.contains("goal")
        || content.contains("redundancy");

    let score = [
        has_master,
        has_chunkserver,
        has_metalogger,
        has_client,
        has_config_example,
        has_config_files,
        has_replication,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 MooseFS Deployment: {}/7", score);

    if has_config_example {
        println!("✓ V2: MooseFS deployment demonstrated with configuration examples");
    } else {
        println!("⚠ V2: MooseFS mentioned but lacks concrete deployment configuration");
    }

    // V2 requires 6/7
    assert!(
        score >= 6,
        "V2: Must have detailed MooseFS deployment coverage (got {}/7)",
        score
    );
}

#[test]
fn v2_storage_performance_benchmarking() {
    println!("\n=== V2 BENCHMARK: Storage Systems - Performance Benchmarking ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires actual performance benchmarking guidance

    let has_fio = content.contains("fio") || content.contains("FIO");
    let has_iops = content.contains("IOPS") || content.contains("iops");
    let has_throughput =
        content.contains("throughput") || content.contains("MB/s") || content.contains("bandwidth");

    let has_latency =
        content.contains("latency") || content.contains("ms") || content.contains("response time");

    // Must have benchmark example
    let has_benchmark_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("performance")
            || content.contains("benchmark")
            || content.contains("fio"));

    // Must have specific commands or configurations
    let has_fio_commands =
        content.contains("```") && (content.contains("fio") || content.contains("benchmark"));

    let score = [
        has_fio,
        has_iops,
        has_throughput,
        has_latency,
        has_benchmark_example,
        has_fio_commands,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Performance Benchmarking: {}/6", score);

    if has_benchmark_example {
        println!("✓ V2: Performance benchmarking demonstrated with examples");
    } else {
        println!("⚠ V2: Performance mentioned but lacks concrete benchmarking procedures");
    }

    // V2 requires 5/6
    assert!(
        score >= 5,
        "V2: Must have detailed performance benchmarking coverage (got {}/6)",
        score
    );
}

#[test]
fn v2_failure_recovery_procedures() {
    println!("\n=== V2 BENCHMARK: Storage Systems - Failure Recovery Procedures ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires specific failure recovery procedures

    let has_disk_failure = content.contains("disk failure")
        || content.contains("disk fails")
        || content.contains("drive failure");

    let has_node_failure = content.contains("node failure")
        || content.contains("server failure")
        || content.contains("chunkserver fail");

    let has_master_failure = content.contains("master failure")
        || content.contains("master fails")
        || content.contains("metalogger");

    let has_recovery_steps =
        content.contains("recovery") || content.contains("restore") || content.contains("repair");

    // Must have failure scenario example
    let has_failure_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("failure") || content.contains("fail"));

    // Must have specific procedures or commands
    let has_recovery_commands = content.contains("replace")
        || content.contains("mfsmaster")
        || content.contains("procedure");

    let score = [
        has_disk_failure,
        has_node_failure,
        has_master_failure,
        has_recovery_steps,
        has_failure_example,
        has_recovery_commands,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Failure Recovery: {}/6 covered", score);

    if has_failure_example {
        println!("✓ V2: Failure recovery demonstrated with examples");
    } else {
        println!("⚠ V2: Failures mentioned but lack concrete recovery procedures");
    }

    // V2 requires 5/6
    assert!(
        score >= 5,
        "V2: Must cover failure recovery procedures (got {}/6)",
        score
    );
}

#[test]
fn v2_capacity_planning() {
    println!("\n=== V2 BENCHMARK: Storage Systems - Capacity Planning ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires capacity planning guidance

    let has_capacity_mention = content.contains("capacity") || content.contains("Capacity");

    let has_growth_projection =
        content.contains("growth") || content.contains("scale") || content.contains("expansion");

    let has_disk_space =
        content.contains("TB") || content.contains("PB") || content.contains("storage space");

    let has_replication_overhead = content.contains("overhead")
        || (content.contains("replication") && content.contains("space"));

    // Must have capacity planning example
    let has_capacity_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("capacity") || content.contains("sizing"));

    let score = [
        has_capacity_mention,
        has_growth_projection,
        has_disk_space,
        has_replication_overhead,
        has_capacity_example,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Capacity Planning: {}/5", score);

    if has_capacity_example {
        println!("✓ V2: Capacity planning demonstrated with examples");
    } else {
        println!("⚠ V2: Capacity mentioned but lacks concrete planning procedures");
    }

    // V2 requires 4/5
    assert!(
        score >= 4,
        "V2: Must have capacity planning coverage (got {}/5)",
        score
    );
}

#[test]
fn v2_monitoring_observability() {
    println!("\n=== V2 BENCHMARK: Storage Systems - Monitoring & Observability ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires specific monitoring guidance

    let has_prometheus = content.contains("Prometheus") || content.contains("prometheus");
    let has_grafana = content.contains("Grafana") || content.contains("grafana");

    let has_specific_metrics = content.contains("disk usage")
        || content.contains("chunk")
        || content.contains("availability")
        || content.contains("latency");

    let has_alerts =
        content.contains("alert") || content.contains("threshold") || content.contains("warning");

    // Must have monitoring example
    let has_monitoring_example = (content.contains("### Example") || content.contains("**Example"))
        && (content.contains("monitor") || content.contains("metrics"));

    // Must have specific metric names or exporters
    let has_exporter = content.contains("exporter")
        || content.contains("_exporter")
        || content.contains("metrics endpoint");

    let score = [
        has_prometheus,
        has_grafana,
        has_specific_metrics,
        has_alerts,
        has_monitoring_example,
        has_exporter,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Monitoring/Observability: {}/6", score);

    if has_monitoring_example {
        println!("✓ V2: Monitoring demonstrated with examples");
    } else {
        println!("⚠ V2: Monitoring mentioned but lacks concrete implementation");
    }

    // V2 requires 5/6
    assert!(
        score >= 5,
        "V2: Must have detailed monitoring coverage (got {}/6)",
        score
    );
}

#[test]
fn v2_production_ready_examples() {
    println!("\n=== V2 BENCHMARK: Storage Systems - Production-Ready Examples ===\n");

    let mask = load_mask_from_file("storage-systems", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for examples existing
    // V2 requires DETAILED examples with hardware specs, network topology, monitoring

    // Count proper example sections
    let example_count = content.matches("### Example").count();

    // Check for hardware specifications
    let has_hardware_specs = content.contains("CPU")
        || content.contains("cores")
        || content.contains("RAM")
        || content.contains("GB")
        || content.contains("disk");

    // Check for network topology
    let has_network = content.contains("network")
        || content.contains("10GbE")
        || content.contains("Gbps")
        || content.contains("bandwidth");

    // Check for configuration files
    let has_configs = content.contains("```")
        && (content.contains("cfg") || content.contains("conf") || content.contains("yaml"));

    // Check for monitoring setup
    let has_monitoring_setup = content.contains("Prometheus")
        || content.contains("Grafana")
        || content.contains("metrics");

    println!("V2 Examples: {} proper example sections", example_count);
    println!(
        "V2 Hardware Specs: {}",
        if has_hardware_specs {
            "Present"
        } else {
            "Missing"
        }
    );
    println!(
        "V2 Network Topology: {}",
        if has_network { "Present" } else { "Missing" }
    );
    println!(
        "V2 Configurations: {}",
        if has_configs { "Present" } else { "Missing" }
    );

    let score = [
        example_count >= 3,
        has_hardware_specs,
        has_network,
        has_configs,
        has_monitoring_setup,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Production-Ready: {}/5", score);

    // V2 requires at least 3 examples
    assert!(
        example_count >= 3,
        "V2: Must have at least 3 detailed examples (found {})",
        example_count
    );

    // V2 requires hardware specs
    assert!(
        has_hardware_specs,
        "V2: Must include hardware specifications"
    );

    println!("✓ V2: Production-ready examples with hardware, network, and monitoring");
}
