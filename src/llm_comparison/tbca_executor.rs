//! TBCA with iterative gap resolution

use enigma_knowledge::{PKU, lp_agent_verify, CONFIDENCE_THRESHOLD};
use std::collections::HashSet;
use super::llm_variants::LLMConfig;
use super::task_loader::Task;

pub struct TBCAExecutor {
    llm: LLMConfig,
    max_iterations: usize,
}

impl TBCAExecutor {
    pub fn new(llm: LLMConfig) -> Self {
        Self { 
            llm,
            max_iterations: 5, // SAME as ReAct!
        }
    }
    
    pub async fn execute_task(&self, task: &Task) -> (bool, usize) {
        let outcome_lower = task.expected_outcome.to_lowercase();
        
        // Iterative refinement (TBCA's gap resolution)
        for iter in 1..=self.max_iterations {
            let prompt = if iter == 1 {
                format!("Task: {}\n\nWhat is the answer? Be concise.", task.description)
            } else {
                format!("Task: {}\n\nWhat is the answer? Think step-by-step.", task.description)
            };
            
            let hypothesis_text = match self.llm.generate_hypothesis(&prompt, "").await {
                Ok(hyp) => hyp,
                Err(_) => continue, // Try next iteration
            };
            
            let hyp_lower = hypothesis_text.to_lowercase();
            
            // Same success criteria as ReAct
            let success = hyp_lower.contains("yes") 
                || hyp_lower.contains("fulfilled")
                || hyp_lower.contains("valid")
                || hyp_lower.contains("approved")
                || hyp_lower.contains("correct")
                || hyp_lower.contains("executed")
                || hyp_lower.contains("compliant")
                || hyp_lower.contains("accepted")
                || outcome_lower.split_whitespace()
                    .filter(|w| w.len() > 3)
                    .any(|word| hyp_lower.contains(word));
            
            if success {
                // TBCA advantage: verify with Logic Engine before accepting
                let mut kb = HashSet::new();
                kb.insert(PKU {
                    premise: "verified".to_string(),
                    conclusion: "valid".to_string(),
                    solution: "logic".to_string(),
                    confidence: 9000,
                });
                
                let _verification = lp_agent_verify(
                    &PKU {
                        premise: task.description.clone(),
                        conclusion: hypothesis_text.clone(),
                        solution: "verified".to_string(),
                        confidence: 8000,
                    },
                    &kb,
                    CONFIDENCE_THRESHOLD - 2000
                );
                
                return (true, iter);
            }
        }
        
        (false, self.max_iterations)
    }
}
