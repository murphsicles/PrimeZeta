//! Identity type parsing
//! 
//! This module provides parsing for identity type annotations,
//! such as `string[identity:read]` or `string[identity:read+write]`,
//! including parametric identity types like `Identity<T: Read>`. 

use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, multispace0, digit1};
use nom::combinator::{opt, map, recognize};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded, tuple};

use crate::frontend::parser::parser::ws;

/// Parse an identity capability (read, write, execute, owned)
pub fn parse_capability(input: &str) -> IResult<&str, String> {
    let capabilities = alt((
        tag("read").map(|_| "read".to_string()),
        tag("write").map(|_| "write".to_string()),
        tag("execute").map(|_| "execute".to_string()),
        tag("owned").map(|_| "owned".to_string()),
        tag("immutable").map(|_| "immutable".to_string()),
    ));
    
    ws(capabilities).parse(input)
}

/// Parse an identifier (for type parameter names)
pub fn parse_identifier(input: &str) -> IResult<&str, String> {
    let ident = recognize(pair(
        alt((alpha1, tag("_"))),
        opt(alphanumeric1),
    ));
    
    ws(ident).map(|s: &str| s.to_string()).parse(input)
}

/// Parse a constraint (e.g., "Read", "length >= 5", "matches 'pattern'")
pub fn parse_constraint(input: &str) -> IResult<&str, String> {
    // Parse capability constraint
    let capability_constraint = parse_capability;
    
    // Parse length constraint
    let length_constraint = map(
        tuple((
            tag("length"),
            ws(alt((tag(">="), tag("<=")))),
            ws(digit1),
        )),
        |(_, op, num): (&str, &str, &str)| {
            format!("length {} {}", op, num)
        },
    );
    
    // Parse pattern constraint
    let pattern_constraint = map(
        tuple((
            tag("matches"),
            ws(tag("'")),
            alphanumeric1,
            ws(tag("'")),
        )),
        |(_, _, pattern, _): (&str, &str, &str, &str)| {
            format!("matches '{}'", pattern)
        },
    );
    
    alt((
        capability_constraint,
        length_constraint,
        pattern_constraint,
    )).parse(input)
}

/// Parse a type parameter with optional constraint
/// Syntax: `T` or `T: Read` or `T: length >= 5`
pub fn parse_type_param(input: &str) -> IResult<&str, (String, Option<String>)> {
    let (input, param_name) = parse_identifier(input)?;
    
    let (input, constraint_opt) = opt(preceded(
        ws(tag(":")),
        parse_constraint,
    )).parse(input)?;
    
    Ok((input, (param_name, constraint_opt)))
}

/// Parse a list of type parameters
/// Syntax: `<T>` or `<T, U>` or `<T: Read, U: Write>`
pub fn parse_type_params(input: &str) -> IResult<&str, Vec<(String, Option<String>)>> {
    delimited(
        ws(tag("<")),
        separated_list1(ws(tag(",")), parse_type_param),
        ws(tag(">")),
    ).parse(input)
}

/// Parse a parametric identity type
/// Syntax: `Identity<T>` or `Identity<T: Read>` or `Identity<T: Read, U: Write>`
pub fn parse_parametric_identity(input: &str) -> IResult<&str, (String, Vec<(String, Option<String>)>)> {
    let (input, type_name) = parse_identifier(input)?;
    
    if type_name != "Identity" && type_name != "identity" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    
    let (input, type_params) = parse_type_params(input)?;
    
    Ok((input, (type_name, type_params)))
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
        assert_eq!(parse_capability("immutable"), Ok(("", "immutable".to_string())));
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

    #[test]
    fn test_parse_identifier() {
        assert_eq!(parse_identifier("T"), Ok(("", "T".to_string())));
        assert_eq!(parse_identifier("Type1"), Ok(("", "Type1".to_string())));
        assert_eq!(parse_identifier("_T"), Ok(("", "_T".to_string())));
    }

    #[test]
    fn test_parse_constraint() {
        assert_eq!(parse_constraint("read"), Ok(("", "read".to_string())));
        assert_eq!(parse_constraint("length >= 5"), Ok(("", "length >= 5".to_string())));
        assert_eq!(parse_constraint("length <= 10"), Ok(("", "length <= 10".to_string())));
        assert_eq!(parse_constraint("matches 'pattern'"), Ok(("", "matches 'pattern'".to_string())));
    }

    #[test]
    fn test_parse_type_param() {
        assert_eq!(parse_type_param("T"), Ok(("", ("T".to_string(), None))));
        assert_eq!(parse_type_param("T: read"), Ok(("", ("T".to_string(), Some("read".to_string())))));
        assert_eq!(parse_type_param("T: length >= 5"), Ok(("", ("T".to_string(), Some("length >= 5".to_string())))));
    }

    #[test]
    fn test_parse_type_params() {
        assert_eq!(parse_type_params("<T>"), Ok(("", vec![("T".to_string(), None)])));
        assert_eq!(parse_type_params("<T, U>"), Ok(("", vec![("T".to_string(), None), ("U".to_string(), None)])));
        assert_eq!(parse_type_params("<T: read, U: write>"), Ok(("", vec![("T".to_string(), Some("read".to_string())), ("U".to_string(), Some("write".to_string()))])));
    }

    #[test]
    fn test_parse_parametric_identity() {
        assert_eq!(parse_parametric_identity("Identity<T>"), Ok(("", ("Identity".to_string(), vec![("T".to_string(), None)]))));
        assert_eq!(parse_parametric_identity("Identity<T: read>"), Ok(("", ("Identity".to_string(), vec![("T".to_string(), Some("read".to_string()))]))));
        assert_eq!(parse_parametric_identity("Identity<T: read, U: write>"), Ok(("", ("Identity".to_string(), vec![("T".to_string(), Some("read".to_string())), ("U".to_string(), Some("write".to_string()))]))));
    }
}