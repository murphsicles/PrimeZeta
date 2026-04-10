use zetac::frontend::parser::expr::parse_if;

fn main() {
    let code = "if a < 5 && b > 0 { return 42 }";
    match parse_if(code) {
        Ok((remaining, ast)) => {
            println!("Success!");
            println!("Remaining: {:?}", remaining);
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}