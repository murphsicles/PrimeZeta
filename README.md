# PrimeZeta — Prime Drag Race Competition Entry

[![Algorithm](https://img.shields.io/badge/Algorithm-base-blue)](https://github.com/PlummersSoftwareLLC/Primes) [![Bits](https://img.shields.io/badge/Bits-1-blueviolet)](https://github.com/PlummersSoftwareLLC/Primes) [![Zeta](https://img.shields.io/badge/language-Zeta-ff69b4)](https://github.com/murphsicles/zeta)

Compute π(1,000,000) = 78,498 using the Sieve of Eratosthenes.
Two solutions showcasing the Zeta compiler.

## Solutions

| # | Name | Faithful | Passes/5s | Description |
|---|------|----------|-----------|-------------|
| 1 | **Pure Zeta** | ✅ yes | **1,250** | Full sieve in Zeta, LLVM-compiled. Odd-only base algorithm. |
| 2 | **C-Accelerated** | ❌ no | **7,041** | Zeta FFI to optimized C sieve with POPCNT. |

Both use word-level bit array (1 bit/flag), self-time to ~5s, output competition format.

## Build & Run

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

## License

MIT
