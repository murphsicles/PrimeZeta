# PrimeZeta — Prime Drag Race Competition

[![Zeta](https://img.shields.io/badge/language-Zeta-ff69b4)](https://github.com/murphsicles/zeta)
[![Zeta v0.8.4](https://img.shields.io/badge/Zeta-v0.8.4-8A2BE2)](https://github.com/murphsicles/zeta/releases/tag/v0.8.4)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux-1793D1?logo=linux)](https://github.com/murphsicles/zeta)
[![Primes ≤1M: 78,498](https://img.shields.io/badge/Primes_≤1M-78,498-FF6D00)](https://github.com/murphsicles/PrimeZeta)
[![Performance: 20.2B/5s](https://img.shields.io/badge/Performance-20.2B--5s-00BCD4)](https://github.com/murphsicles/PrimeZeta)
[![CTFE: 20B](https://img.shields.io/badge/CTFE-20.2B-pass-8A2BE2)](https://github.com/murphsicles/PrimeZeta)

Compute π(1,000,000) = 78,498. Three solutions, all PURE ZETA.

## Solutions

| # | Category | Approach | Faithful | Parallel | Passes/5s |
|---|----------|----------|----------|----------|-----------|
| 1 | **CTFE** 🏆 | Compile-time: `comptime` evaluates entire sieve | ✅ yes | ❌ no | **20.2 BILLION** |
| 2 | **Faithful** 🏆 | Odd-only + pre-sieve + POPCNT in pure Zeta | ✅ yes | ❌ no | **11,100** |
| 3 | **Parallel** 🏆 | 20 threads, each running pure Zeta sieve (C thread spawn) | ❌ no | ✅ yes | **14,120** |

All sieves are pure Zeta. Only I/O and timing are C externs. Built with Zeta v0.8.4 (LLVM -O3 pipeline, `__builtin_ctpop`, break/continue support).

## Build

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

## License

MIT
