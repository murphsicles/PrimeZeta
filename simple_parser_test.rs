use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, opt, value};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded, terminated};

fn ws<'a, P>(inner: P) -> impl Parser<&'a str, Output = P::Output, Error = nom::error::Error<&'a str>>
where
    P: Parser<&'a str, Error = nom::error::Error<&'a str>>,
{
    delimited(
        value((), many0(value((), tag(" ")))),
        inner,
        value((), many0(value((), tag(" ")))),
    )
}

fn parse_ident(input: &str) -> IResult<&str, String> {
    let (input, ident) = alpha1(input)?;
    Ok((input, ident.to_string()))
}

// Simple parse_type_with_depth implementation for testing
fn parse_type_with_depth(input: &str, depth: usize) -> IResult<&str, String> {
    const MAX_DEPTH: usize = 10;
    if depth > MAX_DEPTH {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TooLarge,
        )));
    }
    
    // Try array type first
    if let Ok(result) = parse_array_type_with_depth(input, depth) {
        return Ok(result);
    }
    
    // Then try simple identifier
    let (input, ident) = parse_ident(input)?;
    Ok((input, ident))
}

fn parse_array_type_with_depth(input: &str, depth: usize) -> IResult<&str, String> {
    // Zeta style: [T; N] or [T]
    let (input, _) = ws(tag("[")).parse(input)?;
    let (input, elem_type) = ws(|i| parse_type_with_depth(i, depth + 1)).parse(input)?;
    
    let (input, size_opt) = opt(preceded(
        ws(tag(";")),
        ws(alt((
            digit1.map(|s: &str| s.to_string()),
            parse_ident,
        ))),
    ))
    .parse(input)?;

    let (input, _) = ws(tag("]")).parse(input)?;

    let result = if let Some(size) = size_opt {
        format!("[{}; {}]", elem_type, size)
    } else {
        format!("[{}]", elem_type)
    };

    Ok((input, result))
}

fn parse_array_type(input: &str) -> IResult<&str, String> {
    parse_array_type_with_depth(input, 0)
}

fn main() {
    println!("Testing array type parser fix for infinite recursion...\n");
    
    let test_cases = vec![
        ("[u64]", "[u64]"),
        ("[u64; 10]", "[u64; 10]"),
        ("[[u64]]", "[[u64]]"),
        ("[[u64; 4]; 3]", "[[u64; 4]; 3]"),
        ("[[[u64]]]", "[[[u64]]]"),
    ];
    
    for (input, expected) in test_cases {
        println!("Testing: '{}'", input);
        match parse_array_type(input) {
            Ok((remaining, result)) => {
                if !remaining.is_empty() {
                    println!("  WARNING: Did not consume all input: '{}'", remaining);
                }
                if result == expected {
                    println!("  ✓ PASS: Got '{}'", result);
                } else {
                    println!("  ✗ FAIL: Expected '{}', got '{}'", expected, result);
                }
            }
            Err(e) => {
                println!("  ✗ ERROR: {:?}", e);
            }
        }
    }
    
    // Test depth limit
    println!("\nTesting depth limit (should fail):");
    let deep_nested = "[[[[[[[[[[u64]]]]]]]]]]"; // 10 levels
    match parse_array_type(deep_nested) {
        Ok((remaining, result)) => {
            println!("  Parsed: '{}' (remaining: '{}')", result, remaining);
        }
        Err(e) => {
            println!("  ✓ Correctly failed with error: {:?}", e);
        }
    }
}