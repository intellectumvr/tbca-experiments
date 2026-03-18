//! Multi-domain evaluation (Table 9)

pub mod task_generator;
pub mod domain_evaluator;
pub mod task_definitions;

use domain_evaluator::DomainEvaluator;
use task_definitions::TaskSet;
use crate::llm_comparison::llm_variants::LLMConfig;
use std::path::PathBuf;

pub fn run_evaluation() {
    println!("Running multi-domain evaluation...");
    
    let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("scenarios");
    
    let results_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("results")
        .join("multi_domain");
    
    let evaluator = DomainEvaluator::new(results_dir);
    let llms = LLMConfig::get_all();
    
    // Use first LLM for evaluation
    let llm = &llms[0];
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async {
        let domains = vec!["legal", "economic", "spatial"];
        let mut all_results = Vec::new();
        
        for domain in domains {
            let path = scenarios_dir.join(format!("{}_tasks.json", domain));
            match TaskSet::load_from_file(path.to_str().unwrap()) {
                Ok(task_set) => {
                    let results = evaluator.evaluate_domain(&task_set, llm).await;
                    all_results.push(results);
                }
                Err(e) => {
                    eprintln!("Failed to load {}: {}", domain, e);
                }
            }
        }
        
        if let Err(e) = evaluator.save_results(&all_results) {
            eprintln!("Failed to save results: {}", e);
        }
    });
    
    println!("Multi-domain evaluation complete!");
}
