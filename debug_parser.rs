use contamination_backup::zeta_github::src::frontend::parser::expr::parse_expr;

fn main() {
    println!("Testing && operator parsing");
    
    let test_cases = vec![
        "a && b",
        "j < 5 && j >= 0",
        "x > 0 && x < 10",
        "true && false",
        "a && b && c",
    ];
    
    for test in test_cases {
        println!("\nTesting: '{}'", test);
        match parse_expr(test) {
            Ok((remaining, ast)) => {
                println!("  Success! Remaining: '{}'", remaining);
                println!("  AST: {:?}", ast);
            }
            Err(e) => {
                println!("  Error: {:?}", e);
            }
        }
    }
}