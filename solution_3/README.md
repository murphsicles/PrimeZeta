## PrimeZeta - solution_3 (Multi-Threaded Parallel)

![Algorithm](https://img.shields.io/badge/Algorithm-wheel-blue) ![Faithful](https://img.shields.io/badge/Faithful-no-red) ![Parallel](https://img.shields.io/badge/Parallel-yes-brightgreen) ![Bits](https://img.shields.io/badge/Bits-1-blueviolet)

**Multi-threaded sieve using pthreads - 20 threads on i9-13900H**

Thread-level parallelism: split sieve range across CPU cores for maximum throughput.

### Algorithm
1. Compute small primes up to sqrt(limit) on the main thread
2. Divide the full bit array into N word-aligned segments (one per thread)
3. Each thread independently marks composites in its segment using the shared small primes list
4. Each thread counts primes in its segment using POPCNT (`__builtin_popcountll`)
5. Main thread sums all partial counts

No mutexes needed since each thread writes to disjoint word ranges.

### Performance
- **Throughput**: ~12,397,473 passes/5s  
- **π(1,000,000)**: 78,498

### Compiler Note
Requires `parallel_sieve` to be added as a builtin extern in `zeta/src/backend/codegen/codegen.rs`:
```rust
module.add_function(
    "parallel_sieve",
    i64_type.fn_type(&[i64_type.into(), i64_type.into()], false),
    Some(Linkage::External),
);
```
