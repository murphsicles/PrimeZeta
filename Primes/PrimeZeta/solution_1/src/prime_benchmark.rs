// Benchmark runner for PrimeZeta submission
// Calls the Zeta implementation of Murphy's Sieve and runs for 5 seconds

use std::time::{Instant, Duration};

// External function from Zeta compiled code
extern "C" {
    fn murphy_sieve(limit: u64) -> u64;
}

fn main() {
    println!("=== PrimeZeta Benchmark ===");
    println!("Testing Murphy's Sieve implementation...");
    
    // First, verify the implementation works correctly
    let test_cases = vec![
        (10, 4),     // primes under 10: 2, 3, 5, 7
        (100, 25),   // primes under 100: 25 primes
        (1000, 168), // primes under 1000: 168 primes
    ];
    
    let mut all_pass = true;
    for (limit, expected) in test_cases {
        unsafe {
            let result = murphy_sieve(limit);
            let pass = result == expected;
            println!("  limit={}: result={}, expected={}, {}", 
                     limit, result, expected, if pass { "✓" } else { "✗" });
            all_pass = all_pass && pass;
        }
    }
    
    // Test the main competition case
    println!("\nTesting competition case (limit=1,000,000)...");
    unsafe {
        let result = murphy_sieve(1_000_000);
        let expected = 78498;
        let pass = result == expected;
        println!("  result={}, expected={}, {}", 
                 result, expected, if pass { "✓" } else { "✗" });
        all_pass = all_pass && pass;
    }
    
    if !all_pass {
        eprintln!("\nTests failed! Cannot proceed with benchmark.");
        std::process::exit(1);
    }
    
    println!("\nAll tests passed!");
    
    // Run the benchmark for 5 seconds
    println!("\nRunning 5-second benchmark...");
    println!("(This simulates the competition harness timing)");
    
    let start = Instant::now();
    let target_duration = Duration::from_secs(5);
    
    let mut iterations = 0;
    
    // Run benchmark loop
    while start.elapsed() < target_duration {
        unsafe {
            // Call the Zeta sieve function directly
            let _ = murphy_sieve(1_000_000);
        }
        iterations += 1;
    }
    
    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();
    
    // Output in required format: label;iterations;total_time;num_threads;tags
    println!("\n=== Benchmark Results ===");
    println!("Competition format:");
    println!("zeta;{};{:.3};1;algorithm=wheel;faithful=yes;bits=1;parallel=no", 
             iterations, elapsed_secs);
    
    println!("\nDetailed results:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:.3} seconds", elapsed_secs);
    println!("  Time per iteration: {:.3} ms", (elapsed_secs * 1000.0) / iterations as f64);
    println!("  Iterations per second: {:.1}", iterations as f64 / elapsed_secs);
    
    // Final verification
    unsafe {
        let prime_count = murphy_sieve(1_000_000);
        println!("\nFinal verification:");
        println!("  Prime count up to 1,000,000: {}", prime_count);
        println!("  Expected: 78,498");
        println!("  Match: {}", prime_count == 78498);
    }
    
    // Competition readiness check
    println!("\n=== Competition Readiness ===");
    if iterations > 100 {
        println!("✅ READY FOR COMPETITION");
        println!("   Expected to complete >100 iterations in 5 seconds");
    } else {
        println!("⚠️  PERFORMANCE CONCERN");
        println!("   Only {} iterations in 5 seconds", iterations);
        println!("   Consider optimization for better competition results");
    }
}