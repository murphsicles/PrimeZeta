//! Identity type parsing
//! 
//! This module provides parsing for identity type annotations,
//! such as `string[identity:read]` or `string[identity:read+write]`.

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::{opt, map};
use nom::multi::separated_list0;
use nom::sequence::delimited;

use crate::frontend::parser::parser::ws;

/// Parse an identity capability (read, write, execute, owned)
pub fn parse_capability(input: &str) -> IResult<&str, String> {
    let capabilities = alt((
        tag("read").map(|_| "read".to_string()),
        tag("write").map(|_| "write".to_string()),
        tag("execute").map(|_| "execute".to_string()),
        tag("owned").map(|_| "owned".to_string()),
    ));
    
    ws(capabilities).parse(input)
}

/// Parse a list of capabilities separated by '+'
pub fn parse_capability_list(input: &str) -> IResult<&str, Vec<String>> {
    separated_list0(ws(tag("+")), parse_capability).parse(input)
}

/// Parse identity type annotation
/// Syntax: `[identity:capability1+capability2+...]`
pub fn parse_identity_annotation(input: &str) -> IResult<&str, Vec<String>> {
    delimited(
        ws(tag("[identity:")),
        parse_capability_list,
        ws(tag("]")),
    ).parse(input)
}

/// Parse string type with identity annotation
/// Special case for `string[identity:read]` syntax
pub fn parse_string_with_identity(input: &str) -> IResult<&str, String> {
    let (input, type_name) = ws(alpha1).parse(input)?;
    
    if type_name != "string" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    
    let (input, identity_opt) = opt(parse_identity_annotation).parse(input)?;
    
    let result = if let Some(capabilities) = identity_opt {
        let caps_str = capabilities.join("+");
        format!("string[identity:{}]", caps_str)
    } else {
        "string".to_string()
    };
    
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_capability() {
        assert_eq!(parse_capability("read"), Ok(("", "read".to_string())));
        assert_eq!(parse_capability("write"), Ok(("", "write".to_string())));
        assert_eq!(parse_capability("execute"), Ok(("", "execute".to_string())));
        assert_eq!(parse_capability("owned"), Ok(("", "owned".to_string())));
    }

    #[test]
    fn test_parse_capability_list() {
        assert_eq!(
            parse_capability_list("read"),
            Ok(("", vec!["read".to_string()]))
        );
        assert_eq!(
            parse_capability_list("read+write"),
            Ok(("", vec!["read".to_string(), "write".to_string()]))
        );
        assert_eq!(
            parse_capability_list("read+write+execute"),
            Ok(("", vec!["read".to_string(), "write".to_string(), "execute".to_string()]))
        );
    }

    #[test]
    fn test_parse_identity_annotation() {
        assert_eq!(
            parse_identity_annotation("[identity:read]"),
            Ok(("", vec!["read".to_string()]))
        );
        assert_eq!(
            parse_identity_annotation("[identity:read+write]"),
            Ok(("", vec!["read".to_string(), "write".to_string()]))
        );
        assert_eq!(
            parse_identity_annotation("[identity:read+write+execute]"),
            Ok(("", vec!["read".to_string(), "write".to_string(), "execute".to_string()]))
        );
    }

    #[test]
    fn test_parse_string_with_identity() {
        assert_eq!(
            parse_string_with_identity("string"),
            Ok(("", "string".to_string()))
        );
        assert_eq!(
            parse_string_with_identity("string[identity:read]"),
            Ok(("", "string[identity:read]".to_string()))
        );
        assert_eq!(
            parse_string_with_identity("string[identity:read+write]"),
            Ok(("", "string[identity:read+write]".to_string()))
        );
    }
}