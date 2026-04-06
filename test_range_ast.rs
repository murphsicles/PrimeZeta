use zetac::frontend::parser::expr::parse_expr;

fn main() {
    let code = "0..5";
    match parse_expr(code) {
        Ok((remaining, ast)) => {
            println!("Parsed successfully!");
            println!("Remaining: '{}'", remaining);
            println!("AST: {:?}", ast);
            
            // Check what type of node it is
            match ast {
                zetac::frontend::ast::AstNode::Range { start, end, inclusive } => {
                    println!("It's an AstNode::Range!");
                    println!("Start: {:?}", start);
                    println!("End: {:?}", end);
                    println!("Inclusive: {}", inclusive);
                }
                zetac::frontend::ast::AstNode::BinaryOp { op, left, right, .. } => {
                    println!("It's an AstNode::BinaryOp!");
                    println!("Op: {}", op);
                    println!("Left: {:?}", left);
                    println!("Right: {:?}", right);
                }
                _ => {
                    println!("It's something else: {:?}", ast);
                }
            }
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}