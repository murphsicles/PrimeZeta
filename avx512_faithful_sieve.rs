// AVX-512 Vectorized Faithful Sieve Implementation
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
    buffer: Vec<u8>,  // Bit array for prime flags (1 = prime, 0 = composite)
    avx512_supported: bool,
}

impl FaithfulSieve {
    /// Create a new faithful sieve instance
    /// Dynamic allocation of sieve buffer at runtime (faithfulness requirement)
    pub fn new(limit: usize) -> Self {
        // Check AVX-512 support at runtime
        let avx512_supported = unsafe { is_x86_feature_detected!("avx512f") };
        
        // Allocate bit array (8 bits per byte)
        let buffer_size = (limit + 7) / 8;
        let mut buffer = vec![0xFFu8; buffer_size]; // Initialize all bits to 1 (prime)
        
        // Mark 0 and 1 as not prime
        if limit > 0 {
            buffer[0] &= 0xFC; // Clear bits 0 and 1
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
        
        if self.avx512_supported && self.limit >= 512 {
            // Use AVX-512 vectorized marking for large limits
            self.run_avx512(sqrt_limit);
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
            if self.is_prime_avx512(p) {
                self.mark_multiples_avx512(p);
            }
        }
    }
    
    /// Check if a number is prime (scalar)
    fn is_prime_scalar(&self, n: usize) -> bool {
        if n < 2 || n >= self.limit {
            return false;
        }
        let byte_idx = n / 8;
        let bit_mask = 1 << (n % 8);
        (self.buffer[byte_idx] & bit_mask) != 0
    }
    
    /// Check if a number is prime (AVX-512 optimized)
    unsafe fn is_prime_avx512(&self, n: usize) -> bool {
        if n < 2 || n >= self.limit {
            return false;
        }
        let byte_idx = n / 8;
        let bit_mask = 1 << (n % 8);
        (self.buffer[byte_idx] & bit_mask) != 0
    }
    
    /// Mark multiples of a prime (scalar)
    fn mark_multiples_scalar(&mut self, p: usize) {
        let start = p * p;
        if start >= self.limit {
            return;
        }
        
        for multiple in (start..self.limit).step_by(p) {
            let byte_idx = multiple / 8;
            let bit_mask = !(1 << (multiple % 8));
            self.buffer[byte_idx] &= bit_mask;
        }
    }
    
    /// Mark multiples of a prime (AVX-512 vectorized)
    unsafe fn mark_multiples_avx512(&mut self, p: usize) {
        let start = p * p;
        if start >= self.limit {
            return;
        }
        
        // Use AVX-512 for vectorized marking
        // Process 512 bits (64 bytes) at a time
        let chunk_size = 512;
        let byte_start = start / 8;
        let byte_end = (self.limit + 7) / 8;
        
        // Create mask pattern for this prime
        let pattern = self.create_avx512_pattern(p);
        
        // Process in chunks
        let mut byte_idx = byte_start;
        while byte_idx < byte_end {
            let remaining = byte_end - byte_idx;
            let process_size = if remaining >= 64 { 64 } else { remaining };
            
            if process_size == 64 {
                // Full AVX-512 register
                self.process_avx512_chunk(byte_idx, pattern);
            } else {
                // Partial chunk - fallback to scalar
                self.process_partial_chunk(byte_idx, process_size, p);
            }
            
            byte_idx += process_size;
        }
    }
    
    /// Create AVX-512 pattern for marking multiples
    unsafe fn create_avx512_pattern(&self, p: usize) -> __m512i {
        // Create a pattern where bits corresponding to multiples of p are 0
        let mut pattern_bytes = [0xFFu8; 64];
        
        // Calculate which bits within 512-bit chunk should be cleared
        let byte_offset = 0;
        for bit in 0..512 {
            let global_bit = byte_offset * 8 + bit;
            if global_bit % p == 0 {
                let byte_idx = bit / 8;
                let bit_idx = bit % 8;
                pattern_bytes[byte_idx] &= !(1 << bit_idx);
            }
        }
        
        _mm512_loadu_si512(pattern_bytes.as_ptr() as *const _)
    }
    
    /// Process 64-byte chunk with AVX-512
    unsafe fn process_avx512_chunk(&mut self, byte_idx: usize, pattern: __m512i) {
        let ptr = self.buffer.as_mut_ptr().add(byte_idx);
        let chunk = _mm512_loadu_si512(ptr as *const _);
        let result = _mm512_and_si512(chunk, pattern);
        _mm512_storeu_si512(ptr as *mut _, result);
    }
    
    /// Process partial chunk (scalar fallback)
    fn process_partial_chunk(&mut self, byte_idx: usize, size: usize, p: usize) {
        for offset in 0..size {
            let global_byte = byte_idx + offset;
            let start_bit = global_byte * 8;
            
            // Check each bit in this byte
            for bit in 0..8 {
                let number = start_bit + bit;
                if number >= self.limit {
                    break;
                }
                
                if number >= p * p && number % p == 0 {
                    // Mark as composite
                    self.buffer[global_byte] &= !(1 << bit);
                }
            }
        }
    }
    
    /// Count primes (scalar)
    pub fn count_primes(&self) -> usize {
        let mut count = 0;
        
        if self.avx512_supported && self.buffer.len() >= 64 {
            unsafe {
                count = self.count_primes_avx512();
            }
        } else {
            count = self.count_primes_scalar();
        }
        
        count
    }
    
    /// Count primes (scalar implementation)
    fn count_primes_scalar(&self) -> usize {
        let mut count = 0;
        
        for (byte_idx, &byte) in self.buffer.iter().enumerate() {
            let start_num = byte_idx * 8;
            for bit in 0..8 {
                let num = start_num + bit;
                if num >= self.limit {
                    break;
                }
                if (byte >> bit) & 1 == 1 {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    /// Count primes (AVX-512 vectorized population count)
    unsafe fn count_primes_avx512(&self) -> usize {
        let mut total_count = 0;
        let chunk_size = 64; // 512 bits
        
        // Process in chunks
        let mut byte_idx = 0;
        while byte_idx < self.buffer.len() {
            let remaining = self.buffer.len() - byte_idx;
            let process_size = if remaining >= chunk_size { chunk_size } else { remaining };
            
            if process_size == chunk_size {
                // AVX-512 population count
                let ptr = self.buffer.as_ptr().add(byte_idx);
                let chunk = _mm512_loadu_si512(ptr as *const _);
                
                // Count set bits using AVX-512 popcount
                let popcnt = _mm512_popcnt_epi64(chunk);
                
                // Sum the counts (simplified - actual implementation would extract and sum)
                let mut counts = [0u64; 8];
                _mm512_storeu_si512(counts.as_mut_ptr() as *mut _, popcnt);
                
                for &c in &counts {
                    total_count += c as usize;
                }
            } else {
                // Partial chunk - scalar fallback
                total_count += self.count_partial_chunk(byte_idx, process_size);
            }
            
            byte_idx += process_size;
        }
        
        total_count
    }
    
    /// Count primes in partial chunk (scalar)
    fn count_partial_chunk(&self, byte_idx: usize, size: usize) -> usize {
        let mut count = 0;
        
        for offset in 0..size {
            let byte = self.buffer[byte_idx + offset];
            count += byte.count_ones() as usize;
        }
        
        count
    }
    
    /// Get AVX-512 support status
    pub fn avx512_supported(&self) -> bool {
        self.avx512_supported
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
        let segment_size = 1_000_000; // 1MB segments for cache efficiency
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
                        
                        // Create thread-local sieve with AVX-512
                        let mut segment_sieve = FaithfulSieve::new(high - low);
                        
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
                                // Use AVX-512 vectorized marking within thread
                                unsafe {
                                    if segment_sieve.avx512_supported() {
                                        segment_sieve.mark_multiples_avx512_for_segment(p, start - low, high - low);
                                    } else {
                                        segment_sieve.mark_multiples_scalar_for_segment(p, start - low, high - low);
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
        
        // Add small primes (counted in segments)
        let segment_count = prime_count.load(Ordering::Relaxed);
        
        // Count small primes separately (they're in first segment)
        let small_prime_count = small_primes_arc.len();
        
        small_prime_count + segment_count
    }
}

// Extension methods for FaithfulSieve to support segmented marking
impl FaithfulSieve {
    /// Mark multiples for a segment (scalar)
    fn mark_multiples_scalar_for_segment(&mut self, p: usize, start: usize, segment_size: usize) {
        for multiple in (start..segment_size).step_by(p) {
            let byte_idx = multiple / 8;
            let bit_mask = !(1 << (multiple % 8));
            self.buffer[byte_idx] &= bit_mask;
        }
    }
    
    /// Mark multiples for a segment (AVX-512)
    unsafe fn mark_multiples_avx512_for_segment(&mut self, p: usize, start: usize, segment_size: usize) {
        let chunk_size = 512;
        let byte_start = start / 8;
        let byte_end = (segment_size + 7) / 8;
        
        // Create pattern for this prime
        let pattern = self.create_avx512_pattern(p);
        
        // Process in chunks
        let mut byte_idx = byte_start;
        while byte_idx < byte_end {
            let remaining = byte_end - byte_idx;
            let process_size = if remaining >= 64 { 64 } else { remaining };
            
            if process_size == 64 {
                self.process_avx512_chunk(byte_idx, pattern);
            } else {
                self.process_partial_chunk(byte_idx, process_size, p);
            }
            
            byte_idx += process_size;
        }
    }
}

// ============================================================================
// BENCHMARK AND VALIDATION
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    
    /// Reference implementation for validation
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
    fn test_faithful_sieve_correctness() {
        let test_limits = [10, 100, 1000, 10000, 50000];
        
        for &limit in &test_limits {
            let mut sieve = FaithfulSieve::new(limit);
            sieve.run();
            let count = sieve.count_primes();
            
            let expected = reference_sieve(limit);
            assert_eq!(count, expected, "Failed for limit={}", limit);
        }
    }
    
    #[test]
    fn test_parallel_faithful_sieve_correctness() {
        let test_limits = [1000, 10000, 50000];
        
        for &limit in &test_limits {
            // Test with 1 thread (should match single-threaded)
            let sieve1 =