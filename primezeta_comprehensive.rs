use std::time::Instant;

fn count_primes(limit: i32) -> i32 {
    let mut count: i32 = 0;
    
    for n in 2..limit {
        let mut is_prime: bool = true;
        for i in 2..n {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
        }
    }
    
    count
}

fn main() {
    let test_limits = [10, 100, 1000, 10000];
    let expected_results = [4, 25, 168, 1229];
    
    println!("PrimeZeta Algorithm Validation");
    println!("==============================");
    println!("System: Core i9 13900H, 32GB DDR5 RAM");
    println!("Compiler: Rust 1.93.1 (FIXED - loops execute)\n");
    
    for (idx, &limit) in test_limits.iter().enumerate() {
        let start = Instant::now();
        let result = count_primes(limit);
        let duration = start.elapsed();
        
        let expected = expected_results[idx];
        let correct = result == expected;
        
        println!("Test {}: limit = {}", idx + 1, limit);
        println!("  Result: {}", result);
        println!("  Expected: {}", expected);
        println!("  Correct: {}", if correct { "✓ YES" } else { "✗ NO" });
        println!("  Execution time: {:?}", duration);
        println!();
    }
    
    // Additional performance test with larger limit
    println!("Performance Test: limit = 50000");
    let start = Instant::now();
    let result = count_primes(50000);
    let duration = start.elapsed();
    println!("  Result: {} primes", result);
    println!("  Execution time: {:?}", duration);
}