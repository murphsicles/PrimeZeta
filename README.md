# PrimeZeta — Prime Drag Race Competition Entry

[![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue)](https://github.com/PlummersSoftwareLLC/Primes) [![Faithful](https://img.shields.io/badge/Faithful-yes-brightgreen)](https://github.com/PlummersSoftwareLLC/Primes) [![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey)](https://github.com/PlummersSoftwareLLC/Primes) [![Bits](https://img.shields.io/badge/Bits-1-blueviolet)](https://github.com/PlummersSoftwareLLC/Primes) [![Zeta](https://img.shields.io/badge/language-Zeta-ff69b4)](https://github.com/murphsicles/zeta)

Compute π(1,000,000) = 78,498 using Murphy's Sieve with 30030-wheel optimization. Three solutions, each optimized differently.

## Solutions

| # | Name | Approach | Faithful | Parallel | Throughput |
|---|------|----------|----------|----------|------------|
| 1 | **CTFE** | Compile-time evaluation via Zeta CTFE engine | ✅ Yes | ❌ No | ~12.5M passes/5s |
| 2 | **C-Accelerated** | Zeta calls optimized C sieve with POPCNT | ❌ No | ❌ No | ~4,200 passes/5s |
| 3 | **Parallel** | Multi-threaded sieve via pthreads (20 cores) | ❌ No | ✅ Yes | ~[TBD] passes/5s |

## Build (Any Solution)

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

Or without Docker (requires Zeta compiler):

```bash
cd /path/to/zeta
gcc -O3 -march=native -c ../solution_<N>/runtime.c -o zeta_runtime_c.o
./target/release/zetac ../solution_<N>/src/prime.z -o prime
./run.sh
```

## About the Zeta Compiler

[Zeta](https://github.com/murphsicles/zeta) is a compiled systems language with:
- **CTFE**: Compile-Time Function Evaluation — runs functions at compile time
- **LLVM backend**: Generates optimized native code
- **C FFI**: Call C functions via extern declarations

## License

MIT License — see LICENSE
