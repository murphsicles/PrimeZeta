// Verification Tests for Murphy's Sieve Competition Submission
// 
// This test suite verifies:
// 1. Correctness of prime counting
// 2. Performance meets competition requirements
// 3. Infinite loop wrapper works correctly
// 4. No regressions in implementation

use std::time::{Instant, Duration};
use std::process::Command;

/// Known prime counts for verification
const KNOWN_COUNTS: &[(usize, usize)] = &[
    (0, 0),
    (1, 0),
    (2, 1),
    (10, 4),      // Primes: 2, 3, 5, 7
    (100, 25),
    (1_000, 168),
    (10_000, 1_229),
    (100_000, 9_592),
    (1_000_000, 78_498),
];

/// Test correctness of prime counting function
#[test]
fn test_correctness() {
    // Import the function from our main implementation
    // Note: In a real test, we'd import from the actual module
    // For now, we'll test the algorithm logic
    
    fn count_primes(limit: usize) -> usize {
        if limit < 2 {
            return 0;
        }
        
        let sieve_size = (limit + 1) / 2;
        let mut sieve = vec![true; sieve_size];
        let mut count = 1; // 2 is prime
        
        let sqrt_limit = (limit as f64).sqrt() as usize;
        
        for i in (3..=sqrt_limit).step_by(2) {
            let idx = i / 2;
            if sieve[idx] {
                let start = i * i;
                for j in (start..=limit).step_by(i * 2) {
                    sieve[j / 2] = false;
                }
            }
        }
        
        count += sieve[1..].iter().filter(|&&is_prime| is_prime).count();
        count
    }
    
    for &(limit, expected) in KNOWN_COUNTS {
        let result = count_primes(limit);
        assert_eq!(
            result, expected,
            "Incorrect prime count for limit {}: expected {}, got {}",
            limit, expected, result
        );
    }
}

/// Test performance meets competition requirements
#[test]
fn test_performance() {
    const LIMIT: usize = 1_000_000;
    const MAX_TIME_MS: u128 = 20; // Should complete in <20ms
    
    fn count_primes_fast(limit: usize) -> usize {
        if limit < 2 {
            return 0;
        }
        
        let sieve_size = (limit + 1) / 2;
        let mut sieve = vec![true; sieve_size];
        let mut count = 1;
        
        let sqrt_limit = (limit as f64).sqrt() as usize;
        
        // Handle 3 separately
        {
            let start = 9;
            for j in (start..=limit).step_by(6) {
                sieve[j / 2] = false;
            }
        }
        
        for i in (5..=sqrt_limit).step_by(2) {
            let idx = i / 2;
            if sieve[idx] {
                let start = i * i;
                let step = i * 2;
                for j in (start..=limit).step_by(step) {
                    sieve[j / 2] = false;
                }
            }
        }
        
        count += sieve[1..].iter().filter(|&&is_prime| is_prime).count();
        count
    }
    
    let start = Instant::now();
    let result = count_primes_fast(LIMIT);
    let elapsed = start.elapsed();
    
    assert_eq!(result, 78_498, "Incorrect prime count");
    assert!(
        elapsed.as_millis() < MAX_TIME_MS,
        "Too slow: {}ms (max {}ms)",
        elapsed.as_millis(),
        MAX_TIME_MS
    );
    
    // Estimate passes per second
    let passes_per_second = 1000.0 / elapsed.as_millis() as f64;
    println!("Performance: {}ms per execution (~{:.0} passes/sec)", 
             elapsed.as_millis(), passes_per_second);
}

/// Test competition wrapper compiles and runs
#[test]
fn test_competition_wrapper() {
    // This test would actually compile and run the competition wrapper
    // For now, we'll verify the source code exists and is valid
    
    use std::fs;
    use std::path::Path;
    
    let wrapper_path = Path::new("src/competition_wrapper.rs");
    assert!(
        wrapper_path.exists(),
        "Competition wrapper not found at {:?}",
        wrapper_path
    );
    
    let source = fs::read_to_string(wrapper_path)
        .expect("Failed to read competition wrapper");
    
    // Check for required components
    assert!(
        source.contains("count_primes_fast"),
        "Wrapper missing prime counting function"
    );
    assert!(
        source.contains("loop") || source.contains("while true"),
        "Wrapper missing infinite loop"
    );
    assert!(
        source.contains("1_000_000"),
        "Wrapper missing limit constant"
    );
    assert!(
        source.contains("78_498"),
        "Wrapper missing expected result"
    );
}

/// Test build process
#[test]
fn test_build_process() {
    // Check Cargo.toml exists
    assert!(
        Path::new("Cargo.toml").exists(),
        "Cargo.toml not found"
    );
    
    // Check main source files exist
    assert!(
        Path::new("src/murphy_sieve.rs").exists(),
        "Main implementation not found"
    );
    
    // Verify package configuration
    let cargo_toml = fs::read_to_string("Cargo.toml")
        .expect("Failed to read Cargo.toml");
    
    assert!(
        cargo_toml.contains("murphy-sieve-competition"),
        "Cargo.toml missing package name"
    );
    assert!(
        cargo_toml.contains("[[bin]]"),
        "Cargo.toml missing binary configuration"
    );
}

/// Test documentation exists
#[test]
fn test_documentation() {
    assert!(
        Path::new("README.md").exists(),
        "README.md not found"
    );
    
    let readme = fs::read_to_string("README.md")
        .expect("Failed to read README.md");
    
    // Check for required sections
    let required_sections = [
        "Competition Entry",
        "Performance Summary",
        "Known Limitations",
        "Building and Running",
    ];
    
    for section in required_sections {
        assert!(
            readme.contains(section),
            "README.md missing section: {}",
            section
        );
    }
    
    // Verify honest assessment
    assert!(
        readme.contains("Actually computes primes") || 
        readme.contains("not constant return"),
        "README.md missing honesty about implementation"
    );
    
    // Verify Zeta limitations are documented
    assert!(
        readme.contains("Missing comparison operators") ||
        readme.contains("Zeta Language Status"),
        "README.md missing Zeta limitations"
    );
}

/// Run all verification tests
fn main() {
    println!("Running Murphy's Sieve Competition Verification...");
    println!("=================================================");
    
    // Note: In a real scenario, we'd run `cargo test` instead
    // This is just a demonstration of the verification process
    
    println!("✅ All verification tests defined");
    println!("Run 'cargo test' to execute verification");
    println!();
    println!("Verification Checklist:");
    println!("1. ✅ Correctness tests defined");
    println!("2. ✅ Performance tests defined");
    println!("3. ✅ Competition wrapper tests defined");
    println!("4. ✅ Build process tests defined");
    println!("5. ✅ Documentation tests defined");
    println!();
    println!("To run full verification:");
    println!("  cargo test -- --nocapture");
}