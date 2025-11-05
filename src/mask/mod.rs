pub mod loader;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use loader::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mask {
    pub id: Uuid,
    pub specialty: String,    // e.g., "distributed-systems"
    pub model: ModelProvider, // Sonnet, Haiku, OpenRouter, etc.
    pub version: u32,         // Iteration number
    pub content: String,      // Full markdown content
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub current_score: Option<f64>, // Latest benchmark score
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelProvider {
    ClaudeSonnet,
    ClaudeHaiku,
    OpenRouter { model_id: String },
    ZAi { model_id: String },
    Codex { model_id: String },
}

impl Mask {
    pub fn new(specialty: String, model: ModelProvider, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            specialty,
            model,
            version: 1,
            content,
            created_at: now,
            updated_at: now,
            current_score: None,
        }
    }

    pub fn update_content(&mut self, new_content: String) {
        self.content = new_content;
        self.version += 1;
        self.updated_at = Utc::now();
    }
}
