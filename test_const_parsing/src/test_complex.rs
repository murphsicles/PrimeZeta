use zetac::frontend::ast::AstNode;
use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    let code = r#"
const MAX_SIZE: i64 = 1024;
const DEFAULT_NAME: &str = "Zeta";
const IS_ENABLED: bool = true;
const PI: f64 = 3.14159;

fn main() -> i64 {
    MAX_SIZE
}
"#;

    match parse_zeta(code) {
        Ok((remaining, asts)) => {
            println!("Parsing successful!");
            println!("Remaining characters: {}", remaining.len());
            println!("Number of AST nodes: {}", asts.len());
            
            let mut const_count = 0;
            let mut func_count = 0;
            
            for (i, ast) in asts.iter().enumerate() {
                println!("AST {}: {:?}", i, ast);
                
                match ast {
                    AstNode::ConstDef { name, ty, value } => {
                        println!("  Constant: {}: {} = {:?}", name, ty, value);
                        const_count += 1;
                    }
                    AstNode::FuncDef { name, .. } => {
                        println!("  Function: {}", name);
                        func_count += 1;
                    }
                    _ => {
                        println!("  Other AST node");
                    }
                }
            }
            
            println!("\nSummary:");
            println!("  Constants: {}", const_count);
            println!("  Functions: {}", func_count);
            
            if remaining.is_empty() {
                println!("  All input parsed successfully!");
            } else {
                println!("  Warning: {} characters not parsed", remaining.len());
                if remaining.len() < 100 {
                    println!("  Unparsed: {:?}", remaining);
                }
            }
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
        }
    }
}