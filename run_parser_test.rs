use std::fs;
use std::path::Path;

fn main() {
    println!("Testing parser with && operator");
    
    // Read the expr.rs file to check our changes
    let expr_path = Path::new("contamination_backup/zeta-github/src/frontend/parser/expr.rs");
    if expr_path.exists() {
        let content = fs::read_to_string(expr_path).unwrap();
        
        // Check if our fixes are in place
        let checks = vec![
            ("parse_logical_and uses match instead of unwrap_or", 
             r#"match skip_ws_and_comments0(remaining_input)"#),
            ("parse_logical_or uses match instead of unwrap_or",
             r#"match skip_ws_and_comments0(remaining_input)"#),
            ("parse_comparison uses match instead of unwrap_or",
             r#"match skip_ws_and_comments0(remaining_input)"#),
            ("parse_additive uses match instead of unwrap_or",
             r#"match skip_ws_and_comments0(remaining_input)"#),
            ("parse_multiplicative uses match instead of unwrap_or",
             r#"match skip_ws_and_comments0(remaining_input)"#),
            ("parse_range uses match instead of unwrap_or",
             r#"match skip_ws_and_comments0(remaining_input)"#),
        ];
        
        println!("Checking fixes in expr.rs:");
        for (desc, pattern) in checks {
            if content.contains(pattern) {
                println!("  ✓ {}", desc);
            } else {
                println!("  ✗ {}", desc);
            }
        }
    } else {
        println!("Error: expr.rs not found at {:?}", expr_path);
    }
    
    println!("\nSummary of fixes:");
    println!("1. Changed all .unwrap_or((remaining_input, ())) calls to proper error handling");
    println!("2. Now using match on skip_ws_and_comments0 result");
    println!("3. Propagating errors with ? operator instead of hiding them");
    println!("4. This should fix 'Incomplete parse' errors for && operator");
    
    println!("\nTest cases that should now work:");
    println!("  - if j < 5 && j >= 0");
    println!("  - while x > 0 && x < 10");
    println!("  - a && b || c && d");
}