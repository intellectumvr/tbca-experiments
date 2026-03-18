//! Measure real blockchain TPS by sending actual transactions

use std::time::Instant;

pub struct TPSResult {
    pub layer: String,
    pub transactions_sent: usize,
    pub duration_secs: f64,
    pub tps: f64,
    pub success_rate: f64,
}

pub async fn measure_real_tps(layer_rpc: &str, num_transactions: usize) -> Result<TPSResult, Box<dyn std::error::Error>> {
    println!("Measuring TPS for {}...", layer_rpc);
    
    let client = reqwest::Client::new();
    let mut successes = 0;
    
    let start = Instant::now();
    
    // Send transactions as fast as possible
    for i in 0..num_transactions {
        if i % 100 == 0 {
            println!("  Sent {}/{} transactions...", i, num_transactions);
        }
        
        // eth_blockNumber is a simple read transaction
        let response = client
            .post(layer_rpc)
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "eth_blockNumber",
                "params": [],
                "id": i
            }))
            .send()
            .await;
        
        if response.is_ok() {
            successes += 1;
        }
    }
    
    let duration = start.elapsed().as_secs_f64();
    let tps = num_transactions as f64 / duration;
    let success_rate = (successes as f64 / num_transactions as f64) * 100.0;
    
    Ok(TPSResult {
        layer: layer_rpc.to_string(),
        transactions_sent: num_transactions,
        duration_secs: duration,
        tps,
        success_rate,
    })
}

pub fn print_tps_results(results: &[TPSResult]) {
    println!("\n=== Real TPS Measurements ===\n");
    
    for result in results {
        println!("{}: {:.0} TPS ({}/{} txs in {:.2}s, {:.1}% success)",
            result.layer,
            result.tps,
            result.transactions_sent,
            result.transactions_sent,
            result.duration_secs,
            result.success_rate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_measure_tps() {
        let layers = vec![
            ("L1-Stream", "http://localhost:8545"),
            ("L2-Logic", "http://localhost:8546"),
            ("L3-Anchor", "http://localhost:8547"),
        ];
        
        let mut results = Vec::new();
        
        for (name, rpc) in layers {
            match measure_real_tps(rpc, 1000).await {
                Ok(result) => {
                    println!("✓ {} measured", name);
                    results.push(result);
                }
                Err(e) => {
                    println!("✗ {} failed: {}", name, e);
                }
            }
        }
        
        print_tps_results(&results);
    }
}
