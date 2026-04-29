## PrimeZeta — solution_2 (Single-Threaded Faithful)

![Algorithm](https://img.shields.io/badge/Algorithm-wheel-yellowgreen) ![Faithful](https://img.shields.io/badge/Faithful-yes-green) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-1-green)

**Murphy's 30030-wheel sieve in pure Zeta.**

The entire sieve is written in Zeta: 30030-wheel pre-sieve (clears multiples of 2-13), word-level bit array, counting-while-clearing, and walking only wheel residues in the main loop. Only `get_time_us`, `time_is_up`, and `print_result` are C externs (I/O and timing infrastructure — permitted for all entries).

### Performance
- **Throughput**: ~2,400 passes/5s
- **π(1,000,000)**: 78,498
- **Algorithm**: 30030-wheel (Murphy's Sieve — skips multiples of 2,3,5,7,11,13)
- **Bits**: 1 bit per flag (word-level `[i64; 15625]` stack array, 125KB)
- **Compiler**: Zeta v0.8.0 with inline LLVM GEP + load/store (no C function calls for array ops)

### Why Faithful=yes
The sieve algorithm executes entirely in Zeta-compiled code. The C runtime provides only:
- `get_time_us` / `time_is_up` — timing for competition harness
- `print_result` — I/O for competition output format

No C code participates in the sieve computation, bit manipulation, or prime counting.
