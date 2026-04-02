//! Debug var parsing

use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    let code = r#"
comptime fn test() -> i64 {
    var x = 5
    x
}
    "#;

    println!("Parsing: {}", code);
    match parse_zeta(code) {
        Ok((remaining, ast)) => {
            if remaining.trim().is_empty() {
                println!("✅ Parses successfully");
                println!("AST: {:#?}", ast);
            } else {
                println!("❌ Partial parse, remaining: {}", remaining);
            }
        }
        Err(e) => {
            println!("❌ Parse error: {:?}", e);
        }
    }
}