## PrimeZeta — solution_2 (C-Accelerated)

![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-1-blueviolet)

**Single-threaded C-accelerated sieve — re-computes π(1M) on every pass.**

Zeta calls `run_sieve` via extern bridge. The C runtime implements the sieve with POPCNT word-level bit counting, 30030-wheel skipping, and batched word clearing.

### Performance
- **Throughput**: ~3,424 passes/5s
- **π(1,000,000)**: 78,498
- **Algorithm**: Word-level bit array, POPCNT counting, wheel residue skip
