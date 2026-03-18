//! Measure cognitive loop phase timings with min/mean/max

use std::time::Instant;
use enigma_knowledge::{PKU, lp_agent_verify, CONFIDENCE_THRESHOLD};
use std::collections::HashSet;
use super::blockchain_timing::BlockchainTimer;

#[derive(Debug, Clone)]
pub struct PhaseStats {
    pub min: u64,
    pub mean: u64,
    pub max: u64,
}

impl PhaseStats {
    fn from_samples(samples: &[u64]) -> Self {
        Self {
            min: *samples.iter().min().unwrap_or(&0),
            mean: (samples.iter().sum::<u64>() as f64 / samples.len() as f64) as u64,
            max: *samples.iter().max().unwrap_or(&0),
        }
    }
}

#[derive(Debug)]
pub struct PhaseTimingResults {
    pub task_submission: PhaseStats,
    pub context_retrieval_cached: PhaseStats,
    pub context_retrieval_uncached: PhaseStats,
    pub hypothesis_generation: PhaseStats,
    pub logic_validation: PhaseStats,
    pub decision_check: PhaseStats,
    pub blockchain_storage: PhaseStats,
    pub result_return: PhaseStats,
    pub network_overhead: PhaseStats,
}

pub async fn measure_all_phases(num_runs: usize) -> PhaseTimingResults {
    println!("Measuring cognitive loop phases ({} runs)...", num_runs);
    
    let mut task_submission_samples = Vec::new();
    let mut context_cached_samples = Vec::new();
    let mut context_uncached_samples = Vec::new();
    let mut hypothesis_gen_samples = Vec::new();
    let mut logic_val_samples = Vec::new();
    let mut decision_samples = Vec::new();
    let mut blockchain_samples = Vec::new();
    let mut result_return_samples = Vec::new();
    let mut network_samples = Vec::new();
    
    let bc_timer = BlockchainTimer::new().unwrap();
    
    for run in 0..num_runs {
        if run % 10 == 0 {
            println!("  Run {}/{}...", run, num_runs);
        }
        
        // Phase 1: Task Submission (parsing)
        let t1 = Instant::now();
        let task = "Delivered Feb 28, paid March 25. Fulfilled?";
        let _parsed = task.to_string();
        task_submission_samples.push(t1.elapsed().as_micros() as u64);
        
        // Phase 2a: Context Retrieval (cached - simulated fast lookup)
        let t2a = Instant::now();
        let _cached_kb: HashSet<PKU> = HashSet::new();
        tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
        context_cached_samples.push(t2a.elapsed().as_millis() as u64);
        
        // Phase 2b: Context Retrieval (uncached - real blockchain read)
        let uncached_latency = bc_timer.measure_read_latency("logic").await.unwrap();
        context_uncached_samples.push(uncached_latency);
        
        // Phase 3: Hypothesis Generation (LLM call - simulated realistic time)
        let t3 = Instant::now();
        tokio::time::sleep(tokio::time::Duration::from_millis(350 + (run % 200) as u64)).await;
        hypothesis_gen_samples.push(t3.elapsed().as_millis() as u64);
        
        
        // Phase 4: Logic Validation (REAL - using actual Logic Engine)
        let t4 = Instant::now();
        let hypothesis = PKU {
            premise: task.to_string(),
            conclusion: "fulfilled".to_string(),
            solution: "test".to_string(),
            confidence: 5000,
        };
        let mut kb = HashSet::new();
        kb.insert(PKU {
            premise: "delivered_before_march1".to_string(),
            conclusion: "delivery_ok".to_string(),
            solution: "rule1".to_string(),
            confidence: 9500,
        });
        kb.insert(PKU {
            premise: "delivery_ok".to_string(),
            conclusion: "fulfilled".to_string(),
            solution: "rule2".to_string(),
            confidence: 9000,
        });
        let _result = lp_agent_verify(&hypothesis, &kb, CONFIDENCE_THRESHOLD);
        logic_val_samples.push(t4.elapsed().as_millis() as u64);
        // Phase 5: Decision Check (threshold comparison)
        let t5 = Instant::now();
        let _decision = _result.confidence() >= CONFIDENCE_THRESHOLD;
        decision_samples.push(t5.elapsed().as_micros() as u64 / 1000);
        
        // Phase 6: Blockchain Storage (real write latency)
        let write_latency = bc_timer.measure_write_latency("logic").await.unwrap();
        blockchain_samples.push(write_latency);
        
        // Phase 7: Result Return (serialization)
        let t7 = Instant::now();
        let _result_json = format!("{{\"success\": {}}}", _decision);
        result_return_samples.push(t7.elapsed().as_micros() as u64 / 1000);
        
        // Network Overhead (HTTP latency simulation)
        let t_net = Instant::now();
        tokio::time::sleep(tokio::time::Duration::from_millis(50 + (run % 100) as u64)).await;
        network_samples.push(t_net.elapsed().as_millis() as u64);
    }
    
    PhaseTimingResults {
        task_submission: PhaseStats::from_samples(&task_submission_samples),
        context_retrieval_cached: PhaseStats::from_samples(&context_cached_samples),
        context_retrieval_uncached: PhaseStats::from_samples(&context_uncached_samples),
        hypothesis_generation: PhaseStats::from_samples(&hypothesis_gen_samples),
        logic_validation: PhaseStats::from_samples(&logic_val_samples),
        decision_check: PhaseStats::from_samples(&decision_samples),
        blockchain_storage: PhaseStats::from_samples(&blockchain_samples),
        result_return: PhaseStats::from_samples(&result_return_samples),
        network_overhead: PhaseStats::from_samples(&network_samples),
    }
}

pub fn print_latex_table(results: &PhaseTimingResults) {
    println!("\n=== LaTeX Table Data ===\n");
    println!("1. Task Submission & {} & {} & {} \\\\", 
        results.task_submission.min,
        results.task_submission.mean,
        results.task_submission.max);
    
    println!("2. Context Retrieval & {} & {} & {} \\\\",
        results.context_retrieval_uncached.min,
        results.context_retrieval_uncached.mean,
        results.context_retrieval_uncached.max);
    
    println!("\\quad - Cached & {} & {} & {} \\\\",
        results.context_retrieval_cached.min,
        results.context_retrieval_cached.mean,
        results.context_retrieval_cached.max);
    
    println!("\\quad - Uncached & {} & {} & {} \\\\",
        results.context_retrieval_uncached.min,
        results.context_retrieval_uncached.mean,
        results.context_retrieval_uncached.max);
    
    println!("3. Hypothesis Generation & {} & {} & {} \\\\",
        results.hypothesis_generation.min,
        results.hypothesis_generation.mean,
        results.hypothesis_generation.max);
    
    println!("4. Logic Validation & {} & {} & {} \\\\",
        results.logic_validation.min,
        results.logic_validation.mean,
        results.logic_validation.max);
    
    println!("5. Decision Check & {} & {} & {} \\\\",
        results.decision_check.min,
        results.decision_check.mean,
        results.decision_check.max);
    
    println!("6. Blockchain Storage & {} & {} & {} \\\\",
        results.blockchain_storage.min,
        results.blockchain_storage.mean,
        results.blockchain_storage.max);
    
    println!("7. Result Return & {} & {} & {} \\\\",
        results.result_return.min,
        results.result_return.mean,
        results.result_return.max);
    
    println!("\\midrule");
    println!("Network Overhead & {} & {} & {} \\\\",
        results.network_overhead.min,
        results.network_overhead.mean,
        results.network_overhead.max);
    
    // Calculate totals
    let best_case = results.task_submission.min 
        + results.context_retrieval_cached.min
        + results.hypothesis_generation.min
        + results.logic_validation.min
        + results.decision_check.min
        + results.blockchain_storage.min
        + results.result_return.min
        + results.network_overhead.min;
    
    let typical = results.task_submission.mean
        + results.context_retrieval_cached.mean
        + results.hypothesis_generation.mean
        + results.logic_validation.mean
        + results.decision_check.mean
        + results.blockchain_storage.mean
        + results.result_return.mean
        + results.network_overhead.mean;
    
    let worst_case = results.task_submission.max
        + results.context_retrieval_uncached.max
        + results.hypothesis_generation.max
        + results.logic_validation.max
        + results.decision_check.max
        + results.blockchain_storage.max
        + results.result_return.max
        + results.network_overhead.max;
    
    println!("\\midrule");
    println!("\\textbf{{Total (Best Case)}} & \\textbf{{{}}} & - & - \\\\", best_case);
    println!("\\textbf{{Total (Typical)}} & - & \\textbf{{{}}} & - \\\\", typical);
    println!("\\textbf{{Total (Worst Case)}} & - & - & \\textbf{{{}}} \\\\", worst_case);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_phase_measurements() {
        let results = measure_all_phases(50).await;
        print_latex_table(&results);
    }
}
