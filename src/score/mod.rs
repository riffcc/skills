use crate::ModelProvider;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRun {
    pub id: Uuid,
    pub mask_id: Uuid,
    pub benchmark_id: Uuid,
    pub model_provider: ModelProvider,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub transcript: String, // Full conversation log
    pub output: String,     // Final response
    pub improvement_suggestions: Option<ImprovementSuggestions>,
    pub evaluation: Option<Score>,
    pub git_commit_sha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestions {
    pub suggestions: Vec<String>, // What would make this better next time?
    pub rationale: String,        // Why these specific improvements?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub total: f64,                      // 0-10 scale
    pub breakdown: HashMap<String, f64>, // criterion -> score
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub evaluator_notes: String,
    pub evaluated_by: String, // "lief-unmasked" or model name
    pub evaluated_at: DateTime<Utc>,
    pub suggestions_quality: Option<f64>, // How good were the mask's suggestions? (0-10)
}

impl BenchmarkRun {
    pub fn new(mask_id: Uuid, benchmark_id: Uuid, model_provider: ModelProvider) -> Self {
        Self {
            id: Uuid::new_v4(),
            mask_id,
            benchmark_id,
            model_provider,
            started_at: Utc::now(),
            completed_at: None,
            transcript: String::new(),
            output: String::new(),
            improvement_suggestions: None,
            evaluation: None,
            git_commit_sha: None,
        }
    }

    pub fn complete(&mut self, output: String, transcript: String) {
        self.output = output;
        self.transcript = transcript;
        self.completed_at = Some(Utc::now());
    }
}
