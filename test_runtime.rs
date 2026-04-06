// Test the runtime array functions directly
use std::alloc::{alloc, Layout};

// Simple test of array_get logic
fn test_array_get() {
    println!("Testing array_get logic...");
    
    // Simulate a stack array: [10, 20, 30]
    let stack_array: [i64; 3] = [10, 20, 30];
    let stack_ptr = stack_array.as_ptr() as i64;
    
    // Simulate array_get for stack array
    unsafe {
        // This is what array_get does for stack arrays:
        let data_ptr = stack_ptr as *mut i64;
        let value = *data_ptr.offset(0); // index 0
        println!("Stack array[0] = {}", value); // Should be 10
    }
}

fn main() {
    test_array_get();
}