## PrimeZeta — solution_3 (Multi-Threaded Parallel)

![Algorithm](https://img.shields.io/badge/Algorithm-base-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-yes-brightgreen) ![Bits](https://img.shields.io/badge/Bits-8-yellowgreen)

**Multi-threaded sieve using pthreads — 20 threads on i9-13900H.**

Range-split parallelization: 20 threads clear composites in disjoint byte-aligned ranges using the shared small-primes list. Byte array with 8 bits/flag for simple memory access.

### Performance
- **Throughput**: ~8,000 passes/5s (20 threads)
- **π(1,000,000)**: 78,498 (verified)
- **Limitation**: At 1M limit, the sieve is memory-bandwidth-bound — parallelization has limited scaling. Dominates the parallel Zeta category by default.
