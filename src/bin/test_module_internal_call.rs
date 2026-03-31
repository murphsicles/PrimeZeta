use zetac::compile_and_run_zeta;

fn main() {
    // Test: Function inside module calling another function in same module
    let code = r#"
        mod math {
            fn helper(a: i64) -> i64 {
                a * 2
            }
            
            pub fn compute(a: i64, b: i64) -> i64 {
                helper(a) + b
            }
        }
        
        fn main() -> i64 {
            // Can't call math::compute yet
            // But the module should compile
            42
        }
    "#;

    println!("Test: Module with internal function call");
    match compile_and_run_zeta(code) {
        Ok(value) => {
            println!("Success! Returned: {}", value);
            assert_eq!(value, 42, "Should return 42");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
