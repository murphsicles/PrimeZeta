// Debug script to test generic function call parsing
use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    println!("=== DEBUGGING GENERIC FUNCTION CALL PARSING ===");
    
    // Test 1: Simple function call
    let test1 = "fn main() { let x = foo(); }";
    println!("\nTest 1 (simple call): {}", test1);
    match parse_zeta(test1) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    
    // Test 2: Static method call
    let test2 = "fn main() { let x = Point::new(); }";
    println!("\nTest 2 (static method): {}", test2);
    match parse_zeta(test2) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    
    // Test 3: Generic static method call (what's failing)
    let test3 = "fn main() { let x = vec_new::<i32>(); }";
    println!("\nTest 3 (generic static method): {}", test3);
    match parse_zeta(test3) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => println!("  Error: {:?}", e),
    }
    
    // Test 4: With struct definition first
    let test4 = r#"
struct Vec<T> {
    data: [T],
}

fn vec_new<T>() -> Vec<T> {
    Vec { data: [] }
}

fn main() {
    let v = vec_new::<i32>();
}
"#;
    println!("\nTest 4 (full example):");
    match parse_zeta(test4) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining length: {}", remaining.len());
            if !remaining.is_empty() {
                println!("  Remaining (first 100 chars): '{}'", &remaining[..remaining.len().min(100)]);
            }
        }
        Err(e) => println!("  Error: {:?}", e),
    }
}