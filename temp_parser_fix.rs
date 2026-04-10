pub fn parse_ident(input: &str) -> IResult<&str, String> {
    let (input, ident): (&str, &str) = verify(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(satisfy(|c: char| c.is_alphanumeric() || c == '_')),
        )),
        |s: &str| {
            ![
                "let", "mut", "if", "else", "for", "in", "loop", "while", "unsafe", "return", "break",
                "continue", "fn", "concept", "impl", "enum", "struct", "type", "use", "extern",
                "dyn", "box", "as", "true", "false", "comptime", "const", "async", "pub",
                // Built-in types that shouldn't be parsed as identifiers
                "i8", "i16", "i32", "i64",
                "u8", "u16", "u32", "u64", "usize",
                "f32", "f64",
                "bool", "char", "str", "String",
                // Logical operator keywords (should not be parsed as identifiers)
                "and", "or", "not"
            ]
            .contains(&s)
        },
    )
    .parse(input)?;

    Ok((input, ident.to_string()))
}