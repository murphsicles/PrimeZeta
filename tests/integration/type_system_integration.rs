// Integration test for type system integration
// Verifies that the new algebraic type system is properly integrated with Zeta

use zetac::compile_and_run_zeta;

#[test]
fn test_basic_type_inference_integration() {
    // This test verifies the type system integration is working
    // by testing a simple program that should compile and run
    
    let code = r#"
        fn main() -> i32 {
            let x = 42;
            x
        }
    "#;
    
    let result = compile_and_run_zeta(code);
    assert!(result.is_ok(), "Basic type inference should work with integrated type system");
    assert_eq!(result.unwrap(), 42, "Should return the correct value");
}

#[test]
fn test_function_return_type_integration() {
    // Test that function return type checking works with integrated type system
    
    let code = r#"
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        fn main() -> i32 {
            add(2, 3)
        }
    "#;
    
    let result = compile_and_run_zeta(code);
    assert!(result.is_ok(), "Function return type checking should work");
    assert_eq!(result.unwrap(), 5, "Should compute correct result");
}

#[test]
fn test_new_type_system_example() {
    // Test that the example demonstrating the new type system works
    
    let code = r#"
        // Example that should work with the new algebraic type system
        fn main() -> i32 {
            // Simple expression that uses type inference
            let x = 10;
            let y = 20;
            x + y
        }
    "#;
    
    let result = compile_and_run_zeta(code);
    assert!(result.is_ok(), "New type system should handle basic arithmetic");
    assert_eq!(result.unwrap(), 30, "Should compute correct sum");
}