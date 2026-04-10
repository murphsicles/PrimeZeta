// Competition Infinite Loop Wrapper for Murphy's Sieve
// 
// Competition Format:
// - Program runs in infinite loop
// - Competition harness measures how many complete executions in 5 seconds
// - Each execution must compute primes up to 1,000,000
// - Output format: "author;passes;time;num_threads;tags"
//
// This implementation is optimized for maximum passes in 5 seconds.

use std::time::Instant;

/// Count primes using Murphy's Sieve (optimized for speed)
fn count_primes_fast(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    let sieve_size = (limit + 1) / 2;
    let mut sieve = vec![true; sieve_size];
    let mut count = 1; // 2 is prime
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    
    // Unroll small loops for better performance
    // Handle 3 separately (first odd prime)
    {
        let start = 9;
        for j in (start..=limit).step_by(6) {
            sieve[j / 2] = false;
        }
    }
    
    // Process remaining odd primes
    for i in (5..=sqrt_limit).step_by(2) {
        let idx = i / 2;
        if sieve[idx] {
            let start = i * i;
            let step = i * 2;
            for j in (start..=limit).step_by(step) {
                sieve[j / 2] = false;
            }
        }
    }
    
    // Fast count using iterator (no bounds checking in release mode)
    count += sieve[1..].iter().filter(|&&is_prime| is_prime).count();
    
    count
}

/// Competition entry point with infinite loop
fn main() {
    // Competition parameters
    const LIMIT: usize = 1_000_000;
    const EXPECTED: usize = 78_498;
    
    // For competition submission, we'd use infinite loop
    // but for testing we run a fixed number of iterations
    #[cfg(not(feature = "competition"))]
    {
        let iterations = 100;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let result = count_primes_fast(LIMIT);
            assert_eq!(result, EXPECTED, "Incorrect prime count!");
        }
        
        let elapsed = start.elapsed();
        let passes_per_second = iterations as f64 / elapsed.as_secs_f64();
        
        println!("Author: MurphySieveTeam");
        println!("Passes: {}", iterations);
        println!("Time: {:.3}s", elapsed.as_secs_f64());
        println!("Passes/sec: {:.2}", passes_per_second);
        println!("Threads: 1");
        println!("Tags: rust,sieve,optimized");
    }
    
    // Competition version (infinite loop)
    #[cfg(feature = "competition")]
    {
        loop {
            let result = count_primes_fast(LIMIT);
            // In competition, we might print or just compute
            // The harness counts completed loops
            if result != EXPECTED {
                eprintln!("ERROR: Expected {}, got {}", EXPECTED, result);
                std::process::exit(1);
            }
        }
    }
}

/// Fast verification function for testing
pub fn verify_implementation() -> bool {
    const TEST_CASES: &[(usize, usize)] = &[
        (10, 4),
        (100, 25),
        (1_000, 168),
        (10_000, 1_229),
        (100_000, 9_592),
        (1_000_000, 78_498),
    ];
    
    for &(limit, expected) in TEST_CASES {
        let result = count_primes_fast(limit);
        if result != expected {
            eprintln!("FAIL: limit={}, expected={}, got={}", limit, expected, result);
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fast_implementation() {
        assert!(verify_implementation());
    }
    
    #[test]
    fn test_performance() {
        let start = Instant::now();
        let result = count_primes_fast(1_000_000);
        let elapsed = start.elapsed();
        
        assert_eq!(result, 78_498);
        assert!(elapsed.as_millis() < 20, "Too slow: {}ms", elapsed.as_millis());
    }
}