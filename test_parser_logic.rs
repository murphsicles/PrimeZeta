// Simple test to understand the parser logic
fn main() {
    println!("Testing parser logic for && operator");
    println!("=====================================");
    println!();
    
    // Test case 1: Simple && 
    println!("Test 1: a && b");
    println!("  parse_logical_and(\"a && b\")");
    println!("    parse_comparison(\"a && b\")");
    println!("      parse_additive(\"a && b\") -> parses 'a', leaves ' && b'");
    println!("      Loop: looks for comparison operators, doesn't find any");
    println!("      Returns: (\" && b\", 'a')");
    println!("    Loop: finds '&&'");
    println!("    parse_comparison(\"b\") -> parses 'b'");
    println!("    Returns: (\"\", 'a && b')");
    println!();
    
    // Test case 2: Comparison && comparison
    println!("Test 2: j < 5 && j >= 0");
    println!("  parse_logical_and(\"j < 5 && j >= 0\")");
    println!("    parse_comparison(\"j < 5 && j >= 0\")");
    println!("      parse_additive(\"j < 5 && j >= 0\") -> parses 'j', leaves ' < 5 && j >= 0'");
    println!("      Loop: finds '<' (with whitespace)");
    println!("      parse_additive(\"5 && j >= 0\") -> parses '5', leaves ' && j >= 0'");
    println!("      Returns: (\" && j >= 0\", 'j < 5')");
    println!("    Loop: finds '&&'");
    println!("    parse_comparison(\"j >= 0\")");
    println!("      parse_additive(\"j >= 0\") -> parses 'j', leaves ' >= 0'");
    println!("      Loop: finds '>='");
    println!("      parse_additive(\"0\") -> parses '0'");
    println!("      Returns: (\"\", 'j >= 0')");
    println!("    Returns: (\"\", '(j < 5) && (j >= 0)')");
    println!();
    
    println!("This should work! So why doesn't it?");
    println!();
    println!("Potential issues:");
    println!("1. parse_additive might consume '5 && j >= 0' as '5 && j'");
    println!("   But parse_additive only looks for + and - operators, not &&");
    println!("   So it should stop at '5'");
    println!();
    println!("2. parse_primary might fail to parse 'j'");
    println!("   But 'j' is not a keyword (even with our fixes)");
    println!();
    println!("3. The .unwrap_or() calls might be hiding errors");
    println!("   But skip_ws_and_comments0 should never fail");
    println!();
    println!("I think I need to actually run the parser to see what's happening.");
}