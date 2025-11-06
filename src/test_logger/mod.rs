use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

/// Represents a single test result within a benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Name of the test (e.g., "structure_validation")
    pub test_name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Optional score (e.g., "6/6", "93.8%", "4/5")
    pub score: Option<String>,
    /// Test duration in milliseconds
    pub duration_ms: u64,
    /// Additional details about the test
    pub details: String,
}

/// Represents a complete test run with all context and results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunResults {
    /// Specialty being tested (e.g., "mask-improver")
    pub specialty: String,
    /// Version of the mask being tested
    pub version: u32,
    /// When the benchmark was run
    pub timestamp: DateTime<Utc>,
    /// Git commit hash
    pub git_commit: String,
    /// Git tag (if any)
    pub git_tag: Option<String>,
    /// Git branch
    pub git_branch: Option<String>,
    /// Individual test results
    pub test_results: Vec<TestResult>,
    /// Overall score (if applicable)
    pub total_score: Option<f64>,
    /// Whether all tests passed
    pub passed: bool,
    /// Additional context/notes
    pub context: String,
}

/// Logger for capturing benchmark results during test execution
pub struct BenchmarkLogger {
    specialty: String,
    version: u32,
    test_results: Vec<TestResult>,
    start_time: Instant,
    context: String,
}

impl BenchmarkLogger {
    /// Create a new benchmark logger
    pub fn new(specialty: &str, version: u32) -> Self {
        Self {
            specialty: specialty.to_string(),
            version,
            test_results: Vec::new(),
            start_time: Instant::now(),
            context: String::new(),
        }
    }

    /// Add context/notes to this benchmark run
    pub fn add_context(&mut self, context: &str) {
        if !self.context.is_empty() {
            self.context.push('\n');
        }
        self.context.push_str(context);
    }

    /// Log a test result
    pub fn log_test(
        &mut self,
        test_name: &str,
        passed: bool,
        score: Option<&str>,
        details: &str,
    ) {
        let duration = self.start_time.elapsed();
        self.test_results.push(TestResult {
            test_name: test_name.to_string(),
            passed,
            score: score.map(String::from),
            duration_ms: duration.as_millis() as u64,
            details: details.to_string(),
        });
    }

    /// Calculate total score from all test results
    fn calculate_total_score(&self) -> Option<f64> {
        if self.test_results.is_empty() {
            return None;
        }

        let passed_count = self.test_results.iter().filter(|t| t.passed).count();
        let total_count = self.test_results.len();
        Some((passed_count as f64 / total_count as f64) * 100.0)
    }

    /// Get git commit hash
    fn get_git_commit() -> Result<String> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()?;

        if !output.status.success() {
            anyhow::bail!("Failed to get git commit");
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    /// Get git tag (if current commit has one)
    fn get_git_tag() -> Option<String> {
        let output = Command::new("git")
            .args(["describe", "--exact-match", "--tags", "HEAD"])
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
        } else {
            None
        }
    }

    /// Get git branch
    fn get_git_branch() -> Option<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
        } else {
            None
        }
    }

    /// Save results to disk (both JSON and markdown)
    pub fn save_results(&self) -> Result<()> {
        let git_commit = Self::get_git_commit().unwrap_or_else(|_| "unknown".to_string());
        let git_tag = Self::get_git_tag();
        let git_branch = Self::get_git_branch();
        let timestamp = Utc::now();
        let passed = self.test_results.iter().all(|t| t.passed);
        let total_score = self.calculate_total_score();

        let run = TestRunResults {
            specialty: self.specialty.clone(),
            version: self.version,
            timestamp,
            git_commit: git_commit.clone(),
            git_tag: git_tag.clone(),
            git_branch: git_branch.clone(),
            test_results: self.test_results.clone(),
            total_score,
            passed,
            context: self.context.clone(),
        };

        // Create directory structure
        let base_dir = PathBuf::from("results").join(&self.specialty);
        fs::create_dir_all(&base_dir)?;

        // Generate filename with timestamp and commit
        let timestamp_str = timestamp.format("%Y%m%d_%H%M%S");
        let commit_short = &git_commit.chars().take(8).collect::<String>();
        let filename = format!("{}_v{}_{}_{}", self.specialty, self.version, timestamp_str, commit_short);

        // Save JSON
        let json_path = base_dir.join(format!("{}.json", filename));
        let json_content = serde_json::to_string_pretty(&run)?;
        fs::write(&json_path, json_content)?;
        println!("\n📊 Benchmark results saved to: {}", json_path.display());

        // Save markdown report
        let md_path = base_dir.join(format!("{}.md", filename));
        let md_content = self.generate_markdown_report(&run);
        fs::write(&md_path, md_content)?;
        println!("📄 Report generated: {}", md_path.display());

        Ok(())
    }

    /// Generate a human-readable markdown report
    fn generate_markdown_report(&self, run: &TestRunResults) -> String {
        let mut report = String::new();

        // Header
        report.push_str(&format!("# Benchmark Results: {} v{}\n\n", run.specialty, run.version));

        // Context
        report.push_str("## Context\n\n");
        report.push_str(&format!("- **Timestamp:** {}\n", run.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("- **Git Commit:** `{}`\n", run.git_commit));
        if let Some(tag) = &run.git_tag {
            report.push_str(&format!("- **Git Tag:** `{}`\n", tag));
        }
        if let Some(branch) = &run.git_branch {
            report.push_str(&format!("- **Git Branch:** `{}`\n", branch));
        }
        report.push_str(&format!("- **Overall Status:** {}\n", if run.passed { "✅ PASS" } else { "❌ FAIL" }));
        if let Some(score) = run.total_score {
            report.push_str(&format!("- **Total Score:** {:.1}%\n", score));
        }
        report.push_str("\n");

        if !run.context.is_empty() {
            report.push_str("## Additional Context\n\n");
            report.push_str(&run.context);
            report.push_str("\n\n");
        }

        // Test Results
        report.push_str("## Test Results\n\n");

        let passed_count = run.test_results.iter().filter(|t| t.passed).count();
        let total_count = run.test_results.len();
        report.push_str(&format!("**Summary:** {}/{} tests passed\n\n", passed_count, total_count));

        for (i, test) in run.test_results.iter().enumerate() {
            report.push_str(&format!("### {}. {}\n\n", i + 1, test.test_name));
            report.push_str(&format!("- **Status:** {}\n", if test.passed { "✅ PASS" } else { "❌ FAIL" }));
            if let Some(score) = &test.score {
                report.push_str(&format!("- **Score:** {}\n", score));
            }
            report.push_str(&format!("- **Duration:** {}ms\n", test.duration_ms));
            if !test.details.is_empty() {
                report.push_str(&format!("\n**Details:**\n\n{}\n", test.details));
            }
            report.push_str("\n");
        }

        // Footer
        report.push_str("---\n\n");
        report.push_str("*Generated automatically by palace-skills test logging infrastructure*\n");

        report
    }
}

/// Helper function to create a results directory if it doesn't exist
pub fn ensure_results_dir() -> Result<()> {
    fs::create_dir_all("results")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = BenchmarkLogger::new("test-mask", 1);
        assert_eq!(logger.specialty, "test-mask");
        assert_eq!(logger.version, 1);
        assert!(logger.test_results.is_empty());
    }

    #[test]
    fn test_log_test() {
        let mut logger = BenchmarkLogger::new("test-mask", 1);
        logger.log_test("test1", true, Some("5/5"), "All checks passed");

        assert_eq!(logger.test_results.len(), 1);
        assert_eq!(logger.test_results[0].test_name, "test1");
        assert!(logger.test_results[0].passed);
        assert_eq!(logger.test_results[0].score, Some("5/5".to_string()));
    }

    #[test]
    fn test_calculate_total_score() {
        let mut logger = BenchmarkLogger::new("test-mask", 1);
        logger.log_test("test1", true, Some("5/5"), "");
        logger.log_test("test2", true, Some("3/3"), "");
        logger.log_test("test3", false, Some("0/2"), "");

        let score = logger.calculate_total_score();
        assert!(score.is_some());
        // 2 passed out of 3 = 66.67%
        let score_val = score.unwrap();
        assert!((score_val - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_add_context() {
        let mut logger = BenchmarkLogger::new("test-mask", 1);
        logger.add_context("First line");
        logger.add_context("Second line");

        assert!(logger.context.contains("First line"));
        assert!(logger.context.contains("Second line"));
    }
}
