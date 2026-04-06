# Murphy's Sieve Stall Analysis

## Problem Statement
Agents stall when trying to run PrimeZeta (Murphy's Sieve). REAL-BENCHMARK-EXECUTOR stalled for 4.5 hours, FRESH-BENCHMARK-EXECUTOR likely same.

## Code Analysis

### 1. Array Size Issue
```zeta
let array_size = limit + 1  // For limit=1,000,000 → 1,000,001
let mut bits: [u8] = []
while i < array_size {
    bits.append(1)
    i += 1
}
```
**Issue**: Array of 1,000,001 elements being initialized one by one in a loop.

### 2. Multiple Sequential Loops
Six separate while loops for multiples of 2, 3, 5, 7, 11, 13:
- Each loop iterates ~limit/prime times
- Total iterations: ~1M/2 + 1M/3 + 1M/5 + 1M/7 + 1M/11 + 1M/13 ≈ 1.2M iterations

### 3. Main Sieve Loop Complexity
```zeta
while p * p <= limit {
    if bits[p] == 1 {
        let mut multiple = p * p
        while multiple <= limit {
            bits[multiple] = 0
            multiple += p
        }
    }
    // Wheel increment logic...
}
```
**Complexity**: O(n log log n) for standard sieve, but with wheel optimization.

### 4. Wheel Increment Logic
```zeta
while p <= limit && (p % 2 == 0 || p % 3 == 0 || p % 5 == 0 || 
                    p % 7 == 0 || p % 11 == 0 || p % 13 == 0) {
    p += 1
}
```
**Issue**: 6 modulo operations per candidate check. For p up to 1000, this could be thousands of operations.

## Potential Stall Causes

### A. Memory Allocation
- `[bool; 1000000]` = 1MB (if 1 byte per bool)
- Some languages may use larger representations
- Array initialization could be slow

### B. Loop Execution Time
- Total operations estimated: ~100M operations
- If each operation takes 10ns → 1 second
- If interpreted/emulated → much slower

### C. Zeta Language Specifics
- Zeta might be interpreted or poorly optimized
- Array bounds checking overhead
- Modulo operations could be expensive

### D. Infinite Loop Risk
The wheel increment logic could theoretically stall if:
1. `p` reaches limit but conditions still true
2. Integer overflow in `p * p`
3. Logic error in modulo conditions

## Diagnostic Tests Performed

### 1. Code Analysis Only
- No execution of actual sieve
- Identified potential bottlenecks

### 2. Safe Compilation Test
- Tests if code compiles without syntax errors
- Doesn't test runtime behavior

## Recommended Investigation Steps

### 1. Minimal Execution Test
```rust
// Test with tiny limit
fn test_tiny() {
    let result = murphy_sieve(100);  // Not 1,000,000
    println!("Result: {}", result);
}
```

### 2. Timing Instrumentation
Add timing to each section:
- Array initialization
- Each multiples loop
- Main sieve loop
- Prime counting

### 3. Progressive Testing
1. limit = 10
2. limit = 100  
3. limit = 1000
4. limit = 10000
5. limit = 100000
6. limit = 1000000

### 4. Memory Profiling
Check actual memory usage during execution.

## Critical Finding

The most likely stall point is the **array initialization loop**:
```zeta
while i < array_size {
    bits.append(1)  // Appending 1,000,001 times
    i += 1
}
```

In some languages/implementations, appending to an array in a loop can be O(n²) due to reallocations.

## Immediate Action

**DO NOT RUN THE FULL SIEVE**. Instead:
1. Create a test with limit=100
2. Add debug prints to track progress
3. Compile without optimization to see warnings
4. Check for infinite loops in wheel logic

## Safety Recommendation

Until the stall cause is identified, all PrimeZeta runs should:
1. Use timeout mechanisms
2. Run in isolated processes
3. Have memory limits
4. Include progress reporting