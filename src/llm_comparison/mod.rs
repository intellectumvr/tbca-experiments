//! LLM comparison study (Tables 10-11)

pub mod batch_evaluator;
pub mod statistical_analysis;
pub mod llm_variants;
pub mod python_rag;
pub mod task_loader;
pub mod tbca_executor;
pub mod react_rag_executor;

use batch_evaluator::BatchEvaluator;
pub use llm_variants::LLMConfig;
use std::path::PathBuf;

pub fn run_comparison() {
    println!("Running REAL LLM comparison (TBCA vs ReAct+RAG)...");
    
    let results_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("results")
        .join("llm_comparison");
    
    println!("Results directory: {:?}", results_dir);
    
    let evaluator = BatchEvaluator::new(results_dir);
    let llms = LLMConfig::get_all();
    
    println!("Configured {} LLM variants", llms.len());
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    
    let results = rt.block_on(async {
        println!("Starting REAL batch evaluation with LLM calls...");
        evaluator.run_all_batches(&llms).await
    });
    
    match results {
        Ok(results) => {
            println!("Completed {} batch results", results.len());
            if let Err(e) = evaluator.save_results(&results) {
                eprintln!("Failed to save results: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to run batches: {}", e);
        }
    }
    
    println!("LLM comparison complete!");
}
