// src/frontend/parser/pattern.rs
//! Module for parsing patterns in the Zeta language.

use super::parser::{parse_ident, parse_path, skip_ws_and_comments, ws};
use super::expr::parse_lit;
use crate::frontend::ast::AstNode;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, terminated};

/// Parse a pattern: `_`, identifier, literal, tuple pattern, or struct pattern.
pub fn parse_pattern(input: &str) -> IResult<&str, AstNode> {
    alt((
        // Wildcard pattern
        tag("_").map(|_| AstNode::Ignore),
        // Tuple pattern: `(pattern, pattern, ...)`
        parse_tuple_pattern,
        // Struct pattern: `Path { field: pattern, ... }` or `Path(pattern, ...)`
        parse_struct_pattern,
        // Literal pattern
        parse_lit,
        // Variable pattern
        parse_ident.map(AstNode::Var),
    ))
    .parse(input)
}

/// Parse a tuple pattern: `(pattern, pattern, ...)`
fn parse_tuple_pattern(input: &str) -> IResult<&str, AstNode> {
    delimited(
        ws(tag("(")),
        terminated(
            separated_list0(ws(tag(",")), ws(parse_pattern)),
            opt(ws(tag(","))),
        ),
        ws(tag(")")),
    )
    .map(AstNode::Tuple)
    .parse(input)
}

/// Parse a struct pattern: either tuple struct `Path(pattern, ...)` or named struct `Path { field: pattern, ... }`
fn parse_struct_pattern(input: &str) -> IResult<&str, AstNode> {
    let (input, path) = parse_path(input)?;
    let variant = path.join("::");

    let (input, _) = skip_ws_and_comments(input)?;

    // Try tuple struct pattern first: `Path(pattern, ...)`
    if let Ok((_i, _)) = ws(tag("(")).parse(input) {
        return parse_tuple_struct_pattern(input, variant);
    }

    // Try named struct pattern: `Path { field: pattern, ... }`
    if let Ok((_i, _)) = ws(tag("{")).parse(input) {
        return parse_named_struct_pattern(input, variant);
    }

    // If neither, it's just a variable pattern with a path
    Ok((input, AstNode::Var(variant)))
}

/// Parse a tuple struct pattern: `Variant(pattern, pattern, ...)`
fn parse_tuple_struct_pattern(input: &str, variant: String) -> IResult<&str, AstNode> {
    let (input, pats) = delimited(
        ws(tag("(")),
        terminated(
            separated_list0(ws(tag(",")), ws(parse_pattern)),
            opt(ws(tag(","))),
        ),
        ws(tag(")")),
    )
    .parse(input)?;
    
    Ok((
        input,
        AstNode::StructPattern {
            variant,
            fields: pats
                .into_iter()
                .enumerate()
                .map(|(i, p)| (i.to_string(), p))
                .collect(),
            rest: false,
        },
    ))
}

/// Parse a named struct pattern: `Struct { field: pattern, ... }`
fn parse_named_struct_pattern(input: &str, variant: String) -> IResult<&str, AstNode> {
    let (input, fields) = delimited(
        ws(tag("{")),
        terminated(
            separated_list0(ws(tag(",")), ws(parse_field_pattern)),
            opt(ws(tag(","))),
        ),
        ws(tag("}")),
    )
    .parse(input)?;
    
    let (input, has_rest) = opt(preceded(ws(tag(",")), ws(tag("..")))).parse(input)?;
    
    Ok((
        input,
        AstNode::StructPattern {
            variant,
            fields,
            rest: has_rest.is_some(),
        },
    ))
}

/// Parse a field in a struct pattern: `field: pattern` or `field` (shorthand)
fn parse_field_pattern(input: &str) -> IResult<&str, (String, AstNode)> {
    let (input, name) = ws(parse_ident).parse(input)?;
    let (input, colon) = opt(ws(tag(":"))).parse(input)?;
    
    let (input, pat) = if colon.is_some() {
        // Field with explicit pattern: `field: pattern`
        ws(parse_pattern).parse(input)?
    } else {
        // Field shorthand: `field` is equivalent to `field: field`
        (input, AstNode::Var(name.clone()))
    };
    
    Ok((input, (name, pat)))
}