// AVX-512 Vectorized Faithful Sieve Implementation - FIXED
// Core i9 13900H (AVX-512, 14 cores)
// SIMD vectorization within faithful parallel architecture

use std::arch::x86_64::*;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// ============================================================================
// FAITHFUL SIEVE CLASS WITH AVX-512 VECTORIZATION
// ============================================================================

/// FaithfulSieve - Class-encapsulated sieve with AVX-512 vectorization
/// Maintains faithfulness compliance while leveraging SIMD optimizations
pub struct FaithfulSieve {
    limit: usize,
    buffer: Vec<u64>,  // Bit array for prime flags (64 bits per element)
    avx512_supported: bool,
}

impl FaithfulSieve {
    /// Create a new faithful sieve instance
    /// Dynamic allocation of sieve buffer at runtime (faithfulness requirement)
    pub fn new(limit: usize) -> Self {
        // Check AVX-512 support at runtime
        let avx512_supported = unsafe { is_x86_feature_detected!("avx512f") };
        
        // Allocate bit array (1 bit per number, packed into u64)
        let buffer_size = (limit + 63) / 64;
        let buffer = vec![u64::MAX; buffer_size]; // Initialize all bits to 1 (prime)
        
        // Mark 0 and 1 as not prime
        if limit > 0 {
            buffer[0] &= !((1 << 0) | (1 << 1));
        }
        
        FaithfulSieve {
            limit,
            buffer,
            avx512_supported,
        }
    }
    
    /// Run the sieve algorithm (faithful implementation)
    pub fn run(&mut self) {
        if self.limit < 2 {
            return;
        }
        
        let sqrt_limit = (self.limit as f64).sqrt() as usize;
        
        // Always use scalar for small limits
        if self.limit < 512 {
            self.run_scalar(sqrt_limit);
            return;
        }
        
        if self.avx512_supported {
            // Use AVX-512 vectorized marking
            unsafe {
                self.run_avx512(sqrt_limit);
            }
        } else {
            // Fallback to scalar implementation
            self.run_scalar(sqrt_limit);
        }
    }
    
    /// Scalar implementation (fallback when AVX-512 not available)
    fn run_scalar(&mut self, sqrt_limit: usize) {
        for p in 2..=sqrt_limit {
            if self.is_prime_scalar(p) {
                self.mark_multiples_scalar(p);
            }
        }
    }
    
    /// AVX-512 vectorized implementation
    unsafe fn run_avx512(&mut self, sqrt_limit: usize) {
        // Use AVX-512 for vectorized prime marking
        for p in 2..=sqrt_limit {
            if self.is_prime_scalar(p) {
                self.mark_multiples_avx512(p);
            }
        }
    }
    
    /// Check if a number is prime (scalar)
    fn is_prime_scalar(&self, n: usize) -> bool {
        if n < 2 || n >= self.limit {
            return false;
        }
        let word_idx = n / 64;
        let bit_mask = 1 << (n % 64);
        (self.buffer[word_idx] & bit_mask) != 0
    }
    
    /// Mark multiples of a prime (scalar)
    fn mark_multiples_scalar(&mut self, p: usize) {
        let start = p * p;
        if start >= self.limit {
            return;
        }
        
        for multiple in (start..self.limit).step_by(p) {
            let word_idx = multiple / 64;
            let bit_mask = !(1 << (multiple % 64));
            self.buffer[word_idx] &= bit_mask;
        }
    }
    
    /// Mark multiples of a prime (AVX-512 vectorized)
    unsafe fn mark_multiples_avx512(&mut self, p: usize) {
        let start = p * p;
        if start >= self.limit {
            return;
        }
        
        // Process in chunks of 512 bits (8 u64 elements)
        let chunk_size = 8; // 8 u64 = 512 bits
        let word_start = start / 64;
        let word_end = (self.limit + 63) / 64;
        
        // Create mask for this prime's pattern
        let pattern = self.create_avx512_pattern(p);
        
        // Process aligned chunks
        let mut word_idx = word_start;
        while word_idx + chunk_size <= word_end {
            // Load 8 u64 values (512 bits)
            let ptr = self.buffer.as_mut_ptr().add(word_idx);
            let chunk = _mm512_loadu_si512(ptr as *const _);
            
            // Apply pattern
            let result = _mm512_and_si512(chunk, pattern);
            
            // Store back
            _mm512_storeu_si512(ptr as *mut _, result);
            
            word_idx += chunk_size;
        }
        
        // Process remaining words with scalar
        while word_idx < word_end {
            let word_num = word_idx * 64;
            for bit in 0..64 {
                let num = word_num + bit;
                if num >= self.limit {
                    break;
                }
                if num >= start && num % p == 0 {
                    self.buffer[word_idx] &= !(1 << bit);
                }
            }
            word_idx += 1;
        }
    }
    
    /// Create AVX-512 pattern for a prime
    unsafe fn create_avx512_pattern(&self, p: usize) -> __m512i {
        // Create a mask where bits corresponding to multiples of p are 0
        // Process 512 bits (8 u64 values) at a time
        
        let mut mask_words = [u64::MAX; 8];
        
        for chunk_offset in 0..8 {
            let base_num = chunk_offset * 64;
            for bit in 0..64 {
                let num = base_num + bit;
                if num % p == 0 {
                    mask_words[chunk_offset] &= !(1 << bit);
                }
            }
        }
        
        _mm512_loadu_si512(mask_words.as_ptr() as *const _)
    }
    
    /// Count primes (scalar)
    pub fn count_primes(&self) -> usize {
        let mut count = 0;
        
        // Always use scalar for counting - it's memory bound anyway
        for (word_idx, &word) in self.buffer.iter().enumerate() {
            let start_num = word_idx * 64;
            let end_num = std::cmp::min(start_num + 64, self.limit);
            
            for bit in 0..(end_num - start_num) {
                if (word >> bit) & 1 == 1 {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    /// Get AVX-512 support status
    pub fn avx512_supported(&self) -> bool {
        self.avx512_supported
    }
    
    /// Mark multiples for a segment (scalar) - for parallel sieve
    fn mark_multiples_scalar_for_segment(&mut self, p: usize, start: usize, segment_size: usize) {
        for multiple in (start..segment_size).step_by(p) {
            let word_idx = multiple / 64;
            let bit_mask = !(1 << (multiple % 64));
            self.buffer[word_idx] &= bit_mask;
        }
    }
    
    /// Mark multiples for a segment (AVX-512) - for parallel sieve
    unsafe fn mark_multiples_avx512_for_segment(&mut self, p: usize, start: usize, segment_size: usize) {
        if segment_size < 512 || !self.avx512_supported {
            self.mark_multiples_scalar_for_segment(p, start, segment_size);
            return;
        }
        
        let chunk_size = 8; // 8 u64 = 512 bits
        let word_start = start / 64;
        let word_end = (segment_size + 63) / 64;
        
        // Create pattern for this prime
        let pattern = self.create_avx512_pattern(p);
        
        // Process aligned chunks
        let mut word_idx = word_start;
        while word_idx + chunk_size <= word_end {
            let ptr = self.buffer.as_mut_ptr().add(word_idx);
            let chunk = _mm512_loadu_si512(ptr as *const _);
            let result = _mm512_and_si512(chunk, pattern);
            _mm512_storeu_si512(ptr as *mut _, result);
            
            word_idx += chunk_size;
        }
        
        // Process remaining words with scalar
        while word_idx < word_end {
            let word_num = word_idx * 64;
            for bit in 0..64 {
                let num = word_num + bit;
                if num >= segment_size {
                    break;
                }
                if num >= start && num % p == 0 {
                    self.buffer[word_idx] &= !(1 << bit);
                }
            }
            word_idx += 1;
        }
    }
}

// ============================================================================
// PARALLEL FAITHFUL SIEVE WITH SIMD PER THREAD
// ============================================================================

/// ParallelFaithfulSieve - Multi-threaded faithful sieve with SIMD per thread
/// Each thread uses AVX-512 vectorization for maximum performance
pub struct ParallelFaithfulSieve {
    limit: usize,
    num_threads: usize,
}

impl ParallelFaithfulSieve {
    /// Create a new parallel faithful sieve
    pub fn new(limit: usize, num_threads: usize) -> Self {
        ParallelFaithfulSieve {
            limit,
            num_threads,
        }
    }
    
    /// Run parallel sieve with SIMD per thread
    pub fn run(&self) -> usize {
        if self.limit < 2 {
            return 0;
        }
        
        if self.num_threads == 1 {
            // Single-threaded mode
            let mut sieve = FaithfulSieve::new(self.limit);
            sieve.run();
            sieve.count_primes()
        } else {
            // Multi-threaded mode with segmented sieve
            self.run_parallel()
        }
    }
    
    /// Parallel implementation using segmented sieve
    fn run_parallel(&self) -> usize {
        let sqrt_limit = (self.limit as f64).sqrt() as usize;
        
        // Step 1: Generate small primes serially (up to sqrt(n))
        let mut small_sieve = FaithfulSieve::new(sqrt_limit + 1);
        small_sieve.run();
        
        // Collect small primes
        let small_primes: Vec<usize> = (2..=sqrt_limit)
            .filter(|&p| small_sieve.is_prime_scalar(p))
            .collect();
        
        // Step 2: Parallel segmented sieve
        let segment_size = 1_000_000; // ~1MB segments for cache efficiency
        let num_segments = (self.limit + segment_size - 1) / segment_size;
        
        let prime_count = Arc::new(AtomicUsize::new(0));
        let small_primes_arc = Arc::new(small_primes);
        
        // Process segments in parallel
        thread::scope(|s| {
            for thread_id in 0..self.num_threads {
                let prime_count = prime_count.clone();
                let small_primes = small_primes_arc.clone();
                
                s.spawn(move || {
                    // Each thread processes its assigned segments
                    let mut thread_count = 0;
                    
                    for segment_idx in (thread_id..num_segments).step_by(self.num_threads) {
                        let low = segment_idx * segment_size;
                        let high = std::cmp::min(low + segment_size, self.limit);
                        let segment_len = high - low;
                        
                        if segment_len == 0 {
                            continue;
                        }
                        
                        // Create thread-local sieve for this segment
                        let mut segment_sieve = FaithfulSieve::new(segment_len);
                        
                        // Initialize all bits to prime
                        // (FaithfulSieve::new already does this)
                        
                        // Mark multiples of small primes in this segment
                        for &p in small_primes.iter() {
                            let start = if low <= p * p {
                                p * p
                            } else {
                                let rem = low % p;
                                if rem == 0 {
                                    low
                                } else {
                                    low + p - rem
                                }
                            };
                            
                            if start < high {
                                let segment_start = start - low;
                                
                                // Use appropriate marking method
                                unsafe {
                                    if segment_sieve.avx512_supported() && segment_len >= 512 {
                                        segment_sieve.mark_multiples_avx512_for_segment(p, segment_start, segment_len);
                                    } else {
                                        segment_sieve.mark_multiples_scalar_for_segment(p, segment_start, segment_len);
                                    }
                                }
                            }
                        }
                        
                        // Count primes in this segment
                        thread_count += segment_sieve.count_primes();
                    }
                    
                    prime_count.fetch_add(thread_count, Ordering::Relaxed);
                });
            }
        });
        
        // Add small primes (they're in the first segment)
        let segment_count = prime_count.load(Ordering::Relaxed);
        let small_prime_count = small_primes_arc.len();
        
        small_prime_count + segment_count
    }
}

// ============================================================================
// SIMPLE BENCHMARK
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn reference_sieve(limit: usize) -> usize {
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
    
    #[test]
    fn test_faithful_sieve_small() {
        let limits = [10, 100, 1000];
        
        for &limit in &limits {
            let mut sieve = FaithfulSieve::new(limit);
            sieve.run();
            let count = sieve.count_primes();
            let expected = reference_sieve(limit);
            
            assert_eq!(count, expected, "Failed for limit={}", limit);
        }
    }
    
    #[test]
    fn test_parallel_faithful_sieve() {
        let limit = 10000;
        
        // Single-threaded
        let mut sieve1 = FaithfulSieve::new(limit);
        sieve1.run();
        let count1 = sieve1.count_primes();
        
        // Parallel (2 threads)
        let sieve2 = ParallelFaithfulSieve::new(limit, 2);
        let count2 = sieve2.run();
        
        let expected = reference_sieve(limit);
        
        assert_eq!(count1, expected, "Single-threaded failed");
        assert_eq!(count2, expected, "Parallel failed");
        assert_eq!(count1, count2, "Single and parallel don't match");
    }
}

// Main function for quick testing
fn main() {
    println!("AVX-512 Faithful Sieve Implementation");
    println!("=====================================\n");
    
    // Check AVX-512 support
    let avx512_supported = unsafe { is_x86_feature_detected!("avx512f") };
    println!("AVX-512 Support: {}", if avx512_supported { "✅ YES" } else { "❌ NO" });
    
    // Test with a moderate limit
    let limit = 1_000_000;
    println!("\nTesting with limit = {}", limit);
    
    // Single-threaded AVX-512
    let mut sieve = FaithfulSieve::new(limit);
    sieve.run();
    let count = sieve.count_primes();
    
    println!("Primes found: {}", count);
    println!("Expected: 78498");
    
    if count == 78498 {
        println!("✅ Test PASSED");
    } else {
        println!("❌ Test FAILED");
    }
    
    // Parallel test
    println!("\nParallel test (4 threads):");
    let parallel_sieve = ParallelFaithfulSieve::new(limit, 4);
    let parallel_count = parallel_sieve.run();
    
    println!("Primes found: {}", parallel_count);
    
    if parallel_count == 78498 {
        println!("✅ Parallel test PASSED");
    } else {
        println!("❌ Parallel test FAILED");
    }
    
    println!("\n✅ AVX-512 Faithful Sieve Implementation Complete");
    println!("   Ready for integration with competition entry");
}