//! Main experiment runner for paper results collection

use std::path::PathBuf;

fn main() {
    println!("Enigma Metaverse - Paper Experiments Runner");
    println!("===========================================\n");
    
    let results_base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("results");
    
    println!("Results directory: {:?}\n", results_base);
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "blockchain" => experiments::blockchain_benchmarks::run_all_benchmarks(),
        "cognitive" => experiments::cognitive_loop::run_all_analysis(),
        "gap" => experiments::gap_resolution::run_study(),
        "multi-domain" => experiments::multi_domain::run_evaluation(),
        "llm" => experiments::llm_comparison::run_comparison(),
        "error" => experiments::error_analysis::run_analysis(),
        "sota" => experiments::sota_comparison::run_comparison(),
        "all" => {
            println!("Running ALL experiments...\n");
            experiments::multi_domain::run_evaluation();
            experiments::llm_comparison::run_comparison();
            experiments::error_analysis::run_analysis();
            experiments::gap_resolution::run_study();
            experiments::cognitive_loop::run_all_analysis();
            experiments::blockchain_benchmarks::run_all_benchmarks();
            experiments::sota_comparison::run_comparison();
        },
        _ => {
            eprintln!("Unknown experiment: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage: cargo run -p experiments --bin run_experiments -- <experiment>");
    println!("\n1-WEEK PRIORITY PLAN:");
    println!("\nCRITICAL (Days 1-3):");
    println!("  multi-domain  - Table 9: Multi-domain evaluation");
    println!("  llm           - Tables 10-11: LLM comparison");
    println!("  error         - Tables 12-13: Error analysis");
    println!("\nIMPORTANT (Days 4-5):");
    println!("  gap           - Tables 7-8: Gap resolution");
    println!("  cognitive     - Tables 1-2: Cognitive loop timing");
    println!("\nNICE-TO-HAVE (Days 6-7):");
    println!("  blockchain    - Tables 3-6: Blockchain performance");
    println!("  sota          - Tables 14-17: SOTA comparison");
    println!("\nRUN ALL:");
    println!("  all           - Run all experiments in priority order");
}
