//! Load tasks from scenario JSON files

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub complexity: String,
    pub description: String,
    pub expected_outcome: String,
    pub reasoning_steps: usize,
}

#[derive(Debug, Deserialize)]
struct ScenarioFile {
    domain: String,
    tasks: Vec<Task>,
}

pub struct TaskLoader {
    scenarios_dir: PathBuf,
}

impl TaskLoader {
    pub fn new() -> Self {
        let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("scenarios");
        
        Self { scenarios_dir }
    }
    
    pub fn load_all_tasks(&self) -> Result<Vec<Task>> {
        let mut all_tasks = Vec::new();
        
        for scenario_file in &["legal_tasks.json", "economic_tasks.json", "spatial_tasks.json"] {
            let path = self.scenarios_dir.join(scenario_file);
            
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let scenario: ScenarioFile = serde_json::from_str(&content)?;
                all_tasks.extend(scenario.tasks);
            }
        }
        
        Ok(all_tasks)
    }
    
    pub fn get_batch(&self, batch_num: usize, batch_size: usize) -> Result<Vec<Task>> {
        let all_tasks = self.load_all_tasks()?;
        
        let start = (batch_num - 1) * batch_size;
        let end = start + batch_size;
        
        Ok(all_tasks.into_iter()
            .skip(start)
            .take(batch_size)
            .collect())
    }
}
