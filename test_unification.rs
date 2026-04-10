// Test slice-array unification
use contamination_backup::zeta_github::src::middle::types::{Type, ArraySize, Substitution};

fn test_slice_array_unification() {
    let mut sub = Substitution::new();
    
    // Test 1: Slice(u8) should unify with Array(u8, size=0)
    let slice_type = Type::Slice(Box::new(Type::U8));
    let array_type = Type::Array(Box::new(Type::U8), ArraySize::Literal(0));
    
    match sub.unify(&slice_type, &array_type) {
        Ok(()) => println!("Test 1 PASSED: Slice(u8) unifies with Array(u8, 0)"),
        Err(e) => println!("Test 1 FAILED: {:?}", e),
    }
    
    // Test 2: Array(u8, size=0) should unify with Slice(u8)
    let mut sub2 = Substitution::new();
    match sub2.unify(&array_type, &slice_type) {
        Ok(()) => println!("Test 2 PASSED: Array(u8, 0) unifies with Slice(u8)"),
        Err(e) => println!("Test 2 FAILED: {:?}", e),
    }
    
    // Test 3: Slice(u8) should NOT unify with Array(u8, size=10)
    let mut sub3 = Substitution::new();
    let array_type_10 = Type::Array(Box::new(Type::U8), ArraySize::Literal(10));
    match sub3.unify(&slice_type, &array_type_10) {
        Ok(()) => println!("Test 3 FAILED: Slice(u8) should not unify with Array(u8, 10)"),
        Err(_) => println!("Test 3 PASSED: Slice(u8) correctly fails to unify with Array(u8, 10)"),
    }
    
    // Test 4: Slice-slice unification
    let mut sub4 = Substitution::new();
    let slice_type2 = Type::Slice(Box::new(Type::U8));
    match sub4.unify(&slice_type, &slice_type2) {
        Ok(()) => println!("Test 4 PASSED: Slice(u8) unifies with Slice(u8)"),
        Err(e) => println!("Test 4 FAILED: {:?}", e),
    }
}

fn main() {
    test_slice_array_unification();
}