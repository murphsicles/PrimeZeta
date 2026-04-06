use std::time::Instant;

// ============================================================================
// CURRENT ALGORITHM (Baseline - sqrt optimization)
// ============================================================================

fn count_primes_sqrt(limit: i32) -> i32 {
    if limit < 2 {
        return 0;
    }
    
    let mut count = 1; // 2 is prime
    let mut primes = vec![2];
    
    for n in 3..limit {
        let mut is_prime = true;
        let sqrt_n = (n as f64).sqrt() as i32;
        
        for &p in &primes {
            if p > sqrt_n {
                break;
            }
            if n % p == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            count += 1;
            primes.push(n);
        }
    }
    
    count
}

// ============================================================================
// NAIVE ALGORITHM (Reference)
// ============================================================================

fn count_primes_naive(limit: i32) -> i32 {
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

// ============================================================================
// BASIC SIEVE OF ERATOSTHENES
// ============================================================================

fn sieve_basic(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    if limit > 1 {
        is_prime[1] = false;
    }
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 2..=sqrt_limit {
        if is_prime[i] {
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    
    is_prime.iter().filter(|&&x| x).count()
}

// ============================================================================
// SIEVE WITH WHEEL OPTIMIZATION (2,3,5)
// ============================================================================

fn sieve_wheel_235(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    if limit > 1 {
        is_prime[1] = false;
    }
    
    // Handle small limits
    if limit <= 3 {
        return if limit == 2 { 1 } else { 2 };
    }
    if limit <= 5 {
        return if limit == 4 { 2 } else { 3 };
    }
    
    // Pre-mark multiples of small primes
    // Multiples of 2
    for i in (4..limit).step_by(2) {
        is_prime[i] = false;
    }
    
    // Multiples of 3
    for i in (9..limit).step_by(3) {
        is_prime[i] = false;
    }
    
    // Multiples of 5
    for i in (25..limit).step_by(5) {
        is_prime[i] = false;
    }
    
    // Main sieve starting from 7
    let sqrt_limit = (limit as f64).sqrt() as usize;
    
    // Use wheel increments: +4, +2, +4, +2, +4, +6, +2, +6 (30-wheel pattern)
    let wheel = [4, 2, 4, 2, 4, 6, 2, 6];
    let mut wheel_index = 0;
    
    let mut p = 7;
    while p <= sqrt_limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple < limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
        
        // Move to next candidate using wheel
        p += wheel[wheel_index];
        wheel_index = (wheel_index + 1) % 8;
    }
    
    // Count primes
    let mut count = 0;
    
    // Count small primes we know
    if limit > 2 { count += 1; } // 2
    if limit > 3 { count += 1; } // 3
    if limit > 5 { count += 1; } // 5
    
    // Count remaining primes using wheel
    let mut i = 7;
    wheel_index = 0;
    
    while i < limit {
        if is_prime[i] {
            count += 1;
        }
        
        i += wheel[wheel_index];
        wheel_index = (wheel_index + 1) % 8;
    }
    
    count
}

// ============================================================================
// BIT-PACKED SIEVE (Memory efficient)
// ============================================================================

fn sieve_bitpacked(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    // Calculate number of u64 words needed (64 bits per word)
    let words_needed = (limit + 63) / 64;
    let mut bit_array = vec![0u64; words_needed];
    
    // Initialize: set all bits to 1 (prime)
    for word in &mut bit_array {
        *word = u64::MAX;
    }
    
    // Helper functions
    fn clear_bit(bits: &mut [u64], index: usize) {
        let word = index / 64;
        let bit = index % 64;
        bits[word] &= !(1 << bit);
    }
    
    fn is_set(bits: &[u64], index: usize) -> bool {
        let word = index / 64;
        let bit = index % 64;
        (bits[word] >> bit) & 1 == 1
    }
    
    // Mark 0 and 1 as not prime
    clear_bit(&mut bit_array, 0);
    clear_bit(&mut bit_array, 1);
    
    // Sieve algorithm
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for p in 2..=sqrt_limit {
        if is_set(&bit_array, p) {
            let mut multiple = p * p;
            while multiple < limit {
                clear_bit(&mut bit_array, multiple);
                multiple += p;
            }
        }
    }
    
    // Count primes
    let mut count = 0;
    for i in 2..limit {
        if is_set(&bit_array, i) {
            count += 1;
        }
    }
    
    count
}

// ============================================================================
// SEGMENTED SIEVE (Cache-friendly)
// ============================================================================

const SEGMENT_SIZE: usize = 32768; // 32KB

fn segmented_sieve(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    // For small limits, use basic sieve
    if limit <= SEGMENT_SIZE * 2 {
        return sieve_basic(limit);
    }
    
    // Step 1: Find all primes up to sqrt(limit)
    let sqrt_limit = (limit as f64).sqrt() as usize;
    let small_primes = {
        let mut is_prime = vec![true; sqrt_limit + 1];
        is_prime[0] = false;
        if sqrt_limit >= 1 {
            is_prime[1] = false;
        }
        
        for i in 2..=sqrt_limit {
            if is_prime[i] {
                let mut j = i * i;
                while j <= sqrt_limit {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        
        is_prime.iter()
            .enumerate()
            .filter(|&(_, &is_prime)| is_prime)
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    };
    
    // Step 2: Process in segments
    let mut count = 0;
    let mut low = 0;
    
    while low < limit {
        let high = std::cmp::min(low + SEGMENT_SIZE, limit);
        
        // Segment array (all numbers initially prime)
        let mut segment = vec![true; high - low];
        
        // For each small prime, mark its multiples in this segment
        for &p in &small_primes {
            // Find first multiple of p in [low, high)
            let start = if low % p == 0 {
                low
            } else {
                low + p - (low % p)
            };
            
            // Mark multiples starting from max(p*p, start)
            let mut multiple = std::cmp::max(p * p, start);
            while multiple < high {
                segment[multiple - low] = false;
                multiple += p;
            }
        }
        
        // Count primes in this segment
        for (i, &is_prime) in segment.iter().enumerate() {
            let num = low + i;
            if num >= 2 && is_prime {
                count += 1;
            }
        }
        
        low = high;
    }
    
    count
}

// ============================================================================
// BENCHMARK FUNCTION
// ============================================================================

fn benchmark_algorithm<F>(name: &str, f: F, limit: usize, expected: usize) -> f64 
where
    F: Fn(usize) -> usize,
{
    // Warmup
    let _ = f(1000);
    
    // Run benchmark
    let start = Instant::now();
    let result = f(limit);
    let elapsed = start.elapsed();
    
    // Verify result
    if result != expected {
        println!("  ERROR: {} returned {} but expected {}", name, result, expected);
        return 0.0;
    }
    
    elapsed.as_secs_f64() * 1000.0 // Convert to milliseconds
}

fn main() {
    println!("PHASE 1 ALGORITHM IMPLEMENTATION - COMPREHENSIVE BENCHMARK");
    println!("==========================================================\n");
    println!("Target: 60,000x vs naive (480x improvement over current sqrt optimization)");
    println!("System: Core i9 13900H, 32GB DDR5\n");
    
    // Test limits
    let test_limits = [
        (1000, 168),
        (10000, 1229),
        (50000, 5133),
        (100000, 9592),
        (1000000, 78498),
        (5000000, 348513),
    ];
    
    for (limit, expected) in test_limits {
        println!("Limit = {} (expected primes: {}):", limit, expected);
        
        // Skip naive for larger limits (too slow)
        let naive_time = if limit <= 100000 {
            benchmark_algorithm("Naive", |lim| count_primes_naive(lim as i32) as usize, limit, expected)
        } else {
            -1.0 // Indicate skipped
        };
        
        let sqrt_time = benchmark_algorithm("Sqrt (current)", |lim| count_primes_sqrt(lim as i32) as usize, limit, expected);
        let sieve_time = benchmark_algorithm("Basic Sieve", sieve_basic, limit, expected);
        let wheel_time = benchmark_algorithm("Wheel Sieve", sieve_wheel_235, limit, expected);
        let bitpacked_time = benchmark_algorithm("Bitpacked Sieve", sieve_bitpacked, limit, expected);
        let segmented_time = benchmark_algorithm("Segmented Sieve", segmented_sieve, limit, expected);
        
        // Print results
        if naive_time >= 0.0 {
            println!("  Naive:          {:8.3} ms", naive_time);
        } else {
            println!("  Naive:          (skipped - too slow)");
        }
        
        println!("  Sqrt (current): {:8.3} ms", sqrt_time);
        println!("  Basic Sieve:    {:8.3} ms", sieve_time);
        println!("  Wheel Sieve:    {:8.3} ms", wheel_time);
        println!("  Bitpacked:      {:8.3} ms", bitpacked_time);
        println!("  Segmented:      {:8.3} ms", segmented_time);
        
        // Calculate speedups
        if naive_time > 0.0 {
            let sqrt_vs_naive = naive_time / sqrt_time;
            let sieve_vs_naive = naive_time / sieve_time;
            let wheel_vs_naive = naive_time / wheel_time;
            let bitpacked_vs_naive = naive_time / bitpacked_time;
            let segmented_vs_naive = naive_time / segmented_time;
            
            println!("  Speedup vs Naive:");
            println!("    Sqrt:      {:8.2}x", sqrt_vs_naive);
            println!("    Basic:     {:8.2}x", sieve_vs_naive);
            println!("    Wheel:     {:8.2}x", wheel_vs_naive);
            println!("    Bitpacked: {:8.2}x", bitpacked_vs_naive);
            println!("    Segmented: {:8.2}x", segmented_vs_naive);
        }
        
        // Calculate improvement over current
        let sieve_vs_sqrt = sqrt_time / sieve_time;
        let wheel_vs_sqrt = sqrt_time / wheel_time;
        let bitpacked_vs_sqrt = sqrt_time / bitpacked_time;
        let segmented_vs_sqrt = sqrt_time / segmented_time;
        
        println!("  Improvement over current (Sqrt):");
        println!("    Basic Sieve:    {:6.2}x", sieve_vs_sqrt);
        println!("    Wheel Sieve:    {:6.2}x", wheel_vs_sqrt);
        println!("    Bitpacked:      {:6.2}x", bitpacked_vs_sqrt);
        println!("    Segmented:      {:6.2}x", segmented_vs_sqrt);
        
        println!();
    }
    
    // Summary for Phase 1 target
    println!("PHASE 1 TARGET ASSESSMENT:");
    println!("==========================");
    println!("Target: 60,000x vs naive (480x improvement over current)");
    
    // Use limit=100,000 for assessment (from benchmark data)
    let limit = 100000;
    let expected = 9592;
    
    // Get times for this limit
    let naive_time = 1165.39; // From earlier test
    let sqrt_time = benchmark_algorithm("Sqrt", |lim| count_primes_sqrt(lim as i32) as usize, limit, expected);
    let best_sieve_time = benchmark_algorithm("Best Sieve", segmented_sieve, limit, expected);
    
    println!("At limit = {}:", limit);
    println!("  Naive time:        {:.2} ms", naive_time);
    println!("  Current (sqrt):    {:.2} ms", sqrt_time);
    println!("  Best sieve:        {:.2} ms", best_sieve_time);
    
    let current_vs_naive = naive_time / sqrt_time;
    let best_vs_naive = naive_time / best_sieve_time;
    let improvement = best_vs_naive / current_vs_naive;
    
    println!("\nCurrent performance: {:.0}x vs naive", current_vs_naive);
    println!("Best sieve performance: {:.0}x vs naive", best_vs_naive);
    println!("Improvement over current: {:.1}x", improvement);
    
    let target_vs_naive = 60000.0;
    let needed_improvement = target_vs_naive / current_vs_naive;
    
    println!("\nPhase 1 Target: {:.0}x vs naive", target_vs_naive);
    println!("Additional improvement needed: {:.1}x", needed_improvement);
    
    if best_vs_naive >= target_vs_naive * 0.9 {
        println!("\n✅ PHASE 1 TARGET WITHIN REACH!");
        println!("   Current best: {:.0}x vs naive", best_vs_naive);
        println!("   Target: {:.0}x vs naive", target_vs_naive);
        println!("   Achievement: {:.1}% of target", (best_vs_naive / target_vs_naive) * 100.0);
    } else {
        println!("\n⚠️  PHASE 1 TARGET NOT YET REACHED");
        println!("   Current best: {:.0}x vs naive", best_vs_naive);
        println!("   Target: {:.0}x vs naive", target_vs_naive);
        println!("   Gap: {:.1}x improvement needed", target_vs_naive / best_vs_naive);
    }
}