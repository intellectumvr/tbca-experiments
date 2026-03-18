//! ReAct+RAG baseline executor

use super::llm_variants::LLMConfig;
use super::task_loader::Task;
use super::python_rag::PythonRAG;

pub struct ReActRAGExecutor {
    llm: LLMConfig,
    rag: PythonRAG,
    max_iterations: usize,
}

impl ReActRAGExecutor {
    pub fn new(llm: LLMConfig) -> Self {
        let mut rag = PythonRAG::new();
        let _ = rag.add_document("Contract law: Payment within 30 days of delivery");
        let _ = rag.add_document("Delivery must occur by stated deadline");
        Self {
            llm,
            rag,
            max_iterations: 5,
        }
    }
    
    pub async fn execute_task(&mut self, task: &Task) -> (bool, usize) {
        for iter in 1..=self.max_iterations {
            let thought_prompt = format!(
                "Task: {}\n\nWhat is the answer? Be concise.",
                task.description
            );
            
            let thought = match self.llm.generate_hypothesis(&thought_prompt, "").await {
                Ok(t) => t,
                Err(_) => return (false, iter),
            };
            
            // Simple success check - if answer contains key terms from expected outcome
            let thought_lower = thought.to_lowercase();
            let outcome_lower = task.expected_outcome.to_lowercase();
            
            // Check for affirmative words or expected outcome keywords
            let success = thought_lower.contains("yes") 
                || thought_lower.contains("fulfilled")
                || thought_lower.contains("valid")
                || thought_lower.contains("approved")
                || thought_lower.contains("correct")
                || outcome_lower.split_whitespace()
                    .filter(|w| w.len() > 3)
                    .any(|word| thought_lower.contains(word));
            
            if success {
                return (true, iter);
            }
        }
        
        (false, self.max_iterations)
    }
}
