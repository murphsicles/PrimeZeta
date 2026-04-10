// Minimal test to understand the parser issue
// Let me simulate what happens

fn main() {
    println!("Testing parser logic for: if j < 5 && j >= 0 { return 1; }");
    println!();
    println!("Step 1: parse_if is called");
    println!("  - Consumes 'if'");
    println!("  - Calls ws(parse_full_expr) for condition");
    println!();
    println!("Step 2: parse_full_expr calls parse_expr");
    println!("  - parse_expr tries parse_if (fails)");
    println!("  - Calls parse_expr_no_if");
    println!();
    println!("Step 3: parse_expr_no_if calls parse_logical_or");
    println!("  - parse_logical_or calls parse_logical_and");
    println!();
    println!("Step 4: parse_logical_and calls parse_comparison");
    println!("  - parse_comparison should parse 'j < 5'");
    println!("  - Should leave ' && j >= 0'");
    println!();
    println!("Step 5: parse_logical_and loop finds '&&'");
    println!("  - Calls parse_comparison('j >= 0')");
    println!("  - Should succeed");
    println!();
    println!("Potential issues:");
    println!("1. parse_comparison might not handle whitespace correctly");
    println!("2. parse_primary might fail to parse 'j'");
    println!("3. The .unwrap_or() calls might hide errors");
    println!("4. 'while' not in keyword list (but that's for parse_ident)");
    println!();
    println!("Actually, I just realized: 'j' should parse as an identifier.");
    println!("But what if parse_ident fails for some other reason?");
    println!("Let me check if there are any other keywords that might interfere...");
}