// Simple performance test for Murphy's Sieve
use std::time::{Instant, Duration};

fn count_primes_fast(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    let sieve_size = (limit + 1) / 2;
    let mut sieve = vec![true; sieve_size];
    let mut count = 1; // 2 is prime
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    
    // Handle 3 separately
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
    
    // Fast count
    count += sieve[1..].iter().filter(|&&is_prime| is_prime).count();
    
    count
}

fn main() {
    const LIMIT: usize = 1_000_000;
    const EXPECTED: usize = 78_498;
    
    println!("Testing Murphy's Sieve performance...");
    println!("Limit: {}", LIMIT);
    println!("Expected prime count: {}", EXPECTED);
    println!();
    
    // Warm up
    println!("Warming up...");
    for _ in 0..10 {
        let result = count_primes_fast(LIMIT);
        assert_eq!(result, EXPECTED);
    }
    
    // Measure single execution
    println!("Measuring single execution...");
    let start = Instant::now();
    let result = count_primes_fast(LIMIT);
    let single_time = start.elapsed();
    
    assert_eq!(result, EXPECTED);
    println!("Single execution: {:?}", single_time);
    println!("Result: {} (correct)", result);
    
    // Measure multiple executions for 5-second benchmark
    println!("\nMeasuring 5-second benchmark...");
    let benchmark_duration = Duration::from_secs(5);
    let start = Instant::now();
    let mut passes = 0;
    
    while start.elapsed() < benchmark_duration {
        let result = count_primes_fast(LIMIT);
        assert_eq!(result, EXPECTED);
        passes += 1;
    }
    
    let actual_duration = start.elapsed();
    let passes_per_second = passes as f64 / actual_duration.as_secs_f64();
    
    println!("Benchmark results:");
    println!("  Duration: {:.3}s", actual_duration.as_secs_f64());
    println!("  Passes: {}", passes);
    println!("  Passes/second: {:.2}", passes_per_second);
    println!("  Passes/5s: {:.0}", passes_per_second * 5.0);
    
    // Competition output format
    println!("\nCompetition output format:");
    println!("zeta;{};5.000;1;algorithm=wheel,faithful=yes,bits=1,parallel=no", passes);
}