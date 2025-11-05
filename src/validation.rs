//! Validation functions for masks
//! Can be used by Mask Improver to validate masks during creation/improvement

use crate::*;

/// Result of mask validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub score: usize,
    pub max_score: usize,
    pub issues: Vec<String>,
}

impl ValidationResult {
    pub fn new(score: usize, max_score: usize, issues: Vec<String>) -> Self {
        let passed = issues.is_empty();
        Self {
            passed,
            score,
            max_score,
            issues,
        }
    }

    pub fn percentage(&self) -> f64 {
        if self.max_score == 0 {
            0.0
        } else {
            (self.score as f64 / self.max_score as f64) * 100.0
        }
    }
}

/// Validates a mask has all required structural sections
pub fn validate_structure(specialty: &str, model: &str) -> ValidationResult {
    let mask = match load_mask_from_file(specialty, model) {
        Ok(m) => m,
        Err(e) => return ValidationResult::new(0, 6, vec![format!("Failed to load mask: {}", e)]),
    };

    let required = vec![
        ("Identity", "## Identity"),
        ("Core Expertise", "## Core Expertise"),
        ("Your Mission", "## Your Mission"),
        ("Behavioral Guidelines", "## Behavioral Guidelines"),
        ("Examples", "## Examples"),
        ("Improvement Notes", "## Improvement Notes"),
    ];

    let mut issues = Vec::new();
    let mut score = 0;

    for (name, pattern) in &required {
        if mask.content.contains(pattern) {
            score += 1;
        } else {
            issues.push(format!("Missing section: {}", name));
        }
    }

    ValidationResult::new(score, required.len(), issues)
}

/// Validates mask uses real systems (not abstract examples)
pub fn validate_real_systems(specialty: &str, model: &str) -> ValidationResult {
    let mask = match load_mask_from_file(specialty, model) {
        Ok(m) => m,
        Err(e) => return ValidationResult::new(0, 1, vec![format!("Failed to load mask: {}", e)]),
    };

    // Common real systems by domain
    let real_systems = vec![
        "PostgreSQL",
        "MySQL",
        "MongoDB",
        "Redis",
        "etcd",
        "Consul",
        "HAProxy",
        "Nginx",
        "Kubernetes",
        "Docker",
        "MooseFS",
        "Ceph",
        "Kafka",
        "RabbitMQ",
        "Elasticsearch",
        "Prometheus",
        "Grafana",
        "Cassandra",
        "Patroni",
        "Raft",
        "Paxos",
    ];

    let found: Vec<String> = real_systems
        .iter()
        .filter(|&sys| mask.content.contains(sys))
        .map(|s| s.to_string())
        .collect();

    let mut issues = Vec::new();
    if found.is_empty() {
        issues.push("No real systems referenced - uses only abstract examples".to_string());
    }

    let score = if found.is_empty() { 0 } else { 1 };
    ValidationResult::new(score, 1, issues)
}

/// Validates mask includes trade-off analysis
pub fn validate_tradeoffs(specialty: &str, model: &str) -> ValidationResult {
    let mask = match load_mask_from_file(specialty, model) {
        Ok(m) => m,
        Err(e) => return ValidationResult::new(0, 1, vec![format!("Failed to load mask: {}", e)]),
    };

    let has_tradeoffs = mask.content.contains("Trade-off") || mask.content.contains("Trade-offs:");

    let mut issues = Vec::new();
    if !has_tradeoffs {
        issues.push("No trade-off analysis found".to_string());
    }

    let score = if has_tradeoffs { 1 } else { 0 };
    ValidationResult::new(score, 1, issues)
}

/// Validates mask has concrete examples (not just descriptions)
pub fn validate_examples(specialty: &str, model: &str) -> ValidationResult {
    let mask = match load_mask_from_file(specialty, model) {
        Ok(m) => m,
        Err(e) => return ValidationResult::new(0, 1, vec![format!("Failed to load mask: {}", e)]),
    };

    // Look for example markers
    let example_markers = vec![
        "## Examples",
        "### Example",
        "**Example",
        "**Problem:**",
        "**Solution:**",
    ];

    let has_examples = example_markers
        .iter()
        .any(|marker| mask.content.contains(marker));

    let mut issues = Vec::new();
    if !has_examples {
        issues.push("No concrete examples found".to_string());
    }

    let score = if has_examples { 1 } else { 0 };
    ValidationResult::new(score, 1, issues)
}

/// Comprehensive validation - runs all checks
pub fn validate_mask(specialty: &str, model: &str) -> Vec<ValidationResult> {
    vec![
        validate_structure(specialty, model),
        validate_real_systems(specialty, model),
        validate_tradeoffs(specialty, model),
        validate_examples(specialty, model),
    ]
}

/// Summary of validation results
pub fn validation_summary(results: &[ValidationResult]) -> String {
    let total_score: usize = results.iter().map(|r| r.score).sum();
    let total_max: usize = results.iter().map(|r| r.max_score).sum();
    let total_issues: usize = results.iter().map(|r| r.issues.len()).sum();

    format!(
        "Validation: {}/{} checks passed ({:.1}%), {} issues",
        total_score,
        total_max,
        (total_score as f64 / total_max as f64) * 100.0,
        total_issues
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_structure_distributed_systems() {
        let result = validate_structure("distributed-systems", "sonnet");
        println!("\nStructure validation:");
        println!("  Score: {}/{}", result.score, result.max_score);
        println!("  Passed: {}", result.passed);
        println!("  Issues: {:?}", result.issues);
        assert!(
            result.passed,
            "distributed-systems should have all required sections"
        );
        assert_eq!(result.score, 6);
    }

    #[test]
    fn test_validate_real_systems_distributed_systems() {
        let result = validate_real_systems("distributed-systems", "sonnet");
        println!("\nReal systems validation:");
        println!("  Score: {}/{}", result.score, result.max_score);
        println!("  Passed: {}", result.passed);
        assert!(
            result.passed,
            "distributed-systems should reference real systems"
        );
    }

    #[test]
    fn test_validate_tradeoffs_distributed_systems() {
        let result = validate_tradeoffs("distributed-systems", "sonnet");
        println!("\nTrade-offs validation:");
        println!("  Score: {}/{}", result.score, result.max_score);
        println!("  Passed: {}", result.passed);
        assert!(
            result.passed,
            "distributed-systems should include trade-off analysis"
        );
    }

    #[test]
    fn test_validate_examples_distributed_systems() {
        let result = validate_examples("distributed-systems", "sonnet");
        println!("\nExamples validation:");
        println!("  Score: {}/{}", result.score, result.max_score);
        println!("  Passed: {}", result.passed);
        assert!(
            result.passed,
            "distributed-systems should have concrete examples"
        );
    }

    #[test]
    fn test_comprehensive_validation() {
        let results = validate_mask("distributed-systems", "sonnet");
        let summary = validation_summary(&results);

        println!("\n{}", summary);
        for (i, result) in results.iter().enumerate() {
            println!(
                "  Check {}: {}/{} ({:.1}%)",
                i + 1,
                result.score,
                result.max_score,
                result.percentage()
            );
            for issue in &result.issues {
                println!("    - {}", issue);
            }
        }

        let all_passed = results.iter().all(|r| r.passed);
        assert!(
            all_passed,
            "All validation checks should pass for distributed-systems mask"
        );
    }

    #[test]
    fn test_validate_mask_improver() {
        let results = validate_mask("mask-improver", "sonnet");
        let summary = validation_summary(&results);

        println!("\n{}", summary);

        let all_passed = results.iter().all(|r| r.passed);
        assert!(
            all_passed,
            "All validation checks should pass for mask-improver"
        );
    }
}
