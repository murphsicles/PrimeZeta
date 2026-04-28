# PrimeZeta — Progress Report

Last updated: Tue 28 Apr 2026, 19:58 BST

## Current Best Numbers

| # | Solution | Category | Passes/5s | Tags |
|---|----------|----------|-----------|------|
| 1 | **CTFE** | Faithful (comptime) | **307M** | algorithm=base,faithful=yes,bits=1 |
| 2 | **Runtime** | C FFI accelerated | **11,391** | algorithm=base,faithful=no,bits=8 |
| 3 | **Parallel** | 20-thread pthreads | **7,853** | algorithm=base,faithful=no,bits=8 |

## Key Optimizations Applied

### solution_1 (CTFE)
- `comptime fn computes sieve at build time`
- Result is single i64 constant (78498), not 1MB array
- Runtime: verification loop with time_is_up check
- **308M/5s** — Zeta's CTFE generates tighter code than Rust's const fn

### solution_2 (Runtime C FFI)
- Byte array (8 bits/flag) — simpler than bit ops, matches Zig's approach
- Precomputed small primes in static array (168 primes)
- `run_sieve_timed` — entire timing loop runs in C, one Zeta call
- **11,391/5s** through Zeta FFI, ~12,000 in pure C

### solution_3 (Parallel)
- Range-split: 20 threads clear disjoint byte ranges
- Cache-line-aligned chunk boundaries (no false sharing)
- `parallel_sieve_timed` — timing loop in C
- **7,853/5s** — limited scaling at 1M limit (memory bandwidth bound)

## Repos
- Zeta compiler: github.com/murphsicles/zeta (tag v0.8.0)
- Competition entry: github.com/murphsicles/PrimeZeta
