use zetac::frontend::ast::AstNode;
use zetac::frontend::parser::top_level::parse_zeta;

fn test_simple_const() {
    println!("=== Test 1: Simple constant ===");
    let code = r#"
const TEST_CONST: i64 = 100;

fn main() -> i64 {
    TEST_CONST
}
"#;

    match parse_zeta(code) {
        Ok((remaining, asts)) => {
            println!("✓ Parsing successful!");
            assert_eq!(remaining.len(), 0, "Should parse all input");
            assert_eq!(asts.len(), 2, "Should have 2 AST nodes (const + func)");
            
            let const_ast = &asts[0];
            if let AstNode::ConstDef { name, ty, value } = const_ast {
                assert_eq!(name, "TEST_CONST");
                assert_eq!(ty, "i64");
                if let AstNode::Lit(val) = **value {
                    assert_eq!(val, 100);
                } else {
                    panic!("Expected Lit(100), got {:?}", value);
                }
                println!("  ✓ Constant: {}: {} = {:?}", name, ty, value);
            } else {
                panic!("Expected ConstDef, got {:?}", const_ast);
            }
            
            println!("✓ All assertions passed!");
        }
        Err(e) => {
            panic!("Parsing failed: {:?}", e);
        }
    }
}

fn test_multiple_consts() {
    println!("\n=== Test 2: Multiple constants ===");
    let code = r#"
const MAX_SIZE: i64 = 1024;
const DEFAULT_NAME: &str = "Zeta";
const IS_ENABLED: bool = true;

fn main() -> i64 {
    MAX_SIZE
}
"#;

    match parse_zeta(code) {
        Ok((remaining, asts)) => {
            println!("✓ Parsing successful!");
            assert_eq!(remaining.len(), 0, "Should parse all input");
            assert_eq!(asts.len(), 4, "Should have 4 AST nodes (3 consts + func)");
            
            // Check constants
            for (i, ast) in asts.iter().take(3).enumerate() {
                if let AstNode::ConstDef { name, ty, .. } = ast {
                    println!("  ✓ Constant {}: {}: {}", i + 1, name, ty);
                } else {
                    panic!("Expected ConstDef at position {}, got {:?}", i, ast);
                }
            }
            
            println!("✓ All assertions passed!");
        }
        Err(e) => {
            panic!("Parsing failed: {:?}", e);
        }
    }
}

fn test_const_with_expression() {
    println!("\n=== Test 3: Constant with expression ===");
    let code = r#"
const COMPUTED: i64 = 10 + 20 * 3;

fn main() -> i64 {
    COMPUTED
}
"#;

    match parse_zeta(code) {
        Ok((remaining, asts)) => {
            println!("✓ Parsing successful!");
            assert_eq!(remaining.len(), 0, "Should parse all input");
            assert_eq!(asts.len(), 2, "Should have 2 AST nodes");
            
            let const_ast = &asts[0];
            if let AstNode::ConstDef { name, ty, value } = const_ast {
                assert_eq!(name, "COMPUTED");
                assert_eq!(ty, "i64");
                println!("  ✓ Constant: {}: {} = expression", name, ty);
                println!("    Expression: {:?}", value);
            } else {
                panic!("Expected ConstDef, got {:?}", const_ast);
            }
            
            println!("✓ All assertions passed!");
        }
        Err(e) => {
            panic!("Parsing failed: {:?}", e);
        }
    }
}

fn main() {
    println!("Testing Zeta constant parser implementation");
    println!("===========================================\n");
    
    test_simple_const();
    test_multiple_consts();
    test_const_with_expression();
    
    println!("\n===========================================");
    println!("All tests passed! Constant parsing is working.");
}