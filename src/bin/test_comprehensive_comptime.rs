//! Test comprehensive comptime capabilities

use zetac::frontend::parser::top_level::parse_zeta;
use zetac::middle::const_eval::{ConstEvaluator, ConstValue};
use zetac::AstNode;
use std::fs;

fn test_file(filename: &str) -> Result<(), String> {
    println!("\n=== Testing {} ===", filename);
    
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
    
    match parse_zeta(&content) {
        Ok((remaining, ast)) => {
            if remaining.trim().is_empty() {
                println!("✅ Parses successfully");
                
                // Look for main function
                let mut evaluator = ConstEvaluator::new();
                for node in &ast {
                    if let AstNode::FuncDef { name, .. } = node {
                        if name == "main" {
                            match evaluator.try_eval_const_call(node, &[]) {
                                Ok(Some(ConstValue::Int(value))) => {
                                    println!("✅ Main evaluates to: {}", value);
                                    return Ok(());
                                }
                                Ok(Some(other)) => {
                                    return Err(format!("Main evaluates to non-integer: {:?}", other));
                                }
                                Ok(None) => {
                                    return Err("Main does not evaluate (returns None)".to_string());
                                }
                                Err(e) => {
                                    return Err(format!("Evaluation error: {}", e));
                                }
                            }
                        }
                    }
                }
                
                Err("No main function found".to_string())
            } else {
                Err(format!("Partial parse, remaining: {}", remaining))
            }
        }
        Err(e) => {
            Err(format!("Parse error: {:?}", e))
        }
    }
}

fn main() {
    println!("=== Comprehensive Comptime Tests ===\n");
    
    let test_files = [
        "tests/comptime-completion/test_variable_assignment.z",
        "tests/comptime-completion/test_control_flow.z",
        "tests/comptime-completion/test_array_assignment.z",
        "tests/comptime-completion/test_primezeta_compatibility.z",
        "tests/comptime-completion/test_comprehensive.z",
    ];
    
    let mut passed = 0;
    let mut failed = 0;
    
    for file in &test_files {
        match test_file(file) {
            Ok(_) => {
                println!("✅ PASS: {}", file);
                passed += 1;
            }
            Err(e) => {
                println!("❌ FAIL: {} - {}", file, e);
                failed += 1;
            }
        }
    }
    
    println!("\n=== Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);
    println!("Total:  {}", passed + failed);
    
    if failed > 0 {
        std::process::exit(1);
    }
}