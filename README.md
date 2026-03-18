# TBCA Experiments

Experimental implementation of the Task-Based Cognitive Architecture (TBCA) - a neuro-symbolic system integrating LLM-based hypothesis generation, probabilistic logic verification, and blockchain-based persistent memory.

## Overview

This repository contains the experimental codebase for evaluating TBCA against ReAct+RAG+Toolformer baselines across multi-domain reasoning tasks. The architecture demonstrates that symbolic verification provides measurable advantages (96.7% vs 76.7% success rate) over tool-augmented language models alone.

## Key Components

- **Cognitive Loop**: Integrates LLM hypothesis generation with symbolic logic verification
- **Gap Resolution**: Iterative refinement protocol recovering 57.9% of initially failing hypotheses
- **Blockchain Memory**: Three-layer architecture (PoA/PBFT/PoW) for persistent knowledge storage
- **Multi-Domain Tasks**: Legal reasoning, economic transactions, and spatial navigation benchmarks

## Architecture

The system executes tasks through seven coordinated phases:
1. Task submission
2. Context retrieval from blockchain memory
3. Hypothesis generation (LLM)
4. Symbolic validation (Logic Engine)
5. Confidence threshold check
6. Blockchain storage of validated knowledge
7. Result return

Mean execution time: 2.47 seconds with 84.8% success rate.

## Requirements

- Rust 1.70+
- Ollama (local LLM inference)
- Foundry/Anvil (local blockchain testnet)
- Python 3.11+ (for analysis scripts)

## Running Experiments

```bash
# Start local blockchain instances
anvil --port 8545 &  # Layer 1 (PoA)
anvil --port 8546 &  # Layer 2 (PBFT)
anvil --port 8547 &  # Layer 3 (PoW)

# Start Ollama
ollama serve &

# Run TBCA evaluation
cargo run --release --bin tbca_evaluation

# Run baseline comparison
cargo run --release --bin baseline_comparison

# Run scalability tests
cargo run --release --bin scalability_test
```

## Evaluation Results

### Multi-Domain Performance
- **Legal Reasoning**: 100% success rate (TBCA) vs 83.3% (ReAct+RAG)
- **Economic Tasks**: 100% success rate (TBCA) vs 58.3% (ReAct+RAG)
- **Spatial Navigation**: 90% success rate (TBCA) vs 66.7% (ReAct+RAG)

### Scalability
- Maintains 100% success rate from 1 to 50 concurrent agents
- Latency scales linearly: 21ms (1 agent) to 1093ms (50 agents)
- Knowledge base query performance: 1ms (1 MB) to 186ms (100 MB)

## License

MIT License - see LICENSE file for details.

## Related Work

This implementation accompanies the paper "AGI-Powered Metaverse Architecture: Integrating LLMs, Symbolic Reasoning, and Multi-Blockchain Memory for Collective Intelligence."

## Status

This is a proof-of-concept research implementation. Production deployment requires:
- Integration with production blockchain networks
- Visual perception modules for embodied agents
- Human oversight mechanisms for high-stakes decisions
- Scalability optimization beyond 50 agents

## Contact

For questions or collaboration inquiries, please open an issue on GitHub.