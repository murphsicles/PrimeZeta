use std::fs;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <file.z>", args[0]);
        return Ok(());
    }
    
    let file = &args[1];
    let code = fs::read_to_string(file)?;
    
    // Simple parser - just check if it looks like Zeta code
    if code.contains("fn ") || code.contains("let ") || code.contains("return ") {
        println!("File {} appears to be valid Zeta code", file);
        Ok(())
    } else {
        Err("File doesn't appear to be valid Zeta code".into())
    }
}