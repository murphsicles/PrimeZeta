// Debug test for array type parsing
use zetac::middle::resolver::NewTypeCheck;
use zetac::middle::resolver::Resolver;

fn main() {
    let resolver = Resolver::new();
    
    // Test parsing different array type strings
    let test_cases = vec![
        "[u8]",
        "[u8; 0]",
        "[u8; 10]",
        "[bool]",
        "[bool; 5]",
    ];
    
    for test in test_cases {
        println!("Testing type string: '{}'", test);
        let ty = resolver.string_to_type(test);
        println!("  Result: {:?}", ty);
        println!("  To string: {}", resolver.type_to_string(&ty));
        println!();
    }
}