use zetac::frontend::parser::parser::skip_ws_and_comments0;

fn main() {
    let input = "{";
    match skip_ws_and_comments0(input) {
        Ok((remaining, _)) => println!("OK: remaining = '{}'", remaining),
        Err(e) => println!("Err: {:?}", e),
    }
}