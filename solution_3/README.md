## PrimeZeta — solution_3 (Multi-Threaded Parallel)

![Algorithm](https://img.shields.io/badge/Algorithm-base-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-yes-brightgreen) ![Bits](https://img.shields.io/badge/Bits-1-green)

**Pure Zeta sieve on 20 threads — parallel via pthreads.**

The C runtime creates 20 threads, each running the same pure-Zeta sieve function independently. Each thread allocates its own bit array on its own stack — no shared state, no locks. Total passes = passes × threads.

### Performance
- **Throughput**: ~7,280 passes/5s (20 threads aggregated)
- **π(1,000,000)**: 78,498
- **Algorithm**: Base Eratosthenes, odd-only, bit array
- **Scaling**: 5.6× over single-threaded pure Zeta (memory bandwidth bound)
