use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    // Test 1: Basic module
    let code1 = r#"
        mod math {
            pub fn add(a: i64, b: i64) -> i64 {
                a + b
            }
        }
        
        fn main() -> i64 {
            42
        }
    "#;

    println!("Test 1: Basic module parsing");
    match parse_zeta(code1) {
        Ok((remaining, ast)) => {
            println!("Success! Remaining: '{}'", remaining);
            println!("Number of AST nodes: {}", ast.len());
            // Just print first node to see if it's a ModDef
            if let Some(node) = ast.first() {
                println!("First node: {:?}", node);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // Test 2: Simple function (should still work)
    let code2 = r#"
        fn add(a: i64, b: i64) -> i64 {
            a + b
        }
        
        fn main() -> i64 {
            add(10, 20)
        }
    "#;

    println!("\nTest 2: Simple function (regression test)");
    match parse_zeta(code2) {
        Ok((remaining, ast)) => {
            println!("Success! Remaining: '{}'", remaining);
            println!("Number of AST nodes: {}", ast.len());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
