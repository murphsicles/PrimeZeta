use contamination_backup::zeta_github::src::frontend::parser::parser::parse_type;

fn main() {
    let test_cases = vec![
        "u64",
        "[u64]",
        "[u64; 5]",
        "[100]bool",
    ];
    
    for test in test_cases {
        println!("\nTesting parse_type('{}'):", test);
        match parse_type(test) {
            Ok((remaining, result)) => {
                println!("  Success! Remaining: '{}'", remaining);
                println!("  Result: '{}'", result);
            }
            Err(e) => {
                println!("  Error: {:?}", e);
            }
        }
    }
}