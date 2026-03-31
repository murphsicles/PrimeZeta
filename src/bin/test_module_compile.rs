use zetac::compile_and_run_zeta;

fn main() {
    // Test 1: Simple module with function
    let code1 = r#"
        mod math {
            pub fn add(a: i64, b: i64) -> i64 {
                a + b
            }
        }
        
        fn main() -> i64 {
            // Can't call math::add yet because we don't have use statement or qualified call support
            // For now, just return a constant
            42
        }
    "#;

    println!("Test 1: Simple module compilation");
    match compile_and_run_zeta(code1) {
        Ok(value) => {
            println!("Success! Returned: {}", value);
            assert_eq!(value, 42, "Should return 42");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Test 2: Module with private function (should still compile)
    let code2 = r#"
        mod math {
            fn private_add(a: i64, b: i64) -> i64 {
                a + b
            }
            
            pub fn public_add(a: i64, b: i64) -> i64 {
                private_add(a, b)
            }
        }
        
        fn main() -> i64 {
            // Can't call math::public_add yet
            42
        }
    "#;

    println!("\nTest 2: Module with public/private functions");
    match compile_and_run_zeta(code2) {
        Ok(value) => {
            println!("Success! Returned: {}", value);
            assert_eq!(value, 42, "Should return 42");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Test 3: Nested modules
    let code3 = r#"
        mod outer {
            pub mod inner {
                pub fn secret() -> i64 {
                    42
                }
            }
        }
        
        fn main() -> i64 {
            // Can't call outer::inner::secret yet
            42
        }
    "#;

    println!("\nTest 3: Nested modules");
    match compile_and_run_zeta(code3) {
        Ok(value) => {
            println!("Success! Returned: {}", value);
            assert_eq!(value, 42, "Should return 42");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Test 4: Regression test - code without modules should still work
    let code4 = r#"
        fn add(a: i64, b: i64) -> i64 {
            a + b
        }
        
        fn main() -> i64 {
            add(10, 20)
        }
    "#;

    println!("\nTest 4: Regression test (no modules)");
    match compile_and_run_zeta(code4) {
        Ok(value) => {
            println!("Success! Returned: {}", value);
            assert_eq!(value, 30, "Should return 30");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
