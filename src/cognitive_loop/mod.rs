pub mod blockchain_timing;
pub mod phase_timing;
pub mod success_rates;

pub use blockchain_timing::BlockchainTimer;
pub use phase_timing::measure_all_phases;
pub use success_rates::measure_success_rates;

pub fn run_all_analysis() {
    println!("Cognitive loop analysis - use cargo test");
}
