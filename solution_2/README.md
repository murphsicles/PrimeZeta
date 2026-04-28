## PrimeZeta — solution_2 (Single-Threaded Runtime)

![Algorithm](https://img.shields.io/badge/Algorithm-base-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-1-green)

**Pure Zeta runtime sieve — no C in the algorithm.**

The entire sieve is written in Zeta: bit array initialization, composite clearing, and prime counting. Only `get_time_us`, `time_is_up`, and `print_result` are C externs (I/O and timing infrastructure).

### Performance
- **Throughput**: ~1,300 passes/5s
- **π(1,000,000)**: 78,498
- **Algorithm**: Base Eratosthenes, odd-only, bit array (1 bit/flag)
- **Memory**: `[i64; 15625]` stack-allocated bit array (125KB)
