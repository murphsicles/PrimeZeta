# PrimeZeta — Prime Drag Race Competition Entry

[![Algorithm](https://img.shields.io/badge/Algorithm-base-blue)](https://github.com/PlummersSoftwareLLC/Primes) [![Bits](https://img.shields.io/badge/Bits-1-blueviolet)](https://github.com/PlummersSoftwareLLC/Primes) [![Zeta](https://img.shields.io/badge/language-Zeta-ff69b4)](https://github.com/murphsicles/zeta)

Compute π(1,000,000) = 78,498 using the Sieve of Eratosthenes.
Two solutions showcasing the Zeta compiler and its C FFI.

## Solutions

| # | Name | Approach | Faithful | Passes/5s |
|---|------|----------|----------|-----------|
| 1 | **Pure Zeta** | Full sieve in Zeta, compiled through LLVM | ✅ yes | ~900 |
| 2 | **C-Accelerated** | Zeta FFI to optimized C sieve | ❌ no | ~3,500 |

## Build & Run

```bash
cd solution_<N>
docker build -t primezeta-sln<N> .
docker run primezeta-sln<N>
```

## About Zeta

[Zeta](https://github.com/murphsicles/zeta) is a compiled systems language that targets LLVM IR, with C FFI via extern declarations.

## License

MIT
