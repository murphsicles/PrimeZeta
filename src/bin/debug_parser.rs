use zetac::frontend::parser::expr::parse_expr;

fn main() {
    let input = "match x { 1 => 2, 3 => 4 }";
    match parse_expr(input) {
        Ok((remaining, ast)) => {
            println!("Success!");
            println!("Remaining: '{}'", remaining);
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}