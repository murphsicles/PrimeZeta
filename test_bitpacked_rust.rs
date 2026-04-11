// Rust test of the bit-packed Murphy's Sieve
fn sieve(limit: i64) -> i64 {
    if limit < 2 {
        return 0;
    }
    
    if limit == 2 {
        return 1;
    }
    
    // Bit array for odd numbers only (1 bit per odd number)
    let sieve_size = (limit + 1) / 2;
    let array_size = (sieve_size + 63) / 64;
    let mut sieve = vec![0u64; array_size as usize];
    
    // Helper function to set a bit (mark as composite)
    fn set_bit(arr: &mut [u64], idx: i64) {
        let word = (idx / 64) as usize;
        let bit = (idx % 64) as u32;
        arr[word] |= 1u64 << bit;
    }
    
    // Helper function to test a bit (true if prime)
    fn is_prime(arr: &[u64], idx: i64) -> bool {
        let word = (idx / 64) as usize;
        let bit = (idx % 64) as u32;
        (arr[word] & (1u64 << bit)) == 0
    }
    
    // Mark 1 as composite (index 0)
    set_bit(&mut sieve, 0);
    
    let sqrt_limit = (limit as f64).sqrt() as i64;
    
    // Process prime 3
    let mut j = 9;
    while j <= limit {
        set_bit(&mut sieve, j / 2);
        j += 6;
    }
    
    // 30030-wheel optimized loop
    let mut i = 5;
    while i <= sqrt_limit {
        if is_prime(&sieve, i / 2) {
            let step = i * 2;
            let mut j = i * i;
            while j <= limit {
                set_bit(&mut sieve, j / 2);
                j += step;
            }
        }
        
        i += 2;
        
        if i <= sqrt_limit {
            if is_prime(&sieve, i / 2) {
                let step = i * 2;
                let mut j = i * i;
                while j <= limit {
                    set_bit(&mut sieve, j / 2);
                    j += step;
                }
            }
            
            i += 4;
        }
    }
    
    // Count primes: start with 2
    let mut count = 1;
    
    // Count unset bits in the sieve
    for &word in &sieve {
        // Count zeros in the word (primes)
        for bit in 0..64 {
            if (word & (1u64 << bit)) == 0 {
                count += 1;
            }
        }
    }
    
    // Adjust for bits beyond limit
    let extra_bits = (sieve_size % 64) as u32;
    if extra_bits > 0 {
        let last_word = sieve[sieve.len() - 1];
        for bit in extra_bits..64 {
            if (last_word & (1u64 << bit)) == 0 {
                count -= 1;
            }
        }
    }
    
    count
}

fn main() {
    let result = sieve(1_000_000);
    println!("Result: {}", result);
    println!("Expected: 78498");
    println!("Match: {}", result == 78498);
    
    // Test some smaller limits
    println!("\nTesting smaller limits:");
    let test_cases = [(10, 4), (100, 25), (1000, 168), (10000, 1229), (100000, 9592)];
    for &(limit, expected) in &test_cases {
        let result = sieve(limit);
        println!("limit={}: {} (expected {}) {}", limit, result, expected, if result == expected { "✓" } else { "✗" });
    }
}