//! Test simple comptime capabilities

use zetac::frontend::parser::top_level::parse_zeta;
use zetac::middle::const_eval::{ConstEvaluator, ConstValue};
use zetac::AstNode;

fn main() {
    println!("=== Testing simple comptime capabilities ===\n");
    
    let code = r#"
comptime fn simple() -> i64 {
    42
}

comptime fn with_variables() -> i64 {
    let x = 10
    let y = 20
    x + y
}

comptime fn with_if() -> i64 {
    let condition = true
    if condition {
        100
    } else {
        200
    }
}

comptime fn with_loop() -> i64 {
    var sum = 0
    for i in 0..5 {
        sum = sum + i
    }
    sum
}

comptime fn with_array() -> i64 {
    let arr = [1, 2, 3, 4, 5]
    arr[2]
}

// Main function to test everything
comptime fn test_all() -> i64 {
    simple() + with_variables() + with_if() + with_loop() + with_array()
}
    "#;
    
    match parse_zeta(code) {
        Ok((remaining, ast)) => {
            if remaining.trim().is_empty() {
                println!("✅ Parses successfully");
                
                let mut evaluator = ConstEvaluator::new();
                
                // Test each function
                let test_cases = [
                    ("simple", 42),
                    ("with_variables", 30),
                    ("with_if", 100),
                    ("with_loop", 10), // 0+1+2+3+4 = 10
                    ("with_array", 3),
                    ("test_all", 42 + 30 + 100 + 10 + 3), // 185
                ];
                
                let mut passed = 0;
                let mut failed = 0;
                
                for (func_name, expected) in &test_cases {
                    println!("\nTesting {} (expected {}):", func_name, expected);
                    
                    if let Some(func) = ast.iter().find(|node| {
                        if let AstNode::FuncDef { name, .. } = node {
                            name == func_name
                        } else {
                            false
                        }
                    }) {
                        match evaluator.try_eval_const_call(func, &[]) {
                            Ok(Some(ConstValue::Int(value))) => {
                                if value == *expected {
                                    println!("✅ PASS: evaluates to {}", value);
                                    passed += 1;
                                } else {
                                    println!("❌ FAIL: expected {}, got {}", expected, value);
                                    failed += 1;
                                }
                            }
                            Ok(Some(other)) => {
                                println!("❌ FAIL: evaluates to non-integer: {:?}", other);
                                failed += 1;
                            }
                            Ok(None) => {
                                println!("❌ FAIL: does not evaluate (returns None)");
                                failed += 1;
                            }
                            Err(e) => {
                                println!("❌ FAIL: evaluation error: {}", e);
                                failed += 1;
                            }
                        }
                    } else {
                        println!("❌ FAIL: function {} not found", func_name);
                        failed += 1;
                    }
                }
                
                println!("\n=== Summary ===");
                println!("Passed: {}", passed);
                println!("Failed: {}", failed);
                println!("Total:  {}", passed + failed);
                
                if failed > 0 {
                    std::process::exit(1);
                }
            } else {
                println!("❌ Partial parse, remaining: {}", remaining);
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!("❌ Parse error: {:?}", e);
            std::process::exit(1);
        }
    }
}