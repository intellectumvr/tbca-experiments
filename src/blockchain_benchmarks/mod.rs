pub mod layer_performance;
pub mod scalability_agents;
pub mod scalability_knowledge;

pub use layer_performance::measure_layer_performance;
pub use scalability_agents::measure_agent_scalability;
pub use scalability_knowledge::measure_kb_scalability;

pub fn run_all_benchmarks() {
    println!("Blockchain benchmarks - use: cargo test blockchain");
}
pub mod tps_measurement;
pub use tps_measurement::measure_real_tps;
