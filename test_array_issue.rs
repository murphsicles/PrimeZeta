// Test to reproduce array issues in Zeta
fn main() {
    // Test 1: Empty array literal
    let mut arr: [u8] = [];
    println!("Empty array: {:?}", arr);
    
    // Test 2: Array with values
    let arr2: [u8; 3] = [1, 2, 3];
    println!("Array with values: {:?}", arr2);
    
    // Test 3: Array repeat
    let arr3: [bool; 5] = [false; 5];
    println!("Array repeat: {:?}", arr3);
    
    // Test 4: Dynamic array
    let mut dyn_arr: [dynamic]u8 = [dynamic]u8{};
    dyn_arr.push(1);
    dyn_arr.push(2);
    println!("Dynamic array: {:?}", dyn_arr);
}