use std::env;
use std::fs;

fn main() {
    // Simple test of the parser
    println!("Testing parser with simple expressions");
    
    // Test cases
    let tests = vec![
        "1 < 2",
        "a < b",
        "j < 5 && j >= 0",
        "let x = 5",
        "fn main() -> u64 { return 42 }",
    ];
    
    for test in tests {
        println!("\n=== Testing: '{}' ===", test);
        
        // Write test to file
        fs::write("test_input.z", test).unwrap();
        
        // Try to compile it
        let output = std::process::Command::new("zetac")
            .arg("test_input.z")
            .output();
            
        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("✓ Success!");
                } else {
                    println!("✗ Failed with status: {}", output.status);
                    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                println!("✗ Command failed: {}", e);
            }
        }
        
        // Clean up
        let _ = fs::remove_file("test_input.z");
    }
}