use zetac::frontend::parser::identity_type::parse_string_with_identity;

fn main() {
    let test_cases = vec![
        "string",
        "string[identity:read]",
        "string [identity:read]",  // with space
        "string[identity:read+write]",
        "string [identity:read+write]",  // with space
    ];
    
    for test in test_cases {
        println!("Testing: '{}'", test);
        match parse_string_with_identity(test) {
            Ok((remaining, result)) => {
                println!("  Success: '{}', Remaining: '{}'", result, remaining);
            }
            Err(e) => {
                println!("  Error: {:?}", e);
            }
        }
        println!();
    }
}