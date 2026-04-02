//! Debug array assignment

use zetac::frontend::parser::top_level::parse_zeta;
use zetac::middle::const_eval::{ConstEvaluator, ConstValue};

fn main() {
    let code = r#"
comptime fn test() -> i64 {
    var arr = [0, 0, 0, 0, 0]
    arr[2] = 42
    arr[2]
}
    "#;

    println!("Parsing: {}", code);
    match parse_zeta(code) {
        Ok((remaining, ast)) => {
            if remaining.trim().is_empty() {
                println!("✅ Parses successfully");
                println!("AST: {:#?}", ast);
                
                let mut evaluator = ConstEvaluator::new();
                if let Some(func) = ast.first() {
                    match evaluator.try_eval_const_call(func, &[]) {
                        Ok(Some(ConstValue::Int(value))) => {
                            println!("✅ Evaluates to: {}", value);
                        }
                        Ok(Some(ConstValue::Array(arr))) => {
                            println!("✅ Evaluates to array: {:?}", arr);
                        }
                        Ok(Some(other)) => {
                            println!("❌ Evaluates to: {:?}", other);
                        }
                        Ok(None) => {
                            println!("❌ Does not evaluate (returns None)");
                        }
                        Err(e) => {
                            println!("❌ Evaluation error: {}", e);
                        }
                    }
                }
            } else {
                println!("❌ Partial parse");
            }
        }
        Err(e) => {
            println!("❌ Parse error: {:?}", e);
        }
    }
}