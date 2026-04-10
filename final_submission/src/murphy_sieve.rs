// Murphy's Sieve Implementation for Competition
// Simple, efficient implementation that actually computes primes
// No external dependencies, works on stable Rust

/// Count primes up to limit using Murphy's Sieve (Sieve of Eratosthenes variant)
/// 
/// # Arguments
/// * `limit` - Maximum number to check (inclusive)
/// 
/// # Returns
/// Number of primes ≤ limit
/// 
/// # Examples
/// ```
/// assert_eq!(count_primes(10), 4);  // Primes: 2, 3, 5, 7
/// assert_eq!(count_primes(100), 25);
/// assert_eq!(count_primes(1_000_000), 78_498);
/// ```
pub fn count_primes(limit: usize) -> usize {
    if limit < 2 {
        return 0;
    }
    
    // Only store odd numbers (2 is handled separately)
    let sieve_size = (limit + 1) / 2;
    let mut sieve = vec![true; sieve_size];
    
    // 2 is prime
    let mut count = 1;
    
    let sqrt_limit = (limit as f64).sqrt() as usize;
    
    // Only check odd numbers
    for i in (3..=sqrt_limit).step_by(2) {
        let idx = i / 2;
        if sieve[idx] {
            // i is prime, mark multiples starting from i*i
            let start = i * i;
            for j in (start..=limit).step_by(i * 2) {
                sieve[j / 2] = false;
            }
        }
    }
    
    // Count remaining primes (odd numbers only)
    for i in 1..sieve_size {
        if sieve[i] {
            count += 1;
        }
    }
    
    count
}

/// Competition entry point - infinite loop version
/// The competition harness runs this repeatedly for 5 seconds
/// Each complete execution counts as one "pass"
pub fn competition_main() -> usize {
    // For competition, we compute primes up to 1,000,000
    count_primes(1_000_000)
}

fn main() {
    // Standard execution prints the result
    let count = competition_main();
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_small_limits() {
        assert_eq!(count_primes(0), 0);
        assert_eq!(count_primes(1), 0);
        assert_eq!(count_primes(2), 1);
        assert_eq!(count_primes(10), 4);
        assert_eq!(count_primes(100), 25);
    }
    
    #[test]
    fn test_known_values() {
        assert_eq!(count_primes(1_000), 168);
        assert_eq!(count_primes(10_000), 1_229);
        assert_eq!(count_primes(100_000), 9_592);
        assert_eq!(count_primes(1_000_000), 78_498);
    }
    
    #[test]
    fn test_edge_cases() {
        // Test powers of 2
        assert_eq!(count_primes(16), 6);  // Primes: 2,3,5,7,11,13
        assert_eq!(count_primes(32), 11);
        
        // Test prime numbers themselves
        assert_eq!(count_primes(13), 6);  // Primes: 2,3,5,7,11,13
        assert_eq!(count_primes(17), 7);  // Plus 17
    }
}