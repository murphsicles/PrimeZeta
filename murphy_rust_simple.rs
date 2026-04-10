// Simple Murphy's Sieve in Rust for competition
// No AVX-512, works on stable Rust

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
    let limit = 1_000_000;
    let count = count_primes(limit);
    println!("{}", count);
}