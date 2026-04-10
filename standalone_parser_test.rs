// Let me create a minimal test to understand the issue
// I'll copy the relevant parser code and test it

fn test_parse_logical_and() {
    // Simulate the parse_logical_and logic
    let input = "j < 5 && j >= 0";
    println!("Testing: {}", input);
    
    // The parser should:
    // 1. parse_comparison("j < 5") -> success, remaining: " && j >= 0"
    // 2. loop: find "&&" 
    // 3. parse_comparison("j >= 0") -> success, remaining: ""
    
    // But what if parse_comparison("j < 5") consumes "j < 5 && j >= 0"?
    // That would happen if parse_comparison doesn't stop at "j < 5"
    
    println!("The issue might be that parse_comparison continues parsing past the comparison.");
    println!("parse_comparison calls parse_additive, which calls parse_multiplicative, etc.");
    println!("These functions have loops that continue parsing operators.");
    println!("So parse_comparison(\"j < 5 && j >= 0\") might parse \"j < 5 && j >= 0\" as a single comparison!");
    println!("That would be wrong because \"&&\" is not a comparison operator.");
    println!();
    println!("Let me check the parse_comparison function...");
    println!("It has a loop that looks for comparison operators (==, !=, <, >, <=, >=).");
    println!("When it sees \"j < 5\", it should parse \"j < 5\" and stop.");
    println!("Then remaining_input should be \" && j >= 0\".");
    println!("The loop should break because \"&&\" is not a comparison operator.");
    println!("But what if the logic is wrong?");
}

fn main() {
    test_parse_logical_and();
}