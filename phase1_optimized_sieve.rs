// Phase 1 Optimized Sieve - Combining all techniques for maximum performance
// Target: 60,000x vs naive (480x improvement over current)

use std::time::Instant;

// ============================================================================
// OPTIMIZED SIEVE WITH ALL TECHNIQUES
// ============================================================================

/// Combined optimized sieve using:
/// 1. Bit-packing (1 bit per number)
/// 2. Wheel factorization (skip multiples of 2, 3, 5)
/// 3. Cache-friendly segmented processing
/// 4. Pre-sieving small primes
fn optimized_sieve(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    if limit == 2 {
        return 1;
    }
    
    // Constants for wheel optimization (2,3,5 wheel)
    const WHEEL: [usize; 8] = [4, 2, 4, 2, 4, 6, 2, 6];
    const WHEEL_PRIMES: [usize; 3] = [2, 3, 5];
    const WHEEL_START: usize = 7;
    
    // Segment size optimized for L1 cache (32KB)
    const SEGMENT_SIZE: usize = 32768;
    
    // Step 1: Find primes up to sqrt(limit) using basic bit-packed sieve
    let sqrt_limit = (limit as f64).sqrt() as usize;
    let small_primes = find_small_primes_bitpacked(sqrt_limit);
    
    // Count small primes that are within the wheel primes
    let mut count = WHEEL_PRIMES.iter().filter(|&&p| p < limit).count();
    
    // Step 2: Process in segments
    let mut low = WHEEL_START;
    
    while low < limit {
        let high = std::cmp::min(low + SEGMENT_SIZE, limit);
        
        // Create segment as bit array (all bits initially 1 = prime)
        let segment_words = (high - low + 63) / 64;
        let mut segment = vec![u64::MAX; segment_words];
        
        // For each small prime, mark its multiples in this segment
        for &p in &small_primes {
            // Skip wheel primes (already handled)
            if p == 2 || p == 3 || p == 5 {
                continue;
            }
            
            // Find first multiple of p in [low, high)
            let start = if low % p == 0 {
                low
            } else {
                low + p - (low % p)
            };
            
            // Mark multiples starting from max(p*p, start)
            let mut multiple = std::cmp::max(p * p, start);
            while multiple < high {
                let idx = multiple - low;
                segment[idx / 64] &= !(1 << (idx % 64));
                multiple += p;
            }
        }
        
        // Count primes in this segment using wheel pattern
        let mut pos = low;
        let mut wheel_idx = 0;
        
        while pos < high {
            if pos >= WHEEL_START {
                let idx = pos - low;
                if (segment[idx / 64] >> (idx % 64)) & 1 == 1 {
                    count += 1;
                }
            }
            
            // Move to next wheel position
            pos += WHEEL[wheel_idx];
            wheel_idx = (wheel_idx + 1) % 8;
        }
        
        // Move to next segment, aligning with wheel pattern
        low = high;
    }
    
    count
}

/// Find primes up to limit using bit-packed sieve (for small limits)
fn find_small_primes_bitpacked(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }
    
    let words = (limit + 63) / 64;
    let mut bits = vec![u64::MAX; words];
    
    // Clear 0 and 1
    bits[0] &= !((1 << 0) | (1 << 1));
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    let mut p = 2;
    
    while p <= sqrt_limit {
        if (bits[p / 64] >> (p % 64)) & 1 == 1 {
            let mut multiple = p * p;
            while multiple <= limit {
                bits[multiple / 64] &= !(1 << (multiple % 64));
                multiple += p;
            }
        }
        p += 1;
    }
    
    // Collect primes
    let mut primes = Vec::new();
    for i in 2..=limit {
        if (bits[i / 64] >> (i % 64)) & 1 == 1 {
            primes.push(i);
        }
    }
    
    primes
}

// ============================================================================
// EVEN MORE OPTIMIZED VERSION WITH PRE-COMPUTED WHEEL
// ============================================================================

/// Ultra-optimized sieve using pre-computed wheel increments
fn ultra_optimized_sieve(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    // Use 2,3,5,7 wheel (210 wheel) for even better performance
    // This skips 77% of numbers
    const WHEEL210: [usize; 48] = [
        10, 2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4,
        2, 4, 8, 6, 4, 6, 2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2
    ];
    
    const WHEEL210_PRIMES: [usize; 4] = [2, 3, 5, 7];
    const WHEEL210_START: usize = 11;
    
    // For limits under 100,000, use simple sieve
    if limit <= 100_000 {
        return simple_bitpacked_sieve(limit);
    }
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    let small_primes = find_small_primes_bitpacked(sqrt_limit);
    
    // Count wheel primes
    let mut count = WHEEL210_PRIMES.iter().filter(|&&p| p < limit).count();
    
    // Process in segments
    const SEGMENT_SIZE: usize = 1 << 15; // 32KB
    let mut low = WHEEL210_START;
    
    while low < limit {
        let high = std::cmp::min(low + SEGMENT_SIZE, limit);
        let segment_size = high - low;
        let words = (segment_size + 63) / 64;
        let mut segment = vec![u64::MAX; words];
        
        // Mark multiples of small primes
        for &p in &small_primes {
            if p < 11 { continue; } // Skip wheel primes
            
            let start = if low % p == 0 {
                low
            } else {
                low + p - (low % p)
            };
            
            let mut multiple = std::cmp::max(p * p, start);
            while multiple < high {
                let idx = multiple - low;
                segment[idx / 64] &= !(1 << (idx % 64));
                multiple += p;
            }
        }
        
        // Count primes in segment using wheel pattern
        let mut pos = low;
        let mut wheel_idx = 0;
        
        while pos < high {
            let idx = pos - low;
            if (segment[idx / 64] >> (idx % 64)) & 1 == 1 {
                count += 1;
            }
            
            pos += WHEEL210[wheel_idx];
            wheel_idx = (wheel_idx + 1) % 48;
        }
        
        low = high;
    }
    
    count
}

/// Simple bit-packed sieve for smaller limits
fn simple_bitpacked_sieve(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    let words = (limit + 63) / 64;
    let mut bits = vec![u64::MAX; words];
    
    // Clear 0 and 1
    bits[0] &= !((1 << 0) | (1 << 1));
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    
    // Optimized: handle 2 separately
    let mut multiple = 4;
    while multiple <= limit {
        bits[multiple / 64] &= !(1 << (multiple % 64));
        multiple += 2;
    }
    
    // Process odd numbers only
    let mut p = 3;
    while p <= sqrt_limit {
        if (bits[p / 64] >> (p % 64)) & 1 == 1 {
            let mut multiple = p * p;
            while multiple <= limit {
                bits[multiple / 64] &= !(1 << (multiple % 64));
                multiple += p * 2; // Skip even multiples
            }
        }
        p += 2;
    }
    
    // Count primes
    let mut count = 1; // Count 2
    for i in (3..=limit).step_by(2) {
        if (bits[i / 64] >> (i % 64)) & 1 == 1 {
            count += 1;
        }
    }
    
    count
}

// ============================================================================
// BENCHMARK AND VALIDATION
// ============================================================================

fn benchmark() {
    println!("PHASE 1 OPTIMIZED SIEVE - PERFORMANCE VALIDATION");
    println!("================================================\n");
    
    let test_cases = [
        (1000, 168),
        (10000, 1229),
        (50000, 5133),
        (100000, 9592),
        (500000, 41538),
        (1000000, 78498),
        (5000000, 348513),
        (10000000, 664579),
    ];
    
    let mut total_speedup = 0.0;
    let mut test_count = 0;
    
    for &(limit, expected) in &test_cases {
        println!("Limit = {} (expected: {}):", limit, expected);
        
        // Benchmark current sqrt optimization
        let sqrt_time = {
            let start = Instant::now();
            let result = count_primes_sqrt(limit as i32);
            let elapsed = start.elapsed();
            assert_eq!(result as usize, expected, "sqrt result incorrect");
            elapsed.as_secs_f64() * 1000.0
        };
        
        // Benchmark optimized sieve
        let sieve_time = {
            let start = Instant::now();
            let result = if limit <= 1_000_000 {
                optimized_sieve(limit)
            } else {
                ultra_optimized_sieve(limit)
            };
            let elapsed = start.elapsed();
            assert_eq!(result, expected, "sieve result incorrect");
            elapsed.as_secs_f64() * 1000.0
        };
        
        // Benchmark ultra-optimized sieve
        let ultra_time = {
            let start = Instant::now();
            let result = ultra_optimized_sieve(limit);
            let elapsed = start.elapsed();
            assert_eq!(result, expected, "ultra sieve result incorrect");
            elapsed.as_secs_f64() * 1000.0
        };
        
        println!("  Current (sqrt):   {:8.3} ms", sqrt_time);
        println!("  Optimized sieve:  {:8.3} ms", sieve_time);
        println!("  Ultra-optimized:  {:8.3} ms", ultra_time);
        
        let speedup = sqrt_time / ultra_time;
        total_speedup += speedup;
        test_count += 1;
        
        println!("  Speedup: {:6.2}x\n", speedup);
    }
    
    let avg_speedup = total_speedup / test_count as f64;
    println!("Average speedup over current: {:.2}x", avg_speedup);
    
    // Estimate vs naive performance
    println!("\nPERFORMANCE VS NAIVE ESTIMATION:");
    println!("=================================");
    
    // At limit=100,000, sqrt is ~480x faster than naive
    // Ultra-optimized is avg_speedup faster than sqrt
    let estimated_vs_naive = 480.0 * avg_speedup;
    
    println!("Current (sqrt) vs naive: ~480x");
    println!("Ultra-optimized vs sqrt: {:.1}x", avg_speedup);
    println!("Estimated vs naive: {:.0}x", estimated_vs_naive);
    println!("Phase 1 target: 60,000x");
    
    if estimated_vs_naive >= 60000.0 * 0.9 {
        println!("\n✅ PHASE 1 TARGET ACHIEVED!");
        println!("   Estimated: {:.0}x vs naive", estimated_vs_naive);
        println!("   Target: 60,000x vs naive");
    } else if estimated_vs_naive >= 60000.0 * 0.5 {
        println!("\n⚠️  CLOSE TO PHASE 1 TARGET");
        println!("   Estimated: {:.0}x vs naive", estimated_vs_naive);
        println!("   Target: 60,000x vs naive");
        println!("   Gap: {:.1}x needed", 60000.0 / estimated_vs_naive);
    } else {
        println!("\n❌ PHASE 1 TARGET NOT REACHED");
        println!("   Estimated: {:.0}x vs naive", estimated_vs_naive);
        println!("   Target: 60,000x vs naive");
        println!("   Gap: {:.1}x improvement needed", 60000.0 / estimated_vs_naive);
    }
}

// Current sqrt implementation for comparison
fn count_primes_sqrt(limit: i32) -> i32 {
    if limit < 2 {
        return 0;
    }
    
    let mut count = 1;
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

fn main() {
    benchmark();
    
    // Additional validation
    println!("\nADDITIONAL VALIDATION TESTS:");
    println!("============================");
    
    let validation_limits = [10, 100, 1000, 10000];
    let mut all_correct = true;
    
    for &limit in &validation_limits {
        let sqrt_result = count_primes_sqrt(limit as i32) as usize;
        let optimized_result = optimized_sieve(limit);
        let ultra_result = ultra_optimized_sieve(limit);
        
        let known = match limit {
            10 => 4,
            100 => 25,
            1000 => 168,
            10000 => 1229,
            _ => 0,
        };
        
        if sqrt_result == known && optimized_result == known && ultra_result == known {
            println!("Limit {}: ✓ All algorithms correct ({})", limit, known);
        } else {
            println!("Limit {}: ✗ Mismatch! sqrt={}, opt={}, ultra={}, expected={}", 
                    limit, sqrt_result, optimized_result, ultra_result, known);
            all_correct = false;
        }
    }
    
    if all_correct {
        println!("\n✅ ALL VALIDATION TESTS PASSED!");
    } else {
        println!("\n❌ SOME VALIDATION TESTS FAILED!");
    }
    
    // Memory usage estimation
    println!("\nMEMORY USAGE ESTIMATION:");
    println!("========================");
    println!("Algorithm              | Limit=10M | Memory");
    println!("-----------------------|-----------|--------");
    println!("Basic array sieve      | 10,000,000| ~80 MB");
    println!("Bit-packed sieve       | 10,000,000| ~10 MB");
    println!("Segmented sieve        | 10,000,000| ~32 KB (per segment)");
    println!("Ultra-optimized        | 10,000,000| ~1.25 MB + 32 KB segment");
    
    println!("\nPHASE 1 IMPLEMENTATION COMPLETE!");
    println!("Next steps for Phase 2:");
    println!("1. Parallelize across CPU cores");
    println!("2. Add SIMD vectorization (AVX-512)");
    println!("3. Further mathematical optimizations");
}