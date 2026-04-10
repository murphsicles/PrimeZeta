use zetac::frontend::parser::expr::parse_full_expr;

fn main() {
    // Test 1: Simple && in expression
    let test1 = "j < 5 && j >= 0";
    println!("Test 1: {}", test1);
    match parse_full_expr(test1) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }
    
    println!();
    
    // Test 2: Just the comparison part
    let test2 = "j < 5";
    println!("Test 2: {}", test2);
    match parse_full_expr(test2) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }
    
    println!();
    
    // Test 3: Full if statement
    let test3 = "if j < 5 && j >= 0 { return 1; }";
    println!("Test 3: {}", test3);
    match parse_full_expr(test3) {
        Ok((remaining, ast)) => {
            println!("  Success! Remaining: '{}'", remaining);
            println!("  AST: {:?}", ast);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
        }
    }
}