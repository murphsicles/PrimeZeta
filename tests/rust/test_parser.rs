use zetac::frontend::ast::AstNode;
use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    let code = r#"
const TEST_CONST: i64 = 100;

fn main() -> i64 {
    TEST_CONST
}
"#;

    match parse_zeta(code) {
        Ok((remaining, asts)) => {
            println!("Parsing successful!");
            println!("Remaining characters: {}", remaining.len());
            println!("Number of AST nodes: {}", asts.len());
            
            for (i, ast) in asts.iter().enumerate() {
                println!("AST {}: {:?}", i, ast);
                
                // Check if it's a ConstDef
                if let AstNode::ConstDef { name, ty, value } = ast {
                    println!("Found constant: {}: {} = {:?}", name, ty, value);
                }
                
                // Check if it's a FuncDef
                if let AstNode::FuncDef { name, .. } = ast {
                    println!("Found function: {}", name);
                }
            }
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
        }
    }
}