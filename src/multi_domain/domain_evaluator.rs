//! Domain evaluation logic with REAL executors

use super::task_definitions::{Task, TaskSet};
use crate::llm_comparison::tbca_executor::TBCAExecutor;
use crate::llm_comparison::react_rag_executor::ReActRAGExecutor;
use crate::llm_comparison::llm_variants::LLMConfig;
use crate::llm_comparison::task_loader::Task as ComparisonTask;
use std::path::PathBuf;
use anyhow::Result;

pub struct DomainEvaluator {
    results_dir: PathBuf,
}

impl DomainEvaluator {
    pub fn new(results_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&results_dir).ok();
        Self { results_dir }
    }
    
    pub async fn evaluate_task(
        &self, 
        task: &Task, 
        llm: &LLMConfig
    ) -> TaskResult {
        println!("  Evaluating: {} ({})", task.id, task.complexity);
        
        // Convert to comparison task format
        let comp_task = ComparisonTask {
            id: task.id.clone(),
            complexity: task.complexity.clone(),
            description: task.description.clone(),
            expected_outcome: task.expected_outcome.clone(),
            reasoning_steps: task.reasoning_steps,
        };
        
        // Run TBCA
        let tbca_executor = TBCAExecutor::new(llm.clone());
        let (tbca_success, tbca_iterations) = tbca_executor.execute_task(&comp_task).await;
        
        // Run ReAct
        let mut react_executor = ReActRAGExecutor::new(llm.clone());
        let (react_success, react_iterations) = react_executor.execute_task(&comp_task).await;
        
        TaskResult {
            task_id: task.id.clone(),
            complexity: task.complexity.clone(),
            tbca_success,
            tbca_iterations,
            react_success,
            react_iterations,
        }
    }
    
    pub async fn evaluate_domain(
        &self, 
        task_set: &TaskSet,
        llm: &LLMConfig
    ) -> DomainResults {
        println!("\nEvaluating domain: {} with {}", task_set.domain, llm.name);
        
        let mut results = DomainResults {
            domain: task_set.domain.clone(),
            task_results: Vec::new(),
        };
        
        for task in &task_set.tasks {
            let result = self.evaluate_task(task, llm).await;
            results.task_results.push(result);
        }
        
        results
    }
    
    pub fn save_results(&self, results: &[DomainResults]) -> Result<()> {
        let output_path = self.results_dir.join("table9_domain_performance.csv");
        let mut wtr = csv::Writer::from_path(&output_path)?;
        
        wtr.write_record(&[
            "Domain", "Complexity", "TBCA_SR", "TBCA_Iterations", 
            "ReAct_SR", "ReAct_Iterations"
        ])?;
        
        for domain_result in results {
            for task_result in &domain_result.task_results {
                wtr.write_record(&[
                    &domain_result.domain,
                    &task_result.complexity,
                    &format!("{}", task_result.tbca_success as u8),
                    &format!("{}", task_result.tbca_iterations),
                    &format!("{}", task_result.react_success as u8),
                    &format!("{}", task_result.react_iterations),
                ])?;
            }
        }
        
        wtr.flush()?;
        println!("\n✓ Results saved to: {:?}", output_path);
        Ok(())
    }
}

#[derive(Debug)]
pub struct TaskResult {
    pub task_id: String,
    pub complexity: String,
    pub tbca_success: bool,
    pub tbca_iterations: usize,
    pub react_success: bool,
    pub react_iterations: usize,
}

#[derive(Debug)]
pub struct DomainResults {
    pub domain: String,
    pub task_results: Vec<TaskResult>,
}
