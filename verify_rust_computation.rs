// Verify Rust actually computes by changing limit
fn count_primes(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    let sieve_size = (limit + 1) / 2; // Only odd numbers
    let mut sieve = vec![true; sieve_size];
    
    // 2 is prime
    let mut count = 1;
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    
    // Only check odd numbers
    for i in (3..=sqrt_limit).step_by(2) {
        let idx = i / 2;
        if sieve[idx] {
            // i is prime, mark multiples
            let start = i * i;
            for j in (start..=limit).step_by(i * 2) {
                sieve[j / 2] = false;
            }
        }
    }
    
    // Count remaining primes
    for i in 1..sieve_size {
        if sieve[i] {
            count += 1;
        }
    }
    
    count
}

fn main() {
    // Test with limit = 100 (should be 25 primes)
    let limit = 100;
    let count = count_primes(limit);
    println!("Primes up to {}: {}", limit, count);
    
    // Test with limit = 1000 (should be 168 primes)
    let limit2 = 1000;
    let count2 = count_primes(limit2);
    println!("Primes up to {}: {}", limit2, count2);
}