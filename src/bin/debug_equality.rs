use zetac::middle::types::*;

fn main() {
    println!("Testing type equality:");
    println!("Type::Usize == Type::Usize: {}", Type::Usize == Type::Usize);
    println!("Type::Usize == Type::I64: {}", Type::Usize == Type::I64);
    println!("Type::I64 == Type::I64: {}", Type::I64 == Type::I64);
    println!("Type::U64 == Type::U64: {}", Type::U64 == Type::U64);
    println!("Type::U64 == Type::I64: {}", Type::U64 == Type::I64);
    
    println!("\nTesting pattern matching:");
    let types = vec![Type::Usize, Type::I64, Type::U64];
    
    for t in types {
        match t {
            Type::Usize => println!("Matched Type::Usize"),
            Type::I64 => println!("Matched Type::I64"),
            Type::U64 => println!("Matched Type::U64"),
            _ => println!("Matched something else: {:?}", t),
        }
    }
    
    println!("\nTesting unification error messages:");
    let mut subst = Substitution::new();
    
    match subst.unify(&Type::Usize, &Type::I64) {
        Ok(_) => println!("Usize unifies with I64 (unexpected!)"),
        Err(e) => println!("Usize doesn't unify with I64: {}", e),
    }
    
    // Let's trace through what's happening
    println!("\nDebugging unification step by step:");
    
    let t1 = Type::Usize;
    let t2 = Type::I64;
    
    println!("t1 = {:?}", t1);
    println!("t2 = {:?}", t2);
    
    // Check what the match arms would be
    println!("\nMatch arms would be:");
    println!("  (Type::Usize, Type::I64)");
    println!("  (Type::I64, Type::I64)");
    println!("  (Type::Usize, Type::Usize)");
    println!("  etc...");
    
    // Manually check which pattern would match
    match (&t1, &t2) {
        (Type::Usize, Type::I64) => println!("Would match (Type::Usize, Type::I64)"),
        (Type::I64, Type::I64) => println!("Would match (Type::I64, Type::I64)"),
        (Type::Usize, Type::Usize) => println!("Would match (Type::Usize, Type::Usize)"),
        _ => println!("Would match _ (default)"),
    }
}