//! Task definitions for multi-domain evaluation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub complexity: String,
    pub description: String,
    pub expected_outcome: String,
    pub reasoning_steps: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSet {
    pub domain: String,
    pub tasks: Vec<Task>,
}

impl TaskSet {
    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let task_set: TaskSet = serde_json::from_str(&content)?;
        Ok(task_set)
    }
}
