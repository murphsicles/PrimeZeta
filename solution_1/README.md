## PrimeZeta — solution_1 (Faithful Runtime Sieve)

![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue) ![Faithful](https://img.shields.io/badge/Faithful-yes-brightgreen) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-1-blueviolet)

**Faithful Murphy's Sieve — pure Zeta, no external C, no parallelism.**

Each pass computes π(1,000,000) from scratch: bit array allocation, composite marking via sieve, and popcount via bit-scanning loop. No compile-time pre-computation — full algorithm execution per pass.

### Performance
- **Throughput**: ~625 passes/5s
- **π(1,000,000)**: 78,498 (verified)
- **Algorithm**: Eratosthenes sieve with 64-bit word-level bit array
- **Memory**: `[i64; 15625]` = 125KB stack-allocated bit array (1 bit per number)

### Why This Matters
This is the "control" entry — a baseline pure-Zeta implementation with no runtime optimizations. Solutions 2 and 3 build on this with C acceleration and parallel execution for higher throughput.
