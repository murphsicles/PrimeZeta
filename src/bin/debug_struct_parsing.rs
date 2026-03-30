// Debug script to test struct parsing
use nom::Parser;
use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    println!("=== DEBUGGING STRUCT PARSING ===");

    // Test 1: Simple struct
    let test1 = "struct Point { x: i64 }";
    println!("\nTest 1: {}", test1);
    match parse_zeta(test1) {
        Ok((remaining, _ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", _ast);
        }
        Err(e) => println!("  Error: {:?}", e),
    }

    // Test 2: Generic struct (what's failing)
    let test2 = "struct Vec<T> { data: T }";
    println!("\nTest 2: {}", test2);
    match parse_zeta(test2) {
        Ok((remaining, _ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", _ast);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
            // Try to see what part parses
            let (partial, _) = nom::bytes::complete::take_until::<&str, &str, ()>("{")
                .parse(test2)
                .unwrap();
            println!("  Parsed up to: '{}'", partial);
        }
    }

    // Test 3: Multiple fields
    let test3 = "struct Pair<A, B> { first: A, second: B }";
    println!("\nTest 3: {}", test3);
    match parse_zeta(test3) {
        Ok((remaining, _ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", _ast);
        }
        Err(e) => println!("  Error: {:?}", e),
    }

    // Test 4: What the compiler is seeing
    let test4 = r#"struct Vec<T> {
    data: T,
}"#;
    println!("\nTest 4 (with newline):");
    println!("Input: {:?}", test4);
    match parse_zeta(test4) {
        Ok((remaining, _ast)) => {
            println!("  Success! Remaining length: {}", remaining.len());
            if !remaining.is_empty() {
                println!("  Remaining: {:?}", remaining);
            }
        }
        Err(e) => println!("  Error: {:?}", e),
    }
}
