use std::time::Instant;

fn count_primes_optimized(limit: i32) -> i32 {
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

fn main() {
    println!("PrimeZeta Algorithm - Performance Comparison");
    println!("===========================================\n");
    
    let test_limits = [1000, 10000, 50000];
    
    for &limit in &test_limits {
        println!("Testing limit = {}:", limit);
        
        // Naive algorithm
        let start = Instant::now();
        let naive_result = count_primes_naive(limit);
        let naive_time = start.elapsed();
        
        // Optimized algorithm
        let start = Instant::now();
        let optimized_result = count_primes_optimized(limit);
        let optimized_time = start.elapsed();
        
        println!("  Naive algorithm:");
        println!("    Result: {}", naive_result);
        println!("    Time: {:?}", naive_time);
        
        println!("  Optimized algorithm:");
        println!("    Result: {}", optimized_result);
        println!("    Time: {:?}", optimized_time);
        
        if naive_result == optimized_result {
            println!("  ✓ Results match!");
        } else {
            println!("  ✗ Results differ!");
        }
        
        let speedup = naive_time.as_secs_f64() / optimized_time.as_secs_f64();
        println!("  Speedup: {:.2}x\n", speedup);
    }
}