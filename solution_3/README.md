## PrimeZeta — solution_3 (Multi-Threaded Parallel) 🏆

![Algorithm](https://img.shields.io/badge/Algorithm-base-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-yes-brightgreen) ![Bits](https://img.shields.io/badge/Bits-1-green) ![Zeta v0.8.4](https://img.shields.io/badge/Zeta-v0.8.4-8A2BE2)

**Pure Zeta sieve on 20 threads — parallel via pthreads.**

The C runtime creates 20 threads, each running the same pure-Zeta sieve function independently. Each thread allocates its own bit array — no shared state, no locks. Total passes = passes × threads.

Optimizations applied: `__builtin_ctpop` (POPCNT instruction), LLVM -O3 pass pipeline, pre-sieve clearing, unconditional composite clearing, periodic clock checks.

### Performance
- **Throughput**: **14,120** passes/5s (20 threads aggregated)
- **π(1,000,000)**: 78,498
- **Algorithm**: Base Eratosthenes, odd-only, bit array
- **Scaling**: ~1.3× over single-threaded (memory bandwidth bound)

### Why Faithful=no
The sieve function itself is pure Zeta, but thread creation and management is handled by C pthreads. Zeta does not have built-in threading constructs (yet). The Faithfulness criterion requires the implementation to demonstrate the language's own concurrency features.

### Compiler
Zeta v0.8.4 — LLVM -O3 pipeline, `__builtin_ctpop` intrinsic, POPCNT instruction, AVX2 auto-vectorization.
