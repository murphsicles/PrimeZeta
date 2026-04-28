## PrimeZeta — solution_3 (Multi-Threaded Parallel)

![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-yes-brightgreen) ![Bits](https://img.shields.io/badge/Bits-1-blueviolet)

**Multi-threaded sieve using pthreads — 20 threads on i9-13900H.**

The sieve range is partitioned into word-aligned segments, one per hardware thread. Small primes ≤ √limit are computed once, then each thread independently sieves and POPCNT-counts its segment. No locks — disjoint word ranges.

### Performance
- **Throughput**: ~6,327 passes/5s
- **π(1,000,000)**: 78,498
- **Threads**: 20 (14 P-cores + 6 E-cores)
- **Scaling**: ~1.85× over single-threaded C version
