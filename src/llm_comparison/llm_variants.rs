//! LLM variant configurations using existing Ollama setup

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub name: String,
    pub model: String,
    pub endpoint: String,
    pub temperature: f32,
}

impl LLMConfig {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self {
                name: "1.5B".to_string(),
                model: "deepseek-r1:1.5b".to_string(),
                endpoint: "http://localhost:11434".to_string(),
                temperature: 0.4,
            },
            Self {
                name: "3.8B".to_string(),
                model: "phi3:mini".to_string(),
                endpoint: "http://localhost:11434".to_string(),
                temperature: 0.5,
            },
            Self {
                name: "8B".to_string(),
                model: "llama3.1:8b".to_string(),
                endpoint: "http://localhost:11434".to_string(),
                temperature: 0.7,
            },
        ]
    }
    
    pub async fn check_available(&self) -> bool {
        let client = reqwest::Client::new();
        let url = format!("{}/api/tags", self.endpoint);
        
        match client.get(&url).timeout(std::time::Duration::from_secs(60)).send().await {
            Ok(resp) => {
                if let Ok(data) = resp.json::<serde_json::Value>().await {
                    if let Some(models) = data.get("models").and_then(|m| m.as_array()) {
                        return models.iter().any(|m| {
                            m.get("name")
                                .and_then(|n| n.as_str())
                                .map(|n| n.contains(&self.model))
                                .unwrap_or(false)
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to check Ollama: {}", e);
                return false;
            }
        }
        false
    }
    
    pub async fn generate_hypothesis(
        &self,
        task: &str,
        context: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct OllamaRequest {
            model: String,
            prompt: String,
            temperature: f32,
            stream: bool,
        }
        
        #[derive(Deserialize)]
        struct OllamaResponse {
            response: String,
        }
        
        let prompt = format!(
            "Task: {}\nContext: {}\n\nProvide a clear solution and reasoning.",
            task, context
        );
        
        let client = reqwest::Client::new();
        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            temperature: self.temperature,
            stream: false,
        };
        
        let response = client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&request)
            .timeout(std::time::Duration::from_secs(60)).send()
            .await?;
        
        let result: OllamaResponse = response.json().await?;
        Ok(result.response)
    }
}
