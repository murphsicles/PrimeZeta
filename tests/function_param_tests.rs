//! Tests for function parameter parsing with type annotations

use zetac::frontend::parser::top_level::parse_zeta;

#[test]
fn test_simple_typed_parameters() {
    let test_cases = vec![
        ("fn test(a: i64) -> i64 { a }", 1),
        ("fn add(a: i64, b: i64) -> i64 { a + b }", 2),
        ("fn identity(x: i32) -> i32 { x }", 1),
        ("fn no_params() -> i64 { 42 }", 0),
    ];
    
    for (code, expected_param_count) in test_cases {
        let result = parse_zeta(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
        
        let (remaining, ast) = result.unwrap();
        assert!(remaining.is_empty() || remaining.chars().all(|c| c.is_whitespace()),
                "Did not consume all input: '{}'", remaining);
        
        assert_eq!(ast.len(), 1, "Expected exactly one AST node");
        
        if let zetac::frontend::ast::AstNode::FuncDef { params, .. } = &ast[0] {
            assert_eq!(params.len(), expected_param_count,
                      "Expected {} parameters, got {}", expected_param_count, params.len());
        } else {
            panic!("Expected FuncDef, got {:?}", ast[0]);
        }
    }
}

#[test]
fn test_self_parameters() {
    let test_cases = vec![
        "fn method(&self) -> i64 { 42 }",
        "fn method_mut(&mut self) -> i64 { 42 }",
        "fn method_self(self) -> i64 { 42 }",
        // self with explicit type should also work
        "fn typed_self(self: Self) -> i64 { 42 }",
    ];
    
    for code in test_cases {
        let result = parse_zeta(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
        
        let (remaining, _) = result.unwrap();
        assert!(remaining.is_empty() || remaining.chars().all(|c| c.is_whitespace()),
                "Did not consume all input: '{}'", remaining);
    }
}

#[test]
fn test_various_type_annotations() {
    let test_cases = vec![
        "fn with_ref(x: &i64) -> i64 { *x }",
        "fn with_mut_ref(x: &mut i64) -> i64 { *x }",
        "fn with_string(s: String) -> String { s }",
        "fn with_array(arr: [i64; 10]) -> i64 { arr[0] }",
        "fn with_tuple(t: (i64, i32)) -> i64 { t.0 }",
    ];
    
    for code in test_cases {
        let result = parse_zeta(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
        
        let (remaining, _) = result.unwrap();
        assert!(remaining.is_empty() || remaining.chars().all(|c| c.is_whitespace()),
                "Did not consume all input: '{}'", remaining);
    }
}

#[test]
fn test_whitespace_variations() {
    // Test that whitespace doesn't affect parsing
    let test_cases = vec![
        "fn no_space(x:i64)->i64{x}",
        "fn space_before_colon(x :i64)->i64{x}",
        "fn space_after_colon(x: i64)->i64{x}",
        "fn space_both(x : i64)->i64{x}",
        "fn multiple_spaces(x   :   i64) -> i64 { x }",
    ];
    
    for code in test_cases {
        let result = parse_zeta(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
    }
}