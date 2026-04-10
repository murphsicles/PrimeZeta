use std::env;

fn main() {
    // Let's manually trace what should happen
    let code = "while i < 10 { i = i + 1 }";
    println!("Debugging parse_while for: '{}'", code);
    println!("Length: {}", code.len());
    
    // Step by step:
    // 1. ws(tag("while")) should parse "while" with optional whitespace before
    //    Since we're at start, no whitespace needed
    //    Should consume "while" (5 chars)
    
    // 2. After "while", we have " i < 10 { i = i + 1 }"
    //    ws(parse_full_expr) should parse the condition
    //    parse_full_expr -> parse_expr -> parse_expr_no_if -> parse_logical_or -> parse_logical_and -> ...
    //    Eventually parse_comparison should parse "i < 10"
    
    // 3. After condition, we have " { i = i + 1 }"
    //    delimited(ws(tag("{")), parse_block_body, ws(tag("}"))) should parse the block
    
    println!("\nPotential issues:");
    println!("1. parse_full_expr might fail on 'i < 10'");
    println!("2. Block parsing might fail");
    println!("3. Whitespace handling issues");
    
    // Let me check if the issue is with parse_comparison
    // I fixed the unwrap_or issue, but there might be other problems
    
    println!("\nChecking the comparison parsing fix...");
    println!("I fixed unwrap_or((remaining_input, ())) pattern in parse_comparison");
    println!("This was hiding errors when skip_ws_and_comments0 failed");
    
    // Actually, wait. Let me check if parse_while is even being called
    // The error said "Incomplete parse" - this suggests the parser consumed
    // some input but not all
    
    println!("\n'Incomplete parse' error suggests:");
    println!("1. Parser consumed part of input (e.g., 'while')");
    println!("2. But failed to parse the rest");
    println!("3. So it returns what it consumed + error");
    
    println!("\nLet me check if the issue is in parse_block_body...");
}