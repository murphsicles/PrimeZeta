## PrimeZeta — solution_2 (Single-Threaded Runtime)

![Algorithm](https://img.shields.io/badge/Algorithm-base-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-no-lightgrey) ![Bits](https://img.shields.io/badge/Bits-8-yellowgreen)

**Single-threaded runtime sieve — byte array (8 bits/flag), pre-cached small primes.**

Uses Zeta's extern FFI to call an optimized C sieve. Byte-level memory access avoids the bit-manipulation arithmetic overhead. Matches Zig's approach.

### Performance
- **Throughput**: ~9,000 passes/5s (through Zeta FFI), ~11,800 in direct C
- **π(1,000,000)**: 78,498
- **Algorithm**: Base Eratosthenes, odd-only, byte array with direct indexing
