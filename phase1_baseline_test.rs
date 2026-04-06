use std::time::Instant;

// Current sqrt optimization (baseline)
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

// Naive algorithm for comparison
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

// Sieve of Eratosthenes implementation
fn count_primes_sieve(limit: usize) -> usize {
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

// Sieve with wheel optimization (skip multiples of 2, 3, 5)
fn count_primes_sieve_wheel(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    if limit > 1 {
        is_prime[1] = false;
    }
    
    // Handle small primes manually
    if limit > 2 {
        for i in (4..limit).step_by(2) {
            is_prime[i] = false;
        }
    }
    
    if limit > 3 {
        for i in (9..limit).step_by(3) {
            is_prime[i] = false;
        }
    }
    
    if limit > 5 {
        for i in (25..limit).step_by(5) {
            is_prime[i] = false;
        }
    }
    
    // Main sieve starting from 7
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 7..=sqrt_limit {
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

fn main() {
    println!("Phase 1 Algorithm Implementation - Baseline Test");
    println!("===============================================\n");
    
    let test_limits = [1000, 10000, 50000, 100000];
    
    for &limit in &test_limits {
        println!("Testing limit = {}:", limit);
        
        // Naive algorithm
        let start = Instant::now();
        let naive_result = count_primes_naive(limit);
        let naive_time = start.elapsed();
        
        // Sqrt optimization (current)
        let start = Instant::now();
        let sqrt_result = count_primes_sqrt(limit);
        let sqrt_time = start.elapsed();
        
        // Sieve algorithm
        let start = Instant::now();
        let sieve_result = count_primes_sieve(limit as usize);
        let sieve_time = start.elapsed();
        
        // Sieve with wheel
        let start = Instant::now();
        let wheel_result = count_primes_sieve_wheel(limit as usize);
        let wheel_time = start.elapsed();
        
        println!("  Naive algorithm:");
        println!("    Result: {}, Time: {:?}", naive_result, naive_time);
        
        println!("  Sqrt optimization (current):");
        println!("    Result: {}, Time: {:?}", sqrt_result, sqrt_time);
        
        println!("  Sieve of Eratosthenes:");
        println!("    Result: {}, Time: {:?}", sieve_result, sieve_time);
        
        println!("  Sieve with wheel (2,3,5):");
        println!("    Result: {}, Time: {:?}", wheel_result, wheel_time);
        
        // Verify correctness
        if sqrt_result as usize == sieve_result && sieve_result == wheel_result {
            println!("  ✓ All algorithms agree!");
        } else {
            println!("  ✗ Results differ!");
            println!("    sqrt={}, sieve={}, wheel={}", sqrt_result, sieve_result, wheel_result);
        }
        
        // Calculate speedups
        let sqrt_vs_naive = naive_time.as_secs_f64() / sqrt_time.as_secs_f64();
        let sieve_vs_sqrt = sqrt_time.as_secs_f64() / sieve_time.as_secs_f64();
        let wheel_vs_sieve = sieve_time.as_secs_f64() / wheel_time.as_secs_f64();
        let sieve_vs_naive = naive_time.as_secs_f64() / sieve_time.as_secs_f64();
        let wheel_vs_naive = naive_time.as_secs_f64() / wheel_time.as_secs_f64();
        
        println!("  Speedup calculations:");
        println!("    Sqrt vs Naive: {:.2}x", sqrt_vs_naive);
        println!("    Sieve vs Sqrt: {:.2}x", sieve_vs_sqrt);
        println!("    Wheel vs Sieve: {:.2}x", wheel_vs_sieve);
        println!("    Sieve vs Naive: {:.2}x", sieve_vs_naive);
        println!("    Wheel vs Naive: {:.2}x", wheel_vs_naive);
        
        println!();
    }
}