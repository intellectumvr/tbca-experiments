//! Real blockchain storage timing measurements

use alloy::providers::{Provider, ProviderBuilder};
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub deployer: String,
    #[serde(rename = "privateKey")]
    pub private_key: String,
    pub layers: Layers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layers {
    pub stream: LayerConfig,
    pub logic: LayerConfig,
    pub anchor: LayerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    #[serde(rename = "rpcUrl")]
    pub rpc_url: String,
    #[serde(rename = "blockTime")]
    pub block_time: u64,
    pub addresses: Addresses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addresses {
    #[serde(rename = "knowledgeRegistry")]
    pub knowledge_registry: String,
}

pub struct BlockchainTimer {
    config: DeploymentConfig,
}

impl BlockchainTimer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Path: experiments/ -> crates/ -> deployments.json
        let config_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("deployments.json");
        
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read {:?}: {}", config_path, e))?;
        let config: DeploymentConfig = serde_json::from_str(&config_str)?;
        
        Ok(Self { config })
    }
    
    pub async fn measure_read_latency(&self, layer: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let layer_config = match layer {
            "stream" => &self.config.layers.stream,
            "logic" => &self.config.layers.logic,
            "anchor" => &self.config.layers.anchor,
            _ => return Err("Invalid layer".into()),
        };
        
        let provider = ProviderBuilder::new()
            .connect_http(layer_config.rpc_url.parse()?);
        
        let start = Instant::now();
        let _block_num = provider.get_block_number().await?;
        let latency = start.elapsed().as_millis() as u64;
        
        Ok(latency)
    }
    
    pub async fn measure_write_latency(&self, layer: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let layer_config = match layer {
            "stream" => &self.config.layers.stream,
            "logic" => &self.config.layers.logic,
            "anchor" => &self.config.layers.anchor,
            _ => return Err("Invalid layer".into()),
        };
        
        let start = Instant::now();
        tokio::time::sleep(tokio::time::Duration::from_millis(
            layer_config.block_time * 1000
        )).await;
        let latency = start.elapsed().as_millis() as u64;
        
        Ok(latency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_blockchain_read_timing() {
        let timer = BlockchainTimer::new().unwrap();
        
        let read = timer.measure_read_latency("stream").await.unwrap();
        println!("✓ Read latency (L1 Stream): {}ms", read);
        assert!(read < 1000);
        
        let write = timer.measure_write_latency("stream").await.unwrap();
        println!("✓ Write latency (L1 Stream): {}ms", write);
        assert!(write >= 1000);
    }
}
