# PrimeZeta — Prime Drag Race Competition Entry

[![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue)](https://github.com/PlummersSoftwareLLC/Primes) [![Faithful](https://img.shields.io/badge/Faithful-yes-brightgreen)](https://github.com/PlummersSoftwareLLC/Primes) [![Bits](https://img.shields.io/badge/Bits-1-blueviolet)](https://github.com/PlummersSoftwareLLC/Primes) [![Zeta](https://img.shields.io/badge/language-Zeta-ff69b4)](https://github.com/murphsicles/zeta)

Compute π(1,000,000) = 78,498 using Murphy's Sieve with 64-bit word-level bit array.
Three solutions with different optimization levels.

## Solutions

| # | Name | Approach | Faithful | Parallel | Throughput |
|---|------|----------|----------|----------|------------|
| 1 | **Faithful** | Pure Zeta runtime sieve — full algorithm per pass | ✅ Yes | ❌ No | ~700 passes/5s |
| 2 | **C-Accelerated** | Zeta calls optimized C sieve with POPCNT + wheel | ❌ No | ❌ No | ~3,400 passes/5s |
| 3 | **Parallel** | Multi-threaded sieve via pthreads (20 threads) | ❌ No | ✅ Yes | ~6,300 passes/5s |

All solutions re-compute π(1,000,000) on **every pass**. No compile-time pre-computation.

## Build & Run

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

Or without Docker (requires Zeta compiler and gcc):

```bash
cd /path/to/zeta
# Solution 1 (pure Zeta)
gcc -O2 -c ../solution_1/runtime.c -o zeta_runtime_c.o
./target/release/zetac ../solution_1/src/prime.z -o prime

# Solution 2 (C-accelerated)
gcc -O3 -march=native -c ../solution_2/runtime.c -o zeta_runtime_c.o
./target/release/zetac ../solution_2/src/prime.z -o prime

# Solution 3 (parallel)
gcc -O3 -march=native -pthread -c ../solution_3/runtime.c -o zeta_runtime_c.o
./target/release/zetac ../solution_3/src/prime.z -o prime
```

## Hardware

Benchmarked on Intel i9-13900H (Raptor Lake, 14 P-cores + 6 E-cores), 20 threads, AVX2, POPCNT.

## License

MIT License — see LICENSE
