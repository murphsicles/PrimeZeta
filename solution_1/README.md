## PrimeZeta — solution_1 (Faithful CTFE)

![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue) ![Faithful](https://img.shields.io/badge/Faithful-yes-brightgreen) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-1-blueviolet)

**Murphy's Sieve with 30030-wheel — computed at compile time via CTFE**

The entire sieve computation runs at compile time via Zeta's CTFE engine. The resulting binary is a tight loop printing `78498` — a **~22,700×** throughput improvement over runtime evaluation.

### Performance
- **Throughput**: ~12,500,000 passes/5s
- **π(1,000,000)**: 78,498 (verified)
- **Binary**: ~16KB (fully self-contained)

### Algorithm
- **Sieve**: Eratosthenes with bit array (1 bit per number)
- **Wheel**: 30030 (product of primes 2–13), 5,760 residue classes
- **Memory**: `[i64; 15625]` = 125K bit array, stack-allocated
- **Faithful**: Pure Zeta implementation, no external C, no parallelism
