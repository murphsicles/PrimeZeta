## PrimeZeta — solution_2 (Single-Threaded Faithful)

![Algorithm](https://img.shields.io/badge/Algorithm-base-blue) ![Faithful](https://img.shields.io/badge/Faithful-yes-green) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-1-green)

**Pure Zeta runtime sieve — all algorithm code in Zeta.**

The entire sieve is written in Zeta: bit array initialization, composite clearing with counting-while-clearing, and prime verification. Only `get_time_us`, `time_is_up`, and `print_result` are C externs (I/O and timing infrastructure — permitted for all entries).

### Performance
- **Throughput**: ~2,350 passes/5s
- **π(1,000,000)**: 78,498
- **Algorithm**: Base Eratosthenes, odd-only, bit array (1 bit/flag)
- **Memory**: `[i64; 15625]` stack-allocated bit array (125KB)
- **Compiler**: Zeta v0.8.0 with inline LLVM GEP + load/store (no C function calls for array ops)

### Why Faithful=yes
The sieve algorithm executes entirely in Zeta-compiled code. The C runtime provides only:
- `get_time_us` / `time_is_up` — timing for competition harness
- `print_result` — I/O for competition output format

No C code participates in the sieve computation, bit manipulation, or prime counting.
