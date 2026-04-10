// Let me trace through what happens when we parse: if j < 5 && j >= 0 { return 1; }

// 1. parse_if is called (from parse_expr when parsing the whole statement)
// 2. parse_if consumes "if" 
// 3. parse_if calls ws(parse_full_expr) for the condition
// 4. ws calls skip_ws_and_comments0, then parse_full_expr, then skip_ws_and_comments0
// 5. parse_full_expr calls parse_expr
// 6. parse_expr tries parse_if first (but input is "j < 5 && j >= 0", not "if ...")
// 7. parse_if fails, so parse_expr calls parse_expr_no_if
// 8. parse_expr_no_if calls parse_logical_or
// 9. parse_logical_or calls parse_logical_and
// 10. parse_logical_and calls parse_comparison
// 11. parse_comparison should parse "j < 5", then see "&&", loop back...

// The issue might be in the loop logic in parse_logical_and.
// Let me examine it more carefully.

fn main() {
    println!("Debugging && operator issue");
    println!("============================");
    println!();
    println!("The parse_logical_and function has this logic:");
    println!("1. Parse comparison (j < 5)");
    println!("2. Loop: try to find '&&' operator");
    println!("3. If found, parse another comparison (j >= 0)");
    println!("4. Create BinaryOp node");
    println!();
    println!("Potential issues:");
    println!("1. Whitespace handling around '&&'");
    println!("2. The loop might not handle the case correctly");
    println!("3. parse_comparison might not consume everything");
    println!();
    println!("Let me check the actual parse_logical_and code...");
}