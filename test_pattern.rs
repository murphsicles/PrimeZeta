use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    let code = "fn with_pattern((x, y): (i64, i64)) -> i64 { x + y }";
    
    println!("Testing: {}", code);
    match parse_zeta(code) {
        Ok((remaining, ast)) => {
            println!("Success! Remaining: '{}'", remaining);
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}