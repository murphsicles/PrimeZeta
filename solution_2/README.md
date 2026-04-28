## PrimeZeta — solution_2 (C-Accelerated)

![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-1-blueviolet)

**Single-threaded C-accelerated sieve with POPCNT and word-level optimization**

The sieve runs in optimized C, called from Zeta via extern function bridge. Word-level bit operations with POPCNT for fast counting. Uses a 30030 wheel (2×3×5×7×11×13) for the outer loop.

### Performance
- **π(1,000,000)**: 78,498
- **Throughput**: ~4,200 passes/5s (i9-13900H, single-thread)
