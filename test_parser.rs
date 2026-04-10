use src::frontend::parser::top_level::parse_func;

fn main() {
    let code = "fn test(a: i64) -> i64 { a }";
    match parse_func(code) {
        Ok((remaining, ast)) => {
            println!("Success! Remaining: '{}'", remaining);
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}