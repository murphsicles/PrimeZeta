use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    let bad_code = r#"
    struct Bad< { // Missing closing >
    "#;

    println!("Testing parser with malformed generic: {}", bad_code);
    let result = parse_zeta(bad_code);
    println!("Result: {:?}", result);
    
    if let Ok((remaining, _)) = result {
        println!("Parser succeeded! Remaining: '{}'", remaining);
    } else {
        println!("Parser failed as expected");
    }
}