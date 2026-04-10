use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, not, peek, value};
use nom::sequence::tuple;

fn ws<'a, F: 'a, O>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    nom::sequence::delimited(
        nom::character::complete::multispace0,
        f,
        nom::character::complete::multispace0,
    )
}

fn parse_ident(input: &str) -> IResult<&str, String> {
    let (input, first) = nom::character::complete::alpha1(input)?;
    let (input, rest) = nom::character::complete::alphanumeric0(input)?;
    Ok((input, format!("{}{}", first, rest)))
}

fn parse_type(input: &str) -> IResult<&str, String> {
    // Simplified type parser for testing
    let (input, ty) = parse_ident(input)?;
    Ok((input, ty))
}

fn parse_param(input: &str) -> IResult<&str, (String, String)> {
    // Try to parse &self or &mut self first (must not be followed by :)
    let parse_self = alt((
        // &mut self (must not be followed by :)
        map(
            tuple((ws(tag("&mut")), ws(tag("self")), peek(not(ws(tag(":")))))),
            |_| ("&mut self".to_string(), "Self".to_string()),
        ),
        // &self (must not be followed by :)
        map(
            tuple((ws(tag("&")), ws(tag("self")), peek(not(ws(tag(":")))))),
            |_| ("&self".to_string(), "Self".to_string()),
        ),
        // self (without &, must not be followed by :)
        map(tuple((ws(tag("self")), peek(not(ws(tag(":")))))), |_| {
            ("self".to_string(), "Self".to_string())
        }),
    ));

    // Try regular parameter: name: type
    let parse_regular = map(
        tuple((ws(parse_ident), ws(tag(":")), ws(parse_type))),
        |(name, _, ty)| (name, ty),
    );

    alt((parse_self, parse_regular)).parse(input)
}

fn main() {
    let test_cases = vec![
        "a: i64",
        "b: String",
        "self",
        "&self",
        "&mut self",
    ];
    
    for test in test_cases {
        println!("Testing: '{}'", test);
        match parse_param(test) {
            Ok((remaining, (name, ty))) => {
                println!("  Success! name: '{}', type: '{}', remaining: '{}'", name, ty, remaining);
            }
            Err(e) => {
                println!("  Error: {:?}", e);
            }
        }
        println!();
    }
}