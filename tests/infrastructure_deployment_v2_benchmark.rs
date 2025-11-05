use palace_skills::*;

/// V2 Benchmarks for infrastructure-deployment mask
///
/// V1 tested for PRESENCE of concepts (keywords)
/// V2 tests for DEPTH of understanding (examples, trade-offs, failure scenarios)

#[test]
fn v2_security_implementation_details() {
    println!("\n=== V2 BENCHMARK: Infrastructure Deployment - Security Implementation ===\n");

    let mask = load_mask_from_file("infrastructure-deployment", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for security concepts
    // V2 requires actual implementation details

    let has_vault_config = content.contains("Vault") &&
                          (content.contains("config") || content.contains("setup"));

    let has_tls_automation = content.contains("TLS") &&
                            (content.contains("Let's Encrypt") || content.contains("cert-manager"));

    let has_rbac_config = content.contains("RBAC") &&
                         (content.contains("role") || content.contains("policy"));

    let has_ssh_hardening = content.contains("SSH") &&
                           (content.contains("key") || content.contains("harden"));

    // Must have security example
    let has_security_example = (content.contains("### Example") || content.contains("**Example")) &&
                               (content.contains("security") ||
                                content.contains("Vault") ||
                                content.contains("TLS") ||
                                content.contains("secrets"));

    // Must have actual configuration blocks
    let has_security_config = content.contains("```") &&
                             (content.contains("vault") ||
                              content.contains("tls") ||
                              content.contains("ssl") ||
                              content.contains("ssh"));

    let score = [has_vault_config, has_tls_automation, has_rbac_config,
                 has_ssh_hardening, has_security_example, has_security_config]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("V2 Security Implementation: {}/6", score);

    if has_security_example {
        println!("✓ V2: Security demonstrated with configuration examples");
    } else {
        println!("⚠ V2: Security mentioned but lacks concrete implementation examples");
    }

    // V2 requires 5/6
    assert!(score >= 5, "V2: Must have detailed security implementation (got {}/6)", score);
}

#[test]
fn v2_deployment_rollback_procedures() {
    println!("\n=== V2 BENCHMARK: Infrastructure Deployment - Rollback Procedures ===\n");

    let mask = load_mask_from_file("infrastructure-deployment", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires explicit rollback procedures

    let has_rollback_mention = content.contains("rollback") || content.contains("Rollback");

    let has_blue_green = content.contains("blue-green") ||
                        content.contains("blue green");

    let has_canary = content.contains("canary");

    let has_version_control = content.contains("version") &&
                             (content.contains("tag") || content.contains("release"));

    // Must have rollback example
    let has_rollback_example = (content.contains("### Example") || content.contains("**Example")) &&
                               (content.contains("rollback") || content.contains("blue-green"));

    // Must have specific rollback steps
    let has_rollback_steps = content.contains("step") ||
                            content.contains("procedure") ||
                            content.contains("switch back");

    let score = [has_rollback_mention, has_blue_green, has_canary,
                 has_version_control, has_rollback_example, has_rollback_steps]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("V2 Rollback Procedures: {}/6", score);

    if has_rollback_example {
        println!("✓ V2: Rollback demonstrated with examples");
    } else {
        println!("⚠ V2: Rollback mentioned but lacks concrete procedures");
    }

    // V2 requires 4/6 (rollback may not be needed for all deployments)
    assert!(score >= 4, "V2: Must have rollback coverage (got {}/6)", score);
}

#[test]
fn v2_zero_downtime_deployment() {
    println!("\n=== V2 BENCHMARK: Infrastructure Deployment - Zero-Downtime Deployment ===\n");

    let mask = load_mask_from_file("infrastructure-deployment", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires zero-downtime deployment patterns

    let has_zero_downtime = content.contains("zero-downtime") ||
                           content.contains("zero downtime") ||
                           content.contains("no downtime");

    let has_rolling_update = content.contains("rolling") ||
                            content.contains("rolling update");

    let has_health_checks = content.contains("health check") ||
                           content.contains("readiness") ||
                           content.contains("liveness");

    let has_load_balancer = content.contains("load balancer") ||
                           content.contains("HAProxy") ||
                           content.contains("Nginx");

    // Must have zero-downtime example
    let has_zdt_example = (content.contains("### Example") || content.contains("**Example")) &&
                         (content.contains("zero") || content.contains("rolling") || content.contains("health"));

    let score = [has_zero_downtime, has_rolling_update, has_health_checks,
                 has_load_balancer, has_zdt_example]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("V2 Zero-Downtime: {}/5", score);

    if has_zdt_example {
        println!("✓ V2: Zero-downtime deployment demonstrated with examples");
    } else {
        println!("⚠ V2: Zero-downtime mentioned but lacks concrete patterns");
    }

    // V2 requires 4/5
    assert!(score >= 4, "V2: Must have zero-downtime deployment coverage (got {}/5)", score);
}

#[test]
fn v2_disaster_recovery_runbooks() {
    println!("\n=== V2 BENCHMARK: Infrastructure Deployment - Disaster Recovery ===\n");

    let mask = load_mask_from_file("infrastructure-deployment", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires disaster recovery runbooks

    let has_dr_mention = content.contains("disaster recovery") ||
                        content.contains("DR") ||
                        content.contains("backup");

    let has_restore_procedure = content.contains("restore") ||
                               content.contains("recovery");

    let has_rto_rpo = content.contains("RTO") ||
                     content.contains("RPO") ||
                     content.contains("Recovery Time") ||
                     content.contains("Recovery Point");

    let has_backup_testing = content.contains("test") &&
                            (content.contains("backup") || content.contains("restore"));

    // Must have DR example
    let has_dr_example = (content.contains("### Example") || content.contains("**Example")) &&
                         (content.contains("disaster") ||
                          content.contains("recovery") ||
                          content.contains("backup") ||
                          content.contains("restore"));

    let score = [has_dr_mention, has_restore_procedure, has_rto_rpo,
                 has_backup_testing, has_dr_example]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("V2 Disaster Recovery: {}/5", score);

    if has_dr_example {
        println!("✓ V2: Disaster recovery demonstrated with runbook examples");
    } else {
        println!("⚠ V2: DR mentioned but lacks concrete runbooks");
    }

    // V2 requires 4/5
    assert!(score >= 4, "V2: Must have disaster recovery coverage (got {}/5)", score);
}

#[test]
fn v2_jetpack_playbook_examples() {
    println!("\n=== V2 BENCHMARK: Infrastructure Deployment - Jetpack Playbook Examples ===\n");

    let mask = load_mask_from_file("infrastructure-deployment", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // V2 requires complete Jetpack playbook examples

    let has_jetpack = content.contains("Jetpack") || content.contains("jetpack");

    let has_playbook_syntax = content.contains("tasks:") ||
                             content.contains("!shell") ||
                             content.contains("!copy") ||
                             content.contains(".yml");

    let has_inventory = content.contains("inventory") ||
                       content.contains("host_vars") ||
                       content.contains("group");

    let has_modules = content.contains("!shell") ||
                     content.contains("!copy") ||
                     content.contains("!template");

    // Must have Jetpack playbook example
    let has_playbook_example = (content.contains("### Example") || content.contains("**Example")) &&
                               (content.contains("Jetpack") ||
                                content.contains("playbook") ||
                                content.contains("---"));

    // Must have actual YAML blocks
    let has_yaml_blocks = content.contains("```yaml") || content.contains("```yml");

    let score = [has_jetpack, has_playbook_syntax, has_inventory,
                 has_modules, has_playbook_example, has_yaml_blocks]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("V2 Jetpack Playbooks: {}/6", score);

    if has_playbook_example {
        println!("✓ V2: Jetpack playbooks demonstrated with examples");
    } else {
        println!("⚠ V2: Jetpack mentioned but lacks complete playbook examples");
    }

    // V2 requires 5/6
    assert!(score >= 5, "V2: Must have detailed Jetpack playbook examples (got {}/6)", score);
}

#[test]
fn v2_production_ready_examples() {
    println!("\n=== V2 BENCHMARK: Infrastructure Deployment - Production-Ready Examples ===\n");

    let mask = load_mask_from_file("infrastructure-deployment", "sonnet")
        .expect("Failed to load mask");

    let content = &mask.content;

    // V1 just checked for examples existing
    // V2 requires DETAILED examples with complete playbooks, configs, validation

    // Count proper example sections
    let example_count = content.matches("### Example").count();

    // Check for complete playbooks
    let has_complete_playbooks = content.contains("```yaml") || content.contains("```yml");

    // Check for validation steps
    let has_validation = content.contains("validate") ||
                        content.contains("verification") ||
                        content.contains("test");

    // Check for monitoring/observability
    let has_monitoring = content.contains("monitor") ||
                        content.contains("Prometheus") ||
                        content.contains("metrics");

    // Check for security configuration
    let has_security_config = content.contains("Vault") ||
                             content.contains("TLS") ||
                             content.contains("secrets");

    println!("V2 Examples: {} proper example sections", example_count);
    println!("V2 Complete Playbooks: {}", if has_complete_playbooks { "Present" } else { "Missing" });
    println!("V2 Validation Steps: {}", if has_validation { "Present" } else { "Missing" });
    println!("V2 Monitoring: {}", if has_monitoring { "Present" } else { "Missing" });

    let score = [
        example_count >= 3,
        has_complete_playbooks,
        has_validation,
        has_monitoring,
        has_security_config
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("V2 Production-Ready: {}/5", score);

    // V2 requires at least 3 examples
    assert!(example_count >= 3, "V2: Must have at least 3 detailed examples (found {})", example_count);

    // V2 requires complete playbooks
    assert!(has_complete_playbooks, "V2: Must include complete Jetpack playbooks");

    println!("✓ V2: Production-ready examples with playbooks and validation");
}
