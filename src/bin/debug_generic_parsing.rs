// Debug script to test generic struct parsing
use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    println!("=== DEBUGGING GENERIC STRUCT PARSING ===");

    // Test 1: Simple non-generic struct
    let test1 = "struct Point { x: i64, y: i64 }";
    println!("\nTest 1 (non-generic): {}", test1);
    match parse_zeta(test1) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => println!("  Error: {:?}", e),
    }

    // Test 2: Generic struct
    let test2 = "struct Point<T> { x: T, y: T }";
    println!("\nTest 2 (generic): {}", test2);
    match parse_zeta(test2) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => println!("  Error: {:?}", e),
    }

    // Test 3: Generic struct with multiple params
    let test3 = "struct Pair<A, B> { first: A, second: B }";
    println!("\nTest 3 (multiple generics): {}", test3);
    match parse_zeta(test3) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => println!("  Error: {:?}", e),
    }

    // Test 4: Generic struct with bounds
    let test4 = "struct Container<T: Display> { value: T }";
    println!("\nTest 4 (with bounds): {}", test4);
    match parse_zeta(test4) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => println!("  Error: {:?}", e),
    }
}
