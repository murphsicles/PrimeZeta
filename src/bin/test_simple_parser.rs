use zetac::frontend::parser::expr::parse_expr;
use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    // First test expression parsing
    println!("Testing expression parsing...");
    let expr = "Vec::<i32>::new()";
    match parse_expr(expr) {
        Ok((remaining, ast)) => {
            println!("Expression parsed successfully!");
            println!("Remaining: '{}'", remaining);
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("Error parsing expression: {:?}", e);
        }
    }

    println!("\nTesting top-level parsing with just a function...");
    let simple_code = r#"fn main() {
    let v = Vec::<i32>::new();
}"#;

    match parse_zeta(simple_code) {
        Ok((remaining, ast)) => {
            println!("Success!");
            println!("Remaining: '{}'", remaining);
            println!("AST length: {}", ast.len());
            if !ast.is_empty() {
                println!("First AST node: {:?}", ast[0]);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    println!("\nTesting top-level parsing with struct...");
    let struct_code = r#"struct Vec<T> {
    data: [T],
}"#;

    match parse_zeta(struct_code) {
        Ok((remaining, ast)) => {
            println!("Success!");
            println!("Remaining: '{}'", remaining);
            println!("AST length: {}", ast.len());
            if !ast.is_empty() {
                println!("First AST node: {:?}", ast[0]);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
