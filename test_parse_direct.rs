use zetac::frontend::parser::stmt::parse_stmt;
use zetac::frontend::parser::expr::parse_full_expr;
use zetac::frontend::parser::top_level::parse_zeta;

fn main() {
    let code = "while i < 10 { i = i + 1 }";
    println!("Testing parse_stmt on: '{}'", code);
    
    match parse_stmt(code) {
        Ok((remaining, stmt)) => {
            println!("✓ parse_stmt succeeded!");
            println!("  Remaining: '{}'", remaining);
            println!("  Statement: {:?}", stmt);
        }
        Err(e) => {
            println!("✗ parse_stmt failed: {:?}", e);
        }
    }
    
    println!("\n---\n");
    
    println!("Testing parse_full_expr on: '{}'", code);
    match parse_full_expr(code) {
        Ok((remaining, expr)) => {
            println!("✓ parse_full_expr succeeded!");
            println!("  Remaining: '{}'", remaining);
            println!("  Expression: {:?}", expr);
        }
        Err(e) => {
            println!("✗ parse_full_expr failed: {:?}", e);
        }
    }
    
    println!("\n---\n");
    
    println!("Testing parse_zeta on: '{}'", code);
    match parse_zeta(code) {
        Ok((remaining, asts)) => {
            println!("✓ parse_zeta succeeded!");
            println!("  Remaining: '{}'", remaining);
            println!("  ASTs: {:?}", asts);
        }
        Err(e) => {
            println!("✗ parse_zeta failed: {:?}", e);
        }
    }
}