use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmark {
    pub id: Uuid,
    pub specialty: String,
    pub name: String,
    pub difficulty: Difficulty,
    pub scenario: String,
    pub evaluation_criteria: HashMap<String, f64>, // criterion -> weight
    pub required_elements: Vec<String>,
    pub optional_elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl Benchmark {
    pub fn new(
        specialty: String,
        name: String,
        difficulty: Difficulty,
        scenario: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            specialty,
            name,
            difficulty,
            scenario,
            evaluation_criteria: HashMap::new(),
            required_elements: Vec::new(),
            optional_elements: Vec::new(),
        }
    }
}
