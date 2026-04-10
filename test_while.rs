use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    let code = "while i < 10 { i = i + 1 }";
    println!("Testing while loop parsing: '{}'", code);
    
    match parse_zeta(code) {
        Ok((remaining, asts)) => {
            println!("Success! Parsed ASTs: {:?}", asts);
            if !remaining.is_empty() {
                println!("Remaining input: '{}'", remaining);
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}