// crates/experiments/src/llm_comparison/react_engine.rs
pub struct ReActEngine {
    llm_client: MultiLLMClient,
    max_iterations: usize,
}

impl ReActEngine {
    pub async fn solve_task(&self, task: &str) -> (bool, usize) {
        let mut context = String::new();
        
        for iter in 1..=self.max_iterations {
            // ReAct pattern: Thought → Action → Observation
            let prompt = format!(
                "Task: {}\nContext: {}\n\nThought: ",
                task, context
            );
            
            let thought = self.llm_client.query(&prompt).await?;
            
            let action_prompt = format!("{}\nAction: ", thought);
            let action = self.llm_client.query(&action_prompt).await?;
            
            // Execute action (simplified - just evaluate)
            let observation = self.execute_action(&action, task);
            
            context.push_str(&format!(
                "Iter {}:\nThought: {}\nAction: {}\nObs: {}\n",
                iter, thought, action, observation
            ));
            
            if self.is_solved(&observation) {
                return (true, iter);
            }
        }
        
        (false, self.max_iterations)
    }
}