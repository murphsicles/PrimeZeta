use crate::frontend::ast::AstNode;

/// Stub parse_zeta function for v0.5.0 compilation
pub fn parse_zeta(_input: &str) -> Result<(&str, Vec<AstNode>), &'static str> {
    // Return empty result for now
    Ok(("", vec![]))
}