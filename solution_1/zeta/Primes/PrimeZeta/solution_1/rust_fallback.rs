// Rust fallback submission for PrimeZeta competition
// Murphy's Sieve implementation with basic optimizations
// This is a backup in case Zeta implementation isn't ready

use std::time::{Instant, Duration};

/// Murphy's Sieve implementation
/// Counts primes up to limit using Sieve of Eratosthenes with odd-only optimization
fn murphy_sieve(limit: u64) -> u64 {
    if limit < 2 {
        return 0;
    }
    if limit == 2 {
        return 1;
    }
    
    // Only store odd numbers (3, 5, 7, ...)
    let array_size = ((limit - 1) / 2) as usize;
    let mut is_prime = vec![true; array_size];
    
    let sqrt_limit = (limit as f64).sqrt() as u64;
    
    // Start from 3 (first odd prime)
    let mut i: u64 = 3;
    let mut idx = 0; // index for 3 in the array
    
    while i <= sqrt_limit {
        if is_prime[idx] {
            // Mark multiples of i starting from i*i
            // Only mark odd multiples (i*i, i*i + 2i, i*i + 4i, ...)
            let mut multiple = i * i;
            let step = i * 2;
            
            while multiple <= limit {
                if multiple % 2 == 1 { // Only odd numbers are in our array
                    let m_idx = ((multiple - 1) / 2) as usize;
                    if m_idx < array_size {
                        is_prime[m_idx] = false;
                    }
                }
                multiple += step;
            }
        }
        
        i += 2;
        idx += 1;
    }
    
    // Count primes: start with 2 (the only even prime)
    let mut count = 1;
    
    // Count odd primes
    for &prime in &is_prime {
        if prime {
            count += 1;
        }
    }
    
    count
}

/// Competition main function - runs infinite loop
fn competition_main() {
    let limit = 1_000_000;
    
    // Infinite loop as required by competition
    loop {
        let count = murphy_sieve(limit);
        println!("{}", count); // Should print 78498
    }
}

/// Benchmark function - runs for 5 seconds and reports iterations
fn run_benchmark() {
    let limit = 1_000_000;
    let duration = Duration::from_secs(5);
    let start = Instant::now();
    
    let mut iterations = 0;
    
    while start.elapsed() < duration {
        let _ = murphy_sieve(limit);
        iterations += 1;
    }
    
    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    
    println!("=== Rust Fallback Benchmark Results ===");
    println!("rust;{};{:.3};1;algorithm=sieve;faithful=yes;bits=1;parallel=no", 
             iterations, elapsed_secs);
    println!("Iterations: {}", iterations);
    println!("Time per iteration: {:.3} ms", (elapsed_secs * 1000.0) / iterations as f64);
    
    // Verify correctness
    let prime_count = murphy_sieve(limit);
    println!("Prime count verification: {} (expected: 78498)", prime_count);
    println!("Match: {}", prime_count == 78498);
}

/// Test function to verify correctness
fn run_tests() {
    println!("=== Running Tests ===");
    
    let test_cases = vec![
        (10, 4),
        (100, 25),
        (1000, 168),
        (10000, 1229),
        (100000, 9592),
        (1000000, 78498),
    ];
    
    let mut all_pass = true;
    
    for (limit, expected) in test_cases {
        let result = murphy_sieve(limit);
        let pass = result == expected;
        println!("limit={}: result={}, expected={}, {}", 
                 limit, result, expected, if pass { "✓" } else { "✗" });
        all_pass = all_pass && pass;
    }
    
    if all_pass {
        println!("All tests passed!");
    } else {
        println!("Some tests failed!");
    }
}

fn main() {
    println!("=== Rust Fallback Submission for PrimeZeta ===");
    println!("");
    
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "test" => run_tests(),
            "benchmark" => run_benchmark(),
            "competition" => competition_main(),
            _ => {
                println!("Usage:");
                println!("  {} test        - Run correctness tests", args[0]);
                println!("  {} benchmark   - Run 5-second benchmark", args[0]);
                println!("  {} competition - Run competition infinite loop", args[0]);
            }
        }
    } else {
        // Default: run tests and benchmark
        run_tests();
        println!("");
        run_benchmark();
    }
}