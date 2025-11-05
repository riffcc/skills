use palace_skills::*;

#[test]
fn benchmark_infrastructure_deployment_structure() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - Structure Validation ===\n");

    let mask = load_mask_from_file("infrastructure-deployment", "sonnet")
        .expect("Failed to load infrastructure-deployment mask");

    let content = &mask.content;

    // Core sections
    let has_identity = content.contains("## Identity");
    let has_expertise = content.contains("## Core Expertise");
    let has_mission = content.contains("## Your Mission");
    let has_guidelines = content.contains("## Behavioral Guidelines");
    let has_examples = content.contains("## Examples");

    assert!(has_identity, "Mask must have Identity section");
    assert!(has_expertise, "Mask must have Core Expertise section");
    assert!(has_mission, "Mask must have Your Mission section");
    assert!(
        has_guidelines,
        "Mask must have Behavioral Guidelines section"
    );
    assert!(has_examples, "Mask must have Examples section");

    println!("✓ Structure: 5/5 required sections present");
}

#[test]
fn benchmark_infrastructure_deployment_iac() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - Infrastructure as Code ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check IaC coverage
    let has_iac = content.contains("Infrastructure as Code") || content.contains("IaC");
    let has_terraform = content.contains("Terraform");
    let has_ansible = content.contains("Ansible");
    let has_declarative = content.contains("declarative");
    let has_idempotent = content.contains("idempotent");

    assert!(
        has_iac || has_terraform || has_ansible,
        "Mask should cover Infrastructure as Code"
    );

    let score = [
        has_iac,
        has_terraform,
        has_ansible,
        has_declarative,
        has_idempotent,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    println!("✓ IaC: {}/5 concepts covered", score);
}

#[test]
fn benchmark_infrastructure_deployment_containers() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - Container Platforms ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check container coverage
    let has_docker = content.contains("Docker");
    let has_kubernetes = content.contains("Kubernetes") || content.contains("k8s");
    let has_containers = content.contains("container") || content.contains("Container");
    let has_compose = content.contains("docker-compose") || content.contains("Compose");

    assert!(
        has_containers || has_docker || has_kubernetes,
        "Mask should cover containerization"
    );

    let score = [has_docker, has_kubernetes, has_containers, has_compose]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("✓ Containers: {}/4 technologies covered", score);
}

#[test]
fn benchmark_infrastructure_deployment_cicd() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - CI/CD Pipelines ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check CI/CD coverage
    let has_cicd = content.contains("CI/CD") || content.contains("CI") || content.contains("CD");
    let has_pipeline = content.contains("pipeline") || content.contains("Pipeline");
    let has_github_actions = content.contains("GitHub Actions");
    let has_gitlab = content.contains("GitLab CI");
    let has_automation = content.contains("automation") || content.contains("Automation");

    let score = [
        has_cicd,
        has_pipeline,
        has_github_actions,
        has_gitlab,
        has_automation,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(score >= 2, "Mask should cover CI/CD (found {}/5)", score);

    println!("✓ CI/CD: {}/5 concepts covered", score);
}

#[test]
fn benchmark_infrastructure_deployment_monitoring() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - Monitoring & Observability ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check monitoring coverage
    let has_monitoring = content.contains("monitoring") || content.contains("Monitoring");
    let has_prometheus = content.contains("Prometheus");
    let has_grafana = content.contains("Grafana");
    let has_logging = content.contains("logging") || content.contains("logs");
    let has_alerting = content.contains("alert") || content.contains("Alert");

    let score = [
        has_monitoring,
        has_prometheus,
        has_grafana,
        has_logging,
        has_alerting,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        score >= 2,
        "Mask should cover monitoring/observability (found {}/5)",
        score
    );

    println!("✓ Monitoring: {}/5 components covered", score);
}

#[test]
fn benchmark_infrastructure_deployment_security() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - Security Best Practices ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check security coverage
    let has_security = content.contains("security") || content.contains("Security");
    let has_secrets = content.contains("secrets") || content.contains("Secrets");
    let has_tls = content.contains("TLS") || content.contains("SSL");
    let has_rbac = content.contains("RBAC") || content.contains("access control");
    let has_vault = content.contains("Vault") || content.contains("secrets management");

    let score = [has_security, has_secrets, has_tls, has_rbac, has_vault]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(score >= 2, "Mask should cover security (found {}/5)", score);

    println!("✓ Security: {}/5 practices covered", score);
}

#[test]
fn benchmark_infrastructure_deployment_ha() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - High Availability ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check HA coverage
    let has_ha = content.contains("High Availability")
        || content.contains("high availability")
        || content.contains("HA");
    let has_loadbalancing = content.contains("load balanc") || content.contains("Load balanc");
    let has_failover = content.contains("failover");
    let has_redundancy = content.contains("redundancy") || content.contains("redundant");
    let has_clustering = content.contains("cluster") || content.contains("Cluster");

    let score = [
        has_ha,
        has_loadbalancing,
        has_failover,
        has_redundancy,
        has_clustering,
    ]
    .iter()
    .filter(|&&x| x)
    .count();

    assert!(
        score >= 2,
        "Mask should cover high availability (found {}/5)",
        score
    );

    println!("✓ High Availability: {}/5 concepts covered", score);
}

#[test]
fn benchmark_infrastructure_deployment_proxmox() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - Proxmox/LXC Coverage ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for Proxmox/LXC (specific to our environment)
    let has_proxmox = content.contains("Proxmox");
    let has_lxc = content.contains("LXC");
    let has_virtualization = content.contains("virtualization")
        || content.contains("VM")
        || content.contains("virtual machine");

    let score = [has_proxmox, has_lxc, has_virtualization]
        .iter()
        .filter(|&&x| x)
        .count();

    if score > 0 {
        println!("✓ Proxmox/LXC: {}/3 concepts covered", score);
    } else {
        println!("⚠ Proxmox/LXC: Not covered (may need improvement for our environment)");
    }
}

#[test]
fn benchmark_infrastructure_deployment_rust_tdd() {
    println!("\n=== BENCHMARK: Infrastructure Deployment - Rust + TDD Best Practices ===\n");

    let mask =
        load_mask_from_file("infrastructure-deployment", "sonnet").expect("Failed to load mask");

    let content = &mask.content;

    // Check for Rust + TDD references
    let has_rust = content.contains("Rust");
    let has_tdd = content.contains("TDD") || content.contains("Test-Driven Development");
    let has_testing = content.contains("test") || content.contains("Test");
    let has_precommit = content.contains("pre-commit");

    let score = [has_rust, has_tdd, has_testing, has_precommit]
        .iter()
        .filter(|&&x| x)
        .count();

    if score > 0 {
        println!("✓ Rust+TDD: {}/4 best practices covered", score);
    } else {
        println!("⚠ Rust+TDD: Not covered (may need improvement for tooling development)");
    }
}
