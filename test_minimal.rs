fn main() {
    // Let's manually trace through the parse_while function logic
    let code = "while i < 10 { i = i + 1 }";
    
    // Step 1: Check if it starts with "while"
    if !code.starts_with("while") {
        println!("ERROR: Doesn't start with 'while'");
        return;
    }
    
    // Step 2: Skip "while" and whitespace
    let after_while = &code[5..]; // "while" is 5 chars
    println!("After 'while': '{}'", after_while);
    
    // The parse_while function should:
    // 1. Parse "while" keyword
    // 2. Parse condition expression (i < 10)
    // 3. Parse block body { i = i + 1 }
    
    // Let's check if the condition parsing works
    // The condition should be "i < 10"
    // Looking at parse_comparison in expr.rs, it should handle "i < 10"
    
    // The issue might be in how parse_while is structured
    // Let me check the actual parse_while function
    println!("\nChecking parse_while logic...");
    
    // From reading stmt.rs earlier, parse_while should:
    // 1. Parse "while" keyword
    // 2. Parse expression (condition)
    // 3. Parse block
    
    // The problem might be that after parsing "while", it needs to skip whitespace
    // before parsing the condition
    
    println!("\nTest case analysis:");
    println!("1. 'while' keyword: ✓");
    println!("2. Condition 'i < 10': Should parse with parse_comparison");
    println!("3. Block '{ i = i + 1 }': Should parse with parse_block_body");
    
    // Let me check if there's an issue with the comparison operator parsing
    // The comparison operator "<" should be recognized
    println!("\nChecking comparison operator '<':");
    let test_comparison = "i < 10";
    println!("Test: {}", test_comparison);
    
    // The parse_comparison function should handle this
    // It calls parse_additive for left and right sides
    
    println!("\nPotential issues:");
    println!("1. Whitespace handling after 'while' keyword");
    println!("2. Comparison operator parsing (fixed unwrap_or issue)");
    println!("3. Block parsing");
    
    println!("\nLet me check the actual parse_while function implementation...");
}