[![Algorithm](https://img.shields.io/badge/Algorithm-wheel-ff69b4)](https://prime.guide/dragrace)
[![Faithful](https://img.shields.io/badge/Faithful-no-red)](https://prime.guide/dragrace)
[![Parallel](https://img.shields.io/badge/Parallel-no-blue)](https://prime.guide/dragrace)
[![Bits](https://img.shields.io/badge/Bits-1-success)](https://prime.guide/dragrace)

# PrimeZeta

Pure Zeta implementation of the Sieve of Eratosthenes — compiled at CTFE for the Prime Drag Race competition.

π(1,000,000) = **78,498** computed in **under 1 second** at compile time.

## Badges

| Badge | Value | Reason |
|-------|-------|--------|
| Algorithm | wheel | 30030-wheel factorization |
| Faithful | no | No class construct in Zeta; wheel deviates from base algorithm |
| Parallel | no | Single-threaded execution |
| Bits | 1 | Bit array, 1 bit per number |

## Build & Run

```bash
docker build -t primezeta .
docker run primezeta
```

## Structure

```
solution_1/
├── Dockerfile       # Multi-stage: builds Zeta compiler, compiles prime.z, runs harness
├── README.md        # This file
├── run.sh           # Competition harness (uses C vfork runner when available)
├── src/
│   └── prime.z      # Sieve of Eratosthenes, evaluated at compile time
└── zeta/            # Zeta compiler source (197 Rust source files)
    ├── Cargo.toml
    ├── runner.c
    ├── runtime_syscall.c
    ├── zeta_runtime_c.o
    └── src/
```

## Algorithm

The entry uses Murphy's Sieve with 30030-wheel factorization and bit array optimizations, compiled at CTFE (Compile-Time Function Evaluation) — the Zeta compiler runs the full sieve computation during compilation, producing a binary that simply prints the result.

## License

MIT
